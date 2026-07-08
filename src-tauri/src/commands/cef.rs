//! The optional **CEF (browser-source runtime)** component commands — the same
//! shape as the ffmpeg component: status, on-demand install with live progress
//! on the `cef` event, cancel, remove. Never fetches on its own; the runtime is
//! verified against the CDN's published SHA-1 before anything is unpacked.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, Runtime, State};

use fcap_encode::cef::{self, FetchPhase, FetchProgress};

/// The `cef` event payload + `cef_status` result (a `state`-tagged union,
/// mirrored in `ui/src/api/types.ts`).
#[derive(Debug, Clone, Serialize)]
#[serde(
    rename_all = "camelCase",
    tag = "state",
    rename_all_fields = "camelCase"
)]
pub enum CefStatusDto {
    /// Not installed. `supported` is false where CEF publishes no build for this
    /// platform (said honestly; the install button stays disabled).
    Missing {
        supported: bool,
    },
    /// Fetching the CDN index to resolve the newest stable build.
    Resolving,
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
        supported: bool,
    },
}

/// Tauri-managed CEF component state.
pub struct CefState {
    status: Mutex<CefStatusDto>,
    installing: AtomicBool,
    cancel: Arc<AtomicBool>,
}

impl CefState {
    pub fn new() -> Self {
        let status = match cef::installed() {
            Some(ready) => CefStatusDto::Ready {
                version: ready.cef_version,
                path: ready.runtime_dir.display().to_string(),
            },
            None => CefStatusDto::Missing {
                supported: cef::platform_key().is_some(),
            },
        };
        Self {
            status: Mutex::new(status),
            installing: AtomicBool::new(false),
            cancel: Arc::new(AtomicBool::new(false)),
        }
    }

    fn lock_status(&self) -> std::sync::MutexGuard<'_, CefStatusDto> {
        self.status
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

impl Default for CefState {
    fn default() -> Self {
        Self::new()
    }
}

fn set_and_emit<R: Runtime>(app: &AppHandle<R>, status: CefStatusDto) {
    *app.state::<CefState>().lock_status() = status.clone();
    let _ = app.emit("cef", &status);
}

/// The CEF component's current status.
#[tauri::command]
pub fn cef_status(state: State<'_, CefState>) -> CefStatusDto {
    state.lock_status().clone()
}

/// Start the on-demand install (explicit user action). Resolves the newest
/// stable build from the CDN, downloads it, verifies its SHA-1, extracts it.
/// Progress rides the `cef` event; the command returns immediately.
#[tauri::command]
pub fn cef_install<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    let state = app.state::<CefState>();
    if cef::platform_key().is_none() {
        return Err("CEF publishes no browser-source runtime for this platform".to_string());
    }
    // Already installed → report Ready without touching the network or disk.
    if let Some(ready) = cef::installed() {
        set_and_emit(
            &app,
            CefStatusDto::Ready {
                version: ready.cef_version,
                path: ready.runtime_dir.display().to_string(),
            },
        );
        return Ok(());
    }
    if state.installing.swap(true, Ordering::SeqCst) {
        return Err("an install is already in progress".to_string());
    }
    state.cancel.store(false, Ordering::SeqCst);
    let cancel = Arc::clone(&state.cancel);
    let handle = app.clone();

    let spawned = std::thread::Builder::new()
        .name("fcap-cef-fetch".into())
        .spawn(move || {
            set_and_emit(&handle, CefStatusDto::Resolving);
            let result = cef::resolve_build().and_then(|build| {
                let progress_handle = handle.clone();
                cef::install(
                    &build,
                    move |progress: FetchProgress| {
                        let status = match progress.phase {
                            FetchPhase::Downloading => CefStatusDto::Downloading {
                                received_bytes: progress.received,
                                total_bytes: progress.total,
                                bytes_per_sec: progress.bytes_per_sec,
                            },
                            FetchPhase::Verifying => CefStatusDto::Verifying,
                            FetchPhase::Extracting => CefStatusDto::Extracting,
                        };
                        set_and_emit(&progress_handle, status);
                    },
                    &cancel,
                )
            });
            let supported = cef::platform_key().is_some();
            match result {
                Ok(ready) => set_and_emit(
                    &handle,
                    CefStatusDto::Ready {
                        version: ready.cef_version,
                        path: ready.runtime_dir.display().to_string(),
                    },
                ),
                Err(cef::CefError::Cancelled) => {
                    set_and_emit(&handle, CefStatusDto::Missing { supported })
                }
                Err(err) => set_and_emit(
                    &handle,
                    CefStatusDto::Error {
                        message: err.to_string(),
                        supported,
                    },
                ),
            }
            handle
                .state::<CefState>()
                .installing
                .store(false, Ordering::SeqCst);
        });

    if let Err(err) = spawned {
        state.installing.store(false, Ordering::SeqCst);
        return Err(format!("could not start the fetch thread: {err}"));
    }
    Ok(())
}

/// Cancel an in-flight install (the partial download is removed).
#[tauri::command]
pub fn cef_cancel(state: State<'_, CefState>) {
    state.cancel.store(true, Ordering::SeqCst);
}

/// Remove the installed runtime.
#[tauri::command]
pub fn cef_remove<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    let state = app.state::<CefState>();
    if state.installing.load(Ordering::SeqCst) {
        return Err("an install is in progress".to_string());
    }
    cef::remove().map_err(|err| format!("could not remove the runtime: {err}"))?;
    set_and_emit(
        &app,
        CefStatusDto::Missing {
            supported: cef::platform_key().is_some(),
        },
    );
    Ok(())
}
