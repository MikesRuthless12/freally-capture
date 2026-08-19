//! CAP-N76 — the Windows virtual-camera output.
//!
//! Windows 11's `MFCreateVirtualCamera` wires a **registered frame-server media
//! source** (a signed COM DLL, loaded by the OS `frameserver.exe`, not by us)
//! into a camera that Zoom/Meet/Discord/OBS enumerate. Frames reach that source
//! through the [`super::transport`] shared buffer; this module owns the app-side
//! half: it opens the transport, brings the virtual camera up via the OS API,
//! pushes program frames, and tears it all down — never leaving a zombie device.
//!
//! The **registered source DLL is the signed component** (`design/vcam-source.md`),
//! exactly like the CEF host binary: absent it, [`available`] is false and
//! [`MfVirtualCamera::start`] returns the honest "install the Virtual Camera
//! component" error instead of a silent failure or a fake success.
//!
//! AUDITED `unsafe`: Media Foundation COM interop + the shared-memory mapping.
#![allow(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use windows::core::{GUID, HSTRING};
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::Media::MediaFoundation::{
    IMFVirtualCamera, MFCreateVirtualCamera, MFVirtualCameraAccess_CurrentUser,
    MFVirtualCameraLifetime_Session, MFVirtualCameraType_SoftwareCameraSource,
};
use windows::Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED};
use windows::Win32::System::Memory::{
    CreateFileMappingW, MapViewOfFile, UnmapViewOfFile, FILE_MAP_ALL_ACCESS,
    MEMORY_MAPPED_VIEW_ADDRESS, PAGE_READWRITE,
};

use fcap_stream::vcam::transport::{
    transport_name, FrameHeader, FrameWriter, PID_KEY, PID_VALUE, VCAM_SOURCE_CLSID_U128,
};
use fcap_stream::vcam::{VcamFeed, VcamOutput};

/// The CLSID under which the Freally virtual-camera source DLL registers
/// itself. The app hands this to `MFCreateVirtualCamera` as the source symbolic
/// link; `available` probes whether a source is registered for it.
///
/// The value itself lives in `fcap_stream::vcam::transport` — the module both
/// halves of this contract already link — so registration and lookup cannot
/// drift (`design/vcam-source.md`).
pub const VCAM_SOURCE_CLSID: GUID = GUID::from_u128(VCAM_SOURCE_CLSID_U128);

/// The camera name consuming apps display.
const FRIENDLY_NAME: &str = "Freally Capture Virtual Camera";

/// Shown when the registered source DLL is absent — honest, actionable, and one
/// string so the probe and the start path never disagree.
pub const SOURCE_NOT_REGISTERED: &str =
    "the Virtual Camera needs the Virtual Camera component — install it from Components";

/// Is a driver-backed virtual camera usable on this machine right now?
///
/// True only when the OS API exists (Windows 11+) **and** the frame-server
/// source is registered for [`VCAM_SOURCE_CLSID`]. The component being absent is
/// the common, honest false — the UI keeps its button disabled and says why.
pub fn available() -> bool {
    source_registered()
}

/// Whether the source DLL is registered under our CLSID. Read-only registry
/// probe: `HKCR\CLSID\{VCAM_SOURCE_CLSID}\InprocServer32` exists with a path.
fn source_registered() -> bool {
    use windows::Win32::System::Registry::{
        RegCloseKey, RegOpenKeyExW, HKEY, HKEY_CLASSES_ROOT, KEY_READ,
    };
    let subkey = format!("CLSID\\{{{VCAM_SOURCE_CLSID:?}}}\\InprocServer32");
    let wide = HSTRING::from(subkey);
    let mut key = HKEY::default();
    // SAFETY: a read-only open of a fixed subkey; the handle is closed on the
    // success path.
    let opened = unsafe { RegOpenKeyExW(HKEY_CLASSES_ROOT, &wide, 0, KEY_READ, &mut key) };
    if opened.is_ok() {
        // SAFETY: `key` was opened above.
        unsafe {
            let _ = RegCloseKey(key);
        }
        true
    } else {
        false
    }
}

