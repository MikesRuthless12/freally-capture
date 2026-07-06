//! The direct X11 path (X sessions only — Wayland goes through the portal).
//!
//! Screens come from the connection setup; windows from the window manager's
//! `_NET_CLIENT_LIST`. Capture polls `GetImage` at ~30 fps — simple and
//! correct; X11 delivers whatever is on screen (a window covered by another
//! shows the overlap — an honest, documented limit of unredirected X11
//! capture). Pure `x11rb`, no `unsafe`.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use x11rb::connection::Connection;
use x11rb::protocol::xproto::{AtomEnum, ConnectionExt, ImageFormat, ImageOrder, Window};
use x11rb::rust_connection::RustConnection;

use crate::window_match::{encode_window_id, resolve_best, same_window, WindowKey};
use crate::{
    frame_channel, CaptureError, CaptureSession, Frame, FrameSender, PixelFormat, SourceInfo,
    SourceKind,
};

const FRAME_INTERVAL: Duration = Duration::from_millis(33); // ~30 fps

pub(crate) enum Target {
    Screen(usize),
    Window { xid: u32, key: WindowKey },
}

fn connect() -> Result<(RustConnection, usize), CaptureError> {
    x11rb::connect(None).map_err(|err| CaptureError::Backend(format!("X11 connect: {err}")))
}

fn intern(conn: &RustConnection, name: &str) -> Result<u32, CaptureError> {
    Ok(conn
        .intern_atom(false, name.as_bytes())
        .map_err(|err| CaptureError::Backend(format!("intern {name}: {err}")))?
        .reply()
        .map_err(|err| CaptureError::Backend(format!("intern {name}: {err}")))?
        .atom)
}

pub(crate) fn list_sources() -> Result<Vec<SourceInfo>, CaptureError> {
    let (conn, _) = connect()?;
    if conn.setup().image_byte_order != ImageOrder::LSB_FIRST {
        return Err(CaptureError::Unsupported(
            "big-endian X servers are not supported".into(),
        ));
    }
    let mut sources = Vec::new();

    for (index, screen) in conn.setup().roots.iter().enumerate() {
        sources.push(SourceInfo {
            id: format!("x11screen:{index}"),
            kind: SourceKind::Display,
            label: format!(
                "Screen {} — {}×{}{}",
                index + 1,
                screen.width_in_pixels,
                screen.height_in_pixels,
                if index == 0 { " (primary)" } else { "" }
            ),
            width: screen.width_in_pixels as u32,
            height: screen.height_in_pixels as u32,
        });
    }

    let client_list = intern(&conn, "_NET_CLIENT_LIST")?;
    let net_wm_name = intern(&conn, "_NET_WM_NAME")?;
    let utf8_string = intern(&conn, "UTF8_STRING")?;

    for screen in conn.setup().roots.iter() {
        let Ok(cookie) = conn.get_property(
            false,
            screen.root,
            client_list,
            AtomEnum::WINDOW,
            0,
            u32::MAX,
        ) else {
            continue;
        };
        let Ok(reply) = cookie.reply() else { continue };
        let Some(windows) = reply.value32() else {
            continue;
        };
        for window in windows {
            let title = window_title(&conn, window, net_wm_name, utf8_string);
            let Some(title) = title.filter(|t| !t.is_empty()) else {
                continue;
            };
            let Ok(cookie) = conn.get_geometry(window) else {
                continue;
            };
            let Ok(geometry) = cookie.reply() else {
                continue;
            };
            if geometry.width == 0 || geometry.height == 0 {
                continue;
            }
            let (instance, class) = window_class(&conn, window);
            let key = WindowKey::new(class, instance, title.clone());
            sources.push(SourceInfo {
                id: format!("x11window:{}", encode_window_id(u64::from(window), &key)),
                kind: SourceKind::Window,
                label: title,
                width: geometry.width as u32,
                height: geometry.height as u32,
            });
        }
    }
    Ok(sources)
}

fn window_title(
    conn: &RustConnection,
    window: Window,
    net_wm_name: u32,
    utf8_string: u32,
) -> Option<String> {
    // Prefer the EWMH UTF-8 name, fall back to the legacy WM_NAME.
    if let Ok(reply) = conn
        .get_property(false, window, net_wm_name, utf8_string, 0, 1024)
        .ok()?
        .reply()
    {
        if !reply.value.is_empty() {
            return Some(String::from_utf8_lossy(&reply.value).into_owned());
        }
    }
    let reply = conn
        .get_property(false, window, AtomEnum::WM_NAME, AtomEnum::STRING, 0, 1024)
        .ok()?
        .reply()
        .ok()?;
    if reply.value.is_empty() {
        None
    } else {
        Some(String::from_utf8_lossy(&reply.value).into_owned())
    }
}

