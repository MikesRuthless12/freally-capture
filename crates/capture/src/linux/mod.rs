//! Linux capture: the ScreenCast portal (`ashpd` → PipeWire) on Wayland, a
//! direct X11 path on X sessions.
//!
//! Honesty first: **Wayland capture is portal-only.** The system dialog picks
//! the screen/window — an app cannot enumerate or grab them globally, so on
//! Wayland `list_sources` returns exactly one pseudo-source that opens the
//! picker. On X11 we list screens and top-level windows directly. No `unsafe`
//! anywhere on this path.

pub(crate) mod portal;
pub(crate) mod pw;
pub(crate) mod x11;

use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use crate::{frame_channel, CaptureError, CaptureSession, SourceInfo, SourceKind};

pub(crate) const PORTAL_ID: &str = "portal";
const X11_SCREEN_PREFIX: &str = "x11screen:";
const X11_WINDOW_PREFIX: &str = "x11window:";

/// Which display server this process can reach.
enum SessionType {
    Wayland,
    X11,
    None,
}

fn session_type() -> SessionType {
    let session = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();
    if session == "wayland" || std::env::var_os("WAYLAND_DISPLAY").is_some() {
        return SessionType::Wayland;
    }
    if session == "x11" || std::env::var_os("DISPLAY").is_some() {
        return SessionType::X11;
    }
    SessionType::None
}

fn portal_source() -> SourceInfo {
    SourceInfo {
        id: PORTAL_ID.to_string(),
        kind: SourceKind::Portal,
        label: "Screen or window — the system picker chooses".to_string(),
        width: 0,
        height: 0,
    }
}

pub(crate) fn list_sources() -> Result<Vec<SourceInfo>, CaptureError> {
    match session_type() {
        SessionType::Wayland => {
            // Portal-only, by design of Wayland — say so, don't fake a list.
            Ok(vec![portal_source()])
        }
        SessionType::X11 => x11::list_sources(),
        SessionType::None => Err(CaptureError::Unsupported(
            "no Wayland or X11 display session found".into(),
        )),
    }
}

pub(crate) fn start_capture(id: &str) -> Result<CaptureSession, CaptureError> {
    if id == PORTAL_ID {
        // Blocks on the system picker dialog (that's the whole point).
        let stream = portal::open_portal_stream()?;
        let (sender, receiver) = frame_channel();
        let stop = Arc::new(AtomicBool::new(false));
        let stop_thread = Arc::clone(&stop);
        let join = std::thread::Builder::new()
            .name("fcap-pipewire".into())
            .spawn(move || pw::run(stream, sender, stop_thread))
            .map_err(|err| CaptureError::Backend(format!("could not spawn capture: {err}")))?;
        return Ok(CaptureSession::from_parts(receiver, stop, join));
    }
    if let Some(raw) = id.strip_prefix(X11_SCREEN_PREFIX) {
        let screen: usize = raw
            .parse()
            .map_err(|_| CaptureError::NotFound(format!("bad screen id: {id}")))?;
        return x11::start(x11::Target::Screen(screen));
    }
    if let Some(raw) = id.strip_prefix(X11_WINDOW_PREFIX) {
        let window: u32 = raw
            .parse()
            .map_err(|_| CaptureError::NotFound(format!("bad window id: {id}")))?;
        return x11::start(x11::Target::Window(window));
    }
    Err(CaptureError::NotFound(format!("unknown source id: {id}")))
}
