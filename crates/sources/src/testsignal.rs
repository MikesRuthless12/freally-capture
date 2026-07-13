//! CAP-M21 — **Test signal sources**: SMPTE-style color bars, a calibration
//! grid/crosshatch, a motion sweep, the 1 kHz lineup tone, and the combined
//! A/V sync flash+beep pattern — so scenes, encoders, projectors, and stream
//! targets can be verified with no camera plugged in.
//!
//! Honest signal notes:
//! - The bars/grid are **static** frames (rendered once, like Color/Text).
//! - The sweep and flash+beep are session threads publishing on the same
//!   latest-wins channel every capture uses, paced at 60 fps.
//! - Flash+beep generates video *and* audio from **one clock** (the thread's
//!   start instant): the beep is sample-accurate at the cycle boundary, the
//!   flash quantizes to the 60 fps frame grid (≤ one frame — within the
//!   charter's ±1-frame sync budget). CAP-M20's workbench measures against
//!   the exported cycle constants below.
//! - Audio lands in [`fcap_audio::media_hub`] under the source's id, exactly
//!   like the Media source; the mixer strip owns gain/mute from there.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use fcap_capture::{frame_channel, CaptureError, CaptureSession, Frame};

use crate::static_source::{check_dimension, rgba_frame, StaticSourceError};

// ---------------------------------------------------------------------------
// Cycle constants — CAP-M20 measures against these; keep them in lockstep
// with the generators below.
// ---------------------------------------------------------------------------

/// Flash+beep cycle length.
pub const FLASH_BEEP_PERIOD_MS: u32 = 2_000;
/// How long the flash shows and the beep sounds each cycle.
pub const FLASH_BEEP_PULSE_MS: u32 = 100;
/// The tone (and beep) frequency.
pub const TONE_HZ: f64 = 1_000.0;
/// The lineup tone's amplitude: −20 dBFS, the broadcast alignment level.
pub const TONE_AMPLITUDE: f32 = 0.1;
/// The sync beep's amplitude: −6 dBFS — loud enough to survive a room mic.
pub const BEEP_AMPLITUDE: f32 = 0.5;

/// One full sweep (left edge to right edge) of the motion bar.
const SWEEP_PERIOD_MS: u32 = 2_000;
/// The session generators' frame cadence.
const FPS: u32 = 60;

const OPAQUE_BLACK: [u8; 4] = [0, 0, 0, 255];
const WHITE: [u8; 4] = [255, 255, 255, 255];

// ---------------------------------------------------------------------------
// SMPTE-style color bars (static)
// ---------------------------------------------------------------------------

/// 75%-intensity bars, left to right (gray, yellow, cyan, green, magenta,
/// red, blue) — the canonical 8-bit RGB rendering of SMPTE EG 1-1990.
const BARS_75: [[u8; 4]; 7] = [
    [191, 191, 191, 255],
    [191, 191, 0, 255],
    [0, 191, 191, 255],
    [0, 191, 0, 255],
    [191, 0, 191, 255],
    [191, 0, 0, 255],
    [0, 0, 191, 255],
];

/// The castellation strip under the bars (reverse-blue order).
const CASTELLATION: [[u8; 4]; 7] = [
    [0, 0, 191, 255],
    [19, 19, 19, 255],
    [191, 0, 191, 255],
    [19, 19, 19, 255],
    [0, 191, 191, 255],
    [19, 19, 19, 255],
    [191, 191, 191, 255],
];

/// 7.5%-setup black — the bottom band's black and the PLUGE's middle step.
const SETUP_BLACK: [u8; 4] = [19, 19, 19, 255];
/// −I, 100% white, +Q — the bottom band's first three segments.
const NEG_I: [u8; 4] = [0, 33, 76, 255];
const FULL_WHITE: [u8; 4] = [255, 255, 255, 255];
const POS_Q: [u8; 4] = [50, 0, 106, 255];
/// The PLUGE steps: below / at / above setup black.
const PLUGE: [[u8; 4]; 3] = [[9, 9, 9, 255], SETUP_BLACK, [29, 29, 29, 255]];

