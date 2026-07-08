//! Windows per-application audio capture (WASAPI **process loopback**).
//!
//! Two things live here, and only here (so `fcap-audio` stays
//! `#![forbid(unsafe_code)]`):
//!
//! 1. [`list_audio_apps`] — enumerate the render **audio sessions** on the
//!    default playback device and resolve each to a `(pid, name, exe)`. This is
//!    what the picker shows: the apps actually making sound right now.
//! 2. [`ProcessCapture`] — start a WASAPI capture bound to one process tree via
//!    `ActivateAudioInterfaceAsync` on the `VAD\Process_Loopback` virtual device
//!    with `PROCESS_LOOPBACK_MODE_INCLUDE_TARGET_PROCESS_TREE`. The device
//!    format is fixed by us (there is no `GetMixFormat` for the virtual device):
//!    interleaved **stereo f32 @ 48 kHz**, exactly the mixer's internal clock.
//!
//! Requires Windows 10 build 2004 (19041) or newer; on older builds
//! `ActivateAudioInterfaceAsync` fails and we surface it as a `Backend` error
//! the UI turns into the honest guidance.

#![allow(unsafe_code)]

use std::mem::size_of;
use std::slice;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;

use windows::core::{implement, Interface, PCWSTR};
use windows::Win32::Foundation::{CloseHandle, HANDLE, MAX_PATH, WAIT_OBJECT_0};
use windows::Win32::Media::Audio::{
    ActivateAudioInterfaceAsync, eConsole, eRender, IActivateAudioInterfaceAsyncOperation,
    IActivateAudioInterfaceCompletionHandler, IActivateAudioInterfaceCompletionHandler_Impl,
    IAudioCaptureClient, IAudioClient, IAudioSessionControl2, IAudioSessionEnumerator,
    IAudioSessionManager2, IMMDeviceEnumerator, MMDeviceEnumerator, AUDCLNT_BUFFERFLAGS_SILENT,
    AUDCLNT_SHAREMODE_SHARED, AUDCLNT_STREAMFLAGS_EVENTCALLBACK, AUDCLNT_STREAMFLAGS_LOOPBACK,
    AUDIOCLIENT_ACTIVATION_PARAMS, AUDIOCLIENT_ACTIVATION_PARAMS_0,
    AUDIOCLIENT_ACTIVATION_TYPE_PROCESS_LOOPBACK, AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS,
    AudioSessionStateActive, PROCESS_LOOPBACK_MODE_INCLUDE_TARGET_PROCESS_TREE,
    VIRTUAL_AUDIO_DEVICE_PROCESS_LOOPBACK, WAVEFORMATEX,
};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoUninitialize, BLOB, CLSCTX_ALL, COINIT_MULTITHREADED,
};
use windows::Win32::System::Threading::{
    CreateEventW, OpenProcess, QueryFullProcessImageNameW, SetEvent, WaitForSingleObject, INFINITE,
    PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION,
};
use windows::Win32::System::Variant::VT_BLOB;

use crate::{AppAudioError, AudioApp};

const SAMPLE_RATE: u32 = 48_000;
const CHANNELS: u16 = 2;
/// `WAVE_FORMAT_IEEE_FLOAT` — 32-bit float PCM tag (not surfaced by the
/// `windows` feature set we enable, so named locally).
const WAVE_FORMAT_IEEE_FLOAT: u16 = 0x0003;

fn backend<E: std::fmt::Display>(e: E) -> AppAudioError {
    AppAudioError::Backend(e.to_string())
}

/// A `HANDLE` wrapper that is `Send` so the auto-reset wake event can move onto
/// the capture thread. Sound here: the handle is only ever waited-on/signalled,
/// and its lifetime is bounded by the `ProcessCapture` that owns it.
#[derive(Clone, Copy)]
struct SendHandle(HANDLE);
// SAFETY: a Win32 event HANDLE is just a kernel object id; signalling/waiting it
// from another thread is the documented, thread-safe use.
unsafe impl Send for SendHandle {}

