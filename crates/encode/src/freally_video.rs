//! `freally-video` — the owned lossless codec + `.frec` container.
//!
//! The default **local recording** format: fully lossless, authored here and
//! owned outright, shared with Freally Snipper. Needs **no external tool** —
//! nothing fetched, nothing patent-encumbered: temporal frame deltas +
//! left-pixel prediction (PNG-style filtering, patent-free) + the owned
//! [`crate::flz`] LZ77 stage (Ziv/Lempel 1977) — every technique decades
//! expired or public domain, per the ownership charter.
//!
//! ## Layout (all integers little-endian)
//!
//! ```text
//! file    := header chunk* [index]
//! header  := magic "FREC" | version u8 (1) | pixel_format u8 | audio_tracks u8
//!            | audio_channels u8 | width u32 | height u32 | fps_num u32
//!            | fps_den u32 | sample_rate u32 | flags u16 (0)
//! chunk   := tag u8 | payload_len u32 | payload
//! video   := tag 0x01 | frame_index u64 | kind u8 (0 intra, 1 delta)
//!            | slice_count u16 | slices…
//! slice   := method u8 (0 raw, 1 flz, 2 sub-predict + flz)
//!            | raw_len u32 | data_len u32 | data
//! audio   := tag 0x02 | track u8 | sample_pos u64 | frame_count u32
//!            | f32le interleaved samples
//! index   := tag 0x03 | entry_count u32 | (frame_index u64, offset u64)*
//!            then trailer: index_offset u64 | "FRECIDX1"
//! ```
//!
//! Frames are split into row bands ("slices") compressed and decompressed in
//! parallel; each slice is self-contained. Intra frames land on a ~2-second
//! cadence so damaged/truncated files replay to the last complete chunk and
//! future seeking has anchor points. A truncated tail (crash, power loss) is
//! detected and reading ends cleanly at the last complete chunk.
//!
//! Timestamps: video is CFR — frame `i` presents at `i · fps_den / fps_num`
//! seconds; audio chunks carry their absolute sample position. Pause simply
//! stops feeding the writer (both clocks resume contiguously), which is what
//! makes pause/resume gapless in one playable file.

use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::Path;

use thiserror::Error;

use crate::flz;

const MAGIC: &[u8; 4] = b"FREC";
const VERSION: u8 = 1;
const TRAILER_MAGIC: &[u8; 8] = b"FRECIDX1";

const TAG_VIDEO: u8 = 0x01;
const TAG_AUDIO: u8 = 0x02;
const TAG_INDEX: u8 = 0x03;

const METHOD_RAW: u8 = 0;
const METHOD_FLZ: u8 = 1;
const METHOD_SUB_FLZ: u8 = 2;

/// Bytes per pixel — the codec is BGRA/RGBA-only by design (what the
/// compositor and captures produce).
const BPP: usize = 4;
/// Upper bound on any chunk payload (guards allocations on corrupt input;
/// generous enough for 8K BGRA = ~132 MiB).
const MAX_PAYLOAD: u32 = 256 * 1024 * 1024;
/// Upper bound on audio frames per chunk (10 s at 48 kHz).
const MAX_AUDIO_FRAMES: u32 = 480_000;

#[derive(Debug, Error)]
pub enum FrecError {
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
    #[error("not a freally-video file")]
    NotFrec,
    #[error("unsupported freally-video version {0} (this build reads {VERSION})")]
    Version(u8),
    #[error("corrupt freally-video data: {0}")]
    Corrupt(&'static str),
    #[error("invalid encoder input: {0}")]
    Invalid(String),
}

/// The frame memory layout inside a `.frec` file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    Bgra8,
    Rgba8,
}

impl PixelFormat {
    fn to_byte(self) -> u8 {
        match self {
            PixelFormat::Bgra8 => 0,
            PixelFormat::Rgba8 => 1,
        }
    }

    fn from_byte(byte: u8) -> Result<Self, FrecError> {
        match byte {
            0 => Ok(PixelFormat::Bgra8),
            1 => Ok(PixelFormat::Rgba8),
            _ => Err(FrecError::Corrupt("unknown pixel format")),
        }
    }
}

/// Everything a `.frec` stream is parameterized by.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrecSpec {
    pub width: u32,
    pub height: u32,
    pub fps_num: u32,
    pub fps_den: u32,
    pub pixel_format: PixelFormat,
    /// Audio track count (0..=6); every track is interleaved stereo f32.
    pub audio_tracks: u8,
    pub sample_rate: u32,
}

impl FrecSpec {
    fn frame_bytes(&self) -> usize {
        self.width as usize * self.height as usize * BPP
    }

