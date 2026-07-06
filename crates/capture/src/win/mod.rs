//! Windows capture: DXGI Desktop Duplication for displays (`dxgi`),
//! Windows.Graphics.Capture for individual windows (`wgc`).
//!
//! AUDITED `unsafe`: this module tree is the crate's isolated Windows FFI
//! surface — Win32/COM/WinRT calls plus the raw-pointer plumbing they demand
//! (EnumWindows callback state, mapped-texture reads). Every `unsafe` block
//! stays small and local; nothing else in the crate may use `unsafe`.
#![allow(unsafe_code)]

pub(crate) mod dxgi;
pub(crate) mod pointer;
pub(crate) mod wgc;

use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use windows::core::{Interface, PWSTR};
use windows::Win32::Foundation::{CloseHandle, FALSE, HWND, LPARAM, RECT, TRUE};
use windows::Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE_HARDWARE, D3D_DRIVER_TYPE_UNKNOWN};
use windows::Win32::Graphics::Direct3D11::{
    D3D11CreateDevice, ID3D11Device, ID3D11DeviceContext, D3D11_CREATE_DEVICE_BGRA_SUPPORT,
    D3D11_SDK_VERSION,
};
use windows::Win32::Graphics::Dwm::{DwmGetWindowAttribute, DWMWA_CLOAKED};
use windows::Win32::Graphics::Dxgi::{
    CreateDXGIFactory1, IDXGIAdapter1, IDXGIFactory1, IDXGIOutput, DXGI_ERROR_NOT_FOUND,
    DXGI_OUTPUT_DESC,
};
use windows::Win32::System::Threading::{
    OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION,
};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetClassNameW, GetClientRect, GetWindowLongPtrW, GetWindowPlacement,
    GetWindowTextLengthW, GetWindowTextW, GetWindowThreadProcessId, IsIconic, IsWindow,
    IsWindowVisible, GWL_EXSTYLE, WINDOWPLACEMENT, WS_EX_TOOLWINDOW,
};

use crate::window_match::{
    decode_window_id, encode_window_id, resolve_best, same_window, WindowKey,
};
use crate::{frame_channel, CaptureError, CaptureSession, SourceInfo, SourceKind};

const DISPLAY_PREFIX: &str = "display:";
const WINDOW_PREFIX: &str = "window:";

pub(crate) fn list_sources() -> Result<Vec<SourceInfo>, CaptureError> {
    let mut sources = list_displays()?;
    sources.extend(list_windows());
    Ok(sources)
}

pub(crate) fn start_capture(id: &str) -> Result<CaptureSession, CaptureError> {
    if let Some(device_name) = id.strip_prefix(DISPLAY_PREFIX) {
        // Validate now so "unplugged display" fails fast; the thread re-finds
        // the output by name (indices can shift between calls).
        find_output_by_name(device_name)?;
        let device_name = device_name.to_owned();
        let (sender, receiver) = frame_channel();
        let stop = Arc::new(AtomicBool::new(false));
        let stop_thread = Arc::clone(&stop);
        let join = std::thread::Builder::new()
            .name("fcap-dxgi".into())
            .spawn(move || dxgi::run(&device_name, sender, stop_thread))
            .map_err(|err| CaptureError::Backend(format!("could not spawn capture: {err}")))?;
        return Ok(CaptureSession::from_parts(receiver, stop, join));
    }
    if let Some(raw) = id.strip_prefix(WINDOW_PREFIX) {
        let (stored_hwnd, key) = decode_window_id(raw)
            .ok_or_else(|| CaptureError::NotFound(format!("bad window id: {id}")))?;
        // A HWND is only valid within the session it was picked in — a handle
        // is process-lifetime and Windows recycles the integers. Across an app
        // (or target-app) restart the stored one is stale, so re-bind to the
        // *same* window by its durable identity (executable + class + title),
        // the way OBS re-attaches a Window Capture on launch.
        let hwnd_raw = resolve_target_hwnd(stored_hwnd, &key)
            .ok_or_else(|| CaptureError::NotFound("the window is no longer open".into()))?;
        let (sender, receiver) = frame_channel();
        let stop = Arc::new(AtomicBool::new(false));
        let stop_thread = Arc::clone(&stop);
        let join = std::thread::Builder::new()
            .name("fcap-wgc".into())
            .spawn(move || wgc::run(hwnd_raw, sender, stop_thread))
            .map_err(|err| CaptureError::Backend(format!("could not spawn capture: {err}")))?;
        return Ok(CaptureSession::from_parts(receiver, stop, join));
    }
    Err(CaptureError::NotFound(format!("unknown source id: {id}")))
}

