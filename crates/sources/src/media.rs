//! The **Media source**: a file composed onto the canvas with its audio in
//! the mixer.
//!
//! Three honest paths:
//! - **Still images** behave exactly like the Image source (decoded once).
//! - **`.frec`** plays through the **owned freally-video codec** — nothing
//!   fetched, ever. Multi-track recordings feed **track 1** to the mixer.
//! - **Wire formats** (mp4/mkv/webm/mov/…) decode through the
//!   clearly-labeled, on-demand **ffmpeg component**: `-re`-paced child
//!   processes hand raw RGBA frames and 48 kHz stereo audio over pipes.
//!   Hardware decode (`-hwaccel auto`) falls back to software by itself.
//!   Without the component installed, the source errors with guidance —
//!   never a silent black box.
//!
//! Video rides the same latest-wins [`CaptureSession`] shape as every
//! capture; audio lands in [`fcap_audio::media_hub`] under the source's id,
//! where the mixer drains it like any capture device. A/V sync note: the
//! wire path paces video and audio in two `-re` processes — alignment is
//! within normal tolerance, and the strip's sync-offset covers the rest;
//! the `.frec` path paces both from one clock.

use std::io::Read;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use fcap_capture::{frame_channel, CaptureError, CaptureSession, Frame, FrameSender, PixelFormat};
use fcap_encode::decode;
use fcap_encode::freally_video::{FrecChunk, FrecReader};

use crate::image::load_image_rgba;

const STILL_EXTS: [&str; 8] = ["png", "jpg", "jpeg", "bmp", "gif", "webp", "tif", "tiff"];

/// Start playing a media file. `hub_id` keys the mixer-side audio ring —
/// the studio passes the source id.
pub fn start_media(
    hub_id: &str,
    path: &str,
    looping: bool,
    hw_decode: bool,
) -> Result<CaptureSession, CaptureError> {
    let path_buf = std::path::PathBuf::from(path);
    if path.trim().is_empty() {
        return Err(CaptureError::Backend(
            "pick a media file in the source's properties".into(),
        ));
    }
    if !path_buf.is_file() {
        return Err(CaptureError::NotFound(format!(
            "media file not found: {path}"
        )));
    }
    let ext = path_buf
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase())
        .unwrap_or_default();

    if STILL_EXTS.contains(&ext.as_str()) {
        return start_still(&path_buf);
    }
    if ext == "frec" {
        return start_frec(hub_id.to_string(), path_buf, looping);
    }
    start_wire(hub_id.to_string(), path_buf, looping, hw_decode)
}

// ---------------------------------------------------------------------------
// Still images (behave like the Image source)
// ---------------------------------------------------------------------------

fn start_still(path: &Path) -> Result<CaptureSession, CaptureError> {
    let frame = load_image_rgba(path).map_err(|err| CaptureError::Backend(err.to_string()))?;
    let (sender, receiver) = frame_channel();
    let stop = Arc::new(AtomicBool::new(false));
    let thread_stop = Arc::clone(&stop);
    let join = std::thread::Builder::new()
        .name("fcap-media-still".into())
        .spawn(move || {
            sender.send(frame);
            while !thread_stop.load(Ordering::Relaxed) && sender.is_open() {
                std::thread::sleep(Duration::from_millis(100));
            }
            sender.close(None);
        })
        .map_err(|err| CaptureError::Backend(err.to_string()))?;
    Ok(CaptureSession::from_parts(receiver, stop, join))
}

// ---------------------------------------------------------------------------
// The owned .frec path (no external tool, ever)
// ---------------------------------------------------------------------------

fn start_frec(
    hub_id: String,
    path: std::path::PathBuf,
    looping: bool,
) -> Result<CaptureSession, CaptureError> {
    // Open once up front so a bad file errors at add time, not mid-render.
    let reader = FrecReader::open(&path).map_err(|err| CaptureError::Backend(err.to_string()))?;
    let (sender, receiver) = frame_channel();
    let stop = Arc::new(AtomicBool::new(false));
    let thread_stop = Arc::clone(&stop);
    let join = std::thread::Builder::new()
        .name("fcap-media-frec".into())
        .spawn(move || run_frec(reader, path, hub_id, looping, sender, thread_stop))
        .map_err(|err| CaptureError::Backend(err.to_string()))?;
    Ok(CaptureSession::from_parts(receiver, stop, join))
}