    fn validate(&self) -> Result<(), FrecError> {
        if self.width == 0 || self.height == 0 {
            return Err(FrecError::Invalid("zero canvas".into()));
        }
        if self.width > 16_384 || self.height > 16_384 {
            return Err(FrecError::Invalid("canvas larger than 16384px".into()));
        }
        if self.fps_num == 0 || self.fps_den == 0 {
            return Err(FrecError::Invalid("zero frame rate".into()));
        }
        if self.audio_tracks > 6 {
            return Err(FrecError::Invalid("more than 6 audio tracks".into()));
        }
        if self.sample_rate == 0 || self.sample_rate > 384_000 {
            return Err(FrecError::Invalid("unreasonable sample rate".into()));
        }
        Ok(())
    }
}

/// What [`FrecWriter::finish`] reports.
#[derive(Debug, Clone, Copy)]
pub struct FrecStats {
    pub frames: u64,
    pub raw_video_bytes: u64,
    pub written_bytes: u64,
}

// ---------------------------------------------------------------------------
// Slice-parallel frame compression
// ---------------------------------------------------------------------------

/// Row bands compressed independently (and in parallel). Sized so a 1080p
/// frame yields enough slices to feed every core without drowning small
/// frames in per-slice overhead.
fn slice_rows(height: u32) -> usize {
    match height {
        0..=299 => height.max(1) as usize,
        300..=899 => 96,
        _ => 136,
    }
}

fn workers() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
        .clamp(1, 8)
}

/// Subtract the previous pixel from each pixel, per byte channel, per row —
/// PNG's "Sub" filter on 4-byte pixels. Reversible exactly (wrapping).
fn predict_sub(slice: &mut [u8], row_bytes: usize) {
    for row in slice.chunks_exact_mut(row_bytes) {
        for i in (BPP..row.len()).rev() {
            row[i] = row[i].wrapping_sub(row[i - BPP]);
        }
    }
}

fn unpredict_sub(slice: &mut [u8], row_bytes: usize) {
    for row in slice.chunks_exact_mut(row_bytes) {
        for i in BPP..row.len() {
            row[i] = row[i].wrapping_add(row[i - BPP]);
        }
    }
}

struct CompressedSlice {
    method: u8,
    raw_len: u32,
    data: Vec<u8>,
}

/// Compress one slice: delta slices go straight to FLZ (already mostly
/// zeros); intra slices predict first. Either way, if FLZ expands the data
/// the slice is stored raw — output is never meaningfully larger than input.
fn compress_slice(slice: &[u8], row_bytes: usize, intra: bool) -> CompressedSlice {
    let mut work;
    let (input, method): (&[u8], u8) = if intra {
        work = slice.to_vec();
        predict_sub(&mut work, row_bytes);
        (&work, METHOD_SUB_FLZ)
    } else {
        (slice, METHOD_FLZ)
    };
    let mut data = Vec::with_capacity(input.len() / 2);
    flz::compress(input, &mut data);
    if data.len() >= slice.len() {
        CompressedSlice {
            method: METHOD_RAW,
            raw_len: slice.len() as u32,
            data: slice.to_vec(),
        }
    } else {
        CompressedSlice {
            method,
            raw_len: slice.len() as u32,
            data,
        }
    }
}

/// Split `frame` into row bands and compress them across `workers()` scoped
/// threads. Order is preserved.
fn compress_frame(frame: &[u8], row_bytes: usize, intra: bool) -> Vec<CompressedSlice> {
    let band_bytes = slice_rows((frame.len() / row_bytes) as u32) * row_bytes;
    let slices: Vec<&[u8]> = frame.chunks(band_bytes).collect();
    let workers = workers().min(slices.len().max(1));
    if workers <= 1 || slices.len() <= 1 {
        return slices
            .iter()
            .map(|slice| compress_slice(slice, row_bytes, intra))
            .collect();
    }
    let mut results: Vec<Option<CompressedSlice>> = Vec::new();
    results.resize_with(slices.len(), || None);
    let chunk = slices.len().div_ceil(workers);
    std::thread::scope(|scope| {
        for (band, out) in slices.chunks(chunk).zip(results.chunks_mut(chunk)) {
            scope.spawn(move || {
                for (slice, slot) in band.iter().zip(out.iter_mut()) {
                    *slot = Some(compress_slice(slice, row_bytes, intra));
                }
            });
        }
    });
    results
        .into_iter()
        .map(|slot| slot.expect("every slice compressed"))
        .collect()
}

// ---------------------------------------------------------------------------
// Writer
// ---------------------------------------------------------------------------

/// Streaming `.frec` writer. Feed it frames and audio blocks in recording
/// order; call [`FrecWriter::finish`] to land the seek index + trailer (a
/// file missing them — crash, power loss — still plays to its last chunk).
pub struct FrecWriter {
    spec: FrecSpec,
    out: BufWriter<File>,
    /// Previous frame (reconstruction reference for deltas).
    previous: Option<Vec<u8>>,
    frame_index: u64,
    keyframe_every: u64,
    index: Vec<(u64, u64)>,
    written: u64,
    raw_video: u64,
}

