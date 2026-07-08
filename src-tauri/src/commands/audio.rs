//! The audio command surface: device pickers + every per-source mixer
//! mutation (volume, mute, monitor, tracks, sync, hotkeys, and the audio
//! filter chain). Mutations ride the same studio-state path as the video
//! commands — validate, bump the revision, push the fresh model.

use serde::Serialize;
use tauri::{AppHandle, State};
use tauri_plugin_global_shortcut::Shortcut;

use fcap_scene::{AudioFilterId, AudioFilterKind, MonitorMode, SourceId};

use crate::studio::StudioState;

/// One selectable audio device.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioDeviceDto {
    pub id: String,
    pub name: String,
    pub is_default: bool,
}

fn device_dtos(devices: Vec<fcap_audio::AudioDeviceInfo>) -> Vec<AudioDeviceDto> {
    devices
        .into_iter()
        .map(|device| AudioDeviceDto {
            id: device.id,
            name: device.name,
            is_default: device.is_default,
        })
        .collect()
}

/// Capture devices (microphones / line-in).
#[tauri::command]
pub async fn audio_input_devices() -> Result<Vec<AudioDeviceDto>, String> {
    tauri::async_runtime::spawn_blocking(|| {
        fcap_audio::list_input_devices()
            .map(device_dtos)
            .map_err(|err| err.to_string())
    })
    .await
    .map_err(|err| format!("audio device listing task failed: {err}"))?
}

/// Playback devices (the monitor picker).
#[tauri::command]
pub async fn audio_output_devices() -> Result<Vec<AudioDeviceDto>, String> {
    tauri::async_runtime::spawn_blocking(|| {
        fcap_audio::list_output_devices()
            .map(device_dtos)
            .map_err(|err| err.to_string())
    })
    .await
    .map_err(|err| format!("audio device listing task failed: {err}"))?
}

/// One application currently producing audio (the App Audio picker rows).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppAudioDto {
    pub pid: u32,
    pub name: String,
    pub exe: String,
}

/// What the App Audio picker offers + the per-OS honest guidance.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppAudioListDto {
    pub apps: Vec<AppAudioDto>,
    /// Whether single-call per-app capture exists on this OS/build.
    pub supported: bool,
    /// The honest per-OS guidance (shown when unsupported or the list is empty).
    pub guidance: String,
}

/// Apps currently making sound (Windows: WASAPI process loopback). Elsewhere the
/// list is empty and `supported` is false with the honest guidance.
#[tauri::command]
pub async fn app_audio_apps() -> Result<AppAudioListDto, String> {
    tauri::async_runtime::spawn_blocking(|| {
        let guidance = fcap_appaudio::per_app_guidance();
        match fcap_appaudio::list_audio_apps() {
            Ok(apps) => AppAudioListDto {
                apps: apps
                    .into_iter()
                    .map(|a| AppAudioDto {
                        pid: a.pid,
                        name: a.name,
                        exe: a.exe,
                    })
                    .collect(),
                supported: true,
                guidance,
            },
            Err(_) => AppAudioListDto {
                apps: Vec::new(),
                supported: false,
                guidance,
            },
        }
    })
    .await
    .map_err(|err| format!("app audio listing task failed: {err}"))
}

/// What the Audio Output Capture picker offers + the per-OS honest guidance.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoopbackDevicesDto {
    pub devices: Vec<AudioDeviceDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance: Option<String>,
}

/// Desktop-audio capture candidates (Windows: any output via WASAPI
/// loopback; Linux: monitor devices; macOS: virtual devices — with guidance).
#[tauri::command]
pub async fn audio_loopback_devices() -> Result<LoopbackDevicesDto, String> {
    tauri::async_runtime::spawn_blocking(|| {
        fcap_audio::list_loopback_devices()
            .map(|(devices, guidance)| LoopbackDevicesDto {
                devices: device_dtos(devices),
                guidance,
            })
            .map_err(|err| err.to_string())
    })
    .await
    .map_err(|err| format!("audio device listing task failed: {err}"))?
}

// ---------------------------------------------------------------------------
// Per-source mixer state
// ---------------------------------------------------------------------------

#[tauri::command]
pub fn studio_set_audio_volume(
    app: AppHandle,
    state: State<'_, StudioState>,
    source_id: SourceId,
    volume_db: f32,
) -> Result<(), String> {
    state.mutate(&app, |collection| {
        collection.set_audio_volume(source_id, volume_db)
    })
}

