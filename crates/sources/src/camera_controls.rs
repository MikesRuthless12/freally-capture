//! CAP-M18 — **Camera controls (UVC)**: the rendezvous between the app's
//! command surface and the capture thread that owns the camera handle
//! (nokhwa handles aren't `Send`, so nobody else may touch them).
//!
//! Shape mirrors the audio `media_hub`: one weak-registered entry per
//! device id. The capture thread publishes the controls the backend
//! actually reports (UVC via MSMF on Windows, V4L2 on Linux, whatever
//! AVFoundation exposes on macOS — an empty list is the honest answer on
//! backends without control support) and drains queued writes between
//! frames. Values clamp to the reported range and snap to the step, so a
//! stale profile can never ask a device for an impossible value.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock, Weak};

use nokhwa::utils::{
    CameraControl, ControlValueDescription, ControlValueSetter, KnownCameraControl,
    KnownCameraControlFlag,
};
use nokhwa::Camera;

/// One reported control, engine-agnostic.
#[derive(Debug, Clone)]
pub struct ControlInfo {
    /// The stable tag ("exposure", "whiteBalance", …) — profile key + wire id.
    pub id: &'static str,
    /// The backend's own display name.
    pub name: String,
    /// The bounds the backend reported. `None` when it reports a value but no
    /// range (Windows does this for the CameraControl-class controls —
    /// exposure/focus/zoom/iris/pan/tilt). We do NOT invent a range: the UI
    /// shows a numeric stepper instead of a meaningless ±4.6e18 slider, and
    /// the device itself rejects a value it can't take (logged, never fatal).
    pub range: Option<(i64, i64)>,
    pub step: i64,
    pub default: i64,
    pub value: i64,
    pub writable: bool,
}

/// One device's live control state, shared app ↔ capture thread.
///
/// The hub is shared per device id, so two sessions can target the same
/// camera (the same webcam added twice; an auto-recover retry racing the
/// working session). Publishing stamps the hub with the publisher's session
/// token, and only THAT session may retire it — otherwise a failing second
/// open would wipe the live session's controls and drop its queued profile.
#[derive(Default)]
pub struct DeviceControls {
    info: Mutex<Vec<ControlInfo>>,
    pending: Mutex<Vec<(String, i64)>>,
    /// The session whose controls `info` currently describes (0 = none).
    owner: Mutex<u64>,
}

fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

impl DeviceControls {
    /// The controls the running capture last reported (empty = device not
    /// streaming, or a backend without control support — the UI says which).
    pub fn snapshot(&self) -> Vec<ControlInfo> {
        lock(&self.info).clone()
    }

    /// Queue one write; the capture thread applies it between frames.
    pub fn queue(&self, id: &str, value: i64) {
        lock(&self.pending).push((id.to_string(), value));
    }

    fn take_pending(&self) -> Vec<(String, i64)> {
        std::mem::take(&mut lock(&self.pending))
    }

    fn publish(&self, info: Vec<ControlInfo>) {
        *lock(&self.info) = info;
    }

    fn clear(&self) {
        lock(&self.info).clear();
        lock(&self.pending).clear();
    }
}

/// A fresh capture-session token (see [`DeviceControls`]).
pub fn session_token() -> u64 {
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

fn registry() -> &'static Mutex<HashMap<String, Weak<DeviceControls>>> {
    static REG: OnceLock<Mutex<HashMap<String, Weak<DeviceControls>>>> = OnceLock::new();
    REG.get_or_init(|| Mutex::new(HashMap::new()))
}

/// The control hub for a device id — the live one while either side holds
/// it, else a fresh empty one (media-hub semantics).
pub fn device(device_id: &str) -> Arc<DeviceControls> {
    let mut map = lock(registry());
    map.retain(|_, weak| weak.strong_count() > 0);
    if let Some(existing) = map.get(device_id).and_then(Weak::upgrade) {
        return existing;
    }
    let fresh = Arc::new(DeviceControls::default());
    map.insert(device_id.to_string(), Arc::downgrade(&fresh));
    fresh
}

