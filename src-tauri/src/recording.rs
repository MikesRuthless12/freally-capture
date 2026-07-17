//! The app side of recording: session lifecycle (start / pause / resume /
//! stop), the studio-thread video feed, the audio-engine tap, and the
//! `recording` status event.
//!
//! Recording writes go **only** to the file the user configured — never a
//! socket that leaves the machine, never a cloud. Wire containers need the
//! clearly-labeled ffmpeg component; the owned `.frec` path needs nothing.
//! Recording settings are their own struct, independent of any future
//! stream settings — the local copy never rides a stream's knobs.

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, Runtime};

use fcap_audio::{IsoTap, RecordTap};
use fcap_encode::mux::{Container, FfmpegSink, FrecSink, WirePlan};
use fcap_encode::recorder::{RecordSink, RecordSpec, Recorder, RecorderHandle};
use fcap_encode::VideoCodec;
use fcap_scene::SourceId;

use crate::audio::AudioRuntime;
use crate::commands::recording::EncodeState;
use crate::settings::{RecordingSettings, SettingsStore};
use crate::studio::StudioState;

/// How often the status emitter pushes while a session runs.
const STATUS_TICK: Duration = Duration::from_millis(500);

/// The `recording` event + `recording_status` payload (mirrored in
/// `ui/src/api/types.ts`).
#[derive(Debug, Clone, Serialize)]
#[serde(
    rename_all = "camelCase",
    tag = "state",
    rename_all_fields = "camelCase"
)]
pub enum RecordingDto {
    Idle {
        /// The last finished recording's files (newest session).
        last_paths: Vec<String>,
        /// The last session's error, if it ended badly.
        error: Option<String>,
    },
    Recording {
        duration_sec: f64,
        path: String,
        container: Container,
        tracks: u32,
        frames_duplicated: u64,
        frames_behind: u64,
        audio_blocks_dropped: u64,
        /// Chapter markers dropped so far (TASK-610).
        markers: u32,
        /// CAP-N40: ISO lanes recording alongside the program (0 = none).
        iso_lanes: u32,
    },
    Paused {
        duration_sec: f64,
        path: String,
        container: Container,
        tracks: u32,
    },
    Finalizing {
        path: String,
    },
}

struct Active {
    recorder: Option<Recorder>,
    handle: RecorderHandle,
    /// The optional parallel vertical-canvas recorder (Phase 6): its own
    /// `… (vertical)` file, same settings, paused/stopped with the main one.
    vertical_recorder: Option<Recorder>,
    vertical_handle: Option<RecorderHandle>,
    /// CAP-N40: the ISO lanes — one independent recorder per selected
    /// source, paused/stopped in lockstep with the main recording. A lane
    /// failure never invalidates the finished program file.
    iso_lanes: Vec<IsoLane>,
    display_path: String,
    container: Container,
    tracks: u32,
    finalizing: bool,
}

/// One CAP-N40 ISO lane: a source recorded clean to its own file.
struct IsoLane {
    recorder: Option<Recorder>,
    handle: RecorderHandle,
    source: SourceId,
    /// The source's display name at start — for honest stop-time reporting.
    name: String,
}

/// Tauri-managed recording state.
pub struct RecordingState {
    inner: Mutex<Option<Active>>,
    /// Held across the whole of [`start`] (which does slow I/O) so two
    /// concurrent `recording_start` calls can't both pass the "already
    /// running" check and race onto the same file — the `inner` guard alone
    /// leaves a TOCTOU window while the sink/file is being created.
    starting: AtomicBool,
    /// The render thread's cheap gate + feed (uncontended lock per tick).
    active: AtomicBool,
    feed: Mutex<Option<RecorderHandle>>,
    /// The vertical canvas's gate + feed (set only when it records too).
    vertical_active: AtomicBool,
    vertical_feed: Mutex<Option<RecorderHandle>>,
    /// The ISO lanes' gate + feeds (CAP-N40): `(source, post_filter, handle)`
    /// per lane, set only while ISO lanes record.
    iso_active: AtomicBool,
    #[allow(clippy::type_complexity)]
    iso_feed: Mutex<Vec<(SourceId, bool, RecorderHandle)>>,
    /// CAP-M22 × CAP-N40: while the panic button holds the program on a
    /// slate, the ISO lanes hold too — they record sources CLEAN (pre-fader,
    /// pre-composite), so without this gate a panic would leak straight into
    /// every ISO file.
    iso_panic_hold: AtomicBool,
    /// CAP-N42: the main recording wants the transparent-clear alpha render
    /// instead of the shared opaque program readback (`.frec` + alpha on).
    alpha_active: AtomicBool,
    /// CAP-N43: the session's armed event-split triggers (settings are read
    /// once at start — a mid-session settings edit applies next session).
    split_on_scene: AtomicBool,
    split_on_marker: AtomicBool,
    split_on_rundown: AtomicBool,
    /// CAP-N44: auto-markers armed — studio events (scene switch, replay
    /// save, reconnect, dropped-frame burst, alarm, rule firing) drop typed
    /// chapter markers alongside the manual hotkey's.
    auto_markers: AtomicBool,
    /// The last finished session's result.
    last: Mutex<(Vec<String>, Option<String>)>,
    /// Chapter markers (TASK-610 + CAP-N44): `(position ms, label)` — the
    /// manual hotkey drops "Marker N", auto-markers carry their event label
    /// ("Scene: …", "Replay saved", …). Written as mkv chapters / a sidecar
    /// file at stop.
    markers: Mutex<Vec<(u64, String)>>,
    /// Encoder failover (CAP-M12): the session's ladder, consulted by the
    /// watchdog when the sink dies. `None` for .frec (no wire encoder).
    failover: Mutex<Option<fcap_encode::FailoverLadder>>,
    /// When the current session started — the CAP-M15 "time since recording"
    /// clock. Read gated on `active`, so a stale instant is harmless.
    since: Mutex<Option<Instant>>,
}

