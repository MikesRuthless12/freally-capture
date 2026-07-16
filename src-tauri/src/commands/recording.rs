//! The recording + encoder command surface (Phase 4).
//!
//! - `encoders_list` feeds the encoder picker: the detected hardware
//!   encoders plus the always-available CPU fallbacks, `verified` against
//!   the installed ffmpeg component when there is one (session-cached —
//!   smoke tests spawn processes).
//! - The `ffmpeg_*` commands drive the **clearly-labeled, on-demand ffmpeg
//!   component** (never bundled; hash-verified): status, install with live
//!   progress on the `ffmpeg` event, cancel, remove.
//!
//! The recording session commands land with the recorder (P4.3/P4.5).

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, Runtime, State};

use fcap_encode::ffmpeg::{self, FetchPhase, FetchProgress};
use fcap_encode::Catalog;

/// The pinned-build info the panel shows before anything is fetched.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FfmpegBuildDto {
    pub version: String,
    pub source: String,
    pub url: String,
    pub size_bytes: u64,
}

fn pinned_dto() -> Option<FfmpegBuildDto> {
    ffmpeg::pinned_build().map(|pin| FfmpegBuildDto {
        version: pin.version.to_string(),
        source: pin.source.to_string(),
        url: pin.url.to_string(),
        size_bytes: pin.size_bytes,
    })
}

/// The `ffmpeg` event payload + `ffmpeg_status` result (mirrored in
/// `ui/src/api/types.ts` as a discriminated union on `state`).
#[derive(Debug, Clone, Serialize)]
#[serde(
    rename_all = "camelCase",
    tag = "state",
    rename_all_fields = "camelCase"
)]
pub enum FfmpegStatusDto {
    /// Not installed; `build` describes what an install would fetch
    /// (`None` = no build pinned for this platform, said honestly).
    Missing {
        build: Option<FfmpegBuildDto>,
    },
    Downloading {
        received_bytes: u64,
        total_bytes: Option<u64>,
        bytes_per_sec: u64,
    },
    Verifying,
    Extracting,
    Ready {
        version: String,
        path: String,
    },
    Error {
        message: String,
        build: Option<FfmpegBuildDto>,
    },
}

/// Tauri-managed encode-side runtime state.
pub struct EncodeState {
    status: Mutex<FfmpegStatusDto>,
    installing: AtomicBool,
    cancel: Arc<AtomicBool>,
    /// Session cache of the verified catalog (smoke tests spawn processes —
    /// run them once per install state, not per picker open).
    catalog: Mutex<Option<Catalog>>,
}

impl EncodeState {
    pub fn new() -> Self {
        let status = match ffmpeg::installed() {
            Some(ready) => FfmpegStatusDto::Ready {
                version: ready.version,
                path: ready.path.display().to_string(),
            },
            None => FfmpegStatusDto::Missing {
                build: pinned_dto(),
            },
        };
        Self {
            status: Mutex::new(status),
            installing: AtomicBool::new(false),
            cancel: Arc::new(AtomicBool::new(false)),
            catalog: Mutex::new(None),
        }
    }

    fn lock_status(&self) -> std::sync::MutexGuard<'_, FfmpegStatusDto> {
        self.status
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn lock_catalog(&self) -> std::sync::MutexGuard<'_, Option<Catalog>> {
        self.catalog
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// The installed component, when status says Ready.
    pub fn ready_ffmpeg(&self) -> Option<fcap_encode::Ffmpeg> {
        matches!(*self.lock_status(), FfmpegStatusDto::Ready { .. })
            .then(ffmpeg::installed)
            .flatten()
    }
}

impl Default for EncodeState {
    fn default() -> Self {
        Self::new()
    }
}

fn set_and_emit<R: Runtime>(app: &AppHandle<R>, status: FfmpegStatusDto) {
    let state = app.state::<EncodeState>();
    *state.lock_status() = status.clone();
    let _ = app.emit("ffmpeg", &status);
}

/// Blocking catalog resolution shared by [`encoders_list`] and recording
/// start: detect + verify once per install state, then session-cached.
/// Call from a worker thread — the smoke tests spawn processes.
pub fn ensure_catalog<R: Runtime>(app: &AppHandle<R>) -> Result<Catalog, String> {
    let cached = app.state::<EncodeState>().lock_catalog().clone();
    if let Some(catalog) = cached {
        return Ok(catalog);
    }
    let mut catalog = Catalog::detect();
    if let Some(ready) = app.state::<EncodeState>().ready_ffmpeg() {
        if let Err(err) = ffmpeg::verify_catalog(&mut catalog, &ready) {
            eprintln!("encode: could not verify the catalog against ffmpeg: {err}");
        }
    }
    *app.state::<EncodeState>().lock_catalog() = Some(catalog.clone());
    Ok(catalog)
}