/// The stable tag for a control (None = a backend-specific `Other` GUID we
/// don't surface).
fn control_tag(control: KnownCameraControl) -> Option<&'static str> {
    Some(match control {
        KnownCameraControl::Brightness => "brightness",
        KnownCameraControl::Contrast => "contrast",
        KnownCameraControl::Hue => "hue",
        KnownCameraControl::Saturation => "saturation",
        KnownCameraControl::Sharpness => "sharpness",
        KnownCameraControl::Gamma => "gamma",
        KnownCameraControl::WhiteBalance => "whiteBalance",
        KnownCameraControl::BacklightComp => "backlightComp",
        KnownCameraControl::Gain => "gain",
        KnownCameraControl::Pan => "pan",
        KnownCameraControl::Tilt => "tilt",
        KnownCameraControl::Zoom => "zoom",
        KnownCameraControl::Exposure => "exposure",
        KnownCameraControl::Iris => "iris",
        KnownCameraControl::Focus => "focus",
        KnownCameraControl::Other(_) => return None,
    })
}

fn tag_control(tag: &str) -> Option<KnownCameraControl> {
    Some(match tag {
        "brightness" => KnownCameraControl::Brightness,
        "contrast" => KnownCameraControl::Contrast,
        "hue" => KnownCameraControl::Hue,
        "saturation" => KnownCameraControl::Saturation,
        "sharpness" => KnownCameraControl::Sharpness,
        "gamma" => KnownCameraControl::Gamma,
        "whiteBalance" => KnownCameraControl::WhiteBalance,
        "backlightComp" => KnownCameraControl::BacklightComp,
        "gain" => KnownCameraControl::Gain,
        "pan" => KnownCameraControl::Pan,
        "tilt" => KnownCameraControl::Tilt,
        "zoom" => KnownCameraControl::Zoom,
        "exposure" => KnownCameraControl::Exposure,
        "iris" => KnownCameraControl::Iris,
        "focus" => KnownCameraControl::Focus,
        _ => return None,
    })
}

/// Clamp to the reported range (when there is one) and snap onto the step
/// grid, so a stale profile or a racing slider can never ask for a value the
/// device's own bounds exclude. A rangeless control passes through — the
/// driver is then the only authority, and it refuses what it can't take.
pub(crate) fn conform(value: i64, info: &ControlInfo) -> i64 {
    let Some((min, max)) = info.range else {
        return value;
    };
    let clamped = value.clamp(min, max);
    if info.step > 1 {
        let steps = (clamped - min) / info.step;
        (min + steps * info.step).min(max)
    } else {
        clamped
    }
}

fn describe(control: &CameraControl) -> Option<ControlInfo> {
    let id = control_tag(control.control())?;
    let (range, value, step, default) = match control.description() {
        ControlValueDescription::IntegerRange {
            min,
            max,
            value,
            step,
            default,
        } => (Some((*min, *max)), *value, *step, *default),
        // Reported without bounds (the Windows CameraControl class): keep the
        // control — exposure/focus/zoom live here — but say honestly that we
        // don't know its range rather than fabricating one.
        ControlValueDescription::Integer {
            value,
            default,
            step,
        } => (None, *value, *step, *default),
        _ => return None, // only integer controls are surfaced in v1
    };
    let writable = !control.flag().iter().any(|flag| {
        matches!(
            flag,
            KnownCameraControlFlag::ReadOnly | KnownCameraControlFlag::Disabled
        )
    });
    Some(ControlInfo {
        id,
        name: control.name().to_string(),
        range,
        step: step.max(1),
        default,
        value,
        writable,
    })
}

/// Capture-thread side: enumerate + publish what the backend reports, then
/// queue the saved profile so it reapplies on every (re)open — hotplug and
/// auto-recover restarts included.
pub(crate) fn publish_controls(
    hub: &DeviceControls,
    camera: &Camera,
    profile: &[(String, i64)],
    token: u64,
) {
    let info: Vec<ControlInfo> = camera
        .camera_controls()
        .map(|all| all.iter().filter_map(describe).collect())
        .unwrap_or_default();
    *lock(&hub.owner) = token;
    hub.publish(info);
    for (tag, value) in profile {
        hub.queue(tag, *value);
    }
}

