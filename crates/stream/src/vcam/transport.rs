//! CAP-N76 — the app→camera frame transport.
//!
//! A registered OS virtual-camera source (Windows Frame Server, a macOS CMIO
//! extension, a Linux v4l2 reader) runs in a *different* process than this app,
//! so composed program frames reach it through a named shared-memory buffer:
//! this app is the single writer, the source the single reader.
//!
//! The buffer is one **double slot** — two frame regions plus a header whose
//! `sequence` counter names the slot most recently completed. The writer fills
//! the *other* slot and then publishes it by bumping `sequence`; the reader
//! always reads the slot `sequence` points at. A reader that is mid-copy when
//! the writer publishes simply gets the previous frame that tick — never a torn
//! one — which is exactly the latest-wins contract the rest of the pipeline
//! already uses. No locks on the frame path: a camera must never stall the
//! compositor.
//!
//! This module is the app-side writer and the format contract. It is fully
//! testable in-process (write a frame, read it back), independent of whether a
//! real OS source is registered.

/// The transport header, fixed 32 bytes, little-endian `#[repr(C)]`.
///
/// The layout is asserted by the tests — two processes, two compiler
/// invocations, one contract.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameHeader {
    /// `FVC1` — bump on any layout change.
    pub magic: [u8; 4],
    pub version: u32,
    pub width: u32,
    pub height: u32,
    /// Frames are tightly packed RGBA: `width * height * 4` bytes per slot.
    pub bytes_per_slot: u32,
    /// The slot (0 or 1) the reader should read — the last one fully written.
    pub ready_slot: u32,
    /// Bumped after each publish; the reader's liveness + newest-frame signal.
    pub sequence: u32,
    /// Bit 0: the writer (this app) is streaming.
    pub flags: u32,
}

/// `FrameHeader::magic`.
pub const MAGIC: [u8; 4] = *b"FVC1";
/// Protocol version.
pub const VERSION: u32 = 1;
/// Bit 0 of [`FrameHeader::flags`]: the app is actively pushing frames.
pub const FLAG_STREAMING: u32 = 1 << 0;
/// The two slots the writer alternates between.
pub const SLOTS: u32 = 2;

impl FrameHeader {
    /// A header for a `width`×`height` RGBA stream, nothing published yet.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            magic: MAGIC,
            version: VERSION,
            width,
            height,
            bytes_per_slot: width.saturating_mul(height).saturating_mul(4),
            ready_slot: 0,
            sequence: 0,
            flags: 0,
        }
    }

    /// Does the reader understand this header?
    pub fn is_compatible(&self) -> bool {
        self.magic == MAGIC && self.version == VERSION
    }

    /// Total shared-memory size: the header plus both slots.
    pub fn total_bytes(&self) -> usize {
        std::mem::size_of::<FrameHeader>() + (self.bytes_per_slot as usize) * (SLOTS as usize)
    }

    /// Byte offset of slot `slot` (0 or 1) within the shared region.
    pub fn slot_offset(&self, slot: u32) -> usize {
        std::mem::size_of::<FrameHeader>()
            + (self.bytes_per_slot as usize) * (slot as usize % SLOTS as usize)
    }
}

/// The name the OS source opens to find this app's frames.
pub fn transport_name(pid: u32) -> String {
    format!("Local\\freally-vcam-{pid}")
}

/// The registered source's COM class id, as a `u128` so this crate stays free
/// of the `windows` types. Both halves build their `GUID` from this one value:
/// the app hands it to `MFCreateVirtualCamera`, the source DLL registers under
/// it. They ship as separate artifacts on separate cadences, so a one-sided
/// edit would otherwise produce a camera that registers but never finds frames.
pub const VCAM_SOURCE_CLSID_U128: u128 = 0x0f8e_2a11_9c4d_4b7e_a1c2_3d4e_5f60_7182;

/// Where the app records its pid so the out-of-process source can find the
/// transport (per-user; `HKEY_CURRENT_USER`). The other half of the same
/// contract as [`transport_name`] — the source reads this pid, then opens that
/// name.
pub const PID_KEY: &str = r"Software\Freally\Capture\VirtualCamera";

/// The `REG_DWORD` value under [`PID_KEY`] holding the app's pid.
pub const PID_VALUE: &str = "pid";

