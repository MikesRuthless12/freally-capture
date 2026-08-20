//! The installed hook: the `Present` replacement that copies each back buffer
//! into the shared texture, plus the one-time arming the injector drives.
//!
//! The design keeps the present path minimal and reentrancy-safe: on each
//! present we borrow the game's own device from the swap chain, lazily (re)build
//! the shared texture to match the current back buffer, copy, publish the
//! control block, then **always** call the original `Present`. A copy that fails
//! for any reason is swallowed — the game must present regardless of us.
//!
//! AUDITED `unsafe`: DXGI interop and the global install state. The hook never
//! reads or writes game memory beyond the DXGI vtable slot it owns.

use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};

use windows::core::Interface;
use windows::Win32::Graphics::Direct3D11::{ID3D11Texture2D, D3D11_TEXTURE2D_DESC};
use windows::Win32::Graphics::Dxgi::{IDXGISwapChain, IDXGISwapChain1, DXGI_PRESENT_PARAMETERS};

use crate::protocol::{
    control_name, texture_name, ControlBlock, API_D3D11, API_D3D12, FLAG_GEOMETRY_CHANGED,
    FLAG_PRODUCER_ALIVE, KEY_CONSUMER, KEY_PRODUCER,
};
use crate::win::shmem::ControlMapping;
use crate::win::texture::SharedTexture;
use crate::win::vtable::{VtableHook, PRESENT1_INDEX, PRESENT_INDEX};

/// The original `Present` — `HRESULT Present(this, SyncInterval, Flags)`.
type PresentFn = unsafe extern "system" fn(*mut c_void, u32, u32) -> i32;
/// The original `Present1` —
/// `HRESULT Present1(this, SyncInterval, Flags, *const DXGI_PRESENT_PARAMETERS)`.
type Present1Fn =
    unsafe extern "system" fn(*mut c_void, u32, u32, *const DXGI_PRESENT_PARAMETERS) -> i32;

/// Everything the hook needs across present calls. One instance, built once.
struct HookState {
    /// Held purely so `Drop` restores the vtable when the DLL unloads.
    ///
    /// A `OnceLock` because the state must be published *before* the vtable
    /// slot is patched (see [`arm`]); the hook is filled in immediately after.
    _present_hook: OnceLock<VtableHook>,
    original_present: PresentFn,
    /// The `Present1` slot, when the chain exposed `IDXGISwapChain1`.
    ///
    /// Modern flip-model titles (DXGI 1.2+, which is most of what runs in
    /// exclusive fullscreen today) present through `Present1` and never touch
    /// slot 8 — so hooking `Present` alone armed successfully and then never
    /// saw a single frame.
    _present1_hook: OnceLock<VtableHook>,
    original_present1: OnceLock<Present1Fn>,
    control: ControlMapping,
    shared: Mutex<Option<SharedTexture>>,
    frame_index: AtomicU64,
    /// The one swap chain we capture from, latched on first sight.
    ///
    /// The patched slot is shared by EVERY swap chain in the process, and a
    /// game may own several (a launcher window, an editor viewport, a second
    /// monitor, an in-game overlay, its own splash screen). Without a latch,
    /// two chains of different sizes make the geometry check fire on every
    /// present and the shared texture is destroyed and recreated at full
    /// resolution 60+ times a second.
    swapchain: AtomicPtr<c_void>,
    /// Whether the "not D3D11" block has already been published (once only).
    announced_unsupported: AtomicBool,
    pid: u32,
}

// The swap chain and device pointers are process-lived; the Mutex guards the
// only mutable D3D object.
unsafe impl Send for HookState {}
unsafe impl Sync for HookState {}

static STATE: OnceLock<HookState> = OnceLock::new();
static INSTALLED: AtomicBool = AtomicBool::new(false);

