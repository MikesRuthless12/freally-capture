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

use crate::{
    frame_channel, CaptureError, CaptureSession, Frame, FrameSender, PixelFormat, SourceInfo,
    SourceKind,
};

const FRAME_INTERVAL: Duration = Duration::from_millis(33); // ~30 fps

pub(crate) enum Target {
    Screen(usize),
    Window(u32),
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
            sources.push(SourceInfo {
                id: format!("x11window:{window}"),
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

pub(crate) fn start(target: Target) -> Result<CaptureSession, CaptureError> {
    // Validate the target up front so a bad id fails fast.
    let (conn, _) = connect()?;
    match &target {
        Target::Screen(index) => {
            if conn.setup().roots.get(*index).is_none() {
                return Err(CaptureError::NotFound(format!("no X screen {index}")));
            }
        }
        Target::Window(window) => {
            if conn
                .get_geometry(*window)
                .map_err(|err| CaptureError::Backend(format!("get_geometry: {err}")))?
                .reply()
                .is_err()
            {
                return Err(CaptureError::NotFound("the window no longer exists".into()));
            }
        }
    }
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
            Target::Window(window) => {
                // Geometry every frame — windows resize.
                let Ok(geometry) = conn
                    .get_geometry(*window)
                    .map_err(|err| CaptureError::Backend(format!("get_geometry: {err}")))?
                    .reply()
                else {
                    return Err(CaptureError::NotFound("the window was closed".into()));
                };
                (*window, geometry.width, geometry.height)
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
