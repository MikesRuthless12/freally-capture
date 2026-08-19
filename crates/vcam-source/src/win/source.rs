//! The `IMFMediaSource` + `IMFMediaStream` the Frame Server drives.
//!
//! One video stream, RGB32, fed from the app's shared transport. The shape is
//! the canonical Media Foundation source: an event queue behind
//! `IMFMediaEventGenerator`, a presentation/stream descriptor advertising one
//! RGB32 type, and `RequestSample` producing an `IMFSample` per call by reading
//! the newest published frame (or a "no signal" slate when the app is idle).
//!
//! AUDITED `unsafe`: MF COM interop + the buffer lock during sample fill.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};

use windows::core::IUnknown;
use windows::core::{implement, Interface, GUID, PROPVARIANT};
use windows::Win32::Foundation::E_FAIL;
use windows::Win32::Media::MediaFoundation::{
    IMFAsyncCallback, IMFMediaEvent, IMFMediaEventGenerator_Impl, IMFMediaEventQueue,
    IMFMediaSource, IMFMediaSource_Impl, IMFMediaStream, IMFMediaStream_Impl, IMFMediaType,
    IMFPresentationDescriptor, IMFStreamDescriptor, MENewStream, MESourcePaused, MESourceStarted,
    MESourceStopped, MEStreamPaused, MEStreamStarted, MEStreamStopped, MFCreateEventQueue,
    MFCreateMediaType, MFCreateMemoryBuffer, MFCreatePresentationDescriptor, MFCreateSample,
    MFCreateStreamDescriptor, MFMediaType_Video, MFVideoFormat_RGB32, MFVideoInterlace_Progressive,
    MEDIA_EVENT_GENERATOR_GET_EVENT_FLAGS, MF_EVENT_TYPE, MF_MT_ALL_SAMPLES_INDEPENDENT,
    MF_MT_DEFAULT_STRIDE, MF_MT_FRAME_RATE, MF_MT_FRAME_SIZE, MF_MT_INTERLACE_MODE,
    MF_MT_MAJOR_TYPE, MF_MT_PIXEL_ASPECT_RATIO, MF_MT_SUBTYPE,
};

use crate::reader::{no_signal_slate, rgba_to_bgra, TransportView};
use crate::win::shmem::SharedRegion;
use crate::win::{DEFAULT_FPS, DEFAULT_HEIGHT, DEFAULT_WIDTH};
use fcap_stream::vcam::transport::FrameHeader;

/// 100-ns units per second — Media Foundation's time base.
const HNS_PER_SEC: i64 = 10_000_000;

/// Pack two u32s into the u64 an MF attribute like `MF_MT_FRAME_SIZE` expects
/// (high dword first).
fn pack_2x32(hi: u32, lo: u32) -> u64 {
    ((hi as u64) << 32) | (lo as u64)
}

/// Build the single RGB32 media type the camera advertises.
fn make_media_type(width: u32, height: u32, fps: u32) -> windows::core::Result<IMFMediaType> {
    // SAFETY: MFCreateMediaType returns an owned type we configure below.
    let mt = unsafe { MFCreateMediaType() }?;
    unsafe {
        mt.SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Video)?;
        mt.SetGUID(&MF_MT_SUBTYPE, &MFVideoFormat_RGB32)?;
        mt.SetUINT32(&MF_MT_INTERLACE_MODE, MFVideoInterlace_Progressive.0 as u32)?;
        mt.SetUINT32(&MF_MT_ALL_SAMPLES_INDEPENDENT, 1)?;
        mt.SetUINT32(&MF_MT_DEFAULT_STRIDE, width * 4)?;
        mt.SetUINT64(&MF_MT_FRAME_SIZE, pack_2x32(width, height))?;
        mt.SetUINT64(&MF_MT_FRAME_RATE, pack_2x32(fps, 1))?;
        mt.SetUINT64(&MF_MT_PIXEL_ASPECT_RATIO, pack_2x32(1, 1))?;
    }
    Ok(mt)
}