// ---------------------------------------------------------------------------
// Displays (DXGI enumeration)
// ---------------------------------------------------------------------------

fn utf16_trimmed(raw: &[u16]) -> String {
    let len = raw.iter().position(|&c| c == 0).unwrap_or(raw.len());
    String::from_utf16_lossy(&raw[..len])
}

fn list_displays() -> Result<Vec<SourceInfo>, CaptureError> {
    let mut displays = Vec::new();
    // SAFETY: plain DXGI factory/adapter/output enumeration; every interface
    // is owned by windows-rs wrappers that release on drop.
    unsafe {
        let factory: IDXGIFactory1 = CreateDXGIFactory1()
            .map_err(|err| CaptureError::Backend(format!("DXGI factory: {err}")))?;
        let mut adapter_index = 0u32;
        let mut display_number = 1usize;
        loop {
            let adapter: IDXGIAdapter1 = match factory.EnumAdapters1(adapter_index) {
                Ok(adapter) => adapter,
                Err(err) if err.code() == DXGI_ERROR_NOT_FOUND => break,
                Err(err) => return Err(CaptureError::Backend(format!("EnumAdapters1: {err}"))),
            };
            let mut output_index = 0u32;
            loop {
                let output: IDXGIOutput = match adapter.EnumOutputs(output_index) {
                    Ok(output) => output,
                    Err(err) if err.code() == DXGI_ERROR_NOT_FOUND => break,
                    Err(_) => break,
                };
                if let Ok(desc) = output.GetDesc() {
                    if desc.AttachedToDesktop.as_bool() {
                        displays.push(display_source(&desc, display_number));
                        display_number += 1;
                    }
                }
                output_index += 1;
            }
            adapter_index += 1;
        }
    }
    Ok(displays)
}

fn display_source(desc: &DXGI_OUTPUT_DESC, number: usize) -> SourceInfo {
    let coords = desc.DesktopCoordinates;
    let width = (coords.right - coords.left).max(0) as u32;
    let height = (coords.bottom - coords.top).max(0) as u32;
    let primary = coords.left == 0 && coords.top == 0;
    let device_name = utf16_trimmed(&desc.DeviceName);
    SourceInfo {
        id: format!("{DISPLAY_PREFIX}{device_name}"),
        kind: SourceKind::Display,
        label: format!(
            "Display {number} — {width}×{height}{}",
            if primary { " (primary)" } else { "" }
        ),
        width,
        height,
    }
}

/// Locate the adapter + output pair whose `DXGI_OUTPUT_DESC.DeviceName`
/// matches; used both for validation and by the capture thread.
pub(crate) fn find_output_by_name(
    device_name: &str,
) -> Result<(IDXGIAdapter1, IDXGIOutput), CaptureError> {
    // SAFETY: same enumeration pattern as `list_displays`.
    unsafe {
        let factory: IDXGIFactory1 = CreateDXGIFactory1()
            .map_err(|err| CaptureError::Backend(format!("DXGI factory: {err}")))?;
        let mut adapter_index = 0u32;
        loop {
            let adapter: IDXGIAdapter1 = match factory.EnumAdapters1(adapter_index) {
                Ok(adapter) => adapter,
                Err(err) if err.code() == DXGI_ERROR_NOT_FOUND => break,
                Err(err) => return Err(CaptureError::Backend(format!("EnumAdapters1: {err}"))),
            };
            let mut output_index = 0u32;
            loop {
                let output: IDXGIOutput = match adapter.EnumOutputs(output_index) {
                    Ok(output) => output,
                    Err(_) => break,
                };
                if let Ok(desc) = output.GetDesc() {
                    if utf16_trimmed(&desc.DeviceName) == device_name {
                        return Ok((adapter, output));
                    }
                }
                output_index += 1;
            }
            adapter_index += 1;
        }
    }
    Err(CaptureError::NotFound(format!(
        "display {device_name} is no longer attached"
    )))
}

