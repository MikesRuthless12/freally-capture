//! The X11 child-window overlay that hosts the GPU preview surface **above** the
//! WebKitGTK widget — the Linux analog of the Windows DComp visual / macOS
//! CAMetalLayer.
//!
//! WebKitGTK renders into an ordinary GTK/X11 widget (not a compositor-hosted
//! surface like WebView2 / WKWebView), so a plain X11 **child window** raised
//! over it composites on top — the child-window approach that renders black on
//! Windows works here. We create the child on our **own** Xlib display
//! connection (`XInitThreads` first, so wgpu can present from the render thread
//! while GTK owns the main thread on its separate connection), parented to the
//! Tauri window's X11 id (window ids are server-global, so cross-connection
//! parenting is fine), and hand wgpu the child + our display via
//! `SurfaceTargetUnsafe::RawHandle` (`RawWindowHandle::Xlib`).
//!
//! Wayland is **not** handled here (a subsurface is a separate design + needs a
//! Wayland session to verify); the app keeps the JPEG path there. X11 uses a
//! **top-left** origin like the preview `Bounds`, so no Y-flip. This is the
//! crate's Linux `unsafe`, kept small and audited — `fcap-compositor` and the
//! app stay `#![forbid(unsafe_code)]`.

use std::ffi::c_void;

use x11_dl::xlib::{Display, Visual, Xlib};

use crate::{Bounds, PreviewError};

/// The X11 child-window overlay: a window raised over the WebKitGTK widget,
/// showing the GPU preview. Owns its own Xlib connection. Lives on the UI thread
/// (the render thread only reads the child id + display via the handle).
pub struct LinuxX11Overlay {
    /// Our loaded libX11 entry points.
    xlib: Xlib,
    /// Our **own** display connection (not GTK's) — closed on drop.
    display: *mut Display,
    /// The child window we created over the preview region.
    child: u64,
    screen: i32,
    visual_id: u64,
}

impl LinuxX11Overlay {
    /// Bring up the child window under `parent` (the Tauri window's X11 id),
    /// positioned at `bounds` (physical px, parent-client top-left).
    pub fn create(parent: u64, bounds: Bounds) -> Result<Self, PreviewError> {
        if parent == 0 {
            return Err(PreviewError::Os("null X11 parent window".into()));
        }
        let xlib = Xlib::open().map_err(|err| PreviewError::Os(format!("libX11: {err}")))?;
        // SAFETY: standard Xlib bring-up on our own connection; every returned
        // handle is null-checked before use.
        unsafe {
            // Thread-safe locking for this connection (the render thread presents
            // while the UI thread repositions).
            (xlib.XInitThreads)();
            let display = (xlib.XOpenDisplay)(std::ptr::null());
            if display.is_null() {
                return Err(PreviewError::Os("XOpenDisplay failed (no DISPLAY?)".into()));
            }
            let screen = (xlib.XDefaultScreen)(display);
            let background = (xlib.XBlackPixel)(display, screen);
            let child = (xlib.XCreateSimpleWindow)(
                display,
                parent,
                bounds.x,
                bounds.y,
                bounds.width.max(1),
                bounds.height.max(1),
                0,          // border width
                0,          // border pixel
                background, // background pixel (wgpu paints over it)
            );
            if child == 0 {
                (xlib.XCloseDisplay)(display);
                return Err(PreviewError::Os("XCreateSimpleWindow failed".into()));
            }
            (xlib.XMapWindow)(display, child);
            (xlib.XRaiseWindow)(display, child);
            (xlib.XFlush)(display);

            let visual: *mut Visual = (xlib.XDefaultVisual)(display, screen);
            let visual_id = if visual.is_null() {
                0
            } else {
                (xlib.XVisualIDFromVisual)(visual)
            };

            Ok(Self {
                xlib,
                display,
                child,
                screen,
                visual_id,
            })
        }
    }

    /// The child window id, for wgpu's `RawWindowHandle::Xlib`.
    pub fn child_window(&self) -> u64 {
        self.child
    }

    /// Our Xlib `Display*`, for wgpu's `RawDisplayHandle::Xlib`.
    pub fn display_ptr(&self) -> *mut c_void {
        self.display.cast()
    }

    pub fn screen(&self) -> i32 {
        self.screen
    }

    pub fn visual_id(&self) -> u64 {
        self.visual_id
    }

    /// Reposition the child over the preview region (UI thread). No Y-flip — X11
    /// and the preview `Bounds` are both top-left.
    pub fn set_bounds(&self, bounds: Bounds) {
        // SAFETY: display + child are live; XInitThreads made this connection
        // safe to touch alongside the render thread's present.
        unsafe {
            (self.xlib.XMoveResizeWindow)(
                self.display,
                self.child,
                bounds.x,
                bounds.y,
                bounds.width.max(1),
                bounds.height.max(1),
            );
            (self.xlib.XRaiseWindow)(self.display, self.child);
            (self.xlib.XFlush)(self.display);
        }
    }

    /// Show/hide by mapping/unmapping the child (UI thread).
    pub fn set_visible(&self, visible: bool) {
        // SAFETY: display + child are live.
        unsafe {
            if visible {
                (self.xlib.XMapWindow)(self.display, self.child);
                (self.xlib.XRaiseWindow)(self.display, self.child);
            } else {
                (self.xlib.XUnmapWindow)(self.display, self.child);
            }
            (self.xlib.XFlush)(self.display);
        }
    }

    /// Flush pending X requests so a geometry/visibility change is applied.
    pub fn commit(&self) {
        // SAFETY: display is live.
        unsafe {
            (self.xlib.XFlush)(self.display);
        }
    }
}

impl Drop for LinuxX11Overlay {
    fn drop(&mut self) {
        // SAFETY: display + child are live; destroy the child then close our
        // own connection (which also frees anything left on it).
        unsafe {
            (self.xlib.XDestroyWindow)(self.display, self.child);
            (self.xlib.XFlush)(self.display);
            (self.xlib.XCloseDisplay)(self.display);
        }
    }
}
