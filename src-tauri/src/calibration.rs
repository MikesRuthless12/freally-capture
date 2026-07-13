//! CAP-M20 — A/V sync calibration: the pure signal processing.
//!
//! The workbench plays CAP-M21's flash+beep pattern through the user's
//! display + speakers; their camera and mic capture it back. Two probes
//! record envelopes — mean luma per drained camera frame (render loop) and
//! peak per 10 ms engine block (audio engine tap) — and this module turns
//! them into one number: how much later the video arrives than the audio.
//!
//! Everything here is pure math over `(ms_since_arm, value)` samples: onset
//! detection (adaptive threshold, hysteresis-free rising edge with a
//! refractory window, sub-sample linear interpolation at the crossing),
//! periodicity validation against the pattern's cycle (so room noise or a
//! light switch can't masquerade as the pattern), nearest pairing, and a
//! median with a jitter bound. No ML (charter); no clocks (testable).

use std::sync::Mutex;
use std::time::Instant;

use fcap_scene::SourceId;
use fcap_sources::testsignal::FLASH_BEEP_PERIOD_MS;
use serde::Serialize;

/// One probe sample: milliseconds since arming, and the observed value —
/// mean luma (0..1) for video, block peak (0..1 linear) for audio.
pub type Sample = (f64, f32);

/// The flash must swing the mean luma by at least this much (0..1). A
/// full-frame white flash against a normal room shot swings far more.
const MIN_VIDEO_SWING: f32 = 0.15;
/// The beep must lift the mic peak this far above its quiet floor —
/// ~−34 dBFS, well above typical room floors and far below the −6 dBFS beep.
const MIN_AUDIO_SWING: f32 = 0.02;
/// Ignore re-crossings this soon after an onset (the pulse is 100 ms and
/// the cycle 2 000 ms — nothing legitimate rises again this fast).
const REFRACTORY_MS: f64 = 700.0;
/// Onset spacing must sit within this of a whole number of cycles.
const PERIOD_TOLERANCE_MS: f64 = 250.0;
/// A flash pairs with the nearest beep within this window (< half a cycle,
/// so pairing is unambiguous).
const MAX_PAIR_WINDOW_MS: f64 = 900.0;
/// Fewer paired cycles than this is not a measurement.
const MIN_CYCLES: usize = 3;
/// If the per-cycle deltas spread wider than this, refuse to apply a number.
const MAX_JITTER_MS: f64 = 60.0;

/// A completed measurement.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Measurement {
    /// Positive = the video arrives this much later than the audio (the
    /// usual camera case) — delaying the audio strip by this much aligns
    /// them. Negative = the audio is already the late one.
    pub offset_ms: f64,
    /// How many flash/beep pairs agreed on it.
    pub cycles: usize,
    /// Worst per-cycle deviation from the median, ms.
    pub jitter_ms: f64,
}

/// Why a measurement could not be made. Every variant maps to honest,
/// actionable guidance in the workbench UI.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CalibrationError {
    /// The camera never saw the flash (swing too small).
    NoFlash,
    /// The mic never heard the beep (swing too small).
    NoBeep,
    /// Saw/heard something, but not enough clean cycles.
    TooFewCycles { paired: usize },
    /// Onsets don't repeat at the pattern's cycle — that's not our signal.
    NotThePattern,
    /// The per-cycle deltas disagree too much to trust.
    Unstable { jitter_ms: f64 },
}

/// Rising-edge onsets of a pulse train: adaptive threshold halfway between
/// the quiet floor (20th percentile) and the observed maximum, sub-sample
/// interpolated at the crossing, with a refractory window. Returns `None`
/// when the swing is below `min_swing` (no signal to detect).
fn onsets(samples: &[Sample], min_swing: f32) -> Option<Vec<f64>> {
    if samples.len() < 4 {
        return None;
    }
    let mut values: Vec<f32> = samples.iter().map(|&(_, v)| v).collect();
    values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let floor = values[values.len() / 5];
    let top = values[values.len() - 1];
    if top - floor < min_swing {
        return None;
    }
    let threshold = floor + (top - floor) * 0.5;

    let mut found = Vec::new();
    let mut above = samples[0].1 >= threshold;
    let mut last_onset = f64::NEG_INFINITY;
    for pair in samples.windows(2) {
        let (t_prev, v_prev) = pair[0];
        let (t, v) = pair[1];
        let rises = !above && v >= threshold;
        above = v >= threshold;
        if !rises || t - last_onset < REFRACTORY_MS {
            continue;
        }
        // Sub-sample: linearly interpolate where the value crossed.
        let span = v - v_prev;
        let frac = if span > f32::EPSILON {
            f64::from((threshold - v_prev) / span).clamp(0.0, 1.0)
        } else {
            1.0
        };
        let at = t_prev + (t - t_prev) * frac;
        found.push(at);
        last_onset = at;
    }
    Some(found)
}