/// The double-buffered writer state, backed by any byte buffer (a real shared
/// mapping in production; a `Vec` in tests). Kept generic over the backing
/// store so the publish logic is exercised without an OS mapping.
pub struct FrameWriter<B: AsMut<[u8]>> {
    buffer: B,
    width: u32,
    height: u32,
    bytes_per_slot: usize,
    /// The slot the NEXT frame is written into (the reader reads the other).
    write_slot: u32,
    sequence: u32,
}

impl<B: AsMut<[u8]>> FrameWriter<B> {
    /// Wrap `buffer` (which must be at least `header.total_bytes()`) and stamp
    /// its header. Returns `None` if the buffer is too small.
    pub fn new(mut buffer: B, width: u32, height: u32) -> Option<Self> {
        let header = FrameHeader::new(width, height);
        if buffer.as_mut().len() < header.total_bytes() {
            return None;
        }
        write_header(buffer.as_mut(), &header);
        Some(Self {
            buffer,
            width,
            height,
            bytes_per_slot: header.bytes_per_slot as usize,
            write_slot: 0,
            sequence: 0,
        })
    }

    /// Publish one RGBA frame. `rgba` must be exactly `width*height*4` bytes and
    /// match the writer's geometry; a mismatch is rejected rather than writing a
    /// wrongly-sized slot the reader would misread.
    pub fn publish(&mut self, rgba: &[u8], width: u32, height: u32) -> Result<(), String> {
        if width != self.width || height != self.height {
            return Err(format!(
                "virtual camera frame is {width}x{height}, expected {}x{}",
                self.width, self.height
            ));
        }
        if rgba.len() != self.bytes_per_slot {
            return Err(format!(
                "virtual camera frame is {} bytes, expected {}",
                rgba.len(),
                self.bytes_per_slot
            ));
        }
        let slot = self.write_slot;
        let offset = FrameHeader::new(self.width, self.height).slot_offset(slot);
        let buf = self.buffer.as_mut();
        buf[offset..offset + self.bytes_per_slot].copy_from_slice(rgba);

        // Publish: point the reader at the slot we just filled, bump sequence,
        // then flip so the NEXT frame lands in the other slot (never the one the
        // reader was just handed).
        self.sequence = self.sequence.wrapping_add(1);
        let published = slot;
        self.write_slot = (slot + 1) % SLOTS;
        let mut header = FrameHeader::new(self.width, self.height);
        header.ready_slot = published;
        header.sequence = self.sequence;
        header.flags = FLAG_STREAMING;
        write_header(buf, &header);
        Ok(())
    }

    /// Clear the streaming flag — the reader shows no signal, not a stale frame.
    pub fn mark_stopped(&mut self) {
        let mut header = FrameHeader::new(self.width, self.height);
        header.ready_slot = self.write_slot.wrapping_sub(1) % SLOTS;
        header.sequence = self.sequence;
        header.flags = 0;
        write_header(self.buffer.as_mut(), &header);
    }
}

fn write_header(buf: &mut [u8], header: &FrameHeader) {
    // SAFETY-free: FrameHeader is `#[repr(C)]` POD; copy its bytes into the head
    // of the buffer. `bytemuck` would do this, but the crate takes no new dep
    // for one struct.
    let bytes = header_bytes(header);
    buf[..bytes.len()].copy_from_slice(&bytes);
}

/// The header as its little-endian byte image (hand-serialised so the transport
/// stays dependency-free and endian-explicit across processes).
pub fn header_bytes(h: &FrameHeader) -> [u8; 32] {
    let mut out = [0u8; 32];
    out[0..4].copy_from_slice(&h.magic);
    out[4..8].copy_from_slice(&h.version.to_le_bytes());
    out[8..12].copy_from_slice(&h.width.to_le_bytes());
    out[12..16].copy_from_slice(&h.height.to_le_bytes());
    out[16..20].copy_from_slice(&h.bytes_per_slot.to_le_bytes());
    out[20..24].copy_from_slice(&h.ready_slot.to_le_bytes());
    out[24..28].copy_from_slice(&h.sequence.to_le_bytes());
    out[28..32].copy_from_slice(&h.flags.to_le_bytes());
    out
}