/// Detect the encoder catalog (GPU enumeration + per-OS rules), verified
/// against the installed ffmpeg when present. Async — detection and the
/// first-run smoke tests must not block the UI.
#[tauri::command]
pub async fn encoders_list<R: Runtime>(app: AppHandle<R>) -> Result<Catalog, String> {
    tauri::async_runtime::spawn_blocking(move || ensure_catalog(&app))
        .await
        .map_err(|err| format!("encoder detection task failed: {err}"))?
}

/// The ffmpeg component's current status.
#[tauri::command]
pub fn ffmpeg_status(state: State<'_, EncodeState>) -> FfmpegStatusDto {
    state.lock_status().clone()
}

/// Start the on-demand install (explicit user action from the labeled
/// panel — the app never fetches on its own). Progress rides the `ffmpeg`
/// event; the command returns immediately.
#[tauri::command]
pub fn ffmpeg_install<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    let state = app.state::<EncodeState>();
    if ffmpeg::pinned_build().is_none() {
        return Err("no ffmpeg build is pinned for this platform yet".to_string());
    }
    if state.installing.swap(true, Ordering::SeqCst) {
        return Err("an install is already in progress".to_string());
    }
    state.cancel.store(false, Ordering::SeqCst);
    let cancel = Arc::clone(&state.cancel);
    let handle = app.clone();

    let spawned = std::thread::Builder::new()
        .name("fcap-ffmpeg-fetch".into())
        .spawn(move || {
            let progress_handle = handle.clone();
            let result = ffmpeg::install(
                move |progress: FetchProgress| {
                    let status = match progress.phase {
                        FetchPhase::Downloading => FfmpegStatusDto::Downloading {
                            received_bytes: progress.received,
                            total_bytes: progress.total,
                            bytes_per_sec: progress.bytes_per_sec,
                        },
                        FetchPhase::Verifying => FfmpegStatusDto::Verifying,
                        FetchPhase::Extracting => FfmpegStatusDto::Extracting,
                    };
                    set_and_emit(&progress_handle, status);
                },
                &cancel,
            );
            let state = handle.state::<EncodeState>();
            match result {
                Ok(ready) => {
                    // A fresh component invalidates the verified catalog.
                    *state.lock_catalog() = None;
                    set_and_emit(
                        &handle,
                        FfmpegStatusDto::Ready {
                            version: ready.version,
                            path: ready.path.display().to_string(),
                        },
                    );
                }
                Err(fcap_encode::FfmpegError::Cancelled) => {
                    set_and_emit(
                        &handle,
                        FfmpegStatusDto::Missing {
                            build: pinned_dto(),
                        },
                    );
                }
                Err(err) => {
                    set_and_emit(
                        &handle,
                        FfmpegStatusDto::Error {
                            message: err.to_string(),
                            build: pinned_dto(),
                        },
                    );
                }
            }
            state.installing.store(false, Ordering::SeqCst);
        });

    if let Err(err) = spawned {
        // The flag was swapped true above; the worker that would clear it
        // never started, so clear it here or the panel is wedged at "an
        // install is already in progress" for the rest of the session.
        state.installing.store(false, Ordering::SeqCst);
        return Err(format!("could not start the fetch thread: {err}"));
    }
    Ok(())
}

/// Cancel an in-flight install (the partial download is removed).
#[tauri::command]
pub fn ffmpeg_cancel(state: State<'_, EncodeState>) {
    state.cancel.store(true, Ordering::SeqCst);
}

/// Remove the installed component (the panel's Remove action).
#[tauri::command]
pub fn ffmpeg_remove<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    let state = app.state::<EncodeState>();
    if state.installing.load(Ordering::SeqCst) {
        return Err("an install is in progress".to_string());
    }
    ffmpeg::remove().map_err(|err| format!("could not remove the component: {err}"))?;
    *state.lock_catalog() = None;
    set_and_emit(
        &app,
        FfmpegStatusDto::Missing {
            build: pinned_dto(),
        },
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// Recording session commands (P4.3/P4.5) — thin wrappers over
// `crate::recording`; start/stop block on I/O so they run off-thread.
// ---------------------------------------------------------------------------

/// Start recording with the persisted Settings → Output configuration.
#[tauri::command]
pub async fn recording_start<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || crate::recording::start(&app))
        .await
        .map_err(|err| format!("recording start task failed: {err}"))?
}