/// Capture-thread side: apply everything queued since last frame. Failures
/// are logged per control, honestly, and never kill the stream.
pub(crate) fn apply_pending(hub: &DeviceControls, camera: &mut Camera) {
    let pending = hub.take_pending();
    if pending.is_empty() {
        return;
    }
    let mut info = hub.snapshot();
    for (tag, value) in pending {
        let Some(entry) = info.iter_mut().find(|entry| entry.id == tag) else {
            continue; // the device doesn't report this control — quiet no-op
        };
        if !entry.writable {
            continue;
        }
        let Some(control) = tag_control(&tag) else {
            continue;
        };
        let conformed = conform(value, entry);
        match camera.set_camera_control(control, ControlValueSetter::Integer(conformed)) {
            Ok(()) => entry.value = conformed,
            Err(err) => tracing::warn!("camera control {tag} rejected: {err}"),
        }
    }
    hub.publish(info);
}

/// Capture-thread side: this session ended — an empty list is the honest
/// state. A session that never published (a failed open) or whose hub a NEWER
/// session has since claimed retires nothing: wiping the live session's
/// controls (and its queued profile) is exactly the bug this guards.
pub(crate) fn retire(hub: &DeviceControls, token: u64) {
    let mut owner = lock(&hub.owner);
    if *owner != token {
        return;
    }
    *owner = 0;
    drop(owner);
    hub.clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn info(min: i64, max: i64, step: i64) -> ControlInfo {
        ControlInfo {
            id: "zoom",
            name: "Zoom".into(),
            range: Some((min, max)),
            step,
            default: min,
            value: min,
            writable: true,
        }
    }

    #[test]
    fn values_clamp_and_snap_to_the_step_grid() {
        let zoom = info(100, 400, 10);
        assert_eq!(conform(250, &zoom), 250);
        assert_eq!(conform(256, &zoom), 250, "snaps down onto the grid");
        assert_eq!(conform(-50, &zoom), 100, "clamps below");
        assert_eq!(conform(9_999, &zoom), 400, "clamps above");
        let unit = info(-10, 10, 1);
        assert_eq!(conform(7, &unit), 7, "step 1 passes through");
    }

    #[test]
    fn a_rangeless_control_is_not_given_an_invented_range() {
        // Windows reports exposure/focus/zoom with a value but no bounds.
        // Fabricating i64::MIN/2..i64::MAX/2 produced a slider spanning
        // ±4.6e18 whose every nudge persisted garbage past JS's 2^53.
        let exposure = ControlInfo {
            id: "exposure",
            name: "Exposure".into(),
            range: None,
            step: 1,
            default: -6,
            value: -6,
            writable: true,
        };
        assert_eq!(conform(-8, &exposure), -8, "the driver is the authority");
        assert_eq!(conform(i64::MAX, &exposure), i64::MAX, "no invented clamp");
    }

    #[test]
    fn only_the_publishing_session_retires_a_shared_hub() {
        // Two sessions can target one device id (the same webcam added twice;
        // an auto-recover retry). A failing/older session must never wipe the
        // live one's controls or drop its queued profile.
        let hub = device("cam-test-retire");
        let live = session_token();
        let doomed = session_token();

        *lock(&hub.owner) = live; // the live session published
        hub.publish(vec![info(0, 10, 1)]);
        hub.queue("zoom", 5);

        retire(&hub, doomed); // the second open failed and wound down
        assert_eq!(hub.snapshot().len(), 1, "the live session survives");
        assert_eq!(hub.take_pending().len(), 1, "its queued profile survives");

        retire(&hub, live); // now the live session really ends
        assert!(hub.snapshot().is_empty());
    }

    #[test]
    fn the_registry_rendezvous_is_per_device() {
        let a1 = device("cam-test-a");
        let a2 = device("cam-test-a");
        assert!(Arc::ptr_eq(&a1, &a2), "same id → same hub while alive");
        a1.queue("zoom", 5);
        assert_eq!(a2.take_pending(), vec![("zoom".to_string(), 5)]);

        drop(a1);
        drop(a2);
        let fresh = device("cam-test-a");
        assert!(fresh.snapshot().is_empty(), "all-dropped → a fresh hub");
    }

    #[test]
    fn every_tag_round_trips() {
        for control in nokhwa::utils::all_known_camera_controls() {
            if let Some(tag) = control_tag(control) {
                assert_eq!(tag_control(tag), Some(control), "{tag}");
            }
        }
    }
}
