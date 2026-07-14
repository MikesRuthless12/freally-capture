//! The **Media source**: a file composed onto the canvas with its audio in
//! the mixer.
//!
//! Four honest paths:
//! - **Still images** behave exactly like the Image source (decoded once).
//! - **`.gif`** animates through the **owned image decoder** — nothing
//!   fetched, ever. Per-frame delays are honored; a single-frame GIF holds
//!   like a still.
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

use std::collections::HashMap;
use std::io::Read;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

use fcap_capture::{frame_channel, CaptureError, CaptureSession, Frame, FrameSender, PixelFormat};
use fcap_encode::decode;
use fcap_encode::freally_video::{FrecChunk, FrecReader};

use crate::image::load_image_rgba;

const STILL_EXTS: [&str; 7] = ["png", "jpg", "jpeg", "bmp", "webp", "tif", "tiff"];

// --- Playback pause control (TASK: embed & critique a video mid-stream) -----
//
// A Media source's decode loop **holds position** while its pause flag is set
// — the compositor keeps showing the last frame (latest-wins) and no audio is
// pushed — so a streamer can pause a video to talk over it, then resume, then
// remove it, all live on the broadcast. Keyed by the source id (== the audio
// hub id). Entries are tiny and persist for the process; toggling is cheap.

fn pause_registry() -> &'static Mutex<HashMap<String, Arc<AtomicBool>>> {
    static REG: OnceLock<Mutex<HashMap<String, Arc<AtomicBool>>>> = OnceLock::new();
    REG.get_or_init(|| Mutex::new(HashMap::new()))
}

fn pause_flag(id: &str) -> Arc<AtomicBool> {
    pause_registry()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .entry(id.to_string())
        .or_insert_with(|| Arc::new(AtomicBool::new(false)))
        .clone()
}

/// Pause or resume a Media source's playback (the studio command drives this).
/// Idempotent; a fresh source starts unpaused.
pub fn set_media_paused(id: &str, paused: bool) {
    pause_flag(id).store(paused, Ordering::Relaxed);
}

/// Whether a Media source is currently paused.
pub fn is_media_paused(id: &str) -> bool {
    pause_flag(id).load(Ordering::Relaxed)
}

// --- Transport (position/duration + seek) ------------------------------------
//
// Same shape as the pause registry: keyed by the source id, tiny, persistent
// for the process. Decode loops publish `(position, duration)` as they play
// and consume seek requests at frame boundaries; the transport UI polls the
// state while its scrubber is open. A duration of `0.0` means "not known
// yet" — a `.frec`'s total is only learned at its first end-of-file.

fn transport_registry() -> &'static Mutex<HashMap<String, (f32, f32)>> {
    static REG: OnceLock<Mutex<HashMap<String, (f32, f32)>>> = OnceLock::new();
    REG.get_or_init(|| Mutex::new(HashMap::new()))
}

fn seek_registry() -> &'static Mutex<HashMap<String, f32>> {
    static REG: OnceLock<Mutex<HashMap<String, f32>>> = OnceLock::new();
    REG.get_or_init(|| Mutex::new(HashMap::new()))
}

/// The UI-facing transport state: `(position_secs, duration_secs)`.
/// Duration `0.0` = unknown (the scrubber disables until it is).
pub fn media_transport(id: &str) -> (f32, f32) {
    transport_registry()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .get(id)
        .copied()
        .unwrap_or((0.0, 0.0))
}

/// Request a jump to `seconds` (the decode loop clamps and applies it at the
/// next frame boundary; the newest request wins). Seeking while paused still
/// shows the sought frame — the loops emit exactly one frame after a seek.
pub fn media_seek(id: &str, seconds: f32) {
    if seconds.is_finite() {
        seek_registry()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .insert(id.to_string(), seconds.max(0.0));
    }
}

/// Loop-side: consume the pending seek request, if any.
fn take_seek(id: &str) -> Option<f32> {
    seek_registry()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .remove(id)
}

/// Loop-side: publish the playhead for the transport UI.
fn publish_transport(id: &str, position: f32, duration: f32) {
    transport_registry()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .insert(id.to_string(), (position.max(0.0), duration.max(0.0)));
}

/// Start playing a media file. `hub_id` keys the mixer-side audio ring —
/// the studio passes the source id. `reverse` plays the file backwards:
/// GIFs reverse through the owned decoder; `.frec` and the wire formats
/// render a reversed copy once (cached) through the labeled ffmpeg
/// component, then play it with the full transport (seek/pause/loop).
pub fn start_media(
    hub_id: &str,
    path: &str,
    looping: bool,
    hw_decode: bool,
    reverse: bool,
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
        return start_still(&path_buf); // reverse is meaningless for a still
    }
    if ext == "gif" {
        return start_gif(hub_id.to_string(), path_buf, looping, reverse);
    }
    if ext == "frec" {
        if reverse {
            let reversed = ensure_reversed_frec(&path_buf)?;
            return start_wire(hub_id.to_string(), reversed, looping, hw_decode);
        }
        return start_frec(hub_id.to_string(), path_buf, looping);
    }
    if reverse {
        let reversed = ensure_reversed_wire(&path_buf)?;
        return start_wire(hub_id.to_string(), reversed, looping, hw_decode);
    }
    start_wire(hub_id.to_string(), path_buf, looping, hw_decode)
}

// ---------------------------------------------------------------------------
// Reversed-copy cache (true reverse playback for .frec + wire formats)
// ---------------------------------------------------------------------------
//
// Reversing a video stream live is not bounded (every codec decodes forward),
// so reverse playback renders a **reversed copy once** — segment-by-segment,
// bounded memory — caches it in the temp dir keyed by (path, size, mtime),
// and then plays that file through the ordinary wire path: looping, pause,
// seek, and the record-sync hold all work on it unchanged. Slow for long
// files by nature; the session sits in its honest "starting" state while the
// render runs, and the cache makes every later start instant.