impl FrecWriter {
    pub fn create(path: &Path, spec: FrecSpec) -> Result<Self, FrecError> {
        spec.validate()?;
        let file = File::create(path)?;
        let mut writer = FrecWriter {
            keyframe_every: (2 * spec.fps_num as u64 / spec.fps_den as u64).max(1),
            spec,
            out: BufWriter::with_capacity(1 << 20, file),
            previous: None,
            frame_index: 0,
            index: Vec::new(),
            written: 0,
            raw_video: 0,
        };
        writer.write_header()?;
        Ok(writer)
    }

    pub fn spec(&self) -> &FrecSpec {
        &self.spec
    }

    /// Frames written so far.
    pub fn frames(&self) -> u64 {
        self.frame_index
    }

    fn write_header(&mut self) -> Result<(), FrecError> {
        let spec = &self.spec;
        let mut header = Vec::with_capacity(32);
        header.extend_from_slice(MAGIC);
        header.push(VERSION);
        header.push(spec.pixel_format.to_byte());
        header.push(spec.audio_tracks);
        header.push(2); // channels — stereo end-to-end, like the mixer
        header.extend_from_slice(&spec.width.to_le_bytes());
        header.extend_from_slice(&spec.height.to_le_bytes());
        header.extend_from_slice(&spec.fps_num.to_le_bytes());
        header.extend_from_slice(&spec.fps_den.to_le_bytes());
        header.extend_from_slice(&spec.sample_rate.to_le_bytes());
        header.extend_from_slice(&0u16.to_le_bytes()); // flags
        self.out.write_all(&header)?;
        self.written += header.len() as u64;
        Ok(())
    }

    /// Write one frame (tightly-packed BGRA/RGBA, exactly canvas-sized).
    pub fn write_frame(&mut self, pixels: &[u8]) -> Result<(), FrecError> {
        if pixels.len() != self.spec.frame_bytes() {
            return Err(FrecError::Invalid(format!(
                "frame is {} bytes, canvas needs {}",
                pixels.len(),
                self.spec.frame_bytes()
            )));
        }
        let row_bytes = self.spec.width as usize * BPP;
        let intra = self.frame_index % self.keyframe_every == 0 || self.previous.is_none();

        // Delta the frame against the previous one (wrapping byte subtract);
        // intra frames skip this and predict spatially inside the slices.
        let slices = if intra {
            compress_frame(pixels, row_bytes, true)
        } else {
            let previous = self.previous.as_ref().expect("delta implies a reference");
            let mut delta = pixels.to_vec();
            for (byte, prev) in delta.iter_mut().zip(previous) {
                *byte = byte.wrapping_sub(*prev);
            }
            compress_frame(&delta, row_bytes, false)
        };

        let mut payload =
            Vec::with_capacity(11 + slices.iter().map(|s| s.data.len() + 9).sum::<usize>());
        payload.extend_from_slice(&self.frame_index.to_le_bytes());
        payload.push(if intra { 0 } else { 1 });
        payload.extend_from_slice(&(slices.len() as u16).to_le_bytes());
        for slice in &slices {
            payload.push(slice.method);
            payload.extend_from_slice(&slice.raw_len.to_le_bytes());
            payload.extend_from_slice(&(slice.data.len() as u32).to_le_bytes());
            payload.extend_from_slice(&slice.data);
        }
        if intra {
            self.index.push((self.frame_index, self.written));
        }
        self.write_chunk(TAG_VIDEO, &payload)?;

        self.raw_video += pixels.len() as u64;
        self.frame_index += 1;
        match &mut self.previous {
            Some(previous) => previous.copy_from_slice(pixels),
            None => self.previous = Some(pixels.to_vec()),
        }
        Ok(())
    }

    /// Write one audio block for `track` (interleaved stereo f32).
    /// `sample_pos` is the block's absolute position in frames since
    /// recording start — the reader derives A/V sync from it.
    pub fn write_audio(
        &mut self,
        track: u8,
        sample_pos: u64,
        samples: &[f32],
    ) -> Result<(), FrecError> {
        if track >= self.spec.audio_tracks {
            return Err(FrecError::Invalid(format!(
                "track {track} on a {}-track file",
                self.spec.audio_tracks
            )));
        }
        if samples.len() % 2 != 0 {
            return Err(FrecError::Invalid("odd sample count (stereo)".into()));
        }
        let frames = (samples.len() / 2) as u32;
        if frames > MAX_AUDIO_FRAMES {
            return Err(FrecError::Invalid("audio block over 10 s".into()));
        }
        let mut payload = Vec::with_capacity(13 + samples.len() * 4);
        payload.push(track);
        payload.extend_from_slice(&sample_pos.to_le_bytes());
        payload.extend_from_slice(&frames.to_le_bytes());
        for sample in samples {
            payload.extend_from_slice(&sample.to_le_bytes());
        }
        self.write_chunk(TAG_AUDIO, &payload)
    }

