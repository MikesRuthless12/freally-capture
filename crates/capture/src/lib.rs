//! # fcap-capture
//!
//! Per-OS screen / window capture behind a single interface:
//! **Windows** DXGI Desktop Duplication (displays) + Windows.Graphics.Capture
//! (per-window), **macOS** ScreenCaptureKit via `objc2` (with the
//! Screen-Recording permission flow), **Linux** the ScreenCast portal via
//! `ashpd` + PipeWire (Wayland — the *user* picks the source in the system
//! dialog; global capture is not possible there, and this crate never
//! pretends otherwise) plus a direct X11 path.
//!
//! The surface is deliberately small:
//! [`list_sources`] enumerates what can be captured, [`start_capture`] spawns
//! the OS pipeline and returns a [`CaptureSession`] whose [`FrameReceiver`]
//! yields timestamped, GPU-uploadable BGRA/RGBA [`Frame`]s (latest-wins — a
//! slow consumer sees fresh frames and an honest dropped-frame count, never a
//! growing queue).
//!
//! The unavoidable OS `unsafe` is isolated in small audited modules
//! (`win`, `macos`); the crate root and the Linux path stay `deny(unsafe_code)`.

#![deny(unsafe_code)]

use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

use thiserror::Error;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod win;
/// Durable window identity + re-resolution, shared by the per-OS window paths.
#[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
mod window_match;

pub mod cursorfx;
pub mod game;
pub mod signals;
pub mod tonemap;

/// This crate's version (inherited from the workspace).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Pixel layout of a [`Frame`]. Both are 4 bytes per pixel, alpha last.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    /// Little-endian BGRA (the native DXGI / ScreenCaptureKit / X11 order).
    Bgra8,
    /// RGBA (webcams and some PipeWire negotiations).
    Rgba8,
}

/// One captured frame: CPU-side, tightly rowed by `stride`, ready for GPU
/// upload (the Phase 2 compositor consumes exactly this shape).
#[derive(Clone)]
pub struct Frame {
    pub width: u32,
    pub height: u32,
    /// Bytes per row in `data`; `>= width * 4` (capture APIs pad rows).
    pub stride: u32,
    pub format: PixelFormat,
    /// `stride * height` bytes.
    pub data: Vec<u8>,
    /// Monotonic arrival time — good for ordering and preview pacing.
    /// (Backend presentation timestamps come with the A/V-sync work, Phase 3/4.)
    pub captured_at: Instant,
}

impl std::fmt::Debug for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Frame")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("stride", &self.stride)
            .field("format", &self.format)
            .field("bytes", &self.data.len())
            .finish()
    }
}

/// Why a capture could not start or stopped.
#[derive(Debug, Clone, Error)]
pub enum CaptureError {
    /// The OS denied capture (macOS Screen Recording permission).
    #[error("screen capture permission was denied")]
    PermissionDenied,
    /// The user dismissed the system picker (Linux portal).
    #[error("capture was cancelled in the system picker")]
    Cancelled,
    /// The requested source no longer exists (window closed, display unplugged).
    #[error("capture source not found: {0}")]
    NotFound(String),
    /// This platform/session cannot do the requested capture.
    #[error("capture not supported here: {0}")]
    Unsupported(String),
    /// The OS capture pipeline failed.
    #[error("capture backend error: {0}")]
    Backend(String),
    /// The capture ended (source closed or the session was stopped).
    #[error("capture stopped")]
    Stopped,
}

/// What kind of thing a [`SourceInfo`] is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceKind {
    Display,
    Window,
    /// The Linux ScreenCast portal: the *system dialog* picks the screen or
    /// window (the only capture Wayland allows — surfaced honestly).
    Portal,
}

