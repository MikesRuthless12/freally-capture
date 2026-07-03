//! ITU-R BS.1770 loudness (LUFS): K-weighting (shelf + RLB high-pass
//! biquads) → 100 ms mean-square sub-blocks → **momentary** (400 ms) and
//! **short-term** (3 s) loudness. The public-standard math, implemented here.
//!
//! Fixed to the engine's 48 kHz rate — the coefficient set below is the
//! BS.1770 reference table for exactly that rate.

use std::collections::VecDeque;

use crate::dsp::Biquad;

/// Readings below this are reported as silence (`None`).
const GATE_LUFS: f64 = -70.0;
const SUB_BLOCK_MS: usize = 100;
const MOMENTARY_BLOCKS: usize = 4; // 400 ms
const SHORT_TERM_BLOCKS: usize = 30; // 3 s

/// BS.1770-4 stage 1 (high shelf) for 48 kHz.
fn stage1() -> Biquad {
    Biquad::from_coefficients(
        1.535_124_9,
        -2.691_696_2,
        1.198_392_9,
        -1.690_659_3,
        0.732_480_77,
    )
}

/// BS.1770-4 stage 2 (RLB high-pass) for 48 kHz.
fn stage2() -> Biquad {
    Biquad::from_coefficients(1.0, -2.0, 1.0, -1.990_047_5, 0.990_072_25)
}

pub struct LufsMeter {
    filters: [[Biquad; 2]; 2],
    /// Running sum of K-weighted squares in the current sub-block.
    sum: f64,
    samples_in_block: usize,
    block_len: usize,
    blocks: VecDeque<f64>,
}

impl LufsMeter {
    pub fn new(sample_rate: u32) -> Self {
        debug_assert_eq!(sample_rate, 48_000, "K-coefficients are 48 kHz");
        Self {
            filters: [[stage1(), stage2()], [stage1(), stage2()]],
            sum: 0.0,
            samples_in_block: 0,
            block_len: sample_rate as usize * SUB_BLOCK_MS / 1_000,
            blocks: VecDeque::with_capacity(SHORT_TERM_BLOCKS + 1),
        }
    }

    /// Feed interleaved stereo samples of the program mix.
    pub fn push(&mut self, interleaved: &[f32]) {
        for frame in interleaved.chunks_exact(2) {
            let mut weighted_sq = 0.0f64;
            for (channel, &sample) in frame.iter().enumerate() {
                let [f1, f2] = &mut self.filters[channel];
                let value = f2.process(f1.process(sample));
                weighted_sq += (value as f64) * (value as f64);
            }
            self.sum += weighted_sq;
            self.samples_in_block += 1;
            if self.samples_in_block == self.block_len {
                self.blocks.push_back(self.sum / self.block_len as f64);
                if self.blocks.len() > SHORT_TERM_BLOCKS {
                    self.blocks.pop_front();
                }
                self.sum = 0.0;
                self.samples_in_block = 0;
            }
        }
    }

    fn loudness_over(&self, blocks: usize) -> Option<f32> {
        if self.blocks.len() < blocks {
            return None;
        }
        let mean: f64 = self.blocks.iter().rev().take(blocks).sum::<f64>() / blocks as f64;
        let lufs = -0.691 + 10.0 * (mean.max(1e-12)).log10();
        (lufs > GATE_LUFS).then_some(lufs as f32)
    }

    /// Momentary loudness (400 ms window), `None` below the -70 LUFS gate.
    pub fn momentary(&self) -> Option<f32> {
        self.loudness_over(MOMENTARY_BLOCKS)
    }

    /// Short-term loudness (3 s window), `None` below the -70 LUFS gate.
    pub fn short_term(&self) -> Option<f32> {
        self.loudness_over(SHORT_TERM_BLOCKS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stereo_sine(freq: f32, amp: f32, seconds: f32) -> Vec<f32> {
        let n = (48_000.0 * seconds) as usize;
        let mut out = Vec::with_capacity(n * 2);
        for i in 0..n {
            let s = amp * (2.0 * std::f32::consts::PI * freq * i as f32 / 48_000.0).sin();
            out.push(s);
            out.push(s);
        }
        out
    }

    #[test]
    fn full_scale_997hz_reads_near_the_reference() {
        // BS.1770 calibration: a 0 dBFS 997 Hz sine in ONE channel reads
        // −3.01 LUFS (the −0.691 constant cancels the K-shelf's gain there),
        // so the same sine in BOTH channels reads +3.01 higher ≈ 0.0 LUFS.
        let mut meter = LufsMeter::new(48_000);
        meter.push(&stereo_sine(997.0, 1.0, 1.0));
        let momentary = meter.momentary().expect("above the gate");
        assert!(
            momentary.abs() < 0.5,
            "expected ≈ 0.0 LUFS, got {momentary}"
        );
    }

    #[test]
    fn level_tracks_amplitude_linearly() {
        let mut meter = LufsMeter::new(48_000);
        meter.push(&stereo_sine(997.0, 0.1, 1.0)); // −20 dB
        let momentary = meter.momentary().expect("above the gate");
        assert!(
            (momentary - (-20.0)).abs() < 0.5,
            "expected ≈ -20.0 LUFS, got {momentary}"
        );
    }

    #[test]
    fn silence_gates_to_none() {
        let mut meter = LufsMeter::new(48_000);
        meter.push(&vec![0.0f32; 48_000 * 2]);
        assert_eq!(meter.momentary(), None);
    }

    #[test]
    fn short_term_needs_three_seconds() {
        let mut meter = LufsMeter::new(48_000);
        meter.push(&stereo_sine(997.0, 0.5, 1.0));
        assert!(meter.momentary().is_some());
        assert_eq!(meter.short_term(), None, "not enough history yet");
        meter.push(&stereo_sine(997.0, 0.5, 2.5));
        assert!(meter.short_term().is_some());
    }
}