/// Render SMPTE-style color bars: 67% bars, 8% castellation, 25% PLUGE band.
pub fn smpte_bars_frame(width: u32, height: u32) -> Result<Frame, StaticSourceError> {
    check_dimension("test bars width", width)?;
    check_dimension("test bars height", height)?;
    let w = width as usize;
    let h = height as usize;

    // Build each band's row once, then stamp rows.
    let bar_at = |x: usize, colors: &[[u8; 4]; 7]| colors[(x * 7 / w).min(6)];
    let mut top_row = Vec::with_capacity(w * 4);
    let mut cast_row = Vec::with_capacity(w * 4);
    let mut bottom_row = Vec::with_capacity(w * 4);
    for x in 0..w {
        top_row.extend_from_slice(&bar_at(x, &BARS_75));
        cast_row.extend_from_slice(&bar_at(x, &CASTELLATION));
        // Bottom band: −I / white / +Q / black at 5/28 W each, then the three
        // PLUGE steps at W/21 each, then setup black to the right edge.
        let x28 = x * 28 / w; // 28ths of the width
        let color = match x28 {
            0..=4 => NEG_I,
            5..=9 => FULL_WHITE,
            10..=14 => POS_Q,
            15..=19 => SETUP_BLACK,
            _ => {
                let x21 = x * 21 / w; // 21sts — the PLUGE strips are W/21 wide
                match x21 {
                    15 => PLUGE[0],
                    16 => PLUGE[1],
                    17 => PLUGE[2],
                    _ => SETUP_BLACK,
                }
            }
        };
        bottom_row.extend_from_slice(&color);
    }

    let top_h = h * 67 / 100;
    let cast_h = h * 8 / 100;
    let mut data = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        if y < top_h {
            data.extend_from_slice(&top_row);
        } else if y < top_h + cast_h {
            data.extend_from_slice(&cast_row);
        } else {
            data.extend_from_slice(&bottom_row);
        }
    }
    Ok(rgba_frame(width, height, data))
}

// ---------------------------------------------------------------------------
// Calibration grid / crosshatch (static)
// ---------------------------------------------------------------------------

/// Render a calibration crosshatch: square cells from the center out, a
/// heavier center cross, and a border — black background, white lines.
pub fn grid_frame(width: u32, height: u32) -> Result<Frame, StaticSourceError> {
    check_dimension("test grid width", width)?;
    check_dimension("test grid height", height)?;
    let w = width as usize;
    let h = height as usize;
    let cell = (h / 9).max(8);
    let cx = w / 2;
    let cy = h / 2;

    // A position is on a line when it sits within the line's thickness of a
    // center-anchored multiple of the cell size, or on the border.
    let on_line = |pos: usize, center: usize, extent: usize, thick: usize| -> bool {
        if pos < 2 || pos >= extent - 2 {
            return true; // border
        }
        let offset = pos.abs_diff(center) % cell;
        offset < thick || cell - offset < thick
    };
    let line_row: Vec<u8> = WHITE.repeat(w);
    let mut plain_row = Vec::with_capacity(w * 4);
    for x in 0..w {
        let thick = if x.abs_diff(cx) < cell { 2 } else { 1 };
        if on_line(x, cx, w, thick) {
            plain_row.extend_from_slice(&WHITE);
        } else {
            plain_row.extend_from_slice(&OPAQUE_BLACK);
        }
    }

    let mut data = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        let thick = if y.abs_diff(cy) < cell { 2 } else { 1 };
        if on_line(y, cy, h, thick) {
            data.extend_from_slice(&line_row);
        } else {
            data.extend_from_slice(&plain_row);
        }
    }
    Ok(rgba_frame(width, height, data))
}

// ---------------------------------------------------------------------------
// Motion sweep (session)
// ---------------------------------------------------------------------------

/// The sweep background: dark gray with tick columns every 10% of the width.
fn sweep_template(w: usize, h: usize) -> Vec<u8> {
    const BG: [u8; 4] = [16, 16, 16, 255];
    const TICK: [u8; 4] = [64, 64, 64, 255];
    let mut row = Vec::with_capacity(w * 4);
    for x in 0..w {
        let tick = (1..10).any(|k| x.abs_diff(k * w / 10) < 1);
        row.extend_from_slice(if tick { &TICK } else { &BG });
    }
    let mut data = Vec::with_capacity(w * h * 4);
    for _ in 0..h {
        data.extend_from_slice(&row);
    }
    data
}

/// One sweep frame: the white bar's left edge at `phase` ∈ [0, 1) of its run.
fn sweep_frame(template: &[u8], width: u32, height: u32, phase: f64) -> Frame {
    let w = width as usize;
    let h = height as usize;
    let bar_w = if w < 2 { w } else { (w / 25).max(2) };
    let bar_x = (phase.clamp(0.0, 1.0) * (w - bar_w) as f64) as usize;
    let mut data = template.to_vec();
    let bar: Vec<u8> = WHITE.repeat(bar_w);
    for y in 0..h {
        let start = (y * w + bar_x) * 4;
        data[start..start + bar_w * 4].copy_from_slice(&bar);
    }
    rgba_frame(width, height, data)
}

