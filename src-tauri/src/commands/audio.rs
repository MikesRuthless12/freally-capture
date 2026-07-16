//! The audio command surface: device pickers + every per-source mixer
//! mutation (volume, mute, monitor, tracks, sync, hotkeys, and the audio
//! filter chain). Mutations ride the same studio-state path as the video
//! commands — validate, bump the revision, push the fresh model.

use serde::Serialize;
use tauri::{AppHandle, State};
use tauri_plugin_global_shortcut::Shortcut;

use fcap_scene::{AudioFilterId, AudioFilterKind, MonitorMode, SourceId};

use crate::studio::{coalesce_key, StudioState};

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

/// One discovered CLAP/VST3 plugin (CAP-N33).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClapPluginDto {
    pub path: String,
    pub name: String,
    /// "clap" | "vst3".
    pub format: String,
}

/// The CAP-N33 CLAP scan result: installed plugins + the honest host status.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClapScanDto {
    pub plugins: Vec<ClapPluginDto>,
    pub status: String,
}

/// Scan the standard CLAP directories for installed plugins (CAP-N33). Opt-in
/// (called only when the CLAP panel opens); local-directory only, no fetch.
#[tauri::command]
pub async fn audio_clap_plugins() -> Result<ClapScanDto, String> {
    tauri::async_runtime::spawn_blocking(|| ClapScanDto {
        plugins: fcap_audio::claphost::scan_clap_plugins()
            .into_iter()
            .map(|plugin| ClapPluginDto {
                path: plugin.path.display().to_string(),
                name: plugin.name,
                format: plugin.format.to_string(),
            })
            .collect(),
        status: fcap_audio::claphost::CLAP_STATUS.to_string(),
    })
    .await
    .map_err(|err| format!("clap scan task failed: {err}"))
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
    state.mutate_tracked(
        &app,
        "setVolume",
        Some(coalesce_key("volume", source_id)),
        |collection| collection.set_audio_volume(source_id, volume_db),
    )
}

