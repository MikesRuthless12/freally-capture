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
pub fn start_video_device(
    device_id: &str,
    format: Option<&VideoFormatInfo>,
) -> Result<CaptureSession, CaptureError> {
    ensure_camera_permission()?;
    let index = parse_index(device_id);
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

    let (sender, receiver) = frame_channel();
    let stop = Arc::new(AtomicBool::new(false));
    let stop_thread = Arc::clone(&stop);
    let join = std::thread::Builder::new()
        .name("fcap-webcam".into())
        .spawn(move || run(index, requested_type, sender, stop_thread))
        .map_err(|err| CaptureError::Backend(format!("could not spawn capture: {err}")))?;
    Ok(CaptureSession::from_parts(receiver, stop, join))
}

fn run(
    index: CameraIndex,
    requested_type: RequestedFormatType,
    sender: FrameSender,
    stop: Arc<AtomicBool>,
) {
    match run_inner(index, requested_type, &sender, &stop) {
        Ok(()) => sender.close(None),
        Err(err) => sender.close(Some(err)),
    }
}

fn run_inner(
    index: CameraIndex,
    requested_type: RequestedFormatType,
    sender: &FrameSender,
    stop: &AtomicBool,
) -> Result<(), CaptureError> {
    // The camera lives entirely on this thread (nokhwa handles aren't Send).
    let mut camera = Camera::new(index, RequestedFormat::new::<RgbAFormat>(requested_type))
        .map_err(backend_error)?;
    camera.open_stream().map_err(backend_error)?;

    // Cameras occasionally emit a corrupt MJPEG frame — skip those, but give
    // up honestly if nothing decodes for a while.
    let mut consecutive_failures = 0u32;
    while !stop.load(Ordering::Relaxed) && sender.is_open() {
        let buffer = match camera.frame() {
            Ok(buffer) => buffer,
            Err(err) => return Err(backend_error(err)),
        };
        match buffer.decode_image::<RgbAFormat>() {
            Ok(image) => {
                consecutive_failures = 0;
                let width = image.width();
                let height = image.height();
                sender.send(Frame {
                    width,
                    height,
                    stride: width * 4,
                    format: PixelFormat::Rgba8,
                    data: image.into_raw(),
                    captured_at: Instant::now(),
                });
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
}