/// The camera's live geometry — read from the app's transport header when
/// present, else the default. Split out so it is testable without COM.
pub fn resolve_geometry(app_pid: u32) -> (u32, u32, u32) {
    // A quick, read-only peek at just the header (32 bytes) to size the stream.
    if let Ok(Some(region)) = SharedRegion::open(app_pid, std::mem::size_of::<FrameHeader>()) {
        if let Some(view) = TransportView::parse(region.bytes()) {
            if view.header.width > 0 && view.header.height > 0 {
                return (view.header.width, view.header.height, DEFAULT_FPS);
            }
        }
    }
    (DEFAULT_WIDTH, DEFAULT_HEIGHT, DEFAULT_FPS)
}

/// A thin wrapper turning the four `IMFMediaEventGenerator` methods into
/// forwards to an `IMFMediaEventQueue` — shared by the source and the stream.
struct EventGen {
    queue: IMFMediaEventQueue,
}

impl EventGen {
    fn new() -> windows::core::Result<Self> {
        // SAFETY: MFCreateEventQueue yields an owned queue.
        let queue = unsafe { MFCreateEventQueue() }?;
        Ok(Self { queue })
    }

    fn queue_simple(&self, met: MF_EVENT_TYPE) -> windows::core::Result<()> {
        // SAFETY: a status-only event with no payload — the common signalling case.
        unsafe {
            self.queue.QueueEventParamVar(
                met.0 as u32,
                &GUID::zeroed(),
                windows::Win32::Foundation::S_OK,
                std::ptr::null(),
            )
        }
    }

    fn get_event(
        &self,
        flags: MEDIA_EVENT_GENERATOR_GET_EVENT_FLAGS,
    ) -> windows::core::Result<IMFMediaEvent> {
        // SAFETY: forwards to the owned queue.
        unsafe { self.queue.GetEvent(flags.0) }
    }

    fn begin_get_event(
        &self,
        callback: Option<&IMFAsyncCallback>,
        state: Option<&IUnknown>,
    ) -> windows::core::Result<()> {
        // SAFETY: the queue holds the callback until EndGetEvent.
        unsafe { self.queue.BeginGetEvent(callback, state) }
    }

    fn end_get_event(
        &self,
        result: Option<&windows::Win32::Media::MediaFoundation::IMFAsyncResult>,
    ) -> windows::core::Result<IMFMediaEvent> {
        // SAFETY: pairs with a BeginGetEvent on the same queue.
        unsafe { self.queue.EndGetEvent(result) }
    }

    fn queue_event(
        &self,
        met: u32,
        ext: *const GUID,
        status: windows::core::HRESULT,
        value: *const PROPVARIANT,
    ) -> windows::core::Result<()> {
        // SAFETY: direct forward; pointers are the caller's, valid for the call.
        unsafe { self.queue.QueueEventParamVar(met, ext, status, value) }
    }

    /// Queue an event whose value is a COM object (e.g. MENewStream carries the
    /// stream, MEMediaSample carries the sample).
    fn queue_object<P0: windows::core::Param<IUnknown>>(
        &self,
        met: MF_EVENT_TYPE,
        obj: P0,
    ) -> windows::core::Result<()> {
        // SAFETY: the queue AddRefs the object; a status-only S_OK.
        unsafe {
            self.queue.QueueEventParamUnk(
                met.0 as u32,
                &GUID::zeroed(),
                windows::Win32::Foundation::S_OK,
                obj,
            )
        }
    }

    fn shutdown(&self) {
        // SAFETY: idempotent per MF; safe to call on teardown.
        unsafe {
            let _ = self.queue.Shutdown();
        }
    }
}

/// The one video stream.
#[implement(IMFMediaStream)]
pub struct VcamStream {
    events: EventGen,
    descriptor: IMFStreamDescriptor,
    source: Mutex<Option<IMFMediaSource>>,
    app_pid: u32,
    width: u32,
    height: u32,
    fps: u32,
    /// Monotonic sample clock in 100-ns units.
    clock_hns: AtomicU64,
    active: AtomicBool,
    /// The grey "no signal" frame, built once — it is a constant for this
    /// geometry, and rebuilding it per sample costs an 8 MB alloc + fill at
    /// 1080p for every tick the app is not streaming.
    slate: OnceLock<Vec<u8>>,
}

