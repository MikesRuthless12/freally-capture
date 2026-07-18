//! CAP-N66: dock layout presets.
//!
//! Named snapshots of the studio's view arrangement — "Solo IRL", "Podcast +
//! producer screen" — that the operator saves and switches between. The app's
//! docks are a fixed grid rather than freely draggable panes, so a preset
//! captures the persisted *view* state: the stats-dock visibility and the mixer
//! orientation. Stored as a plain list in `<config_dir>/dock-presets.json`;
//! applying one patches those two fields through the settings store (which
//! preserves every server-owned field) and hands the updated settings back to
//! the UI so the layout repaints at once. Strictly local.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Runtime, State};

use crate::profiles::WorkspaceState;
use crate::settings::{MixerLayout, Settings, SettingsStore};

/// One saved layout.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DockPreset {
    pub name: String,
    pub show_stats_dock: bool,
    pub mixer_layout: MixerLayout,
}

fn presets_path(base: &Path) -> PathBuf {
    base.join("dock-presets.json")
}

fn read_presets(base: &Path) -> Vec<DockPreset> {
    std::fs::read_to_string(presets_path(base))
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_default()
}

fn write_presets(base: &Path, presets: &[DockPreset]) -> Result<(), String> {
    let json = serde_json::to_string_pretty(presets).map_err(|err| err.to_string())?;
    crate::settings::write_atomic(&presets_path(base), &json).map_err(|err| err.to_string())
}

fn clean_name(name: &str) -> Result<String, String> {
    let name = name.trim();
    if name.is_empty() || name.chars().count() > 40 {
        return Err("a preset name needs 1–40 characters".to_string());
    }
    Ok(name.to_string())
}

/// The saved layouts.
#[tauri::command]
pub fn dock_presets_list(state: State<'_, WorkspaceState>) -> Result<Vec<DockPreset>, String> {
    Ok(read_presets(state.base()?))
}

/// Save the CURRENT layout (stats-dock + mixer orientation) under `name`,
/// replacing a same-named preset.
#[tauri::command]
pub fn dock_preset_save<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, WorkspaceState>,
    name: String,
) -> Result<Vec<DockPreset>, String> {
    let name = clean_name(&name)?;
    let base = state.base()?.to_path_buf();
    let settings = app.state::<SettingsStore>().get();
    let preset = DockPreset {
        name: name.clone(),
        show_stats_dock: settings.show_stats_dock,
        mixer_layout: settings.mixer_layout,
    };
    let mut presets = read_presets(&base);
    match presets.iter_mut().find(|p| p.name == name) {
        Some(existing) => *existing = preset,
        None => presets.push(preset),
    }
    write_presets(&base, &presets)?;
    Ok(presets)
}

/// Apply a preset: patch the live settings and return them so the UI adopts the
/// new layout. Server-owned fields are preserved by the store.
#[tauri::command]
pub fn dock_preset_apply<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, WorkspaceState>,
    name: String,
) -> Result<Settings, String> {
    let base = state.base()?.to_path_buf();
    let preset = read_presets(&base)
        .into_iter()
        .find(|p| p.name == name)
        .ok_or_else(|| format!("dock preset {name:?} was not found"))?;
    let store = app.state::<SettingsStore>();
    let mut settings = store.get();
    settings.show_stats_dock = preset.show_stats_dock;
    settings.mixer_layout = preset.mixer_layout;
    store.set(settings.clone()).map_err(|err| err.to_string())?;
    Ok(settings)
}

/// Delete a preset.
#[tauri::command]
pub fn dock_preset_delete(
    state: State<'_, WorkspaceState>,
    name: String,
) -> Result<Vec<DockPreset>, String> {
    let base = state.base()?.to_path_buf();
    let mut presets = read_presets(&base);
    presets.retain(|preset| preset.name != name);
    write_presets(&base, &presets)?;
    Ok(presets)
}
