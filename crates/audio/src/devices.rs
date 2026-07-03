//! Audio device enumeration + the honest per-OS desktop-audio story.
//!
//! Device ids are the cpal device names ("" = the OS default) — the only
//! stable, human-meaningful key cpal exposes on every backend.
//!
//! **Desktop audio ("what you hear") per OS — told honestly:**
//! - **Windows**: any *output* device captures via WASAPI loopback. First class.
//! - **Linux**: PipeWire/PulseAudio expose **monitor** capture devices; the
//!   picker lists capture devices and the user picks the monitor (pavucontrol
//!   / qpwgraph can route one when the ALSA bridge hides it).
//! - **macOS**: Core Audio has no public loopback — a **virtual device**
//!   (e.g. BlackHole) is required until a ScreenCaptureKit audio path lands.
//!   The picker filters for known virtual devices and says so plainly.

use cpal::traits::{DeviceTrait, HostTrait};

use crate::AudioError;

/// One selectable audio device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioDeviceInfo {
    /// The device name — doubles as the id ("" means "the OS default").
    pub id: String,
    pub name: String,
    pub is_default: bool,
}

fn list_devices(
    devices: impl Iterator<Item = cpal::Device>,
    default_name: Option<String>,
) -> Vec<AudioDeviceInfo> {
    let mut out = Vec::new();
    for device in devices {
        let Ok(name) = device.name() else {
            continue;
        };
        let is_default = Some(&name) == default_name.as_ref();
        out.push(AudioDeviceInfo {
            id: name.clone(),
            name,
            is_default,
        });
    }
    // The default first, then alphabetical — stable for pickers.
    out.sort_by(|a, b| {
        b.is_default
            .cmp(&a.is_default)
            .then_with(|| a.name.cmp(&b.name))
    });
    out.dedup_by(|a, b| a.name == b.name);
    out
}

/// Capture devices (microphones / line-in / virtual loopbacks).
pub fn list_input_devices() -> Result<Vec<AudioDeviceInfo>, AudioError> {
    let host = cpal::default_host();
    let default_name = host
        .default_input_device()
        .and_then(|device| device.name().ok());
    let devices = host
        .input_devices()
        .map_err(|err| AudioError::Backend(err.to_string()))?;
    Ok(list_devices(devices, default_name))
}

/// Playback devices (the monitor picker; on Windows also the loopback list).
pub fn list_output_devices() -> Result<Vec<AudioDeviceInfo>, AudioError> {
    let host = cpal::default_host();
    let default_name = host
        .default_output_device()
        .and_then(|device| device.name().ok());
    let devices = host
        .output_devices()
        .map_err(|err| AudioError::Backend(err.to_string()))?;
    Ok(list_devices(devices, default_name))
}

/// Known macOS virtual-loopback device name fragments (lowercase).
#[cfg(target_os = "macos")]
const MAC_VIRTUAL_HINTS: &[&str] = &["blackhole", "soundflower", "loopback", "vb-cable"];

/// What the **Audio Output Capture** picker should offer, plus the honest
/// per-OS guidance line (always shown where the platform needs explaining).
pub fn list_loopback_devices() -> Result<(Vec<AudioDeviceInfo>, Option<String>), AudioError> {
    #[cfg(target_os = "windows")]
    {
        // Every output device is loopback-capturable via WASAPI.
        Ok((list_output_devices()?, None))
    }
    #[cfg(target_os = "macos")]
    {
        let devices: Vec<AudioDeviceInfo> = list_input_devices()?
            .into_iter()
            .filter(|device| {
                let name = device.name.to_lowercase();
                MAC_VIRTUAL_HINTS.iter().any(|hint| name.contains(hint))
            })
            .collect();
        let guidance = if devices.is_empty() {
            "macOS has no public system-audio capture. Install a virtual audio device \
             (e.g. the free BlackHole), set it as a multi-output, and pick it here — \
             a ScreenCaptureKit audio path is planned."
        } else {
            "macOS system audio runs through the virtual device you pick here \
             (route your output to it, e.g. via a multi-output device)."
        };
        Ok((devices, Some(guidance.to_string())))
    }
    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    {
        Ok((
            list_input_devices()?,
            Some(
                "Pick your system's monitor device (usually \"Monitor of …\"). \
                 PipeWire/PulseAudio expose one for every output; pavucontrol or \
                 qpwgraph can surface it if it's hidden."
                    .to_string(),
            ),
        ))
    }
}

/// Find an input device by id ("" = default).
pub(crate) fn find_input_device(device_id: &str) -> Result<cpal::Device, AudioError> {
    let host = cpal::default_host();
    if device_id.is_empty() {
        return host.default_input_device().ok_or(AudioError::NoDevices);
    }
    let devices = host
        .input_devices()
        .map_err(|err| AudioError::Backend(err.to_string()))?;
    for device in devices {
        if device.name().is_ok_and(|name| name == device_id) {
            return Ok(device);
        }
    }
    Err(AudioError::DeviceNotFound(device_id.to_string()))
}

/// Find an output device by id ("" = default).
pub(crate) fn find_output_device(device_id: &str) -> Result<cpal::Device, AudioError> {
    let host = cpal::default_host();
    if device_id.is_empty() {
        return host.default_output_device().ok_or(AudioError::NoDevices);
    }
    let devices = host
        .output_devices()
        .map_err(|err| AudioError::Backend(err.to_string()))?;
    for device in devices {
        if device.name().is_ok_and(|name| name == device_id) {
            return Ok(device);
        }
    }
    Err(AudioError::DeviceNotFound(device_id.to_string()))
}