/// The cached reversed copy for `src` (keyed by path + size + mtime, so an
/// edited file re-renders and an unchanged one never does).
fn reversed_cache_path(src: &Path, tag: &str) -> Result<std::path::PathBuf, CaptureError> {
    use std::hash::{Hash, Hasher};
    let meta = std::fs::metadata(src)
        .map_err(|err| CaptureError::Backend(format!("could not stat {}: {err}", src.display())))?;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    src.to_string_lossy().hash(&mut hasher);
    meta.len().hash(&mut hasher);
    if let Ok(modified) = meta.modified() {
        if let Ok(since) = modified.duration_since(std::time::UNIX_EPOCH) {
            since.as_secs().hash(&mut hasher);
        }
    }
    Ok(std::env::temp_dir().join(format!("fcap-reversed-{tag}-{:016x}.mp4", hasher.finish())))
}

fn require_ffmpeg() -> Result<fcap_encode::ffmpeg::Ffmpeg, CaptureError> {
    fcap_encode::ffmpeg::installed().ok_or_else(|| {
        CaptureError::Backend(
            "reverse playback renders a reversed copy through the ffmpeg component — \
             install it from Components"
                .into(),
        )
    })
}

/// Reversed copy of a wire-format file (video + audio), rendered once.
fn ensure_reversed_wire(src: &Path) -> Result<std::path::PathBuf, CaptureError> {
    let cache = reversed_cache_path(src, "wire")?;
    if cache.is_file() {
        return Ok(cache);
    }
    let ffmpeg = require_ffmpeg()?;
    let info = decode::probe_media(&ffmpeg, src).map_err(CaptureError::Backend)?;
    let work = std::env::temp_dir().join(format!(
        "fcap-reverse-work-{}-{}",
        std::process::id(),
        cache.file_stem().and_then(|s| s.to_str()).unwrap_or("x")
    ));
    decode::reverse_wire_file(&ffmpeg, src, &work, &cache, info.has_audio)
        .map_err(CaptureError::Backend)?;
    Ok(cache)
}

