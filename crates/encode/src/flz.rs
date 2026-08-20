//! FLZ — the owned byte-aligned LZ77 compressor under `freally-video`.
//!
//! Classic, decades-expired techniques only: LZ77 windowed matching
//! (Ziv/Lempel 1977) accelerated by a multiplicative hash over 4-byte
//! sequences (Knuth), emitted as a byte-aligned token stream. No entropy
//! coder (a future format method can add one) — the design goal is
//! **real-time lossless** on delta'd/predicted video slices, where input is
//! dominated by zero runs and repeats, at hundreds of MB/s in safe Rust.
//!
//! Token stream: `token u8` — high nibble = literal count, low nibble =
//! match length − 4 — each nibble extending through `0xFF` continuation
//! bytes when 15; then the literals; then (unless the input ended) a
//! little-endian `u16` back-offset (1..=65535). A final token may carry
//! literals only. The decompressor knows the exact decoded size up front and
//! never reads past it, so truncated/corrupt input is an error, not a panic.

use thiserror::Error;

/// Matches shorter than this cost more than they save.
const MIN_MATCH: usize = 4;
/// The farthest a match may reach back (u16 offsets).
const MAX_OFFSET: usize = 65_535;
const HASH_BITS: u32 = 14;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum FlzError {
    #[error("compressed stream is truncated or corrupt")]
    Corrupt,
}

#[inline]
fn hash4(bytes: [u8; 4]) -> usize {
    let value = u32::from_le_bytes(bytes);
    (value.wrapping_mul(2_654_435_761) >> (32 - HASH_BITS)) as usize
}

#[inline]
fn read4(input: &[u8], pos: usize) -> [u8; 4] {
    [input[pos], input[pos + 1], input[pos + 2], input[pos + 3]]
}

/// Append a nibble-extended length (LZ-token convention: `15` in the nibble
/// means "add the following bytes, each up to 255, until one is < 255").
#[inline]
fn push_extended(out: &mut Vec<u8>, mut remaining: usize) {
    while remaining >= 255 {
        out.push(255);
        remaining -= 255;
    }
    out.push(remaining as u8);
}

fn emit(out: &mut Vec<u8>, literals: &[u8], match_len: Option<usize>, offset: usize) {
    let lit_nibble = literals.len().min(15) as u8;
    let match_nibble = match match_len {
        Some(len) => (len - MIN_MATCH).min(15) as u8,
        None => 0,
    };
    out.push((lit_nibble << 4) | match_nibble);
    if literals.len() >= 15 {
        push_extended(out, literals.len() - 15);
    }
    out.extend_from_slice(literals);
    if let Some(len) = match_len {
        out.extend_from_slice(&(offset as u16).to_le_bytes());
        if len - MIN_MATCH >= 15 {
            push_extended(out, len - MIN_MATCH - 15);
        }
    }
}

/// Compress `input` onto the end of `out`. Deterministic; output for empty
/// input is empty.
pub fn compress(input: &[u8], out: &mut Vec<u8>) {
    if input.is_empty() {
        return;
    }
    // Position + 1 per hash slot; 0 = empty. 64 KiB. Kept per-thread and
    // re-zeroed rather than reallocated: `compress` runs once per slice, so at
    // 1080p60 that was ~480 allocations of 64 KiB a second, each one arriving
    // cold and evicting the working set the match loop depends on.
    thread_local! {
        static TABLE: std::cell::RefCell<Vec<u32>> =
            const { std::cell::RefCell::new(Vec::new()) };
    }
    TABLE.with(|cell| {
        let mut table = cell.borrow_mut();
        table.clear();
        table.resize(1 << HASH_BITS, 0);
        compress_with(input, out, &mut table);
    });
}

/// [`compress`], with the caller supplying the (already zeroed) match table.
fn compress_with(input: &[u8], out: &mut Vec<u8>, table: &mut [u32]) {
    let mut pos = 0usize;
    let mut literal_start = 0usize;

    // The last MIN_MATCH-1 bytes can never start a match.
    while pos + MIN_MATCH <= input.len() {
        let slot = hash4(read4(input, pos));
        let candidate = table[slot] as usize;
        table[slot] = (pos + 1) as u32;
        let found = candidate != 0 && {
            let cand = candidate - 1;
            pos - cand <= MAX_OFFSET && read4(input, cand) == read4(input, pos)
        };
        if !found {
            pos += 1;
            continue;
        }
        let cand = candidate - 1;
        // Extend the match as far as the input allows.
        let mut len = MIN_MATCH;
        while pos + len < input.len() && input[cand + len] == input[pos + len] {
            len += 1;
        }
        emit(out, &input[literal_start..pos], Some(len), pos - cand);
        // Seed the table sparsely inside the match (speed over ratio).
        let step = (len / 8).max(1);
        let mut inside = pos + 1;
        while inside + MIN_MATCH <= input.len() && inside < pos + len {
            table[hash4(read4(input, inside))] = (inside + 1) as u32;
            inside += step;
        }
        pos += len;
        literal_start = pos;
    }
    if literal_start < input.len() {
        emit(out, &input[literal_start..], None, 0);
    }
}

