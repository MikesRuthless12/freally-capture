//! Locating and patching the swap-chain `Present` vtable entry.
//!
//! A COM interface pointer points at a pointer to its **vtable** — an array of
//! function pointers. `IDXGISwapChain::Present` is slot 8 and `Present1`
//! (on `IDXGISwapChain1`) is slot 22. Hooking is a single pointer write in that
//! array, saving the original so every call still reaches the game's real
//! present and so unload can restore it byte-for-byte.
//!
//! Why vtable patching and not an inline trampoline: it is one aligned pointer
//! write inside DXGI's own read/write vtable memory, it needs no
//! executable-page allocation, and it is trivially and completely reversible —
//! the pattern least likely to read as evasion to an anti-cheat or an AV, and
//! the one OBS-lineage hooks have used for a decade.
//!
//! AUDITED `unsafe`: raw vtable reads + a `VirtualProtect`-guarded pointer
//! write. Nothing here calls into the game; it only redirects a slot.

use std::ffi::c_void;

use windows::Win32::System::Memory::{
    VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS,
};

/// The `IDXGISwapChain::Present` vtable index (IUnknown 0-2, IDXGIObject 3-6,
/// IDXGIDeviceSubObject 7, then Present at 8).
pub const PRESENT_INDEX: usize = 8;
/// `IDXGISwapChain1::Present1` — the base 18 IDXGISwapChain entries then
/// GetDesc1/GetFullscreenDesc/GetHwnd/GetCoreWindow, Present1 at 22.
pub const PRESENT1_INDEX: usize = 22;

/// A single saved-and-replaced vtable slot; restores itself on drop.
pub struct VtableHook {
    slot: *mut *mut c_void,
    original: *mut c_void,
}

// The pointers address DXGI's own vtable, valid for the process lifetime while
// a swapchain exists; the hook is installed and removed from one thread.
unsafe impl Send for VtableHook {}

impl VtableHook {
    /// Read the vtable of the COM object `interface_ptr` points at, and replace
    /// entry `index` with `replacement`, returning a hook that restores the
    /// original when dropped. `original()` gives the real function to call
    /// through.
    ///
    /// # Safety
    /// `interface_ptr` must be a live COM interface pointer whose vtable has at
    /// least `index + 1` entries (true for any IDXGISwapChain/…1). `replacement`
    /// must be ABI-compatible with the entry it replaces.
    pub unsafe fn install(
        interface_ptr: *mut c_void,
        index: usize,
        replacement: *mut c_void,
    ) -> Result<Self, String> {
        // *interface_ptr is the vtable pointer; *that is the function array.
        let vtable = *(interface_ptr as *mut *mut *mut c_void);
        let slot = vtable.add(index);
        let original = *slot;

        let size = std::mem::size_of::<*mut c_void>();
        let mut old = PAGE_PROTECTION_FLAGS(0);
        VirtualProtect(
            slot as *const c_void,
            size,
            PAGE_EXECUTE_READWRITE,
            &mut old,
        )
        .map_err(|err| format!("could not unprotect the vtable slot: {err}"))?;
        std::ptr::write(slot, replacement);
        // Restore the original page protection immediately — the slot is only
        // writable for the single store above.
        let mut discard = PAGE_PROTECTION_FLAGS(0);
        let _ = VirtualProtect(slot as *const c_void, size, old, &mut discard);

        Ok(Self { slot, original })
    }

    /// The real function pointer that was in the slot before hooking — the hook
    /// calls through this so the game presents normally.
    pub fn original(&self) -> *mut c_void {
        self.original
    }
}

impl Drop for VtableHook {
    fn drop(&mut self) {
        // Put the original pointer back so unload leaves DXGI byte-for-byte as
        // it was — no lingering redirect after the capture app detaches.
        // SAFETY: `slot` and `original` came from `install`; same lifetime.
        unsafe {
            let size = std::mem::size_of::<*mut c_void>();
            let mut old = PAGE_PROTECTION_FLAGS(0);
            if VirtualProtect(
                self.slot as *const c_void,
                size,
                PAGE_EXECUTE_READWRITE,
                &mut old,
            )
            .is_ok()
            {
                std::ptr::write(self.slot, self.original);
                let mut discard = PAGE_PROTECTION_FLAGS(0);
                let _ = VirtualProtect(self.slot as *const c_void, size, old, &mut discard);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // A fake COM object: [vtable ptr] -> [fn0, fn1, ... fn8]. Enough to prove
    // install() rewrites the right slot and Drop restores it, with no DXGI.
    extern "system" fn original_fn() -> u32 {
        1
    }
    extern "system" fn replacement_fn() -> u32 {
        2
    }

    #[test]
    fn install_replaces_the_slot_and_drop_restores_it() {
        let mut vtable: Vec<*mut c_void> = (0..=PRESENT_INDEX)
            .map(|_| original_fn as *mut c_void)
            .collect();
        let vtable_ptr = vtable.as_mut_ptr();
        // The "interface pointer": a pointer to the vtable pointer.
        let mut vtable_ptr_holder = vtable_ptr;
        let interface_ptr = &mut vtable_ptr_holder as *mut _ as *mut c_void;

        let saved = original_fn as *mut c_void;
        {
            // SAFETY: the fake vtable has PRESENT_INDEX+1 entries and both fns
            // share the ABI.
            let hook = unsafe {
                VtableHook::install(interface_ptr, PRESENT_INDEX, replacement_fn as *mut c_void)
            }
            .expect("install");
            assert_eq!(hook.original(), saved, "the real fn is handed back");
            assert_eq!(
                vtable[PRESENT_INDEX], replacement_fn as *mut c_void,
                "the slot now points at the hook"
            );
        }
        // Drop ran: the slot is the original again.
        assert_eq!(
            vtable[PRESENT_INDEX], saved,
            "unload restores DXGI byte-for-byte"
        );
    }
}
