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
pub mod calibration;
pub mod cef;
pub mod chat;
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

/// Open one of the app's OWN folders in the OS file browser (the menu bar's
/// "Show Recordings" / "Show Settings Folder"). Takes an enum, never a path —
/// the webview cannot name a filesystem location.
#[tauri::command]
pub fn reveal_app_folder(store: State<'_, SettingsStore>, kind: String) -> Result<(), String> {
    let dir = match kind.as_str() {
        "recordings" => crate::recording::recordings_folder(&store.get().recording),
        "settings" => crate::paths::config_dir()
            .ok_or_else(|| "no config directory on this system".to_owned())?,
        _ => return Err("unknown app folder".to_owned()),
    };
    // The recordings folder is user/profile-settable and could be a UNC path;
    // `create_dir_all` + a shell open on it forces an SMB/NTLM handshake that
    // leaks the credential hash (CAP-M16's rule — the same `is_remote` guard
    // the render loop uses). A hostile imported profile must not turn one
    // "Show Recordings" click into a leak. Recording *to* a network share is
    // still fine; the operator opens that folder from their own file manager.
    if crate::commands::studio::is_remote(&dir.to_string_lossy()) {
        return Err("that folder is on a network path — open it from your file manager".to_owned());
    }
    // The recordings folder may not exist until the first recording lands —
    // create it rather than erroring on a fresh install.
    std::fs::create_dir_all(&dir).map_err(|err| format!("could not create the folder: {err}"))?;
    #[cfg(target_os = "windows")]
    let spawned = std::process::Command::new("explorer").arg(&dir).spawn();
    #[cfg(target_os = "macos")]
    let spawned = std::process::Command::new("open").arg(&dir).spawn();
    #[cfg(target_os = "linux")]
    let spawned = std::process::Command::new("xdg-open").arg(&dir).spawn();
    spawned
        .map(|_| ())
        .map_err(|err| format!("could not open the folder: {err}"))
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
pub(crate) fn base64_encode(data: &[u8]) -> String {
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

/// One camera control, as the properties dialog shows it (CAP-M18).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CameraControlDto {
    pub id: String,
    /// The backend's own display name (fallback label for unknown tags).
    pub name: String,
    /// `None` when the backend reports no range (Windows does this for
    /// exposure/focus/zoom): the UI shows a stepper, not a fake slider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i64>,
    pub step: i64,
    pub default: i64,
    pub value: i64,
    pub writable: bool,
}

/// The controls a RUNNING device reports (CAP-M18). Empty while the device
/// isn't streaming (add it to a scene first) or on a backend without control
/// support — the honest per-OS answer, which the UI spells out.
#[tauri::command]
pub fn camera_controls_list(device_id: String) -> Vec<CameraControlDto> {
    fcap_sources::camera_controls::device(&device_id)
        .snapshot()
        .into_iter()
        .map(|control| CameraControlDto {
            id: control.id.to_string(),
            name: control.name,
            min: control.range.map(|(min, _)| min),
            max: control.range.map(|(_, max)| max),
            step: control.step,
            default: control.default,
            value: control.value,
            writable: control.writable,
        })
        .collect()
}

fn validate_camera_ids(device_id: &str, control: Option<&str>) -> Result<(), String> {
    if device_id.is_empty() || device_id.len() > 256 || device_id.chars().any(char::is_control) {
        return Err("invalid device id".to_owned());
    }
    if let Some(control) = control {
        if control.is_empty()
            || control.len() > 32
            || !control.bytes().all(|b| b.is_ascii_alphanumeric())
        {
            return Err("invalid control tag".to_owned());
        }
    }
    Ok(())
}

// -- MIDI control surfaces (CAP-N03) --------------------------------------------

/// The MIDI ports on this machine: `(inputs, outputs)`. Listing opens nothing.
#[tauri::command]
pub fn midi_ports() -> (Vec<String>, Vec<String>) {
    crate::midi::list_ports()
}

/// Arm MIDI-learn: the next pad/knob touched is reported on the
/// `midi-learned` event instead of firing its binding.
#[tauri::command]
pub fn midi_learn(state: tauri::State<'_, crate::midi::MidiState>, on: bool) {
    state.set_learning(on);
}

// -- PTZ camera control (CAP-N08) -----------------------------------------------

/// Find a configured camera by name (the only way a camera is addressed —
/// the app never talks to an address the operator didn't enter).
fn ptz_camera(
    settings: &tauri::State<'_, crate::settings::SettingsStore>,
    name: &str,
) -> Result<crate::ptz::PtzCamera, String> {
    settings
        .get()
        .ptz
        .cameras
        .into_iter()
        .find(|camera| camera.name == name)
        .ok_or_else(|| format!("no PTZ camera named {name}"))
}

/// A rolling VISCA-over-IP sequence number (per process; cameras only need
/// it to be monotonic).
fn ptz_sequence() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static SEQ: AtomicU32 = AtomicU32::new(1);
    SEQ.fetch_add(1, Ordering::Relaxed)
}