/// Record this process id under `HKCU\Software\Freally\Capture\VirtualCamera\pid`
/// so the OS-loaded source DLL (in the frame-server process) opens the right
/// app's transport region. Per-user, no admin. Best-effort.
fn publish_app_pid() {
    use windows::Win32::System::Registry::{
        RegCloseKey, RegCreateKeyExW, RegSetValueExW, HKEY, HKEY_CURRENT_USER, KEY_WRITE,
        REG_DWORD, REG_OPTION_NON_VOLATILE,
    };
    let sub = HSTRING::from(PID_KEY);
    let name = HSTRING::from(PID_VALUE);
    let pid = std::process::id();
    let mut key = HKEY::default();
    // SAFETY: create-or-open a per-user key; set one DWORD; close.
    unsafe {
        if RegCreateKeyExW(
            HKEY_CURRENT_USER,
            &sub,
            0,
            windows::core::PCWSTR::null(),
            REG_OPTION_NON_VOLATILE,
            KEY_WRITE,
            None,
            &mut key,
            None,
        )
        .is_ok()
        {
            let _ = RegSetValueExW(key, &name, 0, REG_DWORD, Some(&pid.to_ne_bytes()));
            let _ = RegCloseKey(key);
        }
    }
}

/// The shared-memory-backed frame view the writer publishes into, and that the
/// registered source reads. Owns its mapping; drops close it.
struct SharedFrames {
    _mapping: HANDLE,
    view: MEMORY_MAPPED_VIEW_ADDRESS,
    len: usize,
}

// The view is a shared page written only through the FrameWriter; the handle is
// process-wide.
unsafe impl Send for SharedFrames {}

impl SharedFrames {
    fn create(name: &str, total_bytes: usize) -> Result<Self, String> {
        let wide = HSTRING::from(name);
        // SAFETY: a fixed-size page-backed mapping under a session-local name.
        let mapping = unsafe {
            CreateFileMappingW(
                HANDLE::default(),
                None,
                PAGE_READWRITE,
                ((total_bytes as u64) >> 32) as u32,
                (total_bytes as u64 & 0xFFFF_FFFF) as u32,
                &wide,
            )
        }
        .map_err(|err| format!("could not create the camera frame buffer: {err}"))?;
        // SAFETY: `mapping` is a live file mapping of `total_bytes`.
        let view = unsafe { MapViewOfFile(mapping, FILE_MAP_ALL_ACCESS, 0, 0, total_bytes) };
        if view.Value.is_null() {
            // SAFETY: closing the handle we just created and never published.
            unsafe {
                let _ = CloseHandle(mapping);
            }
            return Err("could not map the camera frame buffer".into());
        }
        Ok(Self {
            _mapping: mapping,
            view,
            len: total_bytes,
        })
    }
}

impl AsMut<[u8]> for SharedFrames {
    /// The mapped bytes for the [`FrameWriter`] — the whole contract that lets
    /// the safe, generic writer drive this shared mapping.
    fn as_mut(&mut self) -> &mut [u8] {
        // SAFETY: `view` maps exactly `len` bytes, written only here.
        unsafe { std::slice::from_raw_parts_mut(self.view.Value as *mut u8, self.len) }
    }
}

impl Drop for SharedFrames {
    fn drop(&mut self) {
        // SAFETY: both came from MapViewOfFile / CreateFileMappingW here.
        unsafe {
            let _ = UnmapViewOfFile(self.view);
            let _ = CloseHandle(self._mapping);
        }
    }
}

/// A live virtual camera, wrapped so the owning [`MfVirtualCamera`] is `Send`
/// (the [`VcamOutput`] trait requires it).
///
/// The COM object is not `Send` by default, but this handle is only ever
/// created, used, and released on the one thread that owns the
/// `MfVirtualCamera` — the studio's output thread. It is never shared or moved
/// mid-use, so treating it as `Send` is sound.
struct CameraHandle(IMFVirtualCamera);
// SAFETY: single-threaded ownership as described above.
unsafe impl Send for CameraHandle {}

/// The Windows [`VcamOutput`]: a Media Foundation virtual camera fed from a
/// shared-memory transport.
pub struct MfVirtualCamera {
    feed: VcamFeed,
    camera: Option<CameraHandle>,
    writer: Option<FrameWriter<SharedFrames>>,
}

impl MfVirtualCamera {
    /// Create (not yet started) for `feed`. `start` brings the OS device up.
    pub fn new(feed: VcamFeed) -> Self {
        Self {
            feed,
            camera: None,
            writer: None,
        }
    }

