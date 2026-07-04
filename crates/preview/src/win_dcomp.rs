//! The DirectComposition overlay that hosts the GPU preview surface **above**
//! WebView2.
//!
//! WebView2 is itself DirectComposition-hosted, so a plain `WS_CHILD` window
//! (see `win.rs`) composites *behind* it — the dead end the child-window spike
//! hit. The fix: our own DComp **target** created `topmost` on the Tauri
//! window, whose **visual** we hand to wgpu via
//! `SurfaceTargetUnsafe::CompositionVisual`. wgpu builds a composition
//! swapchain and binds it to the visual; DComp then composites it above the
//! window's children (WebView2 included).
//!
//! All COM objects are created + mutated on the **UI thread**. Only the
//! visual's raw pointer crosses to the render thread (in a Send
//! [`CompositionHandle`]) so the render thread can build + present the wgpu
//! surface there. The device is `Commit`ted on the UI thread after each
//! geometry change. This is the crate's DirectComposition `unsafe`, kept small
//! and audited.

use windows::core::Interface;
use windows::Win32::Foundation::{HMODULE, HWND};
use windows::Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE_HARDWARE, D3D_DRIVER_TYPE_WARP};
use windows::Win32::Graphics::Direct3D11::{
    D3D11CreateDevice, ID3D11Device, D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_SDK_VERSION,
};
use windows::Win32::Graphics::DirectComposition::{
    DCompositionCreateDevice3, IDCompositionDesktopDevice, IDCompositionTarget,
    IDCompositionVisual2,
};
use windows::Win32::Graphics::Dxgi::{IDXGIAdapter, IDXGIDevice};

use crate::{Bounds, PreviewError};

/// The DirectComposition overlay: a topmost target on the Tauri window and the
/// visual that shows the GPU preview above WebView2. Lives on the UI thread.
pub struct WinDCompOverlay {
    // Fields drop top-to-bottom: visual + target before the device.
    visual: IDCompositionVisual2,
    target: IDCompositionTarget,
    device: IDCompositionDesktopDevice,
    // Keeps the D3D11 device that backs the DComp device alive.
    _d3d: ID3D11Device,
}

impl WinDCompOverlay {
    /// Bring up the overlay on `parent_hwnd` (the Tauri main window), with the
    /// visual positioned at `bounds`.
    pub fn create(parent_hwnd: isize, bounds: Bounds) -> Result<Self, PreviewError> {
        let parent = HWND(parent_hwnd as *mut core::ffi::c_void);
        // SAFETY: standard DirectComposition bring-up; every call is checked
        // and `parent` is the live Tauri main window.
        unsafe {
            let d3d = create_d3d11_device()?;
            let dxgi: IDXGIDevice = d3d
                .cast()
                .map_err(|e| PreviewError::Os(format!("IDXGIDevice cast: {e}")))?;
            let device: IDCompositionDesktopDevice = DCompositionCreateDevice3(&dxgi)
                .map_err(|e| PreviewError::Os(format!("DCompositionCreateDevice3: {e}")))?;
            // topmost = TRUE → the visual tree composites above the window's
            // children (WebView2 included).
            let target = device
                .CreateTargetForHwnd(parent, true)
                .map_err(|e| PreviewError::Os(format!("CreateTargetForHwnd: {e}")))?;
            let visual = device
                .CreateVisual()
                .map_err(|e| PreviewError::Os(format!("CreateVisual: {e}")))?;
            visual
                .SetOffsetX2(bounds.x as f32)
                .map_err(|e| PreviewError::Os(format!("SetOffsetX: {e}")))?;
            visual
                .SetOffsetY2(bounds.y as f32)
                .map_err(|e| PreviewError::Os(format!("SetOffsetY: {e}")))?;
            target
                .SetRoot(&visual)
                .map_err(|e| PreviewError::Os(format!("SetRoot: {e}")))?;
            device
                .Commit()
                .map_err(|e| PreviewError::Os(format!("Commit: {e}")))?;
            Ok(Self {
                visual,
                target,
                device,
                _d3d: d3d,
            })
        }
    }

    /// The composition visual's raw COM pointer, for wgpu's
    /// `SurfaceTargetUnsafe::CompositionVisual`.
    pub fn visual_ptr(&self) -> *mut core::ffi::c_void {
        self.visual.as_raw()
    }

    /// Reposition the visual over the preview region (UI thread). The surface
    /// *size* (swapchain) is resized on the render thread via wgpu.
    pub fn set_bounds(&self, bounds: Bounds) {
        // SAFETY: `visual` + `device` are live; DComp calls are UI-thread safe.
        unsafe {
            let _ = self.visual.SetOffsetX2(bounds.x as f32);
            let _ = self.visual.SetOffsetY2(bounds.y as f32);
            let _ = self.device.Commit();
        }
    }

    /// Show/hide by attaching/detaching the visual from the target root.
    pub fn set_visible(&self, visible: bool) {
        // SAFETY: `target` + `visual` are live; UI-thread safe.
        unsafe {
            if visible {
                let _ = self.target.SetRoot(&self.visual);
            } else {
                let _ = self.target.SetRoot(None);
            }
            let _ = self.device.Commit();
        }
    }
}

impl Drop for WinDCompOverlay {
    fn drop(&mut self) {
        // Detach the root so nothing dangles mid-composite; the COM objects
        // then release by refcount.
        // SAFETY: live objects, UI thread.
        unsafe {
            let _ = self.target.SetRoot(None);
            let _ = self.device.Commit();
        }
    }
}

/// A lightweight D3D11 device to back the DComp device (hardware, WARP
/// fallback). BGRA support is required by DirectComposition.
unsafe fn create_d3d11_device() -> Result<ID3D11Device, PreviewError> {
    for driver in [D3D_DRIVER_TYPE_HARDWARE, D3D_DRIVER_TYPE_WARP] {
        let mut device: Option<ID3D11Device> = None;
        let result = D3D11CreateDevice(
            None::<&IDXGIAdapter>,
            driver,
            HMODULE::default(),
            D3D11_CREATE_DEVICE_BGRA_SUPPORT,
            None,
            D3D11_SDK_VERSION,
            Some(&mut device),
            None,
            None,
        );
        if result.is_ok() {
            if let Some(device) = device {
                return Ok(device);
            }
        }
    }
    Err(PreviewError::Os("D3D11CreateDevice failed".into()))
}
