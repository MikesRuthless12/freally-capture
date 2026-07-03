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
use std::process::{Child, Stdio};
use std::time::Duration;

use crate::ffmpeg::{command, run_with_timeout, Ffmpeg};

/// What a probe learned about a media file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaInfo {
    pub width: u32,
    pub height: u32,
    pub has_audio: bool,
}

/// Probe a file's banner (`ffmpeg -i`) for its video geometry + whether an
/// audio stream exists.
pub fn probe_media(ffmpeg: &Ffmpeg, path: &Path) -> Result<MediaInfo, String> {
    let mut cmd = command(ffmpeg);
    cmd.args(["-hide_banner", "-i"]).arg(path);
    let output = run_with_timeout(cmd, Duration::from_secs(30))?;
    let banner = String::from_utf8_lossy(&output.stderr);
    parse_banner(&banner).ok_or_else(|| {
        format!(
            "could not read a video stream from {} — is it a media file?",
            path.display()
        )
    })
}

/// Parse the `-i` banner: the `Video:` line's `WxH` + any `Audio:` line.
fn parse_banner(banner: &str) -> Option<MediaInfo> {
    let mut dims: Option<(u32, u32)> = None;
    let mut has_audio = false;
    for line in banner.lines() {
        if line.contains("Audio: ") {
            has_audio = true;
        }
        if line.contains("Video: ") && dims.is_none() {
            // Fields are comma-separated; the geometry field is `WxH`
            // (possibly with a trailing ` [SAR …]`).
            for field in line.split(',') {
                let field = field.trim();
                let core = field.split_whitespace().next().unwrap_or(field);
                if let Some((w, h)) = core.split_once('x') {
                    if let (Ok(w), Ok(h)) = (w.parse::<u32>(), h.parse::<u32>()) {
                        if (16..=16_384).contains(&w) && (16..=16_384).contains(&h) {
                            dims = Some((w, h));
                            break;
                        }
                    }
                }
            }
        }
    }
    dims.map(|(width, height)| MediaInfo {
        width,
        height,
        has_audio,
    })
}

/// Spawn the video decoder: raw RGBA frames of exactly `width×height×4`
/// bytes on stdout, paced to the file's own clock.
pub fn spawn_video_decoder(
    ffmpeg: &Ffmpeg,
    path: &Path,
    looping: bool,
    hw_decode: bool,
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
    cmd.args(["-re", "-i"]).arg(path);
    cmd.args(["-an", "-f", "rawvideo", "-pix_fmt", "rgba", "pipe:1"]);
    cmd.spawn()
        .map_err(|err| format!("could not start the media decoder: {err}"))
}

/// Spawn the audio decoder: interleaved stereo f32le at 48 kHz on stdout,
/// paced to the file's own clock.
pub fn spawn_audio_decoder(ffmpeg: &Ffmpeg, path: &Path, looping: bool) -> Result<Child, String> {
    let mut cmd = command(ffmpeg);
    cmd.stdout(Stdio::piped()).stderr(Stdio::null());
    cmd.args(["-hide_banner", "-v", "error"]);
    if looping {
        cmd.args(["-stream_loop", "-1"]);
    }
    cmd.args(["-re", "-i"]).arg(path);
    cmd.args(["-vn", "-f", "f32le", "-ar", "48000", "-ac", "2", "pipe:1"]);
    cmd.spawn()
        .map_err(|err| format!("could not start the media audio decoder: {err}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn banners_parse_geometry_and_audio() {
        let banner = "Input #0, mov,mp4,m4a,3gp,3g2,mj2, from 'clip.mp4':\n  Duration: 00:00:12.34, start: 0.000000, bitrate: 8123 kb/s\n  Stream #0:0[0x1](und): Video: h264 (High) (avc1 / 0x31637661), yuv420p(progressive), 1920x1080 [SAR 1:1 DAR 16:9], 8000 kb/s, 60 fps, 60 tbr, 15360 tbn (default)\n  Stream #0:1[0x2](und): Audio: aac (LC) (mp4a / 0x6134706D), 48000 Hz, stereo, fltp, 192 kb/s (default)\n";
        assert_eq!(
            parse_banner(banner),
            Some(MediaInfo {
                width: 1920,
                height: 1080,
                has_audio: true
            })
        );
    }

    #[test]
    fn soundless_clips_and_junk_parse_honestly() {
        let soundless = "  Stream #0:0: Video: vp9, yuv420p(tv), 1280x720, 30 fps, 30 tbr\n";
        assert_eq!(
            parse_banner(soundless),
            Some(MediaInfo {
                width: 1280,
                height: 720,
                has_audio: false
            })
        );
        assert_eq!(parse_banner("clip.xyz: Invalid data found"), None);
        // A matrix-size token that is not a geometry must not fool it.
        let weird = "  Stream #0:0: Video: h264, yuv420p, 640x360 [SAR 1:1], 30 fps\n";
        assert_eq!(
            parse_banner(weird).map(|info| (info.width, info.height)),
            Some((640, 360))
        );
    }
}
