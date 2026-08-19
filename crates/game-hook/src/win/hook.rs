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
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};

use windows::core::Interface;
use windows::Win32::Graphics::Direct3D11::{ID3D11Texture2D, D3D11_TEXTURE2D_DESC};
use windows::Win32::Graphics::Dxgi::IDXGISwapChain;

use crate::protocol::{
    control_name, texture_name, ControlBlock, API_D3D11, FLAG_GEOMETRY_CHANGED,
    FLAG_PRODUCER_ALIVE, KEY_CONSUMER, KEY_PRODUCER,
};
use crate::win::shmem::ControlMapping;
use crate::win::texture::SharedTexture;
use crate::win::vtable::{VtableHook, PRESENT_INDEX};

/// The original `Present` — `HRESULT Present(this, SyncInterval, Flags)`.
type PresentFn = unsafe extern "system" fn(*mut c_void, u32, u32) -> i32;

/// Everything the hook needs across present calls. One instance, built once.
struct HookState {
    /// Held purely so `Drop` restores the vtable when the DLL unloads.
    ///
    /// A `OnceLock` because the state must be published *before* the vtable
    /// slot is patched (see [`arm`]); the hook is filled in immediately after.
    _present_hook: OnceLock<VtableHook>,
    original_present: PresentFn,
    control: ControlMapping,
    shared: Mutex<Option<SharedTexture>>,
    frame_index: AtomicU64,
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

/// Copy the current back buffer into the shared texture and publish the block.
/// Best-effort: any early return simply skips this frame.
///
/// # Safety
/// `swapchain_ptr` is the live `this` DXGI handed to `Present`.
unsafe fn capture_backbuffer(state: &HookState, swapchain_ptr: *mut c_void) {
    // Borrow the swap chain without taking a reference off the game.
    let Some(swapchain) = IDXGISwapChain::from_raw_borrowed(&swapchain_ptr) else {
        return;
    };
    let Ok(backbuffer) = swapchain.GetBuffer::<ID3D11Texture2D>(0) else {
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
            tex.width != desc.Width || tex.height != desc.Height || tex.format != desc.Format
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
    let copied = shared.with_lock(KEY_PRODUCER, KEY_CONSUMER, || {
        context.CopyResource(&shared.texture, &backbuffer);
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
            control,
            shared: Mutex::new(None),
            frame_index: AtomicU64::new(0),
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
    Ok(())
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
