//! Offline **export**: decode a `.frec` and re-encode it into a wire container
//! (mp4/mkv/mov/webm) so **any** player can open it. The owned lossless
//! `.frec` stays the master; this is the "plays anywhere" copy.
//!
//! It reuses the two halves that already exist and are tested: the owned
//! decoder ([`FrecReader`]) and the recorder's own ffmpeg encode sink
//! ([`FfmpegSink`]) — so an export takes byte-for-byte the same encode path a
//! live wire-codec recording does (raw RGBA over stdin + f32 audio per track),
//! just fed from a file instead of the live studio, as fast as ffmpeg drains.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::ffmpeg::Ffmpeg;
use crate::freally_video::{frame_count, FrecChunk, FrecReader, PixelFormat};
use crate::mux::{Container, FfmpegSink, WirePlan};
use crate::recorder::{RecordSink, RecordSpec};

/// Export progress: frames re-encoded so far, and the total scanned up front.
#[derive(Debug, Clone, Copy)]
pub struct ExportProgress {
    pub frames_done: u64,
    pub frames_total: u64,
}

/// Decode `frec_path` and re-encode into `plan` (its `container` + `path` set
/// the output format + destination). Blocking — run on a worker thread.
/// `cancel` aborts between frames (the partial output is removed). Returns the
/// written file on success.
pub fn export_frec(
    ffmpeg: &Ffmpeg,
    frec_path: &Path,
    plan: &WirePlan,
    mut progress: impl FnMut(ExportProgress),
    cancel: &AtomicBool,
) -> Result<PathBuf, String> {
    if plan.container == Container::Frec {
        return Err("export targets a wire container (mp4/mkv/mov/webm), not frec".into());
    }

    // A fast header-only scan for the progress total before decoding starts.
    let frames_total = frame_count(frec_path).map_err(|err| err.to_string())?;

    let mut reader = FrecReader::open(frec_path).map_err(|err| err.to_string())?;
    let spec = *reader.spec();
    let fps = ((f64::from(spec.fps_num) / f64::from(spec.fps_den.max(1))).round() as u32).max(1);
    let record_spec = RecordSpec {
        width: spec.width,
        height: spec.height,
        fps,
        tracks: (0..spec.audio_tracks as usize).collect(),
    };
    // FfmpegSink ingests RGBA; frec is usually Rgba8 (the recorder writes that)
    // but tolerate a Bgra8 file by swapping R/B before the sink sees it.
    let needs_swap = spec.pixel_format == PixelFormat::Bgra8;

    let mut sink: Box<dyn RecordSink> = Box::new(FfmpegSink::spawn(ffmpeg, &record_spec, plan)?);

    let mut frames_done = 0u64;
    let pumped = pump(
        &mut reader,
        sink.as_mut(),
        record_spec.tracks.len(),
        needs_swap,
        &mut frames_done,
        frames_total,
        &mut progress,
        cancel,
    );

    match pumped {
        Ok(()) => {
            let paths = sink.finish()?;
            progress(ExportProgress {
                frames_done,
                frames_total,
            });
            paths
                .into_iter()
                .next()
                .ok_or_else(|| "the export produced no file".to_string())
        }
        Err(err) => {
            // Drop (not finish) so ffmpeg is killed mid-write, then bin the
            // half-written output — an aborted export leaves no partial file.
            drop(sink);
            let _ = std::fs::remove_file(&plan.path);
            Err(err)
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn pump(
    reader: &mut FrecReader,
    sink: &mut dyn RecordSink,
    track_count: usize,
    needs_swap: bool,
    frames_done: &mut u64,
    frames_total: u64,
    progress: &mut impl FnMut(ExportProgress),
    cancel: &AtomicBool,
) -> Result<(), String> {
    loop {
        if cancel.load(Ordering::Relaxed) {
            return Err("the export was cancelled".into());
        }
        match reader.next_chunk().map_err(|err| err.to_string())? {
            None => return Ok(()), // clean EOF (or a truncated tail — export what's there)
            Some(FrecChunk::Video { mut pixels, .. }) => {
                if needs_swap {
                    for px in pixels.chunks_exact_mut(4) {
                        px.swap(0, 2);
                    }
                }
                sink.write_video(&Arc::new(pixels))?;
                *frames_done += 1;
                if *frames_done % 10 == 0 {
                    progress(ExportProgress {
                        frames_done: *frames_done,
                        frames_total,
                    });
                }
            }
            Some(FrecChunk::Audio {
                track,
                sample_pos,
                samples,
            }) => {
                let slot = track as usize;
                // Ignore an out-of-range track id from a hand-edited file
                // rather than panic — the sink only has `track_count` lanes.
                if slot < track_count {
                    sink.write_audio(slot, sample_pos, &samples)?;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::freally_video::{FrecSpec, FrecWriter};

    fn write_sample_frec(path: &Path, frames: u32) {
        let spec = FrecSpec {
            width: 16,
            height: 8,
            fps_num: 30,
            fps_den: 1,
            pixel_format: PixelFormat::Rgba8,
            audio_tracks: 1,
            sample_rate: 48_000,
        };
        let mut writer = FrecWriter::create(path, spec).expect("create");
        let frame = vec![0u8; 16 * 8 * 4];
        let mut pos = 0u64;
        for _ in 0..frames {
            writer.write_frame(&frame).expect("frame");
            writer
                .write_audio(0, pos, &vec![0.0f32; 1600])
                .expect("audio");
            pos += 800; // 1600 interleaved stereo samples = 800 frames
        }
        writer.finish().expect("finish");
    }

    #[test]
    fn frame_count_matches_what_was_written() {
        let path =
            std::env::temp_dir().join(format!("fcap-export-count-{}.frec", std::process::id()));
        write_sample_frec(&path, 12);
        assert_eq!(frame_count(&path).expect("count"), 12);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn frame_count_tolerates_a_truncated_tail() {
        // A crashed recording (no trailer index) still counts its real frames.
        let path =
            std::env::temp_dir().join(format!("fcap-export-trunc-{}.frec", std::process::id()));
        write_sample_frec(&path, 6);
        // Lop off the trailer + a bit so there's no clean index.
        let bytes = std::fs::read(&path).expect("read");
        std::fs::write(&path, &bytes[..bytes.len() - 20]).expect("truncate");
        // Counting must not error on the truncated tail.
        let counted = frame_count(&path).expect("count");
        assert!(counted <= 6);
        let _ = std::fs::remove_file(&path);
    }
}
