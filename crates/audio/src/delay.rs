//! A fixed stereo delay line — the per-source A/V **sync offset**.

use std::collections::VecDeque;

pub struct DelayLine {
    buf: VecDeque<f32>,
    delay_frames: usize,
}

impl DelayLine {
    /// A delay of `delay_frames` frames (0 = passthrough).
    pub fn new(delay_frames: usize) -> Self {
        let mut buf = VecDeque::with_capacity((delay_frames + 1) * 2);
        buf.extend(std::iter::repeat(0.0f32).take(delay_frames * 2));
        Self { buf, delay_frames }
    }

    /// Delay the interleaved stereo block in place.
    pub fn process(&mut self, block: &mut [f32]) {
        if self.delay_frames == 0 {
            return;
        }
        for sample in block.iter_mut() {
            self.buf.push_back(*sample);
            *sample = self.buf.pop_front().unwrap_or(0.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn impulse_arrives_exactly_delay_later() {
        let delay = 7;
        let mut line = DelayLine::new(delay);
        let mut block = vec![0.0f32; 2 * 32];
        block[0] = 1.0; // left impulse at frame 0
        block[1] = -1.0; // right impulse at frame 0
        line.process(&mut block);
        for frame in 0..32 {
            let (l, r) = (block[frame * 2], block[frame * 2 + 1]);
            if frame == delay {
                assert_eq!((l, r), (1.0, -1.0), "impulse at frame {frame}");
            } else {
                assert_eq!((l, r), (0.0, 0.0), "silence at frame {frame}");
            }
        }
    }

    #[test]
    fn zero_delay_is_identity() {
        let mut line = DelayLine::new(0);
        let mut block = vec![0.25f32, -0.5, 0.75, 1.0];
        let expected = block.clone();
        line.process(&mut block);
        assert_eq!(block, expected);
    }
}