/// Drive the head (or stop it).
#[tauri::command]
pub fn ptz_move(
    settings: tauri::State<'_, crate::settings::SettingsStore>,
    camera: String,
    direction: crate::ptz::PtzMove,
    pan_speed: u8,
    tilt_speed: u8,
) -> Result<(), String> {
    let target = ptz_camera(&settings, &camera)?;
    let payload = crate::ptz::visca_move(direction, pan_speed, tilt_speed);
    crate::ptz::send(&target.host, target.port, &payload, ptz_sequence())
}

/// Zoom (positive = tele, negative = wide, 0 = stop).
#[tauri::command]
pub fn ptz_zoom(
    settings: tauri::State<'_, crate::settings::SettingsStore>,
    camera: String,
    speed: i8,
) -> Result<(), String> {
    let target = ptz_camera(&settings, &camera)?;
    let payload = crate::ptz::visca_zoom(speed);
    crate::ptz::send(&target.host, target.port, &payload, ptz_sequence())
}

/// Recall a preset slot.
#[tauri::command]
pub fn ptz_preset_recall(
    settings: tauri::State<'_, crate::settings::SettingsStore>,
    camera: String,
    slot: u8,
) -> Result<(), String> {
    let target = ptz_camera(&settings, &camera)?;
    let payload = crate::ptz::visca_preset_recall(slot);
    crate::ptz::send(&target.host, target.port, &payload, ptz_sequence())
}

/// Store the camera's current position into a preset slot.
#[tauri::command]
pub fn ptz_preset_store(
    settings: tauri::State<'_, crate::settings::SettingsStore>,
    camera: String,
    slot: u8,
) -> Result<(), String> {
    let target = ptz_camera(&settings, &camera)?;
    let payload = crate::ptz::visca_preset_store(slot);
    crate::ptz::send(&target.host, target.port, &payload, ptz_sequence())
}

// -- hotkey layers (CAP-N05) ----------------------------------------------------

/// Switch the active hotkey layer (CAP-N05). Layers are **sticky**: a layer
/// stays active until switched back (a true hold-to-shift layer is not
/// reachable through the OS global-shortcut API — the UI says so).
#[tauri::command]
pub fn hotkey_set_layer(
    state: tauri::State<'_, std::sync::Mutex<crate::chords::ChordState>>,
    layer: u8,
) {
    state
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .set_layer(layer);
}

/// The active hotkey layer (0 = base).
#[tauri::command]
pub fn hotkey_layer(state: tauri::State<'_, std::sync::Mutex<crate::chords::ChordState>>) -> u8 {
    state
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .layer()
}

// -- show rundown (CAP-N09) -----------------------------------------------------

/// Start the rundown at a step (the operator's Start / jump-to).
#[tauri::command]
pub fn rundown_start(
    app: tauri::AppHandle,
    state: tauri::State<'_, crate::rundown::RundownState>,
    index: usize,
) -> Result<(), String> {
    state.start(&app, index)
}

/// Advance to the next step (Next). Stops at the end.
#[tauri::command]
pub fn rundown_advance(
    app: tauri::AppHandle,
    state: tauri::State<'_, crate::rundown::RundownState>,
) -> Result<(), String> {
    state.advance(&app)
}

/// Stop the rundown (the scene stays put — no surprise cuts).
#[tauri::command]
pub fn rundown_stop(app: tauri::AppHandle, state: tauri::State<'_, crate::rundown::RundownState>) {
    state.stop(&app);
}