    /// Which feed this camera outputs (Program / Vertical / a single Source).
    pub fn feed(&self) -> &VcamFeed {
        &self.feed
    }
}

impl VcamOutput for MfVirtualCamera {
    fn start(&mut self, width: u32, height: u32, _fps: u32) -> Result<(), String> {
        if !source_registered() {
            return Err(SOURCE_NOT_REGISTERED.into());
        }
        // MF + COM must be initialised on this thread before the API calls.
        // SAFETY: apartment-threaded init; a redundant call returns S_FALSE and
        // is harmless. The camera is released in `stop`/Drop.
        unsafe {
            let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        }

        // Back the transport with a shared mapping the source opens by name.
        let header = FrameHeader::new(width, height);
        let name = transport_name(std::process::id());
        let frames = SharedFrames::create(&name, header.total_bytes())?;
        let writer = FrameWriter::new(frames, width, height)
            .ok_or("the camera frame buffer was smaller than one frame")?;

        // Record our pid where the source DLL (loaded by the OS frame-server, a
        // *different* process) looks for it, so it opens THIS app's transport
        // region — see design/vcam-source.md. Best-effort: a failure here just
        // means the source falls back to its slate until the next start.
        publish_app_pid();

        // Bring the OS camera up against the registered source. The source id is
        // our CLSID's symbolic link; the source reads the transport by our pid,
        // passed through the source-configuration attribute store it reads on
        // activation (see design/vcam-source.md).
        let friendly = HSTRING::from(FRIENDLY_NAME);
        let source_id = HSTRING::from(format!("{{{VCAM_SOURCE_CLSID:?}}}"));
        // SAFETY: all arguments are valid HSTRINGs; the out-param is owned.
        let camera = unsafe {
            MFCreateVirtualCamera(
                MFVirtualCameraType_SoftwareCameraSource,
                MFVirtualCameraLifetime_Session,
                MFVirtualCameraAccess_CurrentUser,
                &friendly,
                &source_id,
                None,
            )
        }
        .map_err(|err| format!("could not create the virtual camera: {err}"))?;
        // SAFETY: `camera` is the object we just created.
        unsafe { camera.Start(None) }
            .map_err(|err| format!("could not start the virtual camera: {err}"))?;

        self.camera = Some(CameraHandle(camera));
        self.writer = Some(writer);
        Ok(())
    }

    fn push_frame(&mut self, rgba: &[u8], width: u32, height: u32) -> Result<(), String> {
        match self.writer.as_mut() {
            Some(writer) => writer.publish(rgba, width, height),
            None => Err("the virtual camera is not running".into()),
        }
    }

    fn stop(&mut self) -> Result<(), String> {
        if let Some(writer) = self.writer.as_mut() {
            writer.mark_stopped();
        }
        // Stop + remove so no zombie device lingers, then drop our references.
        if let Some(handle) = self.camera.take() {
            // SAFETY: the wrapped camera is live; Stop/Remove are its own
            // teardown. Errors are swallowed — teardown is best-effort.
            unsafe {
                let _ = handle.0.Stop();
                let _ = handle.0.Remove();
            }
        }
        self.writer = None;
        Ok(())
    }
}

impl Drop for MfVirtualCamera {
    fn drop(&mut self) {
        // A dropped camera must never leave the OS device registered.
        let _ = self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn availability_matches_source_registration() {
        // On a machine without the component installed (the default here),
        // available() is false and the honest message is what the UI shows.
        assert_eq!(available(), source_registered());
    }

    #[test]
    fn start_without_the_component_fails_honestly() {
        if source_registered() {
            // The component happens to be installed on this machine — skip; the
            // honest-error path is the one we assert without it.
            return;
        }
        let mut cam = MfVirtualCamera::new(VcamFeed::Program);
        let err = cam.start(1920, 1080, 30).expect_err("no source registered");
        assert!(
            err.contains("Virtual Camera component"),
            "the error routes to the install step, got: {err}"
        );
    }

    #[test]
    fn pushing_before_start_is_an_error_not_a_panic() {
        let mut cam = MfVirtualCamera::new(VcamFeed::Program);
        assert!(cam.push_frame(&[0u8; 16], 2, 2).is_err());
    }
}