/// Whether consecutive onsets sit a whole number of cycles apart (missed
/// cycles are fine; off-period events are not).
fn periodic(onsets: &[f64]) -> bool {
    let period = f64::from(FLASH_BEEP_PERIOD_MS);
    onsets.windows(2).all(|pair| {
        let gap = pair[1] - pair[0];
        let remainder = gap % period;
        remainder < PERIOD_TOLERANCE_MS || period - remainder < PERIOD_TOLERANCE_MS
    })
}

/// Turn the two recorded envelopes into one offset.
pub fn estimate_offset(
    video: &[Sample],
    audio: &[Sample],
) -> Result<Measurement, CalibrationError> {
    let flashes = onsets(video, MIN_VIDEO_SWING).ok_or(CalibrationError::NoFlash)?;
    let beeps = onsets(audio, MIN_AUDIO_SWING).ok_or(CalibrationError::NoBeep)?;
    if flashes.is_empty() {
        return Err(CalibrationError::NoFlash);
    }
    if beeps.is_empty() {
        return Err(CalibrationError::NoBeep);
    }
    if !periodic(&flashes) || !periodic(&beeps) {
        return Err(CalibrationError::NotThePattern);
    }

    // Pair each flash with the nearest beep inside the unambiguous window.
    let mut deltas: Vec<f64> = flashes
        .iter()
        .filter_map(|&flash| {
            let nearest = beeps
                .iter()
                .map(|&beep| flash - beep)
                .min_by(|a, b| a.abs().partial_cmp(&b.abs()).unwrap())?;
            (nearest.abs() <= MAX_PAIR_WINDOW_MS).then_some(nearest)
        })
        .collect();
    if deltas.len() < MIN_CYCLES {
        return Err(CalibrationError::TooFewCycles {
            paired: deltas.len(),
        });
    }

    deltas.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = deltas[deltas.len() / 2];
    let jitter = deltas
        .iter()
        .map(|d| (d - median).abs())
        .fold(0.0f64, f64::max);
    if jitter > MAX_JITTER_MS {
        return Err(CalibrationError::Unstable { jitter_ms: jitter });
    }
    Ok(Measurement {
        offset_ms: median,
        cycles: deltas.len(),
        jitter_ms: jitter,
    })
}

// ---------------------------------------------------------------------------
// The armed video probe (Tauri-managed state)
// ---------------------------------------------------------------------------

/// The video probe's series cap (~68 s at 60 fps) — guards a dialog left
/// armed; a run is ~15 s.
const VIDEO_MAX_SAMPLES: usize = 4_096;

struct VideoProbe {
    source: SourceId,
    armed_at: Instant,
    samples: Vec<Sample>,
}

/// Command→render-loop state for the workbench's video probe: a command arms
/// it on one source, the render loop's drain pushes a mean-luma sample per
/// received frame, the finish command takes the series. (The audio twin
/// lives inside the audio engine — see `AudioEngine::calibrate`.)
#[derive(Default)]
pub struct CalibrationState {
    video: Mutex<Option<VideoProbe>>,
}

impl CalibrationState {
    fn lock(&self) -> std::sync::MutexGuard<'_, Option<VideoProbe>> {
        self.video
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// Arm the probe on `source`, clearing any previous series. `armed_at`
    /// must be the same instant the audio tap was armed with — both series
    /// share it as their zero.
    pub fn arm(&self, source: SourceId, armed_at: Instant) {
        *self.lock() = Some(VideoProbe {
            source,
            armed_at,
            samples: Vec::new(),
        });
    }

    pub fn disarm(&self) {
        *self.lock() = None;
    }

    /// The armed source, if any — the render loop's cheap per-tick check.
    pub fn armed_source(&self) -> Option<SourceId> {
        self.lock().as_ref().map(|probe| probe.source)
    }

