//! The app side of live streaming: Go Live / End Stream over `fcap-stream`'s
//! multistream engine (Phase 6), fed by the same program-frame readback and
//! mixer taps the recorder uses — on its **own** tap and state, so the stream
//! and the local recording never touch each other.
//!
//! Go Live publishes to **every enabled target at once**, direct to each
//! platform. Targets with equal encode settings share one encode (the tee
//! lane); different settings encode separately. Each target reports its own
//! health + bitrate, and one dead target never takes a healthy one down.
//!
//! The stream keys are secrets: they exist here only inside the publish URLs
//! on their way into the lane maker, are never logged, and every visible
//! status carries service labels instead.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, Runtime};

use fcap_audio::RecordTap;
use fcap_encode::{
    tee_safe, EncPreset, FfmpegSink, RateControl, RcMode, RecordSink, RecordSpec, RtmpMonitor,
    RtmpPlan, VideoCodec,
};
use fcap_stream::{
    LaneCells, LaneIo, LaneMaker, MemberSpec, MemberStatus, MultiHandle, MultiSession,
    StreamProtocol, StreamState, StreamTarget,
};

use crate::audio::AudioRuntime;
use crate::commands::recording::EncodeState;
use crate::settings::SettingsStore;
use crate::studio::StudioState;

struct ActiveStream {
    session: Option<MultiSession>,
    handle: MultiHandle,
    services: String,
}

