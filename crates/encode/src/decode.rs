//! Media **decode** through the labeled ffmpeg component: probe a file's
//! streams, then spawn real-time-paced decoder processes — raw RGBA video
//! on stdout, and (separately) 48 kHz stereo f32 audio. The Media source
//! (`fcap-sources`) reads these pipes; the owned `.frec` format never
//! comes here (it decodes natively).
//!
//! `-re` paces each decoder to the file's own clock, and `-stream_loop -1`
//! loops inside the process, so the reader side stays a dumb pipe. Hardware
//! decode uses `-hwaccel auto`, which falls back to software by itself —
//! no fake toggles, no double plumbing.

use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::time::Duration;

use crate::ffmpeg::{command, run_with_timeout, Ffmpeg};

/// What a probe learned about a media file.
#[derive(Debug, Clone, PartialEq)]
pub struct MediaInfo {
    pub width: u32,
    pub height: u32,
    pub has_audio: bool,
    /// Total duration in seconds, when the banner states one (`N/A` on some
    /// streams — the transport UI treats `None` as "unknown").
    pub duration_secs: Option<f32>,
    /// The video stream's frame rate, when stated (drives the position clock).
    pub fps: Option<f32>,
}

/// Probe a file's banner (`ffmpeg -i`) for its video geometry + whether an
/// audio stream exists.
pub fn probe_media(ffmpeg: &Ffmpeg, path: &Path) -> Result<MediaInfo, String> {
    let banner = run_probe(ffmpeg, path)?;
    parse_banner(&banner)
        .filter(|info| info.width > 0)
        .ok_or_else(|| {
            format!(
                "could not read a video stream from {} — is it a media file?",
                path.display()
            )
        })
}

/// Probe accepting **audio-only** files too (the playlist's music lane):
/// `width`/`height` are 0 when no video stream exists. Errors only when the
/// file has neither stream.
pub fn probe_media_any(ffmpeg: &Ffmpeg, path: &Path) -> Result<MediaInfo, String> {
    let banner = run_probe(ffmpeg, path)?;
    parse_banner(&banner).ok_or_else(|| {
        format!(
            "could not read a media stream from {} — is it a media file?",
            path.display()
        )
    })
}

/// Run `ffmpeg -hide_banner -i <path>` and return its stderr banner.
fn run_probe(ffmpeg: &Ffmpeg, path: &Path) -> Result<String, String> {
    let mut cmd = command(ffmpeg);
    cmd.args(["-hide_banner", "-i"]).arg(path);
    let output = run_with_timeout(cmd, Duration::from_secs(30))?;
    Ok(String::from_utf8_lossy(&output.stderr).into_owned())
}

/// Parse the `-i` banner: the `Video:` line's `WxH` + fps, any `Audio:`
/// line, and the container `Duration:` line. An audio-only banner yields
/// `width`/`height` 0 (and no fps — that field belongs to a video stream);
/// `None` means neither stream was found.
fn parse_banner(banner: &str) -> Option<MediaInfo> {
    let mut dims: Option<(u32, u32)> = None;
    let mut has_audio = false;
    let mut duration_secs: Option<f32> = None;
    let mut fps: Option<f32> = None;
    for line in banner.lines() {
        if line.contains("Audio: ") {
            has_audio = true;
        }
        // `Duration: 00:01:23.45, start: …` — `N/A` stays None.
        if duration_secs.is_none() {
            if let Some(rest) = line.trim_start().strip_prefix("Duration: ") {
                let stamp = rest.split(',').next().unwrap_or("").trim();
                duration_secs = parse_timestamp(stamp);
            }
        }
        if line.contains("Video: ") && dims.is_none() {
            // Fields are comma-separated; the geometry field is `WxH`
            // (possibly with a trailing ` [SAR …]`); the rate field is
            // `NN fps` (fall back to `NN tbr` when fps is absent).
            for field in line.split(',') {
                let field = field.trim();
                let core = field.split_whitespace().next().unwrap_or(field);
                if dims.is_none() {
                    if let Some((w, h)) = core.split_once('x') {
                        if let (Ok(w), Ok(h)) = (w.parse::<u32>(), h.parse::<u32>()) {
                            if (16..=16_384).contains(&w) && (16..=16_384).contains(&h) {
                                dims = Some((w, h));
                                continue;
                            }
                        }
                    }
                }
                if (field.ends_with(" fps") || (fps.is_none() && field.ends_with(" tbr")))
                    && core
                        .parse::<f32>()
                        .is_ok_and(|rate| rate > 0.0 && rate <= 1000.0)
                {
                    fps = core.parse::<f32>().ok();
                }
            }
        }
    }
    match dims {
        Some((width, height)) => Some(MediaInfo {
            width,
            height,
            has_audio,
            duration_secs,
            fps,
        }),
        // No video stream — a pure audio file.
        None if has_audio => Some(MediaInfo {
            width: 0,
            height: 0,
            has_audio: true,
            duration_secs,
            fps: None,
        }),
        None => None,
    }
}

