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
pub mod cef;
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
                name: "fcap-appaudio",
                version: fcap_appaudio::VERSION,
            },
            CrateHealth {
                name: "fcap-ndi",
                version: fcap_ndi::VERSION,
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

/// Optional-integration status (TASK-804): NDI (detected user runtime) + VST
/// (scoped, licensing-deferred). Read-only — the UI shows availability + the
/// honest guidance; nothing is bundled.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationsStatus {
    pub ndi_available: bool,
    pub ndi_version: Option<String>,
    pub ndi_guidance: String,
    pub vst_available: bool,
    pub vst_status: String,
}

/// Probe the optional NDI runtime + report the VST scope. NDI detection touches
/// the filesystem + link-probes a library, so it runs off the UI thread.
#[tauri::command]
pub async fn integrations_status() -> IntegrationsStatus {
    tauri::async_runtime::spawn_blocking(|| {
        let ndi = fcap_ndi::detect();
        let vst_status = match fcap_audio::vst::support() {
            fcap_audio::vst::VstSupport::Unavailable(reason) => reason.to_string(),
        };
        IntegrationsStatus {
            ndi_available: ndi.available,
            ndi_version: ndi.version,
            ndi_guidance: ndi.guidance,
            vst_available: fcap_audio::vst::is_available(),
            vst_status,
        }
    })
    .await
    .unwrap_or_else(|_| IntegrationsStatus {
        ndi_available: false,
        ndi_version: None,
        ndi_guidance: fcap_ndi::guidance(),
        vst_available: false,
        vst_status: fcap_audio::vst::VST_STATUS.to_string(),
    })
}

/// Read the current settings.
#[tauri::command]
pub fn settings_get(store: State<'_, SettingsStore>) -> Settings {
    store.get()
}

/// Mark the first-run wizard as seen. Called when the user finishes OR skips it.
/// Goes through the store's dedicated writer rather than a read-modify-write of
/// the whole `Settings`, which every later `settings_set` would clobber.
#[tauri::command]
pub fn settings_complete_onboarding(store: State<'_, SettingsStore>) -> Result<(), String> {
    store
        .complete_onboarding()
        .map_err(|err| format!("could not record onboarding: {err}"))
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

/// Game-capture status (TASK-801): how game capture can work here + the honest
/// anti-cheat/AV risk + the working fallback. Read-only; nothing injects.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameCaptureStatusDto {
    /// "hookPlanned" | "portalOnly" | "windowCaptureOnly"
    pub support: &'static str,
    pub hook_possible: bool,
    pub risk: String,
    /// "windowCapture" | "portal"
    pub fallback: &'static str,
    pub guidance: String,
}

#[tauri::command]
pub fn game_capture_status() -> GameCaptureStatusDto {
    use fcap_capture::game::{GameCaptureFallback, GameCaptureSupport};
    let status = fcap_capture::game::status();
    GameCaptureStatusDto {
        support: match status.support {
            GameCaptureSupport::HookPlanned => "hookPlanned",
            GameCaptureSupport::PortalOnly => "portalOnly",
            GameCaptureSupport::WindowCaptureOnly => "windowCaptureOnly",
        },
        hook_possible: status.hook_possible,
        risk: status.risk,
        fallback: match status.fallback {
            GameCaptureFallback::WindowCapture => "windowCapture",
            GameCaptureFallback::Portal => "portal",
        },
        guidance: status.guidance,
    }
}

/// A one-shot JPEG thumbnail (`data:` URI) of the window `id`, for the picker's
/// live preview (the UI re-requests it on a timer). `Ok(None)` = no thumbnail
/// available — a minimized / GPU-composited window, or a platform without it yet
/// — so the UI shows a placeholder instead of an error.
#[tauri::command]
pub async fn capture_window_thumbnail(
    id: String,
    max_dim: Option<u32>,
) -> Result<Option<String>, String> {
    let max = max_dim.unwrap_or(320).clamp(32, 1024);
    tauri::async_runtime::spawn_blocking(move || {
        fcap_capture::window_thumbnail(&id, max)
            .ok()
            .and_then(|thumb| thumbnail_data_uri(&thumb))
    })
    .await
    .map_err(|err| format!("thumbnail task failed: {err}"))
}

/// Encode an RGBA [`fcap_capture::Thumbnail`] as a `data:image/jpeg` URI.
fn thumbnail_data_uri(thumb: &fcap_capture::Thumbnail) -> Option<String> {
    // Thumbnails are clamped to <=1024px, so both dimensions fit a u16.
    let width = u16::try_from(thumb.width).ok()?;
    let height = u16::try_from(thumb.height).ok()?;
    let mut jpeg = Vec::new();
    jpeg_encoder::Encoder::new(&mut jpeg, 80)
        .encode(&thumb.rgba, width, height, jpeg_encoder::ColorType::Rgba)
        .ok()?;
    Some(format!("data:image/jpeg;base64,{}", base64_encode(&jpeg)))
}

/// Minimal standard base64 (padded) — a few KB per thumbnail, not worth a dep.
fn base64_encode(data: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(data.len() * 4 / 3 + 4);
    for chunk in data.chunks(3) {
        let b1 = chunk.get(1).copied().unwrap_or(0);
        let b2 = chunk.get(2).copied().unwrap_or(0);
        let n = (u32::from(chunk[0]) << 16) | (u32::from(b1) << 8) | u32::from(b2);
        out.push(char::from(ALPHABET[((n >> 18) & 63) as usize]));
        out.push(char::from(ALPHABET[((n >> 12) & 63) as usize]));
        out.push(if chunk.len() > 1 {
            char::from(ALPHABET[((n >> 6) & 63) as usize])
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            char::from(ALPHABET[(n & 63) as usize])
        } else {
            '='
        });
    }
    out
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

/// Whether the native preview surface is **viable** — the compositor is on the
/// DX12 backend, the DirectComposition overlay was created, and no runtime
/// surface failure has knocked it out. Only then does the UI hide its JPEG
/// `<canvas>`; a non-DX12 GPU or a lost surface reports `false` so the UI keeps
/// the JPEG fallback (the UI re-polls this, so a mid-session drop is caught).
#[tauri::command]
pub fn native_preview_active(
    state: tauri::State<'_, crate::native_preview::NativePreviewState>,
) -> bool {
    state.is_viable()
}

/// The UI reports which scene item is selected, so the native preview can draw
/// its selection box + handles *into* the GPU frame (they'd otherwise be hidden
/// under the opaque native surface). `None` clears it. A no-op off the native path.
#[tauri::command]
pub fn native_preview_set_selection(
    state: tauri::State<'_, crate::native_preview::NativePreviewState>,
    item: Option<fcap_scene::ItemId>,
) {
    // Test-only: the smoke run forces a selection so the screenshot shows the
    // selection overlay (box + handles + rotate). Ignore the UI's sync, which
    // reports "nothing selected" on load and would clear it. No effect normally.
    if std::env::var_os("FCAP_SMOKE").is_some() {
        return;
    }
    state.set_selection(item);
}

/// The UI reports the preview alignment overlay (safe areas + smart guides,
/// canvas px) so the native preview can draw it *into* the GPU frame, where the
/// SVG below is occluded (CAP-M04). Counts are clamped in `set_overlay`. A no-op
/// off the native path.
#[tauri::command]
pub fn native_preview_set_overlay(
    state: tauri::State<'_, crate::native_preview::NativePreviewState>,
    overlay: crate::native_preview::AlignmentOverlay,
) {
    state.set_overlay(overlay);
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
        assert_eq!(report.crates.len(), 9, "all owned fcap-* crates are linked");
    }
}