/// The `Present` replacement installed in the game's swap-chain vtable.
///
/// # Safety
/// Matches the `IDXGISwapChain::Present` ABI; DXGI calls it with a live
/// swap-chain `this`.
unsafe extern "system" fn present_hook(this: *mut c_void, sync: u32, flags: u32) -> i32 {
    match STATE.get() {
        Some(state) => {
            if flags & DXGI_PRESENT_TEST != 0 {
                return (state.original_present)(this, sync, flags);
            }
            // Never let our work fail the game's present: swallow panics and
            // fall through to the original either way.
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                capture_backbuffer(state, this);
            }));
            (state.original_present)(this, sync, flags)
        }
        // Unreachable in practice (STATE is set before the slot is armed), but
        // there is no original to chain to, so report success and let the game
        // carry on rather than failing its present.
        None => 0,
    }
}

/// The `Present1` replacement — the slot flip-model titles actually use.
///
/// # Safety
/// Matches the `IDXGISwapChain1::Present1` ABI; DXGI calls it with a live
/// swap-chain `this`.
unsafe extern "system" fn present1_hook(
    this: *mut c_void,
    sync: u32,
    flags: u32,
    params: *const DXGI_PRESENT_PARAMETERS,
) -> i32 {
    match STATE.get() {
        Some(state) => {
            let chain_through = |state: &HookState| match state.original_present1.get() {
                Some(original) => original(this, sync, flags, params),
                // Armed slot 22 without recording the original: impossible in
                // practice (both are set together), but never fail the present.
                None => 0,
            };
            if flags & DXGI_PRESENT_TEST != 0 {
                return chain_through(state);
            }
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                capture_backbuffer(state, this);
            }));
            chain_through(state)
        }
        None => 0,
    }
}

/// `DXGI_PRESENT_TEST`: "would this present succeed?", rendering nothing. Games
/// issue it repeatedly while occluded or alt-tabbed — exactly when the back
/// buffer is stale — so capturing it costs a full-surface copy and a keyed-mutex
/// round trip to publish a frame that is not new.
const DXGI_PRESENT_TEST: u32 = 0x0000_0001;

