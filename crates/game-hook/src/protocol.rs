//! The wire types shared by the injected producer and the app-side consumer.
//!
//! `design/game-hook-protocol.md` is normative. This module is
//! platform-independent and dependency-free on purpose: the consumer links it
//! as an `rlib` and the tests exercise it on every OS, so the two sides can
//! never drift on a field offset.

/// Control-block magic — bump on any layout change.
pub const MAGIC: [u8; 4] = *b"FGH1";

/// Protocol version carried in the control block.
pub const VERSION: u32 = 1;

/// Bit 0 of [`ControlBlock::flags`]: the producer is installed and alive.
pub const FLAG_PRODUCER_ALIVE: u32 = 1 << 0;
/// Bit 1: geometry changed since the consumer last looked (resize / fullscreen
/// toggle) — the consumer must reopen the shared texture.
pub const FLAG_GEOMETRY_CHANGED: u32 = 1 << 1;

/// `present_api` values.
pub const API_D3D11: u32 = 11;
/// D3D12 presents through the swap-chain vtable too, but the copy path differs.
pub const API_D3D12: u32 = 12;

/// Keyed-mutex keys. The producer writes under `PRODUCER`, hands off as
/// `CONSUMER`; the consumer copies under `CONSUMER` and hands back as
/// `PRODUCER`. See the protocol doc's stall rule.
pub const KEY_PRODUCER: u64 = 0;
/// The key the consumer acquires to read a finished frame.
pub const KEY_CONSUMER: u64 = 1;

/// The named shared-memory control block, exactly 64 bytes, little-endian.
///
/// `#[repr(C)]` and the explicit padding field are the contract — the layout is
/// asserted by the tests below, because a silent reorder would desync two
/// processes that never share a compiler invocation.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControlBlock {
    pub magic: [u8; 4],
    pub version: u32,
    pub width: u32,
    pub height: u32,
    /// The `DXGI_FORMAT` of the shared texture.
    pub format: u32,
    pub flags: u32,
    /// Incremented after each successful publish. The only liveness signal the
    /// consumer needs: a stalled index means "not presenting", not "broken".
    pub frame_index: u64,
    pub present_api: u32,
    pub reserved: [u32; 7],
}

impl ControlBlock {
    /// A zeroed block stamped with this build's magic + version.
    pub fn new(present_api: u32) -> Self {
        Self {
            magic: MAGIC,
            version: VERSION,
            width: 0,
            height: 0,
            format: 0,
            flags: 0,
            frame_index: 0,
            present_api,
            reserved: [0; 7],
        }
    }

    /// Is this a block this build understands? Checked by the consumer before
    /// reading any other field — a stale DLL from an older install must be
    /// ignored, not misread.
    pub fn is_compatible(&self) -> bool {
        self.magic == MAGIC && self.version == VERSION
    }

    /// Has the producer published a usable frame yet?
    pub fn has_frame(&self) -> bool {
        self.is_compatible()
            && self.flags & FLAG_PRODUCER_ALIVE != 0
            && self.frame_index > 0
            && self.width > 0
            && self.height > 0
    }
}

/// The control-block name for a given game process.
pub fn control_name(pid: u32) -> String {
    format!(r"Local\freally-game-hook-{pid}")
}

/// The shared-texture name for a given game process.
pub fn texture_name(pid: u32) -> String {
    format!(r"Local\freally-game-hook-tex-{pid}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_control_block_layout_is_the_contract() {
        // Two processes, two compiler invocations, one layout. If this ever
        // changes, MAGIC must change with it.
        assert_eq!(std::mem::size_of::<ControlBlock>(), 64);
        assert_eq!(std::mem::align_of::<ControlBlock>(), 8);
    }

    #[test]
    fn a_fresh_block_is_compatible_but_has_no_frame() {
        let block = ControlBlock::new(API_D3D11);
        assert!(block.is_compatible());
        assert!(!block.has_frame(), "nothing presented yet");
    }

    #[test]
    fn foreign_or_older_blocks_are_refused_outright() {
        let mut block = ControlBlock::new(API_D3D11);
        block.magic = *b"XXXX";
        assert!(!block.is_compatible());

        let mut older = ControlBlock::new(API_D3D11);
        older.version = VERSION - 1;
        assert!(
            !older.is_compatible(),
            "a stale injected DLL must be ignored"
        );
    }

    #[test]
    fn has_frame_needs_liveness_geometry_and_a_published_index() {
        let mut block = ControlBlock::new(API_D3D11);
        block.flags = FLAG_PRODUCER_ALIVE;
        block.width = 1920;
        block.height = 1080;
        assert!(!block.has_frame(), "index still zero");

        block.frame_index = 1;
        assert!(block.has_frame());

        block.flags = 0;
        assert!(!block.has_frame(), "producer gone");
    }

    #[test]
    fn names_are_per_process_and_session_local() {
        assert_eq!(control_name(4242), r"Local\freally-game-hook-4242");
        assert_eq!(texture_name(4242), r"Local\freally-game-hook-tex-4242");
        assert_ne!(control_name(1), control_name(2));
    }
}