    /// Record one drained frame's luma. `captured_at` is the frame's arrival
    /// stamp; frames captured before arming are skipped.
    pub fn push_frame(&self, source: SourceId, captured_at: Instant, luma: f32) {
        let mut guard = self.lock();
        let Some(probe) = guard.as_mut() else { return };
        if probe.source != source || probe.samples.len() >= VIDEO_MAX_SAMPLES {
            return;
        }
        if let Some(since) = captured_at.checked_duration_since(probe.armed_at) {
            probe.samples.push((since.as_secs_f64() * 1_000.0, luma));
        }
    }

    /// A copy of the series recorded so far.
    pub fn series(&self) -> Vec<Sample> {
        self.lock()
            .as_ref()
            .map(|probe| probe.samples.clone())
            .unwrap_or_default()
    }
}

/// Mean luma (0..1) of ~2 k stride-aware sampled pixels. Channel order is
/// irrelevant (the mean of r, g, b is the same for RGBA and BGRA), so both
/// pixel formats sample without a branch.
pub fn mean_luma(frame: &fcap_capture::Frame) -> f32 {
    let width = frame.width as usize;
    let height = frame.height as usize;
    let stride = frame.stride as usize;
    if width == 0 || height == 0 || frame.data.len() < stride * height {
        return 0.0;
    }
    let step_y = (height / 45).max(1);
    let step_x = (width / 45).max(1);
    let mut sum: u64 = 0;
    let mut count: u64 = 0;
    let mut y = 0;
    while y < height {
        let row = y * stride;
        let mut x = 0;
        while x < width {
            let at = row + x * 4;
            sum += u64::from(frame.data[at])
                + u64::from(frame.data[at + 1])
                + u64::from(frame.data[at + 2]);
            count += 1;
            x += step_x;
        }
        y += step_y;
    }
    (sum as f64 / (count as f64 * 3.0 * 255.0)) as f32
}

/// The observed swing (max above the quiet floor) of a series — the live
/// "seeing flashes / hearing beeps" feedback while measuring.
pub fn swing(samples: &[Sample]) -> f32 {
    if samples.len() < 4 {
        return 0.0;
    }
    let mut values: Vec<f32> = samples.iter().map(|&(_, v)| v).collect();
    values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    values[values.len() - 1] - values[values.len() / 5]
}

/// The swing thresholds, shared with the live status so "signal seen" in
/// the dialog means exactly "the estimator will accept it".
pub const VIDEO_SWING: f32 = MIN_VIDEO_SWING;
pub const AUDIO_SWING: f32 = MIN_AUDIO_SWING;

#[cfg(test)]
mod tests {
    use super::*;

    /// A camera-style luma envelope: `fps` sampling, flashes (100 ms of
    /// 0.85 over a 0.08 floor) starting at each time in `flash_starts`.
    fn video_series(fps: f64, until_ms: f64, flash_starts: &[f64]) -> Vec<Sample> {
        let step = 1_000.0 / fps;
        let mut out = Vec::new();
        let mut t = 0.0;
        while t < until_ms {
            let lit = flash_starts.iter().any(|&s| t >= s && t < s + 100.0);
            out.push((t, if lit { 0.85 } else { 0.08 }));
            t += step;
        }
        out
    }

    /// A mic-style peak envelope: 10 ms blocks, beeps (100 ms at 0.4 over a
    /// 0.005 floor) starting at each time in `beep_starts`.
    fn audio_series(until_ms: f64, beep_starts: &[f64]) -> Vec<Sample> {
        let mut out = Vec::new();
        let mut t = 0.0;
        while t < until_ms {
            let loud = beep_starts.iter().any(|&s| t >= s && t < s + 100.0);
            out.push((t, if loud { 0.4 } else { 0.005 }));
            t += 10.0;
        }
        out
    }

    const PERIOD: f64 = FLASH_BEEP_PERIOD_MS as f64;

    fn cycle_starts(first: f64, count: usize) -> Vec<f64> {
        (0..count).map(|k| first + k as f64 * PERIOD).collect()
    }

