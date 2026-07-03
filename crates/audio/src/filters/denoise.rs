//! Spectral noise suppression — **owned classic DSP, no ML** (per charter).
//!
//! An STFT gate: 512-sample frames, 50% overlap, √Hann analysis + synthesis
//! windows (exact COLA). Per bin, a noise-floor estimate follows the
//! **noise-dominated** frames (falls fast onto minima, averages gently while
//! near the floor, only creeps when the bin is occupied by signal — speech
//! pauses teach it the room), and a spectral-subtraction gain with a
//! strength-dependent floor suppresses noise-dominated bins. Gains are
//! smoothed over time and frequency to keep "musical noise" down.
//! Algorithmic latency: one frame (512 samples ≈ 10.7 ms at 48 kHz).

use std::collections::VecDeque;

use fcap_scene::AudioFilterKind;

use crate::fft::fft_in_place;
use crate::filters::{FilterCtx, FilterProc};

const FRAME: usize = 512;
const HOP: usize = FRAME / 2;
const BINS: usize = FRAME / 2 + 1;
/// Per-frame creep of the noise estimate while the bin is occupied by signal
/// (~×1.45/s at the 187.5 frames/s hop rate) — sustained speech barely lifts
/// the floor; pauses re-teach it within ~100 ms.
const NOISE_RISE: f32 = 1.002;
/// A bin is "occupied by signal" past this many times the tracked floor.
const OCCUPIED_RATIO: f32 = 3.0;
/// Temporal gain smoothing (previous vs fresh gain).
const GAIN_SMOOTH: f32 = 0.5;

struct ChannelState {
    /// Accumulating input; drained one hop at a time.
    in_buf: Vec<f32>,
    /// Overlap-add accumulator (one frame long, shifted by a hop per frame).
    ola: Vec<f32>,
    /// Finished output samples, ready to hand back.
    out: VecDeque<f32>,
    /// Per-bin noise magnitude estimate.
    noise: Vec<f32>,
    gain_prev: Vec<f32>,
    seeded: bool,
}

impl ChannelState {
    fn new() -> Self {
        let mut out = VecDeque::with_capacity(FRAME * 2);
        // Prime one hop of silence so output never underruns mid-stream
        // (the streaming deficit is at most HOP-1 samples).
        out.extend(std::iter::repeat(0.0f32).take(HOP));
        Self {
            // Half a frame of pre-roll so the first real samples land in a
            // full analysis window.
            in_buf: vec![0.0; FRAME - HOP],
            ola: vec![0.0; FRAME],
            out,
            noise: vec![0.0; BINS],
            gain_prev: vec![1.0; BINS],
            seeded: false,
        }
    }
}

pub struct Denoiser {
    window: [f32; FRAME],
    /// Oversubtraction factor (how aggressively the floor is subtracted).
    oversub: f32,
    /// The gain floor — how far a noise-only bin is pushed down.
    floor: f32,
    channels: [ChannelState; 2],
    scratch: Vec<f32>,
}

impl Denoiser {
    pub fn new(strength: f32) -> Self {
        let strength = strength.clamp(0.0, 1.0);
        let mut window = [0.0f32; FRAME];
        for (i, w) in window.iter_mut().enumerate() {
            // √Hann == sin(πn/N); applied at analysis *and* synthesis it
            // multiplies to Hann, whose 50%-overlap sum is exactly 1.
            *w = (std::f32::consts::PI * i as f32 / FRAME as f32).sin();
        }
        Self {
            window,
            oversub: 1.0 + 2.0 * strength,
            floor: crate::dsp::db_to_lin(-8.0 - 22.0 * strength),
            channels: [ChannelState::new(), ChannelState::new()],
            scratch: Vec::new(),
        }
    }