/// `HH:MM:SS.cc` → seconds (`None` for `N/A` or anything malformed).
fn parse_timestamp(stamp: &str) -> Option<f32> {
    let mut parts = stamp.split(':');
    let hours: f32 = parts.next()?.parse().ok()?;
    let minutes: f32 = parts.next()?.parse().ok()?;
    let seconds: f32 = parts.next()?.parse().ok()?;
    if parts.next().is_some() || !(0.0..60.0).contains(&minutes) || !(0.0..60.0).contains(&seconds)
    {
        return None;
    }
    Some(hours * 3600.0 + minutes * 60.0 + seconds)
}

/// The shared PCM output tail: no video, interleaved stereo f32le at
/// 48 kHz on stdout.
fn pcm_out(cmd: &mut Command) {
    cmd.args(["-vn", "-f", "f32le", "-ar", "48000", "-ac", "2", "pipe:1"]);
}

/// The shared raw-video output tail: no audio, raw RGBA frames on stdout.
fn rawvideo_out(cmd: &mut Command) {
    cmd.args(["-an", "-f", "rawvideo", "-pix_fmt", "rgba", "pipe:1"]);
}

/// The fit+pad video filter: scale into `width`×`height` preserving aspect,
/// pad the rest with black, output RGBA — the pipe's frame size never
/// changes whatever the input geometry is.
fn fit_pad_filter(width: u32, height: u32) -> String {
    format!(
        "scale={width}:{height}:force_original_aspect_ratio=decrease,\
         pad={width}:{height}:(ow-iw)/2:(oh-ih)/2:color=black,format=rgba"
    )
}

/// Spawn the video decoder: raw RGBA frames of exactly `width×height×4`
/// bytes on stdout, paced to the file's own clock.
pub fn spawn_video_decoder(
    ffmpeg: &Ffmpeg,
    path: &Path,
    looping: bool,
    hw_decode: bool,
    start_at: f32,
) -> Result<Child, String> {
    let mut cmd = command(ffmpeg);
    cmd.stdout(Stdio::piped()).stderr(Stdio::null());
    cmd.args(["-hide_banner", "-v", "error"]);
    if hw_decode {
        cmd.args(["-hwaccel", "auto"]);
    }
    if looping {
        cmd.args(["-stream_loop", "-1"]);
    }
    if start_at > 0.0 {
        // Input-side seek: fast keyframe jump before decode (the transport
        // scrubber). With `-stream_loop`, later loops replay from the top.
        cmd.args(["-ss", &format!("{start_at:.3}")]);
    }
    cmd.args(["-re", "-i"]).arg(path);
    rawvideo_out(&mut cmd);
    cmd.spawn()
        .map_err(|err| format!("could not start the media decoder: {err}"))
}

/// Spawn the audio decoder: interleaved stereo f32le at 48 kHz on stdout,
/// paced to the file's own clock.
pub fn spawn_audio_decoder(
    ffmpeg: &Ffmpeg,
    path: &Path,
    looping: bool,
    start_at: f32,
) -> Result<Child, String> {
    let mut cmd = command(ffmpeg);
    cmd.stdout(Stdio::piped()).stderr(Stdio::null());
    cmd.args(["-hide_banner", "-v", "error"]);
    if looping {
        cmd.args(["-stream_loop", "-1"]);
    }
    if start_at > 0.0 {
        cmd.args(["-ss", &format!("{start_at:.3}")]);
    }
    cmd.args(["-re", "-i"]).arg(path);
    pcm_out(&mut cmd);
    cmd.spawn()
        .map_err(|err| format!("could not start the media audio decoder: {err}"))
}