impl RecordingState {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(None),
            starting: AtomicBool::new(false),
            active: AtomicBool::new(false),
            feed: Mutex::new(None),
            vertical_active: AtomicBool::new(false),
            vertical_feed: Mutex::new(None),
            iso_active: AtomicBool::new(false),
            iso_feed: Mutex::new(Vec::new()),
            iso_panic_hold: AtomicBool::new(false),
            alpha_active: AtomicBool::new(false),
            split_on_scene: AtomicBool::new(false),
            split_on_marker: AtomicBool::new(false),
            split_on_rundown: AtomicBool::new(false),
            auto_markers: AtomicBool::new(false),
            last: Mutex::new((Vec::new(), None)),
            markers: Mutex::new(Vec::new()),
            failover: Mutex::new(None),
            since: Mutex::new(None),
        }
    }

    /// When the running session started; `None` while idle.
    pub fn recording_since(&self) -> Option<Instant> {
        if !self.active.load(Ordering::Relaxed) {
            return None;
        }
        *self
            .since
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn lock_inner(&self) -> std::sync::MutexGuard<'_, Option<Active>> {
        self.inner
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// The render thread asks this every tick — a relaxed atomic, no lock.
    pub fn wants_frames(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }

    /// Whether the vertical canvas records too (a relaxed atomic, no lock).
    pub fn wants_vertical_frames(&self) -> bool {
        self.vertical_active.load(Ordering::Relaxed)
    }

    /// Push the newest program frame (render thread; latest wins).
    pub fn push_video(&self, pixels: Arc<Vec<u8>>) {
        let feed = self
            .feed
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(handle) = feed.as_ref() {
            handle.push_frame(pixels);
        }
    }

    /// Push the newest vertical-canvas frame to its recorder.
    pub fn push_video_vertical(&self, pixels: Arc<Vec<u8>>) {
        let feed = self
            .vertical_feed
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(handle) = feed.as_ref() {
            handle.push_frame(pixels);
        }
    }

    /// Whether any ISO lane records (a relaxed atomic — the render thread's
    /// per-tick gate, like the two above).
    pub fn wants_iso_frames(&self) -> bool {
        self.iso_active.load(Ordering::Relaxed)
    }

    /// CAP-N42: whether the main recording wants the transparent alpha
    /// render instead of the shared opaque program readback.
    pub fn wants_alpha_frames(&self) -> bool {
        self.alpha_active.load(Ordering::Relaxed)
    }

    /// CAP-N43: whether this session splits on scene switches (the loop's
    /// per-tick gate — a relaxed atomic).
    pub fn splits_on_scene(&self) -> bool {
        self.split_on_scene.load(Ordering::Relaxed)
    }

    /// CAP-N43: whether this session splits on rundown steps.
    pub fn splits_on_rundown(&self) -> bool {
        self.split_on_rundown.load(Ordering::Relaxed)
    }

    /// CAP-N44: whether this session drops auto-markers on studio events.
    pub fn auto_markers(&self) -> bool {
        self.auto_markers.load(Ordering::Relaxed)
    }

    /// CAP-N43: cut every lane (main + vertical + ISO) to a new part at the
    /// next frame boundary. Wire sinks ignore it (they cut by time alone);
    /// frec lanes rotate together, so part boundaries stay aligned across
    /// the whole take.
    pub fn request_split_all(&self) {
        let inner = self.lock_inner();
        if let Some(active) = inner.as_ref() {
            if active.finalizing {
                return;
            }
            active.handle.request_split();
            if let Some(vertical) = &active.vertical_handle {
                vertical.request_split();
            }
            for lane in &active.iso_lanes {
                lane.handle.request_split();
            }
        }
    }

    /// The lanes the render thread must feed: `(source, post_filter)` pairs.
    pub fn iso_lanes_wanted(&self) -> Vec<(SourceId, bool)> {
        self.iso_feed
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .iter()
            .map(|(source, post_filter, _)| (*source, *post_filter))
            .collect()
    }

    /// Push the newest clean frame for one ISO lane (render thread).
    pub fn push_video_iso(&self, source: SourceId, pixels: Arc<Vec<u8>>) {
        let feed = self
            .iso_feed
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some((_, _, handle)) = feed.iter().find(|(lane, _, _)| *lane == source) {
            handle.push_frame(pixels);
        }
    }

    /// Hold (or release) every ISO lane for the panic slate (CAP-M22): lanes
    /// record sources clean, so a panic must pause them — video AND audio —
    /// or the cut content lands in the ISO files. Pause is the recorder's
    /// gapless gate, so release resumes one contiguous timeline. A
    /// user-initiated pause is respected on release.
    pub fn set_iso_panic_hold(&self, hold: bool) {
        self.iso_panic_hold.store(hold, Ordering::Relaxed);
        let inner = self.lock_inner();
        if let Some(active) = inner.as_ref() {
            let user_paused = active.handle.is_paused();
            for lane in &active.iso_lanes {
                lane.handle.set_paused(hold || user_paused);
            }
        }
    }

    /// Whether a session is up (running, paused or finalizing) — the quit
    /// guard's check (CAP-M23).
    pub fn is_active(&self) -> bool {
        self.lock_inner().is_some()
    }

    /// Markers dropped so far this session — the `{marker-count}` filename
    /// token (CAP-M25); 0 outside a session.
    pub fn marker_count(&self) -> u32 {
        self.markers
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .len() as u32
    }

    /// The current status snapshot.
    pub fn status(&self) -> RecordingDto {
        let inner = self.lock_inner();
        match inner.as_ref() {
            Some(active) if active.finalizing => RecordingDto::Finalizing {
                path: active.display_path.clone(),
            },
            Some(active) => {
                let stats = active.handle.stats();
                let duration_sec = active.handle.duration().as_secs_f64();
                if active.handle.is_paused() {
                    RecordingDto::Paused {
                        duration_sec,
                        path: active.display_path.clone(),
                        container: active.container,
                        tracks: active.tracks,
                    }
                } else {
                    RecordingDto::Recording {
                        duration_sec,
                        path: active.display_path.clone(),
                        container: active.container,
                        tracks: active.tracks,
                        frames_duplicated: stats.frames_duplicated,
                        frames_behind: stats.frames_behind,
                        audio_blocks_dropped: stats.audio_blocks_dropped,
                        markers: self
                            .markers
                            .lock()
                            .unwrap_or_else(std::sync::PoisonError::into_inner)
                            .len() as u32,
                        iso_lanes: active.iso_lanes.len() as u32,
                    }
                }
            }
            None => {
                let last = self
                    .last
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
                RecordingDto::Idle {
                    last_paths: last.0.clone(),
                    error: last.1.clone(),
                }
            }
        }
    }
}

impl Default for RecordingState {
    fn default() -> Self {
        Self::new()
    }
}

/// Emit the current status; `false` once the app is gone (shutdown signal
/// for the emitter thread, matching the other emitters).
fn emit_status<R: Runtime>(app: &AppHandle<R>) -> bool {
    let status = app.state::<RecordingState>().status();
    app.emit("recording", &status).is_ok()
}