    fn write_chunk(&mut self, tag: u8, payload: &[u8]) -> Result<(), FrecError> {
        if payload.len() as u64 > MAX_PAYLOAD as u64 {
            return Err(FrecError::Invalid("chunk over the payload cap".into()));
        }
        self.out.write_all(&[tag])?;
        self.out.write_all(&(payload.len() as u32).to_le_bytes())?;
        self.out.write_all(payload)?;
        self.written += 5 + payload.len() as u64;
        Ok(())
    }

    /// Land the seek index + trailer and flush everything to disk.
    pub fn finish(mut self) -> Result<FrecStats, FrecError> {
        let index_offset = self.written;
        let mut payload = Vec::with_capacity(4 + self.index.len() * 16);
        payload.extend_from_slice(&(self.index.len() as u32).to_le_bytes());
        for (frame, offset) in &self.index {
            payload.extend_from_slice(&frame.to_le_bytes());
            payload.extend_from_slice(&offset.to_le_bytes());
        }
        self.write_chunk(TAG_INDEX, &payload)?;
        self.out.write_all(&index_offset.to_le_bytes())?;
        self.out.write_all(TRAILER_MAGIC)?;
        self.written += 16;
        self.out.flush()?;
        self.out.get_ref().sync_all()?;
        Ok(FrecStats {
            frames: self.frame_index,
            raw_video_bytes: self.raw_video,
            written_bytes: self.written,
        })
    }
}

// ---------------------------------------------------------------------------
// Reader
// ---------------------------------------------------------------------------

/// One decoded chunk, in file order.
pub enum FrecChunk {
    /// A full reconstructed frame (tightly-packed, canvas-sized).
    Video { frame_index: u64, pixels: Vec<u8> },
    /// One track's audio block (interleaved stereo f32).
    Audio {
        track: u8,
        sample_pos: u64,
        samples: Vec<f32>,
    },
}

/// Sequential `.frec` reader/decoder. Yields chunks in file order and ends
/// cleanly (Ok(None)) at EOF **or** at a truncated tail — a crashed
/// recording plays to its last complete chunk.
pub struct FrecReader {
    spec: FrecSpec,
    input: BufReader<File>,
    /// Last reconstructed frame (delta reference).
    previous: Option<Vec<u8>>,
}

impl FrecReader {
    pub fn open(path: &Path) -> Result<Self, FrecError> {
        let file = File::open(path)?;
        let mut input = BufReader::with_capacity(1 << 20, file);
        // 4 magic + 4 single bytes + 5×u32 + u16 flags = 30 bytes.
        let mut header = [0u8; 30];
        input
            .read_exact(&mut header)
            .map_err(|_| FrecError::NotFrec)?;
        if &header[0..4] != MAGIC {
            return Err(FrecError::NotFrec);
        }
        if header[4] != VERSION {
            return Err(FrecError::Version(header[4]));
        }
        let le_u32 = |at: usize| u32::from_le_bytes(header[at..at + 4].try_into().expect("4"));
        let spec = FrecSpec {
            pixel_format: PixelFormat::from_byte(header[5])?,
            audio_tracks: header[6],
            width: le_u32(8),
            height: le_u32(12),
            fps_num: le_u32(16),
            fps_den: le_u32(20),
            sample_rate: le_u32(24),
        };
        spec.validate()
            .map_err(|_| FrecError::Corrupt("bad header"))?;
        if header[7] != 2 {
            return Err(FrecError::Corrupt("non-stereo audio"));
        }
        Ok(FrecReader {
            spec,
            input,
            previous: None,
        })
    }

    pub fn spec(&self) -> &FrecSpec {
        &self.spec
    }

    /// The next chunk, `Ok(None)` at the end (including a truncated tail).
    pub fn next_chunk(&mut self) -> Result<Option<FrecChunk>, FrecError> {
        loop {
            let mut head = [0u8; 5];
            if !read_exact_or_eof(&mut self.input, &mut head)? {
                return Ok(None); // clean EOF (or truncated mid-head)
            }
            let tag = head[0];
            let len = u32::from_le_bytes(head[1..5].try_into().expect("4"));
            if len > MAX_PAYLOAD {
                return Err(FrecError::Corrupt("payload over cap"));
            }
            let mut payload = vec![0u8; len as usize];
            if !read_exact_or_eof(&mut self.input, &mut payload)? {
                return Ok(None); // truncated tail — play what we have
            }
            match tag {
                TAG_VIDEO => return Ok(Some(self.decode_video(&payload)?)),
                TAG_AUDIO => return Ok(Some(decode_audio(&self.spec, &payload)?)),
                TAG_INDEX => {
                    // Trailer follows; nothing left to play.
                    return Ok(None);
                }
                _ => {
                    // Unknown chunk from a newer minor writer: skip it.
                    continue;
                }
            }
        }
    }

