//! # fcap-preview
//!
//! The **native preview child window**: a small OS child window placed over
//! the webview's preview region, whose handle hosts the compositor's GPU
//! preview surface (`fcap_compositor::NativePreview`). This is what gives the
//! "OBS feel" — the composed program frame is painted on the GPU directly to
//! a window, with no readback → JPEG → webview-canvas round trip.
//!
//! **Platform reality, told honestly:** implemented on **Windows** (a
//! `WS_CHILD` HWND) for now; on macOS/Linux [`PreviewWindow::create`] returns
//! [`PreviewError::Unsupported`] and the app keeps the cross-platform JPEG
//! `preview://` path. The unavoidable native-windowing `unsafe` is isolated
//! in the per-OS module.
//!
//! ## Threading
//!
//! The child window is created + repositioned on the **UI (main) thread**
//! (the only thread with the message pump). Its handle is handed to the
//! **render thread** as a [`SurfaceHandle`] (Send) to create + present the
//! wgpu surface there. Resizes flow UI → render via the app; the window
//! itself never renders.

#![cfg_attr(not(target_os = "windows"), allow(unused_variables))]

use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle,
};
use thiserror::Error;

#[cfg(target_os = "windows")]
mod win;
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

/// The parent window's native handle (the Tauri main window). Constructed by
/// the app from `tauri::window::Window`'s raw handle.
#[derive(Debug, Clone, Copy)]
pub enum ParentHandle {
    /// A Win32 `HWND` as an `isize`.
    Win32(isize),
}

#[derive(Debug, Error)]
pub enum PreviewError {
    #[error("the native preview window is not supported on this platform yet")]
    Unsupported,
    #[error("unexpected parent window handle for this platform")]
    WrongParent,
    #[error("native window error: {0}")]
    Os(String),
}

/// A Send handle to the child window, for creating the GPU surface on the
/// render thread. Implements the `raw-window-handle` traits so it can be
/// passed straight to `wgpu::Instance::create_surface`.
///
/// Safety contract: the [`PreviewWindow`] that produced this handle must
/// outlive the surface created from it. The app enforces this by dropping the
/// surface (on the render thread) before the window (on the UI thread) at
/// shutdown.
#[derive(Debug, Clone, Copy)]
pub struct SurfaceHandle {
    #[cfg(target_os = "windows")]
    hwnd: isize,
    #[cfg(target_os = "windows")]
    hinstance: isize,
}

// SAFETY: the handle is a plain window-handle value; it carries no
// thread-affine resources. Creating the surface on the render thread is the
// documented, supported wgpu pattern.
unsafe impl Send for SurfaceHandle {}
unsafe impl Sync for SurfaceHandle {}

/// The native preview child window. Dropping it destroys the OS window.
pub struct PreviewWindow {
    #[cfg(target_os = "windows")]
    inner: win::WinPreviewWindow,
    // A field so the struct is never empty on non-Windows (keeps the API
    // uniform); nothing reads it there.
    #[cfg(not(target_os = "windows"))]
    _unsupported: (),
}

// SAFETY: the window holds only an HWND — a process-global identifier, not a
// thread-owned resource. It is *created* on the UI thread (required), but the
// operations exposed here (`SetWindowPos`, `ShowWindow`, `DestroyWindow`) are
// callable from any thread for a window whose owning thread pumps messages
// (the Tauri main thread always does). This lets the app hold it in shared
// state; the app still routes creation through the main thread.
unsafe impl Send for PreviewWindow {}
unsafe impl Sync for PreviewWindow {}

impl PreviewWindow {
    /// Create the child window over `bounds`, parented to `parent`. Errors
    /// with [`PreviewError::Unsupported`] off Windows (the app falls back to
    /// the JPEG preview path).
    pub fn create(parent: ParentHandle, bounds: Bounds) -> Result<Self, PreviewError> {
        #[cfg(target_os = "windows")]
        {
            let ParentHandle::Win32(parent_hwnd) = parent;
            Ok(Self {
                inner: win::WinPreviewWindow::create(parent_hwnd, bounds)?,
            })
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = (parent, bounds);
            Err(PreviewError::Unsupported)
        }
    }

    /// A Send surface handle for the render thread.
    pub fn surface_handle(&self) -> SurfaceHandle {
        #[cfg(target_os = "windows")]
        {
            self.inner.surface_handle()
        }
        #[cfg(not(target_os = "windows"))]
        {
            unreachable!("PreviewWindow never constructs off Windows")
        }
    }

    /// Move/resize the child window to `bounds` (UI thread).
    pub fn set_bounds(&self, bounds: Bounds) {
        #[cfg(target_os = "windows")]
        self.inner.set_bounds(bounds);
    }

    /// Show or hide the child window (UI thread) — hidden while a modal
    /// dialog needs to overlay the preview region.
    pub fn set_visible(&self, visible: bool) {
        #[cfg(target_os = "windows")]
        self.inner.set_visible(visible);
    }
}

impl HasWindowHandle for SurfaceHandle {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        #[cfg(target_os = "windows")]
        {
            use raw_window_handle::{RawWindowHandle, Win32WindowHandle};
            let mut handle = Win32WindowHandle::new(
                std::num::NonZeroIsize::new(self.hwnd).ok_or(HandleError::Unavailable)?,
            );
            handle.hinstance = std::num::NonZeroIsize::new(self.hinstance);
            // SAFETY: `hwnd` is a live child window owned by the PreviewWindow,
            // which the app guarantees outlives every surface built from it.
            Ok(unsafe { WindowHandle::borrow_raw(RawWindowHandle::Win32(handle)) })
        }
        #[cfg(not(target_os = "windows"))]
        {
            Err(HandleError::NotSupported)
        }
    }
}

impl HasDisplayHandle for SurfaceHandle {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        #[cfg(target_os = "windows")]
        {
            // SAFETY: Windows has a single implicit display; the raw handle
            // carries no borrowed state.
            Ok(unsafe {
                DisplayHandle::borrow_raw(raw_window_handle::RawDisplayHandle::Windows(
                    raw_window_handle::WindowsDisplayHandle::new(),
                ))
            })
        }
        #[cfg(not(target_os = "windows"))]
        {
            Err(HandleError::NotSupported)
        }
    }
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
