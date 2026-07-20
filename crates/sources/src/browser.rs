//! CAP-N77 — the **Browser source**: an arbitrary URL rendered offscreen
//! (transparency intact, chosen resolution/fps) and composed like any other
//! source.
//!
//! **Architecture (per `design/browser-source-spike.md`, Option A):** the
//! rendering is done by CEF with offscreen rendering — but never in-process.
//! A small **browser-host** helper executable loads the on-demand, hash-
//! verified CEF runtime component (fetched by `fcap_encode::cef`, the ffmpeg
//! pattern) and streams raw RGBA frames back over its stdout — the exact
//! pipe-pump shape the Media source already trusts. The app never links
//! Chromium; the host + runtime are removable components.
//!
//! **Honest state:** the host executable ships as its own component build
//! (`design/browser-host-protocol.md`). Until it is installed, starting this
//! source fails readably — the picker says so up front, nothing crashes.
//!
//! Protocol (v1, `design/browser-host-protocol.md` is normative):
//! - Host is spawned: `freally-browser-host --url U --width W --height H
//!   --fps F [--transparent] --cef DIR`.
//! - Host writes to stdout: the 16-byte header `FBH1` + u32le width +
//!   u32le height + u32le fps, then fixed-size `W*H*4` RGBA frames forever.
//! - Host exits non-zero with a one-line stderr reason on any failure.

use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

use fcap_capture::{frame_channel, CaptureError, CaptureSession, Frame, PixelFormat};

use crate::media::{read_exact_or_end, spawn_kill_watchdog};

/// Shown when the browser-host component is missing — surfaced both here (when
/// a source tries to start) and by the studio dispatch (which can't reach this
/// path without the runtime dir). One string so the two never drift.
pub const BROWSER_COMPONENT_MISSING: &str =
    "the Browser source needs the Browser Runtime component — install it from Components";

/// What the picker configured (mirrors `SourceSettings::Browser`).
#[derive(Debug, Clone)]
pub struct BrowserConfig {
    pub url: String,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    /// Render on a transparent page background (alpha carries through).
    pub transparent: bool,
    /// The verified CEF runtime directory (`fcap_encode::cef::installed`).
    pub cef_dir: PathBuf,
}

/// The stdout stream's magic — bump on any layout change.
const MAGIC: &[u8; 4] = b"FBH1";

/// Where the browser-host executable lives once its component is installed:
/// a `host` sibling of the CEF runtime's `current` install.
pub fn host_path(cef_dir: &std::path::Path) -> PathBuf {
    let exe = if cfg!(windows) {
        "freally-browser-host.exe"
    } else {
        "freally-browser-host"
    };
    // `<cache>/cef/current/<dist>/` → `<cache>/cef/host/<exe>`
    cef_dir
        .parent()
        .and_then(|p| p.parent())
        .map(|root| root.join("host").join(exe))
        .unwrap_or_else(|| PathBuf::from(exe))
}

/// Validate the URL shape: http/https only in v1 (a `file://` UNC form would
/// stat a network path — the CAP-M16 rule — and local files already have the
/// Media/Image sources; said honestly in the picker).
pub fn validate_url(url: &str) -> Result<(), String> {
    // URL schemes are case-insensitive (RFC 3986); lowercase for the check only
    // (the original case is what we hand the host). This is still an allowlist,
    // so it fails closed — `file://`, UNC, `javascript:` all stay rejected.
    let scheme = url.trim().to_ascii_lowercase();
    if scheme.starts_with("http://") || scheme.starts_with("https://") {
        Ok(())
    } else {
        Err("the Browser source loads http:// or https:// pages (local files play through Media/Image)".into())
    }
}

