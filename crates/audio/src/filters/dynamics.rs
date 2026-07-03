//! Envelope-based dynamics: noise gate, compressor, limiter, and the
//! sidechain ducker. All peak-sensing with one-pole attack/release ballistics
//! — classic broadcast processing, owned outright.

use fcap_scene::{AudioFilterKind, SourceId};

use crate::dsp::{db_to_lin, lin_to_db, one_pole_coef};
use crate::filters::{FilterCtx, FilterProc};

/// The engine's fixed rate — the sample rate every processor is built at, so
/// an in-place `update` recomputes its time-constant coefficients against the
/// same clock its running state was accumulated on.
const SR: f32 = crate::SAMPLE_RATE as f32;

/// Stereo peak of one interleaved frame.
#[inline]
fn frame_peak(l: f32, r: f32) -> f32 {
    l.abs().max(r.abs())
}

/// Downward noise gate with hysteresis (open/close thresholds), hold, and
/// attack/release gain ramps.
pub struct NoiseGate {
    open_lin: f32,
    close_lin: f32,
    attack_coef: f32,
    release_coef: f32,
    env_release_coef: f32,
    hold_frames: u32,
    env: f32,
    gain: f32,
    hold: u32,
    open: bool,
}

impl NoiseGate {
    pub fn new(
        sample_rate: f32,
        open_threshold_db: f32,
        close_threshold_db: f32,
        attack_ms: f32,
        hold_ms: f32,
        release_ms: f32,
    ) -> Self {
        Self {
            open_lin: db_to_lin(open_threshold_db),
            close_lin: db_to_lin(close_threshold_db),
            attack_coef: one_pole_coef(attack_ms, sample_rate),
            release_coef: one_pole_coef(release_ms, sample_rate),
            // Fixed detector ballistics: instant rise, ~50 ms fall.
            env_release_coef: one_pole_coef(50.0, sample_rate),
            hold_frames: (hold_ms * 0.001 * sample_rate) as u32,
            env: 0.0,
            gain: 0.0,
            hold: 0,
            open: false,
        }
    }
}

impl FilterProc for NoiseGate {
    fn process(&mut self, block: &mut [f32], _ctx: &FilterCtx) {
        for frame in block.chunks_exact_mut(2) {
            let level = frame_peak(frame[0], frame[1]);
            self.env = level.max(self.env * self.env_release_coef);

            if self.env >= self.open_lin {
                self.open = true;
            }
            if self.open {
                if self.env >= self.close_lin {
                    self.hold = self.hold_frames;
                } else if self.hold > 0 {
                    self.hold -= 1;
                } else {
                    self.open = false;
                }
            }

            let target = if self.open { 1.0 } else { 0.0 };
            let coef = if target > self.gain {
                self.attack_coef
            } else {
                self.release_coef
            };
            self.gain += (target - self.gain) * (1.0 - coef);
            frame[0] *= self.gain;
            frame[1] *= self.gain;
        }
    }

    fn update(&mut self, kind: &AudioFilterKind) -> bool {
        let AudioFilterKind::NoiseGate {
            open_threshold_db,
            close_threshold_db,
            attack_ms,
            hold_ms,
            release_ms,
        } = kind
        else {
            return false;
        };
        let open = open_threshold_db.clamp(-96.0, 0.0);
        self.open_lin = db_to_lin(open);
        self.close_lin = db_to_lin(close_threshold_db.clamp(-96.0, 0.0).min(open));
        self.attack_coef = one_pole_coef(attack_ms.clamp(1.0, 500.0), SR);
        self.release_coef = one_pole_coef(release_ms.clamp(1.0, 3_000.0), SR);
        self.hold_frames = (hold_ms.clamp(0.0, 3_000.0) * 0.001 * SR) as u32;
        true
    }
}

/// Peak-sensing downward compressor, hard knee, with make-up gain.
pub struct Compressor {
    slope: f32,
    threshold_db: f32,
    attack_coef: f32,
    release_coef: f32,
    makeup: f32,
    reduction_db: f32,
}

impl Compressor {
    pub fn new(
        sample_rate: f32,
        ratio: f32,
        threshold_db: f32,
        attack_ms: f32,
        release_ms: f32,
        output_gain_db: f32,
    ) -> Self {
        Self {
            slope: 1.0 - 1.0 / ratio.max(1.0),
            threshold_db,
            attack_coef: one_pole_coef(attack_ms, sample_rate),
            release_coef: one_pole_coef(release_ms, sample_rate),
            makeup: db_to_lin(output_gain_db),
            reduction_db: 0.0,
        }
    }
}