    fn decode_video(&mut self, payload: &[u8]) -> Result<FrecChunk, FrecError> {
        if payload.len() < 11 {
            return Err(FrecError::Corrupt("short video chunk"));
        }
        let frame_index = u64::from_le_bytes(payload[0..8].try_into().expect("8"));
        let kind = payload[8];
        let slice_count = u16::from_le_bytes([payload[9], payload[10]]) as usize;
        let frame_bytes = self.spec.frame_bytes();
        let row_bytes = self.spec.width as usize * BPP;

        // Parse the slice directory first (cheap), then decompress in
        // parallel into one exactly-frame-sized buffer.
        let mut cursor = 11usize;
        let mut slices: Vec<(u8, usize, &[u8])> = Vec::with_capacity(slice_count);
        let mut total_raw = 0usize;
        for _ in 0..slice_count {
            if cursor + 9 > payload.len() {
                return Err(FrecError::Corrupt("slice header past the payload"));
            }
            let method = payload[cursor];
            let raw_len =
                u32::from_le_bytes(payload[cursor + 1..cursor + 5].try_into().expect("4")) as usize;
            let data_len =
                u32::from_le_bytes(payload[cursor + 5..cursor + 9].try_into().expect("4")) as usize;
            cursor += 9;
            if cursor + data_len > payload.len() {
                return Err(FrecError::Corrupt("slice data past the payload"));
            }
            if raw_len % row_bytes != 0 {
                return Err(FrecError::Corrupt("slice not row-aligned"));
            }
            total_raw += raw_len;
            if total_raw > frame_bytes {
                return Err(FrecError::Corrupt("slices exceed the frame"));
            }
            slices.push((method, raw_len, &payload[cursor..cursor + data_len]));
            cursor += data_len;
        }
        if total_raw != frame_bytes {
            return Err(FrecError::Corrupt("slices do not cover the frame"));
        }

        let mut frame = vec![0u8; frame_bytes];
        {
            // Hand each worker a contiguous run of (slice, out-band) pairs.
            let mut bands: Vec<&mut [u8]> = Vec::with_capacity(slices.len());
            let mut rest: &mut [u8] = &mut frame;
            for (_, raw_len, _) in &slices {
                let (band, tail) = rest.split_at_mut(*raw_len);
                bands.push(band);
                rest = tail;
            }
            let workers = workers().min(slices.len().max(1));
            let chunk = slices.len().div_ceil(workers);
            let results: Vec<Result<(), FrecError>> = std::thread::scope(|scope| {
                let mut handles = Vec::new();
                let mut band_iter = bands.into_iter();
                for group in slices.chunks(chunk) {
                    let group_bands: Vec<&mut [u8]> =
                        band_iter.by_ref().take(group.len()).collect();
                    handles.push(scope.spawn(move || {
                        for ((method, _raw_len, data), out) in group.iter().zip(group_bands) {
                            decode_slice(*method, data, out, row_bytes)?;
                        }
                        Ok(())
                    }));
                }
                handles
                    .into_iter()
                    .map(|handle| handle.join().expect("decode worker never panics"))
                    .collect()
            });
            for result in results {
                result?;
            }
        }

        match kind {
            0 => {} // intra — frame is complete
            1 => {
                let previous = self
                    .previous
                    .as_ref()
                    .ok_or(FrecError::Corrupt("delta frame without a reference"))?;
                for (byte, prev) in frame.iter_mut().zip(previous) {
                    *byte = byte.wrapping_add(*prev);
                }
            }
            _ => return Err(FrecError::Corrupt("unknown frame kind")),
        }
        match &mut self.previous {
            Some(previous) => previous.copy_from_slice(&frame),
            None => self.previous = Some(frame.clone()),
        }
        Ok(FrecChunk::Video {
            frame_index,
            pixels: frame,
        })
    }
}

fn decode_slice(
    method: u8,
    data: &[u8],
    out: &mut [u8],
    row_bytes: usize,
) -> Result<(), FrecError> {
    match method {
        METHOD_RAW => {
            if data.len() != out.len() {
                return Err(FrecError::Corrupt("raw slice length mismatch"));
            }
            out.copy_from_slice(data);
        }
        METHOD_FLZ | METHOD_SUB_FLZ => {
            let decoded = flz::decompress(data, out.len())
                .map_err(|_| FrecError::Corrupt("slice stream corrupt"))?;
            out.copy_from_slice(&decoded);
            if method == METHOD_SUB_FLZ {
                unpredict_sub(out, row_bytes);
            }
        }
        _ => return Err(FrecError::Corrupt("unknown slice method")),
    }
    Ok(())
}

