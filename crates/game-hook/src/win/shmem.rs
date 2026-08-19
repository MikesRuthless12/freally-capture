//! The named shared-memory control block — the half of the protocol that needs
//! no D3D device, so both sides (and the tests) can exercise it directly.
//!
//! AUDITED `unsafe`: file-mapping creation/open plus the raw-pointer read and
//! write of a `#[repr(C)]` POD. Every block is small and local.

use std::ffi::c_void;

use windows::core::HSTRING;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Memory::{
    CreateFileMappingW, MapViewOfFile, OpenFileMappingW, UnmapViewOfFile, FILE_MAP_ALL_ACCESS,
    FILE_MAP_READ, MEMORY_MAPPED_VIEW_ADDRESS, PAGE_READWRITE,
};

use crate::protocol::ControlBlock;

/// A mapped view of one process's control block.
///
/// Owns its mapping handle and view; dropping it unmaps and closes both. The
/// producer creates one; the consumer opens the same name read-only.
pub struct ControlMapping {
    mapping: HANDLE,
    view: MEMORY_MAPPED_VIEW_ADDRESS,
}

// The view is a plain shared page of POD; the handle is process-wide. Access is
// through the volatile read/write below, which is what makes cross-process
// sharing sound — there is no Rust-side aliasing invariant to violate.
unsafe impl Send for ControlMapping {}

impl ControlMapping {
    /// Create (or replace) the control block for `name` — the producer side.
    pub fn create(name: &str) -> Result<Self, String> {
        let wide = HSTRING::from(name);
        // SAFETY: a fixed-size, page-backed mapping under a session-local name.
        let mapping = unsafe {
            CreateFileMappingW(
                HANDLE::default(),
                None,
                PAGE_READWRITE,
                0,
                std::mem::size_of::<ControlBlock>() as u32,
                &wide,
            )
        }
        .map_err(|err| format!("could not create the control block: {err}"))?;
        Self::map(mapping, FILE_MAP_ALL_ACCESS)
    }

    /// Open an existing control block read-only — the consumer side. `Ok(None)`
    /// when the game is not hooked (the common, non-error case).
    pub fn open(name: &str) -> Result<Option<Self>, String> {
        let wide = HSTRING::from(name);
        // SAFETY: opening an existing session-local mapping by name.
        let mapping = match unsafe { OpenFileMappingW(FILE_MAP_READ.0, false, &wide) } {
            Ok(handle) => handle,
            // Not hooked yet is not an error — the protocol says so explicitly.
            Err(_) => return Ok(None),
        };
        Self::map(mapping, FILE_MAP_READ).map(Some)
    }

    fn map(
        mapping: HANDLE,
        access: windows::Win32::System::Memory::FILE_MAP,
    ) -> Result<Self, String> {
        // SAFETY: `mapping` is a live file mapping of exactly ControlBlock size.
        let view =
            unsafe { MapViewOfFile(mapping, access, 0, 0, std::mem::size_of::<ControlBlock>()) };
        if view.Value.is_null() {
            // SAFETY: closing the handle we just created and never published.
            unsafe {
                let _ = CloseHandle(mapping);
            }
            return Err("could not map the control block".into());
        }
        Ok(Self { mapping, view })
    }

    /// Read the block. Volatile because the other process writes it without any
    /// Rust-visible synchronisation; `frame_index` ordering is what the
    /// protocol relies on, not field-level atomicity.
    pub fn read(&self) -> ControlBlock {
        // SAFETY: `view` maps exactly one ControlBlock, which is POD.
        unsafe { std::ptr::read_volatile(self.view.Value as *const ControlBlock) }
    }

    /// Overwrite the block — producer only.
    pub fn write(&self, block: &ControlBlock) {
        // SAFETY: as `read`; the producer mapped this FILE_MAP_ALL_ACCESS.
        unsafe { std::ptr::write_volatile(self.view.Value as *mut ControlBlock, *block) }
    }
}

impl Drop for ControlMapping {
    fn drop(&mut self) {
        // SAFETY: both were produced by MapViewOfFile / CreateFileMappingW here
        // and are unmapped/closed exactly once.
        unsafe {
            let _ = UnmapViewOfFile(self.view);
            let _ = CloseHandle(self.mapping);
        }
        self.view = MEMORY_MAPPED_VIEW_ADDRESS {
            Value: std::ptr::null_mut::<c_void>(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::{control_name, API_D3D11, FLAG_PRODUCER_ALIVE};

    #[test]
    fn a_consumer_reads_what_the_producer_published() {
        let name = control_name(std::process::id());
        let producer = ControlMapping::create(&name).expect("create");

        let mut block = ControlBlock::new(API_D3D11);
        block.width = 1920;
        block.height = 1080;
        block.flags = FLAG_PRODUCER_ALIVE;
        block.frame_index = 7;
        producer.write(&block);

        let consumer = ControlMapping::open(&name)
            .expect("open")
            .expect("the producer is live");
        let seen = consumer.read();
        assert_eq!(seen, block, "the block crosses the mapping byte-for-byte");
        assert!(seen.has_frame());
    }

    #[test]
    fn an_unhooked_game_is_not_an_error() {
        // The protocol is explicit: no control block means "not hooked", which
        // the app reports as a fallback, never as a failure.
        let missing = control_name(0xDEAD_BEEF);
        assert!(matches!(ControlMapping::open(&missing), Ok(None)));
    }
}