/// Emit the `encoder-fallback` event (CAP-M12): the honest toast + stats
/// note, with encoder ids resolved to catalog labels when known. Shared by
/// the recording watchdog and the stream lanes.
pub(crate) fn announce_fallback<R: Runtime>(
    app: &AppHandle<R>,
    scope: &str,
    from: &str,
    to: &str,
    catalog: Option<&fcap_encode::Catalog>,
) {
    let label = |id: &str| {
        catalog
            .and_then(|catalog| catalog.get(id))
            .map(|desc| desc.label.clone())
            .unwrap_or_else(|| id.to_string())
    };
    println!("{scope}: encoder failover {from} → {to}");
    // CAP-N50: a failover is exactly the kind of moment a post-show
    // timeline must explain.
    crate::forensic::note(
        app,
        "fallback",
        &format!("{scope}: {} → {}", label(from), label(to)),
    );
    let _ = app.emit(
        "encoder-fallback",
        serde_json::json!({
            "scope": scope,
            "from": label(from),
            "to": label(to),
        }),
    );
}

/// The default recording folder: the OS Videos directory (falling back to
/// the home directory) — never a temp dir the OS may sweep.
fn default_folder() -> PathBuf {
    directories::UserDirs::new()
        .and_then(|dirs| dirs.video_dir().map(PathBuf::from))
        .or_else(|| directories::UserDirs::new().map(|dirs| dirs.home_dir().to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Where recordings land for the given settings — shared by the session
/// start and the recordings list (and its remux path check).
pub fn recordings_folder(settings: &RecordingSettings) -> PathBuf {
    if settings.folder.trim().is_empty() {
        default_folder()
    } else {
        PathBuf::from(settings.folder.trim())
    }
}

/// A per-output folder override (CAP-M25): blank → the recordings folder.
pub fn output_folder(settings: &RecordingSettings, override_folder: &str) -> PathBuf {
    let trimmed = override_folder.trim();
    if trimmed.is_empty() {
        recordings_folder(settings)
    } else {
        PathBuf::from(trimmed)
    }
}

/// The `{counter}` value for one naming event (CAP-M25): advance the
/// persisted counter only when the template actually uses it — no settings
/// write otherwise.
/// CAP-N38: resolve the CAP-M25 filename stem for an audio-only recording,
/// reusing the same template + counter machinery as a video recording (the
/// canvas tokens resolve to 0 — there is no canvas in podcast mode).
pub fn audio_only_stem<R: Runtime>(app: &AppHandle<R>, settings: &RecordingSettings) -> String {
    let counter = counter_for(app, &settings.template, settings.counter);
    let naming = naming_context(app, settings.filename_prefix.clone(), (0, 0), counter);
    crate::filename::resolve_template(&settings.template, &naming)
}

pub(crate) fn counter_for<R: Runtime>(app: &AppHandle<R>, template: &str, current: u32) -> u32 {
    if template.contains("{counter}") {
        app.state::<SettingsStore>().bump_recording_counter()
    } else {
        current
    }
}

/// Gather the filename-token context for one naming event (CAP-M25): the
/// moment's local date/time, the active scene + profile, the recorded
/// geometry and the live marker count.
pub(crate) fn naming_context<R: Runtime>(
    app: &AppHandle<R>,
    prefix: String,
    canvas: (u32, u32),
    counter: u32,
) -> crate::filename::TokenContext {
    let now = chrono::Local::now();
    crate::filename::TokenContext {
        prefix,
        date: now.format("%Y-%m-%d").to_string(),
        time: now.format("%H-%M-%S").to_string(),
        scene: app
            .state::<StudioState>()
            .with_collection(|collection| collection.active_scene().name.clone()),
        profile: app
            .state::<crate::profiles::WorkspaceState>()
            .profile_name(),
        canvas,
        marker_count: app.state::<RecordingState>().marker_count(),
        counter,
    }
}

/// A non-colliding output path: `{stem}.{ext}`, appending ` (2)`, ` (3)`…
/// if that base (or, when splitting, its first `part…` segment) already
/// exists — so two naming events resolving to the same stem (same
/// local-time second, or a static template) never overwrite each other.
pub(crate) fn unique_recording_path(
    folder: &std::path::Path,
    stem: &str,
    ext: &str,
    split: bool,
) -> PathBuf {
    let taken = |base: &std::path::Path| -> bool {
        if base.exists() {
            return true;
        }
        // Splitting never writes `base` itself — the segments are its
        // `{stem} part000/001.{ext}` siblings; check those instead.
        if split {
            if let Some(stem) = base.file_stem().and_then(|s| s.to_str()) {
                return ["part000", "part001"].iter().any(|suffix| {
                    base.with_file_name(format!("{stem} {suffix}.{ext}"))
                        .exists()
                });
            }
        }
        false
    };
    for n in 0..10_000u32 {
        let name = if n == 0 {
            format!("{stem}.{ext}")
        } else {
            format!("{stem} ({}).{ext}", n + 1)
        };
        let base = folder.join(name);
        if !taken(&base) {
            return base;
        }
    }
    folder.join(format!("{stem}.{ext}"))
}

/// Resets an [`AtomicBool`] on scope exit — arms the `starting` guard so it
/// clears however `start` returns (success or any `?` error).
struct ResetOnDrop<'a>(&'a AtomicBool);
impl Drop for ResetOnDrop<'_> {
    fn drop(&mut self) {
        self.0.store(false, Ordering::SeqCst);
    }
}

/// Build one recording sink — the owned `.frec` writer or the labeled ffmpeg
/// muxer — for a lane's `(container, encoder_id, scale, alpha)`, with the
/// rate-control knobs shared from `settings`. The one seam for the program,
/// vertical, and ISO lanes, so a new option (alpha, event splits) is threaded
/// in exactly one place. `encoder_id` is ignored for `.frec` (pass `""`).
#[allow(clippy::too_many_arguments)]
fn make_sink<R: Runtime>(
    app: &AppHandle<R>,
    settings: &RecordingSettings,
    spec: &RecordSpec,
    path: &std::path::Path,
    container: Container,
    encoder_id: &str,
    scale: Option<(u32, u32)>,
    split: Option<u32>,
    alpha: bool,
    event_splits: bool,
) -> Result<Box<dyn RecordSink>, String> {
    match container {
        Container::Frec => Ok(Box::new(FrecSink::create_with_options(
            spec.clone(),
            path.to_path_buf(),
            split,
            alpha,
            event_splits,
        )?)),
        wire => {
            let ready = app.state::<EncodeState>().ready_ffmpeg().ok_or_else(|| {
                "recording to this container needs the ffmpeg component — install it from \
                 Components (the owned lossless .frec format needs nothing)"
                    .to_string()
            })?;
            let plan = WirePlan {
                container: wire,
                encoder_id: encoder_id.to_owned(),
                rate_control: settings.rate_control,
                preset: settings.preset,
                keyframe_sec: settings.keyframe_sec,
                audio_bitrate_kbps: settings.audio_bitrate_kbps,
                split_minutes: split,
                scale,
                path: path.to_path_buf(),
            };
            Ok(Box::new(FfmpegSink::spawn(&ready, spec, &plan)?))
        }
    }
}

/// Start a recording session from the persisted settings.
pub fn start<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    start_with(app, None)
}

