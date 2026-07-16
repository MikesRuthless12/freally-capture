//! Voice chain tools (CAP-N36): a split-band de-esser and a rumble-guard
//! low-cut. Owned classic DSP — no ML, per the charter.

use fcap_scene::AudioFilterKind;

use crate::dsp::{lin_to_db, one_pole_coef, Biquad};
use crate::filters::{FilterCtx, FilterProc};

/// Butterworth Q for the de-esser detector split and the rumble low-cut: a
/// clean cut with no resonant bump.
const CLEAN_Q: f32 = 0.707;
const DEESSER_ENV_ATTACK_MS: f32 = 1.0;
const DEESSER_ENV_RELEASE_MS: f32 = 60.0;

/// Split-band de-esser: a high-pass **detector** measures the sibilance band's
/// level; when it exceeds the threshold, a **high-shelf cut** attenuates that
/// band (up to `amount`) while the low/mid body passes untouched. Driving the
/// reduction through a minimum-phase shelf (rather than subtracting a
/// phase-shifted band) keeps the cut clean. The detector is stereo-linked so
/// de-essing never shifts the stereo image.
pub struct DeEsser {
    sample_rate: f32,
    freq_hz: f32,
    threshold_db: f32,
    amount_db: f32,
    /// Per-channel detector high-pass (measures the sibilance band).
    detect: [Biquad; 2],
    /// Per-channel output high-shelf (the dynamic cut).
    shelf: [Biquad; 2],
    /// Linked sibilance-band envelope.
    env: f32,
    env_attack: f32,
    env_release: f32,
    /// The shelf's last-applied cut (dB); recompute the coefficients only when
    /// the reduction moves past a small threshold, not every sample.
    last_reduction: f32,
}

impl DeEsser {
    pub fn new(sample_rate: f32, freq_hz: f32, threshold_db: f32, amount_db: f32) -> Self {
        Self {
            sample_rate,
            freq_hz,
            threshold_db,
            amount_db,
            detect: [
                Biquad::high_pass(sample_rate, freq_hz, CLEAN_Q),
                Biquad::high_pass(sample_rate, freq_hz, CLEAN_Q),
            ],
            shelf: [
                Biquad::high_shelf(sample_rate, freq_hz, 0.0),
                Biquad::high_shelf(sample_rate, freq_hz, 0.0),
            ],
            env: 0.0,
            env_attack: one_pole_coef(DEESSER_ENV_ATTACK_MS, sample_rate),
            env_release: one_pole_coef(DEESSER_ENV_RELEASE_MS, sample_rate),
            last_reduction: -1.0, // force the first-frame shelf compute
        }
    }
}

impl FilterProc for DeEsser {
    fn process(&mut self, block: &mut [f32], _ctx: &FilterCtx) {
        for frame in block.chunks_exact_mut(2) {
            let h0 = self.detect[0].process(frame[0]);
            let h1 = self.detect[1].process(frame[1]);
            let level = h0.abs().max(h1.abs());
            let coef = if level > self.env {
                self.env_attack
            } else {
                self.env_release
            };
            self.env = level + (self.env - level) * coef;

            // How far the band is over threshold → how deep to cut (0..amount).
            let over = lin_to_db(self.env) - self.threshold_db;
            let reduction = over.clamp(0.0, self.amount_db);
            // Re-tune the shelf's gain in place only when the cut has moved
            // audibly (state carries; freq is fixed) — the smoothed envelope
            // holds steady for long runs, so most samples reuse coefficients.
            if (reduction - self.last_reduction).abs() > 0.02 {
                self.shelf[0].set_high_shelf(self.sample_rate, self.freq_hz, -reduction);
                self.shelf[1].set_high_shelf(self.sample_rate, self.freq_hz, -reduction);
                self.last_reduction = reduction;
            }
            frame[0] = self.shelf[0].process(frame[0]);
            frame[1] = self.shelf[1].process(frame[1]);
        }
    }