/// Copy the current back buffer into the shared texture and publish the block.
/// Best-effort: any early return simply skips this frame.
///
/// # Safety
/// `swapchain_ptr` is the live `this` DXGI handed to `Present`.
unsafe fn capture_backbuffer(state: &HookState, swapchain_ptr: *mut c_void) {
    // One chain only. The patched slot is shared by every swap chain in the
    // process, so latch the first one we see and ignore the rest — otherwise a
    // game with a launcher window, an editor viewport or its own splash screen
    // alternates geometries and rebuilds the shared texture every present.
    let latched = state
        .swapchain
        .compare_exchange(
            std::ptr::null_mut(),
            swapchain_ptr,
            Ordering::AcqRel,
            Ordering::Acquire,
        )
        .unwrap_or_else(|current| current);
    if !latched.is_null() && latched != swapchain_ptr {
        return;
    }

    // Borrow the swap chain without taking a reference off the game.
    let Some(swapchain) = IDXGISwapChain::from_raw_borrowed(&swapchain_ptr) else {
        return;
    };
    let Ok(backbuffer) = swapchain.GetBuffer::<ID3D11Texture2D>(0) else {
        // Not a D3D11 back buffer — a D3D12 title presents through this same
        // vtable but its buffers are `ID3D12Resource`, which needs a command
        // queue and a copy this hook does not implement. Say so once, loudly
        // enough for the app to stop waiting: it polls for FLAG_PRODUCER_ALIVE
        // and would otherwise sit through its whole first-frame timeout before
        // falling back to Window Capture.
        announce_unsupported_api(state);
        return;
    };

    let Ok(device) = backbuffer.GetDevice() else {
        return;
    };
    let Ok(context) = device.GetImmediateContext() else {
        return;
    };

    let mut desc = D3D11_TEXTURE2D_DESC::default();
    backbuffer.GetDesc(&mut desc);

    let Ok(mut guard) = state.shared.lock() else {
        return;
    };

    // (Re)build the shared texture when absent or the geometry changed.
    let needs_rebuild = match guard.as_ref() {
        Some(tex) => {
            // The device comparison is what survives a device-lost: a TDR, a
            // driver update, or an alt-tab out of exclusive fullscreen makes
            // many titles create a NEW device at the SAME resolution, so
            // geometry alone reports "no rebuild needed" and the copy below
            // then mixes resources from two different devices — invalid, and
            // with nothing to recover it, capture is dead for the session.
            tex.device != Interface::as_raw(&device)
                || tex.width != desc.Width
                || tex.height != desc.Height
                || tex.format != desc.Format
        }
        None => true,
    };
    let mut geometry_changed = false;
    if needs_rebuild {
        // Only a *change* is a change — the first build is not one.
        geometry_changed = guard.is_some();
        // Drop the old texture FIRST. It owns the named shared handle, and the
        // name stays registered until that handle closes — so creating the
        // replacement while the old one is still alive fails with
        // DXGI_ERROR_NAME_ALREADY_EXISTS and capture never recovers.
        *guard = None;
        *guard = SharedTexture::create(
            &device,
            &texture_name(state.pid),
            desc.Width,
            desc.Height,
            desc.Format,
        )
        .ok();
    }
    let Some(shared) = guard.as_ref() else { return };

    // Copy under the keyed mutex; a contended frame is skipped, never blocked.
    //
    // A multisampled back buffer (a bitblt-model title with MSAA) cannot be
    // `CopyResource`d into a single-sample texture — D3D11 fails it silently on
    // every frame, so capture looked armed and produced nothing. Resolve
    // instead; the shared texture is deliberately always `Count: 1`.
    let multisampled = desc.SampleDesc.Count > 1;
    let copied = shared.with_lock(KEY_PRODUCER, KEY_CONSUMER, || {
        if multisampled {
            context.ResolveSubresource(&shared.texture, 0, &backbuffer, 0, desc.Format);
        } else {
            context.CopyResource(&shared.texture, &backbuffer);
        }
    });
    if !matches!(copied, Ok(Some(()))) {
        return;
    }

    let index = state.frame_index.fetch_add(1, Ordering::AcqRel) + 1;
    let mut block = ControlBlock::new(API_D3D11);
    block.width = shared.width;
    block.height = shared.height;
    block.format = shared.format.0 as u32;
    block.flags = FLAG_PRODUCER_ALIVE
        | if geometry_changed {
            FLAG_GEOMETRY_CHANGED
        } else {
            0
        };
    block.frame_index = index;
    state.control.write(&block);
}

/// Publish one honest "this title is not D3D11" block, once.
///
/// Deliberately does NOT set `FLAG_PRODUCER_ALIVE`: there is no frame and there
/// never will be on this path. What it does is name the API, so the consumer can
/// distinguish "still starting up" from "this will never work" and fall back to
/// Window Capture immediately instead of after its first-frame timeout.
unsafe fn announce_unsupported_api(state: &HookState) {
    if state.announced_unsupported.swap(true, Ordering::AcqRel) {
        return;
    }
    let mut block = ControlBlock::new(API_D3D12);
    block.flags = 0;
    state.control.write(&block);
}

