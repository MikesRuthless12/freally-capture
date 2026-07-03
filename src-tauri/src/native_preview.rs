//! The app side of the native preview surface (the "OBS feel" path).
//!
//! The child window is created on the **main thread** in `setup` (parented to
//! the Tauri window); its Send [`SurfaceHandle`] is stashed here so the studio
//! render thread can create + present the wgpu surface. The UI reports the
//! preview region's rectangle (physical px) via `native_preview_set_region`,
//! which repositions the child window and signals the render thread to
//! reconfigure the surface. Off Windows (or if creation fails) the window is
//! absent and the app keeps the JPEG `preview://` path — the render thread
//! checks [`NativePreviewState::surface_handle`] and simply does neither.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Mutex;

use fcap_preview::{Bounds, PreviewWindow, SurfaceHandle};
use tauri::{AppHandle, Manager, Runtime};

/// Tauri-managed native-preview state, shared main-thread ↔ render-thread.
pub struct NativePreviewState {
    /// The child window (Windows only, when creation succeeded). Held here so
    /// it outlives the surface; `PreviewWindow` is Send by its own contract.
    window: Mutex<Option<PreviewWindow>>,
    /// The surface target for the render thread (present).
    handle: Mutex<Option<SurfaceHandle>>,
    /// The preview region in physical px (parent-client relative).
    rect: Mutex<Bounds>,
    /// Bumped on every region change so the render thread reconfigures.
    rect_gen: AtomicU64,
    /// The region is currently shown (hidden while a modal covers it).
    visible: AtomicBool,
}

impl NativePreviewState {
    pub fn new() -> Self {
        Self {
            window: Mutex::new(None),
            handle: Mutex::new(None),
            rect: Mutex::new(Bounds::default()),
            rect_gen: AtomicU64::new(0),
            visible: AtomicBool::new(false),
        }
    }

    fn lock_window(&self) -> std::sync::MutexGuard<'_, Option<PreviewWindow>> {
        self.window
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// Install the child window + its surface handle (main thread, in setup).
    pub fn install(&self, window: PreviewWindow, handle: SurfaceHandle) {
        *self
            .handle
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(handle);
        *self.lock_window() = Some(window);
    }

    /// The surface handle, if the native path is available (Windows + created).
    pub fn surface_handle(&self) -> Option<SurfaceHandle> {
        *self
            .handle
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// Whether the region is currently visible (the render thread only
    /// presents while it is — a hidden window has nothing to show).
    pub fn is_visible(&self) -> bool {
        self.visible.load(Ordering::Relaxed)
    }

    /// The render thread's reconcile key: the current region + its generation.
    pub fn region(&self) -> (u64, Bounds) {
        let rect = *self
            .rect
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        (self.rect_gen.load(Ordering::Acquire), rect)
    }

    /// The UI reported a new preview region (position/size in physical px) and
    /// visibility. Repositions the child window and signals a surface resize.
    pub fn set_region(&self, bounds: Bounds, visible: bool) {
        {
            let mut rect = self
                .rect
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            *rect = bounds;
        }
        self.rect_gen.fetch_add(1, Ordering::Release);
        self.visible.store(visible, Ordering::Relaxed);
        if let Some(window) = self.lock_window().as_ref() {
            window.set_bounds(bounds);
            window.set_visible(visible);
        }
    }
}

impl Default for NativePreviewState {
    fn default() -> Self {
        Self::new()
    }
}

/// Create the native preview child window (main thread, in setup) and install
/// it into [`NativePreviewState`]. Any failure — or a non-Windows OS — is
/// logged honestly and leaves the state empty, so the app keeps the JPEG
/// `preview://` path.
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
        match PreviewWindow::create(fcap_preview::ParentHandle::Win32(parent), Bounds::default()) {
            Ok(preview_window) => {
                let handle = preview_window.surface_handle();
                app.state::<NativePreviewState>()
                    .install(preview_window, handle);
                println!("native preview: child window created (GPU surface path)");
            }
            Err(err) => {
                eprintln!("native preview unavailable ({err}) — using the JPEG preview")
            }
        }
    }
    #[cfg(not(windows))]
    {
        let _ = app; // JPEG preview path only
    }
}