/// Start with an optional encoder override — the failover restart path
/// (CAP-M12). `None` (every user-initiated start) resolves the settings
/// encoder and arms a fresh ladder; `Some(id)` keeps the advanced ladder.
pub(crate) fn start_with<R: Runtime>(
    app: &AppHandle<R>,
    encoder_override: Option<String>,
) -> Result<(), String> {
    let state = app.state::<RecordingState>();
    // Serialize the whole start (it does slow file/child I/O before the
    // session is registered): a second concurrent start bails here instead of
    // racing onto the same output file.
    if state.starting.swap(true, Ordering::SeqCst) {
        return Err("a recording is already starting".to_string());
    }
    let _reset = ResetOnDrop(&state.starting);
    if state.lock_inner().is_some() {
        return Err("a recording is already running".to_string());
    }
    // The record tap is a single sink — an audio-only recording owns it if one
    // runs (mirrors the reverse guard in `audiorec::start`).
    if app.state::<crate::audiorec::AudioRecState>().is_active() {
        return Err("stop the audio-only recording before starting a video one".to_owned());
    }

    let settings = app.state::<SettingsStore>().get().recording;
    settings.validate()?;

    // Canvas geometry comes from the live model; the recording locks it in.
    let snapshot = app.state::<StudioState>().snapshot();
    let (width, height) = (
        snapshot.collection.canvas_width,
        snapshot.collection.canvas_height,
    );
    let tracks: Vec<usize> = (0..fcap_scene::TRACK_COUNT)
        .filter(|index| settings.tracks_mask & (1 << index) != 0)
        .collect();
    let track_count = tracks.len() as u32;
    let spec = RecordSpec {
        width,
        height,
        fps: settings.fps,
        tracks,
    };

    let folder = recordings_folder(&settings);
    std::fs::create_dir_all(&folder)
        .map_err(|err| format!("could not create {}: {err}", folder.display()))?;
    let split = (settings.split_minutes > 0).then_some(settings.split_minutes);
    // CAP-M25: resolve the token template once per session (main + vertical
    // share the moment and the counter). Resolved names still collide (a
    // fast restart, a DST fall-back hour, a static template), and both sink
    // paths truncate — so guard uniqueness rather than silently overwrite.
    let counter = counter_for(app, &settings.template, settings.counter);
    let naming = naming_context(
        app,
        settings.filename_prefix.clone(),
        (width, height),
        counter,
    );
    let stem = crate::filename::resolve_template(&settings.template, &naming);
    let path = unique_recording_path(
        &folder,
        &stem,
        settings.container.extension(),
        split.is_some(),
    );

    // CAP-N42: the main lane records real transparency when asked — `.frec`
    // only (wire codecs flatten; validate/UI say so). The vertical + ISO
    // lanes stay opaque: their renders come from opaque-clear passes, and a
    // lying alpha flag would be worse than no flag.
    let alpha = settings.alpha_frec && settings.container == Container::Frec;
    // CAP-N43: whether any event-split trigger is armed this session — frec
    // lanes name their files in part mode from the first one when so.
    let event_splits =
        settings.split_on_scene || settings.split_on_marker || settings.split_on_rundown;

    // Resolve the program lane's wire encoder once (empty for `.frec`); the
    // failover restart keeps its advanced ladder via `encoder_override`.
    let program_encoder = if settings.container == Container::Frec {
        String::new()
    } else {
        match &encoder_override {
            Some(id) => id.clone(),
            None => resolve_encoder(app, &settings, settings.container)?,
        }
    };
    // TASK-609: optional output downscale (wire only — `.frec` always records
    // the canvas). One sink builder serves both canvases — the vertical file
    // is the same configuration at its own geometry and `… (vertical)` path.
    let program_scale = (settings.output_width > 0 && settings.output_height > 0)
        .then_some((settings.output_width, settings.output_height));
    let build_sink = |spec: &RecordSpec, path: &std::path::Path, with_alpha: bool| {
        make_sink(
            app,
            &settings,
            spec,
            path,
            settings.container,
            &program_encoder,
            program_scale,
            split,
            with_alpha,
            event_splits,
        )
    };

    let sink = build_sink(&spec, &path, alpha)?;

    // The optional parallel vertical-canvas recording (Phase 6, TASK-604).
    let vertical = if settings.record_vertical {
        match snapshot.collection.vertical {
            Some(config) => {
                let vertical_spec = RecordSpec {
                    width: config.width,
                    height: config.height,
                    fps: settings.fps,
                    tracks: spec.tracks.clone(),
                };
                let vertical_naming = crate::filename::TokenContext {
                    prefix: format!("{} (vertical)", settings.filename_prefix),
                    canvas: (config.width, config.height),
                    ..naming.clone()
                };
                let mut vertical_stem =
                    crate::filename::resolve_template(&settings.template, &vertical_naming);
                if vertical_stem == stem {
                    // A template using neither {prefix} nor {canvas} resolves
                    // both files identically — label the vertical file rather
                    // than let it pass as a collision suffix.
                    vertical_stem.push_str(" (vertical)");
                }
                let vertical_path = unique_recording_path(
                    &folder,
                    &vertical_stem,
                    settings.container.extension(),
                    split.is_some(),
                );
                // The vertical render is an opaque-clear pass — never alpha.
                let vertical_sink = build_sink(&vertical_spec, &vertical_path, false)?;
                Some((vertical_spec, vertical_sink, vertical_path))
            }
            None => None, // no vertical canvas configured — record main only
        }
    } else {
        None
    };

    // CAP-N40: the ISO lanes — the selected sources recorded clean, each to
    // its own file with the ISO container/encoder (the program's rate-control
    // knobs). Selection resolves against the live collection; a selected
    // source that left the collection is skipped with a note, never a failed
    // start. Lanes record at canvas geometry (the source fit inside), so
    // every file drops onto an NLE timeline frame-aligned with the program.
    let iso_selected: Vec<(SourceId, String, bool)> = settings
        .iso_sources
        .iter()
        .filter_map(|wanted| {
            let wanted = wanted.to_lowercase();
            let found = snapshot
                .collection
                .sources
                .iter()
                .find(|source| source.id.0.to_string() == wanted);
            if found.is_none() {
                println!("recording: ISO source {wanted} is not in the collection — skipped");
            }
            found.map(|source| (source.id, source.name.clone(), source.audio.is_some()))
        })
        .collect();
    let iso_encoder = if iso_selected.is_empty() || settings.iso_container == Container::Frec {
        None
    } else {
        let iso_settings = RecordingSettings {
            encoder_id: settings.iso_encoder_id.clone(),
            ..settings.clone()
        };
        Some(resolve_encoder(app, &iso_settings, settings.iso_container)?)
    };
    type BuiltLane = (
        RecordSpec,
        Box<dyn RecordSink>,
        PathBuf,
        SourceId,
        String,
        bool,
    );
    let mut iso_built: Vec<BuiltLane> = Vec::new();
    for (source, name, has_audio) in &iso_selected {
        let iso_spec = RecordSpec {
            width,
            height,
            fps: settings.fps,
            // Slot 0 carries the source's own clean audio; video-only
            // sources record a video-only file.
            tracks: if *has_audio { vec![0] } else { Vec::new() },
        };
        let iso_stem = format!("{stem} ISO {}", iso_safe_name(name));
        let iso_path = unique_recording_path(
            &folder,
            &iso_stem,
            settings.iso_container.extension(),
            split.is_some(),
        );
        // ISO lanes record the canvas clean (no downscale) and are never
        // alpha; frec lanes honor the same event splits so part boundaries
        // stay aligned with the program's across the whole take.
        let sink = make_sink(
            app,
            &settings,
            &iso_spec,
            &iso_path,
            settings.iso_container,
            iso_encoder.as_deref().unwrap_or_default(),
            None,
            split,
            false,
            event_splits,
        )?;
        iso_built.push((iso_spec, sink, iso_path, *source, name.clone(), *has_audio));
    }

    // CAP-M11: track the in-flight outputs so an unclean exit can offer to
    // repair them next launch. The owned .frec is crash-safe by design and
    // is never listed.
    let mut in_progress = Vec::new();
    if settings.container != Container::Frec {
        in_progress.push(crate::salvage::InProgressFile {
            path: path.clone(),
            split: split.is_some(),
        });
        if let Some((_, _, vertical_path)) = &vertical {
            in_progress.push(crate::salvage::InProgressFile {
                path: vertical_path.clone(),
                split: split.is_some(),
            });
        }
    }
    if settings.iso_container != Container::Frec {
        for (_, _, iso_path, _, _, _) in &iso_built {
            in_progress.push(crate::salvage::InProgressFile {
                path: iso_path.clone(),
                split: split.is_some(),
            });
        }
    }
    if !in_progress.is_empty() {
        crate::salvage::write_in_progress(&in_progress);
    }

    let recorder = Recorder::start(spec, sink);
    let handle = recorder.handle();
    let (vertical_recorder, vertical_handle) = match vertical {
        Some((vertical_spec, vertical_sink, vertical_path)) => {
            let recorder = Recorder::start(vertical_spec, vertical_sink);
            let handle = recorder.handle();
            println!("recording: vertical → {}", vertical_path.display());
            (Some(recorder), Some(handle))
        }
        None => (None, None),
    };
    // CAP-N40: start every ISO lane; remember which carry audio for the tap.
    let mut iso_lanes: Vec<IsoLane> = Vec::with_capacity(iso_built.len());
    let mut iso_audio: Vec<(SourceId, RecorderHandle)> = Vec::new();
    for (iso_spec, iso_sink, iso_path, source, name, has_audio) in iso_built {
        let recorder = Recorder::start(iso_spec, iso_sink);
        let lane_handle = recorder.handle();
        if has_audio {
            iso_audio.push((source, lane_handle.clone()));
        }
        println!("recording: ISO {name} → {}", iso_path.display());
        iso_lanes.push(IsoLane {
            recorder: Some(recorder),
            handle: lane_handle,
            source,
            name,
        });
    }

    // Tap the mixer: every 10 ms block of the enabled tracks flows to the
    // recorder(s) from this moment (pause gating happens in the handles).
    let tap_handle = handle.clone();
    let tap_vertical = vertical_handle.clone();
    app.state::<AudioRuntime>()
        .engine
        .set_record_tap(Some(RecordTap {
            tracks: settings.tracks_mask,
            sink: Box::new(move |blocks| {
                tap_handle.push_audio_blocks(blocks);
                if let Some(vertical) = &tap_vertical {
                    vertical.push_audio_blocks(blocks);
                }
            }),
        }));
    // CAP-N40: the independent per-source clean tap feeds each audio lane.
    if iso_audio.is_empty() {
        app.state::<AudioRuntime>().engine.set_iso_tap(None);
    } else {
        let taps = iso_audio;
        app.state::<AudioRuntime>().engine.set_iso_tap(Some(IsoTap {
            sources: taps.iter().map(|(id, _)| *id).collect(),
            sink: Box::new(move |blocks| {
                for (id, block) in blocks {
                    if let Some((_, lane)) = taps.iter().find(|(source, _)| source == id) {
                        lane.push_audio_blocks(&[(0, *block)]);
                    }
                }
            }),
        }));
    }

    *state
        .feed
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(handle.clone());
    *state
        .vertical_feed
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = vertical_handle.clone();
    state
        .vertical_active
        .store(vertical_handle.is_some(), Ordering::Relaxed);
    *state
        .iso_feed
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = iso_lanes
        .iter()
        .map(|lane| (lane.source, settings.iso_post_filter, lane.handle.clone()))
        .collect();
    state
        .iso_active
        .store(!iso_lanes.is_empty(), Ordering::Relaxed);
    state.alpha_active.store(alpha, Ordering::Relaxed);
    // CAP-N43: arm the session's event-split triggers (frec lanes only —
    // wire sinks ignore split requests by design).
    state
        .split_on_scene
        .store(settings.split_on_scene, Ordering::Relaxed);
    state
        .split_on_marker
        .store(settings.split_on_marker, Ordering::Relaxed);
    state
        .split_on_rundown
        .store(settings.split_on_rundown, Ordering::Relaxed);
    // CAP-N44: arm auto-markers for the session.
    state
        .auto_markers
        .store(settings.auto_markers, Ordering::Relaxed);
    *state.lock_inner() = Some(Active {
        recorder: Some(recorder),
        handle,
        vertical_recorder,
        vertical_handle,
        iso_lanes,
        display_path: path.display().to_string(),
        container: settings.container,
        tracks: track_count,
        finalizing: false,
    });
    // CAP-M22 × CAP-N40: a session started while the panic slate is up
    // begins with its ISO lanes held (the render loop keeps this in sync).
    state.set_iso_panic_hold(app.state::<StudioState>().panic_active());
    *state
        .since
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Instant::now());
    state.active.store(true, Ordering::Relaxed);
    state
        .markers
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .clear();
    // CAP-M12: arm the failover ladder for wire sessions. A user start
    // builds a fresh one; a failover restart keeps its advanced ladder.
    if encoder_override.is_none() {
        let ladder = if settings.container == Container::Frec {
            None // the owned lossless writer has no wire encoder to swap
        } else {
            crate::commands::recording::ensure_catalog(app)
                .ok()
                .and_then(|catalog| {
                    resolve_encoder(app, &settings, settings.container)
                        .ok()
                        .map(|id| fcap_encode::FailoverLadder::new(&id, &catalog))
                })
        };
        *state
            .failover
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = ladder;
    }
    emit_status(app);
    println!("recording: started → {}", path.display());
    Ok(())
}

