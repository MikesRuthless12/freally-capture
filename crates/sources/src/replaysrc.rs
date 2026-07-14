//! CAP-N10 — the **instant-replay playback source**: rolls the replay
//! buffer's last moments INTO the program as a source, with adjustable
//! speed (100 / 50 / 25% — frame-interpolation-free, just retimed), the
//! ordinary scrub/pause transport, and auto-return when the clip ends.
//!
//! How a roll works: the app side snapshots the armed replay ring into a
//! temporary clip file (stream copy — fast), then hands the path to this
//! session through the roll slot. The session decodes the clip **unpaced**
//! and paces frames itself at `fps × speed` — retiming without touching a
//! single pixel. At the end it clears to transparency, so whatever sits
//! under it in the scene shows again ("auto-return to live").
//!
//! Honest notes:
//! - Slow-motion is **silent** (retimed audio would pitch or smear; we
//!   don't fake it). Full-speed rolls play the clip's audio through the
//!   source's mixer strip.
//! - The source idles transparent until the first roll; the replay buffer
//!   must be armed for a roll to have anything to snapshot.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use fcap_capture::{frame_channel, CaptureError, CaptureSession, Frame, FrameSender, PixelFormat};
use fcap_encode::decode;

use crate::media::{
    clamp_seek, f32_samples_into, pause_flag, publish_transport, read_available, read_exact_or_end,
    seek_pending, take_seek,
};

/// Idle poll cadence (waiting for a roll).
const IDLE_POLL: Duration = Duration::from_millis(100);

/// Playback speed — retimed, never interpolated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Speed {
    Full,
    Half,
    Quarter,
}

impl Speed {
    pub fn factor(self) -> f32 {
        match self {
            Speed::Full => 1.0,
            Speed::Half => 0.5,
            Speed::Quarter => 0.25,
        }
    }
}

/// One session's roll slot: the newest requested clip path (latest wins).
type RollSlot = Mutex<Option<std::path::PathBuf>>;

static REGISTRY: crate::registry::WeakRegistry<RollSlot> = crate::registry::WeakRegistry::new();

/// The ids of every live replay source (the roll hotkey's fan-out).
pub fn live_ids() -> Vec<String> {
    REGISTRY.live_ids()
}

/// Hand a fresh clip to one live replay source. `false` = not running.
/// The clip FILE is owned by this call either way: a refused clip and a
/// latest-wins-replaced clip would never be played, so they are deleted
/// here rather than stranded in the temp folder.
pub fn roll(id: &str, clip: std::path::PathBuf) -> bool {
    match REGISTRY.get(id) {
        Some(slot) => {
            let replaced = slot
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .replace(clip);
            if let Some(old) = replaced {
                let _ = std::fs::remove_file(old);
            }
            true
        }
        None => {
            let _ = std::fs::remove_file(&clip);
            false
        }
    }
}

/// Everything the session needs.
#[derive(Debug, Clone)]
pub struct ReplaySourceConfig {
    pub speed: Speed,
    pub hw_decode: bool,
}

/// One transparent frame at the clip's (or a nominal) geometry.
fn transparent(width: u32, height: u32) -> Frame {
    Frame {
        width,
        height,
        stride: width * 4,
        format: PixelFormat::Rgba8,
        data: vec![0u8; (width * height * 4) as usize],
        captured_at: Instant::now(),
    }
}

/// Start the replay-playback session: idle-transparent until a roll lands.
pub fn start_replay_source(
    hub_id: &str,
    config: ReplaySourceConfig,
) -> Result<CaptureSession, CaptureError> {
    let slot: Arc<RollSlot> = Arc::new(Mutex::new(None));
    REGISTRY.insert(hub_id, &slot);
    let (sender, receiver) = frame_channel();
    let stop = Arc::new(AtomicBool::new(false));
    let thread_stop = Arc::clone(&stop);
    let hub_id = hub_id.to_string();
    let join = std::thread::Builder::new()
        .name("fcap-replay-source".into())
        .spawn(move || run_replay_source(config, slot, hub_id, sender, thread_stop))
        .map_err(|err| CaptureError::Backend(err.to_string()))?;
    Ok(CaptureSession::from_parts(receiver, stop, join))
}

fn take_slot(slot: &RollSlot) -> Option<std::path::PathBuf> {
    slot.lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .take()
}

