//! The app side of live streaming (Phase 5): Go Live / End Stream over
//! `fcap-stream`'s supervised session, fed by the same program-frame readback
//! and mixer taps the recorder uses — on its **own** tap and state, so the
//! stream and the local recording never touch each other.
//!
//! The stream key is a secret: it exists here only inside the publish URL on
//! its way into the sink factory, is never logged, and every visible status
//! carries the service label instead.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, Runtime};

use fcap_audio::RecordTap;
use fcap_encode::{FfmpegSink, RecordSink, RecordSpec, RtmpPlan, VideoCodec};
use fcap_stream::{StreamHandle, StreamSession, StreamSpec, StreamState, StreamTarget};

use crate::audio::AudioRuntime;
use crate::commands::recording::EncodeState;
use crate::settings::SettingsStore;
use crate::studio::StudioState;

struct ActiveStream {
    session: Option<StreamSession>,
    handle: StreamHandle,
    service: String,
}

/// Managed Tauri state: the (single) live stream session.
pub struct StreamBridgeState {
    inner: Mutex<Option<ActiveStream>>,
    /// Serializes Go Live (it does catalog + child I/O before registering).
    starting: AtomicBool,
    /// Lock-free "is a session up" for the render loop's per-frame check.
    active: AtomicBool,
    /// The feed the render loop pushes into (cloned out under one lock).
    feed: Mutex<Option<StreamHandle>>,
    /// The last **failed** DTO, kept after teardown so the UI's failure banner
    /// persists (a clean End Stream clears it; the next Go Live clears it).
    terminal: Mutex<Option<StreamDto>>,
}

impl StreamBridgeState {
    pub fn new() -> Self {
        StreamBridgeState {
            inner: Mutex::new(None),
            starting: AtomicBool::new(false),
            active: AtomicBool::new(false),
            feed: Mutex::new(None),
            terminal: Mutex::new(None),
        }
    }

    fn lock_terminal(&self) -> std::sync::MutexGuard<'_, Option<StreamDto>> {
        self.terminal
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn lock_inner(&self) -> std::sync::MutexGuard<'_, Option<ActiveStream>> {
        self.inner
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// Whether the render loop should hand this state program frames.
    pub fn wants_frames(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }

    /// Push the newest program frame (never blocks; the session drops
    /// honestly when the encoder can't keep up).
    pub fn push_video(&self, pixels: Arc<Vec<u8>>) {
        let feed = self
            .feed
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(handle) = feed.as_ref() {
            handle.push_frame(pixels);
        }
    }

    /// Convert the live session status into the DTO (or the sticky terminal /
    /// idle when no session runs).
    pub fn status(&self) -> StreamDto {
        let inner = self.lock_inner();
        match inner.as_ref() {
            None => self.lock_terminal().clone().unwrap_or_else(StreamDto::idle),
            Some(active) => {
                let status = active.handle.status();
                StreamDto {
                    state: match &status.state {
                        StreamState::Live => "live",
                        StreamState::Reconnecting { .. } => "reconnecting",
                        StreamState::Ended { error: Some(_) } => "failed",
                        StreamState::Ended { error: None } => "ended",
                    }
                    .to_string(),
                    error: match status.state {
                        StreamState::Ended { error } => error,
                        _ => None,
                    },
                    elapsed_sec: status.elapsed.as_secs(),
                    reconnects: status.reconnects,
                    frames_dropped: status.frames_dropped,
                    service: active.service.clone(),
                }
            }
        }
    }
}

impl Default for StreamBridgeState {
    fn default() -> Self {
        Self::new()
    }
}

/// The `stream` event payload / `stream_status` result.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamDto {
    /// "idle" | "live" | "reconnecting" | "failed" | "ended".
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub elapsed_sec: u64,
    pub reconnects: u32,
    pub frames_dropped: u64,
    pub service: String,
}

impl StreamDto {
    fn idle() -> Self {
        StreamDto {
            state: "idle".to_string(),
            error: None,
            elapsed_sec: 0,
            reconnects: 0,
            frames_dropped: 0,
            service: String::new(),
        }
    }
}