/// Stop + finalize; returns the finished file paths.
#[tauri::command]
pub async fn recording_stop<R: Runtime>(app: AppHandle<R>) -> Result<Vec<String>, String> {
    tauri::async_runtime::spawn_blocking(move || crate::recording::stop(&app))
        .await
        .map_err(|err| format!("recording stop task failed: {err}"))?
}

/// Drop a chapter marker at the current recording position (TASK-610).
#[tauri::command]
pub fn recording_add_marker<R: Runtime>(app: AppHandle<R>) -> Result<u32, String> {
    crate::recording::add_marker(&app)
}

/// Pause the running recording (no frames written; the file stays one
/// contiguous timeline).
#[tauri::command]
pub fn recording_pause<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    crate::recording::set_paused(&app, true)
}

/// Resume a paused recording.
#[tauri::command]
pub fn recording_resume<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    crate::recording::set_paused(&app, false)
}

/// The current recording status snapshot.
#[tauri::command]
pub fn recording_status(
    state: State<'_, crate::recording::RecordingState>,
) -> crate::recording::RecordingDto {
    state.status()
}

// ---------------------------------------------------------------------------
// Audio-only recording (CAP-N38)
// ---------------------------------------------------------------------------

/// Start an audio-only recording (per-track WAV via the owned writer).
#[tauri::command]
pub async fn audiorec_start<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || crate::audiorec::start(&app))
        .await
        .map_err(|err| format!("audio recording start task failed: {err}"))?
}

/// Stop + finalize the audio-only recording; returns the finished file paths.
#[tauri::command]
pub async fn audiorec_stop<R: Runtime>(app: AppHandle<R>) -> Result<Vec<String>, String> {
    tauri::async_runtime::spawn_blocking(move || crate::audiorec::stop(&app))
        .await
        .map_err(|err| format!("audio recording stop task failed: {err}"))?
}

/// The audio-only recording status.
#[tauri::command]
pub fn audiorec_status<R: Runtime>(app: AppHandle<R>) -> crate::audiorec::AudioRecDto {
    crate::audiorec::status(&app)
}

/// Pause / resume the audio-only recording.
#[tauri::command]
pub fn audiorec_set_paused<R: Runtime>(app: AppHandle<R>, paused: bool) -> Result<(), String> {
    crate::audiorec::set_paused(&app, paused)
}

// ---------------------------------------------------------------------------
// The recordings list + remux (P4.8)
// ---------------------------------------------------------------------------

/// One file in the recordings folder.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordingFileDto {
    pub path: String,
    pub name: String,
    pub size_bytes: u64,
    /// Unix millis of the last modification.
    pub modified_ms: u64,
    /// Lowercase extension ("frec", "mkv", …).
    pub ext: String,
    /// CAP-N42: a `.frec` whose header flags real transparency (`None` for
    /// non-frec files) — gates the alpha-preserving export buttons.
    pub frec_alpha: Option<bool>,
}

const RECORDING_EXTS: [&str; 5] = ["frec", "mkv", "mp4", "mov", "webm"];

/// List the recordings folder's media files, newest first (capped at 200).
#[tauri::command]
pub async fn recordings_list<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Vec<RecordingFileDto>, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let settings = app
            .state::<crate::settings::SettingsStore>()
            .get()
            .recording;
        let folder = crate::recording::recordings_folder(&settings);
        let mut files: Vec<RecordingFileDto> = Vec::new();
        let entries = match std::fs::read_dir(&folder) {
            Ok(entries) => entries,
            Err(_) => return Ok(Vec::new()), // no folder yet — nothing recorded
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let Some(ext) = path
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.to_ascii_lowercase())
            else {
                continue;
            };
            if !RECORDING_EXTS.contains(&ext.as_str()) {
                continue;
            }
            let Ok(meta) = entry.metadata() else { continue };
            if !meta.is_file() {
                continue;
            }
            let modified_ms = meta
                .modified()
                .ok()
                .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|duration| duration.as_millis() as u64)
                .unwrap_or(0);
            // CAP-N42: read the 30-byte `.frec` header for the alpha flag —
            // cheap (bounded read), and only for frec files.
            let frec_alpha = (ext == "frec").then(|| {
                fcap_encode::freally_video::FrecReader::open(&path)
                    .map(|reader| reader.spec().alpha)
                    .unwrap_or(false)
            });
            files.push(RecordingFileDto {
                name: entry.file_name().to_string_lossy().to_string(),
                path: path.display().to_string(),
                size_bytes: meta.len(),
                modified_ms,
                ext,
                frec_alpha,
            });
        }
        files.sort_by_key(|file| std::cmp::Reverse(file.modified_ms));
        files.truncate(200);
        Ok(files)
    })
    .await
    .map_err(|err| format!("recordings listing task failed: {err}"))?
}

