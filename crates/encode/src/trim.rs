//! CAP-N41: the replay & clip trimmer's engine.
//!
//! Probe a saved wire-container recording (duration, fps, keyframe times),
//! extract preview frames for the trim window, and export the `[in, out)`
//! range: **stream copy** when the in-point lands on a keyframe (no
//! re-encode, no quality loss, near-instant) and an **honest re-encode**
//! otherwise — the caller can tell the user which will happen *before*
//! exporting. A 9:16 reframe export (center crop + scale through the
//! vertical-canvas geometry) always re-encodes, by nature.
//!
//! Everything runs the clearly-labeled, on-demand ffmpeg component against
//! local files the user picked; nothing here touches the network.

use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::process::Stdio;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use crate::decode::probe_media;
use crate::ffmpeg::{command, run_with_timeout, Ffmpeg};
use crate::mux::{audio_args, video_args, Container, EncPreset, RateControl};

/// Keyframe scans and preview grabs are bounded; a broken file fails loudly
/// instead of hanging a worker thread forever.
const PROBE_TIMEOUT: Duration = Duration::from_secs(300);
const PREVIEW_TIMEOUT: Duration = Duration::from_secs(30);
/// A defensive cap on the keyframe list (a 24 h recording at a 1 s keyframe
/// cadence is ~86 k entries; this is far above any sane file).
const MAX_KEYFRAMES: usize = 500_000;

/// What the trim window needs to drive its scrubber and honesty badge.
#[derive(Debug, Clone)]
pub struct TrimInfo {
    pub duration_secs: f64,
    pub fps: f64,
    pub width: u32,
    pub height: u32,
    pub has_audio: bool,
    /// Keyframe presentation times, seconds, ascending.
    pub keyframes_secs: Vec<f64>,
}

/// The export request: the `[in, out)` range and an optional 9:16 reframe
/// target (width, height) — `Some` always re-encodes.
#[derive(Debug, Clone, Copy)]
pub struct TrimSpec {
    pub in_secs: f64,
    pub out_secs: f64,
    pub reframe: Option<(u32, u32)>,
}

/// The re-encode fallback's encoder shape (the user's recording settings).
#[derive(Debug, Clone)]
pub struct TrimEncode {
    pub encoder_id: String,
    pub rate_control: RateControl,
    pub preset: EncPreset,
    pub keyframe_sec: f32,
    pub audio_bitrate_kbps: u32,
}

/// Probe `path` for the trim window: geometry + duration + fps from the
/// banner, then a keyframes-only decode pass for their timestamps.
pub fn trim_info(ffmpeg: &Ffmpeg, path: &Path) -> Result<TrimInfo, String> {
    let media = probe_media(ffmpeg, path)?;
    let duration_secs = media
        .duration_secs
        .ok_or("this file does not state a duration — it cannot be trimmed")?
        as f64;
    let fps = media.fps.unwrap_or(30.0) as f64;

    // Keyframes only (`-skip_frame nokey`), timestamps preserved (`-copyts`),
    // printed by `showinfo` on stderr. Demuxes the whole file but decodes
    // only keyframes — seconds, not minutes, for normal recordings.
    let mut cmd = command(ffmpeg);
    cmd.args(["-hide_banner", "-skip_frame", "nokey", "-copyts", "-i"])
        .arg(path);
    cmd.args(["-an", "-vf", "showinfo", "-f", "null", "-"]);
    let output = run_with_timeout(cmd, PROBE_TIMEOUT)?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    let keyframes_secs = parse_keyframe_times(&stderr);
    if keyframes_secs.is_empty() {
        return Err("no keyframes found — is this a video file?".into());
    }

    Ok(TrimInfo {
        duration_secs,
        fps,
        width: media.width,
        height: media.height,
        has_audio: media.has_audio,
        keyframes_secs,
    })
}

/// Parse `showinfo`'s stderr lines for frame times: every processed frame
/// (keyframes only, given `-skip_frame nokey`) logs `… pts_time:12.345 …`.
/// Sorted ascending, de-duplicated, defensively bounded.
pub fn parse_keyframe_times(stderr: &str) -> Vec<f64> {
    let mut times: Vec<f64> = stderr
        .lines()
        .filter(|line| line.contains("Parsed_showinfo"))
        .filter_map(|line| {
            let rest = line.split("pts_time:").nth(1)?;
            let token = rest.split_whitespace().next()?;
            token
                .parse::<f64>()
                .ok()
                .filter(|t| t.is_finite() && *t >= 0.0)
        })
        .take(MAX_KEYFRAMES)
        .collect();
    times.sort_by(|a, b| a.partial_cmp(b).expect("finite by filter"));
    times.dedup();
    times
}

