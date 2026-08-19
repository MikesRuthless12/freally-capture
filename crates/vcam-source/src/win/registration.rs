//! The COM plumbing the OS calls: the class factory, the DLL entry points
//! (`DllGetClassObject`, `DllCanUnloadNow`), and per-user self-registration
//! (`DllRegisterServer` / `DllUnregisterServer` writing under
//! `HKCU\Software\Classes` — no admin).
//!
//! AUDITED `unsafe`: the `extern "system"` COM DLL exports + registry writes.

use std::ffi::c_void;
use std::sync::atomic::{AtomicUsize, Ordering};

// The app records its pid under `PID_KEY`/`PID_VALUE` (per-user) — both come
// from the shared transport contract both halves link.
use fcap_stream::vcam::transport::{PID_KEY, PID_VALUE};

use windows::core::{implement, IUnknown, Interface, GUID, HRESULT, PCWSTR};
use windows::Win32::Foundation::{
    BOOL, CLASS_E_CLASSNOTAVAILABLE, E_NOINTERFACE, E_POINTER, HMODULE, S_FALSE, S_OK,
};
use windows::Win32::System::Com::{IClassFactory, IClassFactory_Impl};
use windows::Win32::System::Registry::{
    RegCloseKey, RegCreateKeyExW, RegDeleteTreeW, RegSetValueExW, HKEY, HKEY_CURRENT_USER,
    KEY_WRITE, REG_OPTION_NON_VOLATILE, REG_SZ,
};

use crate::win::source::VcamSource;
use crate::win::VCAM_SOURCE_CLSID;

/// The module handle, captured in `DllMain`, used to resolve this DLL's own path
/// for registration.
static mut MODULE: HMODULE = HMODULE(std::ptr::null_mut());

/// The app pid the source reads frames from. The app passes it to
/// `MFCreateVirtualCamera` via the source's configuration; until that plumbing
/// lands we read the single most-recent writer (pid 0 = "resolve at open").
/// Kept as a seam so the source and the app agree on how the pid arrives.
fn configured_app_pid() -> u32 {
    // v1: the app is the process that created the newest transport region; the
    // source resolves it by scanning is out of scope, so the app writes its pid
    // into a well-known per-user value on Start. Default 0 falls back to the
    // default geometry + slate until the app publishes.
    read_app_pid().unwrap_or(0)
}

fn read_app_pid() -> Option<u32> {
    use windows::Win32::System::Registry::{RegOpenKeyExW, RegQueryValueExW, KEY_READ, REG_DWORD};
    let sub = wide(PID_KEY);
    let mut key = HKEY::default();
    // SAFETY: read-only open of a fixed per-user subkey.
    if unsafe {
        RegOpenKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR(sub.as_ptr()),
            0,
            KEY_READ,
            &mut key,
        )
    }
    .is_err()
    {
        return None;
    }
    let name = wide(PID_VALUE);
    let mut ty = REG_DWORD;
    let mut data = 0u32;
    let mut len = 4u32;
    // SAFETY: reading a 4-byte DWORD into `data`.
    let r = unsafe {
        RegQueryValueExW(
            key,
            PCWSTR(name.as_ptr()),
            None,
            Some(&mut ty),
            Some(&mut data as *mut u32 as *mut u8),
            Some(&mut len),
        )
    };
    // SAFETY: closing the key we opened.
    unsafe {
        let _ = RegCloseKey(key);
    }
    r.is_ok().then_some(data)
}

/// UTF-16 NUL-terminated, for the registry APIs.
fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

/// The class factory that hands out `IMFMediaSource` instances.
#[implement(IClassFactory)]
struct Factory;

impl IClassFactory_Impl for Factory_Impl {
    fn CreateInstance(
        &self,
        punkouter: Option<&IUnknown>,
        riid: *const GUID,
        ppvobject: *mut *mut c_void,
    ) -> windows::core::Result<()> {
        if ppvobject.is_null() {
            return Err(E_POINTER.into());
        }
        // SAFETY: `ppvobject` is non-null per the check.
        unsafe { *ppvobject = std::ptr::null_mut() };
        if punkouter.is_some() {
            // No aggregation.
            return Err(windows::Win32::Foundation::CLASS_E_NOAGGREGATION.into());
        }
        let source = VcamSource::create(configured_app_pid())?;
        // SAFETY: query the requested interface into the out-param.
        unsafe { source.query(&*riid, ppvobject).ok() }
    }

    fn LockServer(&self, flock: BOOL) -> windows::core::Result<()> {
        // A real count — see DllCanUnloadNow. saturating_sub semantics via
        // fetch_update so an unbalanced release can never wrap to usize::MAX
        // and pin the DLL in memory forever.
        if flock.as_bool() {
            LOCK_COUNT.fetch_add(1, Ordering::SeqCst);
        } else {
            let _ = LOCK_COUNT.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |n| {
                Some(n.saturating_sub(1))
            });
        }
        Ok(())
    }
}

/// `DllMain` — capture the module handle for registration; nothing heavy.
///
/// # Safety
/// Standard `DllMain` ABI.
#[no_mangle]
pub unsafe extern "system" fn DllMain(module: HMODULE, reason: u32, _reserved: *mut c_void) -> i32 {
    if reason == windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH {
        // SAFETY: single write at load, before any other export runs.
        unsafe { MODULE = module };
    }
    1
}