/// Drop a chapter marker at the current recording position (TASK-610) —
/// the marker hotkey's action. Platform-side stream markers need account
/// APIs, which the charter excludes; these land in the RECORDING (mkv
/// chapters, or a sidecar for other containers).
pub fn add_marker<R: Runtime>(app: &AppHandle<R>) -> Result<u32, String> {
    let state = app.state::<RecordingState>();
    let position_ms = {
        let inner = state.lock_inner();
        let active = inner.as_ref().ok_or("no recording is running")?;
        if active.finalizing {
            return Err("the recording is finalizing".to_string());
        }
        active.handle.duration().as_millis() as u64
    };
    let (count, label) = {
        let mut markers = state
            .markers
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        // Number by MANUAL markers only — auto-markers (CAP-N44) share the vec
        // but carry their own event labels, so "Marker N" must not count them
        // (or the operator's first manual marker reads "Marker 3" after two
        // auto-markers fired).
        let manual = markers
            .iter()
            .filter(|(_, label)| label.starts_with("Marker "))
            .count();
        let label = format!("Marker {}{}", manual + 1, ltc_suffix(app));
        markers.push((position_ms, label.clone()));
        (markers.len() as u32, label)
    };
    // CAP-N50/N51: markers land on the forensic timeline + session report.
    crate::forensic::note(app, "marker", &label);
    // CAP-N43: a marker can also cut a new part (frec lanes only).
    if state.split_on_marker.load(Ordering::Relaxed) {
        state.request_split_all();
    }
    emit_status(app);
    println!("recording: marker {count} at {position_ms} ms");
    Ok(count)
}

