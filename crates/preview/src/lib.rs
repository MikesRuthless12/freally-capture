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
//! DirectComposition overlay) and **macOS** (a `CAMetalLayer` composited above
//! the WKWebView). On Linux the constructors return
//! [`PreviewError::Unsupported`] and the app keeps the cross-platform JPEG
//! `preview://` path. The unavoidable native `unsafe` (Windows COM /
//! DirectComposition, macOS AppKit / Core Animation, plus the one
//! `create_surface_unsafe`) is isolated in this crate so `fcap-compositor` and
//! the app can stay `#![forbid(unsafe_code)]`.
//!
//! ## Threading
//!
//! The overlay is created + repositioned on the **UI (main) thread** (the only
//! thread with the message pump). A Send [`CompositionHandle`] (a raw COM
//! pointer to the visual) is handed to the **render thread** to build + present
//! the wgpu surface there. Geometry changes flow UI → render via the app.

#![cfg_attr(
    not(any(target_os = "windows", target_os = "macos")),
    allow(unused_variables)
)]

use thiserror::Error;

#[cfg(target_os = "windows")]
mod win_dcomp;

#[cfg(target_os = "macos")]
mod mac_metal;

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
    #[cfg(target_os = "macos")]
    layer: *mut core::ffi::c_void,
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    _unsupported: (),
}

// SAFETY: the pointer is a native compositor object owned by the
// `CompositionOverlay` — a COM `IDCompositionVisual` on Windows, a
// `CAMetalLayer` on macOS — which the app guarantees outlives every surface
// built from it. The render thread only reads it to construct the surface (the
// documented wgpu `CompositionVisual` / `CoreAnimationLayer` pattern); the
// object's own methods stay on the UI thread.
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
        #[cfg(target_os = "macos")]
        {
            let target = wgpu::SurfaceTargetUnsafe::CoreAnimationLayer(self.layer);
            // SAFETY: the CAMetalLayer is owned by the `CompositionOverlay`,
            // which the app guarantees outlives the surface; wgpu retains it.
            unsafe { instance.create_surface_unsafe(target) }
                .map_err(|err| PreviewError::Os(format!("core animation surface: {err}")))
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            let _ = instance;
            Err(PreviewError::Unsupported)
        }
    }
}

/// The native preview overlay: a topmost DirectComposition visual over WebView2
/// on Windows, a `CAMetalLayer` over the WKWebView on macOS — either way showing
/// the GPU preview above the webview. Dropping it tears the overlay down. On
/// Linux [`CompositionOverlay::create`] returns [`PreviewError::Unsupported`].
pub struct CompositionOverlay {
    #[cfg(target_os = "windows")]
    inner: win_dcomp::WinDCompOverlay,
    #[cfg(target_os = "macos")]
    inner: mac_metal::MacMetalOverlay,
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    _unsupported: (),
}

// SAFETY: the overlay owns native compositor objects created on the UI thread
// (DirectComposition COM on Windows; a retained CAMetalLayer + a non-owning
// NSWindow pointer on macOS); the app routes creation + geometry through the
// main thread and only hands the render thread a `CompositionHandle` (a raw
// pointer). Holding it in shared state is sound under that discipline.
unsafe impl Send for CompositionOverlay {}
unsafe impl Sync for CompositionOverlay {}

impl CompositionOverlay {
    /// Bring up the overlay on `parent` — the Tauri main window's native handle
    /// (an `HWND` on Windows, an `NSWindow*` on macOS) — with the surface
    /// positioned at `bounds`. Errors with [`PreviewError::Unsupported`] on Linux.
    pub fn create(parent: isize, bounds: Bounds) -> Result<Self, PreviewError> {
        #[cfg(target_os = "windows")]
        {
            Ok(Self {
                inner: win_dcomp::WinDCompOverlay::create(parent, bounds)?,
            })
        }
        #[cfg(target_os = "macos")]
        {
            Ok(Self {
                inner: mac_metal::MacMetalOverlay::create(parent, bounds)?,
            })
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            let _ = (parent, bounds);
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
        #[cfg(target_os = "macos")]
        {
            CompositionHandle {
                layer: self.inner.layer_ptr(),
            }
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            unreachable!("CompositionOverlay never constructs off Windows/macOS")
        }
    }

    /// Reposition the visual over the preview region (UI thread).
    pub fn set_bounds(&self, bounds: Bounds) {
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        self.inner.set_bounds(bounds);
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        let _ = bounds;
    }

    /// Show or hide the overlay (UI thread) — hidden while a modal dialog needs
    /// to cover the preview region.
    pub fn set_visible(&self, visible: bool) {
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        self.inner.set_visible(visible);
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        let _ = visible;
    }

    /// Commit the composition tree — call once after the surface is created so
    /// wgpu's `SetContent` is composited (else the visual stays blank until the
    /// next geometry change).
    pub fn commit(&self) {
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        self.inner.commit();
    }
}

/// This crate's version (inherited from the workspace).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