/// Start the motion-sweep session: a white bar crossing the frame at a
/// constant speed, once per two seconds, over faint tick marks.
pub fn start_sweep(width: u32, height: u32) -> Result<CaptureSession, CaptureError> {
    check_dimension("motion sweep width", width)
        .and_then(|()| check_dimension("motion sweep height", height))
        .map_err(|err| CaptureError::Backend(err.to_string()))?;
    let (sender, receiver) = frame_channel();
    let stop = Arc::new(AtomicBool::new(false));
    let thread_stop = Arc::clone(&stop);
    let join = std::thread::Builder::new()
        .name("fcap-testsweep".into())
        .spawn(move || {
            let template = sweep_template(width as usize, height as usize);
            let period = Duration::from_micros(1_000_000 / u64::from(FPS));
            let started = Instant::now();
            let mut next = started;
            loop {
                if thread_stop.load(Ordering::Relaxed) || !sender.is_open() {
                    return;
                }
                let phase = (started.elapsed().as_millis() % u128::from(SWEEP_PERIOD_MS)) as f64
                    / f64::from(SWEEP_PERIOD_MS);
                sender.send(sweep_frame(&template, width, height, phase));
                next += period;
                let now = Instant::now();
                if next > now {
                    std::thread::sleep(next - now);
                } else {
                    next = now; // fell behind — never burst to catch up
                }
            }
        })
        .map_err(|err| CaptureError::Backend(err.to_string()))?;
    Ok(CaptureSession::from_parts(receiver, stop, join))
}

// ---------------------------------------------------------------------------
// A/V sync flash + beep (session: video + audio from one clock)
// ---------------------------------------------------------------------------

/// Whether the flash shows at `cycle_ms` into a cycle.
fn flash_active(cycle_ms: u32) -> bool {
    cycle_ms < FLASH_BEEP_PULSE_MS
}

/// Whether the beep sounds at absolute sample-frame `n` (48 kHz grid).
fn beep_active(n: u64) -> bool {
    let per_cycle = u64::from(FLASH_BEEP_PERIOD_MS) * u64::from(fcap_audio::SAMPLE_RATE) / 1_000;
    let pulse = u64::from(FLASH_BEEP_PULSE_MS) * u64::from(fcap_audio::SAMPLE_RATE) / 1_000;
    n % per_cycle < pulse
}

/// One flash+beep video frame at `cycle_ms` into the cycle: full white during
/// the flash, else black with a gray metronome column that reaches the right
/// edge exactly as the next flash fires.
fn flash_frame(width: u32, height: u32, cycle_ms: u32) -> Frame {
    let w = width as usize;
    let h = height as usize;
    if flash_active(cycle_ms) {
        return rgba_frame(width, height, WHITE.repeat(w * h));
    }
    const MARKER: [u8; 4] = [96, 96, 96, 255];
    let bar_w = if w < 2 { w } else { (w / 100).max(2) };
    let progress = f64::from(cycle_ms) / f64::from(FLASH_BEEP_PERIOD_MS);
    let bar_x = (progress.clamp(0.0, 1.0) * (w - bar_w) as f64) as usize;
    let mut data = OPAQUE_BLACK.repeat(w * h);
    let bar: Vec<u8> = MARKER.repeat(bar_w);
    for y in 0..h {
        let start = (y * w + bar_x) * 4;
        data[start..start + bar_w * 4].copy_from_slice(&bar);
    }
    rgba_frame(width, height, data)
}

/// The sine value at absolute sample-frame `n`, for `amplitude`.
fn sine_sample(n: u64, amplitude: f32) -> f32 {
    let phase =
        (n % u64::from(fcap_audio::SAMPLE_RATE)) as f64 / f64::from(fcap_audio::SAMPLE_RATE);
    (amplitude as f64 * (std::f64::consts::TAU * TONE_HZ * phase).sin()) as f32
}

/// Push interleaved-stereo audio into `ring` up to the clock's current
/// sample-frame target; `sample_for` decides each frame's value.
fn push_audio_to(
    ring: &fcap_audio::capture::CaptureRing,
    pushed: &mut u64,
    target: u64,
    sample_for: impl Fn(u64) -> f32,
) {
    const BLOCK: u64 = 480; // 10 ms — the engine's own block size
    while *pushed < target {
        let frames = (target - *pushed).min(BLOCK) as usize;
        let mut block = Vec::with_capacity(frames * 2);
        for i in 0..frames {
            let sample = sample_for(*pushed + i as u64);
            block.push(sample);
            block.push(sample);
        }
        ring.push(&block);
        *pushed += frames as u64;
    }
}