/// Reversed copy of a `.frec`: decode natively, bridge the frames to an MP4
/// through the labeled component, then reverse that. **Video only** — a
/// reversed backdrop is a visual; the honest limitation is documented where
/// the toggle lives.
fn ensure_reversed_frec(src: &Path) -> Result<std::path::PathBuf, CaptureError> {
    let cache = reversed_cache_path(src, "frec")?;
    if cache.is_file() {
        return Ok(cache);
    }
    let ffmpeg = require_ffmpeg()?;
    let mut reader = FrecReader::open(src).map_err(|err| CaptureError::Backend(err.to_string()))?;
    let spec = *reader.spec();
    let forward = cache.with_extension("fwd.mp4");
    let mut encoder = decode::spawn_rawvideo_encoder(
        &ffmpeg,
        spec.width,
        spec.height,
        spec.fps_num.max(1),
        spec.fps_den.max(1),
        &forward,
    )
    .map_err(CaptureError::Backend)?;
    {
        use std::io::Write;
        let mut stdin = encoder
            .stdin
            .take()
            .ok_or_else(|| CaptureError::Backend("encoder stdin unavailable".into()))?;
        // frec frames may be BGRA; the bridge feeds RGBA, so swizzle when needed.
        let swizzle = matches!(
            spec.pixel_format,
            fcap_encode::freally_video::PixelFormat::Bgra8
        );
        loop {
            match reader.next_chunk() {
                Ok(Some(FrecChunk::Video { mut pixels, .. })) => {
                    if swizzle {
                        for px in pixels.chunks_exact_mut(4) {
                            px.swap(0, 2);
                        }
                    }
                    if stdin.write_all(&pixels).is_err() {
                        break; // the encoder died — its exit status reports below
                    }
                }
                Ok(Some(FrecChunk::Audio { .. })) => {} // video-only, documented
                Ok(None) => break,
                Err(err) => {
                    let _ = encoder.kill();
                    let _ = encoder.wait();
                    let _ = std::fs::remove_file(&forward);
                    return Err(CaptureError::Backend(format!(
                        "freally-video reverse bridge: {err}"
                    )));
                }
            }
        }
    }
    let status = encoder
        .wait()
        .map_err(|err| CaptureError::Backend(err.to_string()))?;
    if !status.success() {
        let _ = std::fs::remove_file(&forward);
        return Err(CaptureError::Backend(
            "the reverse bridge encoder failed".into(),
        ));
    }
    let work = std::env::temp_dir().join(format!(
        "fcap-reverse-work-{}-{}",
        std::process::id(),
        cache.file_stem().and_then(|s| s.to_str()).unwrap_or("x")
    ));
    let result = decode::reverse_wire_file(&ffmpeg, &forward, &work, &cache, false)
        .map_err(CaptureError::Backend);
    let _ = std::fs::remove_file(&forward);
    result?;
    Ok(cache)
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
// Animated GIFs — the owned image decoder (no external tool, ever)
// ---------------------------------------------------------------------------

fn start_gif(
    hub_id: String,
    path: std::path::PathBuf,
    looping: bool,
    reverse: bool,
) -> Result<CaptureSession, CaptureError> {
    // Open once up front so a bad file errors at add time, not mid-render.
    let decoder = open_gif(&path)?;
    let (sender, receiver) = frame_channel();
    let stop = Arc::new(AtomicBool::new(false));
    let thread_stop = Arc::clone(&stop);
    let join = std::thread::Builder::new()
        .name("fcap-media-gif".into())
        .spawn(move || run_gif(decoder, path, hub_id, looping, reverse, sender, thread_stop))
        .map_err(|err| CaptureError::Backend(err.to_string()))?;
    Ok(CaptureSession::from_parts(receiver, stop, join))
}

fn open_gif(
    path: &Path,
) -> Result<image::codecs::gif::GifDecoder<std::io::BufReader<std::fs::File>>, CaptureError> {
    let file = std::fs::File::open(path).map_err(|err| {
        CaptureError::Backend(format!("could not read {}: {err}", path.display()))
    })?;
    image::codecs::gif::GifDecoder::new(std::io::BufReader::new(file))
        .map_err(|err| CaptureError::Backend(format!("could not decode {}: {err}", path.display())))
}

/// One decoded, size-checked GIF frame ready to send.
struct GifFrame {
    width: u32,
    height: u32,
    data: Vec<u8>,
    delay: Duration,
}

/// Decode + size-check one frame of the animation; per-frame delay with the
/// browser convention for "as fast as possible" encodings (< 20 ms → 100 ms).
fn gif_frame(frame: Result<image::Frame, image::ImageError>) -> Result<GifFrame, CaptureError> {
    let frame = frame.map_err(|err| CaptureError::Backend(format!("gif playback: {err}")))?;
    let (numer, denom) = frame.delay().numer_denom_ms();
    let mut delay = Duration::from_millis(u64::from(numer / denom.max(1)));
    if delay < Duration::from_millis(20) {
        delay = Duration::from_millis(100);
    }
    let buffer = frame.into_buffer();
    let (width, height) = (buffer.width(), buffer.height());
    if width == 0
        || height == 0
        || width > crate::static_source::MAX_STATIC_DIMENSION
        || height > crate::static_source::MAX_STATIC_DIMENSION
    {
        return Err(CaptureError::Backend(format!(
            "gif playback: {width}×{height} frame is outside the supported size"
        )));
    }
    Ok(GifFrame {
        width,
        height,
        data: buffer.into_raw(),
        delay,
    })
}

/// The largest decoded-frame total a reversed GIF may buffer. Reverse needs
/// the whole pass in memory (frames only decode forward); past this the
/// source errors honestly instead of eating the machine.
const GIF_REVERSE_CAP_BYTES: usize = 256 * 1024 * 1024;

fn run_gif(
    decoder: image::codecs::gif::GifDecoder<std::io::BufReader<std::fs::File>>,
    path: std::path::PathBuf,
    hub_id: String,
    looping: bool,
    reverse: bool,
    sender: FrameSender,
    stop: Arc<AtomicBool>,
) {
    use image::AnimationDecoder;

    let pause = pause_flag(&hub_id);
    // Reverse buffers one pass up front (bounded), then plays indices
    // backwards with full seek support; forward streams and re-decodes on
    // loop/backward-seek, holding only one frame in memory.
    if reverse {
        let mut frames: Vec<GifFrame> = Vec::new();
        let mut bytes = 0usize;
        for frame in decoder.into_frames() {
            if stop.load(Ordering::Relaxed) || !sender.is_open() {
                sender.close(None);
                return;
            }
            let frame = match gif_frame(frame) {
                Ok(frame) => frame,
                Err(err) => {
                    sender.close(Some(err));
                    return;
                }
            };
            bytes += frame.data.len();
            if bytes > GIF_REVERSE_CAP_BYTES {
                sender.close(Some(CaptureError::Backend(
                    "this GIF is too large to reverse (over 256 MiB decoded) — \
                     it plays forward fine"
                        .into(),
                )));
                return;
            }
            frames.push(frame);
        }
        run_gif_reversed(frames, &hub_id, looping, &sender, &stop, &pause);
        sender.close(None);
        return;
    }

    // The forward loop holds internally at end-of-animation (the scrubber
    // stays live) and only returns on stop or error.
    run_gif_forward(decoder, &path, &hub_id, looping, &sender, &stop, &pause);
    sender.close(None);
}

/// Streamed forward playback with the transport: publishes the playhead,
/// consumes seeks (forward = skip within the pass; backward = re-decode from
/// the top), and after a seek emits exactly one frame even while paused so a
/// paused scrub still updates the canvas.
#[allow(clippy::too_many_arguments)]
fn run_gif_forward(
    mut decoder: image::codecs::gif::GifDecoder<std::io::BufReader<std::fs::File>>,
    path: &Path,
    hub_id: &str,
    looping: bool,
    sender: &FrameSender,
    stop: &Arc<AtomicBool>,
    pause: &Arc<AtomicBool>,
) {
    use image::AnimationDecoder;

    let mut duration = 0.0f32; // learned at the end of the first full pass
    let mut pending_seek: Option<f32> = None;
    'playback: loop {
        let mut clock = 0.0f32;
        let mut pass_frames: u64 = 0;
        let mut force_frame = false;
        for frame in decoder.into_frames() {
            if stop.load(Ordering::Relaxed) || !sender.is_open() {
                break 'playback;
            }
            let frame = match gif_frame(frame) {
                Ok(frame) => frame,
                Err(err) => {
                    sender.close(Some(err));
                    return;
                }
            };
            let delay_secs = frame.delay.as_secs_f32();
            // A pending forward seek skips frames without pacing.
            if let Some(target) = pending_seek {
                if clock + delay_secs <= target {
                    clock += delay_secs;
                    pass_frames += 1;
                    continue;
                }
                pending_seek = None;
                force_frame = true; // show the sought frame even while paused
            }
            // Pause: hold (no bursting) — but a seek while paused re-enters
            // the seek path so the held picture follows the scrubber.
            while !force_frame
                && pause.load(Ordering::Relaxed)
                && !stop.load(Ordering::Relaxed)
                && sender.is_open()
            {
                if let Some(target) = take_seek(hub_id) {
                    pending_seek = Some(target);
                    if target < clock {
                        // Backward: re-decode from the top of the file.
                        match open_gif(path) {
                            Ok(next) => decoder = next,
                            Err(err) => {
                                sender.close(Some(err));
                                return;
                            }
                        }
                        continue 'playback;
                    }
                    break; // skip forward from here
                }
                std::thread::sleep(Duration::from_millis(30));
            }
            if pending_seek.is_some() {
                // The paused-seek above targets a later frame: skip on.
                // (Reaching the target frame just falls through to the send
                // below — the pause gate is already behind us.)
                if clock + delay_secs <= pending_seek.expect("just set") {
                    clock += delay_secs;
                    pass_frames += 1;
                    continue;
                }
                pending_seek = None;
            }
            sender.send(Frame {
                width: frame.width,
                height: frame.height,
                stride: frame.width * 4,
                format: PixelFormat::Rgba8,
                data: frame.data,
                captured_at: Instant::now(),
            });
            force_frame = false;
            publish_transport(hub_id, clock, duration);
            // Pace in short slices so stop/seek stay responsive.
            let due = Instant::now() + frame.delay;
            while Instant::now() < due {
                if stop.load(Ordering::Relaxed) || !sender.is_open() {
                    break 'playback;
                }
                if let Some(target) = take_seek(hub_id) {
                    pending_seek = Some(target);
                    if target < clock {
                        // Backward: re-decode from the top of the file.
                        match open_gif(path) {
                            Ok(next) => decoder = next,
                            Err(err) => {
                                sender.close(Some(err));
                                return;
                            }
                        }
                        continue 'playback;
                    }
                    break;
                }
                std::thread::sleep((due - Instant::now()).min(Duration::from_millis(30)));
            }
            clock += delay_secs;
            pass_frames += 1;
        }
        if duration == 0.0 {
            duration = clock;
            publish_transport(hub_id, clock.min(duration), duration);
        }
        // A seek past the end lands on the top of the next pass.
        if let Some(target) = pending_seek {
            if duration > 0.0 && target >= duration {
                pending_seek = None;
            }
        }
        // A single-frame (or empty) file behaves like a still; a finished
        // non-looping animation holds its last frame — but the scrubber
        // still works: a seek re-decodes from the top.
        if pending_seek.is_none() && (!looping || pass_frames <= 1) {
            loop {
                if stop.load(Ordering::Relaxed) || !sender.is_open() {
                    break 'playback;
                }
                if let Some(target) = take_seek(hub_id) {
                    pending_seek = Some(target);
                    break;
                }
                std::thread::sleep(Duration::from_millis(50));
            }
        }
        match open_gif(path) {
            Ok(next) => decoder = next,
            Err(err) => {
                sender.close(Some(err));
                return;
            }
        }
    }
}