/// The dock's view: where we are, what's next, and the countdown.
#[tauri::command]
pub fn rundown_status(
    settings: tauri::State<'_, crate::settings::SettingsStore>,
    state: tauri::State<'_, crate::rundown::RundownState>,
) -> crate::rundown::RundownStatus {
    state.status(&settings.get().rundown)
}

// -- automation (CAP-N01 / CAP-N02) --------------------------------------------

/// Run a macro by name (the UI's Run button; hotkeys and the rules engine
/// call the same path). Unknown names are a no-op, logged.
#[tauri::command]
pub fn automation_run_macro(app: tauri::AppHandle, name: String) {
    crate::automation::run_macro_by_name(&app, &name);
}

/// Every studio variable (CAP-N02) — the UI's variables panel reads this.
#[tauri::command]
pub fn automation_variables(
    state: tauri::State<'_, crate::automation::AutomationState>,
) -> std::collections::HashMap<String, String> {
    state.variables()
}

/// Set one studio variable by hand (the UI). Bounded like every other write.
#[tauri::command]
pub fn automation_set_variable(
    state: tauri::State<'_, crate::automation::AutomationState>,
    name: String,
    value: String,
) -> Result<(), String> {
    if name.trim().is_empty() || name.len() > 64 || value.len() > 512 {
        return Err("bad variable name or value".to_owned());
    }
    state.set_variable(&name, &value);
    Ok(())
}

/// Set one display's HDR→SDR tone-map (CAP-N74): persisted like a camera
/// profile AND pushed into the live registry, so the very next captured
/// frame retunes — no session restart.
#[tauri::command]
pub fn hdr_tone_map_set(
    settings: tauri::State<'_, crate::settings::SettingsStore>,
    capture_id: String,
    operator: String,
    paper_white_nits: u32,
) -> Result<(), String> {
    let parsed = fcap_capture::tonemap::ToneMapOperator::from_name(&operator)
        .ok_or_else(|| format!("unknown tone-map operator: {operator}"))?;
    if !(80..=1000).contains(&paper_white_nits) {
        return Err("paper white must be 80–1000 nits".to_owned());
    }
    settings.set_hdr_tone_map(
        &capture_id,
        crate::settings::HdrToneMapSetting {
            operator,
            paper_white_nits,
        },
    );
    fcap_capture::tonemap::set_tone_map(
        &capture_id,
        fcap_capture::tonemap::ToneMapConfig {
            operator: parsed,
            paper_white_nits: paper_white_nits as f32,
        },
    );
    Ok(())
}

/// Set one capture's cursor effects (CAP-N19): persisted like a tone-map AND
/// pushed into the live registry, so the very next frame redraws — no
/// session restart. All-off configs clear the registry entry, so the capture
/// thread goes back to sampling no input at all.
#[tauri::command]
pub fn cursor_fx_set(
    settings: tauri::State<'_, crate::settings::SettingsStore>,
    capture_id: String,
    fx: crate::settings::CursorFxSetting,
) -> Result<(), String> {
    if capture_id.is_empty() || capture_id.len() > 512 || capture_id.chars().any(char::is_control) {
        return Err("invalid capture id".to_owned());
    }
    fx.validate()?;
    let config = fx.to_config();
    settings.set_cursor_fx(&capture_id, fx);
    fcap_capture::cursorfx::set_cursor_fx(&capture_id, config);
    Ok(())
}

/// Set one control on the running device AND save it into the per-device
/// profile, so it reapplies on hotplug/restart (CAP-M18). The capture thread
/// clamps the value to the device's reported range.
#[tauri::command]
pub fn camera_control_set(
    store: State<'_, SettingsStore>,
    device_id: String,
    control: String,
    value: i64,
) -> Result<(), String> {
    validate_camera_ids(&device_id, Some(&control))?;
    fcap_sources::camera_controls::device(&device_id).queue(&control, value);
    store.set_camera_control(&device_id, &control, value);
    Ok(())
}

/// Drop a device's saved profile and push the backend defaults back onto the
/// running device (CAP-M18).
#[tauri::command]
pub fn camera_profile_reset(
    store: State<'_, SettingsStore>,
    device_id: String,
) -> Result<(), String> {
    validate_camera_ids(&device_id, None)?;
    let hub = fcap_sources::camera_controls::device(&device_id);
    for control in hub.snapshot() {
        if control.writable {
            hub.queue(control.id, control.default);
        }
    }
    store.reset_camera_profile(&device_id);
    Ok(())
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
