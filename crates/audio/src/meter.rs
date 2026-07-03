//! Per-source level metering: peak + RMS per channel, accumulated between
//! UI polls so a 20 Hz event never misses a transient between blocks.

/// One reading: linear peak and RMS per channel (L, R).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Levels {
    pub peak: [f32; 2],
    pub rms: [f32; 2],
}

/// Accumulates blocks; `take()` yields the levels since the last take.
#[derive(Debug, Default)]
pub struct LevelAccumulator {
    peak: [f32; 2],
    sq_sum: [f64; 2],
    frames: usize,
}

impl LevelAccumulator {
    pub fn push_block(&mut self, interleaved: &[f32]) {
        for frame in interleaved.chunks_exact(2) {
            for (channel, &sample) in frame.iter().enumerate() {
                self.peak[channel] = self.peak[channel].max(sample.abs());
                self.sq_sum[channel] += (sample as f64) * (sample as f64);
            }
        }
        self.frames += interleaved.len() / 2;
    }

    /// The levels accumulated since the last call, then reset.
    pub fn take(&mut self) -> Levels {
        let rms = if self.frames == 0 {
            [0.0, 0.0]
        } else {
            [
                (self.sq_sum[0] / self.frames as f64).sqrt() as f32,
                (self.sq_sum[1] / self.frames as f64).sqrt() as f32,
            ]
        };
        let levels = Levels {
            peak: self.peak,
            rms,
        };
        *self = Self::default();
        levels
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peak_and_rms_of_a_sine() {
        let mut acc = LevelAccumulator::default();
        let block: Vec<f32> = (0..48_000)
            .flat_map(|i| {
                let s = 0.5 * (2.0 * std::f32::consts::PI * 440.0 * i as f32 / 48_000.0).sin();
                [s, s * 0.5]
            })
            .collect();
        acc.push_block(&block);
        let levels = acc.take();
        assert!((levels.peak[0] - 0.5).abs() < 1e-3);
        assert!((levels.peak[1] - 0.25).abs() < 1e-3);
        // Sine RMS = amp / √2.
        assert!((levels.rms[0] - 0.5 / 2f32.sqrt()).abs() < 1e-3);

        // take() resets.
        let empty = acc.take();
        assert_eq!(empty, Levels::default());
    }
}
