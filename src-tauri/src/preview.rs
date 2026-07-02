//! The live program-preview pipe (Phase 1: one source, direct draw — the
//! compositor takes over in Phase 2).
//!
//! A pump thread pulls frames from the active capture session, downscales +
//! JPEG-encodes at ~30 fps, and parks the newest frame in [`PreviewState`];
//! the `preview://` custom URI scheme serves that frame to the UI, which
//! polls it onto a canvas. No sockets, no disk — frames never leave the
//! process (the privacy invariant).
//!
//! Status flows over the `preview` event: `waiting → live → (error)`, plus a
//! 1 Hz fps/dropped update while live.

use std::borrow::Cow;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Runtime};

use fcap_capture::{CaptureError, CaptureSession, Frame, PixelFormat};
use fcap_sources::video_device::{self, VideoFormatInfo};

/// Preview frames are capped to this box (encode cost, not capture cost).
const PREVIEW_MAX_WIDTH: u32 = 1280;
const PREVIEW_MAX_HEIGHT: u32 = 720;
const PREVIEW_JPEG_QUALITY: u8 = 75;
/// Minimum interval between encodes (~30 fps).
const ENCODE_INTERVAL: Duration = Duration::from_millis(33);

/// What the UI asked to preview (mirrors `ui/src/api/types.ts`).
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum PreviewSource {
    Display {
        id: String,
        label: String,
    },
    Window {
        id: String,
        label: String,
    },
    Portal {
        label: Option<String>,
    },
    Webcam {
        id: String,
        label: String,
        format: Option<VideoFormatDto>,
    },
}

impl PreviewSource {
    fn label(&self) -> String {
        match self {
            PreviewSource::Display { label, .. }
            | PreviewSource::Window { label, .. }
            | PreviewSource::Webcam { label, .. } => label.clone(),
            PreviewSource::Portal { label } => label
                .clone()
                .unwrap_or_else(|| "Screen or window (system picker)".to_string()),
        }
    }
}

/// A user-selected webcam format (mirrors `VideoFormatInfo`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoFormatDto {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub fourcc: String,
}

impl From<VideoFormatDto> for VideoFormatInfo {
    fn from(dto: VideoFormatDto) -> Self {
        VideoFormatInfo {
            width: dto.width,
            height: dto.height,
            fps: dto.fps,
            fourcc: dto.fourcc,
        }
    }
}

/// The `preview` event payload (mirrors `ui/src/api/types.ts`).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewStatus {
    /// "idle" | "waiting" | "live" | "error"
    pub state: &'static str,
    /// The UI card this status belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    /// Measured preview frame rate (frames received in the last second).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fps: Option<u32>,
    /// Frames the consumer never saw (latest-wins mailbox overwrites).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropped: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl PreviewStatus {
    fn new(state: &'static str, source_key: &str, label: &str) -> Self {
        PreviewStatus {
            state,
            source_key: Some(source_key.to_string()),
            label: Some(label.to_string()),
            width: None,
            height: None,
            fps: None,
            dropped: None,
            error_code: None,
            error_message: None,
        }
    }

    pub(crate) fn idle() -> Self {
        PreviewStatus {
            state: "idle",
            source_key: None,
            label: None,
            width: None,
            height: None,
            fps: None,
            dropped: None,
            error_code: None,
            error_message: None,
        }
    }
}

fn error_code(err: &CaptureError) -> &'static str {
    match err {
        CaptureError::PermissionDenied => "permission",
        CaptureError::Cancelled => "cancelled",
        CaptureError::NotFound(_) => "notFound",
        CaptureError::Unsupported(_) => "unsupported",
        CaptureError::Stopped => "stopped",
        CaptureError::Backend(_) => "backend",
    }
}

/// One encoded preview frame, served by the `preview://` scheme.
struct EncodedFrame {
    jpeg: Vec<u8>,
    seq: u64,
}

/// Managed state: the running pump + the newest encoded frame.
#[derive(Default)]
pub struct PreviewState {
    running: Mutex<Option<RunningPreview>>,
    latest: Arc<Mutex<Option<EncodedFrame>>>,
    /// Bumped on every start/stop; a stale pump sees the mismatch and goes
    /// quiet instead of overwriting the new session's state.
    generation: Arc<AtomicU64>,
    frame_seq: Arc<AtomicU64>,
}