/// `DllGetClassObject` — the Frame Server calls this to get our class factory.
///
/// # Safety
/// COM export ABI; `riid`/`ppv` are valid per the COM contract.
#[no_mangle]
pub unsafe extern "system" fn DllGetClassObject(
    rclsid: *const GUID,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    // SAFETY: non-null per the check.
    unsafe { *ppv = std::ptr::null_mut() };
    // SAFETY: `rclsid` is a valid pointer per the ABI.
    if unsafe { *rclsid } != VCAM_SOURCE_CLSID {
        return CLASS_E_CLASSNOTAVAILABLE;
    }
    let factory: IClassFactory = Factory.into();
    // SAFETY: hand the requested interface back.
    match unsafe { factory.query(&*riid, ppv) } {
        S_OK => S_OK,
        _ => E_NOINTERFACE,
    }
}

/// `DllCanUnloadNow` — `S_OK` only when nothing we handed out is still live.
///
/// This export **is** the mechanism COM uses to ask whether the DLL may be
/// unloaded; answering `S_OK` unconditionally means a `CoFreeUnusedLibraries`
/// in the frame-server host can unload us while a consuming app still holds an
/// `IMFMediaSource`, leaving live vtable pointers into freed memory. So it has
/// to be a real count: objects we created, plus outstanding `LockServer` locks.
///
/// # Safety
/// COM export ABI.
#[no_mangle]
pub unsafe extern "system" fn DllCanUnloadNow() -> HRESULT {
    if OBJECT_COUNT.load(Ordering::SeqCst) == 0 && LOCK_COUNT.load(Ordering::SeqCst) == 0 {
        S_OK
    } else {
        S_FALSE
    }
}

/// Live objects this DLL has handed out (see [`DllCanUnloadNow`]).
pub(crate) static OBJECT_COUNT: AtomicUsize = AtomicUsize::new(0);
/// Outstanding `IClassFactory::LockServer` locks.
static LOCK_COUNT: AtomicUsize = AtomicUsize::new(0);

/// `DllRegisterServer` — per-user self-registration (no admin).
///
/// # Safety
/// COM export ABI.
#[no_mangle]
pub unsafe extern "system" fn DllRegisterServer() -> HRESULT {
    match register(true) {
        Ok(()) => S_OK,
        Err(e) => e.into(),
    }
}

/// `DllUnregisterServer` — remove the per-user registration.
///
/// # Safety
/// COM export ABI.
#[no_mangle]
pub unsafe extern "system" fn DllUnregisterServer() -> HRESULT {
    match register(false) {
        Ok(()) => S_OK,
        Err(e) => e.into(),
    }
}

/// The DLL's own path, for the `InprocServer32` value.
fn module_path() -> windows::core::Result<String> {
    use windows::Win32::System::LibraryLoader::GetModuleFileNameW;
    let mut buf = [0u16; 512];
    // SAFETY: MODULE was set in DllMain; the buffer is sized.
    let len = unsafe { GetModuleFileNameW(MODULE, &mut buf) };
    if len == 0 {
        return Err(windows::core::Error::from_win32());
    }
    Ok(String::from_utf16_lossy(&buf[..len as usize]))
}

/// Write (or delete) the per-user COM registration:
/// `HKCU\Software\Classes\CLSID\{clsid}\InprocServer32` = this DLL, threading
/// model "Both".
fn register(install: bool) -> windows::core::Result<()> {
    let clsid_key = format!("Software\\Classes\\CLSID\\{{{VCAM_SOURCE_CLSID:?}}}");

    if !install {
        let sub = wide(&clsid_key);
        // SAFETY: deletes the subtree we created; missing key is not fatal.
        unsafe {
            let _ = RegDeleteTreeW(HKEY_CURRENT_USER, PCWSTR(sub.as_ptr()));
        }
        return Ok(());
    }

    let dll = module_path()?;
    // CLSID root: friendly name.
    write_value(&clsid_key, None, "Freally Capture Virtual Camera Source")?;
    // InprocServer32: the DLL path + threading model.
    let inproc = format!("{clsid_key}\\InprocServer32");
    write_value(&inproc, None, &dll)?;
    write_value(&inproc, Some("ThreadingModel"), "Both")?;
    Ok(())
}

/// Create `subkey` under HKCU and set its (default or named) string value.
fn write_value(subkey: &str, name: Option<&str>, value: &str) -> windows::core::Result<()> {
    let sub = wide(subkey);
    let mut key = HKEY::default();
    // SAFETY: create-or-open a per-user key; closed below.
    unsafe {
        RegCreateKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR(sub.as_ptr()),
            0,
            PCWSTR::null(),
            REG_OPTION_NON_VOLATILE,
            KEY_WRITE,
            None,
            &mut key,
            None,
        )
        .ok()?;
    }
    let data = wide(value);
    // SAFETY: reinterpret the wide string as its byte image for a REG_SZ write.
    let bytes = unsafe {
        std::slice::from_raw_parts(data.as_ptr() as *const u8, std::mem::size_of_val(&data[..]))
    };
    let name_w = name.map(wide);
    let name_ptr = name_w
        .as_ref()
        .map(|w| PCWSTR(w.as_ptr()))
        .unwrap_or(PCWSTR::null());
    // SAFETY: writing a REG_SZ; `bytes` covers the wide string incl. NUL.
    let result = unsafe { RegSetValueExW(key, name_ptr, 0, REG_SZ, Some(bytes)) };
    // SAFETY: closing the key we created.
    unsafe {
        let _ = RegCloseKey(key);
    }
    result.ok()
}