/// Create the D3D11 device (+ immediate context) captures copy through.
/// `adapter: None` = default hardware adapter (WGC); `Some` = the adapter the
/// duplicated output lives on (DXGI duplication requires same-adapter).
pub(crate) fn create_d3d_device(
    adapter: Option<&IDXGIAdapter1>,
) -> Result<(ID3D11Device, ID3D11DeviceContext), CaptureError> {
    let mut device: Option<ID3D11Device> = None;
    let mut context: Option<ID3D11DeviceContext> = None;
    let driver_type = if adapter.is_some() {
        D3D_DRIVER_TYPE_UNKNOWN
    } else {
        D3D_DRIVER_TYPE_HARDWARE
    };
    // SAFETY: out-params are locals; windows-rs owns the returned interfaces.
    unsafe {
        D3D11CreateDevice(
            adapter
                .map(|a| a.cast::<windows::Win32::Graphics::Dxgi::IDXGIAdapter>())
                .transpose()
                .map_err(|err| CaptureError::Backend(format!("adapter cast: {err}")))?
                .as_ref(),
            driver_type,
            windows::Win32::Foundation::HMODULE::default(),
            D3D11_CREATE_DEVICE_BGRA_SUPPORT,
            None,
            D3D11_SDK_VERSION,
            Some(&mut device),
            None,
            Some(&mut context),
        )
        .map_err(|err| CaptureError::Backend(format!("D3D11CreateDevice: {err}")))?;
    }
    match (device, context) {
        (Some(device), Some(context)) => Ok((device, context)),
        _ => Err(CaptureError::Backend(
            "D3D11CreateDevice returned no device".into(),
        )),
    }
}

// ---------------------------------------------------------------------------
// Windows (EnumWindows) — enumeration + durable re-resolution
// ---------------------------------------------------------------------------

/// One enumerated top-level window: the live handle plus its identity + size.
struct EnumWindow {
    hwnd: isize,
    key: WindowKey,
    width: u32,
    height: u32,
}

fn list_windows() -> Vec<SourceInfo> {
    enumerate_windows()
        .into_iter()
        .map(|w| SourceInfo {
            id: format!("{WINDOW_PREFIX}{}", encode_window_id(w.hwnd as u64, &w.key)),
            kind: SourceKind::Window,
            label: w.key.title.clone(),
            width: w.width,
            height: w.height,
        })
        .collect()
}

fn enumerate_windows() -> Vec<EnumWindow> {
    let mut windows_out: Vec<EnumWindow> = Vec::new();
    // SAFETY: the LPARAM carries a pointer to `windows_out`, which outlives
    // the synchronous EnumWindows call; the callback is the only writer.
    unsafe {
        let _ = EnumWindows(
            Some(enum_windows_cb),
            LPARAM(&mut windows_out as *mut Vec<EnumWindow> as isize),
        );
    }
    windows_out
}

unsafe extern "system" fn enum_windows_cb(
    hwnd: HWND,
    lparam: LPARAM,
) -> windows::Win32::Foundation::BOOL {
    // SAFETY: lparam is the Vec pointer passed by `enumerate_windows` above.
    let out = unsafe { &mut *(lparam.0 as *mut Vec<EnumWindow>) };

    if !unsafe { IsWindowVisible(hwnd) }.as_bool() {
        return TRUE;
    }
    // Tool windows (palettes, IME candidates…) aren't real capture targets.
    let ex_style = unsafe { GetWindowLongPtrW(hwnd, GWL_EXSTYLE) } as u32;
    if ex_style & WS_EX_TOOLWINDOW.0 != 0 {
        return TRUE;
    }
    // Skip DWM-cloaked windows (suspended UWP apps look "open" but are not).
    let mut cloaked: u32 = 0;
    let cloak_ok = unsafe {
        DwmGetWindowAttribute(
            hwnd,
            DWMWA_CLOAKED,
            &mut cloaked as *mut u32 as *mut core::ffi::c_void,
            std::mem::size_of::<u32>() as u32,
        )
    };
    if cloak_ok.is_ok() && cloaked != 0 {
        return TRUE;
    }
    let title = title_of(hwnd);
    if title.is_empty() {
        return TRUE;
    }

    let mut rect = RECT::default();
    let (mut width, mut height) = if unsafe { GetClientRect(hwnd, &mut rect) }.is_ok() {
        (
            (rect.right - rect.left).max(0) as u32,
            (rect.bottom - rect.top).max(0) as u32,
        )
    } else {
        (0, 0)
    };
    // A minimized window reports a 0×0 client rect. Fall back to its restored
    // (normal) size so it still appears in the picker and can be selected — the
    // list should include every open window, minimized or not.
    if (width == 0 || height == 0) && unsafe { IsIconic(hwnd) }.as_bool() {
        let mut placement = WINDOWPLACEMENT {
            length: std::mem::size_of::<WINDOWPLACEMENT>() as u32,
            ..Default::default()
        };
        if unsafe { GetWindowPlacement(hwnd, &mut placement) }.is_ok() {
            let normal = placement.rcNormalPosition;
            width = (normal.right - normal.left).max(0) as u32;
            height = (normal.bottom - normal.top).max(0) as u32;
        }
    }
    // Drop only genuinely sizeless windows (hidden helpers); a minimized window
    // now carries its restored size from the fallback above.
    if width == 0 || height == 0 {
        return TRUE;
    }

    let key = WindowKey::new(exe_of(hwnd), class_of(hwnd), title);
    out.push(EnumWindow {
        hwnd: hwnd.0 as isize,
        key,
        width,
        height,
    });
    TRUE
}