/// Reject a webview-supplied path that resolves off the local disk (UNC/SMB
/// share, `file://`, or a URL) **before** any `canonicalize()`/stat probe.
/// Resolving a `\\host\share\…` path performs an SMB/NTLM handshake that would
/// leak the user's credential hash to that host — the CAP-M16 rule the rest of
/// the codebase enforces (`commands::studio::is_remote`). Legitimate local
/// paths (including mapped drive letters) pass through untouched.
fn reject_remote(path: &str) -> Result<(), String> {
    if crate::commands::studio::is_remote(path) {
        return Err("network paths are not accepted".to_string());
    }
    Ok(())
}

/// Remux an mkv recording to a sibling mp4 (stream copy — no re-encode).
/// The path must live in the recordings folder: the webview never gets to
/// point this at arbitrary files.
#[tauri::command]
pub async fn recording_remux<R: Runtime>(
    app: AppHandle<R>,
    path: String,
) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let input = guarded_recording_path(&app, &path)?;
        let ready = app
            .state::<EncodeState>()
            .ready_ffmpeg()
            .ok_or("remuxing needs the ffmpeg component — install it from Components")?;
        fcap_encode::remux::remux_to_mp4(&ready, &input).map(|out| out.display().to_string())
    })
    .await
    .map_err(|err| format!("remux task failed: {err}"))?
}

/// CAP-N34: normalize a wire recording to the app's loudness target via ffmpeg
/// `loudnorm`, writing a `(normalized)` sibling. Same recordings-folder guard as
/// remux — the webview never points this at an arbitrary file.
#[tauri::command]
pub async fn recording_normalize<R: Runtime>(
    app: AppHandle<R>,
    path: String,
) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let input = guarded_recording_path(&app, &path)?;
        let settings = app.state::<crate::settings::SettingsStore>().get();
        let ready = app
            .state::<EncodeState>()
            .ready_ffmpeg()
            .ok_or("normalizing needs the ffmpeg component — install it from Components")?;
        fcap_encode::remux::normalize_loudness(
            &ready,
            &input,
            settings.loudness.target_lufs,
            settings.loudness.ceiling_db,
        )
        .map(|out| out.display().to_string())
    })
    .await
    .map_err(|err| format!("normalize task failed: {err}"))?
}

// --- .frec → MP4/MKV export (decode + re-encode) ---------------------------

/// `<stem>.<ext>` beside `input`, adding ` (1)`, ` (2)`, … until it's free so
/// an export never clobbers an existing file.
fn unique_sibling(input: &std::path::Path, ext: &str) -> std::path::PathBuf {
    let stem = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("export");
    let base = input.with_file_name(format!("{stem}.{ext}"));
    if !base.exists() {
        return base;
    }
    for n in 1..10_000 {
        let candidate = input.with_file_name(format!("{stem} ({n}).{ext}"));
        if !candidate.exists() {
            return candidate;
        }
    }
    base
}

/// Export progress + terminal state, pushed on the `recording-export` event so
/// the Recordings dialog can show a live percentage, a bar, and a cancel.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "state")]
pub enum ExportStatusDto {
    Exporting { frames_done: u64, frames_total: u64 },
    Done { path: String },
    Error { message: String },
    Cancelled,
}

/// Managed state for the single in-flight export (one at a time).
#[derive(Default)]
pub struct ExportState {
    running: AtomicBool,
    cancel: Arc<AtomicBool>,
}

/// Cancel the running export (the partial output is removed by the worker).
#[tauri::command]
pub fn recording_export_cancel(state: State<'_, ExportState>) {
    state.cancel.store(true, Ordering::SeqCst);
}

fn parse_container(container: &str) -> Result<fcap_encode::Container, String> {
    use fcap_encode::Container;
    match container {
        "mp4" => Ok(Container::Mp4),
        "mkv" => Ok(Container::Mkv),
        "mov" => Ok(Container::Mov),
        "webm" => Ok(Container::Webm),
        other => Err(format!("unsupported export container: {other}")),
    }
}