/// `WM_CLASS`: two NUL-separated strings, the instance then the class. Empty
/// strings on failure (matching then leans on the title).
fn window_class(conn: &RustConnection, window: Window) -> (String, String) {
    let Ok(cookie) = conn.get_property(false, window, AtomEnum::WM_CLASS, AtomEnum::STRING, 0, 256)
    else {
        return (String::new(), String::new());
    };
    let Ok(reply) = cookie.reply() else {
        return (String::new(), String::new());
    };
    let mut parts = reply.value.split(|&byte| byte == 0);
    let instance = parts
        .next()
        .map(|bytes| String::from_utf8_lossy(bytes).into_owned())
        .unwrap_or_default();
    let class = parts
        .next()
        .map(|bytes| String::from_utf8_lossy(bytes).into_owned())
        .unwrap_or_default();
    (instance, class)
}

/// Whether `window` is still a live drawable (its geometry query succeeds).
fn window_geometry_ok(conn: &RustConnection, window: Window) -> bool {
    conn.get_geometry(window)
        .ok()
        .and_then(|cookie| cookie.reply().ok())
        .is_some()
}

/// The durable identity of a live X11 window — the `WM_CLASS` class as the
/// anchor, its instance as the secondary field, and the title.
fn x11_window_key(
    conn: &RustConnection,
    window: Window,
    net_wm_name: u32,
    utf8_string: u32,
) -> WindowKey {
    let (instance, class) = window_class(conn, window);
    let title = window_title(conn, window, net_wm_name, utf8_string).unwrap_or_default();
    WindowKey::new(class, instance, title)
}

/// Every managed window's (XID, identity) across all screens — the candidate
/// set for re-binding a persisted window whose stored XID went stale.
fn enumerate_window_keys(
    conn: &RustConnection,
    net_wm_name: u32,
    utf8_string: u32,
) -> Vec<(u64, WindowKey)> {
    // Prefer stacking order so a tie among identical windows resolves to the
    // topmost — matching Windows' EnumWindows Z-order (resolve_best takes the
    // first candidate). `_NET_CLIENT_LIST_STACKING` is bottom→top, so reverse
    // it; fall back to `_NET_CLIENT_LIST` (mapping order) when it's absent.
    let (list_atom, is_stacking) = match intern(conn, "_NET_CLIENT_LIST_STACKING").ok() {
        Some(atom) => (atom, true),
        None => match intern(conn, "_NET_CLIENT_LIST").ok() {
            Some(atom) => (atom, false),
            None => return Vec::new(),
        },
    };
    let mut out = Vec::new();
    for screen in conn.setup().roots.iter() {
        let Ok(cookie) =
            conn.get_property(false, screen.root, list_atom, AtomEnum::WINDOW, 0, u32::MAX)
        else {
            continue;
        };
        let Ok(reply) = cookie.reply() else { continue };
        let Some(windows) = reply.value32() else {
            continue;
        };
        let mut screen_windows: Vec<u32> = windows.collect();
        if is_stacking {
            screen_windows.reverse(); // bottom→top ⇒ top→bottom
        }
        for window in screen_windows {
            if window_geometry_ok(conn, window) {
                out.push((
                    u64::from(window),
                    x11_window_key(conn, window, net_wm_name, utf8_string),
                ));
            }
        }
    }
    out
}

/// Re-bind a persisted X11 window: an XID is only valid within the session it
/// was picked in, so trust the stored one while it still exists and still
/// matches, else re-scan `_NET_CLIENT_LIST` for the same window by its
/// `WM_CLASS` + title identity — the X11 side of OBS-style window recovery.
fn resolve_x11_window(
    conn: &RustConnection,
    xid: u32,
    key: &WindowKey,
) -> Result<u32, CaptureError> {
    let net_wm_name = intern(conn, "_NET_WM_NAME").unwrap_or(0);
    let utf8_string = intern(conn, "UTF8_STRING").unwrap_or(0);
    if window_geometry_ok(conn, xid)
        && (key.is_empty()
            || same_window(key, &x11_window_key(conn, xid, net_wm_name, utf8_string)))
    {
        return Ok(xid);
    }
    if key.is_empty() {
        return Err(CaptureError::NotFound("the window no longer exists".into()));
    }
    let candidates = enumerate_window_keys(conn, net_wm_name, utf8_string);
    resolve_best(key, &candidates)
        .map(|handle| handle as u32)
        .ok_or_else(|| CaptureError::NotFound("the window is no longer open".into()))
}

