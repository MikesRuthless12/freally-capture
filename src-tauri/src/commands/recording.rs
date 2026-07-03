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

/// Detect the encoder catalog (GPU enumeration + per-OS rules), verified
/// against the installed ffmpeg when present. Async — detection and the
/// first-run smoke tests must not block the UI.
#[tauri::command]
pub async fn encoders_list<R: Runtime>(app: AppHandle<R>) -> Result<Catalog, String> {
    let cached = app.state::<EncodeState>().lock_catalog().clone();
    if let Some(catalog) = cached {
        return Ok(catalog);
    }
    let handle = app.clone();
    let catalog = tauri::async_runtime::spawn_blocking(move || {
        let mut catalog = Catalog::detect();
        if let Some(ready) = handle.state::<EncodeState>().ready_ffmpeg() {
            if let Err(err) = ffmpeg::verify_catalog(&mut catalog, &ready) {
                eprintln!("encode: could not verify the catalog against ffmpeg: {err}");
            }
        }
        catalog
    })
    .await
    .map_err(|err| format!("encoder detection task failed: {err}"))?;
    *app.state::<EncodeState>().lock_catalog() = Some(catalog.clone());
    Ok(catalog)
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

    std::thread::Builder::new()
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
        })
        .map_err(|err| format!("could not start the fetch thread: {err}"))?;
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