fn run_frec(
    mut reader: FrecReader,
    path: std::path::PathBuf,
    hub_id: String,
    looping: bool,
    sender: FrameSender,
    stop: Arc<AtomicBool>,
) {
    let ring = fcap_audio::media_hub::ring(&hub_id);
    ring.clear();
    let spec = *reader.spec();
    let format = match spec.pixel_format {
        fcap_encode::freally_video::PixelFormat::Bgra8 => PixelFormat::Bgra8,
        fcap_encode::freally_video::PixelFormat::Rgba8 => PixelFormat::Rgba8,
    };
    let frame_period =
        Duration::from_secs_f64(spec.fps_den.max(1) as f64 / spec.fps_num.max(1) as f64);

    'playback: loop {
        let started = Instant::now();
        let mut frames_sent: u64 = 0;
        loop {
            if stop.load(Ordering::Relaxed) || !sender.is_open() {
                break 'playback;
            }
            match reader.next_chunk() {
                Ok(Some(FrecChunk::Video { pixels, .. })) => {
                    // Pace to the file's clock (chunks are in record order).
                    let due = started + frame_period * frames_sent as u32;
                    let now = Instant::now();
                    if due > now {
                        std::thread::sleep(due - now);
                    }
                    sender.send(Frame {
                        width: spec.width,
                        height: spec.height,
                        stride: spec.width * 4,
                        format,
                        data: pixels,
                        captured_at: Instant::now(),
                    });
                    frames_sent += 1;
                }
                Ok(Some(FrecChunk::Audio { track, samples, .. })) => {
                    // Multi-track recordings feed track 1 (documented).
                    if track == 0 {
                        ring.push(&samples);
                    }
                }
                Ok(None) => break, // end of file
                Err(err) => {
                    sender.close(Some(CaptureError::Backend(format!(
                        "freally-video playback: {err}"
                    ))));
                    return;
                }
            }
        }
        if !looping {
            break;
        }
        match FrecReader::open(&path) {
            Ok(next) => reader = next,
            Err(err) => {
                sender.close(Some(CaptureError::Backend(format!(
                    "freally-video loop restart: {err}"
                ))));
                return;
            }
        }
    }
    sender.close(None);
}

// ---------------------------------------------------------------------------
// Wire formats — the labeled, on-demand ffmpeg path
// ---------------------------------------------------------------------------

fn start_wire(
    hub_id: String,
    path: std::path::PathBuf,
    looping: bool,
    hw_decode: bool,
) -> Result<CaptureSession, CaptureError> {
    let Some(ffmpeg) = fcap_encode::ffmpeg::installed() else {
        return Err(CaptureError::Backend(
            "playing this format needs the ffmpeg component — install it from Components \
             (the owned .frec format plays with nothing extra)"
                .into(),
        ));
    };
    let info = decode::probe_media(&ffmpeg, &path).map_err(CaptureError::Backend)?;
    let video = decode::spawn_video_decoder(&ffmpeg, &path, looping, hw_decode)
        .map_err(CaptureError::Backend)?;
    let audio = if info.has_audio {
        Some(decode::spawn_audio_decoder(&ffmpeg, &path, looping).map_err(CaptureError::Backend)?)
    } else {
        None
    };

    let (sender, receiver) = frame_channel();
    let stop = Arc::new(AtomicBool::new(false));
    let thread_stop = Arc::clone(&stop);
    let join = std::thread::Builder::new()
        .name("fcap-media-wire".into())
        .spawn(move || run_wire(video, audio, info, hub_id, sender, thread_stop))
        .map_err(|err| CaptureError::Backend(err.to_string()))?;
    Ok(CaptureSession::from_parts(receiver, stop, join))
}

fn run_wire(
    mut video: std::process::Child,
    audio: Option<std::process::Child>,
    info: decode::MediaInfo,
    hub_id: String,
    sender: FrameSender,
    stop: Arc<AtomicBool>,
) {
    // The stop watchdog kills the children the moment stop is set, which
    // unblocks any thread sitting in a pipe read — a wedged decoder can
    // never wedge the studio's reconcile.
    let mut audio = audio;
    let audio_stdout = audio.as_mut().and_then(|child| child.stdout.take());
    let video_stdout = video.stdout.take();
    let watchdog_stop = Arc::clone(&stop);
    let (kill_tx, kill_rx) = std::sync::mpsc::channel::<()>();
    let watchdog = std::thread::Builder::new()
        .name("fcap-media-watchdog".into())
        .spawn(move || {
            loop {
                if watchdog_stop.load(Ordering::Relaxed) {
                    break;
                }
                match kill_rx.recv_timeout(Duration::from_millis(100)) {
                    Ok(()) | Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
                }
            }
            let _ = video.kill();
            let _ = video.wait();
            if let Some(mut child) = audio {
                let _ = child.kill();
                let _ = child.wait();
            }
        })
        .ok();

    // The audio pump: decoded 48 kHz stereo straight into the mixer's ring.
    let audio_thread = audio_stdout.map(|mut stdout| {
        let ring = fcap_audio::media_hub::ring(&hub_id);
        ring.clear();
        std::thread::Builder::new()
            .name("fcap-media-audio".into())
            .spawn(move || {
                let mut bytes = [0u8; 3840]; // one 10 ms stereo f32 block
                let mut samples = vec![0.0f32; 960];
                while read_exact_or_end(&mut stdout, &mut bytes) {
                    for (sample, chunk) in samples.iter_mut().zip(bytes.chunks_exact(4)) {
                        *sample = f32::from_le_bytes(chunk.try_into().expect("4 bytes"));
                    }
                    ring.push(&samples);
                }
            })
            .ok()
    });

    // The video pump: exact frames off the pipe (ffmpeg -re paces them).
    let frame_bytes = info.width as usize * info.height as usize * 4;
    let mut ended_cleanly = false;
    if let Some(mut stdout) = video_stdout {
        let mut data = vec![0u8; frame_bytes];
        loop {
            if stop.load(Ordering::Relaxed) || !sender.is_open() {
                ended_cleanly = true; // stopped by the studio — not an error
                break;
            }
            if !read_exact_or_end(&mut stdout, &mut data) {
                ended_cleanly = true; // end of file (or the child was killed)
                break;
            }
            sender.send(Frame {
                width: info.width,
                height: info.height,
                stride: info.width * 4,
                format: PixelFormat::Rgba8,
                data: data.clone(),
                captured_at: Instant::now(),
            });
        }
    }

    // Wind down: kill the children (via the watchdog), join the pumps.
    let _ = kill_tx.send(());
    if let Some(handle) = watchdog {
        let _ = handle.join();
    }
    if let Some(Some(handle)) = audio_thread {
        let _ = handle.join();
    }
    if ended_cleanly {
        sender.close(None);
    } else {
        sender.close(Some(CaptureError::Backend(
            "the media decoder stopped unexpectedly".into(),
        )));
    }
}