/// Kick off a `.frec` → wire-container export on a worker thread: resolve the
/// ffmpeg component + encoder, plan the encode to a sibling output that never
/// clobbers, and pump progress on the `recording-export` event. Shared by the
/// recordings-list export (folder-guarded) and the open-with-`.frec` export
/// (a file the user chose via the OS). Callers validate the input path first.
pub(crate) fn start_frec_export<R: Runtime>(
    app: &AppHandle<R>,
    input: std::path::PathBuf,
    target: fcap_encode::Container,
) -> Result<(), String> {
    use fcap_encode::WirePlan;

    let export = app.state::<ExportState>();
    if export.running.swap(true, Ordering::SeqCst) {
        return Err("an export is already running".to_string());
    }
    export.cancel.store(false, Ordering::SeqCst);
    let cancel = Arc::clone(&export.cancel);

    let settings = app
        .state::<crate::settings::SettingsStore>()
        .get()
        .recording;

    let prep = (|| -> Result<(fcap_encode::Ffmpeg, String), String> {
        let ready = app
            .state::<EncodeState>()
            .ready_ffmpeg()
            .ok_or("exporting needs the ffmpeg component — install it from Components")?;
        let encoder_id = crate::recording::resolve_encoder(app, &settings, target)?;
        Ok((ready, encoder_id))
    })();
    let (ready, encoder_id) = match prep {
        Ok(v) => v,
        Err(err) => {
            export.running.store(false, Ordering::SeqCst);
            return Err(err);
        }
    };

    let output = unique_sibling(&input, target.extension());
    let plan = WirePlan {
        container: target,
        encoder_id,
        rate_control: settings.rate_control,
        preset: settings.preset,
        keyframe_sec: settings.keyframe_sec,
        audio_bitrate_kbps: settings.audio_bitrate_kbps,
        split_minutes: None,
        scale: None,
        path: output,
    };

    let handle = app.clone();
    let spawned = std::thread::Builder::new()
        .name("fcap-frec-export".into())
        .spawn(move || {
            let progress_handle = handle.clone();
            let result = fcap_encode::export_frec(
                &ready,
                &input,
                &plan,
                move |p: fcap_encode::ExportProgress| {
                    let _ = progress_handle.emit(
                        "recording-export",
                        ExportStatusDto::Exporting {
                            frames_done: p.frames_done,
                            frames_total: p.frames_total,
                        },
                    );
                },
                &cancel,
            );
            let status = match result {
                Ok(out) => ExportStatusDto::Done {
                    path: out.display().to_string(),
                },
                Err(err) if err.contains("cancelled") => ExportStatusDto::Cancelled,
                Err(err) => ExportStatusDto::Error { message: err },
            };
            let _ = handle.emit("recording-export", status);
            handle
                .state::<ExportState>()
                .running
                .store(false, Ordering::SeqCst);
        });
    if let Err(err) = spawned {
        app.state::<ExportState>()
            .running
            .store(false, Ordering::SeqCst);
        return Err(format!("could not start the export: {err}"));
    }
    Ok(())
}

fn is_frec(path: &std::path::Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.eq_ignore_ascii_case("frec"))
        .unwrap_or(false)
}

/// Export a `.frec` from the **recordings folder** to a sibling wire file.
/// Progress rides the `recording-export` event. `container` is `"mp4"` |
/// `"mkv"` | `"mov"` | `"webm"`.
#[tauri::command]
pub fn recording_export<R: Runtime>(
    app: AppHandle<R>,
    path: String,
    container: String,
) -> Result<(), String> {
    let target = parse_container(&container)?;
    let input = guarded_recording_path(&app, &path)?;
    if !is_frec(&input) {
        return Err("only .frec recordings need exporting — others already play anywhere".into());
    }
    start_frec_export(&app, input, target)
}

/// Export a `.frec` the user **opened via the OS** (double-click / Open with).
/// The path is confined only to being an existing `.frec` — the user's own
/// file-open is the consent, so it need not live in the recordings folder.
#[tauri::command]
pub fn open_frec_export<R: Runtime>(
    app: AppHandle<R>,
    path: String,
    container: String,
) -> Result<(), String> {
    let target = parse_container(&container)?;
    reject_remote(&path)?;
    let input = std::path::Path::new(&path)
        .canonicalize()
        .map_err(|err| format!("file not found: {err}"))?;
    if !input.is_file() || !is_frec(&input) {
        return Err("not a .frec file".into());
    }
    start_frec_export(&app, input, target)
}

// --- CAP-N46: the recording integrity verifier ------------------------------