/// CAP-N44: drop a typed auto-marker at the current recording position — the
/// studio-event path (scene switch, replay save, reconnect, dropped-frame
/// burst, alarm, rule firing). A silent no-op unless a session is running
/// with auto-markers armed, so callers can fire unconditionally. Labels are
/// bounded and flattened to one line (scene/rule names are user text).
pub fn add_auto_marker<R: Runtime>(app: &AppHandle<R>, label: &str) {
    let state = app.state::<RecordingState>();
    if !state.auto_markers.load(Ordering::Relaxed) {
        return;
    }
    let position_ms = {
        let inner = state.lock_inner();
        let Some(active) = inner.as_ref() else {
            return;
        };
        if active.finalizing {
            return;
        }
        active.handle.duration().as_millis() as u64
    };
    let clean: String = label
        .chars()
        .take(80)
        .map(|c| if c.is_control() { ' ' } else { c })
        .collect();
    let clean = format!("{clean}{}", ltc_suffix(app));
    let count = {
        let mut markers = state
            .markers
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        markers.push((position_ms, clean.clone()));
        markers.len() as u32
    };
    // CAP-N50/N51: auto-markers land on the forensic timeline + report too.
    crate::forensic::note(app, "marker", &clean);
    emit_status(app);
    println!("recording: auto-marker {count} \u{201c}{clean}\u{201d} at {position_ms} ms");
}

/// CAP-N47: when the LTC reader is armed and locked, markers carry the
/// incoming timecode — a bridge from this recording to the external
/// recorders synced to the same LTC.
fn ltc_suffix<R: Runtime>(app: &AppHandle<R>) -> String {
    app.state::<AudioRuntime>()
        .engine
        .ltc()
        .map(|tc| format!(" @ LTC {tc}"))
        .unwrap_or_default()
}

/// A source name reduced to filename-safe characters for an ISO file stem
/// (CAP-N40). Source names are user text — separators and reserved
/// characters become `-`, and an empty result falls back honestly.
fn iso_safe_name(name: &str) -> String {
    let safe: String = name
        .chars()
        .take(48)
        .map(|c| {
            if crate::filename::is_reserved(c) {
                '-'
            } else {
                c
            }
        })
        .collect();
    let trimmed = safe.trim().trim_end_matches('.').trim();
    if trimmed.is_empty() {
        "source".to_owned()
    } else {
        trimmed.to_owned()
    }
}

/// `HH:MM:SS {label}` lines — the sidecar shape (YouTube-chapter-like);
/// CAP-N44 labels carry the event through ("Scene: …", "Replay saved", …).
fn markers_sidecar_text(markers: &[(u64, String)]) -> String {
    let mut text = String::new();
    for (ms, label) in markers {
        let secs = ms / 1000;
        text.push_str(&format!(
            "{:02}:{:02}:{:02} {label}\n",
            secs / 3600,
            (secs % 3600) / 60,
            secs % 60,
        ));
    }
    text
}

/// Resolve "auto" (or validate an explicit encoder) against the verified
/// catalog, honestly: an encoder this machine refused is an error, not a
/// silent switch.
pub(crate) fn resolve_encoder<R: Runtime>(
    app: &AppHandle<R>,
    settings: &RecordingSettings,
    container: Container,
) -> Result<String, String> {
    let catalog = crate::commands::recording::ensure_catalog(app)?;
    let wanted_codec = |id: &str| catalog.get(id).map(|desc| desc.codec);

    let encoder_id = if settings.encoder_id == "auto" {
        // Auto picks the best encoder for a codec the *container* accepts —
        // WebM only holds AV1, so H.264 (the default elsewhere) would always
        // be rejected below and make WebM impossible without a manual pick.
        let codec = match container {
            Container::Webm => VideoCodec::Av1,
            _ => VideoCodec::H264,
        };
        catalog
            .best(codec)
            .map(|desc| desc.id.clone())
            .ok_or_else(|| format!("no usable {} encoder was detected", codec.label()))?
    } else {
        let desc = catalog.get(&settings.encoder_id).ok_or_else(|| {
            format!(
                "encoder {} is not offered on this machine — pick another in Settings → Output",
                settings.encoder_id
            )
        })?;
        if desc.verified == Some(false) {
            return Err(format!(
                "encoder {} is unavailable here: {} — pick another in Settings → Output",
                desc.label, desc.note
            ));
        }
        desc.id.clone()
    };

    // Container/codec compatibility, said before ffmpeg fails cryptically.
    if let Some(codec) = wanted_codec(&encoder_id) {
        let legal = match container {
            Container::Webm => codec == VideoCodec::Av1,
            Container::Mov => matches!(codec, VideoCodec::H264 | VideoCodec::Hevc),
            _ => true,
        };
        if !legal {
            return Err(format!(
                "{} cannot hold {} — pick a matching encoder or container",
                container.extension(),
                codec.label()
            ));
        }
    }
    Ok(encoder_id)
}