fn decode_audio(spec: &FrecSpec, payload: &[u8]) -> Result<FrecChunk, FrecError> {
    if payload.len() < 13 {
        return Err(FrecError::Corrupt("short audio chunk"));
    }
    let track = payload[0];
    if track >= spec.audio_tracks {
        return Err(FrecError::Corrupt("audio track out of range"));
    }
    let sample_pos = u64::from_le_bytes(payload[1..9].try_into().expect("8"));
    let frames = u32::from_le_bytes(payload[9..13].try_into().expect("4"));
    if frames > MAX_AUDIO_FRAMES {
        return Err(FrecError::Corrupt("audio block over cap"));
    }
    let expected = frames as usize * 2 * 4;
    if payload.len() != 13 + expected {
        return Err(FrecError::Corrupt("audio payload length mismatch"));
    }
    let samples = payload[13..]
        .chunks_exact(4)
        .map(|bytes| f32::from_le_bytes(bytes.try_into().expect("4")))
        .collect();
    Ok(FrecChunk::Audio {
        track,
        sample_pos,
        samples,
    })
}

/// `read_exact` that reports a clean `false` on EOF-before-anything or a
/// truncated tail (partial read then EOF) instead of an error.
fn read_exact_or_eof(reader: &mut impl Read, buf: &mut [u8]) -> Result<bool, FrecError> {
    let mut filled = 0usize;
    while filled < buf.len() {
        match reader.read(&mut buf[filled..]) {
            Ok(0) => return Ok(false),
            Ok(n) => filled += n,
            Err(err) if err.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(err) => return Err(err.into()),
        }
    }
    Ok(true)
}

/// Read a finished file's seek index (frame index → file offset of its intra
/// chunk). `None` when the trailer is missing (unfinished recording) — the
/// file still plays sequentially.
pub fn read_index(path: &Path) -> Result<Option<Vec<(u64, u64)>>, FrecError> {
    let mut file = File::open(path)?;
    let len = file.metadata()?.len();
    if len < 16 {
        return Ok(None);
    }
    file.seek(SeekFrom::End(-16))?;
    let mut trailer = [0u8; 16];
    file.read_exact(&mut trailer)?;
    if &trailer[8..16] != TRAILER_MAGIC {
        return Ok(None);
    }
    let index_offset = u64::from_le_bytes(trailer[0..8].try_into().expect("8"));
    if index_offset >= len {
        return Err(FrecError::Corrupt("index offset past EOF"));
    }
    file.seek(SeekFrom::Start(index_offset))?;
    let mut head = [0u8; 5];
    file.read_exact(&mut head)?;
    if head[0] != TAG_INDEX {
        return Err(FrecError::Corrupt("trailer points away from the index"));
    }
    let payload_len = u32::from_le_bytes(head[1..5].try_into().expect("4"));
    if payload_len > MAX_PAYLOAD {
        return Err(FrecError::Corrupt("index over cap"));
    }
    let mut payload = vec![0u8; payload_len as usize];
    file.read_exact(&mut payload)?;
    if payload.len() < 4 {
        return Err(FrecError::Corrupt("short index"));
    }
    let count = u32::from_le_bytes(payload[0..4].try_into().expect("4")) as usize;
    if payload.len() != 4 + count * 16 {
        return Err(FrecError::Corrupt("index length mismatch"));
    }
    let mut entries = Vec::with_capacity(count);
    for entry in payload[4..].chunks_exact(16) {
        entries.push((
            u64::from_le_bytes(entry[0..8].try_into().expect("8")),
            u64::from_le_bytes(entry[8..16].try_into().expect("8")),
        ));
    }
    Ok(Some(entries))
}