/// Verify a recording's integrity on demand — the owned deep walk for
/// `.frec`, the honest ffmpeg checks for wire files (`deep` scans the whole
/// file; otherwise the last 10 s). Folder-guarded like every recordings
/// action. Returns the typed report for the dialog to render.
#[tauri::command]
pub async fn recording_verify<R: Runtime>(
    app: AppHandle<R>,
    path: String,
    deep: bool,
) -> Result<fcap_encode::verify::VerifyReport, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let input = guarded_recording_path(&app, &path)?;
        if is_frec(&input) {
            fcap_encode::verify::verify_frec(&input, None)
        } else {
            let ready = app.state::<EncodeState>().ready_ffmpeg().ok_or(
                "verifying wire files needs the ffmpeg component — install it from \
                        Components (.frec verifies with nothing installed)",
            )?;
            fcap_encode::verify::verify_wire(&ready, &input, None, deep)
        }
    })
    .await
    .map_err(|err| format!("verify task failed: {err}"))?
}

// --- CAP-N41: the replay & clip trimmer -------------------------------------

/// The trim window's probe payload (mirrors `fcap_encode::trim::TrimInfo`).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrimInfoDto {
    pub duration_secs: f64,
    pub fps: f64,
    pub width: u32,
    pub height: u32,
    pub has_audio: bool,
    pub keyframes_secs: Vec<f64>,
}

/// The shared recordings-folder guard for every recordings-list action
/// (remux / normalize / export / verify / trim / alpha-export): reject remote
/// paths BEFORE any stat, canonicalize, and require the file to live in the
/// recordings folder — the webview never points these at arbitrary files.
fn guarded_recording_path<R: Runtime>(
    app: &AppHandle<R>,
    path: &str,
) -> Result<std::path::PathBuf, String> {
    let settings = app
        .state::<crate::settings::SettingsStore>()
        .get()
        .recording;
    let folder = crate::recording::recordings_folder(&settings)
        .canonicalize()
        .map_err(|err| format!("recordings folder: {err}"))?;
    reject_remote(path)?;
    let input = std::path::Path::new(path)
        .canonicalize()
        .map_err(|err| format!("recording not found: {err}"))?;
    if input.parent() != Some(folder.as_path()) {
        return Err("only files in the recordings folder are accepted".to_string());
    }
    Ok(input)
}

/// The wire container a file's extension implies. The trimmer works on wire
/// files; a `.frec` exports first through the existing path — said honestly.
fn container_of(input: &std::path::Path) -> Result<fcap_encode::Container, String> {
    let ext = input
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    match ext.as_str() {
        "mp4" | "mkv" | "mov" | "webm" => parse_container(&ext),
        "frec" => Err("export the .frec to MP4/MKV first — the trimmer works on wire files".into()),
        other => Err(format!("cannot trim .{other} files")),
    }
}

/// Probe a recording for the trim window: duration, fps, geometry, and the
/// keyframe times that drive the "no re-encode" badge.
#[tauri::command]
pub async fn recording_trim_info<R: Runtime>(
    app: AppHandle<R>,
    path: String,
) -> Result<TrimInfoDto, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let input = guarded_recording_path(&app, &path)?;
        container_of(&input)?;
        let ready = app
            .state::<EncodeState>()
            .ready_ffmpeg()
            .ok_or("trimming needs the ffmpeg component — install it from Components")?;
        let mut info = fcap_encode::trim::trim_info(&ready, &input)?;
        // Bound the IPC payload — far above any sane keyframe count.
        info.keyframes_secs.truncate(100_000);
        Ok(TrimInfoDto {
            duration_secs: info.duration_secs,
            fps: info.fps,
            width: info.width,
            height: info.height,
            has_audio: info.has_audio,
            keyframes_secs: info.keyframes_secs,
        })
    })
    .await
    .map_err(|err| format!("trim probe task failed: {err}"))?
}

/// One preview frame at `at_secs`, as a bounded JPEG data URL.
#[tauri::command]
pub async fn recording_trim_preview<R: Runtime>(
    app: AppHandle<R>,
    path: String,
    at_secs: f64,
) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let input = guarded_recording_path(&app, &path)?;
        container_of(&input)?;
        let ready = app
            .state::<EncodeState>()
            .ready_ffmpeg()
            .ok_or("trimming needs the ffmpeg component — install it from Components")?;
        let jpeg = fcap_encode::trim::trim_preview_jpeg(&ready, &input, at_secs)?;
        Ok(format!(
            "data:image/jpeg;base64,{}",
            crate::commands::base64_encode(&jpeg)
        ))
    })
    .await
    .map_err(|err| format!("trim preview task failed: {err}"))?
}

