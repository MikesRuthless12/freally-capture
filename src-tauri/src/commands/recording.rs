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
            files.push(RecordingFileDto {
                name: entry.file_name().to_string_lossy().to_string(),
                path: path.display().to_string(),
                size_bytes: meta.len(),
                modified_ms,
                ext,
            });
        }
        files.sort_by_key(|file| std::cmp::Reverse(file.modified_ms));
        files.truncate(200);
        Ok(files)
    })
    .await
    .map_err(|err| format!("recordings listing task failed: {err}"))?
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
        let settings = app
            .state::<crate::settings::SettingsStore>()
            .get()
            .recording;
        let folder = crate::recording::recordings_folder(&settings)
            .canonicalize()
            .map_err(|err| format!("recordings folder: {err}"))?;
        let input = std::path::Path::new(&path)
            .canonicalize()
            .map_err(|err| format!("recording not found: {err}"))?;
        if input.parent() != Some(folder.as_path()) {
            return Err("only files in the recordings folder can be remuxed".to_string());
        }
        let ready = app
            .state::<EncodeState>()
            .ready_ffmpeg()
            .ok_or("remuxing needs the ffmpeg component — install it from Components")?;
        fcap_encode::remux::remux_to_mp4(&ready, &input).map(|out| out.display().to_string())
    })
    .await
    .map_err(|err| format!("remux task failed: {err}"))?
}
