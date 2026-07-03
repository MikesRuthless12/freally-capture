//! The three-band tone EQ: low shelf (250 Hz), mid peak (1 kHz), high shelf
//! (4 kHz) — RBJ biquads per channel.

use fcap_scene::AudioFilterKind;

use crate::dsp::Biquad;
use crate::filters::{FilterCtx, FilterProc};

const SR: f32 = crate::SAMPLE_RATE as f32;

const LOW_SHELF_HZ: f32 = 250.0;
const MID_PEAK_HZ: f32 = 1_000.0;
const MID_Q: f32 = 0.8;
const HIGH_SHELF_HZ: f32 = 4_000.0;

pub struct ToneEq {
    /// Three sections per channel: [low, mid, high] × [L, R].
    sections: [[Biquad; 3]; 2],
}

impl ToneEq {
    pub fn new(sample_rate: f32, low_db: f32, mid_db: f32, high_db: f32) -> Self {
        let build = || {
            [
                Biquad::low_shelf(sample_rate, LOW_SHELF_HZ, low_db),
                Biquad::peaking(sample_rate, MID_PEAK_HZ, mid_db, MID_Q),
                Biquad::high_shelf(sample_rate, HIGH_SHELF_HZ, high_db),
            ]
        };
        Self {
            sections: [build(), build()],
        }
    }
}

impl FilterProc for ToneEq {
    fn process(&mut self, block: &mut [f32], _ctx: &FilterCtx) {
        for frame in block.chunks_exact_mut(2) {
            for (channel, sample) in frame.iter_mut().enumerate() {
                let mut value = *sample;
                for section in &mut self.sections[channel] {
                    value = section.process(value);
                }
                *sample = value;
            }
        }
    }

    fn update(&mut self, kind: &AudioFilterKind) -> bool {
        let AudioFilterKind::Eq {
            low_db,
            mid_db,
            high_db,
        } = kind
        else {
            return false;
        };
        let (low, mid, high) = (
            low_db.clamp(-20.0, 20.0),
            mid_db.clamp(-20.0, 20.0),
            high_db.clamp(-20.0, 20.0),
        );
        // Recompute each channel's sections in place — biquad memory carries,
        // so a live gain drag re-shapes without a click.
        for sections in &mut self.sections {
            sections[0].set_low_shelf(SR, LOW_SHELF_HZ, low);
            sections[1].set_peaking(SR, MID_PEAK_HZ, mid, MID_Q);
            sections[2].set_high_shelf(SR, HIGH_SHELF_HZ, high);
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsp::{db_to_lin, lin_to_db};

    const SR: f32 = 48_000.0;

    /// Feed a stereo sine through the EQ and measure the late amplitude.
    fn measure(eq: &mut ToneEq, freq: f32) -> f32 {
        let frames = 19_200;
        let mut block = Vec::with_capacity(frames * 2);
        for i in 0..frames {
            let sample = (2.0 * std::f32::consts::PI * freq * i as f32 / SR).sin();
            block.push(sample);
            block.push(sample);
        }
        eq.process(&mut block, &FilterCtx::empty());
        // Goertzel on the late left channel (transient settled).
        let left: Vec<f32> = block[frames..].iter().step_by(2).copied().collect();
        crate::dsp::goertzel(&left, SR, freq)
    }

    #[test]
    fn bands_shape_their_own_ranges() {
        let mut eq = ToneEq::new(SR, 9.0, 0.0, -9.0);
        let low = measure(&mut eq, 60.0);
        assert!(
            low > db_to_lin(7.5),
            "60 Hz should gain ~9 dB, got {} dB",
            lin_to_db(low)
        );
        let mut eq = ToneEq::new(SR, 9.0, 0.0, -9.0);
        let high = measure(&mut eq, 12_000.0);
        assert!(
            high < db_to_lin(-7.5),
            "12 kHz should lose ~9 dB, got {} dB",
            lin_to_db(high)
        );
        let mut eq = ToneEq::new(SR, 0.0, 6.0, 0.0);
        let mid = measure(&mut eq, 1_000.0);
        assert!(
            (lin_to_db(mid) - 6.0).abs() < 1.0,
            "1 kHz should gain ~6 dB, got {} dB",
            lin_to_db(mid)
        );
    }

    #[test]
    fn flat_eq_passes_through() {
        let mut eq = ToneEq::new(SR, 0.0, 0.0, 0.0);
        let amp = measure(&mut eq, 440.0);
        assert!((lin_to_db(amp)).abs() < 0.1, "got {} dB", lin_to_db(amp));
    }
}