impl FilterProc for Compressor {
    fn process(&mut self, block: &mut [f32], _ctx: &FilterCtx) {
        for frame in block.chunks_exact_mut(2) {
            let over = lin_to_db(frame_peak(frame[0], frame[1])) - self.threshold_db;
            let target = (over * self.slope).max(0.0);
            let coef = if target > self.reduction_db {
                self.attack_coef
            } else {
                self.release_coef
            };
            self.reduction_db += (target - self.reduction_db) * (1.0 - coef);
            let gain = db_to_lin(-self.reduction_db) * self.makeup;
            frame[0] *= gain;
            frame[1] *= gain;
        }
    }

    fn update(&mut self, kind: &AudioFilterKind) -> bool {
        let AudioFilterKind::Compressor {
            ratio,
            threshold_db,
            attack_ms,
            release_ms,
            output_gain_db,
        } = kind
        else {
            return false;
        };
        self.slope = 1.0 - 1.0 / ratio.clamp(1.0, 32.0);
        self.threshold_db = threshold_db.clamp(-60.0, 0.0);
        self.attack_coef = one_pole_coef(attack_ms.clamp(0.1, 500.0), SR);
        self.release_coef = one_pole_coef(release_ms.clamp(1.0, 3_000.0), SR);
        self.makeup = db_to_lin(output_gain_db.clamp(-30.0, 30.0));
        true
    }
}

/// Fast peak limiter: instant attack on the gain computer plus a hard sample
/// clamp at the ceiling as the safety net.
pub struct Limiter {
    ceiling_db: f32,
    ceiling_lin: f32,
    release_coef: f32,
    reduction_db: f32,
}

impl Limiter {
    pub fn new(sample_rate: f32, threshold_db: f32, release_ms: f32) -> Self {
        Self {
            ceiling_db: threshold_db,
            ceiling_lin: db_to_lin(threshold_db),
            release_coef: one_pole_coef(release_ms, sample_rate),
            reduction_db: 0.0,
        }
    }
}

impl FilterProc for Limiter {
    fn process(&mut self, block: &mut [f32], _ctx: &FilterCtx) {
        for frame in block.chunks_exact_mut(2) {
            let over = lin_to_db(frame_peak(frame[0], frame[1])) - self.ceiling_db;
            if over > self.reduction_db {
                self.reduction_db = over; // instant attack
            } else {
                self.reduction_db *= self.release_coef;
            }
            let gain = db_to_lin(-self.reduction_db.max(0.0));
            frame[0] = (frame[0] * gain).clamp(-self.ceiling_lin, self.ceiling_lin);
            frame[1] = (frame[1] * gain).clamp(-self.ceiling_lin, self.ceiling_lin);
        }
    }

    fn update(&mut self, kind: &AudioFilterKind) -> bool {
        let AudioFilterKind::Limiter {
            threshold_db,
            release_ms,
        } = kind
        else {
            return false;
        };
        let ceiling = threshold_db.clamp(-30.0, 0.0);
        self.ceiling_db = ceiling;
        self.ceiling_lin = db_to_lin(ceiling);
        self.release_coef = one_pole_coef(release_ms.clamp(1.0, 1_000.0), SR);
        true
    }
}

/// Sidechain ducker: dips this source while the trigger source's envelope
/// (from [`FilterCtx`]) sits above the threshold.
pub struct Ducker {
    trigger: Option<SourceId>,
    threshold_lin: f32,
    amount_db: f32,
    attack_coef: f32,
    release_coef: f32,
    current_db: f32,
}

impl Ducker {
    pub fn new(
        sample_rate: f32,
        trigger: Option<SourceId>,
        threshold_db: f32,
        amount_db: f32,
        attack_ms: f32,
        release_ms: f32,
    ) -> Self {
        Self {
            trigger,
            threshold_lin: db_to_lin(threshold_db),
            amount_db,
            attack_coef: one_pole_coef(attack_ms, sample_rate),
            release_coef: one_pole_coef(release_ms, sample_rate),
            current_db: 0.0,
        }
    }
}

impl FilterProc for Ducker {
    fn process(&mut self, block: &mut [f32], ctx: &FilterCtx) {
        let env = self
            .trigger
            .and_then(|id| ctx.envelopes.get(&id))
            .copied()
            .unwrap_or(0.0);
        let target = if env >= self.threshold_lin {
            self.amount_db
        } else {
            0.0
        };
        for frame in block.chunks_exact_mut(2) {
            let coef = if target > self.current_db {
                self.attack_coef
            } else {
                self.release_coef
            };
            self.current_db += (target - self.current_db) * (1.0 - coef);
            let gain = db_to_lin(-self.current_db);
            frame[0] *= gain;
            frame[1] *= gain;
        }
    }