/// `read_exact` that treats EOF/broken pipe as a clean end.
fn read_exact_or_end(reader: &mut impl Read, buf: &mut [u8]) -> bool {
    let mut filled = 0usize;
    while filled < buf.len() {
        match reader.read(&mut buf[filled..]) {
            Ok(0) => return false,
            Ok(n) => filled += n,
            Err(err) if err.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(_) => return false,
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_and_empty_paths_error_honestly() {
        assert!(matches!(
            start_media("test", "", false, true),
            Err(CaptureError::Backend(_))
        ));
        assert!(matches!(
            start_media("test", "C:/definitely/not/here.mp4", false, true),
            Err(CaptureError::NotFound(_))
        ));
    }

    #[test]
    fn a_frec_file_plays_natively_with_audio_in_the_hub() {
        use fcap_encode::freally_video::{FrecSpec, FrecWriter};
        let dir = std::env::temp_dir().join(format!(
            "fcap-media-frec-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).expect("mkdir");
        let path = dir.join("clip.frec");
        let spec = FrecSpec {
            width: 32,
            height: 16,
            fps_num: 100, // fast playback keeps the test quick
            fps_den: 1,
            pixel_format: fcap_encode::freally_video::PixelFormat::Rgba8,
            audio_tracks: 1,
            sample_rate: 48_000,
        };
        let mut writer = FrecWriter::create(&path, spec).expect("create");
        for t in 0..10u64 {
            let frame = vec![(t * 20) as u8; 32 * 16 * 4];
            writer.write_frame(&frame).expect("frame");
            writer
                .write_audio(0, t * 480, &vec![0.25f32; 960])
                .expect("audio");
        }
        writer.finish().expect("finish");

        let hub_id = format!("media-test-{}", std::process::id());
        let session =
            start_media(&hub_id, path.to_str().expect("utf8"), false, true).expect("starts");
        let mut frames = 0;
        loop {
            match session.frames().recv_timeout(Duration::from_secs(2)) {
                Ok(Some(frame)) => {
                    assert_eq!((frame.width, frame.height), (32, 16));
                    frames += 1;
                }
                Ok(None) => continue,
                Err(CaptureError::Stopped) => break,
                Err(err) => panic!("unexpected error: {err}"),
            }
        }
        assert_eq!(frames, 10, "every frame plays");
        let ring = fcap_audio::media_hub::ring(&hub_id);
        assert_eq!(ring.len(), 10 * 960, "track 1 audio landed in the hub");
        fcap_audio::media_hub::retain(&[]);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn wire_formats_without_the_component_guide_the_user() {
        // Only meaningful when no ffmpeg component is installed — on a dev
        // machine with one, the error path is exercised by the probe of a
        // junk file instead.
        let dir = std::env::temp_dir().join(format!("fcap-media-wire-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("mkdir");
        let path = dir.join("fake.mp4");
        std::fs::write(&path, b"not actually an mp4").expect("write");
        let result = start_media("test-wire", path.to_str().expect("utf8"), false, true);
        match result {
            Err(CaptureError::Backend(message)) => {
                assert!(
                    message.contains("ffmpeg component") || message.contains("video stream"),
                    "honest guidance either way, got: {message}"
                );
            }
            Err(other) => panic!("junk mp4 must error with guidance, got {other}"),
            Ok(_) => panic!("junk mp4 must not start a session"),
        }
        let _ = std::fs::remove_dir_all(&dir);
    }
}
