//! The app side of the rolling replay buffer (Phase 6, TASK-603): while
//! armed, a background encode keeps the last N seconds as small MPEG-TS
//! segments in a transient ring directory (bounded disk, tiny memory — the
//! Phase 5 lesson: never buffer raw frames). **Save Replay** concat-copies
//! the ring's tail into a playable `.mkv` in the recordings folder without
//! re-encoding — and without touching the live stream or the recording,
//! which run on their own taps and sessions.

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, Runtime};

use fcap_audio::RecordTap;
use fcap_encode::{
    concat_copy, EncPreset, FfmpegSink, RateControl, RcMode, RecordSink, RecordSpec, ReplayPlan,
};
use fcap_stream::replay as ring;
use fcap_stream::{StreamHandle, StreamSession, StreamSpec, StreamState};

use crate::audio::AudioRuntime;
use crate::commands::recording::EncodeState;
use crate::settings::SettingsStore;
use crate::studio::StudioState;

struct ActiveReplay {
    session: Option<StreamSession>,
    handle: StreamHandle,
    dir: PathBuf,
    seconds: u32,
    janitor_stop: Arc<AtomicBool>,
    /// Held true while a Save picks + concats the tail, so the janitor
    /// never unlinks a segment ffmpeg is about to open.
    save_lock: Arc<AtomicBool>,
    janitor: Option<std::thread::JoinHandle<()>>,
}

/// Managed Tauri state: the (single) armed replay buffer.
pub struct ReplayState {
    inner: Mutex<Option<ActiveReplay>>,
    /// Serializes arm (it does catalog + child I/O before registering).
    starting: AtomicBool,
    /// Lock-free "is the buffer armed" for the render loop's per-frame check.
    active: AtomicBool,
    /// The feed the render loop pushes into (cloned out under one lock).
    feed: Mutex<Option<StreamHandle>>,
    /// One save at a time (concat runs a child process).
    saving: AtomicBool,
    /// The last saved replay path, for the UI's confirmation.
    last_saved: Mutex<Option<PathBuf>>,
}

impl ReplayState {
    pub fn new() -> Self {
        ReplayState {
            inner: Mutex::new(None),
            starting: AtomicBool::new(false),
            active: AtomicBool::new(false),
            feed: Mutex::new(None),
            saving: AtomicBool::new(false),
            last_saved: Mutex::new(None),
        }
    }

    fn lock_inner(&self) -> std::sync::MutexGuard<'_, Option<ActiveReplay>> {
        self.inner
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// Whether the render loop should hand this state program frames.
    pub fn wants_frames(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }

    /// Whether the buffer is armed — the quit guard's check (CAP-M23).
    pub fn is_armed(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }

    /// Push the newest program frame (never blocks; the buffer drops
    /// honestly when its encoder can't keep up).
    pub fn push_video(&self, pixels: Arc<Vec<u8>>) {
        let feed = self
            .feed
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(handle) = feed.as_ref() {
            handle.push_frame(pixels);
        }
    }

    pub fn status(&self) -> ReplayDto {
        let inner = self.lock_inner();
        let last_saved = self
            .last_saved
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .as_ref()
            .map(|path| path.display().to_string());
        match inner.as_ref() {
            None => ReplayDto {
                armed: false,
                state: "idle".to_string(),
                seconds: 0,
                error: None,
                last_saved,
            },
            Some(active) => {
                let status = active.handle.status();
                ReplayDto {
                    armed: true,
                    state: match &status.state {
                        StreamState::Live => "buffering",
                        StreamState::Reconnecting { .. } => "recovering",
                        StreamState::Ended { error: Some(_) } => "failed",
                        StreamState::Ended { error: None } => "idle",
                    }
                    .to_string(),
                    seconds: active.seconds,
                    error: match status.state {
                        StreamState::Ended { error } => error,
                        _ => None,
                    },
                    last_saved,
                }
            }
        }
    }
}

impl Default for ReplayState {
    fn default() -> Self {
        Self::new()
    }
}