    fn process_frame(window: &[f32; FRAME], oversub: f32, floor: f32, state: &mut ChannelState) {
        let mut re = [0.0f32; FRAME];
        let mut im = [0.0f32; FRAME];
        for i in 0..FRAME {
            re[i] = state.in_buf[i] * window[i];
        }
        fft_in_place(&mut re, &mut im, false);

        // Per-bin gains from the tracked noise floor. The very first frame
        // only *seeds* the floor and passes through at unity — otherwise, if
        // the denoiser is added (or its params edited, which rebuilt an
        // unfamiliar chain in earlier revisions) while speech is playing, the
        // floor would seed at speech level and gate the first ~100 ms to
        // silence. Under-suppressing the first frame is far less audible.
        let first_frame = !state.seeded;
        let mut gains = [1.0f32; BINS];
        for k in 0..BINS {
            let mag = (re[k] * re[k] + im[k] * im[k]).sqrt();
            let noise = &mut state.noise[k];
            if first_frame {
                *noise = mag;
            } else if mag < *noise {
                *noise = 0.7 * *noise + 0.3 * mag; // fall fast onto minima
            } else if mag < *noise * OCCUPIED_RATIO {
                *noise = 0.98 * *noise + 0.02 * mag; // ride the noise band
            } else {
                *noise *= NOISE_RISE; // occupied by signal: only creep
            }
            let fresh = if first_frame {
                1.0 // pass the seeding frame through
            } else if mag > 1e-9 {
                (1.0 - oversub * (*noise / mag)).max(floor)
            } else {
                floor
            };
            let smoothed = GAIN_SMOOTH * state.gain_prev[k] + (1.0 - GAIN_SMOOTH) * fresh;
            state.gain_prev[k] = smoothed;
            gains[k] = smoothed;
        }
        state.seeded = true;

        // Frequency smoothing (1-2-1) knocks down isolated "musical" bins.
        let mut smoothed = gains;
        for k in 1..BINS - 1 {
            smoothed[k] = 0.25 * gains[k - 1] + 0.5 * gains[k] + 0.25 * gains[k + 1];
        }

        // Apply symmetrically (real signal ⇒ conjugate-symmetric spectrum).
        for (k, gain) in smoothed.iter().enumerate() {
            re[k] *= gain;
            im[k] *= gain;
            if k > 0 && k < FRAME / 2 {
                re[FRAME - k] *= gain;
                im[FRAME - k] *= gain;
            }
        }

        fft_in_place(&mut re, &mut im, true);
        for i in 0..FRAME {
            state.ola[i] += re[i] * window[i];
        }
        state.out.extend(state.ola[..HOP].iter().copied());
        state.ola.copy_within(HOP.., 0);
        state.ola[FRAME - HOP..].fill(0.0);
    }
}

impl FilterProc for Denoiser {
    fn process(&mut self, block: &mut [f32], _ctx: &FilterCtx) {
        let frames = block.len() / 2;
        for channel in 0..2 {
            let state = &mut self.channels[channel];
            self.scratch.clear();
            self.scratch
                .extend(block.iter().skip(channel).step_by(2).copied());
            for &sample in &self.scratch {
                state.in_buf.push(sample);
                if state.in_buf.len() == FRAME {
                    Self::process_frame(&self.window, self.oversub, self.floor, state);
                    state.in_buf.drain(..HOP);
                }
            }
            for i in 0..frames {
                block[i * 2 + channel] = state.out.pop_front().unwrap_or(0.0);
            }
        }
    }