fn emit_status<R: Runtime>(app: &AppHandle<R>) {
    let dto = app.state::<StreamBridgeState>().status();
    let _ = app.emit("stream", &dto);
}

struct ResetOnDrop<'a>(&'a AtomicBool);
impl Drop for ResetOnDrop<'_> {
    fn drop(&mut self) {
        self.0.store(false, Ordering::SeqCst);
    }
}

/// Resolve "auto" (or validate an explicit encoder) to a **verified H.264**
/// encoder — FLV/RTMP carries H.264; anything else fails at the ingest.
fn resolve_stream_encoder<R: Runtime>(app: &AppHandle<R>, wanted: &str) -> Result<String, String> {
    let catalog = crate::commands::recording::ensure_catalog(app)?;
    if wanted == "auto" {
        return catalog
            .best(VideoCodec::H264)
            .map(|desc| desc.id.clone())
            .ok_or_else(|| "no usable H.264 encoder was detected".to_string());
    }
    let desc = catalog.get(wanted).ok_or_else(|| {
        format!("encoder {wanted} is not offered on this machine — pick another in Settings")
    })?;
    if desc.verified == Some(false) {
        return Err(format!(
            "encoder {} is unavailable here: {} — pick another in Settings",
            desc.label, desc.note
        ));
    }
    if desc.codec != VideoCodec::H264 {
        return Err("RTMP streaming needs an H.264 encoder".to_string());
    }
    Ok(desc.id.clone())
}

/// Go Live: validate the target, build the supervised session, tap the mixer.
pub fn start<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    let state = app.state::<StreamBridgeState>();
    if state.starting.swap(true, Ordering::SeqCst) {
        return Err("a stream is already starting".to_string());
    }
    let _reset = ResetOnDrop(&state.starting);
    if state.lock_inner().is_some() {
        return Err("a stream is already running".to_string());
    }
    *state.lock_terminal() = None; // a fresh Go Live clears the last failure

    let settings = app.state::<SettingsStore>().get().stream;
    settings.validate()?;
    let target = StreamTarget {
        service: settings.service,
        ingest_url: settings.ingest_url.clone(),
        key: settings.stream_key.clone(),
    };
    let publish_url = target.publish_url().map_err(|err| err.to_string())?;

    let ready = app.state::<EncodeState>().ready_ffmpeg().ok_or_else(|| {
        "streaming needs the ffmpeg component — install it from Components".to_string()
    })?;
    let encoder_id = resolve_stream_encoder(app, &settings.encoder_id)?;

    let snapshot = app.state::<StudioState>().snapshot();
    let (width, height) = (
        snapshot.collection.canvas_width,
        snapshot.collection.canvas_height,
    );

    // Streaming rate control is CBR at the configured bitrate — the shape
    // every RTMP ingest expects.
    let plan = RtmpPlan {
        encoder_id,
        rate_control: fcap_encode::RateControl {
            mode: fcap_encode::RcMode::Cbr,
            bitrate_kbps: settings.bitrate_kbps,
            cq: 23,
        },
        preset: fcap_encode::EncPreset::Performance,
        keyframe_sec: settings.keyframe_sec,
        audio_bitrate_kbps: settings.audio_bitrate_kbps,
        url: publish_url,
    };
    let sink_spec = RecordSpec {
        width,
        height,
        fps: settings.fps,
        tracks: vec![0], // the sink's single lane (the session maps it)
    };
    let factory = {
        let plan = plan.clone();
        let spec = sink_spec.clone();
        Box::new(move || {
            Ok(Box::new(FfmpegSink::spawn_rtmp(&ready, &spec, &plan)?) as Box<dyn RecordSink>)
        })
    };

    let session = StreamSession::start(
        StreamSpec {
            width,
            height,
            fps: settings.fps,
        },
        factory,
    );
    let handle = session.handle();

    // Tap the chosen mixer track — the stream's independent tap.
    let tap_handle = handle.clone();
    app.state::<AudioRuntime>()
        .engine
        .set_stream_tap(Some(RecordTap {
            tracks: 1 << (settings.track - 1),
            sink: Box::new(move |blocks| {
                if let Some((_, samples)) = blocks.first() {
                    tap_handle.push_audio(samples);
                }
            }),
        }));

    *state
        .feed
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(handle.clone());
    *state.lock_inner() = Some(ActiveStream {
        session: Some(session),
        handle,
        service: settings.service.label().to_string(),
    });
    state.active.store(true, Ordering::Relaxed);
    emit_status(app);
    println!("stream: live → {}", settings.service.label());

    // TASK-508: optionally bring the local recording up with the stream —
    // best-effort, and its failure never stops the stream.
    if settings.auto_record
        && !app
            .state::<crate::recording::RecordingState>()
            .wants_frames()
    {
        if let Err(err) = crate::recording::start(app) {
            eprintln!("stream: auto-record could not start: {err}");
        }
    }
    Ok(())
}

