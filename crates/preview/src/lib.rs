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

/// This crate's version (inherited from the workspace).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