/// Arm the hook against a live swap chain. Called from the install thread, never
/// from `DllMain` (loader lock). Idempotent — a second call is a no-op.
///
/// # Safety
/// `swapchain` must be a live `IDXGISwapChain` belonging to this process.
pub unsafe fn arm(swapchain: &IDXGISwapChain) -> Result<(), String> {
    if INSTALLED.swap(true, Ordering::SeqCst) {
        return Ok(());
    }
    let this = swapchain.as_raw();
    // Read the real Present out of the vtable first, so the hook can chain it.
    let vtable = *(this as *mut *mut *mut c_void);
    let original_present: PresentFn = std::mem::transmute(*vtable.add(PRESENT_INDEX));

    let pid = std::process::id();

    // Publish the state BEFORE patching the vtable. The game's render thread
    // can enter the hook the instant the slot changes, and a hook that finds no
    // state takes the `None => 0` arm — returning S_OK without ever chaining to
    // the real Present, silently swallowing that flip. An earlier attempt that
    // got this far leaves the state in place, so reuse it rather than remapping
    // a control block this process already owns.
    if STATE.get().is_none() {
        let control = match ControlMapping::create(&control_name(pid)) {
            Ok(control) => control,
            Err(err) => {
                // Nothing was installed, so let a later attempt retry.
                INSTALLED.store(false, Ordering::SeqCst);
                return Err(format!("control block: {err}"));
            }
        };
        let _ = STATE.set(HookState {
            _present_hook: OnceLock::new(),
            original_present,
            _present1_hook: OnceLock::new(),
            original_present1: OnceLock::new(),
            control,
            shared: Mutex::new(None),
            frame_index: AtomicU64::new(0),
            swapchain: AtomicPtr::new(std::ptr::null_mut()),
            announced_unsupported: AtomicBool::new(false),
            pid,
        });
    }

    let present_hook_slot = VtableHook::install(this, PRESENT_INDEX, present_hook as *mut c_void)
        .inspect_err(|_| {
        // Leave the flag clear so a later attempt can retry.
        INSTALLED.store(false, Ordering::SeqCst);
    })?;
    if let Some(state) = STATE.get() {
        let _ = state._present_hook.set(present_hook_slot);
    }

    // Slot 22 as well, whenever the chain is a DXGI 1.2+ IDXGISwapChain1.
    // Flip-model titles present through Present1 and never touch slot 8, so
    // without this the hook armed 'successfully' and then saw no frames at
    // all on most modern games. Best-effort: a chain that is only
    // IDXGISwapChain (older bitblt-model titles) keeps the slot-8 path.
    if let Ok(chain1) = swapchain.cast::<IDXGISwapChain1>() {
        let this1 = chain1.as_raw();
        let vtable1 = *(this1 as *mut *mut *mut c_void);
        let original_present1: Present1Fn = std::mem::transmute(*vtable1.add(PRESENT1_INDEX));
        if let Ok(hook) = VtableHook::install(this1, PRESENT1_INDEX, present1_hook as *mut c_void) {
            if let Some(state) = STATE.get() {
                // The original goes in FIRST: the moment the slot is patched
                // the game can enter the hook, which needs it to chain.
                let _ = state.original_present1.set(original_present1);
                let _ = state._present1_hook.set(hook);
            }
        }
    }
    Ok(())
}

/// Put both vtable slots back, if they still hold our functions.
///
/// Rust never drops a `static`, so `HookState`'s `VtableHook`s never run their
/// `Drop` — the "restores itself at process teardown" the module used to claim
/// simply did not happen. That matters for any path that unloads this DLL while
/// the game keeps running (an anti-cheat sweep, a future eject): the slot would
/// point into freed pages and the next present would jump into them.
///
/// # Safety
/// Call only when no other thread can be inside the hooks — i.e. from
/// `DLL_PROCESS_DETACH`.
pub unsafe fn restore() {
    let Some(state) = STATE.get() else { return };
    if let Some(hook) = state._present1_hook.get() {
        hook.restore_if_ours(present1_hook as *mut c_void);
    }
    if let Some(hook) = state._present_hook.get() {
        hook.restore_if_ours(present_hook as *mut c_void);
    }
    INSTALLED.store(false, Ordering::SeqCst);
}

/// Whether the hook is currently armed in this process.
pub fn is_armed() -> bool {
    INSTALLED.load(Ordering::SeqCst)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_hook_matches_the_dxgi_present_abi() {
        // A compile-time contract: the replacement must be assignable to the
        // same fn pointer type we transmute the real Present into. If
        // IDXGISwapChain::Present ever changed shape, this stops compiling
        // instead of corrupting the stack at runtime.
        let _f: PresentFn = present_hook;
    }

    #[test]
    fn arming_twice_is_a_no_op() {
        // A real swap chain is not available in a unit test, but the guard that
        // makes arm() idempotent is observable on its own.
        assert!(!is_armed());
        assert!(!INSTALLED.swap(true, Ordering::SeqCst), "first arm wins");
        assert!(INSTALLED.swap(true, Ordering::SeqCst), "second is a no-op");
        INSTALLED.store(false, Ordering::SeqCst);
    }
}
