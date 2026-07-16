//! CAP-N35 spectrum tap: a rolling FFT magnitude spectrum for one armed source,
//! feeding the parametric-EQ editor's live analyzer. Owned radix-2 FFT, no deps.
//!
//! Only one source is analyzed at a time — the strip whose EQ editor is open —
//! so this costs nothing until armed. The UI maps bin `i` to frequency with the
//! same log spacing declared here ([`F_MIN`]..[`F_MAX`], [`SPECTRUM_BINS`] bins).

use crate::fft::fft_in_place;
use crate::SAMPLE_RATE;

const FFT_SIZE: usize = 2048;
/// How many log-spaced bins the analyzer reports to the UI.
pub const SPECTRUM_BINS: usize = 48;
/// The analyzer's frequency range (the editor plots this exact span).
pub const F_MIN: f32 = 30.0;
pub const F_MAX: f32 = 16_000.0;

/// A rolling windowed-FFT magnitude analyzer for one source.
pub struct SpectrumAnalyzer {
    ring: Vec<f32>,
    head: usize,
    window: Vec<f32>,
    re: Vec<f32>,
    im: Vec<f32>,
}

impl Default for SpectrumAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl SpectrumAnalyzer {
    pub fn new() -> Self {
        let window = (0..FFT_SIZE)
            .map(|i| {
                // Hann.
                let x = std::f32::consts::PI * i as f32 / (FFT_SIZE as f32 - 1.0);
                x.sin() * x.sin()
            })
            .collect();
        Self {
            ring: vec![0.0; FFT_SIZE],
            head: 0,
            window,
            re: vec![0.0; FFT_SIZE],
            im: vec![0.0; FFT_SIZE],
        }
    }

    /// Forget the accumulated history (on a fresh arm/re-arm).
    pub fn clear(&mut self) {
        self.ring.iter_mut().for_each(|s| *s = 0.0);
        self.head = 0;
    }

    /// Append an interleaved stereo block (down-mixed to mono), keeping only the
    /// last [`FFT_SIZE`] samples in the ring.
    pub fn push(&mut self, block: &[f32]) {
        for frame in block.chunks_exact(2) {
            self.ring[self.head] = (frame[0] + frame[1]) * 0.5;
            self.head = (self.head + 1) % FFT_SIZE;
        }
    }

    /// The current spectrum as [`SPECTRUM_BINS`] log-spaced points in dBFS
    /// (clamped to a display range).
    pub fn magnitudes(&mut self) -> Vec<f32> {
        for i in 0..FFT_SIZE {
            self.re[i] = self.ring[(self.head + i) % FFT_SIZE] * self.window[i];
            self.im[i] = 0.0;
        }
        fft_in_place(&mut self.re, &mut self.im, false);
        // Coherent-gain normalization for the Hann window (its mean is 0.5), so
        // a full-scale sine reads ~0 dBFS.
        let norm = 2.0 / (FFT_SIZE as f32 * 0.5);
        let hz_per_bin = SAMPLE_RATE as f32 / FFT_SIZE as f32;
        let step = 1.0 / (SPECTRUM_BINS as f32 - 1.0);
        let ui_freq = |t: f32| F_MIN * (F_MAX / F_MIN).powf(t);
        (0..SPECTRUM_BINS)
            .map(|i| {
                // Take the loudest FFT bin inside this log band (between the
                // midpoints to its neighbors) so a peak between bins isn't
                // missed by exact-frequency sampling.
                let center = i as f32 * step;
                let f_lo = ui_freq((center - step * 0.5).max(0.0));
                let f_hi = ui_freq((center + step * 0.5).min(1.0));
                let k_lo = ((f_lo / hz_per_bin).floor() as usize).clamp(1, FFT_SIZE / 2 - 1);
                let k_hi = ((f_hi / hz_per_bin).ceil() as usize).clamp(k_lo, FFT_SIZE / 2 - 1);
                let mut peak = 0.0f32;
                for k in k_lo..=k_hi {
                    peak = peak.max((self.re[k] * self.re[k] + self.im[k] * self.im[k]).sqrt());
                }
                (20.0 * (peak * norm).max(1e-9).log10()).clamp(-90.0, 6.0)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_tone_peaks_in_its_own_bin() {
        let mut analyzer = SpectrumAnalyzer::new();
        // Fill the ring with a 1 kHz stereo tone.
        let tone: Vec<f32> = (0..FFT_SIZE)
            .flat_map(|i| {
                let s =
                    (2.0 * std::f32::consts::PI * 1_000.0 * i as f32 / SAMPLE_RATE as f32).sin();
                [s, s]
            })
            .collect();
        analyzer.push(&tone);
        let bins = analyzer.magnitudes();
        assert_eq!(bins.len(), SPECTRUM_BINS);

        let bin_freq =
            |i: usize| F_MIN * (F_MAX / F_MIN).powf(i as f32 / (SPECTRUM_BINS - 1) as f32);
        // The loudest bin should sit at ~1 kHz — the tone peaks in its own bin.
        let argmax = (0..SPECTRUM_BINS)
            .max_by(|&a, &b| bins[a].partial_cmp(&bins[b]).unwrap())
            .unwrap();
        let peak_freq = bin_freq(argmax);
        assert!(
            (700.0..=1_400.0).contains(&peak_freq),
            "the loudest bin ({peak_freq:.0} Hz, {} dB) should be ~1 kHz",
            bins[argmax]
        );
        assert!(
            bins[argmax] > -12.0,
            "the tone bin is loud, got {}",
            bins[argmax]
        );
        // ...and it towers over the low end (well away from 1 kHz).
        assert!(
            bins[argmax] > bins[1] + 20.0,
            "peak {} dB should dominate the ~34 Hz bin {} dB",
            bins[argmax],
            bins[1]
        );
    }

    #[test]
    fn silence_reads_the_floor() {
        let mut analyzer = SpectrumAnalyzer::new();
        analyzer.push(&vec![0.0; FFT_SIZE * 2]);
        assert!(analyzer.magnitudes().iter().all(|&db| db <= -80.0));
    }
}
