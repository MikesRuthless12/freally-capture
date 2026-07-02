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

use crate::{frame_channel, CaptureError, CaptureSession, SourceInfo, SourceKind};

const DISPLAY_PREFIX: &str = "display:";
const WINDOW_PREFIX: &str = "window:";

/// Check (and, once, request) the Screen-Recording permission.
fn ensure_permission() -> Result<(), CaptureError> {
    // SAFETY: plain permission query/request calls with no arguments.
    unsafe {
        if CGPreflightScreenCaptureAccess() {
            return Ok(());
        }
        // Shows the system prompt the first time; returns false immediately
        // if the user has already denied (they must flip it in Settings).
        if CGRequestScreenCaptureAccess() {
            return Ok(());
        }
    }
    Err(CaptureError::PermissionDenied)
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
            if !window.isOnScreen() || window.windowLayer() != 0 {
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
            sources.push(SourceInfo {
                id: format!("{WINDOW_PREFIX}{}", window.windowID()),
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
        sck::Target::Window(
            raw.parse()
                .map_err(|_| CaptureError::NotFound(format!("bad window id: {id}")))?,
        )
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