pub(crate) fn start(target: Target) -> Result<CaptureSession, CaptureError> {
    // Validate/resolve the target up front so a bad id fails fast.
    let (conn, _) = connect()?;
    let target = match target {
        Target::Screen(index) => {
            if conn.setup().roots.get(index).is_none() {
                return Err(CaptureError::NotFound(format!("no X screen {index}")));
            }
            Target::Screen(index)
        }
        Target::Window { xid, key } => {
            let xid = resolve_x11_window(&conn, xid, &key)?;
            Target::Window { xid, key }
        }
    };
    drop(conn);

    let (sender, receiver) = frame_channel();
    let stop = Arc::new(AtomicBool::new(false));
    let stop_thread = Arc::clone(&stop);
    let join = std::thread::Builder::new()
        .name("fcap-x11".into())
        .spawn(move || run(target, sender, stop_thread))
        .map_err(|err| CaptureError::Backend(format!("could not spawn capture: {err}")))?;
    Ok(CaptureSession::from_parts(receiver, stop, join))
}

fn run(target: Target, sender: FrameSender, stop: Arc<AtomicBool>) {
    match run_inner(target, &sender, &stop) {
        Ok(()) => sender.close(None),
        Err(err) => sender.close(Some(err)),
    }
}

fn run_inner(target: Target, sender: &FrameSender, stop: &AtomicBool) -> Result<(), CaptureError> {
    let (conn, _) = connect()?;
    if conn.setup().image_byte_order != ImageOrder::LSB_FIRST {
        return Err(CaptureError::Unsupported(
            "big-endian X servers are not supported".into(),
        ));
    }

    let mut next_frame = Instant::now();
    while !stop.load(Ordering::Relaxed) && sender.is_open() {
        let now = Instant::now();
        if now < next_frame {
            std::thread::sleep(next_frame - now);
        }
        next_frame = Instant::now() + FRAME_INTERVAL;

        let (drawable, width, height) = match &target {
            Target::Screen(index) => {
                let screen = conn
                    .setup()
                    .roots
                    .get(*index)
                    .ok_or_else(|| CaptureError::NotFound(format!("no X screen {index}")))?;
                (screen.root, screen.width_in_pixels, screen.height_in_pixels)
            }
            Target::Window { xid, .. } => {
                // Geometry every frame — windows resize.
                let Ok(geometry) = conn
                    .get_geometry(*xid)
                    .map_err(|err| CaptureError::Backend(format!("get_geometry: {err}")))?
                    .reply()
                else {
                    return Err(CaptureError::NotFound("the window was closed".into()));
                };
                (*xid, geometry.width, geometry.height)
            }
        };
        if width == 0 || height == 0 {
            continue;
        }

        let image = match conn
            .get_image(
                ImageFormat::Z_PIXMAP,
                drawable,
                0,
                0,
                width,
                height,
                u32::MAX,
            )
            .map_err(|err| CaptureError::Backend(format!("get_image: {err}")))?
            .reply()
        {
            Ok(image) => image,
            // Unmapped/minimized windows fail GetImage — keep trying rather
            // than kill the session (the user may restore the window).
            Err(_) => continue,
        };
        if image.depth != 24 && image.depth != 32 {
            return Err(CaptureError::Unsupported(format!(
                "X visual depth {} is not supported (need 24/32-bit TrueColor)",
                image.depth
            )));
        }
        // Depth 24 can still be *packed* 3-bytes-per-pixel on some servers
        // (Xvfb/legacy drivers). Fail honestly instead of silently skipping
        // every under-sized reply forever.
        let bits_per_pixel = conn
            .setup()
            .pixmap_formats
            .iter()
            .find(|format| format.depth == image.depth)
            .map(|format| format.bits_per_pixel)
            .unwrap_or(32);
        if bits_per_pixel != 32 {
            return Err(CaptureError::Unsupported(format!(
                "X pixmap layout {bits_per_pixel} bpp at depth {} is not supported (need 32 bpp)",
                image.depth
            )));
        }

        let stride = (width as u32) * 4;
        let needed = stride as usize * height as usize;
        let mut data = image.data;
        if data.len() < needed {
            continue; // short read — skip the frame
        }
        data.truncate(needed);
        if image.depth == 24 {
            // ZPixmap depth-24 on LSB servers is BGRX — force real alpha.
            for px in data.chunks_exact_mut(4) {
                px[3] = 0xFF;
            }
        }

        sender.send(Frame {
            width: width as u32,
            height: height as u32,
            stride,
            format: PixelFormat::Bgra8,
            data,
            captured_at: Instant::now(),
        });
    }
    Ok(())
}
