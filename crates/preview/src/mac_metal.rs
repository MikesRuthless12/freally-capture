//! The `CAMetalLayer` overlay that hosts the GPU preview surface **above** the
//! WKWebView — the macOS analog of the Windows DirectComposition overlay.
//!
//! Tauri's webview is a `WKWebView` inside the window's content view. We create
//! a `CAMetalLayer`, add it as a sublayer of the content view's root layer with
//! a high `zPosition` so Core Animation composites it above the webview's
//! sibling layer, and hand wgpu the layer pointer via
//! `SurfaceTargetUnsafe::CoreAnimationLayer`. wgpu draws straight into the layer
//! — no readback → JPEG round trip ("OBS feel").
//!
//! All AppKit / Core Animation objects are created + mutated on the **main
//! thread** (Tauri `setup` + the app's UI-thread region updates). Only the
//! layer's raw pointer crosses to the render thread (in a Send
//! [`CompositionHandle`](crate::CompositionHandle)) to build + present the wgpu
//! surface. This is the crate's macOS `unsafe`, kept small and audited — the
//! compositor + app stay `#![forbid(unsafe_code)]`.
//!
//! Everything goes through raw `msg_send!` (the crate is the designated unsafe
//! isolation layer, exactly like the Windows COM path): standard AppKit
//! selectors on the `NSWindow`/`NSView`/`CALayer`, so there are no typed-binding
//! feature flags to track. `NSWindow`/`NSView` are owned by Tauri and outlive
//! the overlay; the `CAMetalLayer` is retained here and detached on drop.

use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::{class, msg_send};
use objc2_core_foundation::{CGPoint, CGRect, CGSize};

use crate::{Bounds, PreviewError};

// QuartzCore (CAMetalLayer) is linked by wgpu's Metal backend; link it here too
// so `class!(CAMetalLayer)` resolves even if that ever changes. AppKit is linked
// by Tauri's webview. A duplicate framework link is a no-op for the linker.
#[link(name = "QuartzCore", kind = "framework")]
extern "C" {}

/// The `CAMetalLayer` overlay: a metal layer composited above the WKWebView,
/// showing the GPU preview. Lives on the main thread.
pub struct MacMetalOverlay {
    /// The metal layer wgpu draws into (a sublayer of the content view's layer).
    /// Retained here so it outlives the wgpu surface built from it.
    layer: Retained<AnyObject>,
    /// The Tauri main window's `NSWindow` — **non-owning** (Tauri owns it and it
    /// outlives this overlay). Re-queried each reposition for its content view's
    /// current size + backing scale, so geometry stays correct across resizes /
    /// monitor moves.
    ns_window: *mut AnyObject,
}

impl MacMetalOverlay {
    /// Bring up the overlay on `ns_window` (the Tauri main window's `NSWindow`
    /// pointer), with the layer positioned at `bounds` (physical px, top-left).
    pub fn create(ns_window: isize, bounds: Bounds) -> Result<Self, PreviewError> {
        if ns_window == 0 {
            return Err(PreviewError::Os("null NSWindow".into()));
        }
        let ns_window = ns_window as *mut AnyObject;
        // SAFETY: `ns_window` is the live Tauri main window's NSWindow; every
        // selector below is a standard AppKit / Core Animation message sent on
        // the main thread.
        unsafe {
            let content_view: *mut AnyObject = msg_send![ns_window, contentView];
            if content_view.is_null() {
                return Err(PreviewError::Os("NSWindow has no contentView".into()));
            }
            // Make the content view layer-backed so it has a root CALayer we can
            // add our sublayer to (the WKWebView subview is already layer-backed).
            let _: () = msg_send![content_view, setWantsLayer: true];
            let view_layer: *mut AnyObject = msg_send![content_view, layer];
            if view_layer.is_null() {
                return Err(PreviewError::Os("contentView has no layer".into()));
            }

            let layer: Retained<AnyObject> = msg_send![class!(CAMetalLayer), new];
            // Composite above the webview's sibling layer.
            let _: () = msg_send![&*layer, setZPosition: 1000.0f64];
            // Add before positioning; `apply_frame` sizes it to the region.
            let _: () = msg_send![view_layer, addSublayer: &*layer];

            let overlay = Self { layer, ns_window };
            overlay.apply_frame(bounds);
            Ok(overlay)
        }
    }

    /// The `CAMetalLayer`'s raw pointer, for wgpu's `CoreAnimationLayer` target.
    pub fn layer_ptr(&self) -> *mut core::ffi::c_void {
        Retained::as_ptr(&self.layer) as *mut core::ffi::c_void
    }

    /// Position + scale the layer over the preview region. `Bounds` are physical
    /// px with a **top-left** origin (webview / DOM space); a non-flipped NSView
    /// layer is **bottom-left**, so flip Y and convert px → points via the
    /// window's backing scale. Re-queries the content view each call so a resize
    /// or a move to a different-DPI display stays correct.
    fn apply_frame(&self, bounds: Bounds) {
        // SAFETY: `ns_window` is live; standard AppKit / Core Animation messages
        // on the main thread.
        unsafe {
            let content_view: *mut AnyObject = msg_send![self.ns_window, contentView];
            if content_view.is_null() {
                return;
            }
            let scale: f64 = msg_send![self.ns_window, backingScaleFactor];
            let scale = if scale > 0.0 { scale } else { 1.0 };
            let view_frame: CGRect = msg_send![content_view, frame];

            let w = f64::from(bounds.width) / scale;
            let h = f64::from(bounds.height) / scale;
            let x = f64::from(bounds.x) / scale;
            let top = f64::from(bounds.y) / scale;
            let y = view_frame.size.height - top - h; // top-left → bottom-left

            let rect = CGRect {
                origin: CGPoint { x, y },
                size: CGSize {
                    width: w,
                    height: h,
                },
            };
            let _: () = msg_send![&*self.layer, setContentsScale: scale];
            let _: () = msg_send![&*self.layer, setFrame: rect];
        }
    }

    /// Reposition the layer over the preview region (main thread).
    pub fn set_bounds(&self, bounds: Bounds) {
        self.apply_frame(bounds);
    }

    /// Show/hide the layer (main thread).
    pub fn set_visible(&self, visible: bool) {
        // SAFETY: `layer` is live; main thread.
        unsafe {
            let _: () = msg_send![&*self.layer, setHidden: !visible];
        }
    }

    /// No-op on macOS: Core Animation composites the layer at display refresh
    /// once wgpu presents a drawable — unlike Windows DirectComposition, which
    /// needs an explicit device `Commit`. Kept for interface parity.
    pub fn commit(&self) {}
}

impl Drop for MacMetalOverlay {
    fn drop(&mut self) {
        // Detach the layer from its superlayer so nothing dangles; the retained
        // layer then releases by refcount.
        // SAFETY: `layer` is live; main thread.
        unsafe {
            let _: () = msg_send![&*self.layer, removeFromSuperlayer];
        }
    }
}