/// One capturable source, as shown in a picker.
#[derive(Debug, Clone)]
pub struct SourceInfo {
    /// Opaque id understood by [`start_capture`] (stable within one listing).
    pub id: String,
    pub kind: SourceKind,
    /// Human label: display name + resolution, or app/window title.
    pub label: String,
    /// Pixel size when known, `0` when the OS only reveals it after start.
    pub width: u32,
    pub height: u32,
}

/// A one-shot, downscaled **RGBA** thumbnail of a window — for the source
/// picker. Grab it repeatedly (see [`window_thumbnail`]) for a live preview.
#[derive(Debug, Clone)]
pub struct Thumbnail {
    pub width: u32,
    pub height: u32,
    /// Tightly-packed RGBA — `width * height * 4` bytes, opaque alpha.
    pub rgba: Vec<u8>,
}

// ---------------------------------------------------------------------------
// Latest-wins frame channel
// ---------------------------------------------------------------------------

struct MailboxInner {
    slot: Mutex<MailboxSlot>,
    cond: Condvar,
    dropped: AtomicU64,
    /// Live [`FrameSender`] handles; the last one to drop closes the channel.
    senders: AtomicUsize,
}

struct MailboxSlot {
    frame: Option<Frame>,
    closed: Option<CaptureError>,
}

/// Producer half — owned by the OS capture thread.
pub struct FrameSender {
    inner: Arc<MailboxInner>,
}

/// Consumer half — a single-slot, latest-wins mailbox. A slow consumer never
/// builds a queue; overwritten frames are counted in [`FrameReceiver::dropped`].
pub struct FrameReceiver {
    inner: Arc<MailboxInner>,
}

/// Create the latest-wins frame channel a capture backend feeds.
pub fn frame_channel() -> (FrameSender, FrameReceiver) {
    let inner = Arc::new(MailboxInner {
        slot: Mutex::new(MailboxSlot {
            frame: None,
            closed: None,
        }),
        cond: Condvar::new(),
        dropped: AtomicU64::new(0),
        senders: AtomicUsize::new(1),
    });
    (
        FrameSender {
            inner: Arc::clone(&inner),
        },
        FrameReceiver { inner },
    )
}

impl FrameSender {
    /// Publish a frame, replacing (and counting) any unconsumed one.
    pub fn send(&self, frame: Frame) {
        let mut slot = self.inner.slot.lock().expect("frame mailbox poisoned");
        if slot.closed.is_some() {
            return;
        }
        if slot.frame.replace(frame).is_some() {
            self.inner.dropped.fetch_add(1, Ordering::Relaxed);
        }
        self.inner.cond.notify_one();
    }

    /// Close the channel; `error` tells the consumer *why* (None = clean stop).
    pub fn close(&self, error: Option<CaptureError>) {
        let mut slot = self.inner.slot.lock().expect("frame mailbox poisoned");
        if slot.closed.is_none() {
            slot.closed = Some(error.unwrap_or(CaptureError::Stopped));
        }
        self.inner.cond.notify_all();
    }

    /// Whether the channel is still open (backends stop pushing once closed).
    pub fn is_open(&self) -> bool {
        let slot = self.inner.slot.lock().expect("frame mailbox poisoned");
        slot.closed.is_none()
    }
}

