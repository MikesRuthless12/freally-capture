//! macOS capture via ScreenCaptureKit (macOS 12.3+).
//!
//! Enumeration and streaming both go through `SCShareableContent` /
//! `SCStream`; the Screen-Recording permission is checked first
//! (`CGPreflightScreenCaptureAccess`) and requested once
//! (`CGRequestScreenCaptureAccess`). A denial surfaces as
//! [`CaptureError::PermissionDenied`] so the app can deep-link the user to
//! System Settings → Privacy & Security → Screen Recording.
//!
//! AUDITED `unsafe`: this module tree is the crate's isolated Objective-C /
//! CoreFoundation FFI surface (objc2 message sends, CF buffer reads, the
//! completion-handler retain dance). Every block is small and commented;
//! nothing else in the crate may use `unsafe`.
#![allow(unsafe_code)]

pub(crate) mod sck;

use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use objc2_core_graphics::{CGPreflightScreenCaptureAccess, CGRequestScreenCaptureAccess};
use objc2_screen_capture_kit::SCWindow;

use crate::window_match::{decode_window_id, encode_window_id, WindowKey};
use crate::{frame_channel, CaptureError, CaptureSession, SourceInfo, SourceKind};

const DISPLAY_PREFIX: &str = "display:";
const WINDOW_PREFIX: &str = "window:";

/// Check (and, once, request) the Screen-Recording permission.
/// (The CG access functions are safe wrappers in objc2-core-graphics.)
fn ensure_permission() -> Result<(), CaptureError> {
    if CGPreflightScreenCaptureAccess() {
        return Ok(());
    }
    // Shows the system prompt the first time; returns false immediately
    // if the user has already denied (they must flip it in Settings).
    if CGRequestScreenCaptureAccess() {
        return Ok(());
    }
    Err(CaptureError::PermissionDenied)
}

/// The durable identity of a window — the owning app's name as the anchor plus
/// the title. Used both to list windows and to re-bind a persisted one after a
/// restart, since a `CGWindowID` is only valid within the session it was picked
/// in (like an HWND). macOS has no per-window "class", so that field stays empty.
pub(super) fn window_key(window: &SCWindow) -> WindowKey {
    // SAFETY: property getters on a live SCWindow from the shareable snapshot.
    unsafe {
        let app = window
            .owningApplication()
            .map(|app| app.applicationName().to_string())
            .unwrap_or_default();
        let title = window
            .title()
            .map(|title| title.to_string())
            .unwrap_or_default();
        WindowKey::new(app, String::new(), title)
    }
}

pub(crate) fn list_sources() -> Result<Vec<SourceInfo>, CaptureError> {
    ensure_permission()?;
    let content = sck::fetch_shareable_content()?;
    let mut sources = Vec::new();

    // SAFETY: property getters on the retained snapshot.
    let displays = unsafe { content.displays() };
    for (index, display) in displays.iter().enumerate() {
        // SAFETY: SCDisplay getters on a live object.
        let (id, rect) = unsafe { (display.displayID(), display.frame()) };
        let width = rect.size.width.max(0.0) as u32;
        let height = rect.size.height.max(0.0) as u32;
        sources.push(SourceInfo {
            id: format!("{DISPLAY_PREFIX}{id}"),
            kind: SourceKind::Display,
            label: format!(
                "Display {} — {width}×{height}{}",
                index + 1,
                if index == 0 { " (primary)" } else { "" }
            ),
            width,
            height,
        });
    }

    // SAFETY: property getters on the retained snapshot.
    let windows = unsafe { content.windows() };
    for window in windows.iter() {
        // SAFETY: SCWindow getters on a live object.
        unsafe {
            // Include off-screen windows too — a minimized window is off-screen,
            // and the picker should list every open window. `windowLayer != 0`
            // still drops non-normal layers (menu bar, Dock, overlays).
            if window.windowLayer() != 0 {
                continue;
            }
            let Some(title) = window.title() else {
                continue;
            };
            let title = title.to_string();
            if title.is_empty() {
                continue;
            }
            let app_name = window
                .owningApplication()
                .map(|app| app.applicationName().to_string())
                .filter(|name| !name.is_empty());
            let rect = window.frame();
            let width = rect.size.width.max(0.0) as u32;
            let height = rect.size.height.max(0.0) as u32;
            if width == 0 || height == 0 {
                continue;
            }
            let key = window_key(&window);
            sources.push(SourceInfo {
                id: format!(
                    "{WINDOW_PREFIX}{}",
                    encode_window_id(u64::from(window.windowID()), &key)
                ),
                kind: SourceKind::Window,
                label: match app_name {
                    Some(app) => format!("{app} — {title}"),
                    None => title,
                },
                width,
                height,
            });
        }
    }
    Ok(sources)
}

pub(crate) fn start_capture(id: &str) -> Result<CaptureSession, CaptureError> {
    ensure_permission()?;
    let target = if let Some(raw) = id.strip_prefix(DISPLAY_PREFIX) {
        sck::Target::Display(
            raw.parse()
                .map_err(|_| CaptureError::NotFound(format!("bad display id: {id}")))?,
        )
    } else if let Some(raw) = id.strip_prefix(WINDOW_PREFIX) {
        let (window_id, key) = decode_window_id(raw)
            .ok_or_else(|| CaptureError::NotFound(format!("bad window id: {id}")))?;
        sck::Target::Window {
            window_id: window_id as u32,
            key,
        }
    } else {
        return Err(CaptureError::NotFound(format!("unknown source id: {id}")));
    };

    let (sender, receiver) = frame_channel();
    let stop = Arc::new(AtomicBool::new(false));
    let stop_thread = Arc::clone(&stop);
    let join = std::thread::Builder::new()
        .name("fcap-sck".into())
        .spawn(move || sck::run(target, sender, stop_thread))
        .map_err(|err| CaptureError::Backend(format!("could not spawn capture: {err}")))?;
    Ok(CaptureSession::from_parts(receiver, stop, join))
}
