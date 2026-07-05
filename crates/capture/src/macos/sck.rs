//! The SCStream pipeline: shareable-content fetch, stream construction, and
//! the `SCStreamOutput` delegate that turns `CMSampleBuffer`s into [`Frame`]s.
//!
//! AUDITED `unsafe`: see the module note in `macos/mod.rs`.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::time::{Duration, Instant};

use block2::RcBlock;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{define_class, msg_send, AllocAnyThread, DefinedClass};
use objc2_core_media::{CMSampleBuffer, CMTime, CMTimeFlags};
use objc2_core_video::{
    CVImageBuffer, CVPixelBuffer, CVPixelBufferGetBaseAddress, CVPixelBufferGetBytesPerRow,
    CVPixelBufferGetHeight, CVPixelBufferGetWidth, CVPixelBufferLockBaseAddress,
    CVPixelBufferLockFlags, CVPixelBufferUnlockBaseAddress,
};
use objc2_foundation::{NSArray, NSError, NSObject, NSObjectProtocol};
use objc2_screen_capture_kit::{
    SCContentFilter, SCShareableContent, SCStream, SCStreamConfiguration, SCStreamDelegate,
    SCStreamOutput, SCStreamOutputType,
};

use crate::{CaptureError, Frame, FrameSender, PixelFormat};

/// FourCC `'BGRA'` — `kCVPixelFormatType_32BGRA` (spelled out so we don't
/// depend on the constant's generated location).
const PIXEL_FORMAT_BGRA: u32 = u32::from_be_bytes(*b"BGRA");

/// `SCStreamErrorCode.userDeclined` — the TCC "user said no" code. Matching
/// the code (not the localized description) keeps permission detection
/// working in every macOS language.
const SC_ERROR_USER_DECLINED: isize = -3801;

/// What a completion handler reports about an NSError, in a Send-able shape.
struct ErrorInfo {
    code: isize,
    message: String,
}

impl ErrorInfo {
    /// SAFETY-free helper: callers pass a live, non-null NSError reference.
    fn from_ns_error(error: &NSError) -> Self {
        ErrorInfo {
            code: error.code(),
            message: error.localizedDescription().to_string(),
        }
    }

    fn into_capture_error(self, context: &str) -> CaptureError {
        // Primary: the stable error code. Fallback: English substrings, in
        // case a permission failure arrives under a different domain/code.
        let lowered = self.message.to_lowercase();
        if self.code == SC_ERROR_USER_DECLINED
            || lowered.contains("declined")
            || lowered.contains("permission")
        {
            CaptureError::PermissionDenied
        } else {
            CaptureError::Backend(format!("{context}: {}", self.message))
        }
    }
}

pub(crate) enum Target {
    Display(u32),
    Window(u32),
}

/// A retained `SCShareableContent` pointer that may cross the completion
/// queue → caller thread boundary.
struct SendContent(*mut SCShareableContent);
// SAFETY: SCShareableContent is an immutable snapshot; we only move the
// (retained) pointer across threads, never share it concurrently.
unsafe impl Send for SendContent {}

/// Fetch the current displays + windows, incl. minimized/off-screen ones
/// (blocking, 10 s timeout).
pub(crate) fn fetch_shareable_content() -> Result<Retained<SCShareableContent>, CaptureError> {
    let (tx, rx) = mpsc::channel::<Result<SendContent, ErrorInfo>>();
    let block = RcBlock::new(
        move |content: *mut SCShareableContent, error: *mut NSError| {
            let result = if content.is_null() {
                let info = if error.is_null() {
                    ErrorInfo {
                        code: 0,
                        message: "unknown ScreenCaptureKit error".to_string(),
                    }
                } else {
                    // SAFETY: SCK hands a live NSError when content is null.
                    ErrorInfo::from_ns_error(unsafe { &*error })
                };
                Err(info)
            } else {
                // SAFETY: retain the non-null snapshot so it outlives the block;
                // the raw pointer is re-owned via from_raw on the receiving side.
                let retained =
                    unsafe { Retained::retain(content) }.expect("retained non-null content");
                Ok(SendContent(Retained::into_raw(retained)))
            };
            let _ = tx.send(result);
        },
    );
    // SAFETY: SCK invokes the completion block exactly once. onScreenWindowsOnly
    // = false so minimized/off-screen windows are included (the picker lists every
    // open window); `list_sources` trims non-app layers via windowLayer.
    unsafe {
        SCShareableContent::getShareableContentExcludingDesktopWindows_onScreenWindowsOnly_completionHandler(
            true, false, &block,
        );
    }
    match rx.recv_timeout(Duration::from_secs(10)) {
        // SAFETY: re-own the pointer retained inside the block.
        Ok(Ok(content)) => Ok(unsafe { Retained::from_raw(content.0) }.expect("non-null")),
        Ok(Err(info)) => Err(info.into_capture_error("shareable content")),
        Err(_) => Err(CaptureError::Backend(
            "shareable content lookup timed out".into(),
        )),
    }
}

