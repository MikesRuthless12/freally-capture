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
use std::time::Duration;

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
    /// The last finished session's result.
    last: Mutex<(Vec<String>, Option<String>)>,
}

impl RecordingState {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(None),
            starting: AtomicBool::new(false),
            active: AtomicBool::new(false),
            feed: Mutex::new(None),
            last: Mutex::new((Vec::new(), None)),
        }
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

/// A non-colliding output path: `{prefix} {timestamp}.{ext}`, appending
/// ` (2)`, ` (3)`… if that base (or, when splitting, its first `part…`
/// segment) already exists — so two sessions in the same local-time second
/// never overwrite each other.
fn unique_recording_path(
    folder: &std::path::Path,
    prefix: &str,
    timestamp: &str,
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
            format!("{prefix} {timestamp}.{ext}")
        } else {
            format!("{prefix} {timestamp} ({}).{ext}", n + 1)
        };
        let base = folder.join(name);
        if !taken(&base) {
            return base;
        }
    }
    folder.join(format!("{prefix} {timestamp}.{ext}"))
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
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H-%M-%S").to_string();
    // Second-granularity local timestamps collide (a fast restart, or a DST
    // fall-back hour), and both sink paths truncate — so guard uniqueness
    // rather than silently overwrite a finished recording.
    let path = unique_recording_path(
        &folder,
        &settings.filename_prefix,
        &timestamp,
        settings.container.extension(),
        split.is_some(),
    );

    let sink: Box<dyn RecordSink> = match settings.container {
        Container::Frec => Box::new(FrecSink::create(spec.clone(), path.clone(), split)?),
        wire => {
            let ready = app.state::<EncodeState>().ready_ffmpeg().ok_or_else(|| {
                "recording to this container needs the ffmpeg component — install it from \
                 Components (the owned lossless .frec format needs nothing)"
                    .to_string()
            })?;
            let encoder_id = resolve_encoder(app, &settings, wire)?;
            let plan = WirePlan {
                container: wire,
                encoder_id,
                rate_control: settings.rate_control,
                preset: settings.preset,
                keyframe_sec: settings.keyframe_sec,
                audio_bitrate_kbps: settings.audio_bitrate_kbps,
                split_minutes: split,
                path: path.clone(),
            };
            Box::new(FfmpegSink::spawn(&ready, &spec, &plan)?)
        }
    };

    let recorder = Recorder::start(spec, sink);
    let handle = recorder.handle();

    // Tap the mixer: every 10 ms block of the enabled tracks flows to the
    // recorder from this moment (pause gating happens in the handle).
    let tap_handle = handle.clone();
    app.state::<AudioRuntime>()
        .engine
        .set_record_tap(Some(RecordTap {
            tracks: settings.tracks_mask,
            sink: Box::new(move |blocks| tap_handle.push_audio_blocks(blocks)),
        }));

    *state
        .feed
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(handle.clone());
    *state.lock_inner() = Some(Active {
        recorder: Some(recorder),
        handle,
        display_path: path.display().to_string(),
        container: settings.container,
        tracks: track_count,
        finalizing: false,
    });
    state.active.store(true, Ordering::Relaxed);
    emit_status(app);
    println!("recording: started → {}", path.display());
    Ok(())
}

/// Resolve "auto" (or validate an explicit encoder) against the verified
/// catalog, honestly: an encoder this machine refused is an error, not a
/// silent switch.
fn resolve_encoder<R: Runtime>(
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
    drop(inner);
    emit_status(app);
    Ok(())
}

/// Stop and finalize. Blocking (trailers, faststart) — call off the UI
/// thread. Returns the finished file paths.
pub fn stop<R: Runtime>(app: &AppHandle<R>) -> Result<Vec<String>, String> {
    let state = app.state::<RecordingState>();
    let recorder = {
        let mut inner = state.lock_inner();
        let active = inner.as_mut().ok_or("no recording is running")?;
        if active.finalizing {
            return Err("the recording is already finalizing".to_string());
        }
        active.finalizing = true;
        active
            .recorder
            .take()
            .ok_or("the recorder is already stopping")?
    };
    // Stop the feeds first: no frame lands after the user pressed Stop.
    state.active.store(false, Ordering::Relaxed);
    *state
        .feed
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = None;
    app.state::<AudioRuntime>().engine.set_record_tap(None);
    emit_status(app);

    let result = recorder.stop();
    let (paths, error) = match &result {
        Ok(paths) => (
            paths.iter().map(|p| p.display().to_string()).collect(),
            None,
        ),
        Err(err) => (Vec::new(), Some(err.clone())),
    };
    {
        let mut last = state
            .last
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        *last = (paths, error);
    }
    *state.lock_inner() = None;
    emit_status(app);
    match result {
        Ok(paths) => {
            let paths: Vec<String> = paths.iter().map(|p| p.display().to_string()).collect();
            println!("recording: finished → {}", paths.join(", "));
            Ok(paths)
        }
        Err(err) => Err(err),
    }
}

/// The status emitter: ~2 Hz while a session runs, and the watchdog that
/// stops a session whose sink died (ffmpeg crash, disk full) so the failure
/// is surfaced instead of silently eating frames forever.
pub fn spawn_status_thread<R: Runtime>(app: AppHandle<R>) {
    std::thread::Builder::new()
        .name("fcap-rec-status".into())
        .spawn(move || loop {
            let state = app.state::<RecordingState>();
            let (running, broken) = {
                let inner = state.lock_inner();
                match inner.as_ref() {
                    Some(active) => (true, !active.finalizing && active.handle.error().is_some()),
                    None => (false, false),
                }
            };
            if broken {
                let err = stop(&app).err().unwrap_or_else(|| "unknown".into());
                eprintln!("recording: session died: {err}");
            } else if running && !emit_status(&app) {
                return; // the app is gone — wind down
            }
            std::thread::sleep(STATUS_TICK);
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
    fn same_second_never_overwrites_a_finished_recording() {
        let dir = temp_dir("collide");
        let ts = "2026-07-03 14-30-00";

        let first = unique_recording_path(&dir, "Freally Capture", ts, "frec", false);
        assert_eq!(
            first.file_name().unwrap(),
            "Freally Capture 2026-07-03 14-30-00.frec"
        );
        std::fs::write(&first, b"x").expect("write");

        // A second session in the same local-time second gets a fresh name.
        let second = unique_recording_path(&dir, "Freally Capture", ts, "frec", false);
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
        let ts = "2026-07-03 14-30-00";
        // A split session writes `… part001.frec`, not the bare base — the
        // guard must still see the clash.
        std::fs::write(dir.join("Rec 2026-07-03 14-30-00 part001.frec"), b"x").expect("write");
        let next = unique_recording_path(&dir, "Rec", ts, "frec", true);
        assert_eq!(
            next.file_name().unwrap(),
            "Rec 2026-07-03 14-30-00 (2).frec"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }
}