/// Whether an in-point lands on a keyframe (within half a frame), i.e. the
/// export can stream-copy instead of re-encoding.
pub fn lands_on_keyframe(in_secs: f64, keyframes_secs: &[f64], fps: f64) -> bool {
    let tolerance = 0.5 / fps.max(1.0);
    // Binary search for the nearest keyframe, then check the distance.
    let index = keyframes_secs.partition_point(|&kf| kf < in_secs);
    let mut nearest = f64::INFINITY;
    if index < keyframes_secs.len() {
        nearest = nearest.min((keyframes_secs[index] - in_secs).abs());
    }
    if index > 0 {
        nearest = nearest.min((in_secs - keyframes_secs[index - 1]).abs());
    }
    nearest <= tolerance
}

/// Extract one preview frame at `at_secs` as a JPEG (bounded width — this is
/// a dialog thumbnail, not an export).
pub fn trim_preview_jpeg(ffmpeg: &Ffmpeg, path: &Path, at_secs: f64) -> Result<Vec<u8>, String> {
    let at = at_secs.max(0.0);
    let mut cmd = command(ffmpeg);
    cmd.args([
        "-hide_banner",
        "-v",
        "error",
        "-ss",
        &format!("{at:.3}"),
        "-i",
    ])
    .arg(path);
    cmd.args([
        "-frames:v",
        "1",
        "-vf",
        "scale=min(iw\\,640):-2",
        "-c:v",
        "mjpeg",
        "-f",
        "image2pipe",
        "pipe:1",
    ]);
    let output = run_with_timeout(cmd, PREVIEW_TIMEOUT)?;
    if output.stdout.is_empty() {
        return Err("no frame at that position".into());
    }
    Ok(output.stdout)
}

/// One `-progress pipe:1` line → seconds of output written, when the line
/// carries it (`out_time_us=1234567`).
pub fn parse_progress_secs(line: &str) -> Option<f64> {
    let value = line.strip_prefix("out_time_us=")?.trim();
    let us = value.parse::<i64>().ok()?;
    (us >= 0).then(|| us as f64 / 1_000_000.0)
}