    fn update(&mut self, kind: &AudioFilterKind) -> bool {
        let AudioFilterKind::DeEsser {
            freq_hz,
            threshold_db,
            amount_db,
        } = kind
        else {
            return false;
        };
        let freq = freq_hz.clamp(3_000.0, 12_000.0);
        if (freq - self.freq_hz).abs() > f32::EPSILON {
            self.freq_hz = freq;
            for channel in &mut self.detect {
                channel.set_high_pass(self.sample_rate, freq, CLEAN_Q);
            }
            self.last_reduction = -1.0; // the shelf must retune to the new freq
        }
        self.threshold_db = threshold_db.clamp(-96.0, 0.0);
        self.amount_db = amount_db.clamp(0.0, 24.0);
        true
    }
}

/// Rumble guard: a clean 2nd-order low-cut for desk thumps and plosives.
pub struct RumbleGuard {
    sample_rate: f32,
    freq_hz: f32,
    high: [Biquad; 2],
}

impl RumbleGuard {
    pub fn new(sample_rate: f32, freq_hz: f32) -> Self {
        Self {
            sample_rate,
            freq_hz,
            high: [
                Biquad::high_pass(sample_rate, freq_hz, CLEAN_Q),
                Biquad::high_pass(sample_rate, freq_hz, CLEAN_Q),
            ],
        }
    }
}

impl FilterProc for RumbleGuard {
    fn process(&mut self, block: &mut [f32], _ctx: &FilterCtx) {
        for frame in block.chunks_exact_mut(2) {
            frame[0] = self.high[0].process(frame[0]);
            frame[1] = self.high[1].process(frame[1]);
        }
    }

    fn update(&mut self, kind: &AudioFilterKind) -> bool {
        let AudioFilterKind::RumbleGuard { freq_hz } = kind else {
            return false;
        };
        let freq = freq_hz.clamp(20.0, 300.0);
        if (freq - self.freq_hz).abs() > f32::EPSILON {
            self.freq_hz = freq;
            for channel in &mut self.high {
                channel.set_high_pass(self.sample_rate, freq, CLEAN_Q);
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsp::{goertzel, lin_to_db};

    const SR: f32 = 48_000.0;

    fn tone(freq: f32, amp: f32, frames: usize) -> Vec<f32> {
        (0..frames)
            .flat_map(|i| {
                let s = amp * (2.0 * std::f32::consts::PI * freq * i as f32 / SR).sin();
                [s, s]
            })
            .collect()
    }

    fn run(proc: &mut dyn FilterProc, mut block: Vec<f32>) -> Vec<f32> {
        proc.process(&mut block, &FilterCtx::empty());
        block
    }

    #[test]
    fn de_esser_tames_sibilance_but_spares_the_body() {
        // A loud 7 kHz "ess" is reduced; a 300 Hz body tone passes ~untouched.
        let frames = 19_200;
        let mut de = DeEsser::new(SR, 6_000.0, -35.0, 12.0);
        let out = run(&mut de, tone(7_000.0, 0.8, frames));
        let sib: Vec<f32> = out[frames..].iter().step_by(2).copied().collect();
        let sib_db = lin_to_db(goertzel(&sib, SR, 7_000.0));
        assert!(sib_db < -3.0, "sibilance reduced, got {sib_db} dB");

        let mut de = DeEsser::new(SR, 6_000.0, -35.0, 12.0);
        let out = run(&mut de, tone(300.0, 0.5, frames));
        let body: Vec<f32> = out[frames..].iter().step_by(2).copied().collect();
        let body_db = lin_to_db(goertzel(&body, SR, 300.0) / 0.5);
        assert!(body_db.abs() < 1.0, "the body is spared, got {body_db} dB");
    }

    #[test]
    fn rumble_guard_cuts_the_lows() {
        let frames = 19_200;
        let mut guard = RumbleGuard::new(SR, 90.0);
        let out = run(&mut guard, tone(35.0, 0.8, frames));
        let low: Vec<f32> = out[frames..].iter().step_by(2).copied().collect();
        assert!(
            lin_to_db(goertzel(&low, SR, 35.0) / 0.8) < -12.0,
            "35 Hz well below a 90 Hz corner is cut hard"
        );

        let mut guard = RumbleGuard::new(SR, 90.0);
        let out = run(&mut guard, tone(1_000.0, 0.5, frames));
        let mid: Vec<f32> = out[frames..].iter().step_by(2).copied().collect();
        assert!(
            lin_to_db(goertzel(&mid, SR, 1_000.0) / 0.5).abs() < 1.0,
            "1 kHz passes"
        );
    }
}