/// A layout-compatible stand-in for the C `PROPVARIANT` used solely to hand a
/// `VT_BLOB` (the process-loopback activation params) to
/// `ActivateAudioInterfaceAsync`. `windows_core::PROPVARIANT` is opaque and
/// exposes no blob constructor, so we build the exact C layout and cast the
/// pointer — the FFI only reads it as a raw `*const PROPVARIANT`.
#[repr(C)]
struct PropVariantBlob {
    vt: u16,
    _w1: u16,
    _w2: u16,
    _w3: u16,
    blob: BLOB,
}

/// RAII COM apartment for the calling thread (MTA). Uninitializes on drop.
struct ComGuard;
impl ComGuard {
    fn new() -> Result<Self, AppAudioError> {
        // SAFETY: paired with CoUninitialize in Drop; MTA is fine for WASAPI.
        unsafe { CoInitializeEx(None, COINIT_MULTITHREADED).ok().map_err(backend)? };
        Ok(ComGuard)
    }
}
impl Drop for ComGuard {
    fn drop(&mut self) {
        // SAFETY: balances the CoInitializeEx in `new`.
        unsafe { CoUninitialize() };
    }
}

/// Resolve a pid to its executable file name (e.g. `chrome.exe`), best-effort.
fn exe_for_pid(pid: u32) -> Option<String> {
    if pid == 0 {
        return None;
    }
    // SAFETY: OpenProcess/QueryFullProcessImageNameW with a stack buffer; the
    // handle is closed on every path.
    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid).ok()?;
        let mut buf = [0u16; MAX_PATH as usize];
        let mut len = buf.len() as u32;
        let ok = QueryFullProcessImageNameW(
            handle,
            PROCESS_NAME_WIN32,
            windows::core::PWSTR(buf.as_mut_ptr()),
            &mut len,
        );
        let _ = CloseHandle(handle);
        if ok.is_err() || len == 0 {
            return None;
        }
        let full = String::from_utf16_lossy(&buf[..len as usize]);
        full.rsplit(['\\', '/']).next().map(|s| s.to_string())
    }
}

/// Enumerate the active render sessions on the default playback device and map
/// each to an [`AudioApp`]. Sessions with no pid (the system session) or an
/// unresolvable process are skipped; duplicates by pid are collapsed.
pub fn list_audio_apps() -> Result<Vec<AudioApp>, AppAudioError> {
    let _com = ComGuard::new()?;
    // SAFETY: a straight-line COM call chain; every interface is released by
    // the `windows` RAII wrappers when it drops.
    unsafe {
        let enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL).map_err(backend)?;
        let device = enumerator
            .GetDefaultAudioEndpoint(eRender, eConsole)
            .map_err(backend)?;
        let manager: IAudioSessionManager2 =
            device.Activate(CLSCTX_ALL, None).map_err(backend)?;
        let sessions: IAudioSessionEnumerator =
            manager.GetSessionEnumerator().map_err(backend)?;
        let count = sessions.GetCount().map_err(backend)?;

        let mut out: Vec<AudioApp> = Vec::new();
        for i in 0..count {
            let Ok(control) = sessions.GetSession(i) else {
                continue;
            };
            let Ok(control2) = control.cast::<IAudioSessionControl2>() else {
                continue;
            };
            // Skip the system-sounds session and anything not currently active.
            if control2.IsSystemSoundsSession().is_ok() {
                continue;
            }
            if control2.GetState().unwrap_or(AudioSessionStateActive) != AudioSessionStateActive {
                continue;
            }
            let pid = control2.GetProcessId().unwrap_or(0);
            if pid == 0 || out.iter().any(|a| a.pid == pid) {
                continue;
            }
            let exe = exe_for_pid(pid).unwrap_or_else(|| format!("pid {pid}"));
            let name = exe.strip_suffix(".exe").unwrap_or(&exe).to_string();
            out.push(AudioApp { pid, name, exe });
        }
        out.sort_by_key(|a| a.name.to_lowercase());
        Ok(out)
    }
}