struct OutputIvars {
    sender: FrameSender,
}

define_class!(
    #[unsafe(super(NSObject))]
    #[name = "FcapCaptureOutput"]
    #[ivars = OutputIvars]
    struct CaptureOutput;

    unsafe impl NSObjectProtocol for CaptureOutput {}

    unsafe impl SCStreamOutput for CaptureOutput {
        #[unsafe(method(stream:didOutputSampleBuffer:ofType:))]
        fn stream_did_output_sample_buffer_of_type(
            &self,
            _stream: &SCStream,
            sample_buffer: &CMSampleBuffer,
            of_type: SCStreamOutputType,
        ) {
            self.handle_sample(sample_buffer, of_type);
        }
    }

    unsafe impl SCStreamDelegate for CaptureOutput {
        #[unsafe(method(stream:didStopWithError:))]
        fn stream_did_stop_with_error(&self, _stream: &SCStream, error: &NSError) {
            let msg = error.localizedDescription().to_string();
            self.ivars()
                .sender
                .close(Some(CaptureError::Backend(format!(
                    "capture stopped: {msg}"
                ))));
        }
    }
);

impl CaptureOutput {
    fn new(sender: FrameSender) -> Retained<Self> {
        let this = Self::alloc().set_ivars(OutputIvars { sender });
        // SAFETY: plain NSObject init on a freshly allocated instance.
        unsafe { msg_send![super(this), init] }
    }

    fn handle_sample(&self, sample_buffer: &CMSampleBuffer, of_type: SCStreamOutputType) {
        if of_type != SCStreamOutputType::Screen {
            return;
        }
        let sender = &self.ivars().sender;
        if !sender.is_open() {
            return;
        }
        // SAFETY: Get-rule accessor; the wrapper retains the buffer. Idle
        // SCK samples carry no image buffer — skip those.
        let Some(image) = (unsafe { sample_buffer.image_buffer() }) else {
            return;
        };
        let image_ref: &CVImageBuffer = &image;
        // SAFETY: SCK video samples are always CVPixelBuffer-backed — the CF
        // "subclass" relationship the C headers express by pointer cast.
        let pixel: &CVPixelBuffer =
            unsafe { &*(image_ref as *const CVImageBuffer as *const CVPixelBuffer) };

        // SAFETY: lock → read (base/stride/dims) → copy out → unlock; the
        // base pointer is only used inside the lock window.
        unsafe {
            let _ = CVPixelBufferLockBaseAddress(pixel, CVPixelBufferLockFlags::ReadOnly);
            let base = CVPixelBufferGetBaseAddress(pixel);
            let width = CVPixelBufferGetWidth(pixel) as u32;
            let height = CVPixelBufferGetHeight(pixel) as u32;
            let stride = CVPixelBufferGetBytesPerRow(pixel) as u32;
            if base.is_null() || width == 0 || height == 0 || stride < width * 4 {
                let _ = CVPixelBufferUnlockBaseAddress(pixel, CVPixelBufferLockFlags::ReadOnly);
                return;
            }
            let len = stride as usize * height as usize;
            let data = std::slice::from_raw_parts(base as *const u8, len).to_vec();
            let _ = CVPixelBufferUnlockBaseAddress(pixel, CVPixelBufferLockFlags::ReadOnly);
            sender.send(Frame {
                width,
                height,
                stride,
                format: PixelFormat::Bgra8,
                data,
                captured_at: Instant::now(),
            });
        }
    }
}

pub(crate) fn run(target: Target, sender: FrameSender, stop: Arc<AtomicBool>) {
    match run_inner(target, &sender, &stop) {
        Ok(()) => sender.close(None),
        Err(err) => sender.close(Some(err)),
    }
}