impl VcamStream {
    fn new(
        descriptor: IMFStreamDescriptor,
        app_pid: u32,
        width: u32,
        height: u32,
        fps: u32,
    ) -> windows::core::Result<Self> {
        Ok(Self {
            events: EventGen::new()?,
            descriptor,
            source: Mutex::new(None),
            app_pid,
            width,
            height,
            fps,
            clock_hns: AtomicU64::new(0),
            active: AtomicBool::new(false),
            slate: OnceLock::new(),
        })
    }

    fn set_source(&self, source: IMFMediaSource) {
        *self.source.lock().unwrap_or_else(|e| e.into_inner()) = Some(source);
    }

    fn start(&self) -> windows::core::Result<()> {
        self.active.store(true, Ordering::SeqCst);
        self.events.queue_simple(MEStreamStarted)
    }

    fn stop(&self) -> windows::core::Result<()> {
        self.active.store(false, Ordering::SeqCst);
        self.events.queue_simple(MEStreamStopped)
    }

    fn pause(&self) -> windows::core::Result<()> {
        self.events.queue_simple(MEStreamPaused)
    }

    /// Produce one RGB32 sample from the newest published frame (or the slate),
    /// stamped and queued as an `MEMediaSample`.
    fn produce_sample(&self, token: Option<&IUnknown>) -> windows::core::Result<()> {
        let frame_bytes = (self.width as usize) * (self.height as usize) * 4;

        // Read the newest frame (or serve the slate when the app isn't
        // streaming). The conversion buffer is allocated only when there is
        // actually a frame to convert, so the idle path allocates nothing and
        // a streaming tick allocates once instead of twice.
        let mut converted: Option<Vec<u8>> = None;
        if let Ok(Some(region)) = SharedRegion::open(
            self.app_pid,
            FrameHeader::new(self.width, self.height).total_bytes(),
        ) {
            if let Some(view) = TransportView::parse(region.bytes()) {
                if view.streaming() {
                    if let Some(rgba) = view.ready_rgba() {
                        let mut buf = vec![0u8; frame_bytes];
                        if rgba_to_bgra(rgba, &mut buf, self.width, self.height) {
                            converted = Some(buf);
                        }
                    }
                }
            }
        }
        let bgra: &[u8] = match converted.as_deref() {
            Some(frame) => frame,
            None => self
                .slate
                .get_or_init(|| no_signal_slate(self.width, self.height)),
        };

        // Wrap it in an IMFSample.
        // SAFETY: standard MF sample construction; the buffer is locked/unlocked
        // in a matched pair and sized to exactly `frame_bytes`.
        let sample = unsafe {
            let sample = MFCreateSample()?;
            let buffer = MFCreateMemoryBuffer(frame_bytes as u32)?;
            let mut ptr = std::ptr::null_mut();
            let mut max = 0u32;
            buffer.Lock(&mut ptr, Some(&mut max), None)?;
            std::ptr::copy_nonoverlapping(bgra.as_ptr(), ptr, frame_bytes);
            buffer.SetCurrentLength(frame_bytes as u32)?;
            buffer.Unlock()?;
            sample.AddBuffer(&buffer)?;

            let per_frame = HNS_PER_SEC / (self.fps.max(1) as i64);
            let t = self.clock_hns.fetch_add(per_frame as u64, Ordering::SeqCst) as i64;
            sample.SetSampleTime(t)?;
            sample.SetSampleDuration(per_frame)?;
            if let Some(token) = token {
                // The frame server hands a token to correlate the sample; echo it.
                sample.SetUnknown(
                    &windows::Win32::Media::MediaFoundation::MFSampleExtension_Token,
                    token,
                )?;
            }
            sample
        };

        // Queue MEMediaSample carrying the sample object.
        self.events.queue_object(
            windows::Win32::Media::MediaFoundation::MEMediaSample,
            &IUnknown::from(sample),
        )
    }
}

impl IMFMediaStream_Impl for VcamStream_Impl {
    fn GetMediaSource(&self) -> windows::core::Result<IMFMediaSource> {
        self.source
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
            .ok_or_else(|| E_FAIL.into())
    }