/// Pause / resume the running session.
pub fn set_paused<R: Runtime>(app: &AppHandle<R>, paused: bool) -> Result<(), String> {
    let state = app.state::<RecordingState>();
    let inner = state.lock_inner();
    let active = inner.as_ref().ok_or("no recording is running")?;
    if active.finalizing {
        return Err("the recording is finalizing".to_string());
    }
    active.handle.set_paused(paused);
    if let Some(vertical) = &active.vertical_handle {
        vertical.set_paused(paused);
    }
    // ISO lanes also stay held while the panic slate is up (CAP-M22).
    let panic_hold = state.iso_panic_hold.load(Ordering::Relaxed);
    for lane in &active.iso_lanes {
        lane.handle.set_paused(paused || panic_hold);
    }
    drop(inner);
    emit_status(app);
    Ok(())
}

/// Stop and finalize. Blocking (trailers, faststart) — call off the UI
/// thread. Returns the finished file paths.
pub fn stop<R: Runtime>(app: &AppHandle<R>) -> Result<Vec<String>, String> {
    let state = app.state::<RecordingState>();
    let (recorder, vertical_recorder, iso_lanes, total_ms) = {
        let mut inner = state.lock_inner();
        let active = inner.as_mut().ok_or("no recording is running")?;
        if active.finalizing {
            return Err("the recording is already finalizing".to_string());
        }
        active.finalizing = true;
        let recorder = active
            .recorder
            .take()
            .ok_or("the recorder is already stopping")?;
        let total_ms = active.handle.duration().as_millis() as u64;
        (
            recorder,
            active.vertical_recorder.take(),
            std::mem::take(&mut active.iso_lanes),
            total_ms,
        )
    };
    // Stop the feeds first: no frame lands after the user pressed Stop.
    state.active.store(false, Ordering::Relaxed);
    state.vertical_active.store(false, Ordering::Relaxed);
    state.iso_active.store(false, Ordering::Relaxed);
    state.alpha_active.store(false, Ordering::Relaxed);
    *state
        .feed
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = None;
    *state
        .vertical_feed
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = None;
    state
        .iso_feed
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .clear();
    app.state::<AudioRuntime>().engine.set_record_tap(None);
    app.state::<AudioRuntime>().engine.set_iso_tap(None);
    emit_status(app);

    let result = recorder.stop();
    // The vertical file finalizes too — its failure is reported but never
    // discards the finished main recording.
    let vertical_result = vertical_recorder.map(|recorder| recorder.stop());
    let (mut paths, mut error) = match &result {
        Ok(paths) => (
            paths
                .iter()
                .map(|p| p.display().to_string())
                .collect::<Vec<_>>(),
            None,
        ),
        Err(err) => (Vec::new(), Some(err.clone())),
    };
    // How many files the MAIN recording produced — the chapter-embed check
    // below must see the main file alone, not the vertical/ISO additions.
    let main_paths = paths.len();
    match &vertical_result {
        Some(Ok(vertical_paths)) => {
            paths.extend(vertical_paths.iter().map(|p| p.display().to_string()));
        }
        Some(Err(err)) => {
            let note = format!("the vertical recording failed: {err}");
            error = Some(match error {
                Some(main) => format!("{main}; {note}"),
                None => note,
            });
        }
        None => {}
    }
    // CAP-N40: finalize every ISO lane. Lane failures are reported in the
    // sticky idle error, never as a stop failure that hides the program file.
    for lane in iso_lanes {
        let IsoLane { recorder, name, .. } = lane;
        let Some(recorder) = recorder else { continue };
        match recorder.stop() {
            Ok(iso_paths) => {
                paths.extend(iso_paths.iter().map(|p| p.display().to_string()));
            }
            Err(err) => {
                let note = format!("the ISO recording for \u{201c}{name}\u{201d} failed: {err}");
                error = Some(match error {
                    Some(main) => format!("{main}; {note}"),
                    None => note,
                });
            }
        }
    }
    // CAP-M11: every output finalized cleanly — nothing to salvage next
    // launch. Any finalize failure keeps the sidecar, so a later crash still
    // surfaces the repair offer for the damaged file.
    if result.is_ok() && error.is_none() {
        crate::salvage::clear_in_progress();
    }
    // TASK-610: attach the dropped markers — embedded chapters for a single
    // mkv, a readable sidecar otherwise. Best-effort: a chapter failure
    // never invalidates the finished recording. This MUST run before the
    // post-record pipeline is enqueued: the pipeline opens/remuxes/moves the
    // same main file on a worker thread, so chapters have to be on the file
    // first (a Move step would otherwise relocate it out from under the
    // chapter-embed rewrite, silently losing the operator's chapters).
    let markers: Vec<(u64, String)> = std::mem::take(
        &mut *state
            .markers
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner),
    );
    if !markers.is_empty() && !paths.is_empty() {
        let first = std::path::PathBuf::from(&paths[0]);
        let embedded = main_paths == 1
            && first.extension().and_then(|ext| ext.to_str()) == Some("mkv")
            && match app.state::<EncodeState>().ready_ffmpeg() {
                Some(ready) => {
                    match fcap_encode::write_mkv_chapters(&ready, &first, &markers, Some(total_ms))
                    {
                        Ok(()) => true,
                        Err(err) => {
                            eprintln!("recording: chapter embed failed ({err}) — sidecar instead");
                            false
                        }
                    }
                }
                None => false,
            };
        if !embedded {
            let sidecar = first.with_extension("chapters.txt");
            if let Err(err) = std::fs::write(&sidecar, markers_sidecar_text(&markers)) {
                eprintln!("recording: could not write the chapter sidecar: {err}");
            } else {
                println!("recording: chapters → {}", sidecar.display());
            }
        }
    }
    // CAP-N45: hand the finished MAIN file(s) — now carrying their embedded
    // chapters — to the post-record pipeline (vertical/ISO lanes are
    // companions, not the show master; a lane failure never blocks the
    // master's chain). No-op unless enabled.
    if result.is_ok() {
        let main_files: Vec<std::path::PathBuf> = paths[..main_paths.min(paths.len())]
            .iter()
            .map(std::path::PathBuf::from)
            .collect();
        crate::pipeline::enqueue(app, main_files, Some(total_ms as f64 / 1000.0));
    }
    {
        let mut last = state
            .last
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        *last = (paths.clone(), error.clone());
    }
    *state.lock_inner() = None;
    emit_status(app);
    match result {
        // The MAIN recording finalized — always hand its paths back, even if
        // the optional vertical file failed. A vertical-only failure is kept
        // in the sticky idle DTO's `error` (a visible note) instead of being
        // reported as a stop failure that hides the good main recording.
        Ok(_) => {
            println!("recording: finished → {}", paths.join(", "));
            if let Some(err) = &error {
                eprintln!("recording: {err} (the main recording finished)");
            }
            Ok(paths)
        }
        Err(err) => Err(err),
    }
}