    fn update(&mut self, kind: &AudioFilterKind) -> bool {
        let AudioFilterKind::Denoise { strength } = kind else {
            return false;
        };
        // Only the aggressiveness changes; the learned per-bin noise floor,
        // the overlap-add state, and the streaming buffers all carry — so a
        // strength drag re-tunes without re-adapting from scratch.
        let strength = strength.clamp(0.0, 1.0);
        self.oversub = 1.0 + 2.0 * strength;
        self.floor = crate::dsp::db_to_lin(-8.0 - 22.0 * strength);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsp::{db_to_lin, goertzel, lin_to_db};

    const SR: f32 = 48_000.0;
    const BLOCK_FRAMES: usize = 480;

    /// Deterministic pseudo-noise in [-1, 1] (no rand dependency).
    struct Lcg(u32);

    impl Lcg {
        fn next(&mut self) -> f32 {
            self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
            (self.0 >> 8) as f32 / (1 << 24) as f32 * 2.0 - 1.0
        }
    }

    fn rms(samples: &[f32]) -> f32 {
        (samples.iter().map(|s| (s * s) as f64).sum::<f64>() / samples.len() as f64).sqrt() as f32
    }

    /// Run a mono signal through the denoiser (duplicated to stereo),
    /// returning the mono output.
    fn run(denoiser: &mut Denoiser, signal: &[f32]) -> Vec<f32> {
        let mut out = Vec::with_capacity(signal.len());
        for chunk in signal.chunks(BLOCK_FRAMES) {
            let mut block: Vec<f32> = chunk.iter().flat_map(|&s| [s, s]).collect();
            denoiser.process(&mut block, &FilterCtx::empty());
            out.extend(block.iter().step_by(2).copied());
        }
        out
    }

    #[test]
    fn steady_noise_is_suppressed() {
        let mut lcg = Lcg(0x1234_5678);
        let noise_amp = 0.05;
        let signal: Vec<f32> = (0..SR as usize * 2)
            .map(|_| noise_amp * lcg.next())
            .collect();
        let mut denoiser = Denoiser::new(0.5);
        let out = run(&mut denoiser, &signal);

        // After adaptation the steady floor should drop hard (> 6 dB).
        let late_in = &signal[SR as usize..];
        let late_out = &out[SR as usize..];
        let reduction = rms(late_out) / rms(late_in);
        assert!(
            reduction < 0.5,
            "expected > 6 dB noise reduction, got {:.1} dB",
            -lin_to_db(reduction)
        );
    }

    #[test]
    fn speech_like_bursts_survive_while_the_floor_drops() {
        // 500 ms tone bursts (speech stand-in) over continuous noise: the
        // pauses teach the noise floor; the bursts must come through.
        let mut lcg = Lcg(0x8BAD_F00D);
        let tone_amp = 0.5;
        let noise_amp = 0.02;
        let total = SR as usize * 3;
        let signal: Vec<f32> = (0..total)
            .map(|i| {
                let t = i as f32 / SR;
                let burst_on = (t * 2.0) as usize % 2 == 0; // 0.5 s on/off
                let tone = if burst_on {
                    tone_amp * (2.0 * std::f32::consts::PI * 440.0 * t).sin()
                } else {
                    0.0
                };
                tone + noise_amp * lcg.next()
            })
            .collect();
        let mut denoiser = Denoiser::new(0.5);
        let out = run(&mut denoiser, &signal);

        // A late ON window (2.0–2.4 s), shifted by the 512-sample latency.
        let latency = FRAME;
        let on = &out[(SR as usize * 2 + latency)..(SR as usize * 2 + latency + 19_200)];
        let tone_out = goertzel(on, SR, 440.0);
        assert!(
            tone_out > tone_amp * db_to_lin(-3.0),
            "the tone should survive within ~3 dB, got {:.1} dB rel",
            lin_to_db(tone_out / tone_amp)
        );

        // A late OFF window (2.5–2.9 s): the floor should sit well under the
        // raw noise.
        let off_start = (SR as usize * 5 / 2) + latency;
        let off = &out[off_start..off_start + 19_200];
        assert!(
            rms(off) < noise_amp * 0.5,
            "the pause floor should drop > 6 dB, got {:.4} vs {noise_amp}",
            rms(off)
        );
    }

    #[test]
    fn output_length_matches_input_length() {
        let mut denoiser = Denoiser::new(0.3);
        let mut total_out = 0usize;
        for len in [480usize, 480, 128, 997, 480] {
            let mut block = vec![0.25f32; len * 2];
            denoiser.process(&mut block, &FilterCtx::empty());
            total_out += block.len() / 2;
        }
        assert_eq!(total_out, 480 + 480 + 128 + 997 + 480);
    }
}