/// Count the video frames in a `.frec` by scanning chunk **headers only**
/// (payloads are seeked past, never decoded) — fast enough for a progress
/// total before an export. Stops at the trailer index or a truncated tail, so
/// a crashed recording reports the frames it actually holds.
pub fn frame_count(path: &Path) -> Result<u64, FrecError> {
    let file = File::open(path)?;
    let mut input = BufReader::with_capacity(1 << 16, file);
    let mut header = [0u8; 30];
    input.read_exact(&mut header).map_err(|_| FrecError::NotFrec)?;
    if &header[0..4] != MAGIC {
        return Err(FrecError::NotFrec);
    }
    if header[4] != VERSION {
        return Err(FrecError::Version(header[4]));
    }
    let mut frames = 0u64;
    loop {
        let mut head = [0u8; 5];
        if !read_exact_or_eof(&mut input, &mut head)? {
            break; // clean EOF or truncated head
        }
        let tag = head[0];
        let len = u32::from_le_bytes(head[1..5].try_into().expect("4"));
        if len > MAX_PAYLOAD {
            return Err(FrecError::Corrupt("payload over cap"));
        }
        if tag == TAG_INDEX {
            break; // the trailer follows; no more frames
        }
        if tag == TAG_VIDEO {
            frames += 1;
        }
        if input.seek(SeekFrom::Current(i64::from(len))).is_err() {
            break; // truncated tail — count what we could reach
        }
    }
    Ok(frames)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_frec(tag: &str) -> std::path::PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "fcap-frec-test-{}-{nanos}-{tag}.frec",
            std::process::id()
        ))
    }

    fn spec_64x48() -> FrecSpec {
        FrecSpec {
            width: 64,
            height: 48,
            fps_num: 60,
            fps_den: 1,
            pixel_format: PixelFormat::Rgba8,
            audio_tracks: 2,
            sample_rate: 48_000,
        }
    }

    /// A deterministic synthetic frame: gradient + a moving box + a noisy
    /// band (exercises intra prediction, deltas, and the stored fallback).
    fn synth_frame(spec: &FrecSpec, t: usize) -> Vec<u8> {
        let (w, h) = (spec.width as usize, spec.height as usize);
        let mut pixels = vec![0u8; w * h * 4];
        let mut seed = 0x9E37_79B9u32 ^ (t as u32);
        for y in 0..h {
            for x in 0..w {
                let at = (y * w + x) * 4;
                let in_box =
                    x >= (t * 3) % w && x < ((t * 3) % w + 8).min(w) && (8..16).contains(&y);
                if y >= h - 8 {
                    // noisy band
                    seed ^= seed << 13;
                    seed ^= seed >> 17;
                    seed ^= seed << 5;
                    pixels[at] = (seed >> 8) as u8;
                    pixels[at + 1] = (seed >> 16) as u8;
                    pixels[at + 2] = (seed >> 24) as u8;
                } else if in_box {
                    pixels[at] = 255;
                    pixels[at + 1] = 32;
                    pixels[at + 2] = 32;
                } else {
                    pixels[at] = (x * 255 / w) as u8;
                    pixels[at + 1] = (y * 255 / h) as u8;
                    pixels[at + 2] = 96;
                }
                pixels[at + 3] = 255;
            }
        }
        pixels
    }

    #[test]
    fn video_round_trips_exactly_across_intra_and_delta() {
        let path = temp_frec("video");
        let spec = spec_64x48();
        let frames: Vec<Vec<u8>> = (0..150).map(|t| synth_frame(&spec, t)).collect();

        let mut writer = FrecWriter::create(&path, spec).expect("create");
        for frame in &frames {
            writer.write_frame(frame).expect("write");
        }
        let stats = writer.finish().expect("finish");
        assert_eq!(stats.frames, 150);
        assert!(
            stats.written_bytes < stats.raw_video_bytes,
            "synthetic content must compress ({} vs {})",
            stats.written_bytes,
            stats.raw_video_bytes
        );

        let mut reader = FrecReader::open(&path).expect("open");
        assert_eq!(reader.spec(), &spec);
        let mut got = 0usize;
        while let Some(chunk) = reader.next_chunk().expect("read") {
            if let FrecChunk::Video {
                frame_index,
                pixels,
            } = chunk
            {
                assert_eq!(frame_index, got as u64);
                assert_eq!(pixels, frames[got], "frame {got} must be bit-exact");
                got += 1;
            }
        }
        assert_eq!(got, 150);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn audio_round_trips_bit_exactly_and_interleaves_with_video() {
        let path = temp_frec("audio");
        let spec = spec_64x48();
        let mut writer = FrecWriter::create(&path, spec).expect("create");

        let block: Vec<f32> = (0..960).map(|i| ((i as f32) * 0.01).sin() * 0.5).collect();
        writer.write_frame(&synth_frame(&spec, 0)).expect("frame");
        writer.write_audio(0, 0, &block).expect("audio t0");
        writer.write_audio(1, 0, &block).expect("audio t1");
        writer.write_frame(&synth_frame(&spec, 1)).expect("frame");
        writer.write_audio(0, 480, &block).expect("audio t0 b1");
        writer.finish().expect("finish");

        let mut reader = FrecReader::open(&path).expect("open");
        let mut audio_chunks = 0usize;
        let mut video_chunks = 0usize;
        while let Some(chunk) = reader.next_chunk().expect("read") {
            match chunk {
                FrecChunk::Audio {
                    track,
                    sample_pos,
                    samples,
                } => {
                    assert!(track < 2);
                    assert!(sample_pos == 0 || sample_pos == 480);
                    let bits: Vec<u32> = samples.iter().map(|s| s.to_bits()).collect();
                    let expect: Vec<u32> = block.iter().map(|s| s.to_bits()).collect();
                    assert_eq!(bits, expect, "audio must be bit-exact");
                    audio_chunks += 1;
                }
                FrecChunk::Video { .. } => video_chunks += 1,
            }
        }
        assert_eq!(audio_chunks, 3);
        assert_eq!(video_chunks, 2);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn a_truncated_tail_plays_to_the_last_complete_chunk() {
        let path = temp_frec("truncated");
        let spec = spec_64x48();
        let frames: Vec<Vec<u8>> = (0..10).map(|t| synth_frame(&spec, t)).collect();
        let mut writer = FrecWriter::create(&path, spec).expect("create");
        for frame in &frames {
            writer.write_frame(frame).expect("write");
        }
        writer.finish().expect("finish");

        // Chop the file mid-way (simulates a crash mid-write).
        let bytes = std::fs::read(&path).expect("read back");
        std::fs::write(&path, &bytes[..bytes.len() * 2 / 3]).expect("truncate");

        let mut reader = FrecReader::open(&path).expect("open");
        let mut got = 0usize;
        while let Some(chunk) = reader.next_chunk().expect("truncated read never errors") {
            if let FrecChunk::Video { pixels, .. } = chunk {
                assert_eq!(pixels, frames[got]);
                got += 1;
            }
        }
        assert!(got > 0, "some complete frames survive");
        assert!(got < 10, "the chopped tail is gone");
        assert!(
            read_index(&path).expect("no trailer is fine").is_none(),
            "a truncated file has no index"
        );
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn the_index_maps_intra_frames() {
        let path = temp_frec("index");
        let spec = spec_64x48(); // keyframe_every = 120 at 60fps
        let mut writer = FrecWriter::create(&path, spec).expect("create");
        for t in 0..130 {
            writer.write_frame(&synth_frame(&spec, t)).expect("write");
        }
        writer.finish().expect("finish");

        let index = read_index(&path).expect("read").expect("trailer present");
        let frames: Vec<u64> = index.iter().map(|(frame, _)| *frame).collect();
        assert_eq!(frames, vec![0, 120], "intra cadence is ~2 s");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn junk_files_are_rejected_not_panicking() {
        let path = temp_frec("junk");
        std::fs::write(&path, b"MP4 nope, definitely not frec").expect("write");
        assert!(matches!(FrecReader::open(&path), Err(FrecError::NotFrec)));
        let _ = std::fs::remove_file(&path);

        let short = temp_frec("short");
        std::fs::write(&short, b"FR").expect("write");
        assert!(matches!(FrecReader::open(&short), Err(FrecError::NotFrec)));
        let _ = std::fs::remove_file(&short);
    }

    #[test]
    fn writer_rejects_wrong_sized_frames_and_bad_tracks() {
        let path = temp_frec("invalid");
        let spec = spec_64x48();
        let mut writer = FrecWriter::create(&path, spec).expect("create");
        assert!(matches!(
            writer.write_frame(&[0u8; 16]),
            Err(FrecError::Invalid(_))
        ));
        assert!(matches!(
            writer.write_audio(5, 0, &[0.0; 4]),
            Err(FrecError::Invalid(_))
        ));
        assert!(matches!(
            writer.write_audio(0, 0, &[0.0; 3]),
            Err(FrecError::Invalid(_))
        ));
        drop(writer);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn spec_validation_rejects_nonsense() {
        let mut spec = spec_64x48();
        spec.width = 0;
        assert!(spec.validate().is_err());
        let mut spec = spec_64x48();
        spec.audio_tracks = 7;
        assert!(spec.validate().is_err());
        let mut spec = spec_64x48();
        spec.fps_den = 0;
        assert!(spec.validate().is_err());
    }

    /// Not a benchmark — a guardrail that encode keeps a healthy real-time
    /// margin on this synthetic 1080p mix. Runs release-mode only (debug
    /// numbers are meaningless); `--ignored` runs it on demand.
    #[test]
    #[ignore = "perf guardrail — run with --release -- --ignored"]
    fn encode_holds_1080p60_on_synthetic_content() {
        let spec = FrecSpec {
            width: 1920,
            height: 1080,
            fps_num: 60,
            fps_den: 1,
            pixel_format: PixelFormat::Rgba8,
            audio_tracks: 0,
            sample_rate: 48_000,
        };
        let path = temp_frec("perf");
        let frames: Vec<Vec<u8>> = (0..60).map(|t| synth_frame(&spec, t)).collect();
        let mut writer = FrecWriter::create(&path, spec).expect("create");
        let start = std::time::Instant::now();
        for frame in &frames {
            writer.write_frame(frame).expect("write");
        }
        let elapsed = start.elapsed();
        writer.finish().expect("finish");
        let _ = std::fs::remove_file(&path);
        assert!(
            elapsed < std::time::Duration::from_millis(900),
            "60 synthetic 1080p frames must encode in < 0.9 s (got {elapsed:?})"
        );
    }
}
