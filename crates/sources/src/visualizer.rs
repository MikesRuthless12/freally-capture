//! CAP-N15 — the **audio visualizer source**: classic FFT spectrum bars, an
//! oscilloscope, or stereo VU meters, rendered on the CPU from the mixer's
//! visualizer tap and published on the same latest-wins channel every
//! capture uses. Classic DSP only — the owned radix-2 FFT, no ML, no new
//! dependencies.
//!
//! Honest signal notes:
//! - Strips are tapped **post-fader** (the signal that actually mixes) — a
//!   muted strip visualizes flat, exactly like it sounds.
//! - A visualizer whose target stopped being fed (a removed source, a dead
//!   bus) decays to the floor instead of freezing (`VisRing::age`).
//! - The spectrum is a plain Hann-windowed 2048-point FFT into log-spaced
//!   bands, drawn in dB — no smoothing tricks beyond the decay the user set.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use fcap_audio::vis::{self, VisTarget};
use fcap_audio::SAMPLE_RATE;
use fcap_capture::{frame_channel, CaptureError, CaptureSession};

use crate::static_source::{check_dimension, rgba_frame};

/// FFT length (~43 ms at 48 kHz) — enough low-end resolution for a 40 Hz
/// first band without smearing transients.
const FFT_N: usize = 2_048;
/// The drawn floor; anything quieter reads as zero height.
const FLOOR_DB: f32 = -60.0;
/// Spectrum band range (log-spaced).
const BAND_LO_HZ: f32 = 40.0;
const BAND_HI_HZ: f32 = 16_000.0;
/// The generator's frame cadence — smooth motion at half the canvas rate.
const FPS: u32 = 30;
/// How long a peak marker holds before it starts to fall.
const PEAK_HOLD_S: f32 = 1.0;
/// How fast a released peak marker falls, dB/s.
const PEAK_FALL_DB_PER_S: f32 = 12.0;
/// A ring this stale reads as silence (its target stopped being mixed).
const STALE: Duration = Duration::from_millis(500);
/// VU ballistics: the RMS window, interleaved samples (~300 ms).
const VU_WINDOW_SAMPLES: usize = 28_800;

/// Which face the visualizer draws.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Style {
    Bars,
    Scope,
    Vu,
}

/// Everything the generator needs, already resolved by the caller.
#[derive(Debug, Clone)]
pub struct VisualizerConfig {
    pub style: Style,
    pub target: VisTarget,
    pub width: u32,
    pub height: u32,
    /// Spectrum bar count (bars style) — clamped to 8..=128.
    pub bands: u32,
    /// Straight RGBA accent; the background stays transparent.
    pub color: [u8; 4],
    pub peak_hold: bool,
    /// Bar fall rate, dB/s — clamped to 6..=120.
    pub decay_db_per_s: f32,
}

