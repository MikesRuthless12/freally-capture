//! The end-to-end smoke test for CAP-N78: a **flip-model** swap chain, armed,
//! presented through `Present1`, must produce a real captured frame.
//!
//! This is the case the hook used to miss entirely. Only slot 8 (`Present`) was
//! patched, but DXGI 1.2+ flip-model titles — which is most of what runs in
//! exclusive fullscreen today — present through `Present1` (slot 22) and never
//! touch slot 8. Arming reported success and then no frame ever arrived, so the
//! app waited out its whole first-frame timeout before falling back to Window
//! Capture. Nothing in the unit tests could catch that: it needs a real
//! flip-model chain and a real present.
//!
//! It lives in `tests/` rather than beside the unit tests on purpose — arming
//! writes process-global `OnceLock` state, so it needs a process to itself.
//!
//! Skips cleanly (rather than failing) where there is no usable D3D11 adapter,
//! so a headless CI runner without WARP does not turn this red.

#![cfg(windows)]

use windows::Win32::Foundation::{HINSTANCE, HWND};
use windows::Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE_HARDWARE, D3D_FEATURE_LEVEL_11_0};
use windows::Win32::Graphics::Direct3D11::{
    D3D11CreateDevice, ID3D11Device, D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_SDK_VERSION,
};
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_SAMPLE_DESC};
use windows::Win32::Graphics::Dxgi::{
    CreateDXGIFactory1, IDXGIDevice, IDXGIFactory2, IDXGISwapChain, IDXGISwapChain1, DXGI_PRESENT,
    DXGI_SWAP_CHAIN_DESC1, DXGI_SWAP_EFFECT_FLIP_DISCARD, DXGI_USAGE_RENDER_TARGET_OUTPUT,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DestroyWindow, HMENU, WINDOW_EX_STYLE, WS_OVERLAPPED,
};

use freally_game_hook::protocol::{control_name, ControlBlock, FLAG_PRODUCER_ALIVE};
use freally_game_hook::win::hook;
use freally_game_hook::win::shmem::ControlMapping;

/// A hidden 64×64 window to own the chain. Flip-model refuses 1×1 on some
/// drivers, so this is deliberately a real size.
unsafe fn dummy_window() -> Option<HWND> {
    CreateWindowExW(
        WINDOW_EX_STYLE(0),
        windows::core::w!("STATIC"),
        windows::core::w!("freally-hook-flip-test"),
        WS_OVERLAPPED,
        0,
        0,
        64,
        64,
        HWND::default(),
        HMENU::default(),
        HINSTANCE::default(),
        None,
    )
    .ok()
}

unsafe fn make_device() -> Option<ID3D11Device> {
    let mut device: Option<ID3D11Device> = None;
    let levels = [D3D_FEATURE_LEVEL_11_0];
    D3D11CreateDevice(
        None,
        D3D_DRIVER_TYPE_HARDWARE,
        windows::Win32::Foundation::HMODULE::default(),
        D3D11_CREATE_DEVICE_BGRA_SUPPORT,
        Some(&levels),
        D3D11_SDK_VERSION,
        Some(&mut device),
        None,
        None,
    )
    .ok()?;
    device
}

/// Build a genuine flip-model swap chain — `DXGI_SWAP_EFFECT_FLIP_DISCARD`,
/// which is what a modern title uses and what forces `Present1`.
unsafe fn make_flip_chain(device: &ID3D11Device, hwnd: HWND) -> Option<IDXGISwapChain1> {
    let factory: IDXGIFactory2 = CreateDXGIFactory1().ok()?;
    let dxgi_device: IDXGIDevice = windows::core::Interface::cast(device).ok()?;
    let _ = dxgi_device;
    let desc = DXGI_SWAP_CHAIN_DESC1 {
        Width: 64,
        Height: 64,
        Format: DXGI_FORMAT_B8G8R8A8_UNORM,
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: 1,
            Quality: 0,
        },
        BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
        // Flip-model requires >= 2 buffers. This is the whole point of the test.
        BufferCount: 2,
        SwapEffect: DXGI_SWAP_EFFECT_FLIP_DISCARD,
        ..Default::default()
    };
    factory
        .CreateSwapChainForHwnd(device, hwnd, &desc, None, None)
        .ok()
}

#[test]
fn a_flip_model_present1_produces_a_captured_frame() {
    unsafe {
        let Some(device) = make_device() else {
            eprintln!("no D3D11 device — skipping");
            return;
        };
        let Some(hwnd) = dummy_window() else {
            eprintln!("no window — skipping");
            return;
        };
        let Some(chain1) = make_flip_chain(&device, hwnd) else {
            let _ = DestroyWindow(hwnd);
            eprintln!("no flip-model swap chain — skipping");
            return;
        };

        // Arm exactly as the injected DLL does, against the base interface.
        let chain: IDXGISwapChain =
            windows::core::Interface::cast(&chain1).expect("a swap chain is an IDXGISwapChain");
        hook::arm(&chain).expect("arming the hook");
        assert!(hook::is_armed(), "arm() reports armed");

        // Present through Present1 ONLY — never through Present. This is what a
        // flip-model game does, and what used to bypass the hook completely.
        for _ in 0..3 {
            chain1
                .Present1(0, DXGI_PRESENT(0), &Default::default())
                .ok()
                .expect("Present1 succeeds");
        }

        // The producer publishes into the control block by pid. Read it back the
        // way the app's consumer does.
        let control = ControlMapping::open(&control_name(std::process::id()))
            .expect("open the control block")
            .expect("the producer created a control block");
        let block: ControlBlock = control.read();

        assert!(
            block.flags & FLAG_PRODUCER_ALIVE != 0,
            "Present1 must reach the hook and publish a live frame — this is the \
             flip-model path that previously produced nothing at all"
        );
        assert!(
            block.frame_index >= 1,
            "at least one frame was captured (got index {})",
            block.frame_index
        );
        assert_eq!(
            (block.width, block.height),
            (64, 64),
            "the published geometry is the swap chain's"
        );

        // Restoring must put the real functions back, so an unload leaves DXGI
        // exactly as it was.
        hook::restore();
        assert!(!hook::is_armed(), "restore() clears the armed flag");

        // And the chain must still present normally afterwards.
        chain1
            .Present1(0, DXGI_PRESENT(0), &Default::default())
            .ok()
            .expect("Present1 still works after restore");

        let _ = DestroyWindow(hwnd);
    }
}