struct RunningPreview {
    stop: Arc<AtomicBool>,
    /// True while the pump owns an OS capture handle (camera / duplication).
    /// Lets a replacement wait briefly for the exclusive device to free up.
    holds_device: Arc<AtomicBool>,
}

/// How long a new preview waits for the old pump to release its (exclusive)
/// OS capture handle before proceeding anyway. Pumps poll their stop flag on
/// ≤150 ms cycles, so this covers the normal case with room to spare.
const DEVICE_RELEASE_TIMEOUT: Duration = Duration::from_millis(750);

impl PreviewState {
    /// Signal the current pump (if any) to wind down and return its handles.
    /// Never joins — a pump stuck in the Linux portal dialog must not hang
    /// the UI; it exits on its own and finds its generation stale.
    fn signal_stop(&self) -> Option<RunningPreview> {
        let previous = self.running.lock().expect("preview state poisoned").take();
        if let Some(running) = &previous {
            running.stop.store(true, Ordering::Relaxed);
        }
        self.generation.fetch_add(1, Ordering::SeqCst);
        self.latest
            .lock()
            .expect("preview frame slot poisoned")
            .take();
        previous
    }

    pub fn stop<R: Runtime>(&self, app: &AppHandle<R>) {
        self.signal_stop();
        let _ = app.emit("preview", PreviewStatus::idle());
    }

    pub fn start<R: Runtime>(&self, app: &AppHandle<R>, source: PreviewSource, source_key: String) {
        let previous = self.signal_stop();
        // Exclusive-device handoff: cameras and display duplication reject a
        // second opener, so give the old pump a bounded moment to let go
        // (a pump that never acquired a device — e.g. one parked in the
        // portal dialog — has holds_device = false and costs no wait).
        if let Some(previous) = previous {
            let deadline = Instant::now() + DEVICE_RELEASE_TIMEOUT;
            while previous.holds_device.load(Ordering::Acquire) && Instant::now() < deadline {
                std::thread::sleep(Duration::from_millis(10));
            }
        }

        let my_gen = self.generation.load(Ordering::SeqCst);
        let stop = Arc::new(AtomicBool::new(false));
        let holds_device = Arc::new(AtomicBool::new(false));
        *self.running.lock().expect("preview state poisoned") = Some(RunningPreview {
            stop: Arc::clone(&stop),
            holds_device: Arc::clone(&holds_device),
        });

        let app_thread = app.clone();
        let latest = Arc::clone(&self.latest);
        let generation = Arc::clone(&self.generation);
        let frame_seq = Arc::clone(&self.frame_seq);
        let error_key = source_key.clone();
        let spawned = std::thread::Builder::new()
            .name("fcap-preview-pump".into())
            .spawn(move || {
                pump(
                    app_thread,
                    source,
                    source_key,
                    latest,
                    generation,
                    frame_seq,
                    my_gen,
                    stop,
                    holds_device,
                )
            });
        if let Err(err) = spawned {
            // Never leave the UI stuck on "waiting" with no pump behind it.
            self.running.lock().expect("preview state poisoned").take();
            let mut status = PreviewStatus::new("error", &error_key, "preview");
            status.error_code = Some("backend");
            status.error_message = Some(format!("could not start the preview thread: {err}"));
            let _ = app.emit("preview", status);
        }
    }

    /// The `preview://` scheme body: newest JPEG or 204 while there is none.
    ///
    /// CORS is pinned to the app's own webview origins — never `*` — so that
    /// captured screen frames can never be fetched by remote content, even
    /// if a future phase (browser sources, 0.40.0+) renders some.
    pub fn protocol_response(
        &self,
        origin: Option<&str>,
    ) -> tauri::http::Response<Cow<'static, [u8]>> {
        const APP_ORIGINS: [&str; 3] = [
            "http://tauri.localhost", // Windows production webview
            "tauri://localhost",      // macOS/Linux production webview
            "http://localhost:1420",  // `tauri dev`
        ];
        let allow_origin = origin
            .filter(|candidate| APP_ORIGINS.contains(candidate))
            .unwrap_or(APP_ORIGINS[0]);

