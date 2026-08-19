//! Open the app's shared transport region (read-only) from inside the Frame
//! Server host process.
//!
//! AUDITED `unsafe`: `OpenFileMapping` / `MapViewOfFile` + a bounded read of a
//! shared page.

use windows::core::HSTRING;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Memory::{
    MapViewOfFile, OpenFileMappingW, UnmapViewOfFile, FILE_MAP_READ, MEMORY_MAPPED_VIEW_ADDRESS,
};

use fcap_stream::vcam::transport::transport_name;

/// A read-only view of the app's shared frame region.
pub struct SharedRegion {
    mapping: HANDLE,
    view: MEMORY_MAPPED_VIEW_ADDRESS,
    len: usize,
}

// The view is a read-only shared page; the handle is process-wide.
unsafe impl Send for SharedRegion {}
unsafe impl Sync for SharedRegion {}

impl SharedRegion {
    /// Open the region the app with `pid` created, mapping `len` bytes. `Ok(None)`
    /// when the app is not running the camera (the common, non-error case).
    pub fn open(pid: u32, len: usize) -> Result<Option<Self>, String> {
        let wide = HSTRING::from(transport_name(pid));
        // SAFETY: opening an existing session-local mapping by name, read-only.
        let mapping = match unsafe { OpenFileMappingW(FILE_MAP_READ.0, false, &wide) } {
            Ok(handle) => handle,
            Err(_) => return Ok(None), // app not streaming — not an error
        };
        // SAFETY: `mapping` is a live mapping of at least `len` bytes.
        let view = unsafe { MapViewOfFile(mapping, FILE_MAP_READ, 0, 0, len) };
        if view.Value.is_null() {
            // SAFETY: closing the handle we just opened and never published.
            unsafe {
                let _ = CloseHandle(mapping);
            }
            return Err("could not map the app's frame region".into());
        }
        Ok(Some(Self { mapping, view, len }))
    }

    /// The mapped bytes, read-only — the source parses these with
    /// [`crate::reader::TransportView`].
    pub fn bytes(&self) -> &[u8] {
        // SAFETY: `view` maps exactly `len` bytes, only ever read.
        unsafe { std::slice::from_raw_parts(self.view.Value as *const u8, self.len) }
    }
}

impl Drop for SharedRegion {
    fn drop(&mut self) {
        // SAFETY: both came from MapViewOfFile / OpenFileMappingW here.
        unsafe {
            let _ = UnmapViewOfFile(self.view);
            let _ = CloseHandle(self.mapping);
        }
    }
}