/// CAP-M12: after a dead sink was stopped, consult the session's ladder and
/// — when the encoder is to blame — restart the recording on the next rung.
/// The muxer lives inside the encoder child, so the recording necessarily
/// continues as a NEW file; the toast says so honestly.
fn failover_restart<R: Runtime>(app: &AppHandle<R>, why: &str, lived: Duration) {
    // A sink death can race a confirmed quit: the ordered shutdown already
    // finalized recordings — starting a fresh session now would leave a
    // truncated stray file when the process exits moments later.
    if app.state::<crate::shutdown::QuitState>().is_quitting() {
        return;
    }
    let decision = {
        let state = app.state::<RecordingState>();
        let mut guard = state
            .failover
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        match guard.as_mut() {
            Some(ladder) => ladder.on_fault(fcap_encode::classify_fault(why), lived),
            None => return,
        }
    };
    if let fcap_encode::FailoverDecision::Switch { from, to } = decision {
        match start_with(app, Some(to.clone())) {
            Ok(()) => {
                let catalog = crate::commands::recording::ensure_catalog(app).ok();
                announce_fallback(app, "recording", &from, &to, catalog.as_ref());
            }
            Err(err) => eprintln!("recording: failover restart failed: {err}"),
        }
    }
}

/// The status emitter: ~2 Hz while a session runs, the watchdog that stops
/// a session whose sink died (ffmpeg crash, disk full) so the failure is
/// surfaced instead of silently eating frames forever, and the low-disk
/// forecast (CAP-M10) — a filesystem stat every ~5 s, never at 2 Hz.
pub fn spawn_status_thread<R: Runtime>(app: AppHandle<R>) {
    std::thread::Builder::new()
        .name("fcap-rec-status".into())
        .spawn(move || {
            let mut disk_watch = crate::alarms::DiskWatch::default();
            let mut disk_ticks = 0u32;
            loop {
                let state = app.state::<RecordingState>();
                let (running, broken) = {
                    let inner = state.lock_inner();
                    match inner.as_ref() {
                        Some(active) if !active.finalizing => (
                            true,
                            active
                                .handle
                                .error()
                                .map(|why| (why, active.handle.duration())),
                        ),
                        Some(_) => (true, None),
                        None => (false, None),
                    }
                };
                if let Some((why, lived)) = broken {
                    // Finalize whatever the dead sink managed to write.
                    let err = stop(&app).err().unwrap_or_else(|| why.clone());
                    eprintln!("recording: session died: {err}");
                    // CAP-M12: if the ladder blames the encoder, keep the
                    // show — restart on the next rung (necessarily a new
                    // file).
                    failover_restart(&app, &why, lived);
                } else if running && !emit_status(&app) {
                    return; // the app is gone — wind down
                }
                // CAP-M10: the low-disk forecast, every 10th tick (~5 s).
                disk_ticks += 1;
                if disk_ticks >= 10 {
                    disk_ticks = 0;
                    let forecast = if running {
                        let settings = app.state::<SettingsStore>().get().recording;
                        let canvas = app.state::<StudioState>().with_collection(|collection| {
                            (collection.canvas_width, collection.canvas_height)
                        });
                        let rate = crate::alarms::recording_write_rate(
                            settings.container,
                            canvas,
                            settings.fps,
                            settings.rate_control.bitrate_kbps,
                            settings.audio_bitrate_kbps,
                            settings.tracks_mask.count_ones(),
                        );
                        crate::alarms::free_space_for(&recordings_folder(&settings))
                            .and_then(|free| crate::alarms::forecast_secs(free, rate))
                    } else {
                        None // not recording — the alarm clears
                    };
                    if let Some(dto) = disk_watch.evaluate(forecast) {
                        crate::alarms::emit_alarm(&app, dto.kind, dto.active, dto.minutes_left);
                    }
                }
                std::thread::sleep(STATUS_TICK);
            }
        })
        .expect("recording status thread spawns");
}

#[cfg(test)]
mod tests {
    use super::{iso_safe_name, unique_recording_path};

    fn temp_dir(tag: &str) -> std::path::PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let dir =
            std::env::temp_dir().join(format!("fcap-recname-{}-{nanos}-{tag}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("mkdir");
        dir
    }

    /// CAP-N40: source names are user text — the ISO stem must be
    /// filename-safe on every OS, bounded, and never empty.
    #[test]
    fn iso_names_are_reduced_to_filename_safe_stems() {
        assert_eq!(iso_safe_name("Webcam"), "Webcam");
        assert_eq!(iso_safe_name("Guest: Ana / Berlin"), "Guest- Ana - Berlin");
        assert_eq!(iso_safe_name("a\\b*c?d\"e<f>g|h"), "a-b-c-d-e-f-g-h");
        // Trailing dots break Windows filenames; whitespace-only falls back.
        assert_eq!(iso_safe_name("cam..."), "cam");
        assert_eq!(iso_safe_name("   "), "source");
        assert_eq!(iso_safe_name(""), "source");
        // Bounded to 48 chars — measured in characters, never mid-UTF-8.
        let long = "é".repeat(120);
        assert_eq!(iso_safe_name(&long).chars().count(), 48);
    }

    #[test]
    fn same_stem_never_overwrites_a_finished_recording() {
        let dir = temp_dir("collide");
        let stem = "Freally Capture 2026-07-03 14-30-00";

        let first = unique_recording_path(&dir, stem, "frec", false);
        assert_eq!(
            first.file_name().unwrap(),
            "Freally Capture 2026-07-03 14-30-00.frec"
        );
        std::fs::write(&first, b"x").expect("write");

        // A second session resolving the same stem gets a fresh name.
        let second = unique_recording_path(&dir, stem, "frec", false);
        assert_eq!(
            second.file_name().unwrap(),
            "Freally Capture 2026-07-03 14-30-00 (2).frec"
        );
        assert!(!second.exists());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn splitting_collision_checks_the_first_segment() {
        let dir = temp_dir("split");
        // A split session writes `… part001.frec`, not the bare base — the
        // guard must still see the clash.
        std::fs::write(dir.join("Rec 2026-07-03 14-30-00 part001.frec"), b"x").expect("write");
        let next = unique_recording_path(&dir, "Rec 2026-07-03 14-30-00", "frec", true);
        assert_eq!(
            next.file_name().unwrap(),
            "Rec 2026-07-03 14-30-00 (2).frec"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }
}