/// Buffered reverse playback: the pass is in memory, so seek is an index
/// jump. The playhead runs 0 → duration as the reversed frames play.
fn run_gif_reversed(
    frames: Vec<GifFrame>,
    hub_id: &str,
    looping: bool,
    sender: &FrameSender,
    stop: &Arc<AtomicBool>,
    pause: &Arc<AtomicBool>,
) {
    if frames.is_empty() {
        return;
    }
    let duration: f32 = frames.iter().map(|frame| frame.delay.as_secs_f32()).sum();
    publish_transport(hub_id, 0.0, duration);
    // Playback step k shows source frame n-1-k; its display time is its own
    // delay. `starts[k]` = the playhead when step k begins.
    let order: Vec<usize> = (0..frames.len()).rev().collect();
    let mut starts = Vec::with_capacity(order.len());
    let mut clock = 0.0f32;
    for &index in &order {
        starts.push(clock);
        clock += frames[index].delay.as_secs_f32();
    }
    let seek_step = |target: f32| -> usize {
        starts
            .iter()
            .rposition(|&start| start <= target)
            .unwrap_or(0)
    };

    let mut step = 0usize;
    let mut force_frame = false;
    'playback: loop {
        if stop.load(Ordering::Relaxed) || !sender.is_open() {
            return;
        }
        while !force_frame
            && pause.load(Ordering::Relaxed)
            && !stop.load(Ordering::Relaxed)
            && sender.is_open()
        {
            if let Some(target) = take_seek(hub_id) {
                // The break falls through to the send below, so the sought
                // frame shows even while paused.
                step = seek_step(target);
                break;
            }
            std::thread::sleep(Duration::from_millis(30));
        }
        let frame = &frames[order[step]];
        sender.send(Frame {
            width: frame.width,
            height: frame.height,
            stride: frame.width * 4,
            format: PixelFormat::Rgba8,
            data: frame.data.clone(),
            captured_at: Instant::now(),
        });
        force_frame = false;
        publish_transport(hub_id, starts[step], duration);
        let due = Instant::now() + frame.delay;
        while Instant::now() < due {
            if stop.load(Ordering::Relaxed) || !sender.is_open() {
                return;
            }
            if let Some(target) = take_seek(hub_id) {
                step = seek_step(target);
                force_frame = true;
                continue 'playback;
            }
            std::thread::sleep((due - Instant::now()).min(Duration::from_millis(30)));
        }
        step += 1;
        if step >= order.len() {
            if !looping || order.len() <= 1 {
                break;
            }
            step = 0;
        }
    }
    // Non-looping (or single-frame): hold the last frame until stopped —
    // but a seek while holding still moves the picture.
    while !stop.load(Ordering::Relaxed) && sender.is_open() {
        if let Some(target) = take_seek(hub_id) {
            let step = seek_step(target);
            let frame = &frames[order[step]];
            sender.send(Frame {
                width: frame.width,
                height: frame.height,
                stride: frame.width * 4,
                format: PixelFormat::Rgba8,
                data: frame.data.clone(),
                captured_at: Instant::now(),
            });
            publish_transport(hub_id, starts[step], duration);
        }
        std::thread::sleep(Duration::from_millis(50));
    }
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
    let fps = spec.fps_num.max(1) as f32 / spec.fps_den.max(1) as f32;
    let frame_period =
        Duration::from_secs_f64(spec.fps_den.max(1) as f64 / spec.fps_num.max(1) as f64);

    let pause = pause_flag(&hub_id);
    let mut duration = 0.0f32; // learned at the first end-of-file
    let mut last_index: u64 = 0;
    // Seek: drop chunks until this frame, then show it even while paused.
    let mut skip_until: Option<u64> = None;
    let mut force_frame = false;
    // Turns a seek target into loop state; backward targets re-open the file
    // (frec decodes forward). Returns the fresh reader when it re-opened.
    let seek_to = |target: f32, current: u64| -> (u64, Option<Result<FrecReader, String>>) {
        let target_frame = (target.max(0.0) * fps) as u64;
        let reopen = (target_frame < current)
            .then(|| FrecReader::open(&path).map_err(|err| format!("freally-video seek: {err}")));
        (target_frame, reopen)
    };
    'playback: loop {
        let mut started = Instant::now();
        let mut paced: u64 = 0; // frames paced since the clock last reset
        loop {
            if stop.load(Ordering::Relaxed) || !sender.is_open() {
                break 'playback;
            }
            if let Some(target) = take_seek(&hub_id) {
                let (frame, reopen) = seek_to(target, last_index);
                if let Some(fresh) = reopen {
                    match fresh {
                        Ok(next) => reader = next,
                        Err(err) => {
                            sender.close(Some(CaptureError::Backend(err)));
                            return;
                        }
                    }
                }
                skip_until = Some(frame);
                force_frame = true;
                ring.clear(); // queued audio from before the jump must not play
            }
            // Pause: hold the last frame (no video/audio pushed) until resumed,
            // then shift the clock so playback continues — never bursts to
            // "catch up". A seek while paused re-enters the seek path above so
            // the held picture follows the scrubber.
            if pause.load(Ordering::Relaxed) && !force_frame {
                let paused_at = Instant::now();
                while pause.load(Ordering::Relaxed)
                    && !stop.load(Ordering::Relaxed)
                    && sender.is_open()
                    && seek_registry()
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner)
                        .get(hub_id.as_str())
                        .is_none()
                {
                    std::thread::sleep(Duration::from_millis(30));
                }
                started += paused_at.elapsed();
                continue;
            }
            match reader.next_chunk() {
                Ok(Some(FrecChunk::Video {
                    frame_index,
                    pixels,
                })) => {
                    last_index = frame_index;
                    if let Some(target) = skip_until {
                        if frame_index < target {
                            continue; // fast-skip toward the seek target
                        }
                        skip_until = None;
                        started = Instant::now();
                        paced = 0;
                    }
                    // Pace to the file's clock (chunks are in record order).
                    let due = started + frame_period * paced as u32;
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
                    force_frame = false;
                    publish_transport(&hub_id, frame_index as f32 / fps, duration);
                    paced += 1;
                }
                Ok(Some(FrecChunk::Audio { track, samples, .. })) => {
                    // Multi-track recordings feed track 1 (documented); audio
                    // under a seek-skip is dropped, not queued.
                    if track == 0 && skip_until.is_none() {
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
        // The first complete pass teaches the transport its duration; a seek
        // past the end simply lands here and holds/loops like a normal end.
        if duration == 0.0 {
            duration = (last_index + 1) as f32 / fps;
            publish_transport(&hub_id, duration, duration);
        }
        skip_until = None;
        if !looping {
            // Hold the last frame, scrubber still live: a seek re-opens.
            loop {
                if stop.load(Ordering::Relaxed) || !sender.is_open() {
                    break 'playback;
                }
                if let Some(target) = take_seek(&hub_id) {
                    let target_frame = (target.max(0.0) * fps) as u64;
                    skip_until = Some(target_frame);
                    force_frame = true;
                    ring.clear();
                    break;
                }
                std::thread::sleep(Duration::from_millis(50));
            }
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
        last_index = 0;
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
    // Probe up front so a bad file errors at add time, not mid-render.
    let info = decode::probe_media(&ffmpeg, &path).map_err(CaptureError::Backend)?;

    let (sender, receiver) = frame_channel();
    let stop = Arc::new(AtomicBool::new(false));
    let thread_stop = Arc::clone(&stop);
    let join = std::thread::Builder::new()
        .name("fcap-media-wire".into())
        .spawn(move || {
            run_wire(
                ffmpeg,
                path,
                looping,
                hw_decode,
                info,
                hub_id,
                sender,
                thread_stop,
            )
        })
        .map_err(|err| CaptureError::Backend(err.to_string()))?;
    Ok(CaptureSession::from_parts(receiver, stop, join))
}

/// The wire session: decoder children are (re)spawned per playback stretch —
/// once at the start, and again at each seek (`-ss` input seek) or at a seek
/// after a non-looping end. Between stretches the last frame holds on the
/// canvas, so scrubbing feels like a regular player.
#[allow(clippy::too_many_arguments)]
fn run_wire(
    ffmpeg: fcap_encode::ffmpeg::Ffmpeg,
    path: std::path::PathBuf,
    looping: bool,
    hw_decode: bool,
    info: decode::MediaInfo,
    hub_id: String,
    sender: FrameSender,
    stop: Arc<AtomicBool>,
) {
    let fps = info.fps.unwrap_or(30.0).max(1.0);
    let duration = info.duration_secs.unwrap_or(0.0);
    let mut seek_base = 0.0f32;
    'session: loop {
        let video = match decode::spawn_video_decoder(&ffmpeg, &path, looping, hw_decode, seek_base)
        {
            Ok(child) => child,
            Err(err) => {
                sender.close(Some(CaptureError::Backend(err)));
                return;
            }
        };
        let audio = if info.has_audio {
            match decode::spawn_audio_decoder(&ffmpeg, &path, looping, seek_base) {
                Ok(child) => Some(child),
                Err(err) => {
                    sender.close(Some(CaptureError::Backend(err)));
                    return;
                }
            }
        } else {
            None
        };
        match run_wire_stretch(
            video, audio, &info, fps, duration, seek_base, &hub_id, &sender, &stop,
        ) {
            StretchEnd::Seek(target) => {
                let ceiling = if duration > 0.0 {
                    duration - 0.05
                } else {
                    f32::MAX
                };
                seek_base = target.clamp(0.0, ceiling.max(0.0));
                publish_transport(&hub_id, seek_base, duration);
                continue 'session;
            }
            StretchEnd::Ended => {
                // A looping decoder is run with `-stream_loop -1`, so it never
                // reaches EOF on its own — an EOF here means the ffmpeg child
                // died mid-stream. Surface that (error status + retry) instead
                // of freezing on the last frame with a "live" status, so the
                // operator sees the failure and CAP-N01 source-error rules fire.
                if looping {
                    sender.close(Some(CaptureError::Backend(
                        "the looping media decoder stopped unexpectedly".into(),
                    )));
                    return;
                }
                // Non-looping natural end: hold the last frame, scrubber live.
                loop {
                    if stop.load(Ordering::Relaxed) || !sender.is_open() {
                        sender.close(None);
                        return;
                    }
                    if let Some(target) = take_seek(&hub_id) {
                        let ceiling = if duration > 0.0 {
                            duration - 0.05
                        } else {
                            f32::MAX
                        };
                        seek_base = target.clamp(0.0, ceiling.max(0.0));
                        publish_transport(&hub_id, seek_base, duration);
                        continue 'session;
                    }
                    std::thread::sleep(Duration::from_millis(50));
                }
            }
            StretchEnd::Stopped => {
                sender.close(None);
                return;
            }
            StretchEnd::Failed => {
                sender.close(Some(CaptureError::Backend(
                    "the media decoder stopped unexpectedly".into(),
                )));
                return;
            }
        }
    }
}

/// Why one decoder stretch ended.
enum StretchEnd {
    /// A seek request — respawn the children at the target.
    Seek(f32),
    /// Clean end of file (non-looping).
    Ended,
    /// The studio stopped the session.
    Stopped,
    /// The decoder died mid-file.
    Failed,
}

/// One playback stretch: pump the decoder pipes until stop/seek/EOF.
#[allow(clippy::too_many_arguments)]
fn run_wire_stretch(
    mut video: std::process::Child,
    audio: Option<std::process::Child>,
    info: &decode::MediaInfo,
    fps: f32,
    duration: f32,
    seek_base: f32,
    hub_id: &str,
    sender: &FrameSender,
    stop: &Arc<AtomicBool>,
) -> StretchEnd {
    // The stop watchdog kills the children the moment stop is set, which
    // unblocks any thread sitting in a pipe read — a wedged decoder can
    // never wedge the studio's reconcile.
    let mut audio = audio;
    let audio_stdout = audio.as_mut().and_then(|child| child.stdout.take());
    let video_stdout = video.stdout.take();
    let watchdog_stop = Arc::clone(stop);
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

    let pause = pause_flag(hub_id);
    // The audio pump: decoded 48 kHz stereo straight into the mixer's ring.
    // Each stretch clears the ring, so a seek never plays pre-jump audio.
    let audio_thread = audio_stdout.map(|mut stdout| {
        let ring = fcap_audio::media_hub::ring(hub_id);
        ring.clear();
        let audio_stop = Arc::clone(stop);
        let audio_pause = Arc::clone(&pause);
        std::thread::Builder::new()
            .name("fcap-media-audio".into())
            .spawn(move || {
                let mut bytes = [0u8; 3840]; // one 10 ms stereo f32 block
                let mut samples = Vec::with_capacity(960);
                loop {
                    // Pause: stop draining the pipe (ffmpeg -re backpressures
                    // and holds) and push no audio, so a paused video is silent
                    // + frozen on the broadcast.
                    while audio_pause.load(Ordering::Relaxed) && !audio_stop.load(Ordering::Relaxed)
                    {
                        std::thread::sleep(Duration::from_millis(30));
                    }
                    // `filled` may be a whole block, or a short final block at
                    // end-of-stream — push whatever complete stereo frames it
                    // holds so the clip's last < 10 ms of audio isn't dropped.
                    let (filled, done) = read_available(&mut stdout, &mut bytes);
                    let usable = filled - filled % 8; // whole stereo f32 frames
                    if usable > 0 {
                        samples.clear();
                        for chunk in bytes[..usable].chunks_exact(4) {
                            samples.push(f32::from_le_bytes(chunk.try_into().expect("4 bytes")));
                        }
                        ring.push(&samples);
                    }
                    if done {
                        break;
                    }
                }
            })
            .ok()
    });

    // The video pump: exact frames off the pipe (ffmpeg -re paces them).
    let frame_bytes = info.width as usize * info.height as usize * 4;
    let mut end = StretchEnd::Failed;
    if let Some(mut stdout) = video_stdout {
        let mut data = vec![0u8; frame_bytes];
        let mut frames: u64 = 0;
        // The first frame of a stretch sends even while paused, so a paused
        // scrub still updates the canvas with the sought picture.
        let mut force_frame = true;
        loop {
            if stop.load(Ordering::Relaxed) || !sender.is_open() {
                end = StretchEnd::Stopped;
                break;
            }
            if let Some(target) = take_seek(hub_id) {
                end = StretchEnd::Seek(target);
                break;
            }
            // Pause: hold the last frame (don't read the next), which
            // backpressures ffmpeg so playback resumes where it paused. A
            // seek while paused ends the stretch like any other seek.
            while !force_frame
                && pause.load(Ordering::Relaxed)
                && !stop.load(Ordering::Relaxed)
                && sender.is_open()
                && seek_registry()
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .get(hub_id)
                    .is_none()
            {
                std::thread::sleep(Duration::from_millis(30));
            }
            if stop.load(Ordering::Relaxed) || !sender.is_open() {
                end = StretchEnd::Stopped;
                break;
            }
            if pause.load(Ordering::Relaxed) && !force_frame {
                continue; // a seek interrupted the pause-wait
            }
            if !read_exact_or_end(&mut stdout, &mut data) {
                end = StretchEnd::Ended; // end of file (or the child was killed)
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
            force_frame = false;
            frames += 1;
            // The playhead: the seek base plus the frames this stretch has
            // decoded; `-stream_loop` wraps through the duration when known.
            let raw = seek_base + frames as f32 / fps;
            let position = if duration > 0.0 { raw % duration } else { raw };
            publish_transport(hub_id, position, duration);
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
    // A kill mid-pipe reads as EOF; the studio's stop wins over "ended".
    if matches!(end, StretchEnd::Ended) && stop.load(Ordering::Relaxed) {
        end = StretchEnd::Stopped;
    }
    end
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

/// Fill `buf` as far as possible; returns `(bytes_filled, done)`. `done` is
/// true at EOF/broken pipe — the last read of a stream may be a short block,
/// and its bytes must not be discarded (unlike `read_exact_or_end`, which
/// drops a partial final block).
fn read_available(reader: &mut impl Read, buf: &mut [u8]) -> (usize, bool) {
    let mut filled = 0usize;
    while filled < buf.len() {
        match reader.read(&mut buf[filled..]) {
            Ok(0) => return (filled, true),
            Ok(n) => filled += n,
            Err(err) if err.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(_) => return (filled, true),
        }
    }
    (filled, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_available_keeps_a_short_final_block() {
        // 10 full bytes + a 3-byte tail, read into a 4-byte buffer: three full
        // reads, then a partial read of 3 flagged done — the tail is NOT lost.
        let data: Vec<u8> = (0..15u8).collect();
        let mut cursor = std::io::Cursor::new(data);
        let mut buf = [0u8; 4];
        let mut chunks = Vec::new();
        loop {
            let (filled, done) = read_available(&mut cursor, &mut buf);
            if filled > 0 {
                chunks.push((filled, buf[..filled].to_vec()));
            }
            if done {
                break;
            }
        }
        // 15 bytes / 4 = three full (4,4,4) + a final partial (3).
        assert_eq!(
            chunks.iter().map(|(n, _)| *n).collect::<Vec<_>>(),
            vec![4, 4, 4, 3],
            "the trailing partial block survives"
        );
        let total: usize = chunks.iter().map(|(n, _)| *n).sum();
        assert_eq!(total, 15, "no bytes dropped");
    }

    #[test]
    fn missing_and_empty_paths_error_honestly() {
        assert!(matches!(
            start_media("test", "", false, true, false),
            Err(CaptureError::Backend(_))
        ));
        assert!(matches!(
            start_media("test", "C:/definitely/not/here.mp4", false, true, false),
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
        // Hold the ring across playback: the hub is weak-referenced, so the
        // ring is freed once the decoder thread finishes and drops its handle.
        let ring = fcap_audio::media_hub::ring(&hub_id);
        let session =
            start_media(&hub_id, path.to_str().expect("utf8"), false, true, false).expect("starts");
        // A non-looping end now HOLDS (the scrubber stays live) instead of
        // closing — so count the ten frames, then stop the session ourselves.
        let mut frames = 0;
        while frames < 10 {
            match session.frames().recv_timeout(Duration::from_secs(2)) {
                Ok(Some(frame)) => {
                    assert_eq!((frame.width, frame.height), (32, 16));
                    frames += 1;
                }
                Ok(None) => continue,
                Err(err) => panic!("playback ended early: {err}"),
            }
        }
        // The final audio chunk and the end-of-file (which teaches the
        // transport its duration) land just after the last video frame —
        // give the reader a bounded moment to get there.
        let deadline = Instant::now() + Duration::from_secs(2);
        while (ring.len() < 10 * 960 || media_transport(&hub_id).1 == 0.0)
            && Instant::now() < deadline
        {
            std::thread::sleep(Duration::from_millis(10));
        }
        assert_eq!(ring.len(), 10 * 960, "track 1 audio landed in the hub");
        let (position, duration) = media_transport(&hub_id);
        assert!(
            (duration - 0.1).abs() < 0.011,
            "10 frames @ 100 fps = 0.1 s, got {duration}"
        );
        assert!(position <= duration + 0.011, "playhead within the file");
        session.stop();
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn transport_and_seek_registries_roundtrip() {
        media_seek("transport-test", 3.5);
        assert_eq!(take_seek("transport-test"), Some(3.5));
        assert_eq!(take_seek("transport-test"), None, "consumed once");
        media_seek("transport-test", -2.0);
        assert_eq!(take_seek("transport-test"), Some(0.0), "clamped at zero");
        media_seek("transport-test", f32::NAN);
        assert_eq!(take_seek("transport-test"), None, "NaN is ignored");
        publish_transport("transport-test", 1.5, 9.0);
        assert_eq!(media_transport("transport-test"), (1.5, 9.0));
        assert_eq!(media_transport("transport-unknown"), (0.0, 0.0));
    }

    #[test]
    fn frec_seek_jumps_to_the_requested_frame() {
        use fcap_encode::freally_video::{FrecSpec, FrecWriter};
        let dir = std::env::temp_dir().join(format!(
            "fcap-media-frec-seek-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).expect("mkdir");
        let path = dir.join("clip.frec");
        let spec = FrecSpec {
            width: 8,
            height: 8,
            fps_num: 100,
            fps_den: 1,
            pixel_format: fcap_encode::freally_video::PixelFormat::Rgba8,
            audio_tracks: 1,
            sample_rate: 48_000,
        };
        let mut writer = FrecWriter::create(&path, spec).expect("create");
        for t in 0..10u64 {
            writer
                .write_frame(&vec![(t * 20) as u8; 8 * 8 * 4])
                .expect("frame");
        }
        writer.finish().expect("finish");

        let hub_id = format!("media-seek-test-{}", std::process::id());
        let session =
            start_media(&hub_id, path.to_str().expect("utf8"), false, true, false).expect("starts");
        // Ask for frame 7 (0.07 s @ 100 fps). Whether playback is before or
        // past it when the request lands, frame 7 (pixel value 140) must
        // arrive again afterwards.
        media_seek(&hub_id, 0.07);
        let mut saw_target = false;
        for _ in 0..60 {
            match session.frames().recv_timeout(Duration::from_secs(2)) {
                Ok(Some(frame)) => {
                    if frame.data[0] == 140 {
                        saw_target = true;
                        break;
                    }
                }
                Ok(None) => continue,
                Err(err) => panic!("playback ended early: {err}"),
            }
        }
        assert!(saw_target, "the sought frame reaches the canvas");
        session.stop();
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_reversed_gif_plays_its_frames_backwards() {
        let dir = std::env::temp_dir().join(format!(
            "fcap-media-gif-rev-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).expect("mkdir");
        let path = dir.join("anim.gif");
        {
            let file = std::fs::File::create(&path).expect("create");
            let mut encoder = image::codecs::gif::GifEncoder::new(file);
            encoder
                .set_repeat(image::codecs::gif::Repeat::Infinite)
                .expect("repeat");
            // Distinct red values 0, 80, 160 in encode order.
            let frames = (0..3u32).map(|t| {
                let mut buffer = image::RgbaImage::new(8, 8);
                for px in buffer.pixels_mut() {
                    *px = image::Rgba([(t * 80) as u8, 0, 0, 255]);
                }
                image::Frame::from_parts(buffer, 0, 0, image::Delay::from_numer_denom_ms(30, 1))
            });
            encoder.encode_frames(frames).expect("encode");
        }

        let session = start_media(
            "gif-rev-test",
            path.to_str().expect("utf8"),
            true,
            true,
            true,
        )
        .expect("starts");
        let first = loop {
            match session.frames().recv_timeout(Duration::from_secs(2)) {
                Ok(Some(frame)) => break frame,
                Ok(None) => continue,
                Err(err) => panic!("reversed gif ended early: {err}"),
            }
        };
        assert_eq!(
            first.data[0], 160,
            "reverse playback starts on the LAST encoded frame"
        );
        let (_, duration) = media_transport("gif-rev-test");
        assert!(
            (duration - 0.09).abs() < 0.001,
            "3 × 30 ms = 90 ms, got {duration}"
        );
        session.stop();
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn an_animated_gif_loops_through_the_owned_decoder() {
        let dir = std::env::temp_dir().join(format!(
            "fcap-media-gif-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).expect("mkdir");
        let path = dir.join("anim.gif");
        {
            let file = std::fs::File::create(&path).expect("create");
            let mut encoder = image::codecs::gif::GifEncoder::new(file);
            encoder
                .set_repeat(image::codecs::gif::Repeat::Infinite)
                .expect("repeat");
            let frames = (0..2u32).map(|t| {
                let mut buffer = image::RgbaImage::new(8, 8);
                for px in buffer.pixels_mut() {
                    *px = image::Rgba([(t * 200) as u8, 0, 0, 255]);
                }
                image::Frame::from_parts(buffer, 0, 0, image::Delay::from_numer_denom_ms(30, 1))
            });
            encoder.encode_frames(frames).expect("encode");
        }

        let session = start_media("gif-test", path.to_str().expect("utf8"), true, true, false)
            .expect("starts with nothing fetched");
        // Five frames out of a two-frame file proves the loop restarts.
        let mut frames = 0;
        while frames < 5 {
            match session.frames().recv_timeout(Duration::from_secs(2)) {
                Ok(Some(frame)) => {
                    assert_eq!((frame.width, frame.height), (8, 8));
                    frames += 1;
                }
                Ok(None) => continue,
                Err(err) => panic!("gif loop ended early: {err}"),
            }
        }
        session.stop();
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
        let result = start_media(
            "test-wire",
            path.to_str().expect("utf8"),
            false,
            true,
            false,
        );
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