/// The `replay` event payload / `replay_status` result.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplayDto {
    pub armed: bool,
    /// "idle" | "buffering" | "recovering" | "failed".
    pub state: String,
    /// The armed window length (0 when idle).
    pub seconds: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_saved: Option<String>,
}

fn emit_status<R: Runtime>(app: &AppHandle<R>) {
    let dto = app.state::<ReplayState>().status();
    let _ = app.emit("replay", &dto);
}

struct ResetOnDrop<'a>(&'a AtomicBool);
impl Drop for ResetOnDrop<'_> {
    fn drop(&mut self) {
        self.0.store(false, Ordering::SeqCst);
    }
}

/// The transient ring directory — per-process, under the OS temp dir (the
/// buffer is scratch by design; a saved replay moves into the recordings
/// folder).
fn ring_dir() -> PathBuf {
    std::env::temp_dir().join(format!("fcap-replay-{}", std::process::id()))
}

/// This run's roll-clip folder (CAP-N10) — sibling of the ring dir, same
/// per-process scratch contract.
fn roll_dir() -> PathBuf {
    std::env::temp_dir().join(format!("fcap-replay-roll-{}", std::process::id()))
}

/// Reclaim `fcap-replay-*` ring directories left by earlier runs that
/// crashed or were force-killed while armed (a clean disarm removes its
/// own). Best-effort; keeps our own current directory. Called at arm so a
/// stale buffer's gigabytes can never silently accumulate across crashes.
fn sweep_stale_rings() {
    let ours = ring_dir();
    // This run's roll folder is live playback material (CAP-N10) — a re-arm
    // must not rip a playing clip out from under a replay source. Stale
    // roll folders from earlier runs match the prefix and ARE swept.
    let our_rolls = roll_dir();
    let Ok(entries) = std::fs::read_dir(std::env::temp_dir()) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path == ours || path == our_rolls {
            continue;
        }
        let is_ring = path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.starts_with("fcap-replay-"));
        if is_ring && path.is_dir() {
            let _ = std::fs::remove_dir_all(&path);
        }
    }
}

