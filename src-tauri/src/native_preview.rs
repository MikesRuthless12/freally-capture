//! The app side of the native preview surface (the "OBS feel" path).
//!
//! A **DirectComposition overlay** is created on the **main thread** in `setup`
//! (a topmost composition target on the Tauri window, so its visual composites
//! *above* WebView2). Its Send [`CompositionHandle`] is stashed here so the
//! studio render thread can build + present the wgpu surface. The UI reports
//! the preview region's rectangle (physical px) via `native_preview_set_region`,
//! which repositions the overlay's visual and signals the render thread to
//! reconfigure the surface. Off Windows (or if creation fails) the overlay is
//! absent and the app keeps the JPEG `preview://` path — the render thread
//! checks [`NativePreviewState::composition_handle`] and simply does neither.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Mutex;

use fcap_preview::{Bounds, CompositionHandle, CompositionOverlay};
use fcap_scene::ItemId;
use tauri::{AppHandle, Runtime};
// `Manager` (for `app.state` / `app.get_webview_window`) is only needed by the
// native overlay bring-up in `try_create` (Windows, macOS, Linux/X11).
#[cfg(any(windows, target_os = "macos", target_os = "linux"))]
use tauri::Manager;

/// Tauri-managed native-preview state, shared main-thread ↔ render-thread.
pub struct NativePreviewState {
    /// The DirectComposition overlay (Windows only, when creation succeeded).
    /// Held here so it outlives the surface; owns the visual on the UI thread.
    overlay: Mutex<Option<CompositionOverlay>>,
    /// The composition handle for the render thread (build + present).
    handle: Mutex<Option<CompositionHandle>>,
    /// The preview region in physical px (parent-client relative).
    rect: Mutex<Bounds>,
    /// Bumped on every region change so the render thread reconfigures.
    rect_gen: AtomicU64,
    /// The region is currently shown (hidden while a modal covers it).
    visible: AtomicBool,
    /// The native GPU preview is actually viable: the compositor is on DX12 and
    /// the overlay was created, and no runtime surface failure has knocked it
    /// out. The UI reads this (via `native_preview_active`) to decide whether to
    /// hide its JPEG canvas — set by the render thread, which is the only place
    /// that knows the backend and whether the surface presents.
    viable: AtomicBool,
    /// The UI's selected item — drawn as the native preview's selection box.
    selection: Mutex<Option<ItemId>>,
}

impl NativePreviewState {
    pub fn new() -> Self {
        Self {
            overlay: Mutex::new(None),
            handle: Mutex::new(None),
            rect: Mutex::new(Bounds::default()),
            rect_gen: AtomicU64::new(0),
            visible: AtomicBool::new(false),
            viable: AtomicBool::new(false),
            selection: Mutex::new(None),
        }
    }

    fn lock_overlay(&self) -> std::sync::MutexGuard<'_, Option<CompositionOverlay>> {
        self.overlay
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// Install the overlay + its composition handle (main thread, in setup).
    /// Windows, macOS, and Linux/X11 construct the overlay in `try_create`; where
    /// it fails (or on Wayland) the app keeps the JPEG path and installs none.
    #[cfg(any(windows, target_os = "macos", target_os = "linux"))]
    pub fn install(&self, overlay: CompositionOverlay, handle: CompositionHandle) {
        *self
            .handle
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(handle);
        *self.lock_overlay() = Some(overlay);
    }