/// Managed Tauri state: the (single) live multistream.
pub struct StreamBridgeState {
    inner: Mutex<Option<ActiveStream>>,
    /// Serializes Go Live (it does catalog + child I/O before registering).
    starting: AtomicBool,
    /// Lock-free "is a session up" for the render loop's per-frame check.
    active: AtomicBool,
    /// Which canvases this session's targets consume (set at Go Live).
    wants_main: AtomicBool,
    wants_vertical: AtomicBool,
    /// The feed the render loop pushes into (cloned out under one lock).
    feed: Mutex<Option<MultiHandle>>,
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
            wants_main: AtomicBool::new(false),
            wants_vertical: AtomicBool::new(false),
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
        self.active.load(Ordering::Relaxed) && self.wants_main.load(Ordering::Relaxed)
    }

    /// Whether the render loop should hand this state vertical-canvas frames.
    pub fn wants_vertical_frames(&self) -> bool {
        self.active.load(Ordering::Relaxed) && self.wants_vertical.load(Ordering::Relaxed)
    }

    /// Push the newest program frame (never blocks; a lane drops honestly
    /// when its encoder can't keep up).
    pub fn push_video(&self, pixels: Arc<Vec<u8>>) {
        let feed = self
            .feed
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(handle) = feed.as_ref() {
            handle.push_frame(0, pixels);
        }
    }

    /// Push the newest vertical-canvas frame to its lanes.
    pub fn push_video_vertical(&self, pixels: Arc<Vec<u8>>) {
        let feed = self
            .feed
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(handle) = feed.as_ref() {
            handle.push_frame(1, pixels);
        }
    }

    /// Convert the live session statuses into the DTO (or the sticky
    /// terminal / idle when no session runs).
    pub fn status(&self) -> StreamDto {
        let inner = self.lock_inner();
        match inner.as_ref() {
            None => self.lock_terminal().clone().unwrap_or_else(StreamDto::idle),
            Some(active) => {
                let statuses = active.handle.statuses();
                let (reconnects, frames_dropped) = active.handle.totals();
                let targets: Vec<StreamTargetDto> =
                    statuses.iter().map(StreamTargetDto::from).collect();
                StreamDto {
                    state: aggregate_state(&statuses).to_string(),
                    error: statuses.iter().find_map(|status| match &status.state {
                        StreamState::Ended { error: Some(err) } => {
                            Some(format!("{}: {err}", status.label))
                        }
                        _ => None,
                    }),
                    elapsed_sec: active.handle.elapsed().as_secs(),
                    reconnects,
                    frames_dropped,
                    service: active.services.clone(),
                    targets,
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

/// The one honest rollup for the Go Live button: live while anything
/// publishes, reconnecting while anything retries, failed only when it's
/// over and something died.
fn aggregate_state(statuses: &[MemberStatus]) -> &'static str {
    if statuses
        .iter()
        .any(|status| status.state == StreamState::Live)
    {
        "live"
    } else if statuses
        .iter()
        .any(|status| matches!(status.state, StreamState::Reconnecting { .. }))
    {
        "reconnecting"
    } else if statuses
        .iter()
        .any(|status| matches!(status.state, StreamState::Ended { error: Some(_) }))
    {
        "failed"
    } else {
        "ended"
    }
}

/// One target's slice of the `stream` event payload.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamTargetDto {
    /// The settings row this target came from.
    pub id: usize,
    pub label: String,
    /// "live" | "reconnecting" | "failed" | "ended".
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub reconnects: u32,
    pub frames_dropped: u64,
    /// Publish bitrate: measured where the muxer reports it, the configured
    /// rate on a shared (tee) lane, 0 while down.
    pub kbps: u32,
    /// How many other targets share this target's encode.
    pub shared: usize,
}

impl From<&MemberStatus> for StreamTargetDto {
    fn from(status: &MemberStatus) -> Self {
        StreamTargetDto {
            id: status.id,
            label: status.label.clone(),
            state: match &status.state {
                StreamState::Live => "live",
                StreamState::Reconnecting { .. } => "reconnecting",
                StreamState::Ended { error: Some(_) } => "failed",
                StreamState::Ended { error: None } => "ended",
            }
            .to_string(),
            error: match &status.state {
                StreamState::Ended { error } => error.clone(),
                _ => None,
            },
            reconnects: status.reconnects,
            frames_dropped: status.frames_dropped,
            kbps: status.kbps,
            shared: status.shared_with,
        }
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
    /// The enabled services, joined (e.g. "Twitch + YouTube").
    pub service: String,
    /// Per-target health + bitrate (empty when idle).
    pub targets: Vec<StreamTargetDto>,
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
            targets: Vec::new(),
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
pub(crate) fn resolve_stream_encoder<R: Runtime>(
    app: &AppHandle<R>,
    wanted: &str,
) -> Result<String, String> {
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
        return Err(
            "streaming needs an H.264 encoder (RTMP, SRT and WHIP all carry it)".to_string(),
        );
    }
    Ok(desc.id.clone())
}

/// Everything one target needs at spawn time. Holds the publish URL (key
/// included) and the WHIP bearer — no `Debug`, never logged.
#[derive(Clone)]
struct TargetPlan {
    id: usize,
    label: String,
    url: String,
    encoder_id: String,
    bitrate_kbps: u32,
    audio_bitrate_kbps: u32,
    keyframe_sec: f32,
    fps: u32,
    track: u8,
    /// 0 = program canvas, 1 = vertical; with that canvas's dimensions.
    canvas: u8,
    width: u32,
    height: u32,
    /// Publish at this size instead of the canvas size (TASK-609).
    scale: Option<(u32, u32)>,
    protocol: StreamProtocol,
    /// The WHIP bearer token (SECRET) — the target's key, header-borne.
    auth: Option<String>,
}

/// Go Live: validate every enabled target, build the multistream engine
/// (shared encodes where settings match), tap the mixer once.
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
    let enabled: Vec<(usize, &crate::settings::StreamTargetSettings)> = settings
        .targets
        .iter()
        .enumerate()
        .filter(|(_, target)| target.enabled)
        .collect();
    if enabled.is_empty() {
        return Err("no stream target is enabled — add one in ⦿ Stream settings".to_string());
    }

    let ready = app.state::<EncodeState>().ready_ffmpeg().ok_or_else(|| {
        "streaming needs the ffmpeg component — install it from Components".to_string()
    })?;

    // SRT/WHIP ride the installed ffmpeg's own capabilities — probe them
    // honestly instead of failing with a cryptic spawn error.
    let needs = |protocol: StreamProtocol| {
        enabled
            .iter()
            .any(|(_, target)| target.service.protocol() == protocol)
    };
    if needs(StreamProtocol::Srt) || needs(StreamProtocol::Whip) {
        let support = fcap_encode::stream_support(&ready);
        if needs(StreamProtocol::Srt) && !support.srt {
            return Err(
                "the installed ffmpeg component has no SRT support — reinstall it from Components"
                    .to_string(),
            );
        }
        if needs(StreamProtocol::Whip) && !support.whip {
            return Err(
                "the installed ffmpeg component has no WHIP muxer (needs ffmpeg 7.1+) — reinstall it from Components"
                    .to_string(),
            );
        }
    }

    // Canvas geometry per target: the program canvas, or the second
    // (vertical) canvas when the target picks it — which must exist.
    let snapshot = app.state::<StudioState>().snapshot();
    let main_dims = (
        snapshot.collection.canvas_width,
        snapshot.collection.canvas_height,
    );
    let vertical_dims = snapshot
        .collection
        .vertical
        .map(|vertical| (vertical.width, vertical.height));

    let mut plans: Vec<TargetPlan> = Vec::new();
    for (id, target_settings) in &enabled {
        let target = StreamTarget {
            service: target_settings.service,
            ingest_url: target_settings.ingest_url.clone(),
            key: target_settings.stream_key.clone(),
        };
        let url = target
            .publish_url()
            .map_err(|err| format!("{}: {err}", target_settings.service.label()))?;
        let encoder_id = resolve_stream_encoder(app, &target_settings.encoder_id)
            .map_err(|err| format!("{}: {err}", target_settings.service.label()))?;
        let protocol = target_settings.service.protocol();
        let (canvas, (width, height)) = match target_settings.canvas {
            crate::settings::StreamCanvas::Main => (0u8, main_dims),
            crate::settings::StreamCanvas::Vertical => (
                1u8,
                vertical_dims.ok_or_else(|| {
                    format!(
                        "{} streams the vertical canvas — enable one in the studio first",
                        target_settings.service.label()
                    )
                })?,
            ),
        };
        plans.push(TargetPlan {
            id: *id,
            label: target_settings.service.label().to_string(),
            url,
            encoder_id,
            bitrate_kbps: target_settings.bitrate_kbps,
            audio_bitrate_kbps: target_settings.audio_bitrate_kbps,
            keyframe_sec: target_settings.keyframe_sec,
            fps: target_settings.fps,
            track: target_settings.track,
            canvas,
            width,
            height,
            scale: (target_settings.output_width > 0 && target_settings.output_height > 0)
                .then_some((target_settings.output_width, target_settings.output_height)),
            protocol,
            auth: (protocol == StreamProtocol::Whip
                && !target_settings.stream_key.trim().is_empty())
            .then(|| target_settings.stream_key.trim().to_string()),
        });
    }

    // The signature decides encode sharing: everything that shapes the
    // bitstream, including the canvas and the audio track/codec feeding it.
    // RTMP and SRT both carry AAC so they can share; WHIP is Opus and never
    // does; the two canvases are different pictures and never share.
    let members: Vec<MemberSpec> = plans
        .iter()
        .map(|plan| MemberSpec {
            id: plan.id,
            label: plan.label.clone(),
            track: plan.track,
            canvas: plan.canvas,
            width: plan.width,
            height: plan.height,
            fps: plan.fps,
            signature: format!(
                "{}|{}|{}|{}|{}|{}|{}|c{}|s{:?}",
                plan.encoder_id,
                plan.bitrate_kbps,
                plan.audio_bitrate_kbps,
                plan.keyframe_sec.to_bits(),
                plan.fps,
                plan.track,
                if plan.protocol == StreamProtocol::Whip {
                    "opus"
                } else {
                    "aac"
                },
                plan.canvas,
                plan.scale
            ),
            tee_safe: plan.protocol != StreamProtocol::Whip && tee_safe(&plan.url),
            nominal_kbps: plan.bitrate_kbps + plan.audio_bitrate_kbps,
        })
        .collect();

    let maker: LaneMaker = {
        let plans = plans.clone();
        Box::new(move |ids: &[usize]| {
            let cells = LaneCells::new(ids);
            let members_cell = Arc::clone(&cells.members);
            let spawn_order = Arc::clone(&cells.spawn_order);
            let failures = Arc::clone(&cells.slave_failures);
            let bytes_out = Arc::clone(&cells.bytes_out);
            let plans = plans.clone();
            let ready = ready.clone();
            LaneIo {
                factory: Box::new(move || {
                    // Publish to the lane's *current* members — a target the
                    // engine split out must not ride this lane's respawn.
                    let ids: Vec<usize> = members_cell
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner)
                        .clone();
                    let lane_plans: Vec<&TargetPlan> = ids
                        .iter()
                        .filter_map(|id| plans.iter().find(|plan| plan.id == *id))
                        .collect();
                    let Some(first) = lane_plans.first() else {
                        return Err("this lane has no targets left".to_string());
                    };
                    let plan = RtmpPlan {
                        encoder_id: first.encoder_id.clone(),
                        rate_control: RateControl {
                            mode: RcMode::Cbr,
                            bitrate_kbps: first.bitrate_kbps,
                            cq: 23,
                        },
                        preset: EncPreset::Performance,
                        keyframe_sec: first.keyframe_sec,
                        audio_bitrate_kbps: first.audio_bitrate_kbps,
                        urls: lane_plans.iter().map(|plan| plan.url.clone()).collect(),
                        scale: first.scale,
                        auth_bearer: first.auth.clone(),
                    };
                    let spec = RecordSpec {
                        width: first.width,
                        height: first.height,
                        fps: first.fps,
                        tracks: vec![0], // the sink's single lane
                    };
                    // Fresh spawn: reset telemetry so a stale slave report
                    // can never mismap onto the new member order.
                    failures
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner)
                        .clear();
                    *spawn_order
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner) = ids;
                    let monitor = RtmpMonitor {
                        slave_failures: Arc::clone(&failures),
                        bytes_out: Arc::clone(&bytes_out),
                    };
                    Ok(
                        Box::new(FfmpegSink::spawn_rtmp(&ready, &spec, &plan, &monitor)?)
                            as Box<dyn RecordSink>,
                    )
                }),
                cells,
            }
        })
    };

    let mut services: Vec<String> = Vec::new();
    for plan in &plans {
        if !services.contains(&plan.label) {
            services.push(plan.label.clone());
        }
    }
    let services = services.join(" + ");

    let session = MultiSession::start(members, maker);
    let handle = session.handle();

    // One mixer tap for every streamed track; the handle routes each block
    // to the lanes streaming that track.
    let mut mask: u8 = 0;
    for plan in &plans {
        mask |= 1 << (plan.track - 1);
    }
    let tap_handle = handle.clone();
    app.state::<AudioRuntime>()
        .engine
        .set_stream_tap(Some(RecordTap {
            tracks: mask,
            sink: Box::new(move |blocks| {
                for (track_index, samples) in blocks {
                    tap_handle.push_audio(*track_index, samples);
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
        services: services.clone(),
    });
    state
        .wants_main
        .store(plans.iter().any(|plan| plan.canvas == 0), Ordering::Relaxed);
    state
        .wants_vertical
        .store(plans.iter().any(|plan| plan.canvas == 1), Ordering::Relaxed);
    state.active.store(true, Ordering::Relaxed);
    emit_status(app);
    println!("stream: live → {services}");

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

/// Tear the session down (drop the tap, feed, and session; join the lanes'
/// RTMP flushes) and set the sticky terminal DTO — `None` for a clean end,
/// `Some(failed)` when a target spent its retries. Idempotent.
fn teardown<R: Runtime>(app: &AppHandle<R>, terminal: Option<StreamDto>) {
    let state = app.state::<StreamBridgeState>();
    let session = state
        .lock_inner()
        .as_mut()
        .and_then(|active| active.session.take());
    state.active.store(false, Ordering::Relaxed);
    state.wants_main.store(false, Ordering::Relaxed);
    state.wants_vertical.store(false, Ordering::Relaxed);
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

/// End Stream: flush the goodbyes, drop the tap, report the final status.
pub fn stop<R: Runtime>(app: &AppHandle<R>) -> Result<StreamDto, String> {
    if app.state::<StreamBridgeState>().lock_inner().is_none() {
        return Err("no stream is running".to_string());
    }
    teardown(app, None); // a deliberate End Stream clears any failure
    println!("stream: ended");
    Ok(StreamDto::idle())
}

// -- commands -----------------------------------------------------------------

/// Go Live with every enabled target. Off the UI thread — first-run encoder
/// detection + the ffmpeg child spawns are blocking.
#[tauri::command]
pub async fn stream_start<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || start(&app))
        .await
        .map_err(|err| format!("stream start task failed: {err}"))?
}

/// End the stream. Off the UI thread — the lane joins flush the RTMP
/// goodbyes, which can block on a stalled connection.
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

/// ~1 Hz status while a session runs (the elapsed clock + honest per-target
/// health); winds down when the app is gone.
pub fn spawn_status_thread<R: Runtime>(app: AppHandle<R>) {
    std::thread::Builder::new()
        .name("fcap-stream-status".into())
        .spawn(move || loop {
            let state = app.state::<StreamBridgeState>();
            let has_session = state.lock_inner().is_some();
            if has_session {
                // A stream where EVERY target ended on its own (retries
                // spent / all done) tears down here, keeping a failed DTO
                // as the sticky terminal so the UI's failure banner
                // persists. While any target still publishes or retries,
                // the stream stays up — one dead target never ends it.
                let all_ended = state.lock_inner().as_ref().is_some_and(|active| {
                    let statuses = active.handle.statuses();
                    !statuses.is_empty()
                        && statuses
                            .iter()
                            .all(|status| matches!(status.state, StreamState::Ended { .. }))
                });
                if all_ended {
                    let dto = state.status();
                    let terminal = (dto.state == "failed").then_some(dto);
                    teardown(&app, terminal);
                } else if app.emit("stream", &state.status()).is_err() {
                    return;
                }
            }
            std::thread::sleep(Duration::from_secs(1));
        })
        .expect("stream status thread spawns");
}