/// Spawn an **unpaced** video decoder (no `-re`): frames come as fast as
/// they decode, and the CALLER paces them — the CAP-N10 replay source
/// retimes playback (100/50/25%) by pacing the pipe itself.
pub fn spawn_video_decoder_unpaced(
    ffmpeg: &Ffmpeg,
    path: &Path,
    hw_decode: bool,
    start_at: f32,
) -> Result<Child, String> {
    let mut cmd = command(ffmpeg);
    cmd.stdout(Stdio::piped()).stderr(Stdio::null());
    cmd.args(["-hide_banner", "-v", "error"]);
    if hw_decode {
        cmd.args(["-hwaccel", "auto"]);
    }
    if start_at > 0.0 {
        cmd.args(["-ss", &format!("{start_at:.3}")]);
    }
    cmd.args(["-i"]).arg(path);
    rawvideo_out(&mut cmd);
    cmd.spawn()
        .map_err(|err| format!("could not start the replay decoder: {err}"))
}

/// Spawn an **unpaced** audio decoder (no `-re`); the caller pushes blocks
/// on its own clock.
pub fn spawn_audio_decoder_unpaced(
    ffmpeg: &Ffmpeg,
    path: &Path,
    start_at: f32,
) -> Result<Child, String> {
    let mut cmd = command(ffmpeg);
    cmd.stdout(Stdio::piped()).stderr(Stdio::null());
    cmd.args(["-hide_banner", "-v", "error"]);
    if start_at > 0.0 {
        cmd.args(["-ss", &format!("{start_at:.3}")]);
    }
    cmd.args(["-i"]).arg(path);
    pcm_out(&mut cmd);
    cmd.spawn()
        .map_err(|err| format!("could not start the replay audio decoder: {err}"))
}

// ---------------------------------------------------------------------------
// Gapless playlist decoding (CAP-N17) — the concat demuxer
// ---------------------------------------------------------------------------

/// One playlist entry for the gapless concat decoders.
#[derive(Debug, Clone, PartialEq)]
pub struct ConcatItem {
    pub path: std::path::PathBuf,
    /// In-trim, seconds (0 = from the top).
    pub inpoint: f32,
    /// Out-trim, seconds (0 = to the end).
    pub outpoint: f32,
}

/// Write the `ffconcat` script the demuxer reads — one process plays the
/// whole trimmed list **gaplessly**. Returns the script path.
pub fn write_concat_script(dir: &Path, items: &[ConcatItem]) -> Result<std::path::PathBuf, String> {
    let mut script = String::from("ffconcat version 1.0\n");
    for item in items {
        // Concat-syntax quoting: single-quoted, embedded quotes escaped as
        // '\'' — and forward slashes keep Windows paths unambiguous.
        let path = item.path.to_string_lossy().replace('\\', "/");
        let quoted = path.replace('\'', "'\\''");
        script.push_str(&format!("file '{quoted}'\n"));
        if item.inpoint > 0.0 {
            script.push_str(&format!("inpoint {:.3}\n", item.inpoint));
        }
        if item.outpoint > 0.0 {
            script.push_str(&format!("outpoint {:.3}\n", item.outpoint));
        }
    }
    std::fs::create_dir_all(dir).map_err(|err| format!("playlist workdir: {err}"))?;
    let path = dir.join("playlist.ffconcat");
    std::fs::write(&path, script).map_err(|err| format!("playlist script: {err}"))?;
    Ok(path)
}

