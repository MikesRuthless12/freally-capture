//! The UI ↔ core command surface.
//!
//! Typed on both sides: `ui/src/api/` mirrors these signatures and payload
//! shapes — keep them in lockstep.

use serde::{Deserialize, Serialize};
use tauri::State;

use crate::settings::{Settings, SettingsStore};
use fcap_capture::SourceKind;
use fcap_sources::video_device;

pub mod audio;
pub mod recording;
pub mod studio;

/// One linked core crate, as reported by [`health`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CrateHealth {
    pub name: &'static str,
    pub version: &'static str,
}

/// The [`health`] report: proves the command bridge and the owned workspace
/// crates are linked and alive.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Health {
    pub app_version: &'static str,
    pub os: &'static str,
    pub core_ok: bool,
    pub crates: Vec<CrateHealth>,
}

/// Bridge liveness probe.
#[tauri::command]
pub fn health() -> Health {
    Health {
        app_version: env!("CARGO_PKG_VERSION"),
        os: std::env::consts::OS,
        core_ok: true,
        crates: vec![
            CrateHealth {
                name: "fcap-capture",
                version: fcap_capture::VERSION,
            },
            CrateHealth {
                name: "fcap-sources",
                version: fcap_sources::VERSION,
            },
            CrateHealth {
                name: "fcap-compositor",
                version: fcap_compositor::VERSION,
            },
            CrateHealth {
                name: "fcap-scene",
                version: fcap_scene::VERSION,
            },
            CrateHealth {
                name: "fcap-audio",
                version: fcap_audio::VERSION,
            },
            CrateHealth {
                name: "fcap-encode",
                version: fcap_encode::VERSION,
            },
            CrateHealth {
                name: "fcap-stream",
                version: fcap_stream::VERSION,
            },
        ],
    }
}

/// Read the current settings.
#[tauri::command]
pub fn settings_get(store: State<'_, SettingsStore>) -> Settings {
    store.get()
}

/// Replace and persist the settings.
#[tauri::command]
pub fn settings_set(store: State<'_, SettingsStore>, settings: Settings) -> Result<(), String> {
    settings.validate()?;
    store
        .set(settings)
        .map_err(|err| format!("could not save settings: {err}"))
}

// ---------------------------------------------------------------------------
// Capture + preview (Phase 1)
// ---------------------------------------------------------------------------

/// One capturable screen/window source, as the UI picker shows it.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptureSourceDto {
    pub id: String,
    /// "display" | "window" | "portal"
    pub kind: &'static str,
    pub label: String,
    pub width: u32,
    pub height: u32,
}

/// Enumerate screen/window capture sources. On Wayland this is exactly one
/// portal entry — the system dialog picks the real source (honest by design).
/// Async: the first call on macOS can block on the permission prompt.
#[tauri::command]
pub async fn capture_list_sources() -> Result<Vec<CaptureSourceDto>, String> {
    tauri::async_runtime::spawn_blocking(|| {
        fcap_capture::list_sources()
            .map(|sources| {
                sources
                    .into_iter()
                    .map(|source| CaptureSourceDto {
                        id: source.id,
                        kind: match source.kind {
                            SourceKind::Display => "display",
                            SourceKind::Window => "window",
                            SourceKind::Portal => "portal",
                        },
                        label: source.label,
                        width: source.width,
                        height: source.height,
                    })
                    .collect()
            })
            .map_err(|err| err.to_string())
    })
    .await
    .map_err(|err| format!("capture listing task failed: {err}"))?
}

/// One webcam / capture card.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoDeviceDto {
    pub id: String,
    pub name: String,
}

/// Enumerate webcams / capture cards.
#[tauri::command]
pub async fn video_devices_list() -> Result<Vec<VideoDeviceDto>, String> {
    tauri::async_runtime::spawn_blocking(|| {
        video_device::list_video_devices()
            .map(|devices| {
                devices
                    .into_iter()
                    .map(|device| VideoDeviceDto {
                        id: device.id,
                        name: device.name,
                    })
                    .collect()
            })
            .map_err(|err| err.to_string())
    })
    .await
    .map_err(|err| format!("device listing task failed: {err}"))?
}

/// A webcam format offer (mirrors `ui/src/api/types.ts` `VideoFormat`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoFormatDto {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub fourcc: String,
}

/// List a device's formats (opens the device briefly).
#[tauri::command]
pub async fn video_device_formats(device_id: String) -> Result<Vec<VideoFormatDto>, String> {
    tauri::async_runtime::spawn_blocking(move || {
        video_device::list_video_formats(&device_id)
            .map(|formats| {
                formats
                    .into_iter()
                    .map(|format| VideoFormatDto {
                        width: format.width,
                        height: format.height,
                        fps: format.fps,
                        fourcc: format.fourcc,
                    })
                    .collect()
            })
            .map_err(|err| err.to_string())
    })
    .await
    .map_err(|err| format!("format listing task failed: {err}"))?
}

// ---------------------------------------------------------------------------
// Native preview surface (the "OBS feel" path)
// ---------------------------------------------------------------------------

/// The UI reports the preview region's on-screen rectangle (physical pixels,
/// relative to the window's client area) + whether it's currently visible.
/// The native child window follows it; off Windows this is a no-op and the
/// UI keeps the JPEG canvas.
#[tauri::command]
pub fn native_preview_set_region(
    state: tauri::State<'_, crate::native_preview::NativePreviewState>,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    visible: bool,
) {
    state.set_region(
        fcap_preview::Bounds {
            x,
            y,
            width,
            height,
        },
        visible,
    );
}

/// Whether the native preview surface is active (Windows + created OK). When
/// true the UI hides its JPEG `<canvas>` — the native window paints the region.
#[tauri::command]
pub fn native_preview_active(
    state: tauri::State<'_, crate::native_preview::NativePreviewState>,
) -> bool {
    state.surface_handle().is_some()
}

/// macOS: deep-link the user to the Privacy pane they need after a denial.
/// `pane` is "screenRecording" or "camera".
#[tauri::command]
pub fn open_privacy_settings(pane: String) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        let anchor = match pane.as_str() {
            "camera" => "Privacy_Camera",
            _ => "Privacy_ScreenCapture",
        };
        std::process::Command::new("open")
            .arg(format!(
                "x-apple.systempreferences:com.apple.preference.security?{anchor}"
            ))
            .spawn()
            .map_err(|err| format!("could not open System Settings: {err}"))?;
        Ok(())
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = pane;
        Err("privacy settings deep-links only exist on macOS".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::health;

    #[test]
    fn health_reports_every_owned_crate() {
        let report = health();
        assert!(report.core_ok);
        assert_eq!(report.app_version, env!("CARGO_PKG_VERSION"));
        assert_eq!(report.crates.len(), 7, "all owned fcap-* crates are linked");
    }
}
