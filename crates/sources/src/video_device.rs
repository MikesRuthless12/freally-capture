//! The Video Capture Device source: webcams and capture cards via `nokhwa`
//! (MSMF on Windows, AVFoundation on macOS, V4L2 on Linux).
//!
//! Frames are decoded to RGBA and published through the same
//! latest-wins session shape as screen capture, so the preview/compositor
//! side never cares where pixels came from. On macOS the camera permission
//! is requested up front and a denial surfaces as
//! [`CaptureError::PermissionDenied`] — never a silent black frame.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

use nokhwa::pixel_format::RgbAFormat;
use nokhwa::utils::{
    ApiBackend, CameraFormat, CameraIndex, FrameFormat, RequestedFormat, RequestedFormatType,
    Resolution,
};
use nokhwa::Camera;

use fcap_capture::{frame_channel, CaptureError, CaptureSession, Frame, FrameSender, PixelFormat};

/// One attachable camera / capture card.
#[derive(Debug, Clone)]
pub struct VideoDeviceInfo {
    /// Opaque id for [`start_video_device`] (the backend's device index).
    pub id: String,
    pub name: String,
}

/// One capture format a device offers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VideoFormatInfo {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    /// The wire format ("MJPEG", "YUYV", …) — informational in the picker.
    pub fourcc: String,
}

impl std::fmt::Display for VideoFormatInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}×{} @ {} fps ({})",
            self.width, self.height, self.fps, self.fourcc
        )
    }
}

/// macOS: camera access needs an explicit grant before AVFoundation will
/// enumerate or stream. Elsewhere this is a no-op.
#[cfg(target_os = "macos")]
fn ensure_camera_permission() -> Result<(), CaptureError> {
    if nokhwa::nokhwa_check() {
        return Ok(());
    }
    let (tx, rx) = std::sync::mpsc::channel();
    nokhwa::nokhwa_initialize(move |granted| {
        let _ = tx.send(granted);
    });
    match rx.recv_timeout(std::time::Duration::from_secs(60)) {
        Ok(true) => Ok(()),
        Ok(false) => Err(CaptureError::PermissionDenied),
        Err(_) => Err(CaptureError::Backend(
            "camera permission prompt timed out".into(),
        )),
    }
}

#[cfg(not(target_os = "macos"))]
fn ensure_camera_permission() -> Result<(), CaptureError> {
    Ok(())
}

fn backend_error(err: nokhwa::NokhwaError) -> CaptureError {
    CaptureError::Backend(format!("video device: {err}"))
}

fn parse_index(device_id: &str) -> CameraIndex {
    match device_id.parse::<u32>() {
        Ok(index) => CameraIndex::Index(index),
        Err(_) => CameraIndex::String(device_id.to_string()),
    }
}

/// Resolve a friendly camera name (e.g. `"HP True Vision FHD Camera"`) to this
/// machine's device id. Used by the OBS importer: OBS stores a DirectShow
/// name+path while we key cameras by backend index, so matching the human name
/// against the enumerated devices is the only bridge. Exact (case-insensitive)
/// match first, then a prefix match for the trailing-suffix differences OBS and
/// the OS sometimes carry — never a fuzzy guess. `None` when nothing matches, so
/// the caller can fall back to manual re-selection.
pub fn device_id_by_name(name: &str, devices: &[VideoDeviceInfo]) -> Option<String> {
    let target = name.trim().to_ascii_lowercase();
    if target.is_empty() {
        return None;
    }
    devices
        .iter()
        .find(|device| device.name.trim().to_ascii_lowercase() == target)
        .or_else(|| {
            devices.iter().find(|device| {
                let candidate = device.name.trim().to_ascii_lowercase();
                !candidate.is_empty()
                    && (candidate.starts_with(&target) || target.starts_with(&candidate))
            })
        })
        .map(|device| device.id.clone())
}

/// Enumerate cameras / capture cards.
pub fn list_video_devices() -> Result<Vec<VideoDeviceInfo>, CaptureError> {
    ensure_camera_permission()?;
    let devices = nokhwa::query(ApiBackend::Auto).map_err(backend_error)?;
    Ok(devices
        .into_iter()
        .map(|info| VideoDeviceInfo {
            id: info.index().to_string(),
            name: info.human_name(),
        })
        .collect())
}

/// List the formats a device offers (opens the device briefly).
pub fn list_video_formats(device_id: &str) -> Result<Vec<VideoFormatInfo>, CaptureError> {
    ensure_camera_permission()?;
    let mut camera = Camera::new(
        parse_index(device_id),
        RequestedFormat::new::<RgbAFormat>(RequestedFormatType::AbsoluteHighestResolution),
    )
    .map_err(backend_error)?;
    let mut formats: Vec<VideoFormatInfo> = camera
        .compatible_camera_formats()
        .map_err(backend_error)?
        .into_iter()
        .map(|format| VideoFormatInfo {
            width: format.resolution().width(),
            height: format.resolution().height(),
            fps: format.frame_rate(),
            fourcc: format.format().to_string(),
        })
        .collect();
    formats.sort_by(|a, b| {
        (b.width * b.height, b.fps, &a.fourcc).cmp(&(a.width * a.height, a.fps, &b.fourcc))
    });
    formats.dedup();
    Ok(formats)
}