/// CAP-N42: export an alpha `.frec` to an alpha-preserving `.mov` master —
/// `codec` is `"prores4444"` or `"qtrle"`. Folder-guarded like every other
/// recordings action; progress rides the `recording-export` event.
#[tauri::command]
pub fn recording_export_alpha<R: Runtime>(
    app: AppHandle<R>,
    path: String,
    codec: String,
) -> Result<(), String> {
    use fcap_encode::mux::AlphaCodec;

    let alpha_codec = match codec.as_str() {
        "prores4444" => AlphaCodec::Prores4444,
        "qtrle" => AlphaCodec::Qtrle,
        other => return Err(format!("unsupported alpha codec: {other}")),
    };
    let input = guarded_recording_path(&app, &path)?;
    if !is_frec(&input) {
        return Err("alpha export reads the owned .frec format".to_string());
    }

    let export = app.state::<ExportState>();
    if export.running.swap(true, Ordering::SeqCst) {
        return Err("an export is already running".to_string());
    }
    export.cancel.store(false, Ordering::SeqCst);
    let cancel = Arc::clone(&export.cancel);

    let ready = match app
        .state::<EncodeState>()
        .ready_ffmpeg()
        .ok_or("alpha export needs the ffmpeg component — install it from Components")
    {
        Ok(ready) => ready,
        Err(err) => {
            export.running.store(false, Ordering::SeqCst);
            return Err(err.to_string());
        }
    };
    let suffix = match alpha_codec {
        AlphaCodec::Prores4444 => " ProRes4444",
        AlphaCodec::Qtrle => " QTRLE",
    };
    let stem = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("recording");
    let output = unique_sibling(&input.with_file_name(format!("{stem}{suffix}")), "mov");

    let handle = app.clone();
    let spawned = std::thread::Builder::new()
        .name("fcap-alpha-export".into())
        .spawn(move || {
            let progress_handle = handle.clone();
            let result = fcap_encode::export::export_frec_alpha(
                &ready,
                &input,
                &output,
                alpha_codec,
                move |p: fcap_encode::ExportProgress| {
                    let _ = progress_handle.emit(
                        "recording-export",
                        ExportStatusDto::Exporting {
                            frames_done: p.frames_done,
                            frames_total: p.frames_total,
                        },
                    );
                },
                &cancel,
            );
            let status = match result {
                Ok(out) => ExportStatusDto::Done {
                    path: out.display().to_string(),
                },
                Err(err) if err.contains("cancelled") => ExportStatusDto::Cancelled,
                Err(err) => ExportStatusDto::Error { message: err },
            };
            let _ = handle.emit("recording-export", status);
            handle
                .state::<ExportState>()
                .running
                .store(false, Ordering::SeqCst);
        });
    if let Err(err) = spawned {
        app.state::<ExportState>()
            .running
            .store(false, Ordering::SeqCst);
        return Err(format!("could not start the alpha export: {err}"));
    }
    Ok(())
}