    fn GetStreamDescriptor(&self) -> windows::core::Result<IMFStreamDescriptor> {
        Ok(self.descriptor.clone())
    }

    fn RequestSample(&self, ptoken: Option<&IUnknown>) -> windows::core::Result<()> {
        if !self.active.load(Ordering::SeqCst) {
            return Err(E_FAIL.into());
        }
        self.produce_sample(ptoken)
    }
}

impl IMFMediaEventGenerator_Impl for VcamStream_Impl {
    fn GetEvent(
        &self,
        dwflags: MEDIA_EVENT_GENERATOR_GET_EVENT_FLAGS,
    ) -> windows::core::Result<IMFMediaEvent> {
        self.events.get_event(dwflags)
    }
    fn BeginGetEvent(
        &self,
        pcallback: Option<&IMFAsyncCallback>,
        punkstate: Option<&IUnknown>,
    ) -> windows::core::Result<()> {
        self.events.begin_get_event(pcallback, punkstate)
    }
    fn EndGetEvent(
        &self,
        presult: Option<&windows::Win32::Media::MediaFoundation::IMFAsyncResult>,
    ) -> windows::core::Result<IMFMediaEvent> {
        self.events.end_get_event(presult)
    }
    fn QueueEvent(
        &self,
        met: u32,
        guidextendedtype: *const GUID,
        hrstatus: windows::core::HRESULT,
        pvvalue: *const PROPVARIANT,
    ) -> windows::core::Result<()> {
        self.events
            .queue_event(met, guidextendedtype, hrstatus, pvvalue)
    }
}

/// The media source: one stream, one presentation descriptor.
#[implement(IMFMediaSource)]
pub struct VcamSource {
    events: EventGen,
    stream: IMFMediaStream,
    descriptor: IMFPresentationDescriptor,
    started: AtomicBool,
}

impl Drop for VcamSource {
    fn drop(&mut self) {
        // Balances the increment in `create` — see `DllCanUnloadNow`.
        let _ = crate::win::registration::OBJECT_COUNT.fetch_update(
            Ordering::SeqCst,
            Ordering::SeqCst,
            |n| Some(n.saturating_sub(1)),
        );
    }
}

impl VcamSource {
    /// Build a source for the app process `app_pid`. Public so `DllGetClassObject`
    /// / the class factory can construct it.
    pub fn create(app_pid: u32) -> windows::core::Result<IMFMediaSource> {
        // Counted so the DLL cannot be unloaded while this object is alive.
        crate::win::registration::OBJECT_COUNT.fetch_add(1, Ordering::SeqCst);
        let (width, height, fps) = resolve_geometry(app_pid);
        let media_type = make_media_type(width, height, fps)?;

        // Stream descriptor carrying the one media type.
        // SAFETY: MFCreateStreamDescriptor takes the type array by pointer/len.
        let stream_descriptor =
            unsafe { MFCreateStreamDescriptor(0, &[Some(media_type.clone())]) }?;
        // Mark the type current on the descriptor's handler.
        unsafe {
            let handler = stream_descriptor.GetMediaTypeHandler()?;
            handler.SetCurrentMediaType(&media_type)?;
        }

        let stream_obj = VcamStream::new(stream_descriptor.clone(), app_pid, width, height, fps)?;
        let stream: IMFMediaStream = stream_obj.into();

        // Presentation descriptor carrying the one stream descriptor.
        // SAFETY: MFCreatePresentationDescriptor takes the descriptor array.
        let pd = unsafe { MFCreatePresentationDescriptor(Some(&[Some(stream_descriptor)])) }?;
        // Select the stream so the pipeline actually pulls from it.
        unsafe {
            pd.SelectStream(0)?;
        }

        let source_obj = VcamSource {
            events: EventGen::new()?,
            stream: stream.clone(),
            descriptor: pd,
            started: AtomicBool::new(false),
        };
        let source: IMFMediaSource = source_obj.into();

        // Back-link the stream to its source.
        if let Ok(s) = stream.cast_object::<VcamStream>() {
            s.set_source(source.clone());
        }
        Ok(source)
    }
}