fn run_inner(target: Target, sender: &FrameSender, stop: &AtomicBool) -> Result<(), CaptureError> {
    let content = fetch_shareable_content()?;

    // Build the content filter + pick the output size. Sizes are in points —
    // geometry-correct everywhere; Retina-native pixel sizing arrives with
    // the compositor work (Phase 2).
    let (filter, width, height) = match target {
        Target::Display(display_id) => {
            // SAFETY: getters on the retained snapshot.
            let displays = unsafe { content.displays() };
            let display = displays
                .iter()
                // SAFETY: SCDisplay getter.
                .find(|display| unsafe { display.displayID() } == display_id)
                .ok_or_else(|| {
                    CaptureError::NotFound("the display is no longer attached".into())
                })?;
            // SAFETY: SCDisplay getter.
            let rect = unsafe { display.frame() };
            let empty: Retained<NSArray<objc2_screen_capture_kit::SCWindow>> = NSArray::new();
            // SAFETY: documented initializer with a retained display + array.
            let filter = unsafe {
                SCContentFilter::initWithDisplay_excludingWindows(
                    SCContentFilter::alloc(),
                    &display,
                    &empty,
                )
            };
            (filter, rect.size.width as u32, rect.size.height as u32)
        }
        Target::Window(window_id) => {
            // SAFETY: getters on the retained snapshot.
            let windows = unsafe { content.windows() };
            let window = windows
                .iter()
                // SAFETY: SCWindow getter.
                .find(|window| unsafe { window.windowID() } == window_id)
                .ok_or_else(|| CaptureError::NotFound("the window was closed".into()))?;
            // SAFETY: SCWindow getter.
            let rect = unsafe { window.frame() };
            // SAFETY: documented initializer with a retained window.
            let filter = unsafe {
                SCContentFilter::initWithDesktopIndependentWindow(SCContentFilter::alloc(), &window)
            };
            (filter, rect.size.width as u32, rect.size.height as u32)
        }
    };
    let width = width.max(2) & !1;
    let height = height.max(2) & !1;

    // SAFETY: plain +new on the configuration class.
    let config = unsafe { SCStreamConfiguration::new() };
    // SAFETY: property setters on a fresh configuration object.
    unsafe {
        config.setWidth(width as usize);
        config.setHeight(height as usize);
        config.setPixelFormat(PIXEL_FORMAT_BGRA);
        config.setMinimumFrameInterval(CMTime {
            value: 1,
            timescale: 60,
            flags: CMTimeFlags::Valid,
            epoch: 0,
        });
        config.setQueueDepth(4);
        config.setShowsCursor(true);
    }

    let output = CaptureOutput::new(sender.clone());
    let delegate = ProtocolObject::from_ref(&*output);
    // SAFETY: documented initializer; the delegate outlives the stream (both
    // live on this thread until the end of this function).
    let stream = unsafe {
        SCStream::initWithFilter_configuration_delegate(
            SCStream::alloc(),
            &filter,
            &config,
            Some(delegate),
        )
    };

    let queue = dispatch2::DispatchQueue::new("com.freally.capture.sck-frames", None);
    let output_proto = ProtocolObject::from_ref(&*output);
    // SAFETY: registers the output on our serial queue; SCK retains both.
    unsafe {
        stream.addStreamOutput_type_sampleHandlerQueue_error(
            output_proto,
            SCStreamOutputType::Screen,
            Some(&queue),
        )
    }
    .map_err(|err| {
        CaptureError::Backend(format!("addStreamOutput: {}", err.localizedDescription()))
    })?;

    start_capture_blocking(&stream)?;

    // Frames arrive on the dispatch queue; park until stopped or closed.
    while !stop.load(Ordering::Relaxed) && sender.is_open() {
        std::thread::sleep(Duration::from_millis(100));
    }

    stop_capture_blocking(&stream);
    Ok(())
}

fn start_capture_blocking(stream: &SCStream) -> Result<(), CaptureError> {
    let (tx, rx) = mpsc::channel::<Option<ErrorInfo>>();
    let block = RcBlock::new(move |error: *mut NSError| {
        let info = if error.is_null() {
            None
        } else {
            // SAFETY: live NSError for the duration of the block call.
            Some(ErrorInfo::from_ns_error(unsafe { &*error }))
        };
        let _ = tx.send(info);
    });
    // SAFETY: SCK invokes the completion block exactly once.
    unsafe { stream.startCaptureWithCompletionHandler(Some(&*block)) };
    match rx.recv_timeout(Duration::from_secs(10)) {
        Ok(None) => Ok(()),
        Ok(Some(info)) => Err(info.into_capture_error("startCapture")),
        Err(_) => Err(CaptureError::Backend("startCapture timed out".into())),
    }
}

fn stop_capture_blocking(stream: &SCStream) {
    let (tx, rx) = mpsc::channel::<()>();
    let block = RcBlock::new(move |_error: *mut NSError| {
        let _ = tx.send(());
    });
    // SAFETY: SCK invokes the completion block exactly once; best-effort.
    unsafe { stream.stopCaptureWithCompletionHandler(Some(&*block)) };
    let _ = rx.recv_timeout(Duration::from_secs(2));
}