/// Parse a header from the head of a shared region — the reader's entry point.
pub fn read_header(buf: &[u8]) -> Option<FrameHeader> {
    if buf.len() < 32 {
        return None;
    }
    let u32at = |i: usize| u32::from_le_bytes([buf[i], buf[i + 1], buf[i + 2], buf[i + 3]]);
    let header = FrameHeader {
        magic: [buf[0], buf[1], buf[2], buf[3]],
        version: u32at(4),
        width: u32at(8),
        height: u32at(12),
        bytes_per_slot: u32at(16),
        ready_slot: u32at(20),
        sequence: u32at(24),
        flags: u32at(28),
    };
    header.is_compatible().then_some(header)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_header_layout_is_the_contract() {
        assert_eq!(std::mem::size_of::<FrameHeader>(), 32);
        // The hand-serialised image must match the struct size exactly.
        let h = FrameHeader::new(1920, 1080);
        assert_eq!(header_bytes(&h).len(), std::mem::size_of::<FrameHeader>());
    }

    #[test]
    fn a_published_frame_reads_back_from_the_ready_slot() {
        let (w, h) = (4u32, 2u32);
        let header = FrameHeader::new(w, h);
        let buffer = vec![0u8; header.total_bytes()];
        let mut writer = FrameWriter::new(buffer, w, h).expect("fits");

        let frame: Vec<u8> = (0..(w * h * 4) as u8).collect();
        writer.publish(&frame, w, h).expect("published");

        // Read it back exactly as the OS source would: parse the header, then
        // read the slot it points at.
        let buf: &mut [u8] = writer.buffer.as_mut();
        let hdr = read_header(buf).expect("compatible header");
        assert_eq!(hdr.sequence, 1);
        assert_eq!(hdr.flags & FLAG_STREAMING, FLAG_STREAMING);
        let off = hdr.slot_offset(hdr.ready_slot);
        assert_eq!(&buf[off..off + (w * h * 4) as usize], &frame[..]);
    }

    #[test]
    fn consecutive_frames_alternate_slots_so_a_reader_never_sees_a_torn_write() {
        let (w, h) = (2u32, 2u32);
        let header = FrameHeader::new(w, h);
        let mut writer = FrameWriter::new(vec![0u8; header.total_bytes()], w, h).unwrap();

        let a: Vec<u8> = vec![0xAAu8; (w * h * 4) as usize];
        let b: Vec<u8> = vec![0xBBu8; (w * h * 4) as usize];
        writer.publish(&a, w, h).unwrap();
        let slot_a = read_header(AsMut::<[u8]>::as_mut(&mut writer.buffer))
            .unwrap()
            .ready_slot;
        writer.publish(&b, w, h).unwrap();
        let slot_b = read_header(AsMut::<[u8]>::as_mut(&mut writer.buffer))
            .unwrap()
            .ready_slot;

        assert_ne!(
            slot_a, slot_b,
            "the newest frame never lands in the slot just handed out"
        );
        // Frame A's bytes are still intact in its slot while B is being read.
        let off_a = header.slot_offset(slot_a);
        let buf: &mut [u8] = writer.buffer.as_mut();
        assert!(buf[off_a..off_a + 4].iter().all(|&x| x == 0xAA));
    }

    #[test]
    fn a_wrongly_sized_frame_is_rejected_not_written() {
        let (w, h) = (2u32, 2u32);
        let header = FrameHeader::new(w, h);
        let mut writer = FrameWriter::new(vec![0u8; header.total_bytes()], w, h).unwrap();

        assert!(writer.publish(&[0u8; 3], w, h).is_err(), "too few bytes");
        assert!(writer.publish(&[0u8; 16], 4, 1).is_err(), "wrong geometry");
        // A rejected publish must not have advanced the sequence.
        assert_eq!(
            read_header(AsMut::<[u8]>::as_mut(&mut writer.buffer))
                .unwrap()
                .sequence,
            0
        );
    }

    #[test]
    fn stop_clears_the_streaming_flag() {
        let (w, h) = (2u32, 2u32);
        let header = FrameHeader::new(w, h);
        let mut writer = FrameWriter::new(vec![0u8; header.total_bytes()], w, h).unwrap();
        writer
            .publish(&vec![1u8; (w * h * 4) as usize], w, h)
            .unwrap();
        writer.mark_stopped();
        let hdr = read_header(AsMut::<[u8]>::as_mut(&mut writer.buffer)).unwrap();
        assert_eq!(
            hdr.flags & FLAG_STREAMING,
            0,
            "reader sees no signal, not a stale frame"
        );
    }

    #[test]
    fn names_are_per_process() {
        assert_eq!(transport_name(7), r"Local\freally-vcam-7");
        assert_ne!(transport_name(1), transport_name(2));
    }

    #[test]
    fn a_buffer_too_small_is_refused() {
        let tiny: Vec<u8> = vec![0u8; 8];
        assert!(FrameWriter::new(tiny, 1920, 1080).is_none());
    }
}