// -- per-window inspection ---------------------------------------------------

/// The window title (empty when it has none — the enumeration skips those).
fn title_of(hwnd: HWND) -> String {
    // SAFETY: length-then-copy, the standard GetWindowText pattern.
    let len = unsafe { GetWindowTextLengthW(hwnd) };
    if len <= 0 {
        return String::new();
    }
    let mut buf = vec![0u16; len as usize + 1];
    let copied = unsafe { GetWindowTextW(hwnd, &mut buf) };
    if copied <= 0 {
        return String::new();
    }
    String::from_utf16_lossy(&buf[..copied as usize])
}

/// The Win32 window class (empty on failure).
fn class_of(hwnd: HWND) -> String {
    let mut buf = [0u16; 256];
    // SAFETY: GetClassNameW writes up to buf.len()-1 UTF-16 units + a NUL.
    let len = unsafe { GetClassNameW(hwnd, &mut buf) };
    if len <= 0 {
        return String::new();
    }
    String::from_utf16_lossy(&buf[..len as usize])
}

/// The owning process's image *file name*, e.g. `chrome.exe` (empty when it
/// can't be read — a protected/elevated process we can't open; matching then
/// leans on class + title).
fn exe_of(hwnd: HWND) -> String {
    let mut pid: u32 = 0;
    // SAFETY: writes the local `pid` out-param; the thread-id return is unused.
    unsafe { GetWindowThreadProcessId(hwnd, Some(&mut pid)) };
    if pid == 0 {
        return String::new();
    }
    // SAFETY: a limited-query open; fails (handled) for protected processes.
    let Ok(handle) = (unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, FALSE, pid) }) else {
        return String::new();
    };
    // Roomy enough for deep install paths (beyond legacy MAX_PATH) so the exe
    // never silently truncates to an empty match key.
    let mut buf = [0u16; 512];
    let mut len = buf.len() as u32;
    // SAFETY: `buf`/`len` are locals; the PWSTR points into `buf`.
    let queried = unsafe {
        QueryFullProcessImageNameW(
            handle,
            PROCESS_NAME_WIN32,
            PWSTR(buf.as_mut_ptr()),
            &mut len,
        )
    };
    // SAFETY: pairs with the successful OpenProcess above.
    let _ = unsafe { CloseHandle(handle) };
    if queried.is_err() {
        return String::new();
    }
    let full = String::from_utf16_lossy(&buf[..len as usize]);
    // Keep just the file name (drop the directory path).
    full.rsplit(['\\', '/']).next().unwrap_or(&full).to_string()
}

// -- re-resolution -----------------------------------------------------------

/// Resolve the HWND to actually capture. Trust the stored handle only while it
/// still points at the *same* window (handles are process-lifetime and get
/// recycled); otherwise re-find the window by its durable [`WindowKey`].
fn resolve_target_hwnd(stored_hwnd: u64, key: &WindowKey) -> Option<isize> {
    let hwnd = HWND(stored_hwnd as *mut core::ffi::c_void);
    // SAFETY: constructing a HWND from an integer is sound; IsWindow validates.
    if unsafe { IsWindow(hwnd) }.as_bool() {
        // Legacy ids carry no identity — preserve the original trust-the-handle
        // behavior. With one, confirm the live window is still ours before
        // trusting it (guards against a recycled handle value).
        if key.is_empty() || same_window(key, &current_key(hwnd)) {
            return Some(stored_hwnd as isize);
        }
    }
    // Stale or recycled handle: re-bind by identity (needs something to match).
    if key.is_empty() {
        return None;
    }
    resolve_window(key)
}

/// The live identity of an existing window (to validate a stored handle).
fn current_key(hwnd: HWND) -> WindowKey {
    WindowKey::new(exe_of(hwnd), class_of(hwnd), title_of(hwnd))
}

/// The best live match for `target` among all enumerated windows, or `None`.
/// (The scoring itself lives in `window_match`, shared with macOS + Linux.)
fn resolve_window(target: &WindowKey) -> Option<isize> {
    let candidates: Vec<(u64, WindowKey)> = enumerate_windows()
        .into_iter()
        .map(|w| (w.hwnd as u64, w.key))
        .collect();
    resolve_best(target, &candidates).map(|hwnd| hwnd as isize)
}