fn run_replay_source(
    config: ReplaySourceConfig,
    slot: Arc<RollSlot>,
    hub_id: String,
    sender: FrameSender,
    stop: Arc<AtomicBool>,
) {
    // Idle geometry before the first clip teaches us better.
    let mut last_dims = (640u32, 360u32);
    sender.send(transparent(last_dims.0, last_dims.1));
    publish_transport(&hub_id, 0.0, 0.0);
    let mut previous_clip: Option<std::path::PathBuf> = None;
    loop {
        if stop.load(Ordering::Relaxed) || !sender.is_open() {
            sender.close(None);
            break;
        }
        let Some(clip) = take_slot(&slot) else {
            std::thread::sleep(IDLE_POLL);
            continue;
        };
        // A replaced clip's file is ours to clean up.
        if let Some(old) = previous_clip.take() {
            if old != clip {
                let _ = std::fs::remove_file(&old);
            }
        }
        match play_clip(&config, &slot, &clip, &hub_id, &sender, &stop) {
            Ok(dims) => last_dims = dims,
            Err(err) => {
                sender.close(Some(CaptureError::Backend(err)));
                break;
            }
        }
        previous_clip = Some(clip);
        if stop.load(Ordering::Relaxed) || !sender.is_open() {
            sender.close(None);
            break;
        }
        // Auto-return: the clip is over — clear to transparency so the
        // scene under this source shows again.
        sender.send(transparent(last_dims.0, last_dims.1));
        publish_transport(&hub_id, 0.0, 0.0);
    }
    if let Some(old) = previous_clip {
        let _ = std::fs::remove_file(old);
    }
    // A roll that landed as the session ended will never play — drain it.
    if let Some(pending) = take_slot(&slot) {
        let _ = std::fs::remove_file(pending);
    }
}

