//! The Windows child window that hosts the GPU preview surface.
//!
//! A `WS_CHILD` HWND parented to the Tauri main window, placed over the
//! webview's preview region and z-ordered above the (sibling) WebView2 HWND
//! so the composited program frame shows there. This module is the crate's
//! only `unsafe`: Win32 window create / move / destroy, and it is kept small
//! and audited. All calls except surface use run on the UI (main) thread.

use std::sync::atomic::{AtomicBool, Ordering};

use windows::core::PCWSTR;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DestroyWindow, RegisterClassExW, SetWindowPos, ShowWindow,
    HMENU, SET_WINDOW_POS_FLAGS, SHOW_WINDOW_CMD, SWP_NOACTIVATE, SWP_NOZORDER, SW_HIDE, SW_SHOWNA,
    WINDOW_EX_STYLE, WM_ERASEBKGND, WNDCLASSEXW, WS_CHILD, WS_CLIPSIBLINGS, WS_VISIBLE,
};

use crate::{Bounds, PreviewError, SurfaceHandle};

/// UTF-16, null-terminated class name.
const CLASS_NAME: &[u16] = &[
    b'f' as u16,
    b'c' as u16,
    b'a' as u16,
    b'p' as u16,
    b'_' as u16,
    b'p' as u16,
    b'r' as u16,
    b'e' as u16,
    b'v' as u16,
    b'i' as u16,
    b'e' as u16,
    b'w' as u16,
    0,
];

static CLASS_REGISTERED: AtomicBool = AtomicBool::new(false);

fn module_handle() -> HINSTANCE {
    // SAFETY: GetModuleHandleW(None) returns this module's instance and never
    // borrows; a failure yields a null handle, which CreateWindowExW rejects.
    match unsafe { GetModuleHandleW(PCWSTR::null()) } {
        Ok(hmodule) => HINSTANCE(hmodule.0),
        Err(_) => HINSTANCE::default(),
    }
}

/// The child window's WndProc. For now it only suppresses background erase
/// (so the region never flashes a blank fill before the first GPU present);
/// everything else defers to the default. Mouse input is added with the
/// native interaction step.
extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if msg == WM_ERASEBKGND {
        // Handled — do not erase (the swapchain owns every pixel).
        return LRESULT(1);
    }
    // SAFETY: standard default handling for messages we don't process.
    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}

fn ensure_class(hinstance: HINSTANCE) {
    if CLASS_REGISTERED.swap(true, Ordering::SeqCst) {
        return; // registered once per process
    }
    let class = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
        lpfnWndProc: Some(wnd_proc),
        hInstance: hinstance,
        lpszClassName: PCWSTR(CLASS_NAME.as_ptr()),
        ..unsafe { std::mem::zeroed() }
    };
    // SAFETY: `class` is fully initialized above (zeroed elsewhere); a
    // duplicate registration just returns 0, which we ignore.
    let atom = unsafe { RegisterClassExW(&class) };
    if atom == 0 {
        // Another registration raced us or it failed; either way the class
        // name is usable if it already exists. Reset so a real failure retries.
        CLASS_REGISTERED.store(true, Ordering::SeqCst);
    }
}

pub struct WinPreviewWindow {
    hwnd: HWND,
    hinstance: HINSTANCE,
}

// The HWND is only touched from the UI thread (create/move/destroy); the
// Send/Sync surface handle is a separate value. This struct itself stays on
// the UI thread, so it is neither Send nor Sync (no impls).

impl WinPreviewWindow {
    pub fn create(parent_hwnd: isize, bounds: Bounds) -> Result<Self, PreviewError> {
        let hinstance = module_handle();
        if hinstance.0.is_null() {
            return Err(PreviewError::Os("no module handle".into()));
        }
        ensure_class(hinstance);
        let parent = HWND(parent_hwnd as *mut core::ffi::c_void);

        // SAFETY: a WS_CHILD window under a valid parent HWND; all args are
        // valid (registered class, sized rect). The returned HWND is checked.
        let hwnd = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE(0),
                PCWSTR(CLASS_NAME.as_ptr()),
                PCWSTR::null(),
                WS_CHILD | WS_VISIBLE | WS_CLIPSIBLINGS,
                bounds.x,
                bounds.y,
                bounds.width.max(1) as i32,
                bounds.height.max(1) as i32,
                parent,
                HMENU::default(),
                hinstance,
                None,
            )
        }
        .map_err(|err| PreviewError::Os(format!("CreateWindowExW: {err}")))?;

        if hwnd.0.is_null() {
            return Err(PreviewError::Os("CreateWindowExW returned null".into()));
        }
        Ok(Self { hwnd, hinstance })
    }

    pub fn surface_handle(&self) -> SurfaceHandle {
        SurfaceHandle {
            hwnd: self.hwnd.0 as isize,
            hinstance: self.hinstance.0 as isize,
        }
    }

    pub fn set_bounds(&self, bounds: Bounds) {
        // SAFETY: `hwnd` is a live child window; SetWindowPos is UI-thread safe.
        unsafe {
            let _ = SetWindowPos(
                self.hwnd,
                HWND::default(),
                bounds.x,
                bounds.y,
                bounds.width.max(1) as i32,
                bounds.height.max(1) as i32,
                SET_WINDOW_POS_FLAGS(SWP_NOZORDER.0 | SWP_NOACTIVATE.0),
            );
        }
    }

    pub fn set_visible(&self, visible: bool) {
        let cmd: SHOW_WINDOW_CMD = if visible { SW_SHOWNA } else { SW_HIDE };
        // SAFETY: `hwnd` is a live child window.
        unsafe {
            let _ = ShowWindow(self.hwnd, cmd);
        }
    }
}

impl Drop for WinPreviewWindow {
    fn drop(&mut self) {
        // SAFETY: destroy our own child window. Must run on the UI thread —
        // the app drops the PreviewWindow there.
        unsafe {
            let _ = DestroyWindow(self.hwnd);
        }
    }
}
