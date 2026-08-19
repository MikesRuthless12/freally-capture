//! The source side of the vcam frame transport: open the app's named region by
//! pid, read the `ready_slot` the app published, and convert its RGBA to the
//! BGRA/RGB32 the Media Foundation Frame Server hands to consuming apps.
//!
//! Mirrors `fcap_stream::vcam::transport::FrameWriter`; the format contract is
//! shared through that crate so the two halves can never drift. This module's
//! pure conversion is unit-tested here; the OS mapping is opened in `win`.

use fcap_stream::vcam::transport::{read_header, FrameHeader, FLAG_STREAMING};

/// A parsed view of the shared region: the current header plus a slice over the
/// bytes. The lifetime ties the slot borrow to the backing buffer.
pub struct TransportView<'a> {
    pub header: FrameHeader,
    buffer: &'a [u8],
}

impl<'a> TransportView<'a> {
    /// Parse `buffer` (the mapped shared region). `None` when the header is
    /// missing or from a different protocol version — a stale app build must be
    /// ignored, not misread.
    pub fn parse(buffer: &'a [u8]) -> Option<Self> {
        let header = read_header(buffer)?;
        Some(Self { header, buffer })
    }

    /// Is the app actively pushing frames right now?
    pub fn streaming(&self) -> bool {
        self.header.flags & FLAG_STREAMING != 0
    }

    /// The RGBA bytes of the slot the app most recently published, or `None`
    /// when the geometry does not fit the buffer (a torn/half-initialised
    /// region during startup).
    pub fn ready_rgba(&self) -> Option<&'a [u8]> {
        let bytes = self.header.bytes_per_slot as usize;
        if bytes == 0 {
            return None;
        }
        let off = self.header.slot_offset(self.header.ready_slot);
        self.buffer.get(off..off + bytes)
    }
}

/// Convert one tightly-packed RGBA frame to BGRA in place into `dst` (the layout
/// MF's RGB32 / `MFVideoFormat_ARGB32` uses on little-endian Windows: B,G,R,A
/// bytes). `dst` must be `width*height*4` bytes. Returns false on a size
/// mismatch rather than writing a wrong frame the consumer would tear on.
pub fn rgba_to_bgra(src: &[u8], dst: &mut [u8], width: u32, height: u32) -> bool {
    let expected = (width as usize) * (height as usize) * 4;
    if src.len() != expected || dst.len() != expected {
        return false;
    }
    for (s, d) in src.chunks_exact(4).zip(dst.chunks_exact_mut(4)) {
        d[0] = s[2]; // B
        d[1] = s[1]; // G
        d[2] = s[0]; // R
        d[3] = s[3]; // A
    }
    true
}

/// A solid "no signal" BGRA slate the source serves when the app is not
/// streaming — a neutral dark grey so a consumer shows a steady frame, never a
/// frozen last image or a black hole that reads as a broken camera.
pub fn no_signal_slate(width: u32, height: u32) -> Vec<u8> {
    let mut buf = vec![0u8; (width as usize) * (height as usize) * 4];
    for px in buf.chunks_exact_mut(4) {
        px[0] = 0x20; // B
        px[1] = 0x20; // G
        px[2] = 0x20; // R
        px[3] = 0xff; // A (opaque)
    }
    buf
}

#[cfg(test)]
mod tests {
    use super::*;
    use fcap_stream::vcam::transport::{FrameHeader, FrameWriter};

    #[test]
    fn the_source_reads_the_slot_the_app_published() {
        // Drive the real app-side writer over a Vec, then hand that exact byte
        // image to the source's reader — the two halves must agree slot-for-slot.
        let (w, h) = (2u32, 2u32);
        let header = FrameHeader::new(w, h);
        let mut backing = vec![0u8; header.total_bytes()];
        {
            let mut writer = FrameWriter::new(&mut backing[..], w, h).unwrap();
            let frame: Vec<u8> = (0..(w * h * 4) as u8).collect();
            writer.publish(&frame, w, h).unwrap();
        }
        let view = TransportView::parse(&backing).expect("compatible");
        assert!(view.streaming());
        assert_eq!(
            view.ready_rgba().unwrap(),
            &(0..16u8).collect::<Vec<_>>()[..]
        );
    }

    #[test]
    fn rgba_becomes_bgra_byte_for_byte() {
        let src = [10u8, 20, 30, 40]; // R,G,B,A for one pixel
        let mut dst = [0u8; 4];
        assert!(rgba_to_bgra(&src, &mut dst, 1, 1));
        assert_eq!(dst, [30, 20, 10, 40]); // B,G,R,A
    }

    #[test]
    fn conversion_rejects_a_size_mismatch() {
        let mut dst = [0u8; 4];
        assert!(!rgba_to_bgra(&[0u8; 8], &mut dst, 1, 1), "src too big");
        assert!(!rgba_to_bgra(&[0u8; 4], &mut [0u8; 8], 1, 1), "dst too big");
    }

    #[test]
    fn the_slate_is_opaque_neutral_grey() {
        let slate = no_signal_slate(2, 1);
        assert_eq!(slate.len(), 8);
        assert!(slate.chunks_exact(4).all(|p| p == [0x20, 0x20, 0x20, 0xff]));
    }

    #[test]
    fn a_foreign_or_empty_region_is_refused() {
        assert!(TransportView::parse(&[0u8; 4]).is_none(), "too small");
        assert!(TransportView::parse(&[0xFFu8; 64]).is_none(), "wrong magic");
    }
}
