//! # fcap-preview
//!
//! The **native preview overlay**: a topmost DirectComposition visual placed
//! over the webview's preview region, whose composition surface hosts the
//! compositor's GPU preview (`fcap_compositor::NativePreview`). This is what
//! gives the "OBS feel" — the composed program frame is painted on the GPU
//! directly *above* WebView2 (itself DirectComposition-hosted), with no
//! readback → JPEG → webview-canvas round trip.
//!
//! **Platform reality, told honestly:** implemented on **Windows** (a
//! DirectComposition overlay). On macOS/Linux the constructors return
//! [`PreviewError::Unsupported`] and the app keeps the cross-platform JPEG
//! `preview://` path. The unavoidable native `unsafe` (COM / DirectComposition
//! plus the one `create_surface_unsafe`) is isolated in this crate so
//! `fcap-compositor` and the app can stay `#![forbid(unsafe_code)]`.
//!
//! ## Threading
//!
//! The overlay is created + repositioned on the **UI (main) thread** (the only
//! thread with the message pump). A Send [`CompositionHandle`] (a raw COM
//! pointer to the visual) is handed to the **render thread** to build + present
//! the wgpu surface there. Geometry changes flow UI → render via the app.

#![cfg_attr(not(target_os = "windows"), allow(unused_variables))]

use thiserror::Error;

#[cfg(target_os = "windows")]
mod win_dcomp;

/// A rectangle in **physical pixels**, relative to the parent window's client
/// area — the preview region the webview reserves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Bounds {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Error)]
pub enum PreviewError {
    #[error("the native preview overlay is not supported on this platform yet")]
    Unsupported,
    #[error("native window error: {0}")]
    Os(String),
}

/// A Send handle to the composition visual, for building the wgpu preview
/// surface on the render thread. The DirectComposition overlay hosts the GPU
/// preview *above* WebView2 (Windows); off Windows it is never produced.
#[derive(Debug, Clone, Copy)]
pub struct CompositionHandle {
    #[cfg(target_os = "windows")]
    visual: *mut core::ffi::c_void,
    #[cfg(not(target_os = "windows"))]
    _unsupported: (),
}

// SAFETY: the pointer is a COM interface pointer to a visual owned by the
// `CompositionOverlay`, which the app guarantees outlives every surface built
// from it. The render thread only reads it to construct the surface (the
// documented wgpu `CompositionVisual` pattern); the visual's methods stay on
// the UI thread.
unsafe impl Send for CompositionHandle {}
unsafe impl Sync for CompositionHandle {}

impl CompositionHandle {
    /// Build the wgpu preview surface on the composition visual, using the
    /// compositor's own `wgpu::Instance` (so it validates against the same
    /// adapter). The one place the unsafe `CompositionVisual` target lives —
    /// `fcap-compositor` and the app both `#![forbid(unsafe_code)]`.
    pub fn create_surface(
        &self,
        instance: &wgpu::Instance,
    ) -> Result<wgpu::Surface<'static>, PreviewError> {
        #[cfg(target_os = "windows")]
        {
            let target = wgpu::SurfaceTargetUnsafe::CompositionVisual(self.visual);
            // SAFETY: the visual is owned by the `CompositionOverlay`, which the
            // app guarantees outlives the surface; wgpu increments its refcount.
            unsafe { instance.create_surface_unsafe(target) }
                .map_err(|err| PreviewError::Os(format!("composition surface: {err}")))
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = instance;
            Err(PreviewError::Unsupported)
        }
    }
}

/// The native preview DirectComposition overlay: a topmost composition target
/// on the Tauri window whose visual shows the GPU preview above WebView2.
/// Dropping it tears the overlay down. Windows only; off Windows
/// [`CompositionOverlay::create`] returns [`PreviewError::Unsupported`].
pub struct CompositionOverlay {
    #[cfg(target_os = "windows")]
    inner: win_dcomp::WinDCompOverlay,
    #[cfg(not(target_os = "windows"))]
    _unsupported: (),
}

// SAFETY: the overlay owns DirectComposition COM objects created on the UI
// thread; the app routes creation + geometry through the main thread and only
// hands the render thread a `CompositionHandle` (a raw pointer). Holding it in
// shared state is sound under that discipline.
unsafe impl Send for CompositionOverlay {}
unsafe impl Sync for CompositionOverlay {}

impl CompositionOverlay {
    /// Bring up the overlay on `parent_hwnd` (the Tauri main window), with the
    /// visual positioned at `bounds`. Errors with [`PreviewError::Unsupported`]
    /// off Windows.
    pub fn create(parent_hwnd: isize, bounds: Bounds) -> Result<Self, PreviewError> {
        #[cfg(target_os = "windows")]
        {
            Ok(Self {
                inner: win_dcomp::WinDCompOverlay::create(parent_hwnd, bounds)?,
            })
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = (parent_hwnd, bounds);
            Err(PreviewError::Unsupported)
        }
    }

    /// A Send handle for the render thread to build the wgpu surface.
    pub fn handle(&self) -> CompositionHandle {
        #[cfg(target_os = "windows")]
        {
            CompositionHandle {
                visual: self.inner.visual_ptr(),
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            unreachable!("CompositionOverlay never constructs off Windows")
        }
    }

    /// Reposition the visual over the preview region (UI thread).
    pub fn set_bounds(&self, bounds: Bounds) {
        #[cfg(target_os = "windows")]
        self.inner.set_bounds(bounds);
        #[cfg(not(target_os = "windows"))]
        let _ = bounds;
    }

    /// Show or hide the overlay (UI thread) — hidden while a modal dialog needs
    /// to cover the preview region.
    pub fn set_visible(&self, visible: bool) {
        #[cfg(target_os = "windows")]
        self.inner.set_visible(visible);
        #[cfg(not(target_os = "windows"))]
        let _ = visible;
    }

    /// Commit the composition tree — call once after the surface is created so
    /// wgpu's `SetContent` is composited (else the visual stays blank until the
    /// next geometry change).
    pub fn commit(&self) {
        #[cfg(target_os = "windows")]
        self.inner.commit();
    }
}

/// This crate's version (inherited from the workspace).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