/// Export `[inSecs, outSecs)` of a recording to a ` trim` sibling — stream
/// copy when the in-point lands on a keyframe, an honest re-encode otherwise;
/// `reframe916` re-encodes through the vertical-canvas geometry. Progress
/// rides the `recording-export` event (one export at a time, shared cancel).
#[tauri::command]
pub fn recording_trim<R: Runtime>(
    app: AppHandle<R>,
    path: String,
    in_secs: f64,
    out_secs: f64,
    reframe916: bool,
) -> Result<(), String> {
    let input = guarded_recording_path(&app, &path)?;
    let container = container_of(&input)?;
    if !(in_secs >= 0.0 && out_secs > in_secs && in_secs.is_finite() && out_secs.is_finite()) {
        return Err("the out-point must come after the in-point".to_string());
    }

    let export = app.state::<ExportState>();
    if export.running.swap(true, Ordering::SeqCst) {
        return Err("an export is already running".to_string());
    }
    export.cancel.store(false, Ordering::SeqCst);
    let cancel = Arc::clone(&export.cancel);

    let settings = app
        .state::<crate::settings::SettingsStore>()
        .get()
        .recording;
    let prep = (|| -> Result<(fcap_encode::Ffmpeg, String), String> {
        let ready = app
            .state::<EncodeState>()
            .ready_ffmpeg()
            .ok_or("trimming needs the ffmpeg component — install it from Components")?;
        let encoder_id = crate::recording::resolve_encoder(&app, &settings, container)?;
        Ok((ready, encoder_id))
    })();
    let (ready, encoder_id) = match prep {
        Ok(v) => v,
        Err(err) => {
            export.running.store(false, Ordering::SeqCst);
            return Err(err);
        }
    };

    // The 9:16 reframe rides the vertical-canvas geometry when configured.
    let reframe = reframe916.then(|| {
        app.state::<crate::studio::StudioState>()
            .with_collection(|collection| {
                collection
                    .vertical
                    .as_ref()
                    .map(|vertical| (vertical.width, vertical.height))
            })
            .unwrap_or((1080, 1920))
    });
    let suffix = if reframe.is_some() {
        " trim 9x16"
    } else {
        " trim"
    };
    let stem = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("recording");
    let output = unique_sibling(
        &input.with_file_name(format!("{stem}{suffix}")),
        container.extension(),
    );
    let encode = fcap_encode::trim::TrimEncode {
        encoder_id,
        rate_control: settings.rate_control,
        preset: settings.preset,
        keyframe_sec: settings.keyframe_sec,
        audio_bitrate_kbps: settings.audio_bitrate_kbps,
    };

    let handle = app.clone();
    let spawned = std::thread::Builder::new()
        .name("fcap-trim-export".into())
        .spawn(move || {
            let result = (|| -> Result<std::path::PathBuf, String> {
                // Re-probe server-side: the copy-vs-re-encode decision never
                // trusts webview-supplied keyframe data.
                let info = fcap_encode::trim::trim_info(&ready, &input)?;
                if in_secs >= info.duration_secs {
                    return Err("the in-point is past the end of the file".into());
                }
                let spec = fcap_encode::trim::TrimSpec {
                    in_secs,
                    out_secs: out_secs.min(info.duration_secs),
                    reframe,
                };
                let progress_handle = handle.clone();
                let copied = fcap_encode::trim::trim_export(
                    &ready,
                    &input,
                    &output,
                    container,
                    spec,
                    &encode,
                    &info,
                    move |frames_done, frames_total| {
                        let _ = progress_handle.emit(
                            "recording-export",
                            ExportStatusDto::Exporting {
                                frames_done,
                                frames_total,
                            },
                        );
                    },
                    &cancel,
                )?;
                println!(
                    "trim: {} → {} ({})",
                    input.display(),
                    output.display(),
                    if copied { "stream copy" } else { "re-encoded" }
                );
                Ok(output.clone())
            })();
            let status = match result {
                Ok(out) => ExportStatusDto::Done {
                    path: out.display().to_string(),
                },
                Err(err) if err.contains("cancelled") => ExportStatusDto::Cancelled,
                Err(err) => ExportStatusDto::Error { message: err },
            };
            let _ = handle.emit("recording-export", status);
            handle
                .state::<ExportState>()
                .running
                .store(false, Ordering::SeqCst);
        });
    if let Err(err) = spawned {
        app.state::<ExportState>()
            .running
            .store(false, Ordering::SeqCst);
        return Err(format!("could not start the trim export: {err}"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::container_of;
    use crate::commands::base64_encode;

    /// The shared base64 (used by the trim preview data URL): RFC 4648 vectors.
    #[test]
    fn base64_matches_the_rfc_vectors() {
        assert_eq!(base64_encode(b""), "");
        assert_eq!(base64_encode(b"f"), "Zg==");
        assert_eq!(base64_encode(b"fo"), "Zm8=");
        assert_eq!(base64_encode(b"foo"), "Zm9v");
        assert_eq!(base64_encode(b"foob"), "Zm9vYg==");
        assert_eq!(base64_encode(b"fooba"), "Zm9vYmE=");
        assert_eq!(base64_encode(b"foobar"), "Zm9vYmFy");
        assert_eq!(base64_encode(&[0xff, 0xef, 0xbe]), "/+++");
    }

    /// The trimmer works on wire files; .frec and strangers are told honestly.
    #[test]
    fn trim_accepts_wire_containers_only() {
        use std::path::Path;
        assert!(container_of(Path::new("C:/r/a.mkv")).is_ok());
        assert!(container_of(Path::new("C:/r/a.MP4")).is_ok());
        assert!(container_of(Path::new("C:/r/a.frec"))
            .unwrap_err()
            .contains("export the .frec"));
        assert!(container_of(Path::new("C:/r/a.wav")).is_err());
        assert!(container_of(Path::new("C:/r/noext")).is_err());
    }
}