/// Play one clip start-to-finish (or until stop / a fresh roll). Seeks
/// respawn at the target like the Media scrubber. Returns the clip's dims.
fn play_clip(
    config: &ReplaySourceConfig,
    slot: &RollSlot,
    clip: &std::path::Path,
    hub_id: &str,
    sender: &FrameSender,
    stop: &Arc<AtomicBool>,
) -> Result<(u32, u32), String> {
    let Some(ffmpeg) = fcap_encode::ffmpeg::installed() else {
        return Err("replay playback needs the ffmpeg component".into());
    };
    let info = decode::probe_media(&ffmpeg, clip)?;
    let fps = info.fps.unwrap_or(30.0).max(1.0);
    let duration = info.duration_secs.unwrap_or(0.0);
    let speed = config.speed.factor();
    let pause = pause_flag(hub_id);
    let mut seek_base = 0.0f32;

    'stretch: loop {
        let mut video =
            decode::spawn_video_decoder_unpaced(&ffmpeg, clip, config.hw_decode, seek_base)?;
        let Some(mut stdout) = video.stdout.take() else {
            return Err("the replay decoder gave no pipe".into());
        };
        // Full speed plays the clip's audio; slow-mo is silent (honest).
        let mut audio_child = if config.speed == Speed::Full && info.has_audio {
            decode::spawn_audio_decoder_unpaced(&ffmpeg, clip, seek_base).ok()
        } else {
            None
        };
        let audio_stdout = audio_child.as_mut().and_then(|child| child.stdout.take());
        let audio_stop = Arc::new(AtomicBool::new(false));
        let audio_thread = audio_stdout.and_then(|mut pipe| {
            let ring = fcap_audio::media_hub::ring(hub_id);
            ring.clear();
            let flag = Arc::clone(&audio_stop);
            let outer_stop = Arc::clone(stop);
            let audio_pause = Arc::clone(&pause);
            std::thread::Builder::new()
                .name("fcap-replay-audio".into())
                .spawn(move || {
                    // Self-paced: one 10 ms block per 10 ms — the pipe is
                    // unpaced, so the pacing lives here.
                    let mut bytes = [0u8; 3840];
                    let mut samples = Vec::with_capacity(960);
                    let mut next = Instant::now();
                    loop {
                        if flag.load(Ordering::Relaxed) || outer_stop.load(Ordering::Relaxed) {
                            return;
                        }
                        // Pause: stop draining the pipe (it backpressures and
                        // holds) and push no audio — a paused replay is silent
                        // + frozen, and resume picks up exactly where the
                        // video did (media.rs's rule). Re-anchor the pacing so
                        // resume doesn't burst-drain the backlog.
                        if audio_pause.load(Ordering::Relaxed) {
                            std::thread::sleep(Duration::from_millis(30));
                            next = Instant::now();
                            continue;
                        }
                        let (filled, _done) = read_available(&mut pipe, &mut bytes);
                        f32_samples_into(&bytes[..filled], &mut samples);
                        if samples.is_empty() {
                            return; // end of the clip's audio
                        }
                        ring.push(&samples);
                        next += Duration::from_millis(10);
                        let now = Instant::now();
                        if next > now {
                            std::thread::sleep(next - now);
                        } else {
                            next = now;
                        }
                    }
                })
                .ok()
        });

        let frame_bytes = info.width as usize * info.height as usize * 4;
        let mut data = vec![0u8; frame_bytes];
        let mut frames: u64 = 0;
        // The retimed frame clock: fps × speed on OUR pacing.
        let period = Duration::from_secs_f32(1.0 / (fps * speed));
        let mut next = Instant::now();
        let mut force_frame = true;
        let end = loop {
            if stop.load(Ordering::Relaxed) || !sender.is_open() {
                break None;
            }
            // A fresh roll interrupts the current playback.
            if slot
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .is_some()
            {
                break None;
            }
            if let Some(target) = take_seek(hub_id) {
                break Some(target);
            }
            while !force_frame
                && pause.load(Ordering::Relaxed)
                && !stop.load(Ordering::Relaxed)
                && sender.is_open()
                && !seek_pending(hub_id)
            {
                std::thread::sleep(Duration::from_millis(30));
            }
            if pause.load(Ordering::Relaxed) && !force_frame {
                continue;
            }
            if !read_exact_or_end(&mut stdout, &mut data) {
                break None; // clip over
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
            publish_transport(hub_id, seek_base + frames as f32 / fps, duration);
            next += period;
            let now = Instant::now();
            if next > now {
                std::thread::sleep(next - now);
            } else {
                next = now; // fell behind — never burst
            }
        };

        audio_stop.store(true, Ordering::Relaxed);
        let _ = video.kill();
        let _ = video.wait();
        if let Some(mut child) = audio_child {
            let _ = child.kill();
            let _ = child.wait();
        }
        if let Some(handle) = audio_thread {
            let _ = handle.join();
        }
        match end {
            Some(target) => {
                seek_base = clamp_seek(target, duration);
                publish_transport(hub_id, seek_base, duration);
                continue 'stretch;
            }
            None => return Ok((info.width, info.height)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn speeds_are_the_three_honest_factors() {
        assert_eq!(Speed::Full.factor(), 1.0);
        assert_eq!(Speed::Half.factor(), 0.5);
        assert_eq!(Speed::Quarter.factor(), 0.25);
    }

    #[test]
    fn a_roll_reaches_a_live_slot_and_dies_with_it() {
        let slot: Arc<RollSlot> = Arc::new(Mutex::new(None));
        REGISTRY.insert("replay-test", &slot);
        assert!(roll("replay-test", "C:/tmp/clip.mkv".into()));
        assert_eq!(
            take_slot(&slot),
            Some(std::path::PathBuf::from("C:/tmp/clip.mkv"))
        );
        assert!(live_ids().contains(&"replay-test".to_string()));
        drop(slot);
        assert!(!roll("replay-test", "C:/tmp/clip.mkv".into()), "dead slot");
        assert!(!live_ids().contains(&"replay-test".to_string()));
    }

    #[test]
    fn the_latest_roll_wins() {
        let slot: Arc<RollSlot> = Arc::new(Mutex::new(None));
        REGISTRY.insert("replay-test-latest", &slot);
        assert!(roll("replay-test-latest", "a.mkv".into()));
        assert!(roll("replay-test-latest", "b.mkv".into()));
        assert_eq!(take_slot(&slot), Some(std::path::PathBuf::from("b.mkv")));
        assert_eq!(take_slot(&slot), None, "consumed");
    }

    #[test]
    fn transparent_frames_are_fully_clear() {
        let frame = transparent(4, 2);
        assert_eq!(frame.data.len(), 4 * 2 * 4);
        assert!(frame.data.iter().all(|byte| *byte == 0));
    }
}