/// Start the browser-host session: spawn the helper against the installed
/// CEF runtime and pump its fixed-size RGBA frames into the standard
/// latest-wins channel. Fails readably when the host is not installed.
pub fn start_browser(hub_id: &str, config: BrowserConfig) -> Result<CaptureSession, CaptureError> {
    validate_url(&config.url).map_err(CaptureError::Backend)?;
    let width = config.width.clamp(64, 3840);
    let height = config.height.clamp(64, 2160);
    let fps = config.fps.clamp(1, 60);

    let host = host_path(&config.cef_dir);
    if !host.is_file() {
        return Err(CaptureError::Backend(BROWSER_COMPONENT_MISSING.into()));
    }

    let mut command = Command::new(&host);
    command
        .arg("--url")
        .arg(config.url.trim())
        .arg("--width")
        .arg(width.to_string())
        .arg("--height")
        .arg(height.to_string())
        .arg("--fps")
        .arg(fps.to_string())
        .arg("--cef")
        .arg(&config.cef_dir)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    if config.transparent {
        command.arg("--transparent");
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    }
    let mut child = command
        .spawn()
        .map_err(|err| CaptureError::Backend(format!("could not start the browser host: {err}")))?;
    let mut stdout = child
        .stdout
        .take()
        .ok_or_else(|| CaptureError::Backend("the browser host gave no output pipe".into()))?;

    let (sender, receiver) = frame_channel();
    let stop = Arc::new(AtomicBool::new(false));
    let thread_stop = Arc::clone(&stop);
    let hub = hub_id.to_string();
    let join = std::thread::Builder::new()
        .name("fcap-browser".into())
        .spawn(move || {
            // The stop watchdog kills the host the instant `stop` is set, which
            // unblocks the fixed-size read below. Without it a host that paints
            // once and then goes quiet (a static overlay — a lower-third, a
            // logo) leaves the pump blocked forever, and a later `stop()` (which
            // joins this thread) would wedge the whole studio reconcile loop.
            let (kill_tx, watchdog) =
                spawn_kill_watchdog("fcap-browser-watchdog", &thread_stop, vec![child]);
            let wind_down =
                |kill_tx: std::sync::mpsc::Sender<()>,
                 watchdog: Option<std::thread::JoinHandle<()>>| {
                    let _ = kill_tx.send(());
                    if let Some(handle) = watchdog {
                        let _ = handle.join();
                    }
                };

            // Header first: magic + the host's ACTUAL geometry (it may differ
            // across CEF versions — the stream is the truth, within bounds).
            let mut header = [0u8; 16];
            if !read_exact_or_end(&mut stdout, &mut header) || &header[0..4] != MAGIC {
                eprintln!("browser[{hub}]: host handshake failed — wrong or missing header");
                wind_down(kill_tx, watchdog);
                return;
            }
            let w = u32::from_le_bytes([header[4], header[5], header[6], header[7]]);
            let h = u32::from_le_bytes([header[8], header[9], header[10], header[11]]);
            // Reject an out-of-range geometry rather than clamp-and-desync: a
            // host we can't size a frame for is a handshake failure, not a torn
            // stream we keep reading forever at the wrong stride.
            if !(1..=7680).contains(&w) || !(1..=4320).contains(&h) {
                eprintln!("browser[{hub}]: host reported out-of-range geometry {w}x{h}");
                wind_down(kill_tx, watchdog);
                return;
            }
            let frame_bytes = (w as usize) * (h as usize) * 4;
            let mut data = vec![0u8; frame_bytes];
            loop {
                if thread_stop.load(Ordering::Relaxed) || !sender.is_open() {
                    break;
                }
                if !read_exact_or_end(&mut stdout, &mut data) {
                    break; // host exited — the studio's auto-recover restarts us
                }
                sender.send(Frame {
                    width: w,
                    height: h,
                    stride: w * 4,
                    format: PixelFormat::Rgba8,
                    data: data.clone(),
                    captured_at: Instant::now(),
                });
            }
            wind_down(kill_tx, watchdog);
        })
        .map_err(|err| CaptureError::Backend(err.to_string()))?;

    Ok(CaptureSession::from_parts(receiver, stop, join))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urls_are_http_only_in_v1() {
        assert!(validate_url("https://example.com/overlay").is_ok());
        assert!(validate_url("  http://localhost:8080/x ").is_ok());
        assert!(
            validate_url("HTTPS://example.com").is_ok(),
            "schemes are case-insensitive"
        );
        assert!(validate_url("file:///C:/x.html").is_err());
        assert!(
            validate_url(r"file://\\server\share\x.html").is_err(),
            "UNC never stats"
        );
        assert!(validate_url("javascript:alert(1)").is_err());
        assert!(validate_url("").is_err());
    }

    #[test]
    fn host_path_is_a_sibling_of_the_runtime() {
        let cef = std::path::Path::new("/cache/cef/current/cef_binary_x_minimal");
        let host = host_path(cef);
        assert!(host
            .to_string_lossy()
            .replace('\\', "/")
            .ends_with(if cfg!(windows) {
                "cache/cef/host/freally-browser-host.exe"
            } else {
                "cache/cef/host/freally-browser-host"
            }));
    }

    #[test]
    fn a_missing_host_fails_readably() {
        let err = start_browser(
            "test",
            BrowserConfig {
                url: "https://example.com".into(),
                width: 1280,
                height: 720,
                fps: 30,
                transparent: true,
                cef_dir: PathBuf::from("/definitely/not/installed/current/dist"),
            },
        )
        .err()
        .expect("no host installed");
        let text = format!("{err:?}");
        assert!(
            text.contains("Browser Runtime component"),
            "honest message: {text}"
        );
    }
}