/// Start the combined A/V sync pattern: a full-frame white flash and a 1 kHz
/// beep, both `FLASH_BEEP_PULSE_MS` long, every `FLASH_BEEP_PERIOD_MS`, from
/// one clock. `hub_id` keys the mixer-side audio ring (the source id).
pub fn start_flash_beep(
    hub_id: &str,
    width: u32,
    height: u32,
) -> Result<CaptureSession, CaptureError> {
    check_dimension("sync pattern width", width)
        .and_then(|()| check_dimension("sync pattern height", height))
        .map_err(|err| CaptureError::Backend(err.to_string()))?;
    let ring = fcap_audio::media_hub::ring(hub_id);
    ring.clear();
    let (sender, receiver) = frame_channel();
    let stop = Arc::new(AtomicBool::new(false));
    let thread_stop = Arc::clone(&stop);
    let join = std::thread::Builder::new()
        .name("fcap-flashbeep".into())
        .spawn(move || {
            let period = Duration::from_micros(1_000_000 / u64::from(FPS));
            let started = Instant::now();
            let mut next = started;
            let mut pushed: u64 = 0;
            loop {
                if thread_stop.load(Ordering::Relaxed) || !sender.is_open() {
                    return;
                }
                let elapsed = started.elapsed();
                let cycle_ms = (elapsed.as_millis() % u128::from(FLASH_BEEP_PERIOD_MS)) as u32;
                sender.send(flash_frame(width, height, cycle_ms));
                let target = (elapsed.as_secs_f64() * f64::from(fcap_audio::SAMPLE_RATE)) as u64;
                push_audio_to(&ring, &mut pushed, target, |n| {
                    if beep_active(n) {
                        sine_sample(n, BEEP_AMPLITUDE)
                    } else {
                        0.0
                    }
                });
                next += period;
                let now = Instant::now();
                if next > now {
                    std::thread::sleep(next - now);
                } else {
                    next = now;
                }
            }
        })
        .map_err(|err| CaptureError::Backend(err.to_string()))?;
    Ok(CaptureSession::from_parts(receiver, stop, join))
}

// ---------------------------------------------------------------------------
// 1 kHz lineup tone (audio-only)
// ---------------------------------------------------------------------------

/// A running tone generator. Dropping it stops the thread.
pub struct ToneTask {
    stop: Arc<AtomicBool>,
    join: Option<JoinHandle<()>>,
}

impl Drop for ToneTask {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(join) = self.join.take() {
            let _ = join.join();
        }
    }
}

/// Start the continuous 1 kHz lineup tone at −20 dBFS, feeding the mixer-side
/// audio ring keyed by `hub_id` (the source id) in real time.
pub fn start_tone(hub_id: &str) -> ToneTask {
    let ring = fcap_audio::media_hub::ring(hub_id);
    ring.clear();
    let stop = Arc::new(AtomicBool::new(false));
    let thread_stop = Arc::clone(&stop);
    let join = std::thread::Builder::new()
        .name("fcap-testtone".into())
        .spawn(move || {
            let started = Instant::now();
            let mut pushed: u64 = 0;
            while !thread_stop.load(Ordering::Relaxed) {
                let target =
                    (started.elapsed().as_secs_f64() * f64::from(fcap_audio::SAMPLE_RATE)) as u64;
                push_audio_to(&ring, &mut pushed, target, |n| {
                    sine_sample(n, TONE_AMPLITUDE)
                });
                std::thread::sleep(Duration::from_millis(5));
            }
        })
        .ok();
    ToneTask { stop, join }
}

// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use fcap_capture::PixelFormat;

    fn px(frame: &Frame, x: usize, y: usize) -> [u8; 4] {
        let at = (y * frame.width as usize + x) * 4;
        frame.data[at..at + 4].try_into().unwrap()
    }

    #[test]
    fn bars_land_the_canonical_bands() {
        let frame = smpte_bars_frame(700, 400).expect("render");
        assert_eq!(frame.format, PixelFormat::Rgba8);
        // Top band: first bar gray, last bar blue.
        assert_eq!(px(&frame, 10, 10), [191, 191, 191, 255]);
        assert_eq!(px(&frame, 690, 10), [0, 0, 191, 255]);
        // Castellation (67%..75% of height): starts blue under the gray bar.
        let cast_y = 400 * 67 / 100 + 5;
        assert_eq!(px(&frame, 10, cast_y), [0, 0, 191, 255]);
        // Bottom band: −I first, then 100% white; PLUGE middle = setup black.
        let bot_y = 400 * 75 / 100 + 5;
        assert_eq!(px(&frame, 10, bot_y), [0, 33, 76, 255]);
        assert_eq!(px(&frame, 700 * 7 / 28 + 5, bot_y), [255, 255, 255, 255]);
        let pluge_mid_x = 700 * 16 / 21 + 5; // the middle PLUGE strip
        assert_eq!(px(&frame, pluge_mid_x, bot_y), [19, 19, 19, 255]);
        // Far bottom-right corner: setup black.
        assert_eq!(px(&frame, 695, 395), [19, 19, 19, 255]);
    }

    #[test]
    fn bars_reject_degenerate_sizes() {
        assert!(smpte_bars_frame(0, 100).is_err());
        assert!(smpte_bars_frame(100, 99_999).is_err());
    }

    #[test]
    fn grid_draws_border_center_and_background() {
        let frame = grid_frame(640, 360).expect("render");
        assert_eq!(px(&frame, 0, 180), WHITE, "border");
        assert_eq!(px(&frame, 320, 180), WHITE, "center cross");
        assert_eq!(px(&frame, 320, 0), WHITE, "top border over center");
        // Mid-cell: black background (half a cell off the center lines).
        let cell = 360 / 9;
        assert_eq!(px(&frame, 320 + cell / 2, 180 + cell / 2), OPAQUE_BLACK);
    }

    #[test]
    fn sweep_bar_tracks_the_phase() {
        let template = sweep_template(200, 20);
        let left = sweep_frame(&template, 200, 20, 0.0);
        assert_eq!(px(&left, 0, 10), WHITE, "phase 0 → bar at the left edge");
        assert_ne!(px(&left, 199, 10), WHITE);
        let right = sweep_frame(&template, 200, 20, 1.0);
        assert_eq!(px(&right, 199, 10), WHITE, "phase 1 → bar at the right");
        let mid = sweep_frame(&template, 200, 20, 0.5);
        assert_eq!(px(&mid, 100, 10), WHITE, "phase 0.5 → bar centered");
    }

    #[test]
    fn flash_and_beep_share_the_cycle() {
        // The flash shows exactly while the beep sounds, on both clocks'
        // units: ms into the cycle vs 48 kHz sample frames.
        for ms in [0u32, 50, 99] {
            assert!(flash_active(ms));
            assert!(beep_active(u64::from(ms) * 48));
        }
        for ms in [100u32, 500, 1_999] {
            assert!(!flash_active(ms));
            assert!(!beep_active(u64::from(ms) * 48));
        }
        // And the cycle wraps identically.
        assert!(flash_active(
            (FLASH_BEEP_PERIOD_MS + 10) % FLASH_BEEP_PERIOD_MS
        ));
        assert!(beep_active(u64::from(FLASH_BEEP_PERIOD_MS + 10) * 48));
    }

    #[test]
    fn flash_frame_is_white_then_black_with_marker() {
        let flash = flash_frame(64, 8, 0);
        assert!(flash.data.chunks(4).all(|p| p == WHITE));
        let dark = flash_frame(64, 8, 1_000);
        assert_eq!(px(&dark, 0, 4), OPAQUE_BLACK);
        // The metronome column sits mid-frame at half cycle.
        let bar_x = 31; // 0.5 * (64 - 2), bar 2 px wide at this size
        assert_eq!(px(&dark, bar_x, 4), [96, 96, 96, 255]);
    }

    #[test]
    fn tone_is_one_kilohertz_at_lineup_level() {
        // One second of samples: a 1 kHz sine crosses zero 2 000 times and
        // peaks at the −20 dBFS amplitude.
        let rate = u64::from(fcap_audio::SAMPLE_RATE);
        let samples: Vec<f32> = (0..rate).map(|n| sine_sample(n, TONE_AMPLITUDE)).collect();
        let crossings = samples
            .windows(2)
            .filter(|w| (w[0] >= 0.0) != (w[1] >= 0.0))
            .count();
        assert!((1_990..=2_010).contains(&crossings), "got {crossings}");
        let peak = samples.iter().fold(0.0f32, |a, &s| a.max(s.abs()));
        assert!((peak - TONE_AMPLITUDE).abs() < 0.001, "peak {peak}");
    }

    #[test]
    fn audio_push_reaches_the_target_exactly() {
        let ring = fcap_audio::media_hub::ring("testsignal-push-test");
        ring.clear();
        let mut pushed = 0u64;
        push_audio_to(&ring, &mut pushed, 1_234, |n| sine_sample(n, 1.0));
        assert_eq!(pushed, 1_234);
        assert_eq!(ring.len(), 1_234 * 2, "interleaved stereo");
    }
}
