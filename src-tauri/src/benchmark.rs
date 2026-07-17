//! CAP-N52 encoder benchmark wizard — the app side: run the measured
//! ladder off-thread, stream per-case progress to the UI, and hand back
//! the recommendation. Explicitly distinct from first-run autoconfig
//! (TASK-905, heuristics): this *measures*, and only when the user asks.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, Runtime};

use fcap_encode::EncPreset;

/// One measured rung, as the UI shows it.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BenchResultDto {
    pub encoder_id: String,
    pub encoder_label: String,
    pub hardware: bool,
    pub preset: EncPreset,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub achieved_fps: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headroom: Option<f32>,
    /// The documented gap: this machine offers the family but it failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendationDto {
    pub encoder_id: String,
    pub encoder_label: String,
    pub preset: EncPreset,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub bitrate_kbps: u32,
    pub headroom: f32,
}

/// The `benchmark` event payload / `benchmark_status` result.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BenchProgressDto {
    pub running: bool,
    pub total: u32,
    pub results: Vec<BenchResultDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<RecommendationDto>,
}

#[derive(Default)]
pub struct BenchmarkState {
    running: AtomicBool,
    cancel: AtomicBool,
    progress: Mutex<BenchProgressDto>,
}

impl BenchmarkState {
    fn lock(&self) -> std::sync::MutexGuard<'_, BenchProgressDto> {
        self.progress
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

fn emit_progress<R: Runtime>(app: &AppHandle<R>) {
    let dto = app.state::<BenchmarkState>().lock().clone();
    let _ = app.emit("benchmark", &dto);
}

/// Run the full measured ladder. Refused while anything is on air — a
/// benchmark stealing the encoder mid-show would both skew its own numbers
/// and hurt the show.
#[tauri::command]
pub async fn benchmark_start<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    let state = app.state::<BenchmarkState>();
    if state.running.swap(true, Ordering::SeqCst) {
        return Err("the benchmark is already running".to_string());
    }
    if app.state::<crate::stream::StreamBridgeState>().is_live()
        || app.state::<crate::recording::RecordingState>().is_active()
    {
        state.running.store(false, Ordering::SeqCst);
        return Err(
            "stop the stream/recording first — a benchmark needs the encoders to itself"
                .to_string(),
        );
    }
    state.cancel.store(false, Ordering::SeqCst);

    let ready = match app
        .state::<crate::commands::recording::EncodeState>()
        .ready_ffmpeg()
    {
        Some(ready) => ready,
        None => {
            state.running.store(false, Ordering::SeqCst);
            return Err(
                "the benchmark needs the ffmpeg component — install it from Components".to_string(),
            );
        }
    };
    let catalog = match crate::commands::recording::ensure_catalog(&app) {
        Ok(catalog) => catalog,
        Err(err) => {
            state.running.store(false, Ordering::SeqCst);
            return Err(err);
        }
    };

    tauri::async_runtime::spawn_blocking(move || {
        let state = app.state::<BenchmarkState>();
        let cases = fcap_encode::ladder(&catalog);
        {
            let mut progress = state.lock();
            *progress = BenchProgressDto {
                running: true,
                total: cases.len() as u32,
                results: Vec::new(),
                recommendation: None,
            };
        }
        emit_progress(&app);

        let mut measured: Vec<fcap_encode::BenchResult> = Vec::new();
        for case in cases {
            if state.cancel.load(Ordering::SeqCst) {
                break;
            }
            let outcome = fcap_encode::run_case(&ready, &case);
            let result = fcap_encode::BenchResult {
                case: case.clone(),
                outcome,
            };
            {
                let mut progress = state.lock();
                progress.results.push(BenchResultDto {
                    encoder_id: case.encoder_id.clone(),
                    encoder_label: case.encoder_label.clone(),
                    hardware: case.hardware,
                    preset: case.preset,
                    width: case.width,
                    height: case.height,
                    fps: case.fps,
                    achieved_fps: result.outcome.as_ref().ok().copied(),
                    headroom: result.headroom(),
                    error: result.outcome.as_ref().err().cloned(),
                });
            }
            measured.push(result);
            emit_progress(&app);
        }

        {
            let mut progress = state.lock();
            progress.recommendation =
                fcap_encode::recommend(&measured).map(|rec| RecommendationDto {
                    encoder_id: rec.encoder_id,
                    encoder_label: rec.encoder_label,
                    preset: rec.preset,
                    width: rec.width,
                    height: rec.height,
                    fps: rec.fps,
                    bitrate_kbps: rec.bitrate_kbps,
                    headroom: rec.headroom,
                });
            progress.running = false;
        }
        state.running.store(false, Ordering::SeqCst);
        emit_progress(&app);
    });
    Ok(())
}

/// Ask the running ladder to stop after the current case.
#[tauri::command]
pub fn benchmark_cancel(state: tauri::State<'_, BenchmarkState>) {
    state.cancel.store(true, Ordering::SeqCst);
}

/// The current (or last) progress — for a wizard opened mid-run.
#[tauri::command]
pub fn benchmark_status(state: tauri::State<'_, BenchmarkState>) -> BenchProgressDto {
    state.lock().clone()
}