fn frame_format_from_fourcc(fourcc: &str) -> Option<FrameFormat> {
    // Match against nokhwa's Display names (what `list_video_formats` emits).
    [
        FrameFormat::MJPEG,
        FrameFormat::YUYV,
        FrameFormat::NV12,
        FrameFormat::GRAY,
        FrameFormat::RAWRGB,
    ]
    .into_iter()
    .find(|candidate| candidate.to_string() == fourcc)
}

/// Start streaming a device. `format: None` = the highest resolution the
/// device offers; `Some` = the user's pick from [`list_video_formats`].
/// `deinterlace: Some` runs the chosen classic algorithm (CAP-M17) over
/// every frame on this capture thread — identical on every OS.
/// `camera_profile` (CAP-M18) is the saved per-device control profile,
/// reapplied on every (re)open — hotplug and auto-recover included.
pub fn start_video_device(
    device_id: &str,
    format: Option<&VideoFormatInfo>,
    deinterlace: Option<(crate::deinterlace::Mode, crate::deinterlace::FieldOrder)>,
    camera_profile: Vec<(String, i64)>,
) -> Result<CaptureSession, CaptureError> {
    ensure_camera_permission()?;
    let index = parse_index(device_id);
    let controls_hub = crate::camera_controls::device(device_id);
    let requested_type = match format {
        Some(sel) => {
            let resolution = Resolution::new(sel.width, sel.height);
            match frame_format_from_fourcc(&sel.fourcc) {
                Some(frame_format) => RequestedFormatType::Closest(CameraFormat::new(
                    resolution,
                    frame_format,
                    sel.fps,
                )),
                None => RequestedFormatType::HighestResolution(resolution),
            }
        }
        None => RequestedFormatType::AbsoluteHighestResolution,
    };

    let deinterlacer =
        deinterlace.map(|(mode, order)| crate::deinterlace::Deinterlacer::new(mode, order));
    let (sender, receiver) = frame_channel();
    let stop = Arc::new(AtomicBool::new(false));
    let stop_thread = Arc::clone(&stop);
    let join = std::thread::Builder::new()
        .name("fcap-webcam".into())
        .spawn(move || {
            run(
                index,
                requested_type,
                sender,
                stop_thread,
                deinterlacer,
                controls_hub,
                camera_profile,
            )
        })
        .map_err(|err| CaptureError::Backend(format!("could not spawn capture: {err}")))?;
    Ok(CaptureSession::from_parts(receiver, stop, join))
}

#[allow(clippy::too_many_arguments)]
fn run(
    index: CameraIndex,
    requested_type: RequestedFormatType,
    sender: FrameSender,
    stop: Arc<AtomicBool>,
    deinterlacer: Option<crate::deinterlace::Deinterlacer>,
    controls_hub: Arc<crate::camera_controls::DeviceControls>,
    camera_profile: Vec<(String, i64)>,
) {
    // This session's token: only the session that published a device's
    // controls may retire them (two sources can share one camera).
    let token = crate::camera_controls::session_token();
    let result = run_inner(
        index,
        requested_type,
        &sender,
        &stop,
        deinterlacer,
        &controls_hub,
        &camera_profile,
        token,
    );
    crate::camera_controls::retire(&controls_hub, token);
    match result {
        Ok(()) => sender.close(None),
        Err(err) => sender.close(Some(err)),
    }
}

/// How many first-frame attempts a format gets before it is judged
/// unstreamable. Many cameras drop the first frame or two while the sensor
/// warms up, so a single failure must not condemn the user's chosen format —
/// only a *persistent* one (a truly unsupported mode, e.g. MSMF `0xC00D36D5`).
const WARMUP_FRAME_ATTEMPTS: usize = 8;
const WARMUP_RETRY_DELAY: std::time::Duration = std::time::Duration::from_millis(60);

/// Open a camera, start streaming, and prove it streams by pulling a frame. The
/// first frame is where an unsupported format errors — but a warming-up camera
/// can also drop its first frame(s), so retry briefly before rejecting the
/// format, so a transient blip never silently downgrades the requested mode.
fn try_stream(index: &CameraIndex, requested: RequestedFormatType) -> Result<Camera, CaptureError> {
    let mut camera = Camera::new(index.clone(), RequestedFormat::new::<RgbAFormat>(requested))
        .map_err(backend_error)?;
    camera.open_stream().map_err(backend_error)?;
    let mut last = None;
    for _ in 0..WARMUP_FRAME_ATTEMPTS {
        match camera.frame() {
            Ok(_) => return Ok(camera),
            Err(err) => last = Some(err),
        }
        std::thread::sleep(WARMUP_RETRY_DELAY);
    }
    Err(backend_error(
        last.expect("the warmup loop runs at least once"),
    ))
}