#[inline]
fn take_extended(input: &[u8], cursor: &mut usize, nibble: u8) -> Result<usize, FlzError> {
    let mut len = nibble as usize;
    if nibble == 15 {
        loop {
            let byte = *input.get(*cursor).ok_or(FlzError::Corrupt)?;
            *cursor += 1;
            len += byte as usize;
            if byte < 255 {
                break;
            }
        }
    }
    Ok(len)
}

/// Decompress a stream produced by [`compress`]. `decoded_len` is the exact
/// original size (the container stores it); output never exceeds it.
pub fn decompress(input: &[u8], decoded_len: usize) -> Result<Vec<u8>, FlzError> {
    let mut out = vec![0u8; decoded_len];
    decompress_into(input, &mut out)?;
    Ok(out)
}

/// [`decompress`] straight into a caller-owned buffer, whose length IS the
/// expected decoded size. Callers that already have an exactly-sized
/// destination (the `.frec` slice decoder) use this so a decoded frame costs no
/// intermediate allocation and no second copy.
pub fn decompress_into(input: &[u8], out: &mut [u8]) -> Result<(), FlzError> {
    let decoded_len = out.len();
    let mut cursor = 0usize;
    let mut written = 0usize;

    while written < decoded_len {
        let token = *input.get(cursor).ok_or(FlzError::Corrupt)?;
        cursor += 1;
        let literal_len = take_extended(input, &mut cursor, token >> 4)?;
        let literal_end = cursor.checked_add(literal_len).ok_or(FlzError::Corrupt)?;
        if literal_end > input.len() || written + literal_len > decoded_len {
            return Err(FlzError::Corrupt);
        }
        out[written..written + literal_len].copy_from_slice(&input[cursor..literal_end]);
        written += literal_len;
        cursor = literal_end;

        if cursor == input.len() {
            break; // final literals-only token
        }
        if cursor + 2 > input.len() {
            return Err(FlzError::Corrupt);
        }
        let offset = u16::from_le_bytes([input[cursor], input[cursor + 1]]) as usize;
        cursor += 2;
        let match_len = MIN_MATCH + take_extended(input, &mut cursor, token & 0x0F)?;
        if offset == 0 || offset > written || written + match_len > decoded_len {
            return Err(FlzError::Corrupt);
        }
        let start = written - offset;
        if offset == 1 {
            // Run of one byte — the dominant shape in delta'd frames.
            let byte = out[start];
            out[written..written + match_len].fill(byte);
        } else if match_len <= offset {
            // Non-overlapping: one memmove.
            out.copy_within(start..start + match_len, written);
        } else {
            // Overlapping run: each byte must be copied after the byte it
            // repeats has itself been written, so this stays a forward loop.
            for k in 0..match_len {
                out[written + k] = out[start + k];
            }
        }
        written += match_len;
    }

    if written != decoded_len {
        return Err(FlzError::Corrupt);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_trip(data: &[u8]) {
        let mut compressed = Vec::new();
        compress(data, &mut compressed);
        let decoded = decompress(&compressed, data.len()).expect("valid stream");
        assert_eq!(decoded, data, "lossless round-trip");
    }

    /// Deterministic pseudo-random bytes (xorshift — no rand dependency).
    fn noise(len: usize, mut seed: u32) -> Vec<u8> {
        (0..len)
            .map(|_| {
                seed ^= seed << 13;
                seed ^= seed >> 17;
                seed ^= seed << 5;
                (seed >> 24) as u8
            })
            .collect()
    }

    #[test]
    fn empty_input_round_trips() {
        round_trip(&[]);
    }

    #[test]
    fn zeros_compress_hard_and_round_trip() {
        let data = vec![0u8; 1 << 20];
        let mut compressed = Vec::new();
        compress(&data, &mut compressed);
        assert!(
            compressed.len() < data.len() / 100,
            "1 MiB of zeros must compress > 100:1, got {}",
            compressed.len()
        );
        assert_eq!(decompress(&compressed, data.len()).unwrap(), data);
    }

    #[test]
    fn short_inputs_round_trip() {
        for len in 0..=20 {
            round_trip(&noise(len, 7 + len as u32));
        }
    }

    #[test]
    fn text_like_input_round_trips_and_shrinks() {
        let data = "the quick brown fox jumps over the lazy dog — "
            .repeat(512)
            .into_bytes();
        let mut compressed = Vec::new();
        compress(&data, &mut compressed);
        assert!(compressed.len() < data.len() / 4);
        assert_eq!(decompress(&compressed, data.len()).unwrap(), data);
    }

    #[test]
    fn incompressible_noise_round_trips() {
        round_trip(&noise(64 * 1024, 0xC0FFEE));
    }

    #[test]
    fn overlapping_matches_round_trip() {
        // Period-2 and period-3 patterns force overlap copies (offset < len).
        let mut data = Vec::new();
        for _ in 0..4000 {
            data.extend_from_slice(&[0xAB, 0xCD]);
        }
        for _ in 0..3000 {
            data.extend_from_slice(&[1, 2, 3]);
        }
        round_trip(&data);
    }

    #[test]
    fn gradient_like_input_round_trips() {
        let data: Vec<u8> = (0..1 << 16).map(|i| (i / 64) as u8).collect();
        round_trip(&data);
    }

    #[test]
    fn long_range_matches_stay_within_the_offset_cap() {
        // A repeat > 64 KiB apart cannot be matched (u16 offset) — but must
        // still round-trip as literals.
        let mut data = noise(70_000, 42);
        let head: Vec<u8> = data[..1000].to_vec();
        data.extend_from_slice(&head);
        round_trip(&data);
    }

    /// An in-tree fuzz sweep: every mutation of a valid stream, and pure noise,
    /// must return `Err` rather than panic, hang, or write out of bounds.
    ///
    /// `decompress_into` writes through a cursor into a caller-owned slice and
    /// trusts `input` for every length and back-reference, so a malformed
    /// stream is the one thing that could make it index past the end. The
    /// `.frec` payloads it decodes arrive from recordings and imported packs,
    /// i.e. from outside. Deterministic (fixed seeds) so a failure is
    /// reproducible, and cheap enough to run in the normal suite on every OS —
    /// deeper coverage is the `cargo-fuzz` target, which needs nightly.
    #[test]
    fn malformed_streams_never_panic_or_overrun() {
        let sources: [Vec<u8>; 3] = [
            b"hello hello hello hello hello".repeat(20),
            vec![0u8; 4096],
            noise(4096, 0x5eed),
        ];
        for data in &sources {
            let mut valid = Vec::new();
            compress(data, &mut valid);

            // Single-byte mutations at every offset, cycling bit patterns.
            for at in 0..valid.len() {
                for xor in [0x01u8, 0x7f, 0x80, 0xff] {
                    let mut broken = valid.clone();
                    broken[at] ^= xor;
                    // Any outcome is fine except a panic; when it does decode,
                    // it must respect the destination length exactly.
                    let mut out = vec![0u8; data.len()];
                    if decompress_into(&broken, &mut out).is_ok() {
                        assert_eq!(out.len(), data.len());
                    }
                }
            }

            // Every truncation.
            for keep in 0..valid.len() {
                let mut out = vec![0u8; data.len()];
                let _ = decompress_into(&valid[..keep], &mut out);
            }

            // Lying about the decoded length in both directions.
            for len in [0usize, 1, data.len() / 2, data.len() + 1, data.len() * 2] {
                let mut out = vec![0u8; len];
                let _ = decompress_into(&valid, &mut out);
            }
        }

        // Pure noise as a "stream", at several lengths and seeds.
        for seed in [1u32, 7, 99, 0xabcd] {
            for len in [1usize, 3, 17, 64, 512] {
                let junk = noise(len, seed);
                for out_len in [0usize, 1, 64, 4096] {
                    let mut out = vec![0u8; out_len];
                    let _ = decompress_into(&junk, &mut out);
                }
            }
        }
    }

    #[test]
    fn corrupt_streams_error_out() {
        let data = b"hello hello hello hello hello".repeat(20);
        let mut compressed = Vec::new();
        compress(&data, &mut compressed);

        // Truncation.
        assert_eq!(
            decompress(&compressed[..compressed.len() / 2], data.len()),
            Err(FlzError::Corrupt)
        );
        // Wrong decoded length (too large).
        assert_eq!(
            decompress(&compressed, data.len() + 10),
            Err(FlzError::Corrupt)
        );
        // Empty stream, non-empty expectation.
        assert_eq!(decompress(&[], 5), Err(FlzError::Corrupt));
    }

    #[test]
    fn corrupt_offset_is_rejected_not_panicking() {
        // A hand-built token asking to copy from before the start.
        let stream = [0x04u8, 5, 0, 0, 0]; // 0 literals, match len 8, offset 5 — nothing decoded yet
        assert_eq!(decompress(&stream, 8), Err(FlzError::Corrupt));
    }
}