    fn update(&mut self, kind: &AudioFilterKind) -> bool {
        let AudioFilterKind::Ducker {
            trigger,
            threshold_db,
            amount_db,
            attack_ms,
            release_ms,
        } = kind
        else {
            return false;
        };
        self.trigger = *trigger;
        self.threshold_lin = db_to_lin(threshold_db.clamp(-96.0, 0.0));
        self.amount_db = amount_db.clamp(0.0, 60.0);
        self.attack_coef = one_pole_coef(attack_ms.clamp(1.0, 1_000.0), SR);
        self.release_coef = one_pole_coef(release_ms.clamp(1.0, 5_000.0), SR);
        true
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    const SR: f32 = 48_000.0;

    fn sine_block(freq: f32, amp: f32, frames: usize, phase0: usize) -> Vec<f32> {
        let mut block = Vec::with_capacity(frames * 2);
        for i in 0..frames {
            let t = (phase0 + i) as f32 / SR;
            let sample = amp * (2.0 * std::f32::consts::PI * freq * t).sin();
            block.push(sample);
            block.push(sample);
        }
        block
    }

    fn peak(block: &[f32]) -> f32 {
        block.iter().fold(0.0f32, |acc, s| acc.max(s.abs()))
    }

    #[test]
    fn gate_passes_loud_and_silences_quiet() {
        let mut gate = NoiseGate::new(SR, -26.0, -32.0, 5.0, 50.0, 20.0);
        let ctx = FilterCtx::empty();

        // Loud (-6 dB): the gate opens and passes nearly unity.
        let mut loud = sine_block(440.0, db_to_lin(-6.0), 4_800, 0);
        gate.process(&mut loud, &ctx);
        let late = &loud[4_800..];
        assert!(
            peak(late) > db_to_lin(-7.0),
            "open gate should pass ~unity, got {}",
            lin_to_db(peak(late))
        );

        // Quiet (-48 dB): after hold + release the gate closes to ~silence.
        let mut quiet = sine_block(440.0, db_to_lin(-48.0), 48_000, 0);
        gate.process(&mut quiet, &ctx);
        let tail = &quiet[80_000..];
        assert!(
            peak(tail) < db_to_lin(-48.0) * 0.05,
            "closed gate should attenuate hard, got {}",
            lin_to_db(peak(tail))
        );
    }

    #[test]
    fn compressor_reduces_by_the_expected_ratio() {
        // -6 dB input, threshold -18, ratio 4 → 12 dB over → 9 dB reduction.
        let mut comp = Compressor::new(SR, 4.0, -18.0, 1.0, 100.0, 0.0);
        let ctx = FilterCtx::empty();
        let mut block = sine_block(1_000.0, db_to_lin(-6.0), 48_000, 0);
        comp.process(&mut block, &ctx);
        let late_peak_db = lin_to_db(peak(&block[80_000..]));
        assert!(
            (late_peak_db - (-15.0)).abs() < 1.0,
            "expected ~-15 dBFS out, got {late_peak_db}"
        );
    }

    #[test]
    fn limiter_caps_at_the_ceiling() {
        let mut limiter = Limiter::new(SR, -3.0, 50.0);
        let ctx = FilterCtx::empty();
        let mut block = sine_block(1_000.0, 1.0, 9_600, 0);
        limiter.process(&mut block, &ctx);
        let ceiling = db_to_lin(-3.0);
        assert!(
            peak(&block) <= ceiling + 1e-4,
            "no sample may pass the ceiling"
        );
    }

    #[test]
    fn ducker_dips_on_the_trigger_and_recovers() {
        let trigger = SourceId::new();
        let mut ducker = Ducker::new(SR, Some(trigger), -30.0, 12.0, 5.0, 5.0);

        let mut envelopes = HashMap::new();
        envelopes.insert(trigger, db_to_lin(-10.0)); // trigger speaking
        let ctx = FilterCtx {
            envelopes: &envelopes,
        };
        let mut block = vec![1.0f32; 9_600];
        ducker.process(&mut block, &ctx);
        let ducked = block[9_598];
        assert!(
            (lin_to_db(ducked) - (-12.0)).abs() < 0.5,
            "expected ~-12 dB duck, got {}",
            lin_to_db(ducked)
        );

        // Trigger goes quiet → gain recovers to unity.
        let quiet = HashMap::new();
        let ctx = FilterCtx { envelopes: &quiet };
        let mut block = vec![1.0f32; 9_600];
        ducker.process(&mut block, &ctx);
        assert!(
            block[9_598] > db_to_lin(-0.5),
            "expected recovery to ~unity, got {}",
            lin_to_db(block[9_598])
        );
    }

    #[test]
    fn ducker_without_a_trigger_is_inert() {
        let mut ducker = Ducker::new(SR, None, -30.0, 12.0, 5.0, 5.0);
        let ctx = FilterCtx::empty();
        let mut block = vec![0.5f32; 960];
        ducker.process(&mut block, &ctx);
        assert!((block[958] - 0.5).abs() < 1e-4);
    }
}