/// Bring a camera up on the requested format, falling back through
/// progressively more universally-supported ones — some cameras advertise
/// high-res modes Media Foundation can't actually stream.
fn open_streaming_camera(
    index: CameraIndex,
    requested: RequestedFormatType,
) -> Result<Camera, CaptureError> {
    let candidates = [
        requested,
        RequestedFormatType::Closest(CameraFormat::new(
            Resolution::new(1280, 720),
            FrameFormat::MJPEG,
            30,
        )),
        RequestedFormatType::None,
    ];
    let mut last = None;
    for candidate in candidates {
        match try_stream(&index, candidate) {
            Ok(camera) => return Ok(camera),
            Err(err) => {
                tracing::warn!("webcam: a requested format did not stream: {err}");
                last = Some(err);
            }
        }
    }
    Err(last.unwrap_or_else(|| CaptureError::Backend("no camera format streamed".into())))
}

#[allow(clippy::too_many_arguments)]
fn run_inner(
    index: CameraIndex,
    requested_type: RequestedFormatType,
    sender: &FrameSender,
    stop: &AtomicBool,
    mut deinterlacer: Option<crate::deinterlace::Deinterlacer>,
    controls_hub: &crate::camera_controls::DeviceControls,
    camera_profile: &[(String, i64)],
    token: u64,
) -> Result<(), CaptureError> {
    // The camera lives entirely on this thread (nokhwa handles aren't Send).
    let mut camera = open_streaming_camera(index, requested_type)?;

    // CAP-M18: surface whatever controls this backend reports, then queue
    // the saved profile so it reapplies on every (re)open. Claims the hub for
    // this session (a failed open never publishes, so it never claims).
    crate::camera_controls::publish_controls(controls_hub, &camera, camera_profile, token);

    // Cameras occasionally emit a corrupt MJPEG frame — skip those, but give
    // up honestly if nothing decodes for a while.
    let mut consecutive_failures = 0u32;
    while !stop.load(Ordering::Relaxed) && sender.is_open() {
        crate::camera_controls::apply_pending(controls_hub, &mut camera);
        let buffer = match camera.frame() {
            Ok(buffer) => buffer,
            Err(err) => return Err(backend_error(err)),
        };
        match buffer.decode_image::<RgbAFormat>() {
            Ok(image) => {
                consecutive_failures = 0;
                let width = image.width();
                let height = image.height();
                let mut frame = Frame {
                    width,
                    height,
                    stride: width * 4,
                    format: PixelFormat::Rgba8,
                    data: image.into_raw(),
                    captured_at: Instant::now(),
                };
                if let Some(deint) = deinterlacer.as_mut() {
                    deint.process(&mut frame);
                }
                sender.send(frame);
            }
            Err(err) => {
                consecutive_failures += 1;
                tracing::debug!("webcam frame decode failed: {err}");
                if consecutive_failures > 60 {
                    return Err(CaptureError::Backend(format!(
                        "camera frames stopped decoding: {err}"
                    )));
                }
            }
        }
    }
    let _ = camera.stop_stream();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn device_listing_never_panics() {
        // CI runners have no cameras (and macOS runners no TCC grant) — an
        // error or an empty list is fine; panicking is not.
        let _ = list_video_devices();
    }

    #[test]
    fn format_display_reads_naturally() {
        let format = VideoFormatInfo {
            width: 1920,
            height: 1080,
            fps: 30,
            fourcc: "MJPEG".into(),
        };
        assert_eq!(format.to_string(), "1920×1080 @ 30 fps (MJPEG)");
    }

    #[test]
    fn unknown_ids_become_string_indices() {
        assert!(matches!(parse_index("2"), CameraIndex::Index(2)));
        assert!(matches!(parse_index("usb-cam"), CameraIndex::String(_)));
    }

    #[test]
    fn device_id_resolves_by_name() {
        let devices = vec![
            VideoDeviceInfo {
                id: "0".into(),
                name: "HP True Vision FHD Camera".into(),
            },
            VideoDeviceInfo {
                id: "1".into(),
                name: "OBS Virtual Camera".into(),
            },
        ];
        // Exact, case-insensitive.
        assert_eq!(
            device_id_by_name("hp true vision fhd camera", &devices).as_deref(),
            Some("0")
        );
        // Prefix — OBS/OS can differ by a trailing suffix.
        assert_eq!(
            device_id_by_name("HP True Vision FHD Camera (USB2.0)", &devices).as_deref(),
            Some("0")
        );
        // No match / empty → None (caller re-selects).
        assert_eq!(device_id_by_name("Logitech Brio", &devices), None);
        assert_eq!(device_id_by_name("   ", &devices), None);
    }
}
