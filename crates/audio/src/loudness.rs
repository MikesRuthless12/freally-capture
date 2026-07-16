//! CAP-N34 loudness rider: a slow gain that steers the program toward a chosen
//! integrated-loudness target (EBU R128 / platform LUFS), with a peak ceiling.
//!
//! The rider measures the program's short-term LUFS **before** its own gain
//! (so the control loop can't chase its own tail), slews a bounded gain toward
//! `target − measured`, and applies that gain — plus a peak-ceiling limiter — to
//! the master **and every track bus**, exactly like the transition duck. So
//! everything that records, streams, or meters sees the normalized program.
//! Off unless armed; the mix is bit-identical when idle.

use fcap_scene::TRACK_COUNT;

use crate::dsp::db_to_lin;
use crate::filters::dynamics::Limiter;
use crate::filters::{FilterCtx, FilterProc};
use crate::lufs::LufsMeter;
use crate::{BLOCK_FRAMES, SAMPLE_RATE};

/// The rider never trims or boosts more than this — a safety cage around a
/// mis-set target or a very quiet/loud source.
const MAX_GAIN_DB: f32 = 12.0;
/// A deliberately slow rider: at most this many dB of change per second, so the
/// loudness is steered, not pumped.
const SLEW_DB_PER_SEC: f32 = 3.0;

/// One armed loudness rider over the program buses.
pub struct LoudnessRider {
    target_lufs: f32,
    /// Measures the pre-rider master (the control signal).
    control: LufsMeter,
    gain_db: f32,
    /// Max gain change per 10 ms block.
    max_step: f32,
    /// Peak-ceiling limiters: `[0]` = master, `[1..=TRACK_COUNT]` = tracks.
    limiters: Vec<Limiter>,
}

impl LoudnessRider {
    pub fn new(target_lufs: f32, ceiling_db: f32) -> Self {
        let limiters = (0..=TRACK_COUNT)
            .map(|_| Limiter::new(SAMPLE_RATE as f32, ceiling_db, 60.0))
            .collect();
        Self {
            target_lufs,
            control: LufsMeter::new(SAMPLE_RATE),
            gain_db: 0.0,
            max_step: SLEW_DB_PER_SEC * (BLOCK_FRAMES as f32 / SAMPLE_RATE as f32),
            limiters,
        }
    }

    /// The gain the rider is currently applying (dB) — handy for tests/metering.
    pub fn gain_db(&self) -> f32 {
        self.gain_db
    }

    /// Retune target/ceiling in place, preserving the accumulated `gain_db` and
    /// the LUFS history — so nudging the target or ceiling steers from where the
    /// rider already is instead of snapping the whole program back to unity.
    pub fn retune(&mut self, target_lufs: f32, ceiling_db: f32) {
        self.target_lufs = target_lufs;
        for limiter in &mut self.limiters {
            limiter.set_ceiling(ceiling_db);
        }
    }

    /// Advance one block: measure, steer, apply to master + every track bus.
    pub fn process(&mut self, master: &mut [f32], tracks: &mut [Vec<f32>]) {
        self.control.push(master); // pre-rider — no feedback into the loop
        if let Some(short_term) = self.control.short_term() {
            let desired = (self.target_lufs - short_term).clamp(-MAX_GAIN_DB, MAX_GAIN_DB);
            let step = (desired - self.gain_db).clamp(-self.max_step, self.max_step);
            self.gain_db += step;
        }
        let gain = db_to_lin(self.gain_db);
        let ctx = FilterCtx::empty();
        for sample in master.iter_mut() {
            *sample *= gain;
        }
        self.limiters[0].process(master, &ctx);
        for (index, track) in tracks.iter_mut().enumerate() {
            let Some(limiter) = self.limiters.get_mut(index + 1) else {
                break;
            };
            for sample in track.iter_mut() {
                *sample *= gain;
            }
            limiter.process(track, &ctx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BLOCK_SAMPLES: usize = BLOCK_FRAMES * 2;

    fn tone_block(amp: f32, phase0: usize) -> Vec<f32> {
        (0..BLOCK_FRAMES)
            .flat_map(|i| {
                let t = (phase0 + i) as f32 / SAMPLE_RATE as f32;
                let s = amp * (2.0 * std::f32::consts::PI * 440.0 * t).sin();
                [s, s]
            })
            .collect()
    }

    fn peak(block: &[f32]) -> f32 {
        block.iter().fold(0.0f32, |acc, s| acc.max(s.abs()))
    }

    #[test]
    fn rider_boosts_a_quiet_program_toward_the_target() {
        // A quiet tone (~-30 LUFS) with a -16 LUFS target: the rider slews a
        // positive gain in (bounded, slow) and lifts the program.
        let mut rider = LoudnessRider::new(-16.0, -1.0);
        let mut master;
        let mut tracks = vec![vec![0.0f32; BLOCK_SAMPLES]; TRACK_COUNT];
        // ~6 s of audio so the 3 s short-term window fills and the slow slew runs.
        for block in 0..600 {
            master = tone_block(0.03, block * BLOCK_FRAMES);
            for track in &mut tracks {
                track.copy_from_slice(&master);
            }
            rider.process(&mut master, &mut tracks);
        }
        assert!(
            rider.gain_db() > 3.0,
            "a quiet program should be boosted toward target, gain {} dB",
            rider.gain_db()
        );
    }

    #[test]
    fn retune_preserves_the_accumulated_gain() {
        // Slew a positive gain in on a quiet program, then retune the target and
        // ceiling: the gain must carry (no snap back to unity), so the program
        // level doesn't jump on a live target/ceiling edit.
        let mut rider = LoudnessRider::new(-16.0, -1.0);
        let mut tracks = vec![vec![0.0f32; BLOCK_SAMPLES]; TRACK_COUNT];
        for block in 0..600 {
            let mut master = tone_block(0.03, block * BLOCK_FRAMES);
            for track in &mut tracks {
                track.copy_from_slice(&master);
            }
            rider.process(&mut master, &mut tracks);
        }
        let before = rider.gain_db();
        assert!(before > 3.0, "rider slewed a boost in, gain {before} dB");
        rider.retune(-14.0, -2.0);
        assert_eq!(rider.gain_db(), before, "retune keeps the accumulated gain");
    }

    #[test]
    fn rider_holds_the_peak_ceiling() {
        // A hot program with a -1 dBFS ceiling: the limiter keeps peaks under it.
        let mut rider = LoudnessRider::new(-16.0, -1.0);
        let mut tracks = vec![vec![0.0f32; BLOCK_SAMPLES]; TRACK_COUNT];
        let mut out = Vec::new();
        for block in 0..200 {
            let mut master = tone_block(0.98, block * BLOCK_FRAMES);
            for track in &mut tracks {
                track.copy_from_slice(&master);
            }
            rider.process(&mut master, &mut tracks);
            out = master;
        }
        assert!(
            peak(&out) <= db_to_lin(-1.0) + 0.02,
            "peaks stay under the ceiling, got {}",
            peak(&out)
        );
    }
}