impl Clone for FrameSender {
    fn clone(&self) -> Self {
        self.inner.senders.fetch_add(1, Ordering::SeqCst);
        FrameSender {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl Drop for FrameSender {
    fn drop(&mut self) {
        if self.inner.senders.fetch_sub(1, Ordering::SeqCst) == 1 {
            self.close(None);
        }
    }
}

impl FrameReceiver {
    /// Wait up to `timeout` for the next frame. `Ok(None)` = no new frame yet;
    /// `Err` = the capture ended (the error says why).
    pub fn recv_timeout(&self, timeout: Duration) -> Result<Option<Frame>, CaptureError> {
        let deadline = Instant::now() + timeout;
        let mut slot = self.inner.slot.lock().expect("frame mailbox poisoned");
        loop {
            if let Some(frame) = slot.frame.take() {
                return Ok(Some(frame));
            }
            if let Some(err) = &slot.closed {
                return Err(err.clone());
            }
            let now = Instant::now();
            let Some(remaining) = deadline
                .checked_duration_since(now)
                .filter(|d| !d.is_zero())
            else {
                return Ok(None);
            };
            let (guard, _timeout) = self
                .inner
                .cond
                .wait_timeout(slot, remaining)
                .expect("frame mailbox poisoned");
            slot = guard;
        }
    }

    /// Frames overwritten before the consumer took them (honest drop count).
    pub fn dropped(&self) -> u64 {
        self.inner.dropped.load(Ordering::Relaxed)
    }
}

// ---------------------------------------------------------------------------
// Capture session
// ---------------------------------------------------------------------------

/// A running capture: the frame stream + the stop signal for the OS pipeline.
pub struct CaptureSession {
    receiver: FrameReceiver,
    stop: Arc<AtomicBool>,
    join: Option<std::thread::JoinHandle<()>>,
}

impl CaptureSession {
    /// Assemble a session from the parts a backend spawns. Public so sibling
    /// crates (`fcap-sources`' webcam) can reuse the same session shape.
    pub fn from_parts(
        receiver: FrameReceiver,
        stop: Arc<AtomicBool>,
        join: std::thread::JoinHandle<()>,
    ) -> Self {
        Self {
            receiver,
            stop,
            join: Some(join),
        }
    }

    /// The frame stream.
    pub fn frames(&self) -> &FrameReceiver {
        &self.receiver
    }

    /// Stop the capture and wait for the OS pipeline to wind down
    /// (backends poll the stop flag on ≤150 ms cycles).
    pub fn stop(mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(join) = self.join.take() {
            let _ = join.join();
        }
    }
}

impl Drop for CaptureSession {
    fn drop(&mut self) {
        // Signal only — never block a drop on an OS thread. Explicit `stop()`
        // is the joining path.
        self.stop.store(true, Ordering::Relaxed);
    }
}

// ---------------------------------------------------------------------------
// Per-OS entry points
// ---------------------------------------------------------------------------

/// Enumerate capturable sources on this machine.
///
/// Linux honesty: under Wayland this returns only the portal pseudo-source —
/// the system dialog picks the actual screen/window; under X11 it lists
/// screens and top-level windows directly.
pub fn list_sources() -> Result<Vec<SourceInfo>, CaptureError> {
    #[cfg(target_os = "windows")]
    {
        win::list_sources()
    }
    #[cfg(target_os = "macos")]
    {
        macos::list_sources()
    }
    #[cfg(target_os = "linux")]
    {
        linux::list_sources()
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Err(CaptureError::Unsupported(format!(
            "no capture backend for {}",
            std::env::consts::OS
        )))
    }
}

/// The desktop rectangle `(x, y, w, h)` a display/window capture id
/// currently covers, in virtual-screen pixels — re-resolved per call so a
/// moved window stays mapped. Windows-only today (CAP-N71 follow-pan; other
/// platforms return `None` and the UI is honest about it).
pub fn source_screen_rect(id: &str) -> Option<(i32, i32, u32, u32)> {
    #[cfg(target_os = "windows")]
    {
        win::source_screen_rect(id)
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = id;
        None
    }
}

/// Whether a display capture targets an HDR-enabled output (CAP-N74's
/// tone-map auto-suggest). Windows-only; `None` elsewhere or for windows.
pub fn display_is_hdr(id: &str) -> Option<bool> {
    #[cfg(target_os = "windows")]
    {
        win::display_is_hdr(id)
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = id;
        None
    }
}

/// The process behind a window-capture id — `(pid, exe name)`, re-resolved
/// by window identity per call (CAP-N73's window↔app-audio auto-link).
/// Windows-only today; other platforms return `None` and the UI says so.
pub fn window_process(id: &str) -> Option<(u32, String)> {
    #[cfg(target_os = "windows")]
    {
        win::window_process(id)
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = id;
        None
    }
}

/// Point-in-time "is this key down?" for a FIXED set of virtual keys —
/// CAP-N13's input overlay (mouse buttons are VKs too: 0x01/0x02/0x04).
/// A poll of `GetAsyncKeyState`, deliberately NOT a keyboard hook: no event
/// queue, no buffer, nothing logged or stored — and the overlay calls it
/// only while its source session is live. Windows-only today; other
/// platforms return `None` and the overlay draws its keys unpressed (the
/// picker says so honestly).
pub fn keys_down(vks: &[u16]) -> Option<Vec<bool>> {
    #[cfg(target_os = "windows")]
    {
        Some(win::keys::keys_down(vks))
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = vks;
        None
    }
}

/// The cursor's position on the virtual desktop (pairs with
/// [`source_screen_rect`] for CAP-N71's follow-pan). Windows-only today.
pub fn cursor_screen_position() -> Option<(i32, i32)> {
    #[cfg(target_os = "windows")]
    {
        win::cursor_position()
    }
    #[cfg(not(target_os = "windows"))]
    {
        None
    }
}

/// Start capturing the source `id` (from [`list_sources`]).
///
/// Blocking: on Linux the portal path waits for the user's choice in the
/// system picker — call from a worker thread, not a UI thread.
pub fn start_capture(id: &str) -> Result<CaptureSession, CaptureError> {
    #[cfg(target_os = "windows")]
    {
        win::start_capture(id)
    }
    #[cfg(target_os = "macos")]
    {
        macos::start_capture(id)
    }
    #[cfg(target_os = "linux")]
    {
        linux::start_capture(id)
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        let _ = id;
        Err(CaptureError::Unsupported(format!(
            "no capture backend for {}",
            std::env::consts::OS
        )))
    }
}

/// Grab a one-shot [`Thumbnail`] of the window `id` (from [`list_sources`]),
/// downscaled so its longer side is at most `max_dim`. It runs the *real*
/// capture path ([`start_capture`]) just long enough for one frame, so the
/// preview matches what a capture actually shows — including GPU-rendered apps
/// (games, DAWs like FL Studio, browsers) that a GDI grab captures only partly.
/// The picker calls this on a timer for a live preview.
///
/// Blocking: it starts and stops a short capture — call it off the UI thread.
pub fn window_thumbnail(id: &str, max_dim: u32) -> Result<Thumbnail, CaptureError> {
    let session = start_capture(id)?;
    // Wait for the first composited frame (the backend needs a beat to warm up).
    let deadline = Instant::now() + Duration::from_millis(1200);
    let mut latest: Option<Frame> = None;
    while Instant::now() < deadline {
        match session.frames().recv_timeout(Duration::from_millis(150)) {
            Ok(Some(frame)) => {
                latest = Some(frame);
                break;
            }
            Ok(None) => continue,
            Err(err) => {
                session.stop();
                return Err(err);
            }
        }
    }
    session.stop();
    latest
        .map(|frame| downscale_frame_to_thumbnail(&frame, max_dim.max(1)))
        .ok_or_else(|| CaptureError::Backend("no frame arrived for the thumbnail".into()))
}

/// Box-average downscale of a captured [`Frame`] (BGRA/RGBA, rows padded to
/// `stride`) into tightly-packed opaque RGBA, longer side clamped to `max_dim`.
fn downscale_frame_to_thumbnail(frame: &Frame, max_dim: u32) -> Thumbnail {
    let src_w = frame.width.max(1);
    let src_h = frame.height.max(1);
    let scale = (f64::from(max_dim) / f64::from(src_w.max(src_h))).min(1.0);
    let dst_w = ((f64::from(src_w) * scale).round() as u32).max(1);
    let dst_h = ((f64::from(src_h) * scale).round() as u32).max(1);
    let stride = frame.stride as usize;
    // Channel offsets into each 4-byte source pixel for R, G, B.
    let (ro, go, bo) = match frame.format {
        PixelFormat::Bgra8 => (2usize, 1usize, 0usize),
        PixelFormat::Rgba8 => (0usize, 1usize, 2usize),
    };
    let mut rgba = vec![0u8; (dst_w * dst_h * 4) as usize];
    for dy in 0..dst_h {
        let sy0 = dy * src_h / dst_h;
        let sy1 = ((dy + 1) * src_h / dst_h).max(sy0 + 1).min(src_h);
        for dx in 0..dst_w {
            let sx0 = dx * src_w / dst_w;
            let sx1 = ((dx + 1) * src_w / dst_w).max(sx0 + 1).min(src_w);
            let (mut r, mut g, mut b, mut count) = (0u32, 0u32, 0u32, 0u32);
            for sy in sy0..sy1 {
                let row = sy as usize * stride;
                for sx in sx0..sx1 {
                    let px = row + sx as usize * 4;
                    if px + 3 < frame.data.len() {
                        r += u32::from(frame.data[px + ro]);
                        g += u32::from(frame.data[px + go]);
                        b += u32::from(frame.data[px + bo]);
                        count += 1;
                    }
                }
            }
            let count = count.max(1);
            let o = (dy * dst_w + dx) as usize * 4;
            rgba[o] = (r / count) as u8;
            rgba[o + 1] = (g / count) as u8;
            rgba[o + 2] = (b / count) as u8;
            rgba[o + 3] = 255;
        }
    }
    Thumbnail {
        width: dst_w,
        height: dst_h,
        rgba,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_a_semver_triple() {
        assert_eq!(
            VERSION.split('.').count(),
            3,
            "workspace version should be MAJOR.MINOR.PATCH"
        );
    }

    fn test_frame(tag: u8) -> Frame {
        Frame {
            width: 2,
            height: 1,
            stride: 8,
            format: PixelFormat::Bgra8,
            data: vec![tag; 8],
            captured_at: Instant::now(),
        }
    }

    #[test]
    fn mailbox_delivers_latest_and_counts_drops() {
        let (tx, rx) = frame_channel();
        tx.send(test_frame(1));
        tx.send(test_frame(2)); // overwrites 1
        let got = rx
            .recv_timeout(Duration::from_millis(50))
            .expect("channel open")
            .expect("frame available");
        assert_eq!(got.data[0], 2, "latest frame wins");
        assert_eq!(rx.dropped(), 1, "the overwritten frame is counted");
        assert!(
            rx.recv_timeout(Duration::from_millis(10))
                .expect("still open")
                .is_none(),
            "no frame pending after take"
        );
    }

    #[test]
    fn mailbox_close_reports_the_error() {
        let (tx, rx) = frame_channel();
        tx.close(Some(CaptureError::PermissionDenied));
        match rx.recv_timeout(Duration::from_millis(10)) {
            Err(CaptureError::PermissionDenied) => {}
            other => panic!("expected PermissionDenied, got {other:?}"),
        }
    }

    #[test]
    fn mailbox_drop_of_sender_closes_cleanly() {
        let (tx, rx) = frame_channel();
        drop(tx);
        match rx.recv_timeout(Duration::from_millis(10)) {
            Err(CaptureError::Stopped) => {}
            other => panic!("expected Stopped, got {other:?}"),
        }
    }

    #[test]
    fn mailbox_recv_times_out_without_frames() {
        let (_tx, rx) = frame_channel();
        let started = Instant::now();
        let got = rx.recv_timeout(Duration::from_millis(30)).expect("open");
        assert!(got.is_none());
        assert!(started.elapsed() >= Duration::from_millis(25));
    }

    #[test]
    fn list_sources_never_panics() {
        // CI runners are headless — an error (or an empty list) is fine;
        // panicking is not.
        let _ = list_sources();
    }
}
