//! `DllMain` and the off-loader-lock arming thread.
//!
//! The cardinal rule of `DllMain`: under the loader lock you may not create
//! D3D devices, load other DLLs, or block. So process-attach does the minimum —
//! spawn a thread — and every real step (find the swap chain, patch the vtable)
//! happens on that thread once the lock is released.
//!
//! Finding the game's swap chain from inside the process without one handed to
//! us means the classic dummy-swapchain trick: create a throwaway swap chain on
//! a hidden message-only window, read `Present` out of *its* vtable (all
//! swap chains of the same D3D version share one vtable), patch that shared
//! slot, and release the dummy. From then on the game's real presents run
//! through our hook.
//!
//! AUDITED `unsafe`: DXGI device/swapchain creation and the FFI `DllMain`.

use std::ffi::c_void;

use windows::Win32::Foundation::{HINSTANCE, HWND};
use windows::Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE_HARDWARE, D3D_FEATURE_LEVEL_11_0};
use windows::Win32::Graphics::Direct3D11::{
    D3D11CreateDeviceAndSwapChain, D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_SDK_VERSION,
};
use windows::Win32::Graphics::Dxgi::Common::{
    DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_MODE_DESC, DXGI_SAMPLE_DESC,
};
use windows::Win32::Graphics::Dxgi::{
    IDXGISwapChain, DXGI_SWAP_CHAIN_DESC, DXGI_USAGE_RENDER_TARGET_OUTPUT,
};
use windows::Win32::System::LibraryLoader::{DisableThreadLibraryCalls, FreeLibraryAndExitThread};
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DestroyWindow, HMENU, WINDOW_EX_STYLE, WS_OVERLAPPED,
};

/// The DLL entry point. Real for the injected DLL (`cdylib`); the app side links
/// this crate as an `rlib` and never calls it.
///
/// # Safety
/// The system calls this with the standard `DllMain` ABI.
#[no_mangle]
pub unsafe extern "system" fn DllMain(
    module: HINSTANCE,
    reason: u32,
    _reserved: *mut c_void,
) -> i32 {
    match reason {
        DLL_PROCESS_ATTACH => {
            // We never care about thread attach/detach — opting out is a small
            // perf win and removes a class of loader-lock reentrancy.
            let _ = DisableThreadLibraryCalls(module);
            spawn_arming_thread(module);
        }
        DLL_PROCESS_DETACH => {
            // The vtable hook restores itself when HookState drops at process
            // teardown; nothing to force here.
        }
        _ => {}
    }
    1 // TRUE
}

/// Start the thread that arms the hook once the loader lock is released.
///
/// # Safety
/// `module` is this DLL's instance handle from `DllMain`.
unsafe fn spawn_arming_thread(module: HINSTANCE) {
    let module_raw = module.0 as usize;
    let _ = std::thread::Builder::new()
        .name("freally-game-hook-arm".into())
        .spawn(move || {
            // Best effort: if we can't build the dummy swap chain (unusual D3D
            // config), we simply never arm — the app falls back to WGC.
            if let Err(err) = arm_via_dummy_swapchain() {
                eprintln!("freally-game-hook: could not arm: {err}");
            }
            // Whether or not arming succeeded, the DLL stays resident while the
            // hook is installed; unload happens at process exit. If arming
            // failed we free ourselves so we leave no footprint in the game.
            if !super::hook::is_armed() {
                // SAFETY: module_raw is our own instance; exiting this thread.
                FreeLibraryAndExitThread(HINSTANCE(module_raw as *mut c_void), 0);
            }
        });
}

/// Create a throwaway swap chain, read its shared `Present` vtable, and arm.
fn arm_via_dummy_swapchain() -> Result<(), String> {
    // A message-only-ish hidden window to own the dummy swap chain.
    // SAFETY: standard hidden-window creation with a null class uses the
    // system's default; the handle is destroyed below.
    let hwnd = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE(0),
            windows::core::w!("STATIC"),
            windows::core::w!("freally-hook-dummy"),
            WS_OVERLAPPED,
            0,
            0,
            1,
            1,
            HWND::default(),
            HMENU::default(),
            HINSTANCE::default(),
            None,
        )
    }
    .map_err(|err| format!("dummy window: {err}"))?;

    let desc = DXGI_SWAP_CHAIN_DESC {
        BufferDesc: DXGI_MODE_DESC {
            Width: 1,
            Height: 1,
            Format: DXGI_FORMAT_B8G8R8A8_UNORM,
            ..Default::default()
        },
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: 1,
            Quality: 0,
        },
        BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
        BufferCount: 1,
        OutputWindow: hwnd,
        Windowed: true.into(),
        ..Default::default()
    };

    let mut swapchain: Option<IDXGISwapChain> = None;
    let feature_levels = [D3D_FEATURE_LEVEL_11_0];
    // SAFETY: every out-param is an Option we own; the dummy device/context are
    // dropped with this function.
    let result = unsafe {
        D3D11CreateDeviceAndSwapChain(
            None,
            D3D_DRIVER_TYPE_HARDWARE,
            windows::Win32::Foundation::HMODULE::default(),
            D3D11_CREATE_DEVICE_BGRA_SUPPORT,
            Some(&feature_levels),
            D3D11_SDK_VERSION,
            Some(&desc),
            Some(&mut swapchain),
            None,
            None,
            None,
        )
    };

    let armed = match (result, swapchain) {
        (Ok(()), Some(chain)) => {
            // SAFETY: `chain` is a live swap chain; arm reads/patches its
            // vtable, which the game's real swap chain shares.
            unsafe { super::hook::arm(&chain) }
        }
        (Err(err), _) => Err(format!("dummy swap chain: {err}")),
        (Ok(()), None) => Err("dummy swap chain: none returned".into()),
    };

    // SAFETY: `hwnd` was created above and is used nowhere else.
    unsafe {
        let _ = DestroyWindow(hwnd);
    }
    armed
}