/// Arm the buffer: start the background segment encode + the ring janitor.
pub fn arm<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    let state = app.state::<ReplayState>();
    if state.starting.swap(true, Ordering::SeqCst) {
        return Err("the replay buffer is already starting".to_string());
    }
    let _reset = ResetOnDrop(&state.starting);
    if state.lock_inner().is_some() {
        return Err("the replay buffer is already armed".to_string());
    }

    let settings = app.state::<SettingsStore>().get().replay;
    settings.validate()?;
    let ready = app.state::<EncodeState>().ready_ffmpeg().ok_or_else(|| {
        "the replay buffer needs the ffmpeg component — install it from Components".to_string()
    })?;
    let encoder_id = crate::stream::resolve_stream_encoder(app, "auto")?;

    let snapshot = app.state::<StudioState>().snapshot();
    let (width, height) = (
        snapshot.collection.canvas_width,
        snapshot.collection.canvas_height,
    );

    // Reclaim any ring dirs orphaned by an earlier crash/kill-while-armed.
    sweep_stale_rings();

    let dir = ring_dir();
    std::fs::create_dir_all(&dir)
        .map_err(|err| format!("could not create the replay buffer dir: {err}"))?;
    // A fresh arm starts a fresh history.
    for (_, stale) in ring::list_segments(&dir) {
        let _ = std::fs::remove_file(stale);
    }

    let factory = {
        let ready = ready.clone();
        let dir = dir.clone();
        let settings = settings.clone();
        // The replay ring keeps its encoder — no failover ladder here.
        Box::new(move |_death: Option<&fcap_stream::SinkDeath>| {
            let plan = ReplayPlan {
                encoder_id: encoder_id.clone(),
                rate_control: RateControl {
                    mode: RcMode::Cbr,
                    bitrate_kbps: settings.bitrate_kbps,
                    cq: 23,
                },
                preset: EncPreset::Performance,
                audio_bitrate_kbps: settings.audio_bitrate_kbps,
                segment_sec: ring::SEGMENT_SEC,
                dir: dir.clone(),
                // Continue numbering after a respawn — never collide.
                start_number: ring::next_start_number(&dir),
            };
            let spec = RecordSpec {
                width,
                height,
                fps: settings.fps,
                tracks: vec![0],
            };
            Ok(Box::new(FfmpegSink::spawn_replay(&ready, &spec, &plan)?) as Box<dyn RecordSink>)
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

    // The buffer's independent mixer tap (the third twin).
    let tap_handle = handle.clone();
    app.state::<AudioRuntime>()
        .engine
        .set_replay_tap(Some(RecordTap {
            tracks: 1 << (settings.track - 1),
            sink: Box::new(move |blocks| {
                if let Some((_, samples)) = blocks.first() {
                    tap_handle.push_audio(samples);
                }
            }),
        }));

    // The janitor prunes the ring a little slower than segments appear —
    // but never while a Save is stitching the tail (it would unlink a
    // segment ffmpeg is about to open).
    let janitor_stop = Arc::new(AtomicBool::new(false));
    let save_lock = Arc::new(AtomicBool::new(false));
    let janitor = {
        let stop = Arc::clone(&janitor_stop);
        let saving = Arc::clone(&save_lock);
        let dir = dir.clone();
        let keep = ring::keep_count(settings.seconds, ring::SEGMENT_SEC);
        std::thread::Builder::new()
            .name("fcap-replay-janitor".into())
            .spawn(move || {
                while !stop.load(Ordering::Relaxed) {
                    if !saving.load(Ordering::Relaxed) {
                        ring::prune(&dir, keep);
                    }
                    std::thread::sleep(Duration::from_secs(2));
                }
            })
            .map_err(|err| err.to_string())?
    };

    *state
        .feed
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(handle.clone());
    *state.lock_inner() = Some(ActiveReplay {
        session: Some(session),
        handle,
        dir,
        seconds: settings.seconds,
        janitor_stop,
        save_lock,
        janitor: Some(janitor),
    });
    state.active.store(true, Ordering::Relaxed);
    emit_status(app);
    println!("replay: armed ({} s buffer)", settings.seconds);
    Ok(())
}

/// Disarm the buffer: stop the encode + janitor, clear the ring dir.
pub fn disarm<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    let state = app.state::<ReplayState>();
    let Some(mut active) = state.lock_inner().take() else {
        return Err("the replay buffer is not armed".to_string());
    };
    state.active.store(false, Ordering::Relaxed);
    *state
        .feed
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = None;
    app.state::<AudioRuntime>().engine.set_replay_tap(None);
    active.janitor_stop.store(true, Ordering::Relaxed);
    if let Some(janitor) = active.janitor.take() {
        let _ = janitor.join();
    }
    if let Some(session) = active.session.take() {
        let _ = session.stop();
    }
    let _ = std::fs::remove_dir_all(&active.dir);
    emit_status(app);
    println!("replay: disarmed");
    Ok(())
}

/// Save the last N seconds to a playable file — the hotkey's action. Never
/// interrupts the buffer, the stream, or the recording.
pub fn save<R: Runtime>(app: &AppHandle<R>) -> Result<PathBuf, String> {
    let state = app.state::<ReplayState>();
    let (dir, seconds, save_lock) = {
        let inner = state.lock_inner();
        let Some(active) = inner.as_ref() else {
            return Err("arm the replay buffer first".to_string());
        };
        (
            active.dir.clone(),
            active.seconds,
            Arc::clone(&active.save_lock),
        )
    };
    if state.saving.swap(true, Ordering::SeqCst) {
        return Err("a replay save is already running".to_string());
    }
    let _reset = ResetOnDrop(&state.saving);
    // Pause the janitor for the whole pick + concat so it can never unlink a
    // segment ffmpeg is about to open.
    save_lock.store(true, Ordering::SeqCst);
    let _release = ResetOnDrop(&save_lock);

    let ready = app.state::<EncodeState>().ready_ffmpeg().ok_or_else(|| {
        "the replay buffer needs the ffmpeg component — install it from Components".to_string()
    })?;
    let segments = ring::list_segments(&dir);
    let picked = ring::pick_for_save(&segments, seconds, ring::SEGMENT_SEC);
    if picked.is_empty() {
        return Err("the replay buffer is still empty — give it a moment".to_string());
    }

    let recording = app.state::<SettingsStore>().get().recording;
    let folder = crate::recording::output_folder(&recording, &recording.replay_folder);
    std::fs::create_dir_all(&folder)
        .map_err(|err| format!("could not create the replay folder: {err}"))?;
    // CAP-M25: replays resolve their own token template; `{marker-count}`
    // is the live session's count ("saved after marker 3").
    let counter = crate::recording::counter_for(app, &recording.replay_template, recording.counter);
    let canvas = app
        .state::<crate::studio::StudioState>()
        .with_collection(|collection| (collection.canvas_width, collection.canvas_height));
    let naming = crate::recording::naming_context(app, "Replay".to_owned(), canvas, counter);
    let stem = crate::filename::resolve_template(&recording.replay_template, &naming);
    let out = crate::recording::unique_recording_path(&folder, &stem, "mkv", false);

    concat_copy(&ready, &picked, &out)?;
    *state
        .last_saved
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(out.clone());
    let _ = app.emit(
        "replay_saved",
        serde_json::json!({ "path": out.display().to_string() }),
    );
    emit_status(app);
    println!("replay: saved {}", out.display());
    Ok(out)
}

/// CAP-N10: snapshot the last `seconds` into a TEMPORARY clip for a replay
/// playback source (stream copy — fast; never touches the recordings
/// folder). Alternating file names, so a clip still playing is never
/// overwritten by the next roll.
fn snapshot_clip<R: Runtime>(app: &AppHandle<R>, seconds: u32) -> Result<PathBuf, String> {
    static ROLL_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let state = app.state::<ReplayState>();
    let (dir, armed_seconds, save_lock) = {
        let inner = state.lock_inner();
        let Some(active) = inner.as_ref() else {
            return Err("arm the replay buffer first".to_string());
        };
        (
            active.dir.clone(),
            active.seconds,
            Arc::clone(&active.save_lock),
        )
    };
    if state.saving.swap(true, Ordering::SeqCst) {
        return Err("a replay save is already running".to_string());
    }
    let _reset = ResetOnDrop(&state.saving);
    save_lock.store(true, Ordering::SeqCst);
    let _release = ResetOnDrop(&save_lock);

    let ready = app.state::<EncodeState>().ready_ffmpeg().ok_or_else(|| {
        "the replay buffer needs the ffmpeg component — install it from Components".to_string()
    })?;
    let segments = ring::list_segments(&dir);
    let take = seconds.clamp(2, armed_seconds.max(2));
    let picked = ring::pick_for_save(&segments, take, ring::SEGMENT_SEC);
    if picked.is_empty() {
        return Err("the replay buffer is still empty — give it a moment".to_string());
    }
    let folder = roll_dir();
    std::fs::create_dir_all(&folder)
        .map_err(|err| format!("could not create the roll folder: {err}"))?;
    let out = folder.join(format!(
        "roll-{}.mkv",
        ROLL_COUNTER.fetch_add(1, Ordering::Relaxed)
    ));
    concat_copy(&ready, &picked, &out)?;
    Ok(out)
}

/// CAP-N10: roll every LIVE replay playback source — each gets a clip cut
/// to its own configured length. The hotkey's action.
pub fn roll_sources<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    let live = fcap_sources::replaysrc::live_ids();
    if live.is_empty() {
        return Err("no Instant Replay source is in the scene".to_string());
    }
    let wanted: Vec<(String, u32)> =
        app.state::<crate::studio::StudioState>()
            .with_collection(|collection| {
                collection
                    .sources
                    .iter()
                    .filter_map(|source| match &source.settings {
                        fcap_scene::SourceSettings::ReplayPlayback { seconds, .. } => {
                            Some((source.id.0.to_string(), *seconds))
                        }
                        _ => None,
                    })
                    .collect()
            });
    let mut rolled = false;
    let mut failures: Vec<String> = Vec::new();
    for (id, seconds) in wanted {
        if !live.contains(&id) {
            continue;
        }
        // One source's snapshot failing must not strand the rest un-rolled —
        // try every live source, then report honestly.
        match snapshot_clip(app, seconds) {
            Ok(clip) => rolled = fcap_sources::replaysrc::roll(&id, clip) || rolled,
            Err(err) => failures.push(err),
        }
    }
    if let Some(detail) = failures.first() {
        return Err(if rolled {
            format!("some Instant Replay sources did not roll: {detail}")
        } else {
            detail.clone()
        });
    }
    if rolled {
        Ok(())
    } else {
        Err("no Instant Replay source is live".to_string())
    }
}

