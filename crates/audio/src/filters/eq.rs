//! The three-band tone EQ: low shelf (250 Hz), mid peak (1 kHz), high shelf
//! (4 kHz) — RBJ biquads per channel. Plus the CAP-N35 N-band parametric EQ.

use fcap_scene::{AudioFilterKind, EqBand, EqBandType};

use crate::dsp::Biquad;
use crate::filters::{FilterCtx, FilterProc};

const SR: f32 = crate::SAMPLE_RATE as f32;

/// The most bands a parametric EQ builds (a hand-edited file can't grow it).
const MAX_EQ_BANDS: usize = 16;

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

/// Pull one band's parameters inside their documented ranges.
fn clamp_band(band: &EqBand) -> (f32, f32, f32) {
    (
        band.freq_hz.clamp(20.0, 20_000.0),
        band.gain_db.clamp(-30.0, 30.0),
        band.q.clamp(0.1, 18.0),
    )
}

/// Recompute a biquad in place for one band's shape (keeps its state, so a live
/// drag re-shapes click-free even across a band-type change).
fn set_band(biquad: &mut Biquad, band: &EqBand, sample_rate: f32) {
    let (freq, gain, q) = clamp_band(band);
    match band.kind {
        EqBandType::Bell => biquad.set_peaking(sample_rate, freq, gain, q),
        EqBandType::LowShelf => biquad.set_low_shelf(sample_rate, freq, gain),
        EqBandType::HighShelf => biquad.set_high_shelf(sample_rate, freq, gain),
        EqBandType::Notch => biquad.set_notch(sample_rate, freq, q),
        EqBandType::HighPass => biquad.set_high_pass(sample_rate, freq, q),
        EqBandType::LowPass => biquad.set_low_pass(sample_rate, freq, q),
    }
}

fn band_biquad(band: &EqBand, sample_rate: f32) -> Biquad {
    let mut biquad = Biquad::from_coefficients(1.0, 0.0, 0.0, 0.0, 0.0);
    set_band(&mut biquad, band, sample_rate);
    biquad
}

/// Owned N-band parametric EQ (CAP-N35): a cascade of one biquad per band, per
/// channel. Bell / shelf / notch / high-pass / low-pass, all RBJ.
pub struct ParametricEq {
    sample_rate: f32,
    /// One `[L, R]` biquad pair per band, in band order.
    bands: Vec<[Biquad; 2]>,
}

impl ParametricEq {
    pub fn new(sample_rate: f32, bands: &[EqBand]) -> Self {
        let bands = bands
            .iter()
            .take(MAX_EQ_BANDS)
            .map(|band| {
                [
                    band_biquad(band, sample_rate),
                    band_biquad(band, sample_rate),
                ]
            })
            .collect();
        Self { sample_rate, bands }
    }
}

impl FilterProc for ParametricEq {
    fn process(&mut self, block: &mut [f32], _ctx: &FilterCtx) {
        for frame in block.chunks_exact_mut(2) {
            for (channel, sample) in frame.iter_mut().enumerate() {
                let mut value = *sample;
                for filters in &mut self.bands {
                    value = filters[channel].process(value);
                }
                *sample = value;
            }
        }
    }

    fn update(&mut self, kind: &AudioFilterKind) -> bool {
        let AudioFilterKind::ParametricEq { bands } = kind else {
            return false;
        };
        let bands: Vec<&EqBand> = bands.iter().take(MAX_EQ_BANDS).collect();
        // A band added or removed changes the cascade length — rebuild. A pure
        // parameter (or band-type) edit at the same count updates in place.
        if bands.len() != self.bands.len() {
            return false;
        }
        for (filters, band) in self.bands.iter_mut().zip(&bands) {
            set_band(&mut filters[0], band, self.sample_rate);
            set_band(&mut filters[1], band, self.sample_rate);
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

    /// Feed a stereo sine through any filter and measure the late amplitude.
    fn measure_proc(proc: &mut dyn FilterProc, freq: f32) -> f32 {
        let frames = 19_200;
        let mut block = Vec::with_capacity(frames * 2);
        for i in 0..frames {
            let sample = (2.0 * std::f32::consts::PI * freq * i as f32 / SR).sin();
            block.push(sample);
            block.push(sample);
        }
        proc.process(&mut block, &FilterCtx::empty());
        let left: Vec<f32> = block[frames..].iter().step_by(2).copied().collect();
        crate::dsp::goertzel(&left, SR, freq)
    }

    #[test]
    fn parametric_bell_boosts_its_band() {
        let bands = [EqBand {
            kind: EqBandType::Bell,
            freq_hz: 1_000.0,
            gain_db: 9.0,
            q: 1.0,
        }];
        let mut eq = ParametricEq::new(SR, &bands);
        let at = measure_proc(&mut eq, 1_000.0);
        assert!(
            (lin_to_db(at) - 9.0).abs() < 1.0,
            "1 kHz should gain ~9 dB, got {} dB",
            lin_to_db(at)
        );
        // A distant frequency is untouched.
        let mut eq = ParametricEq::new(SR, &bands);
        let away = measure_proc(&mut eq, 60.0);
        assert!(lin_to_db(away).abs() < 1.0, "60 Hz unchanged");
    }

    #[test]
    fn parametric_high_pass_cuts_the_lows() {
        let bands = [EqBand {
            kind: EqBandType::HighPass,
            freq_hz: 300.0,
            gain_db: 0.0,
            q: 0.707,
        }];
        let mut eq = ParametricEq::new(SR, &bands);
        let low = measure_proc(&mut eq, 60.0);
        assert!(
            lin_to_db(low) < -9.0,
            "60 Hz well below a 300 Hz corner should be cut hard, got {} dB",
            lin_to_db(low)
        );
        let mut eq = ParametricEq::new(SR, &bands);
        let high = measure_proc(&mut eq, 4_000.0);
        assert!(lin_to_db(high).abs() < 1.0, "4 kHz passes");
    }

    #[test]
    fn empty_parametric_eq_passes_through() {
        let mut eq = ParametricEq::new(SR, &[]);
        let amp = measure_proc(&mut eq, 440.0);
        assert!(lin_to_db(amp).abs() < 0.01, "no bands = unity");
    }

    #[test]
    fn parametric_update_in_place_when_band_count_matches() {
        let flat = [EqBand {
            kind: EqBandType::Bell,
            freq_hz: 1_000.0,
            gain_db: 0.0,
            q: 1.0,
        }];
        let mut eq = ParametricEq::new(SR, &flat);
        // Same count → an in-place update (returns true), reshaping the curve.
        let boosted = AudioFilterKind::ParametricEq {
            bands: vec![EqBand {
                kind: EqBandType::Bell,
                freq_hz: 1_000.0,
                gain_db: 9.0,
                q: 1.0,
            }],
        };
        assert!(eq.update(&boosted), "same count updates in place");
        let at = measure_proc(&mut eq, 1_000.0);
        assert!((lin_to_db(at) - 9.0).abs() < 1.0, "boost applied");

        // A different band count forces a rebuild (returns false).
        let two = AudioFilterKind::ParametricEq {
            bands: vec![flat[0], flat[0]],
        };
        assert!(!eq.update(&two), "band count change rebuilds");
    }
}