/// Spawn the playlist video decoder: the concat demuxer over the script,
/// every item scaled/padded to one geometry, raw RGBA on stdout. `start_at`
/// seeks on the concat timeline (a next/previous jump).
#[allow(clippy::too_many_arguments)] // one call site (the playlist run); a struct would just rename these
pub fn spawn_concat_video_decoder(
    ffmpeg: &Ffmpeg,
    script: &Path,
    width: u32,
    height: u32,
    fps: f32,
    looping: bool,
    hw_decode: bool,
    start_at: f32,
) -> Result<Child, String> {
    let mut cmd = command(ffmpeg);
    cmd.stdout(Stdio::piped()).stderr(Stdio::null());
    cmd.args(["-hide_banner", "-v", "error"]);
    if hw_decode {
        cmd.args(["-hwaccel", "auto"]);
    }
    if looping {
        cmd.args(["-stream_loop", "-1"]);
    }
    if start_at > 0.0 {
        cmd.args(["-ss", &format!("{start_at:.3}")]);
    }
    cmd.args(["-re", "-f", "concat", "-safe", "0", "-i"])
        .arg(script);
    // Mixed resolutions normalize to the playlist geometry (fit + pad) so
    // the pipe stays a fixed frame size — and mixed FRAME RATES normalize
    // to `fps`, so the caller's `frames / fps` position clock is exact for
    // every item (a native-rate pipe drifted it on mixed-fps playlists).
    let filter = format!("fps={fps:.3},{}", fit_pad_filter(width, height));
    cmd.args([
        "-an", "-vf", &filter, "-f", "rawvideo", "-pix_fmt", "rgba", "pipe:1",
    ]);
    cmd.spawn()
        .map_err(|err| format!("could not start the playlist decoder: {err}"))
}

/// Spawn the playlist audio decoder over the same script (48 kHz stereo
/// f32le on stdout).
pub fn spawn_concat_audio_decoder(
    ffmpeg: &Ffmpeg,
    script: &Path,
    looping: bool,
    start_at: f32,
) -> Result<Child, String> {
    let mut cmd = command(ffmpeg);
    cmd.stdout(Stdio::piped()).stderr(Stdio::null());
    cmd.args(["-hide_banner", "-v", "error"]);
    if looping {
        cmd.args(["-stream_loop", "-1"]);
    }
    if start_at > 0.0 {
        cmd.args(["-ss", &format!("{start_at:.3}")]);
    }
    cmd.args(["-re", "-f", "concat", "-safe", "0", "-i"])
        .arg(script);
    pcm_out(&mut cmd);
    cmd.spawn()
        .map_err(|err| format!("could not start the playlist audio decoder: {err}"))
}

// ---------------------------------------------------------------------------
// LAN ingest (CAP-N11) — the SRT/RTMP listener/decoder
// ---------------------------------------------------------------------------

/// Spawn the LAN-ingest LISTENER/decoder: ffmpeg both accepts the sender's
/// connection (SRT `mode=listener` / RTMP `-listen 1` via
/// `extra_input_args`) and decodes the feed. It must be ONE process — two
/// cannot bind one listen port — so video and audio come back together:
/// video scaled+padded to a fixed `width`×`height` canvas (the playlist's
/// fit+pad precedent, so the pipe never changes frame size and no probe
/// roundtrip burns the sender's connection) and 48 kHz stereo f32 audio,
/// INTERLEAVED as rawvideo/pcm_f32le chunks of a streamed AVI on stdout
/// (`00dc`/`01wb` — the caller demuxes; layout verified against the pinned
/// 8.1.2 build). No `-re` (a live feed paces itself), no `-stream_loop`;
/// audio is mapped optionally so a video-only sender still plays. The
/// process exits when the sender disconnects — the caller re-listens.
pub fn spawn_url_av_decoder(
    ffmpeg: &Ffmpeg,
    url: &str,
    extra_input_args: &[String],
    width: u32,
    height: u32,
) -> Result<Child, String> {
    let mut cmd = command(ffmpeg);
    // stderr is PIPED (unlike the file decoders): a live listener that dies
    // names its real cause there — e.g. a sender that delivered no video
    // stream — and the caller reports it instead of guessing at the port.
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    cmd.args(["-hide_banner", "-v", "error"]);
    // Bound the live-stream probe (default is seconds of buffering) so the
    // first frames land fast once a sender connects.
    cmd.args(["-analyzeduration", "2000000", "-probesize", "1000000"]);
    for arg in extra_input_args {
        cmd.arg(arg);
    }
    cmd.args(["-i", url]);
    cmd.args(["-map", "0:v:0", "-map", "0:a:0?"]);
    let filter = fit_pad_filter(width, height);
    cmd.args(["-vf", &filter, "-c:v", "rawvideo"]);
    cmd.args(["-c:a", "pcm_f32le", "-ar", "48000", "-ac", "2"]);
    cmd.args(["-f", "avi", "pipe:1"]);
    cmd.spawn()
        .map_err(|err| format!("could not start the LAN ingest listener: {err}"))
}

// ---------------------------------------------------------------------------
// Reverse rendering (the transport's "true reverse playback")
// ---------------------------------------------------------------------------