/// Tear the session down (drop the tap, feed, and session; join the
/// supervisor's RTMP flush) and set the sticky terminal DTO — `None` for a
/// clean end, `Some(failed)` when the retries were spent. Idempotent.
fn teardown<R: Runtime>(app: &AppHandle<R>, terminal: Option<StreamDto>) {
    let state = app.state::<StreamBridgeState>();
    let session = state
        .lock_inner()
        .as_mut()
        .and_then(|active| active.session.take());
    state.active.store(false, Ordering::Relaxed);
    *state
        .feed
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = None;
    app.state::<AudioRuntime>().engine.set_stream_tap(None);
    if let Some(session) = session {
        let _ = session.stop();
    }
    *state.lock_inner() = None;
    *state.lock_terminal() = terminal;
    emit_status(app);
}

/// End Stream: flush the goodbye, drop the tap, report the final status.
pub fn stop<R: Runtime>(app: &AppHandle<R>) -> Result<StreamDto, String> {
    if app.state::<StreamBridgeState>().lock_inner().is_none() {
        return Err("no stream is running".to_string());
    }
    teardown(app, None); // a deliberate End Stream clears any failure
    println!("stream: ended");
    Ok(StreamDto::idle())
}

// -- commands -----------------------------------------------------------------

/// Go Live with the configured target. Off the UI thread — first-run encoder
/// detection + the ffmpeg child spawn are blocking.
#[tauri::command]
pub async fn stream_start<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || start(&app))
        .await
        .map_err(|err| format!("stream start task failed: {err}"))?
}

/// End the stream. Off the UI thread — the supervisor join flushes the RTMP
/// goodbye, which can block on a stalled connection.
#[tauri::command]
pub async fn stream_stop<R: Runtime>(app: AppHandle<R>) -> Result<StreamDto, String> {
    tauri::async_runtime::spawn_blocking(move || stop(&app))
        .await
        .map_err(|err| format!("stream stop task failed: {err}"))?
}

/// The current stream status (the `stream` event pushes the same shape).
#[tauri::command]
pub fn stream_status(state: tauri::State<'_, StreamBridgeState>) -> StreamDto {
    state.status()
}

/// ~1 Hz status while a session runs (the elapsed clock + honest health);
/// winds down when the app is gone.
pub fn spawn_status_thread<R: Runtime>(app: AppHandle<R>) {
    std::thread::Builder::new()
        .name("fcap-stream-status".into())
        .spawn(move || loop {
            let state = app.state::<StreamBridgeState>();
            let has_session = state.lock_inner().is_some();
            if has_session {
                // A session that ended on its own (retries spent) tears down
                // here, KEEPING the failed DTO as the sticky terminal so the
                // UI's failure banner persists (teardown does not overwrite it
                // with idle).
                let ended = state
                    .lock_inner()
                    .as_ref()
                    .map(|active| active.handle.status().state);
                match ended {
                    Some(StreamState::Ended { .. }) => {
                        let dto = state.status(); // the failed DTO
                        teardown(&app, Some(dto));
                    }
                    _ if app.emit("stream", &state.status()).is_err() => return,
                    _ => {}
                }
            }
            std::thread::sleep(Duration::from_secs(1));
        })
        .expect("stream status thread spawns");
}
