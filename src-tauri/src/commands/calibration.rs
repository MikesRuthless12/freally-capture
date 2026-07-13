//! CAP-M20 — the A/V sync calibration workbench's command surface: arm the
//! two probes (video luma in the render loop, audio block peaks in the
//! engine) against ONE shared instant, poll live signal status, and turn the
//! recorded envelopes into a measurement. The dialog owns the guided flow
//! (temp pattern source, projector, apply); these commands own the clocks.

use std::time::Instant;

use serde::Serialize;
use tauri::{AppHandle, Manager, State};

use fcap_scene::SourceId;

use crate::audio::AudioRuntime;
use crate::calibration::{self, CalibrationState};

/// Live progress while measuring — "seen/heard" use the estimator's own
/// swing thresholds, so a green check means the finish will accept it.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalibrationStatus {
    pub video_samples: usize,
    pub audio_samples: usize,
    pub flash_seen: bool,
    pub beep_heard: bool,
}

/// The finish verdict: exactly one of the two is present.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalibrationResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurement: Option<calibration::Measurement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<calibration::CalibrationError>,
}

/// Arm both probes with one shared zero instant.
#[tauri::command]
pub fn calibration_start(
    app: AppHandle,
    state: State<'_, CalibrationState>,
    video_source: SourceId,
    audio_source: SourceId,
) {
    let armed_at = Instant::now();
    state.arm(video_source, armed_at);
    app.state::<AudioRuntime>()
        .engine
        .calibrate(Some((audio_source, armed_at)));
}

/// Disarm both probes (cancel / dialog closed).
#[tauri::command]
pub fn calibration_stop(app: AppHandle, state: State<'_, CalibrationState>) {
    state.disarm();
    app.state::<AudioRuntime>().engine.calibrate(None);
}

/// Live signal feedback for the measuring step.
#[tauri::command]
pub fn calibration_status(app: AppHandle, state: State<'_, CalibrationState>) -> CalibrationStatus {
    let video = state.series();
    let audio = app.state::<AudioRuntime>().engine.calibration_series();
    CalibrationStatus {
        video_samples: video.len(),
        audio_samples: audio.len(),
        flash_seen: calibration::swing(&video) >= calibration::VIDEO_SWING,
        beep_heard: calibration::swing(&audio) >= calibration::AUDIO_SWING,
    }
}

/// Take both series, disarm, and estimate. Errors come back structured so
/// the dialog can render the honest, actionable guidance per kind.
#[tauri::command]
pub fn calibration_finish(app: AppHandle, state: State<'_, CalibrationState>) -> CalibrationResult {
    let video = state.series();
    let audio = app.state::<AudioRuntime>().engine.calibration_series();
    state.disarm();
    app.state::<AudioRuntime>().engine.calibrate(None);
    match calibration::estimate_offset(&video, &audio) {
        Ok(measurement) => CalibrationResult {
            measurement: Some(measurement),
            error: None,
        },
        Err(error) => CalibrationResult {
            measurement: None,
            error: Some(error),
        },
    }
}