/// How long each reversal segment runs. `-vf reverse` buffers a whole input
/// in RAM (ffmpeg's own docs warn about it), so long files are reversed
/// segment-by-segment: split (stream copy, cuts on keyframes) → reverse each
/// short segment → concatenate the reversed segments in reverse order.
const REVERSE_SEGMENT_SECS: u32 = 10;

/// Render a **reversed copy** of a wire-format media file into `dst`
/// (H.264/AAC in MP4; audio reversed too when present). Bounded memory at
/// any input length via segmented reversal; `work_dir` holds the temporary
/// segments and is cleaned up on success. Slow for long files by nature —
/// callers surface honest progress/waiting states.
pub fn reverse_wire_file(
    ffmpeg: &Ffmpeg,
    src: &Path,
    work_dir: &Path,
    dst: &Path,
    has_audio: bool,
) -> Result<(), String> {
    std::fs::create_dir_all(work_dir)
        .map_err(|err| format!("could not create {}: {err}", work_dir.display()))?;

    // 1. Split into short segments, stream-copied (fast, cuts at keyframes).
    let seg_pattern = work_dir.join("seg%05d.mkv");
    let mut cmd = command(ffmpeg);
    cmd.stdout(Stdio::null()).stderr(Stdio::null());
    cmd.args(["-hide_banner", "-v", "error", "-y", "-i"])
        .arg(src);
    cmd.args([
        "-c",
        "copy",
        "-f",
        "segment",
        "-segment_time",
        &REVERSE_SEGMENT_SECS.to_string(),
        "-reset_timestamps",
        "1",
    ])
    .arg(&seg_pattern);
    run_with_timeout(cmd, Duration::from_secs(600))?;

    let mut segments: Vec<std::path::PathBuf> = std::fs::read_dir(work_dir)
        .map_err(|err| format!("could not list {}: {err}", work_dir.display()))?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("seg") && name.ends_with(".mkv"))
        })
        .collect();
    segments.sort();
    if segments.is_empty() {
        return Err("reversing produced no segments — is the file a video?".into());
    }

    // 2. Reverse each segment (short → the frame buffer stays small).
    let mut reversed: Vec<std::path::PathBuf> = Vec::with_capacity(segments.len());
    for (index, segment) in segments.iter().enumerate() {
        let out = work_dir.join(format!("rev{index:05}.mkv"));
        let mut cmd = command(ffmpeg);
        cmd.stdout(Stdio::null()).stderr(Stdio::null());
        cmd.args(["-hide_banner", "-v", "error", "-y", "-i"])
            .arg(segment);
        cmd.args(["-vf", "reverse"]);
        if has_audio {
            cmd.args(["-af", "areverse"]);
        } else {
            cmd.arg("-an");
        }
        cmd.args(["-c:v", "libx264", "-preset", "veryfast", "-crf", "18"]);
        cmd.arg(&out);
        run_with_timeout(cmd, Duration::from_secs(600))?;
        reversed.push(out);
    }

    // 3. Concatenate the reversed segments, last first (stream copy).
    let list_path = work_dir.join("concat.txt");
    let list: String = reversed
        .iter()
        .rev()
        .map(|path| {
            format!(
                "file '{}'\n",
                path.display().to_string().replace('\'', "'\\''")
            )
        })
        .collect();
    std::fs::write(&list_path, list)
        .map_err(|err| format!("could not write {}: {err}", list_path.display()))?;
    let mut cmd = command(ffmpeg);
    cmd.stdout(Stdio::null()).stderr(Stdio::null());
    cmd.args([
        "-hide_banner",
        "-v",
        "error",
        "-y",
        "-f",
        "concat",
        "-safe",
        "0",
        "-i",
    ])
    .arg(&list_path);
    cmd.args(["-c", "copy"]).arg(dst);
    run_with_timeout(cmd, Duration::from_secs(600))?;

    let _ = std::fs::remove_dir_all(work_dir);
    Ok(())
}

