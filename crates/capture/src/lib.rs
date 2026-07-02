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
