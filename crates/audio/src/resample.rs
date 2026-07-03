//! Streaming linear resampler (interleaved stereo, any rate → the engine
//! rate). Linear interpolation is the honest, owned baseline for capture
//! alignment — device clocks drift far more than its error floor.

pub struct Resampler {
    /// Input frames advanced per output frame.
    step: f64,
    /// Position inside the *previous + current* input, in input frames,
    /// relative to `prev` (so 0.0 = the carried frame).
    pos: f64,
    /// The last input frame carried across calls.
    prev: [f32; 2],
    primed: bool,
    passthrough: bool,
}

impl Resampler {
    pub fn new(in_rate: u32, out_rate: u32) -> Self {
        Self {
            step: in_rate as f64 / out_rate as f64,
            pos: 1.0,
            prev: [0.0; 2],
            primed: false,
            passthrough: in_rate == out_rate,
        }
    }

    /// Convert interleaved stereo `input`, appending to `out`.
    pub fn process(&mut self, input: &[f32], out: &mut Vec<f32>) {
        if self.passthrough {
            out.extend_from_slice(input);
            return;
        }
        let frames = input.len() / 2;
        if frames == 0 {
            return;
        }
        let prev = self.prev;
        let frame = |index: usize| -> [f32; 2] {
            if index == 0 {
                prev
            } else {
                [input[(index - 1) * 2], input[(index - 1) * 2 + 1]]
            }
        };
        if !self.primed {
            self.primed = true;
            // Start interpolating from the first real frame (there is no
            // carried frame yet).
            self.pos = 1.0;
        }
        // Valid positions: 0.0 ..= frames (index into [prev, input...]).
        while self.pos <= frames as f64 {
            let base = self.pos.floor();
            let frac = (self.pos - base) as f32;
            let i = base as usize;
            let a = frame(i);
            let b = if i < frames { frame(i + 1) } else { a };
            out.push(a[0] + (b[0] - a[0]) * frac);
            out.push(a[1] + (b[1] - a[1]) * frac);
            self.pos += self.step;
        }
        // Carry the last input frame; re-base pos onto it.
        self.prev = frame(frames);
        self.pos -= frames as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsp::goertzel;

    #[test]
    fn passthrough_is_exact() {
        let mut resampler = Resampler::new(48_000, 48_000);
        let input: Vec<f32> = (0..960).map(|i| i as f32 / 960.0).collect();
        let mut out = Vec::new();
        resampler.process(&input, &mut out);
        assert_eq!(out, input);
    }

    #[test]
    fn ratio_and_tone_survive_44100_to_48000() {
        let mut resampler = Resampler::new(44_100, 48_000);
        let freq = 440.0;
        let seconds = 1.0f32;
        let n = (44_100.0 * seconds) as usize;
        let mut out = Vec::new();
        // Feed in uneven chunks to exercise the streaming carry.
        let input: Vec<f32> = (0..n)
            .flat_map(|i| {
                let s = (2.0 * std::f32::consts::PI * freq * i as f32 / 44_100.0).sin();
                [s, s]
            })
            .collect();
        for chunk in input.chunks(2 * 441) {
            resampler.process(chunk, &mut out);
        }
        let out_frames = out.len() / 2;
        let expected = (48_000.0 * seconds) as usize;
        assert!(
            (out_frames as i64 - expected as i64).abs() <= 2,
            "expected ~{expected} frames, got {out_frames}"
        );
        // The tone must land at the same absolute frequency at 48 kHz.
        let left: Vec<f32> = out.iter().step_by(2).copied().collect();
        let amp = goertzel(&left[4_800..43_200], 48_000.0, freq);
        assert!(
            (amp - 1.0).abs() < 0.05,
            "the 440 Hz tone should survive at ~unity, got {amp}"
        );
    }

    #[test]
    fn downsampling_matches_the_ratio() {
        let mut resampler = Resampler::new(96_000, 48_000);
        let input = vec![0.1f32; 2 * 9_600];
        let mut out = Vec::new();
        resampler.process(&input, &mut out);
        let frames = out.len() / 2;
        assert!(
            (frames as i64 - 4_800).abs() <= 2,
            "expected ~4800 frames, got {frames}"
        );
    }
}
