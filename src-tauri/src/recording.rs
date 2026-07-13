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

use fcap_audio::RecordTap;
use fcap_encode::mux::{Container, FfmpegSink, FrecSink, WirePlan};
use fcap_encode::recorder::{RecordSink, RecordSpec, Recorder, RecorderHandle};
use fcap_encode::VideoCodec;

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
    display_path: String,
    container: Container,
    tracks: u32,
    finalizing: bool,
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
    /// The last finished session's result.
    last: Mutex<(Vec<String>, Option<String>)>,
    /// Stream markers (TASK-610): timestamps (ms) dropped by the hotkey,
    /// written as mkv chapters / a sidecar file at stop.
    markers: Mutex<Vec<u64>>,
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

    // One sink builder serves both canvases — the vertical file is the same
    // configuration at its own geometry and `… (vertical)` path.
    let build_sink =
        |spec: &RecordSpec, path: &std::path::Path| -> Result<Box<dyn RecordSink>, String> {
            match settings.container {
                Container::Frec => Ok(Box::new(FrecSink::create(
                    spec.clone(),
                    path.to_path_buf(),
                    split,
                )?)),
                wire => {
                    let ready = app.state::<EncodeState>().ready_ffmpeg().ok_or_else(|| {
                        "recording to this container needs the ffmpeg component — install it from \
                     Components (the owned lossless .frec format needs nothing)"
                            .to_string()
                    })?;
                    let encoder_id = match &encoder_override {
                        Some(id) => id.clone(),
                        None => resolve_encoder(app, &settings, wire)?,
                    };
                    let plan = WirePlan {
                        container: wire,
                        encoder_id,
                        rate_control: settings.rate_control,
                        preset: settings.preset,
                        keyframe_sec: settings.keyframe_sec,
                        audio_bitrate_kbps: settings.audio_bitrate_kbps,
                        split_minutes: split,
                        // TASK-609: optional output downscale (wire only —
                        // the lossless .frec always records the canvas).
                        scale: (settings.output_width > 0 && settings.output_height > 0)
                            .then_some((settings.output_width, settings.output_height)),
                        path: path.to_path_buf(),
                    };
                    Ok(Box::new(FfmpegSink::spawn(&ready, spec, &plan)?))
                }
            }
        };

    let sink = build_sink(&spec, &path)?;

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
                let vertical_sink = build_sink(&vertical_spec, &vertical_path)?;
                Some((vertical_spec, vertical_sink, vertical_path))
            }
            None => None, // no vertical canvas configured — record main only
        }
    } else {
        None
    };

    // CAP-M11: track the in-flight outputs so an unclean exit can offer to
    // repair them next launch. The owned .frec is crash-safe by design and
    // is never listed.
    if settings.container != Container::Frec {
        let mut in_progress = vec![crate::salvage::InProgressFile {
            path: path.clone(),
            split: split.is_some(),
        }];
        if let Some((_, _, vertical_path)) = &vertical {
            in_progress.push(crate::salvage::InProgressFile {
                path: vertical_path.clone(),
                split: split.is_some(),
            });
        }
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
    *state.lock_inner() = Some(Active {
        recorder: Some(recorder),
        handle,
        vertical_recorder,
        vertical_handle,
        display_path: path.display().to_string(),
        container: settings.container,
        tracks: track_count,
        finalizing: false,
    });
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
    let count = {
        let mut markers = state
            .markers
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        markers.push(position_ms);
        markers.len() as u32
    };
    emit_status(app);
    println!("recording: marker {count} at {position_ms} ms");
    Ok(count)
}

/// `HH:MM:SS Marker N` lines — the sidecar shape (YouTube-chapter-like).
fn markers_sidecar_text(markers_ms: &[u64]) -> String {
    let mut text = String::new();
    for (index, &ms) in markers_ms.iter().enumerate() {
        let secs = ms / 1000;
        text.push_str(&format!(
            "{:02}:{:02}:{:02} Marker {}\n",
            secs / 3600,
            (secs % 3600) / 60,
            secs % 60,
            index + 1
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
    drop(inner);
    emit_status(app);
    Ok(())
}

/// Stop and finalize. Blocking (trailers, faststart) — call off the UI
/// thread. Returns the finished file paths.
pub fn stop<R: Runtime>(app: &AppHandle<R>) -> Result<Vec<String>, String> {
    let state = app.state::<RecordingState>();
    let (recorder, vertical_recorder, total_ms) = {
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
        (recorder, active.vertical_recorder.take(), total_ms)
    };
    // Stop the feeds first: no frame lands after the user pressed Stop.
    state.active.store(false, Ordering::Relaxed);
    state.vertical_active.store(false, Ordering::Relaxed);
    *state
        .feed
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = None;
    *state
        .vertical_feed
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = None;
    app.state::<AudioRuntime>().engine.set_record_tap(None);
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
    // CAP-M11: every output finalized cleanly — nothing to salvage next
    // launch. Any finalize failure keeps the sidecar, so a later crash still
    // surfaces the repair offer for the damaged file.
    if result.is_ok() && error.is_none() {
        crate::salvage::clear_in_progress();
    }
    // TASK-610: attach the dropped markers — embedded chapters for a single
    // mkv, a readable sidecar otherwise. Best-effort: a chapter failure
    // never invalidates the finished recording.
    let markers: Vec<u64> = std::mem::take(
        &mut *state
            .markers
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner),
    );
    if !markers.is_empty() && !paths.is_empty() {
        let first = std::path::PathBuf::from(&paths[0]);
        let embedded = paths.len() == 1
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
    use super::unique_recording_path;

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