#[tauri::command]
pub fn studio_set_audio_muted(
    app: AppHandle,
    state: State<'_, StudioState>,
    source_id: SourceId,
    muted: bool,
) -> Result<(), String> {
    state.mutate_tracked(&app, "toggleMute", None, |collection| {
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
    state.mutate_tracked(&app, "setMonitor", None, |collection| {
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
    state.mutate_tracked(&app, "setTracks", None, |collection| {
        collection.set_audio_tracks(source_id, tracks)
    })
}

/// Stereo balance (CAP-M19) — drag-coalesced like the fader.
#[tauri::command]
pub fn studio_set_audio_pan(
    app: AppHandle,
    state: State<'_, StudioState>,
    source_id: SourceId,
    pan: f32,
) -> Result<(), String> {
    state.mutate_tracked(
        &app,
        "setPan",
        Some(coalesce_key("pan", source_id)),
        |collection| collection.set_audio_pan(source_id, pan),
    )
}

/// PFL solo (CAP-M19) — monitor routing only; the program mix never changes.
#[tauri::command]
pub fn studio_set_audio_solo(
    app: AppHandle,
    state: State<'_, StudioState>,
    source_id: SourceId,
    solo: bool,
) -> Result<(), String> {
    state.mutate_tracked(&app, "toggleSolo", None, |collection| {
        collection.set_audio_solo(source_id, solo)
    })
}

/// Mono downmix (CAP-M19).
#[tauri::command]
pub fn studio_set_audio_mono(
    app: AppHandle,
    state: State<'_, StudioState>,
    source_id: SourceId,
    mono: bool,
) -> Result<(), String> {
    state.mutate_tracked(&app, "toggleMono", None, |collection| {
        collection.set_audio_mono(source_id, mono)
    })
}

/// Join/leave the gain-sharing auto-mixer (CAP-N32).
#[tauri::command]
pub fn studio_set_audio_automix(
    app: AppHandle,
    state: State<'_, StudioState>,
    source_id: SourceId,
    automix: bool,
) -> Result<(), String> {
    state.mutate_tracked(&app, "toggleAutomix", None, |collection| {
        collection.set_audio_automix(source_id, automix)
    })
}

/// Toggle per-guest mix-minus (CAP-N39): an N−1 return for this source.
#[tauri::command]
pub fn studio_set_audio_mix_minus(
    app: AppHandle,
    state: State<'_, StudioState>,
    source_id: SourceId,
    mix_minus: bool,
) -> Result<(), String> {
    state.mutate_tracked(&app, "toggleMixMinus", None, |collection| {
        collection.set_audio_mix_minus(source_id, mix_minus)
    })
}

/// The filter chain for a CAP-N39 voice-chain preset (gate→comp→EQ→limiter
/// shapes, from owned classic-DSP filters).
fn voice_preset_chain(preset: &str) -> Result<Vec<AudioFilterKind>, String> {
    let gate = AudioFilterKind::NoiseGate {
        open_threshold_db: -26.0,
        close_threshold_db: -32.0,
        attack_ms: 25.0,
        hold_ms: 200.0,
        release_ms: 150.0,
    };
    let comp = AudioFilterKind::Compressor {
        ratio: 3.0,
        threshold_db: -18.0,
        attack_ms: 6.0,
        release_ms: 60.0,
        output_gain_db: 3.0,
    };
    let eq = AudioFilterKind::Eq {
        low_db: 0.0,
        mid_db: 1.0,
        high_db: 2.0,
    };
    let limiter = AudioFilterKind::Limiter {
        threshold_db: -1.0,
        release_ms: 60.0,
    };
    let rumble = AudioFilterKind::RumbleGuard { freq_hz: 90.0 };
    let deesser = AudioFilterKind::DeEsser {
        freq_hz: 6_500.0,
        threshold_db: -30.0,
        amount_db: 8.0,
    };
    Ok(match preset {
        "broadcast" => vec![gate, comp, eq, limiter],
        "podcast" => vec![rumble, gate, comp, deesser, limiter],
        "clean" => vec![rumble, comp, limiter],
        "none" => vec![],
        other => return Err(format!("unknown voice preset: {other}")),
    })
}

/// CAP-N39: apply a one-click voice-chain preset to a source (replacing its
/// filter chain).
#[tauri::command]
pub fn studio_apply_voice_preset(
    app: AppHandle,
    state: State<'_, StudioState>,
    source_id: SourceId,
    preset: String,
) -> Result<(), String> {
    let kinds = voice_preset_chain(&preset)?;
    state.mutate_tracked(&app, "voicePreset", None, |collection| {
        collection.set_audio_filters(source_id, kinds)
    })
}

/// Arm (or clear with `null`) the CAP-N35 spectrum tap on a source — the strip
/// whose parametric-EQ editor is open. Purely a view concern; no model change.
#[tauri::command]
pub fn audio_arm_spectrum(
    audio: State<'_, crate::audio::AudioRuntime>,
    source_id: Option<SourceId>,
) -> Result<(), String> {
    audio.engine.set_spectrum_target(source_id);
    Ok(())
}

/// Arm (or clear with `null`) per-filter live metering on a source — the strip
/// whose filter editor is open. A view concern; no model change.
#[tauri::command]
pub fn audio_arm_filter_meters(
    audio: State<'_, crate::audio::AudioRuntime>,
    source_id: Option<SourceId>,
) -> Result<(), String> {
    audio.engine.set_meter_target(source_id);
    Ok(())
}

#[tauri::command]
pub fn studio_set_audio_sync_offset(
    app: AppHandle,
    state: State<'_, StudioState>,
    source_id: SourceId,
    sync_offset_ms: u32,
) -> Result<(), String> {
    state.mutate_tracked(
        &app,
        "setSyncOffset",
        Some(coalesce_key("syncOffset", source_id)),
        |collection| collection.set_audio_sync_offset(source_id, sync_offset_ms),
    )
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
    state.mutate_tracked(&app, "setAudioHotkeys", None, |collection| {
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
    state.mutate_tracked(&app, "addFilter", None, |collection| {
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
    state.mutate_tracked(&app, "removeFilter", None, |collection| {
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
    state.mutate_tracked(&app, "reorderFilter", None, |collection| {
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
    state.mutate_tracked(
        &app,
        "editFilter",
        Some(coalesce_key("audioFilterParams", filter_id)),
        |collection| collection.update_audio_filter(source_id, filter_id, kind),
    )
}

#[tauri::command]
pub fn studio_set_audio_filter_enabled(
    app: AppHandle,
    state: State<'_, StudioState>,
    source_id: SourceId,
    filter_id: AudioFilterId,
    enabled: bool,
) -> Result<(), String> {
    state.mutate_tracked(&app, "toggleFilter", None, |collection| {
        collection.set_audio_filter_enabled(source_id, filter_id, enabled)
    })
}

// ---------------------------------------------------------------------------
// Soundboard (CAP-N37)
// ---------------------------------------------------------------------------

/// Fire a soundboard pad (decode its clip into the mixer via the ffmpeg
/// component; choke group handled engine-side).
#[tauri::command]
pub fn soundboard_trigger(
    settings: State<'_, crate::settings::SettingsStore>,
    encode: State<'_, crate::commands::recording::EncodeState>,
    board: State<'_, crate::soundboard::SoundboardState>,
    id: String,
) -> Result<(), String> {
    let pad = settings
        .soundboard_pad(&id)
        .ok_or("no such soundboard pad")?;
    if pad.path.trim().is_empty() {
        return Err("this pad has no clip assigned yet".to_owned());
    }
    let ready = encode
        .ready_ffmpeg()
        .ok_or("soundboard playback needs the ffmpeg component — install it from Components")?;
    board.trigger(
        &ready,
        pad.id,
        std::path::PathBuf::from(&pad.path),
        pad.looping,
        pad.gain_db,
        pad.tracks,
        pad.choke_group,
        pad.auto_duck,
    )
}

/// Stop one sounding pad.
#[tauri::command]
pub fn soundboard_stop(board: State<'_, crate::soundboard::SoundboardState>, id: String) {
    board.stop(&id);
}

/// Stop every sounding pad.
#[tauri::command]
pub fn soundboard_stop_all(board: State<'_, crate::soundboard::SoundboardState>) {
    board.stop_all();
}