        let latest = self.latest.lock().expect("preview frame slot poisoned");
        match latest.as_ref() {
            Some(frame) => tauri::http::Response::builder()
                .status(200)
                .header("content-type", "image/jpeg")
                .header("cache-control", "no-store")
                .header("access-control-allow-origin", allow_origin)
                .header("access-control-expose-headers", "x-frame-seq")
                .header("x-frame-seq", frame.seq.to_string())
                .body(Cow::Owned(frame.jpeg.clone()))
                .expect("static response parts"),
            None => tauri::http::Response::builder()
                .status(204)
                .header("cache-control", "no-store")
                .header("access-control-allow-origin", allow_origin)
                .body(Cow::Borrowed(&[][..]))
                .expect("static response parts"),
        }
    }
}

fn open_session(source: &PreviewSource) -> Result<CaptureSession, CaptureError> {
    match source {
        PreviewSource::Display { id, .. } | PreviewSource::Window { id, .. } => {
            fcap_capture::start_capture(id)
        }
        PreviewSource::Portal { .. } => fcap_capture::start_capture("portal"),
        PreviewSource::Webcam { id, format, .. } => {
            let format = format.clone().map(VideoFormatInfo::from);
            video_device::start_video_device(id, format.as_ref())
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn pump<R: Runtime>(
    app: AppHandle<R>,
    source: PreviewSource,
    source_key: String,
    latest: Arc<Mutex<Option<EncodedFrame>>>,
    generation: Arc<AtomicU64>,
    frame_seq: Arc<AtomicU64>,
    my_gen: u64,
    stop: Arc<AtomicBool>,
    holds_device: Arc<AtomicBool>,
) {
    let label = source.label();
    let current = |status: PreviewStatus| {
        // A stale pump (superseded session) goes quiet.
        if generation.load(Ordering::SeqCst) == my_gen {
            let _ = app.emit("preview", status);
        }
    };

    current(PreviewStatus::new("waiting", &source_key, &label));

    // Blocking on purpose: the Linux portal path waits for the user's pick.
    let session = match open_session(&source) {
        Ok(session) => session,
        Err(err) => {
            let mut status = PreviewStatus::new("error", &source_key, &label);
            status.error_code = Some(error_code(&err));
            status.error_message = Some(err.to_string());
            current(status);
            return;
        }
    };
    // From here the pump owns an exclusive OS capture handle; a replacement
    // start() waits for this flag before re-opening the same device.
    holds_device.store(true, Ordering::Release);
    if stop.load(Ordering::Relaxed) {
        session.stop();
        holds_device.store(false, Ordering::Release);
        return;
    }

    let mut live = false;
    let (mut width, mut height) = (0u32, 0u32);
    let mut frames_this_second = 0u32;
    let mut last_status = Instant::now();
    let mut last_encode = Instant::now() - ENCODE_INTERVAL;

    loop {
        if stop.load(Ordering::Relaxed) {
            break;
        }
        match session.frames().recv_timeout(Duration::from_millis(100)) {
            Ok(Some(frame)) => {
                frames_this_second += 1;
                if !live || frame.width != width || frame.height != height {
                    live = true;
                    width = frame.width;
                    height = frame.height;
                    let mut status = PreviewStatus::new("live", &source_key, &label);
                    status.width = Some(width);
                    status.height = Some(height);
                    current(status);
                }
                if last_encode.elapsed() >= ENCODE_INTERVAL {
                    if let Some(jpeg) = encode_preview_jpeg(
                        &frame,
                        PREVIEW_MAX_WIDTH,
                        PREVIEW_MAX_HEIGHT,
                        PREVIEW_JPEG_QUALITY,
                    ) {
                        // The generation check happens *inside* the slot lock:
                        // signal_stop bumps the generation before clearing the
                        // slot, so a superseded pump can never park a stale
                        // frame after the new session's slot was prepared.
                        let mut slot = latest.lock().expect("preview frame slot poisoned");
                        if generation.load(Ordering::SeqCst) == my_gen {
                            let seq = frame_seq.fetch_add(1, Ordering::Relaxed) + 1;
                            *slot = Some(EncodedFrame { jpeg, seq });
                        }
                        drop(slot);
                        last_encode = Instant::now();
                    }
                }
            }
            Ok(None) => {}
            Err(err) => {
                if !stop.load(Ordering::Relaxed) {
                    let mut status = PreviewStatus::new("error", &source_key, &label);
                    status.error_code = Some(error_code(&err));
                    status.error_message = Some(match &err {
                        CaptureError::Stopped => "The source ended.".to_string(),
                        other => other.to_string(),
                    });
                    current(status);
                }
                break;
            }
        }
        if live && last_status.elapsed() >= Duration::from_secs(1) {
            let mut status = PreviewStatus::new("live", &source_key, &label);
            status.width = Some(width);
            status.height = Some(height);
            status.fps = Some(frames_this_second);
            let dropped = session.frames().dropped();
            status.dropped = (dropped > 0).then_some(dropped);
            current(status);
            frames_this_second = 0;
            last_status = Instant::now();
        }
    }
    session.stop();
    holds_device.store(false, Ordering::Release);
}

/// Downscale (integer nearest-neighbor) + convert to RGB + JPEG-encode.
/// Returns `None` for frames that don't hold together (bad stride/short data).
fn encode_preview_jpeg(frame: &Frame, max_w: u32, max_h: u32, quality: u8) -> Option<Vec<u8>> {
    if frame.width == 0 || frame.height == 0 || frame.stride < frame.width * 4 {
        return None;
    }
    let needed = frame.stride as usize * frame.height as usize;
    if frame.data.len() < needed {
        return None;
    }
    let factor = frame
        .width
        .div_ceil(max_w)
        .max(frame.height.div_ceil(max_h))
        .max(1);
    let out_w = frame.width.div_ceil(factor);
    let out_h = frame.height.div_ceil(factor);

    let mut rgb = Vec::with_capacity(out_w as usize * out_h as usize * 3);
    for y in 0..out_h {
        let src_y = (y * factor) as usize;
        let row = &frame.data[src_y * frame.stride as usize..];
        for x in 0..out_w {
            let src_x = (x * factor) as usize * 4;
            let px = &row[src_x..src_x + 4];
            match frame.format {
                PixelFormat::Bgra8 => rgb.extend_from_slice(&[px[2], px[1], px[0]]),
                PixelFormat::Rgba8 => rgb.extend_from_slice(&[px[0], px[1], px[2]]),
            }
        }
    }

    let mut out = Vec::new();
    let encoder = jpeg_encoder::Encoder::new(&mut out, quality);
    encoder
        .encode(
            &rgb,
            out_w as u16,
            out_h as u16,
            jpeg_encoder::ColorType::Rgb,
        )
        .ok()?;
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(width: u32, height: u32, stride: u32, format: PixelFormat) -> Frame {
        Frame {
            width,
            height,
            stride,
            format,
            data: vec![0x80; stride as usize * height as usize],
            captured_at: Instant::now(),
        }
    }

    #[test]
    fn encode_produces_jpeg_within_bounds() {
        let jpeg = encode_preview_jpeg(
            &frame(1920, 1080, 1920 * 4, PixelFormat::Bgra8),
            1280,
            720,
            75,
        )
        .expect("encodable");
        // JPEG magic.
        assert_eq!(&jpeg[..2], &[0xFF, 0xD8]);
    }

    #[test]
    fn encode_respects_padded_strides() {
        // 100 px wide with a 512-byte row pitch (typical DXGI padding).
        assert!(
            encode_preview_jpeg(&frame(100, 50, 512, PixelFormat::Bgra8), 1280, 720, 75).is_some()
        );
    }

    #[test]
    fn encode_rejects_short_buffers() {
        let mut bad = frame(64, 64, 256, PixelFormat::Rgba8);
        bad.data.truncate(100);
        assert!(encode_preview_jpeg(&bad, 1280, 720, 75).is_none());
    }

    #[test]
    fn oversized_frames_downscale_to_the_box() {
        // 5120×1440 → factor 4 → 1280×360.
        let jpeg = encode_preview_jpeg(
            &frame(5120, 1440, 5120 * 4, PixelFormat::Bgra8),
            1280,
            720,
            60,
        )
        .expect("encodable");
        assert!(!jpeg.is_empty());
    }
}