/// Start the visualizer session thread.
pub fn start_visualizer(config: VisualizerConfig) -> Result<CaptureSession, CaptureError> {
    check_dimension("visualizer width", config.width)
        .and_then(|()| check_dimension("visualizer height", config.height))
        .map_err(|err| CaptureError::Backend(err.to_string()))?;
    let bands = config.bands.clamp(8, 128) as usize;
    let decay = config.decay_db_per_s.clamp(6.0, 120.0);
    let (sender, receiver) = frame_channel();
    let stop = Arc::new(AtomicBool::new(false));
    let thread_stop = Arc::clone(&stop);
    let join = std::thread::Builder::new()
        .name("fcap-visualizer".into())
        .spawn(move || {
            let ring = vis::ring(&config.target);
            let width = config.width;
            let height = config.height;
            let color = config.color;
            let period = Duration::from_micros(1_000_000 / u64::from(FPS));
            let dt = 1.0 / FPS as f32;
            // Bars state: heights + peak markers, both in dB.
            let mut window = vec![0.0f32; FFT_N * 2];
            let mut mono = vec![0.0f32; FFT_N];
            let mut heights = vec![FLOOR_DB; bands];
            let mut peaks: Vec<(f32, f32)> = vec![(FLOOR_DB, 0.0); bands];
            // Scope state: one column per output pixel, two samples each.
            let mut scope_window = vec![0.0f32; (width as usize) * 4];
            let mut scope_mono = vec![0.0f32; (width as usize) * 2];
            // VU state: per-channel peak markers, in dB.
            let mut vu_window = vec![0.0f32; VU_WINDOW_SAMPLES];
            let mut vu_peaks: [(f32, f32); 2] = [(FLOOR_DB, 0.0), (FLOOR_DB, 0.0)];
            let mut next = Instant::now();
            loop {
                if thread_stop.load(Ordering::Relaxed) || !sender.is_open() {
                    return;
                }
                // A stale ring is silence — the face decays instead of freezing.
                let stale = ring.age().map_or(true, |age| age > STALE);
                let frame = match config.style {
                    Style::Bars => {
                        if stale {
                            window.fill(0.0);
                        } else {
                            ring.latest(&mut window);
                        }
                        downmix_mono(&window, &mut mono);
                        let spectrum = spectrum_db(&mono, bands);
                        step_heights(&mut heights, &spectrum, dt, decay);
                        step_peaks(&mut peaks, &heights, dt);
                        let heights01: Vec<f32> = heights.iter().copied().map(level01).collect();
                        let peaks01: Vec<f32> = peaks.iter().map(|(db, _)| level01(*db)).collect();
                        draw_bars(
                            width,
                            height,
                            &heights01,
                            config.peak_hold.then_some(peaks01.as_slice()),
                            color,
                        )
                    }
                    Style::Scope => {
                        if stale {
                            scope_window.fill(0.0);
                        } else {
                            ring.latest(&mut scope_window);
                        }
                        downmix_mono(&scope_window, &mut scope_mono);
                        draw_scope(width, height, &scope_mono, color)
                    }
                    Style::Vu => {
                        if stale {
                            vu_window.fill(0.0);
                        } else {
                            ring.latest(&mut vu_window);
                        }
                        let levels = vu_levels_db(&vu_window);
                        let mut shown = [(0.0f32, 0.0f32); 2];
                        for ch in 0..2 {
                            let (rms_db, peak_db) = levels[ch];
                            let marker = &mut vu_peaks[ch];
                            step_peak(marker, peak_db, dt);
                            shown[ch] = (level01(rms_db), level01(marker.0));
                        }
                        draw_vu(width, height, &shown, config.peak_hold, color)
                    }
                };
                sender.send(rgba_frame(width, height, frame));
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

/// Interleaved stereo → mono average; `out.len() * 2 == interleaved.len()`.
fn downmix_mono(interleaved: &[f32], out: &mut [f32]) {
    for (dst, frame) in out.iter_mut().zip(interleaved.chunks_exact(2)) {
        *dst = (frame[0] + frame[1]) * 0.5;
    }
}

/// `bands + 1` log-spaced edges across [`BAND_LO_HZ`], [`BAND_HI_HZ`].
fn band_edges(bands: usize) -> Vec<f32> {
    let ratio = BAND_HI_HZ / BAND_LO_HZ;
    (0..=bands)
        .map(|index| BAND_LO_HZ * ratio.powf(index as f32 / bands as f32))
        .collect()
}

/// Hann-windowed FFT of `mono` (must be [`FFT_N`] long) into `bands`
/// log-spaced dB values, clamped to [`FLOOR_DB`]..=0. Each band takes its
/// loudest bin; a band narrower than one bin reads its nearest bin.
fn spectrum_db(mono: &[f32], bands: usize) -> Vec<f32> {
    debug_assert_eq!(mono.len(), FFT_N);
    let mut re: Vec<f32> = (0..FFT_N)
        .map(|index| {
            // Hann window (coherent gain 0.5, compensated below).
            let phase = std::f32::consts::TAU * index as f32 / FFT_N as f32;
            mono[index] * 0.5 * (1.0 - phase.cos())
        })
        .collect();
    let mut im = vec![0.0f32; FFT_N];
    fcap_audio::fft::fft_in_place(&mut re, &mut im, false);
    // Sine of amplitude 1.0 → |bin| = N/2 · window gain 0.5 → N/4.
    let full_scale = FFT_N as f32 / 4.0;
    let magnitude = |bin: usize| -> f32 {
        let (r, i) = (re[bin], im[bin]);
        (r * r + i * i).sqrt() / full_scale
    };
    let hz_per_bin = SAMPLE_RATE as f32 / FFT_N as f32;
    let edges = band_edges(bands);
    let max_bin = FFT_N / 2 - 1;
    edges
        .windows(2)
        .map(|edge| {
            let lo = (edge[0] / hz_per_bin).ceil() as usize;
            let hi = ((edge[1] / hz_per_bin).ceil() as usize).min(max_bin + 1);
            let peak = if lo < hi {
                (lo..hi).map(magnitude).fold(0.0f32, f32::max)
            } else {
                // Band narrower than one bin — read the nearest.
                let center = ((edge[0] * edge[1]).sqrt() / hz_per_bin).round() as usize;
                magnitude(center.clamp(1, max_bin))
            };
            (20.0 * (peak.max(1e-9)).log10()).clamp(FLOOR_DB, 0.0)
        })
        .collect()
}

/// Rise instantly, fall at most `decay` dB/s — the classic analyzer feel.
fn step_heights(heights: &mut [f32], targets: &[f32], dt_s: f32, decay_db_per_s: f32) {
    for (height, target) in heights.iter_mut().zip(targets) {
        if *target >= *height {
            *height = *target;
        } else {
            *height = (*height - decay_db_per_s * dt_s).max(*target).max(FLOOR_DB);
        }
    }
}

/// One peak marker `(db, age_s)`: latch on a new peak, hold for
/// [`PEAK_HOLD_S`], then fall at [`PEAK_FALL_DB_PER_S`].
fn step_peak(marker: &mut (f32, f32), current_db: f32, dt_s: f32) {
    if current_db >= marker.0 {
        *marker = (current_db, 0.0);
    } else {
        marker.1 += dt_s;
        if marker.1 > PEAK_HOLD_S {
            marker.0 = (marker.0 - PEAK_FALL_DB_PER_S * dt_s).max(FLOOR_DB);
        }
    }
}

fn step_peaks(peaks: &mut [(f32, f32)], heights: &[f32], dt_s: f32) {
    for (marker, height) in peaks.iter_mut().zip(heights) {
        step_peak(marker, *height, dt_s);
    }
}

/// dB → 0..1 against the drawn floor.
fn level01(db: f32) -> f32 {
    ((db - FLOOR_DB) / -FLOOR_DB).clamp(0.0, 1.0)
}

/// Per-channel `(rms_db, peak_db)` of an interleaved stereo window.
fn vu_levels_db(interleaved: &[f32]) -> [(f32, f32); 2] {
    let mut sum_sq = [0.0f64; 2];
    let mut peak = [0.0f32; 2];
    for frame in interleaved.chunks_exact(2) {
        for ch in 0..2 {
            sum_sq[ch] += f64::from(frame[ch]) * f64::from(frame[ch]);
            peak[ch] = peak[ch].max(frame[ch].abs());
        }
    }
    let frames = (interleaved.len() / 2).max(1) as f64;
    let mut out = [(FLOOR_DB, FLOOR_DB); 2];
    for ch in 0..2 {
        let rms = (sum_sq[ch] / frames).sqrt() as f32;
        out[ch] = (
            (20.0 * rms.max(1e-9).log10()).clamp(FLOOR_DB, 0.0),
            (20.0 * peak[ch].max(1e-9).log10()).clamp(FLOOR_DB, 0.0),
        );
    }
    out
}

/// The peak markers' tick color — the accent lifted halfway to white.
fn lighten(color: [u8; 4]) -> [u8; 4] {
    [
        (u16::from(color[0]) + 255).div_euclid(2) as u8,
        (u16::from(color[1]) + 255).div_euclid(2) as u8,
        (u16::from(color[2]) + 255).div_euclid(2) as u8,
        color[3],
    ]
}

fn put(data: &mut [u8], width: u32, x: usize, y: usize, color: [u8; 4]) {
    let index = (y * width as usize + x) * 4;
    data[index..index + 4].copy_from_slice(&color);
}

/// Spectrum bars on a transparent background, filled bottom-up, one pixel
/// gap between bars (when they are wide enough), peak ticks in the lifted
/// accent.
fn draw_bars(
    width: u32,
    height: u32,
    heights01: &[f32],
    peaks01: Option<&[f32]>,
    color: [u8; 4],
) -> Vec<u8> {
    let (w, h) = (width as usize, height as usize);
    let mut data = vec![0u8; w * h * 4];
    let bands = heights01.len().max(1);
    let tick = lighten(color);
    for (band, level) in heights01.iter().enumerate() {
        let x0 = band * w / bands;
        let mut x1 = (band + 1) * w / bands;
        if x1 > x0 + 2 {
            x1 -= 1; // the gap
        }
        let filled = (level.clamp(0.0, 1.0) * h as f32).round() as usize;
        for y in h - filled..h {
            for x in x0..x1.min(w) {
                put(&mut data, width, x, y, color);
            }
        }
        if let Some(peaks) = peaks01 {
            let peak = peaks.get(band).copied().unwrap_or(0.0);
            if peak > 0.0 {
                let peak_row = h.saturating_sub((peak * h as f32).round() as usize);
                for y in peak_row..(peak_row + 2).min(h) {
                    for x in x0..x1.min(w) {
                        put(&mut data, width, x, y, tick);
                    }
                }
            }
        }
    }
    data
}

/// Oscilloscope: a faint center line and one min/max column per pixel of
/// the mono window (two samples per column), on transparency.
fn draw_scope(width: u32, height: u32, mono: &[f32], color: [u8; 4]) -> Vec<u8> {
    let (w, h) = (width as usize, height as usize);
    let mut data = vec![0u8; w * h * 4];
    let center_color = [color[0], color[1], color[2], color[3] / 4];
    let center = (h - 1) / 2;
    for x in 0..w {
        put(&mut data, width, x, center, center_color);
    }
    let map = |sample: f32| -> usize {
        ((1.0 - sample.clamp(-1.0, 1.0)) * 0.5 * (h - 1) as f32).round() as usize
    };
    let mut previous = mono.first().copied().unwrap_or(0.0);
    for x in 0..w {
        let a = mono.get(x * 2).copied().unwrap_or(0.0);
        let b = mono.get(x * 2 + 1).copied().unwrap_or(a);
        // Bridge with the previous column so steep edges stay connected.
        let lo = a.min(b).min(previous);
        let hi = a.max(b).max(previous);
        for y in map(hi)..=map(lo) {
            put(&mut data, width, x, y.min(h - 1), color);
        }
        previous = b;
    }
    data
}

/// Stereo VU: two horizontal lanes (L above R) with faint tracks, RMS fill,
/// and optional peak-hold ticks.
fn draw_vu(
    width: u32,
    height: u32,
    shown: &[(f32, f32); 2],
    peak_hold: bool,
    color: [u8; 4],
) -> Vec<u8> {
    let (w, h) = (width as usize, height as usize);
    let mut data = vec![0u8; w * h * 4];
    let track = [color[0], color[1], color[2], color[3] / 8];
    let tick = lighten(color);
    let lane_h = h / 4;
    for (ch, (rms01, peak01)) in shown.iter().enumerate() {
        let y0 = if ch == 0 { h / 8 } else { h * 5 / 8 };
        let y1 = (y0 + lane_h).min(h);
        let filled = (rms01.clamp(0.0, 1.0) * w as f32).round() as usize;
        for y in y0..y1 {
            for x in 0..w {
                put(
                    &mut data,
                    width,
                    x,
                    y,
                    if x < filled { color } else { track },
                );
            }
        }
        if peak_hold && *peak01 > 0.0 {
            let x0 = ((peak01 * w as f32).round() as usize).min(w.saturating_sub(2));
            for y in y0..y1 {
                for x in x0..(x0 + 2).min(w) {
                    put(&mut data, width, x, y, tick);
                }
            }
        }
    }
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sine(freq: f32, amplitude: f32) -> Vec<f32> {
        (0..FFT_N)
            .map(|index| {
                amplitude * (std::f32::consts::TAU * freq * index as f32 / SAMPLE_RATE as f32).sin()
            })
            .collect()
    }

    /// The band whose range contains `freq`.
    fn band_of(freq: f32, bands: usize) -> usize {
        let edges = band_edges(bands);
        edges
            .windows(2)
            .position(|edge| freq >= edge[0] && freq < edge[1])
            .expect("in range")
    }

    #[test]
    fn a_1khz_sine_lands_in_its_band_and_nowhere_near_8khz() {
        let spectrum = spectrum_db(&sine(1_000.0, 1.0), 48);
        let hot = band_of(1_000.0, 48);
        let loudest = spectrum
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.total_cmp(b.1))
            .expect("bands")
            .0;
        // Spectral leakage can put the crown on the adjacent band edge —
        // one band of tolerance, never eight.
        assert!(
            loudest.abs_diff(hot) <= 1,
            "loudest {loudest}, expected ~{hot}"
        );
        assert!(spectrum[hot.max(loudest)] > -6.0, "near full scale");
        let far = band_of(8_000.0, 48);
        assert!(
            spectrum[far] < spectrum[hot.max(loudest)] - 40.0,
            "8 kHz stays ≥40 dB down: {}",
            spectrum[far]
        );
    }

    #[test]
    fn heights_rise_instantly_and_fall_at_the_decay_rate() {
        let mut heights = vec![FLOOR_DB];
        step_heights(&mut heights, &[-3.0], 0.1, 30.0);
        assert_eq!(heights[0], -3.0, "rise is instant");
        step_heights(&mut heights, &[FLOOR_DB], 0.1, 30.0);
        assert!((heights[0] - -6.0).abs() < 1e-4, "fell 3 dB in 100 ms");
    }

    #[test]
    fn a_peak_marker_holds_then_falls() {
        let mut marker = (FLOOR_DB, 0.0);
        step_peak(&mut marker, -3.0, 0.1);
        assert_eq!(marker, (-3.0, 0.0), "latched");
        // Held below the hold window: value keeps, age grows.
        step_peak(&mut marker, -30.0, 0.5);
        assert_eq!(marker.0, -3.0, "holding");
        // Past the hold window it falls.
        step_peak(&mut marker, -30.0, 1.0);
        assert!(marker.0 < -3.0, "falling after the hold");
    }

    #[test]
    fn bars_fill_from_the_bottom_over_transparency() {
        let data = draw_bars(8, 8, &[1.0, 0.0], None, [10, 20, 30, 255]);
        let pixel = |x: usize, y: usize| &data[(y * 8 + x) * 4..(y * 8 + x) * 4 + 4];
        assert_eq!(pixel(0, 7), &[10, 20, 30, 255], "first bar bottom filled");
        assert_eq!(pixel(0, 0), &[10, 20, 30, 255], "full height");
        assert_eq!(pixel(3, 0)[3], 0, "gap column transparent");
        assert_eq!(pixel(5, 7)[3], 0, "silent band transparent");
    }

    #[test]
    fn scope_of_silence_draws_only_the_center_line() {
        let width = 16usize;
        let mono = vec![0.0f32; width * 2];
        let data = draw_scope(width as u32, 9, &mono, [255, 0, 0, 255]);
        let pixel = |x: usize, y: usize| &data[(y * width + x) * 4..(y * width + x) * 4 + 4];
        assert_eq!(pixel(3, 4), &[255, 0, 0, 255], "flat trace on the center");
        assert_eq!(pixel(3, 0)[3], 0, "top transparent");
        assert_eq!(pixel(3, 8)[3], 0, "bottom transparent");
    }

    #[test]
    fn vu_fills_with_level_and_empties_in_silence() {
        let loud = draw_vu(16, 16, &[(1.0, 0.0), (1.0, 0.0)], false, [0, 255, 0, 255]);
        let quiet = draw_vu(16, 16, &[(0.0, 0.0), (0.0, 0.0)], false, [0, 255, 0, 255]);
        let pixel = |data: &[u8], x: usize, y: usize| data[(y * 16 + x) * 4 + 3];
        assert_eq!(pixel(&loud, 15, 3), 255, "left lane full");
        assert!(
            pixel(&quiet, 15, 3) < 64,
            "silent lane shows only the track"
        );
    }

    #[test]
    fn vu_levels_measure_a_full_scale_sine_at_minus_three_db() {
        let window: Vec<f32> = (0..VU_WINDOW_SAMPLES / 2)
            .flat_map(|index| {
                let sample =
                    (std::f32::consts::TAU * 1_000.0 * index as f32 / SAMPLE_RATE as f32).sin();
                [sample, sample]
            })
            .collect();
        let [(left_rms, left_peak), _] = vu_levels_db(&window);
        assert!((left_rms - -3.01).abs() < 0.1, "rms {left_rms}");
        assert!(left_peak > -0.1, "peak {left_peak}");
    }
}