/// CAP-N10: roll ONE replay source (the properties-dialog button).
pub fn roll_one<R: Runtime>(
    app: &AppHandle<R>,
    source_id: fcap_scene::SourceId,
) -> Result<(), String> {
    let seconds = app
        .state::<crate::studio::StudioState>()
        .with_collection(|collection| {
            collection
                .sources
                .iter()
                .find_map(|source| match &source.settings {
                    fcap_scene::SourceSettings::ReplayPlayback { seconds, .. }
                        if source.id == source_id =>
                    {
                        Some(*seconds)
                    }
                    _ => None,
                })
        })
        .ok_or_else(|| "that source is not an Instant Replay".to_string())?;
    let clip = snapshot_clip(app, seconds)?;
    if !fcap_sources::replaysrc::roll(&source_id.0.to_string(), clip) {
        return Err("that Instant Replay source is not live — put it in the scene".to_string());
    }
    Ok(())
}

// -- commands -----------------------------------------------------------------

/// Arm the replay buffer. Off the UI thread — encoder detection + the
/// ffmpeg child spawn are blocking.
#[tauri::command]
pub async fn replay_arm<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || arm(&app))
        .await
        .map_err(|err| format!("replay arm task failed: {err}"))?
}

/// Disarm the replay buffer (drops the un-saved history).
#[tauri::command]
pub async fn replay_disarm<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || disarm(&app))
        .await
        .map_err(|err| format!("replay disarm task failed: {err}"))?
}