    /// The composition handle, if the native path is available (Windows + created).
    pub fn composition_handle(&self) -> Option<CompositionHandle> {
        *self
            .handle
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// Commit the overlay's composition tree — the render thread calls this
    /// right after building the surface, so wgpu's `SetContent` is composited
    /// (otherwise the visual stays blank until the next region change).
    pub fn commit(&self) {
        if let Some(overlay) = self.lock_overlay().as_ref() {
            overlay.commit();
        }
    }

    /// The UI's selected item, drawn as the native preview's selection box
    /// (`None` when nothing is selected). Read by the render thread each tick.
    pub fn selection(&self) -> Option<ItemId> {
        *self
            .selection
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// The UI reported which item is selected (or none). A no-op visual on the
    /// JPEG path; on the native path the render thread draws its box + handles.
    pub fn set_selection(&self, item: Option<ItemId>) {
        *self
            .selection
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = item;
    }

    /// Whether the region is currently visible (the render thread only presents
    /// while it is — a hidden overlay has nothing to show).
    pub fn is_visible(&self) -> bool {
        self.visible.load(Ordering::Relaxed)
    }

    /// Whether the native GPU preview is viable (DX12 + overlay + not failed).
    /// The UI hides its JPEG canvas only when this is true, so a non-DX12
    /// machine or a lost surface transparently keeps the JPEG fallback.
    pub fn is_viable(&self) -> bool {
        self.viable.load(Ordering::Relaxed)
    }

    /// The render thread reports whether the native path is currently viable
    /// (set once it knows the backend; cleared on a surface build/present error).
    pub fn set_viable(&self, viable: bool) {
        self.viable.store(viable, Ordering::Relaxed);
    }

    /// The render thread's reconcile key: the current region + its generation.
    /// Read the generation *while still holding the rect lock* so the pair is
    /// always consistent — `set_region` bumps the generation under the same lock.
    pub fn region(&self) -> (u64, Bounds) {
        let rect = self
            .rect
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        (self.rect_gen.load(Ordering::Acquire), *rect)
    }

    /// The UI reported a new preview region (position/size in physical px) and
    /// visibility. Repositions the overlay's visual and signals a surface resize.
    pub fn set_region(&self, bounds: Bounds, visible: bool) {
        {
            let mut rect = self
                .rect
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            *rect = bounds;
            // Bump the generation under the same lock so `region()` can never
            // pair a new generation with stale bounds (torn read).
            self.rect_gen.fetch_add(1, Ordering::Release);
        }
        self.visible.store(visible, Ordering::Relaxed);
        if let Some(overlay) = self.lock_overlay().as_ref() {
            overlay.set_bounds(bounds);
            overlay.set_visible(visible);
        }
    }
}

impl Default for NativePreviewState {
    fn default() -> Self {
        Self::new()
    }
}

/// Create the native preview overlay (main thread, in setup) — a
/// DirectComposition visual on Windows, a `CAMetalLayer` on macOS, an X11 child
/// window on Linux — and install it into [`NativePreviewState`]. Any failure —
/// or Wayland — is logged honestly and leaves the state empty, so the app keeps
/// the JPEG `preview://` path.
pub fn try_create<R: Runtime>(app: &AppHandle<R>) {
    #[cfg(windows)]
    {
        let Some(window) = app.get_webview_window("main") else {
            eprintln!("native preview: no main window — using the JPEG preview");
            return;
        };
        let parent = match window.hwnd() {
            Ok(hwnd) => hwnd.0 as isize,
            Err(err) => {
                eprintln!("native preview: no window handle ({err}) — using the JPEG preview");
                return;
            }
        };
        match CompositionOverlay::create(parent, Bounds::default()) {
            Ok(overlay) => {
                let handle = overlay.handle();
                app.state::<NativePreviewState>().install(overlay, handle);
                println!("native preview: DirectComposition overlay created (GPU surface path)");
            }
            Err(err) => {
                eprintln!("native preview unavailable ({err}) — using the JPEG preview")
            }
        }
    }
    #[cfg(target_os = "macos")]
    {
        let Some(window) = app.get_webview_window("main") else {
            eprintln!("native preview: no main window — using the JPEG preview");
            return;
        };
        let parent = match window.ns_window() {
            Ok(ptr) => ptr as isize,
            Err(err) => {
                eprintln!("native preview: no NSWindow ({err}) — using the JPEG preview");
                return;
            }
        };
        match CompositionOverlay::create(parent, Bounds::default()) {
            Ok(overlay) => {
                let handle = overlay.handle();
                app.state::<NativePreviewState>().install(overlay, handle);
                println!("native preview: CAMetalLayer overlay created (GPU surface path)");
            }
            Err(err) => {
                eprintln!("native preview unavailable ({err}) — using the JPEG preview")
            }
        }
    }
    #[cfg(target_os = "linux")]
    {
        use raw_window_handle::{HasWindowHandle, RawWindowHandle};
        let Some(window) = app.get_webview_window("main") else {
            eprintln!("native preview: no main window — using the JPEG preview");
            return;
        };
        println!("native preview: linux try_create — reading the X11 window handle");
        // The Tauri window's X11 id (Xlib). On Wayland the handle is Wayland-
        // shaped, so keep the JPEG path there (an X11 child window doesn't apply).
        let parent = match window.window_handle() {
            Ok(handle) => match handle.as_raw() {
                RawWindowHandle::Xlib(xlib) => xlib.window as isize,
                _ => {
                    eprintln!("native preview: not X11 (Wayland?) — using the JPEG preview");
                    return;
                }
            },
            Err(err) => {
                eprintln!("native preview: no window handle ({err}) — using the JPEG preview");
                return;
            }
        };
        println!("native preview: X11 parent {parent:#x} — creating the child window");
        match CompositionOverlay::create(parent, Bounds::default()) {
            Ok(overlay) => {
                let handle = overlay.handle();
                app.state::<NativePreviewState>().install(overlay, handle);
                println!("native preview: X11 child window created (GPU surface path)");
            }
            Err(err) => {
                eprintln!("native preview unavailable ({err}) — using the JPEG preview")
            }
        }
    }
    #[cfg(not(any(windows, target_os = "macos", target_os = "linux")))]
    {
        let _ = app; // JPEG preview path only
    }
}
