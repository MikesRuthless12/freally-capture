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
    // Position + 1 per hash slot; 0 = empty. 64 KiB, cheap to zero per call.
    let mut table = vec![0u32; 1 << HASH_BITS];
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
    let mut out: Vec<u8> = Vec::with_capacity(decoded_len);
    let mut cursor = 0usize;

    while out.len() < decoded_len {
        let token = *input.get(cursor).ok_or(FlzError::Corrupt)?;
        cursor += 1;
        let literal_len = take_extended(input, &mut cursor, token >> 4)?;
        let literal_end = cursor.checked_add(literal_len).ok_or(FlzError::Corrupt)?;
        if literal_end > input.len() || out.len() + literal_len > decoded_len {
            return Err(FlzError::Corrupt);
        }
        out.extend_from_slice(&input[cursor..literal_end]);
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
        if offset == 0 || offset > out.len() || out.len() + match_len > decoded_len {
            return Err(FlzError::Corrupt);
        }
        let start = out.len() - offset;
        if offset == 1 {
            // Run of one byte — the dominant shape in delta'd frames.
            out.resize(out.len() + match_len, out[start]);
        } else if match_len <= offset {
            out.extend_from_within(start..start + match_len);
        } else {
            for k in 0..match_len {
                let byte = out[start + k];
                out.push(byte);
            }
        }
    }

    if out.len() != decoded_len {
        return Err(FlzError::Corrupt);
    }
    Ok(out)
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