/// Save the last N seconds to the recordings folder; returns the path.
#[tauri::command]
pub async fn replay_save<R: Runtime>(app: AppHandle<R>) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || save(&app).map(|path| path.display().to_string()))
        .await
        .map_err(|err| format!("replay save task failed: {err}"))?
}

/// The current replay-buffer status (the `replay` event pushes the same).
#[tauri::command]
pub fn replay_status(state: tauri::State<'_, ReplayState>) -> ReplayDto {
    state.status()
}

/// CAP-N10: roll one Instant Replay source (the properties button). Off
/// the UI thread — the snapshot's stream copy is blocking.
#[tauri::command]
pub async fn replay_roll_source<R: Runtime>(
    app: AppHandle<R>,
    source_id: fcap_scene::SourceId,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || roll_one(&app, source_id))
        .await
        .map_err(|err| format!("replay roll task failed: {err}"))?
}

/// ~1 Hz status while armed (buffer health + honest failure); winds down
/// when the app is gone.
pub fn spawn_status_thread<R: Runtime>(app: AppHandle<R>) {
    std::thread::Builder::new()
        .name("fcap-replay-status".into())
        .spawn(move || loop {
            let state = app.state::<ReplayState>();
            if state.lock_inner().is_some() && app.emit("replay", &state.status()).is_err() {
                return;
            }
            std::thread::sleep(Duration::from_secs(1));
        })
        .expect("replay status thread spawns");
}