/// Spawn an encoder that turns raw RGBA frames on stdin into an H.264 MP4 —
/// the `.frec` → wire bridge the reverse path uses (frec decodes natively,
/// then this re-encodes it so [`reverse_wire_file`] can work on it).
pub fn spawn_rawvideo_encoder(
    ffmpeg: &Ffmpeg,
    width: u32,
    height: u32,
    fps_num: u32,
    fps_den: u32,
    dst: &Path,
) -> Result<Child, String> {
    let mut cmd = command(ffmpeg);
    cmd.stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    cmd.args(["-hide_banner", "-v", "error", "-y"]);
    cmd.args(["-f", "rawvideo", "-pix_fmt", "rgba"]);
    cmd.args(["-s", &format!("{width}x{height}")]);
    cmd.args(["-r", &format!("{fps_num}/{}", fps_den.max(1))]);
    cmd.args(["-i", "-"]);
    cmd.args([
        "-c:v", "libx264", "-preset", "veryfast", "-crf", "18", "-pix_fmt", "yuv420p",
    ]);
    cmd.arg(dst);
    cmd.spawn()
        .map_err(|err| format!("could not start the rawvideo encoder: {err}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn banners_parse_geometry_audio_duration_and_fps() {
        let banner = "Input #0, mov,mp4,m4a,3gp,3g2,mj2, from 'clip.mp4':\n  Duration: 00:00:12.34, start: 0.000000, bitrate: 8123 kb/s\n  Stream #0:0[0x1](und): Video: h264 (High) (avc1 / 0x31637661), yuv420p(progressive), 1920x1080 [SAR 1:1 DAR 16:9], 8000 kb/s, 60 fps, 60 tbr, 15360 tbn (default)\n  Stream #0:1[0x2](und): Audio: aac (LC) (mp4a / 0x6134706D), 48000 Hz, stereo, fltp, 192 kb/s (default)\n";
        let info = parse_banner(banner).expect("parses");
        assert_eq!((info.width, info.height), (1920, 1080));
        assert!(info.has_audio);
        assert!((info.duration_secs.expect("duration") - 12.34).abs() < 0.01);
        assert!((info.fps.expect("fps") - 60.0).abs() < 0.01);
    }

    #[test]
    fn soundless_clips_and_junk_parse_honestly() {
        let soundless = "  Stream #0:0: Video: vp9, yuv420p(tv), 1280x720, 30 fps, 30 tbr\n";
        let info = parse_banner(soundless).expect("parses");
        assert_eq!((info.width, info.height), (1280, 720));
        assert!(!info.has_audio);
        assert_eq!(info.duration_secs, None, "no Duration line = unknown");
        assert_eq!(parse_banner("clip.xyz: Invalid data found"), None);
        // Audio-only (the playlist's music lane): zero geometry, no fps.
        let music = "Input #0, mp3, from 'song.mp3':\n  Duration: 00:03:00.00, start: 0.000000, bitrate: 192 kb/s\n  Stream #0:0: Audio: mp3, 44100 Hz, stereo, fltp, 192 kb/s\n";
        let info = parse_banner(music).expect("audio-only parses");
        assert_eq!((info.width, info.height), (0, 0));
        assert!(info.has_audio);
        assert!((info.duration_secs.expect("duration") - 180.0).abs() < 0.01);
        assert_eq!(info.fps, None);
        // A matrix-size token that is not a geometry must not fool it.
        let weird = "  Stream #0:0: Video: h264, yuv420p, 640x360 [SAR 1:1], 30 fps\n";
        assert_eq!(
            parse_banner(weird).map(|info| (info.width, info.height)),
            Some((640, 360))
        );
    }

    #[test]
    fn na_durations_and_tbr_fallback_parse_honestly() {
        // A live-ish stream: `Duration: N/A`, no ` fps` field — `tbr` fills in.
        let banner = "Input #0, matroska,webm, from 'cap.mkv':\n  Duration: N/A, start: 0.000000, bitrate: N/A\n  Stream #0:0: Video: vp8, yuv420p(progressive), 854x480, SAR 1:1 DAR 427:240, 29.97 tbr, 1k tbn (default)\n";
        let info = parse_banner(banner).expect("parses");
        assert_eq!(info.duration_secs, None, "N/A stays unknown");
        assert!((info.fps.expect("tbr fallback") - 29.97).abs() < 0.01);

        // Timestamp parsing itself.
        assert!((parse_timestamp("01:02:03.50").expect("parses") - 3723.5).abs() < 0.01);
        assert_eq!(parse_timestamp("N/A"), None);
        assert_eq!(parse_timestamp("99:99"), None);
    }
}