/// The completion handler `ActivateAudioInterfaceAsync` calls back on. It only
/// signals an event; the caller then reads the result synchronously.
#[implement(IActivateAudioInterfaceCompletionHandler)]
struct ActivateHandler {
    done: HANDLE,
}

impl IActivateAudioInterfaceCompletionHandler_Impl for ActivateHandler_Impl {
    fn ActivateCompleted(
        &self,
        _op: Option<&IActivateAudioInterfaceAsyncOperation>,
    ) -> windows::core::Result<()> {
        // SAFETY: `done` is a valid event handle owned by the caller for the
        // duration of the async activation.
        unsafe {
            let _ = SetEvent(self.done);
        }
        Ok(())
    }
}

/// A running per-app capture. Dropping it stops the WASAPI stream and joins the
/// capture thread.
pub struct ProcessCapture {
    stop: Arc<AtomicBool>,
    wake: HANDLE,
    thread: Option<JoinHandle<()>>,
}

impl ProcessCapture {
    pub fn start(
        pid: u32,
        mut on_frames: impl FnMut(&[f32], u32, u16) + Send + 'static,
    ) -> Result<Self, AppAudioError> {
        let stop = Arc::new(AtomicBool::new(false));
        // An auto-reset event the capture thread waits on; SetEvent both from
        // WASAPI (buffer ready) and from `stop()` (to wake the wait promptly).
        // SAFETY: CreateEventW with defaults; closed when the thread exits.
        let wake = unsafe {
            CreateEventW(None, false, false, PCWSTR::null()).map_err(backend)?
        };
        let stop_thread = stop.clone();
        let wake_send = SendHandle(wake);
        let thread = std::thread::Builder::new()
            .name(format!("appaudio-{pid}"))
            .spawn(move || {
                // Bind the whole SendHandle first so the closure captures it
                // (Send), not the bare HANDLE field (Rust 2021 disjoint capture).
                let wake = wake_send;
                if let Err(e) = capture_loop(pid, wake.0, &stop_thread, &mut on_frames) {
                    tracing_warn(&e);
                }
            })
            .map_err(backend)?;
        Ok(Self {
            stop,
            wake,
            thread: Some(thread),
        })
    }
}

fn tracing_warn(e: &AppAudioError) {
    // fcap-appaudio has no tracing dep; a plain eprintln keeps the failure
    // visible in logs without pulling a logging crate into the unsafe island.
    eprintln!("fcap-appaudio: capture ended: {e}");
}

impl Drop for ProcessCapture {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        // SAFETY: wake the capture thread's wait so it observes `stop`.
        unsafe {
            let _ = SetEvent(self.wake);
        }
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
        // SAFETY: the thread has exited; the event handle is ours to close.
        unsafe {
            let _ = CloseHandle(self.wake);
        }
    }
}