/// Export `[in, out)` of `src` into `dst` (same container as `dst`'s
/// extension implies). Returns `true` when the export was a stream copy.
/// `progress(done_frames, total_frames)` fires while a re-encode runs;
/// `cancel` kills the encode and removes the partial output.
#[allow(clippy::too_many_arguments)]
pub fn trim_export(
    ffmpeg: &Ffmpeg,
    src: &Path,
    dst: &Path,
    container: Container,
    spec: TrimSpec,
    encode: &TrimEncode,
    info: &TrimInfo,
    mut progress: impl FnMut(u64, u64),
    cancel: &AtomicBool,
) -> Result<bool, String> {
    if !(spec.in_secs >= 0.0 && spec.out_secs > spec.in_secs) {
        return Err("the out-point must come after the in-point".into());
    }
    let duration = spec.out_secs - spec.in_secs;
    let copy =
        spec.reframe.is_none() && lands_on_keyframe(spec.in_secs, &info.keyframes_secs, info.fps);

    let mut cmd = command(ffmpeg);
    cmd.stdin(Stdio::null()).stderr(Stdio::piped());
    cmd.args([
        "-hide_banner",
        "-v",
        "error",
        "-y",
        "-ss",
        &format!("{:.3}", spec.in_secs),
        "-i",
    ])
    .arg(src);
    cmd.args(["-t", &format!("{duration:.3}")]);

    if copy {
        cmd.args(["-c", "copy", "-avoid_negative_ts", "make_zero"]);
        cmd.stdout(Stdio::null());
    } else {
        if let Some((width, height)) = spec.reframe {
            // Center-crop to the target aspect, then scale — the 9:16
            // reframe through the vertical-canvas geometry.
            cmd.args([
                "-vf",
                &format!(
                    "crop=min(iw\\,ih*{width}/{height}):min(ih\\,iw*{height}/{width}),\
                     scale={width}:{height}"
                ),
            ]);
        }
        let keyint = (encode.keyframe_sec.max(0.25) * info.fps.max(1.0) as f32).round() as u32;
        cmd.args(video_args(
            &encode.encoder_id,
            &encode.rate_control,
            encode.preset,
            keyint,
        ));
        if info.has_audio {
            cmd.args(audio_args(container, encode.audio_bitrate_kbps));
        } else {
            cmd.arg("-an");
        }
        cmd.args(["-progress", "pipe:1", "-nostats"]);
        cmd.stdout(Stdio::piped());
    }
    if matches!(container, Container::Mp4 | Container::Mov) {
        // A finished clip meant for sharing — classic faststart.
        cmd.args(["-movflags", "+faststart"]);
    }
    cmd.arg(dst);

    let mut child = cmd
        .spawn()
        .map_err(|err| format!("could not start the ffmpeg component: {err}"))?;

    // Drain stderr on a background thread (bounded tail) so a chatty ffmpeg can
    // never block writing to a full stderr pipe while we wait — the copy path
    // reads no other pipe, so without this a warning flood could deadlock it
    // until the timeout.
    let stderr_thread = child.stderr.take().map(|mut stderr| {
        std::thread::spawn(move || {
            let mut buf = Vec::new();
            let _ = stderr.read_to_end(&mut buf);
            if buf.len() > 2048 {
                buf.drain(..buf.len() - 2048);
            }
            String::from_utf8_lossy(&buf).into_owned()
        })
    });

    let frames_total = (duration * info.fps).round().max(1.0) as u64;
    if !copy {
        // Read `-progress` lines until ffmpeg closes the pipe; honor cancel.
        let stdout = child.stdout.take().expect("stdout piped");
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if cancel.load(Ordering::SeqCst) {
                let _ = child.kill();
                let _ = child.wait();
                let _ = std::fs::remove_file(dst);
                return Err("the trim export was cancelled".into());
            }
            let Ok(line) = line else { break };
            if let Some(done_secs) = parse_progress_secs(&line) {
                let frames_done = ((done_secs * info.fps).round() as u64).min(frames_total);
                progress(frames_done, frames_total);
            }
        }
    }

    // Bounded wait for the exit status (copy is near-instant; a re-encode
    // has already streamed its progress to completion by this point).
    let deadline = Instant::now() + PROBE_TIMEOUT;
    let status = loop {
        if cancel.load(Ordering::SeqCst) {
            let _ = child.kill();
            let _ = child.wait();
            let _ = std::fs::remove_file(dst);
            return Err("the trim export was cancelled".into());
        }
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if Instant::now() >= deadline => {
                let _ = child.kill();
                let _ = child.wait();
                let _ = std::fs::remove_file(dst);
                return Err("the trim export timed out".into());
            }
            Ok(None) => std::thread::sleep(Duration::from_millis(50)),
            Err(err) => return Err(format!("could not wait for ffmpeg: {err}")),
        }
    };
    // Join the drain thread now the process has exited — its bounded tail is
    // the honest error message (and joining releases the reader thread).
    let tail = stderr_thread
        .and_then(|thread| thread.join().ok())
        .unwrap_or_default();
    if !status.success() {
        let _ = std::fs::remove_file(dst);
        return Err(format!("the trim export failed: {}", tail.trim()));
    }
    progress(frames_total, frames_total);
    Ok(copy)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn showinfo_lines_yield_sorted_deduped_keyframe_times() {
        let stderr = "\
[Parsed_showinfo_0 @ 0x1] n:   0 pts:      0 pts_time:0       duration_time:0.0166\n\
[Parsed_showinfo_0 @ 0x1] n:   2 pts: 240240 pts_time:4.004   fmt:yuv420p\n\
random ffmpeg noise pts_time:99 (not a showinfo line)\n\
[Parsed_showinfo_0 @ 0x1] n:   1 pts: 120120 pts_time:2.002   fmt:yuv420p\n\
[Parsed_showinfo_0 @ 0x1] n:   3 pts: 240240 pts_time:2.002   duplicate\n";
        assert_eq!(parse_keyframe_times(stderr), vec![0.0, 2.002, 4.004]);
    }

    #[test]
    fn malformed_or_negative_times_are_dropped() {
        let stderr = "\
[Parsed_showinfo_0 @ 0x1] pts_time:NOPE\n\
[Parsed_showinfo_0 @ 0x1] pts_time:-1.5\n\
[Parsed_showinfo_0 @ 0x1] pts_time:1.5\n";
        assert_eq!(parse_keyframe_times(stderr), vec![1.5]);
    }

    #[test]
    fn keyframe_landing_is_within_half_a_frame() {
        let kfs = vec![0.0, 2.0, 4.0];
        let fps = 60.0; // half a frame = ~8.3 ms
        assert!(lands_on_keyframe(2.0, &kfs, fps));
        assert!(lands_on_keyframe(2.008, &kfs, fps), "within tolerance");
        assert!(!lands_on_keyframe(2.02, &kfs, fps), "over half a frame off");
        assert!(!lands_on_keyframe(1.0, &kfs, fps), "between keyframes");
        assert!(lands_on_keyframe(0.0, &kfs, fps), "the file start");
        assert!(!lands_on_keyframe(5.0, &[], fps), "no keyframes at all");
    }

    #[test]
    fn progress_lines_parse_only_out_time_us() {
        assert_eq!(parse_progress_secs("out_time_us=1500000"), Some(1.5));
        assert_eq!(parse_progress_secs("out_time_us=0"), Some(0.0));
        assert_eq!(parse_progress_secs("out_time_us=-1"), None);
        assert_eq!(parse_progress_secs("frame=42"), None);
        assert_eq!(parse_progress_secs("out_time=00:00:01.500000"), None);
    }
}