    #[test]
    fn measures_a_late_camera_within_a_frame() {
        // Beeps land at 500, 2500, …; the camera sees flashes 120 ms later.
        let audio = audio_series(9_000.0, &cycle_starts(500.0, 4));
        let video = video_series(30.0, 9_000.0, &cycle_starts(620.0, 4));
        let m = estimate_offset(&video, &audio).expect("measure");
        assert_eq!(m.cycles, 4);
        assert!(
            (m.offset_ms - 120.0).abs() <= 35.0,
            "got {} — must be within one 30 fps frame of the truth",
            m.offset_ms
        );
    }

    #[test]
    fn a_negative_offset_is_reported_honestly() {
        // Audio later than video: the delta comes out negative.
        let audio = audio_series(9_000.0, &cycle_starts(700.0, 4));
        let video = video_series(60.0, 9_000.0, &cycle_starts(500.0, 4));
        let m = estimate_offset(&video, &audio).expect("measure");
        assert!(m.offset_ms < -150.0, "got {}", m.offset_ms);
    }

    #[test]
    fn a_missed_cycle_still_measures() {
        // The camera dropped the middle flash (auto-exposure hiccup): gaps
        // of 2 and 4 seconds are both whole cycles.
        let audio = audio_series(11_000.0, &cycle_starts(500.0, 5));
        let video = video_series(30.0, 11_000.0, &[600.0, 2_600.0, 6_600.0, 8_600.0]);
        let m = estimate_offset(&video, &audio).expect("measure");
        assert_eq!(m.cycles, 4);
        assert!((m.offset_ms - 100.0).abs() <= 35.0);
    }

    #[test]
    fn a_dark_room_is_no_flash() {
        let audio = audio_series(9_000.0, &cycle_starts(500.0, 4));
        let video = video_series(30.0, 9_000.0, &[]); // camera saw nothing
        assert_eq!(
            estimate_offset(&video, &audio),
            Err(CalibrationError::NoFlash)
        );
    }

    #[test]
    fn a_silent_mic_is_no_beep() {
        let audio = audio_series(9_000.0, &[]);
        let video = video_series(30.0, 9_000.0, &cycle_starts(500.0, 4));
        assert_eq!(
            estimate_offset(&video, &audio),
            Err(CalibrationError::NoBeep)
        );
    }

    #[test]
    fn two_cycles_are_not_enough() {
        let audio = audio_series(5_000.0, &cycle_starts(500.0, 2));
        let video = video_series(30.0, 5_000.0, &cycle_starts(600.0, 2));
        assert_eq!(
            estimate_offset(&video, &audio),
            Err(CalibrationError::TooFewCycles { paired: 2 })
        );
    }

    #[test]
    fn room_events_off_the_cycle_are_rejected() {
        // A light switched on at odd times — bright, but not our pattern.
        let audio = audio_series(9_000.0, &cycle_starts(500.0, 4));
        let video = video_series(30.0, 9_000.0, &[400.0, 1_100.0, 3_800.0, 7_300.0]);
        assert_eq!(
            estimate_offset(&video, &audio),
            Err(CalibrationError::NotThePattern)
        );
    }

    #[test]
    fn disagreeing_cycles_refuse_a_number() {
        // Deltas of +40/+160/+40/+160 ms (e.g. the camera alternating
        // exposure lag): jitter far past the bound. Spacing stays on-cycle
        // within tolerance, so it *looks* like the pattern — but the spread
        // must refuse to produce a single number.
        let audio = audio_series(9_000.0, &cycle_starts(500.0, 4));
        let video = video_series(120.0, 9_000.0, &[540.0, 2_660.0, 4_540.0, 6_660.0]);
        match estimate_offset(&video, &audio) {
            Err(CalibrationError::Unstable { jitter_ms }) => assert!(jitter_ms > MAX_JITTER_MS),
            other => panic!("expected Unstable, got {other:?}"),
        }
    }

    #[test]
    fn sub_sample_interpolation_beats_raw_quantization() {
        // At 30 fps the raw first-frame-above error is up to 33 ms; the
        // interpolated crossing should land much closer on a clean edge.
        let audio = audio_series(9_000.0, &cycle_starts(500.0, 4));
        let video = video_series(30.0, 9_000.0, &cycle_starts(530.0, 4));
        let m = estimate_offset(&video, &audio).expect("measure");
        assert!((m.offset_ms - 30.0).abs() <= 35.0, "got {}", m.offset_ms);
    }
}