#[tauri::command]
pub fn studio_set_audio_muted(
    app: AppHandle,
    state: State<'_, StudioState>,
    source_id: SourceId,
    muted: bool,
) -> Result<(), String> {
    state.mutate(&app, |collection| {
        collection.set_audio_muted(source_id, muted)
    })
}

#[tauri::command]
pub fn studio_set_audio_monitor(
    app: AppHandle,
    state: State<'_, StudioState>,
    source_id: SourceId,
    monitor: MonitorMode,
) -> Result<(), String> {
    state.mutate(&app, |collection| {
        collection.set_audio_monitor(source_id, monitor)
    })
}

#[tauri::command]
pub fn studio_set_audio_tracks(
    app: AppHandle,
    state: State<'_, StudioState>,
    source_id: SourceId,
    tracks: u8,
) -> Result<(), String> {
    state.mutate(&app, |collection| {
        collection.set_audio_tracks(source_id, tracks)
    })
}

#[tauri::command]
pub fn studio_set_audio_sync_offset(
    app: AppHandle,
    state: State<'_, StudioState>,
    source_id: SourceId,
    sync_offset_ms: u32,
) -> Result<(), String> {
    state.mutate(&app, |collection| {
        collection.set_audio_sync_offset(source_id, sync_offset_ms)
    })
}

/// Bind/clear push-to-talk / push-to-mute. Keys must parse as accelerators
/// (e.g. `"Ctrl+Shift+T"`, `"F13"`) so a typo never lands in the model.
#[tauri::command]
pub fn studio_set_audio_hotkeys(
    app: AppHandle,
    state: State<'_, StudioState>,
    source_id: SourceId,
    push_to_talk: Option<String>,
    push_to_mute: Option<String>,
) -> Result<(), String> {
    for key in [&push_to_talk, &push_to_mute].into_iter().flatten() {
        if key.trim().is_empty() {
            continue; // blank clears — the model handles it
        }
        if key.len() > 64 {
            return Err("hotkey is too long".to_string());
        }
        key.parse::<Shortcut>()
            .map_err(|err| format!("not a usable hotkey ({key:?}): {err}"))?;
    }
    state.mutate(&app, |collection| {
        collection.set_audio_hotkeys(source_id, push_to_talk, push_to_mute)
    })
}

// ---------------------------------------------------------------------------
// The audio filter chain
// ---------------------------------------------------------------------------

#[tauri::command]
pub fn studio_add_audio_filter(
    app: AppHandle,
    state: State<'_, StudioState>,
    source_id: SourceId,
    kind: AudioFilterKind,
) -> Result<AudioFilterId, String> {
    state.mutate(&app, |collection| {
        collection.add_audio_filter(source_id, kind)
    })
}

#[tauri::command]
pub fn studio_remove_audio_filter(
    app: AppHandle,
    state: State<'_, StudioState>,
    source_id: SourceId,
    filter_id: AudioFilterId,
) -> Result<(), String> {
    state.mutate(&app, |collection| {
        collection.remove_audio_filter(source_id, filter_id)
    })
}

#[tauri::command]
pub fn studio_reorder_audio_filter(
    app: AppHandle,
    state: State<'_, StudioState>,
    source_id: SourceId,
    filter_id: AudioFilterId,
    to_index: usize,
) -> Result<(), String> {
    state.mutate(&app, |collection| {
        collection.reorder_audio_filter(source_id, filter_id, to_index)
    })
}

#[tauri::command]
pub fn studio_update_audio_filter(
    app: AppHandle,
    state: State<'_, StudioState>,
    source_id: SourceId,
    filter_id: AudioFilterId,
    kind: AudioFilterKind,
) -> Result<(), String> {
    state.mutate(&app, |collection| {
        collection.update_audio_filter(source_id, filter_id, kind)
    })
}

#[tauri::command]
pub fn studio_set_audio_filter_enabled(
    app: AppHandle,
    state: State<'_, StudioState>,
    source_id: SourceId,
    filter_id: AudioFilterId,
    enabled: bool,
) -> Result<(), String> {
    state.mutate(&app, |collection| {
        collection.set_audio_filter_enabled(source_id, filter_id, enabled)
    })
}