/// The whole WASAPI process-loopback lifecycle on the capture thread.
fn capture_loop(
    pid: u32,
    wake: HANDLE,
    stop: &AtomicBool,
    on_frames: &mut (impl FnMut(&[f32], u32, u16) + Send),
) -> Result<(), AppAudioError> {
    let _com = ComGuard::new()?;

    // The activation params blob: capture this process tree's render audio.
    let mut params = AUDIOCLIENT_ACTIVATION_PARAMS {
        ActivationType: AUDIOCLIENT_ACTIVATION_TYPE_PROCESS_LOOPBACK,
        Anonymous: AUDIOCLIENT_ACTIVATION_PARAMS_0 {
            ProcessLoopbackParams: AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
                TargetProcessId: pid,
                ProcessLoopbackMode: PROCESS_LOOPBACK_MODE_INCLUDE_TARGET_PROCESS_TREE,
            },
        },
    };

    // Wrap the blob in a PROPVARIANT (VT_BLOB) for ActivateAudioInterfaceAsync.
    // `pv` and `params` must both outlive the synchronous activation below.
    let pv = PropVariantBlob {
        vt: VT_BLOB.0,
        _w1: 0,
        _w2: 0,
        _w3: 0,
        blob: BLOB {
            cbSize: size_of::<AUDIOCLIENT_ACTIVATION_PARAMS>() as u32,
            pBlobData: &mut params as *mut _ as *mut u8,
        },
    };

    // SAFETY: the event is valid for the whole activation; the handler holds it.
    let done = unsafe { CreateEventW(None, false, false, PCWSTR::null()).map_err(backend)? };
    let handler: IActivateAudioInterfaceCompletionHandler = ActivateHandler { done }.into();

    // SAFETY: ActivateAudioInterfaceAsync with the process-loopback virtual
    // device id; we wait on `done`, then read the result. The `pv` pointer is a
    // layout-correct C PROPVARIANT the FFI only reads as a raw pointer.
    let audio_client: IAudioClient = unsafe {
        let op = ActivateAudioInterfaceAsync(
            VIRTUAL_AUDIO_DEVICE_PROCESS_LOOPBACK,
            &IAudioClient::IID,
            Some(&pv as *const PropVariantBlob as *const windows_core::PROPVARIANT),
            &handler,
        )
        .map_err(backend)?;
        WaitForSingleObject(done, INFINITE);
        let _ = CloseHandle(done);

        let mut hr = windows::core::HRESULT(0);
        let mut activated: Option<windows::core::IUnknown> = None;
        op.GetActivateResult(&mut hr, &mut activated).map_err(backend)?;
        hr.ok().map_err(backend)?;
        activated
            .ok_or_else(|| AppAudioError::Backend("process loopback returned no interface".into()))?
            .cast()
            .map_err(backend)?
    };

    // We choose the format (there is no mix format for the virtual device).
    let format = WAVEFORMATEX {
        wFormatTag: WAVE_FORMAT_IEEE_FLOAT,
        nChannels: CHANNELS,
        nSamplesPerSec: SAMPLE_RATE,
        nAvgBytesPerSec: SAMPLE_RATE * CHANNELS as u32 * 4,
        nBlockAlign: CHANNELS * 4,
        wBitsPerSample: 32,
        cbSize: 0,
    };

    // SAFETY: a standard event-driven loopback capture init on `audio_client`.
    unsafe {
        audio_client
            .Initialize(
                AUDCLNT_SHAREMODE_SHARED,
                AUDCLNT_STREAMFLAGS_LOOPBACK | AUDCLNT_STREAMFLAGS_EVENTCALLBACK,
                2_000_000, // 200 ms buffer, in 100-ns units
                0,
                &format,
                None,
            )
            .map_err(backend)?;
        audio_client.SetEventHandle(wake).map_err(backend)?;
        let capture: IAudioCaptureClient = audio_client.GetService().map_err(backend)?;
        audio_client.Start().map_err(backend)?;

        let frame_floats = CHANNELS as usize;
        while !stop.load(Ordering::SeqCst) {
            if WaitForSingleObject(wake, INFINITE) != WAIT_OBJECT_0 {
                break;
            }
            if stop.load(Ordering::SeqCst) {
                break;
            }
            loop {
                let mut data: *mut u8 = std::ptr::null_mut();
                let mut frames: u32 = 0;
                let mut flags: u32 = 0;
                if capture
                    .GetBuffer(&mut data, &mut frames, &mut flags, None, None)
                    .is_err()
                    || frames == 0
                {
                    break;
                }
                if flags & (AUDCLNT_BUFFERFLAGS_SILENT.0 as u32) == 0 && !data.is_null() {
                    let samples =
                        slice::from_raw_parts(data as *const f32, frames as usize * frame_floats);
                    on_frames(samples, SAMPLE_RATE, CHANNELS);
                }
                let _ = capture.ReleaseBuffer(frames);
            }
        }
        let _ = audio_client.Stop();
    }
    Ok(())
}