impl IMFMediaSource_Impl for VcamSource_Impl {
    fn GetCharacteristics(&self) -> windows::core::Result<u32> {
        // MFMEDIASOURCE_IS_LIVE — a live, non-seekable camera.
        Ok(windows::Win32::Media::MediaFoundation::MFMEDIASOURCE_IS_LIVE.0 as u32)
    }

    fn CreatePresentationDescriptor(&self) -> windows::core::Result<IMFPresentationDescriptor> {
        // SAFETY: Clone returns an independent copy per MF's contract.
        unsafe { self.descriptor.Clone() }
    }

    fn Start(
        &self,
        _pd: Option<&IMFPresentationDescriptor>,
        _timeformat: *const GUID,
        _startposition: *const PROPVARIANT,
    ) -> windows::core::Result<()> {
        let first = !self.started.swap(true, Ordering::SeqCst);
        // Announce the stream on first start, then that the source started.
        if first {
            self.events
                .queue_object(MENewStream, &IUnknown::from(self.stream.clone()))?;
        }
        if let Ok(s) = self.stream.cast_object::<VcamStream>() {
            s.start()?;
        }
        self.events.queue_simple(MESourceStarted)
    }

    fn Stop(&self) -> windows::core::Result<()> {
        self.started.store(false, Ordering::SeqCst);
        if let Ok(s) = self.stream.cast_object::<VcamStream>() {
            s.stop()?;
        }
        self.events.queue_simple(MESourceStopped)
    }

    fn Pause(&self) -> windows::core::Result<()> {
        if let Ok(s) = self.stream.cast_object::<VcamStream>() {
            s.pause()?;
        }
        self.events.queue_simple(MESourcePaused)
    }

    fn Shutdown(&self) -> windows::core::Result<()> {
        self.events.shutdown();
        if let Ok(s) = self.stream.cast_object::<VcamStream>() {
            s.events.shutdown();
            // Break the strong cycle. This source holds the stream, and
            // `set_source` gave the stream a strong reference straight back —
            // so without clearing it here neither object is ever released and
            // the event queues, media type and per-open state leak for the life
            // of the frame-server process, once per camera open.
            *s.source.lock().unwrap_or_else(|e| e.into_inner()) = None;
        }
        Ok(())
    }
}

impl IMFMediaEventGenerator_Impl for VcamSource_Impl {
    fn GetEvent(
        &self,
        dwflags: MEDIA_EVENT_GENERATOR_GET_EVENT_FLAGS,
    ) -> windows::core::Result<IMFMediaEvent> {
        self.events.get_event(dwflags)
    }
    fn BeginGetEvent(
        &self,
        pcallback: Option<&IMFAsyncCallback>,
        punkstate: Option<&IUnknown>,
    ) -> windows::core::Result<()> {
        self.events.begin_get_event(pcallback, punkstate)
    }
    fn EndGetEvent(
        &self,
        presult: Option<&windows::Win32::Media::MediaFoundation::IMFAsyncResult>,
    ) -> windows::core::Result<IMFMediaEvent> {
        self.events.end_get_event(presult)
    }
    fn QueueEvent(
        &self,
        met: u32,
        guidextendedtype: *const GUID,
        hrstatus: windows::core::HRESULT,
        pvvalue: *const PROPVARIANT,
    ) -> windows::core::Result<()> {
        self.events
            .queue_event(met, guidextendedtype, hrstatus, pvvalue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geometry_falls_back_to_default_without_an_app() {
        // No app streaming for this pid → the advertised default, not a panic.
        let (w, h, fps) = resolve_geometry(0xDEAD_BEEF);
        assert_eq!((w, h, fps), (DEFAULT_WIDTH, DEFAULT_HEIGHT, DEFAULT_FPS));
    }

    #[test]
    fn frame_rate_and_size_pack_high_dword_first() {
        assert_eq!(pack_2x32(1920, 1080), (1920u64 << 32) | 1080);
        assert_eq!(pack_2x32(30, 1), (30u64 << 32) | 1);
    }
}
