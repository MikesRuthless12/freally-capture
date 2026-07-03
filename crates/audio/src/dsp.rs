//! Small shared DSP primitives: dB conversion, one-pole smoothing, biquads.
//!
//! Everything here is classic textbook engineering (the biquad constructors
//! are the public RBJ Audio-EQ-Cookbook formulas) — owned code, no ML.

/// Decibels → linear amplitude.
pub fn db_to_lin(db: f32) -> f32 {
    10f32.powf(db / 20.0)
}

/// Linear amplitude → decibels (floored around -200 dB instead of -∞).
pub fn lin_to_db(lin: f32) -> f32 {
    20.0 * lin.max(1e-10).log10()
}

/// One-pole smoothing coefficient for a time constant in milliseconds:
/// `y += (x - y) * (1 - coef)` reaches ~63% of a step after `ms`.
pub fn one_pole_coef(ms: f32, sample_rate: f32) -> f32 {
    if ms <= 0.0 {
        0.0
    } else {
        (-1.0 / (ms * 0.001 * sample_rate)).exp()
    }
}

/// A single biquad section, transposed direct form II.
#[derive(Debug, Clone)]
pub struct Biquad {
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,
    z1: f32,
    z2: f32,
}

impl Biquad {
    /// From already-normalized coefficients (a0 = 1).
    pub fn from_coefficients(b0: f32, b1: f32, b2: f32, a1: f32, a2: f32) -> Self {
        Self {
            b0,
            b1,
            b2,
            a1,
            a2,
            z1: 0.0,
            z2: 0.0,
        }
    }

    /// Assign already-normalized coefficients (a0 = 1) **without touching the
    /// filter memory** (`z1`/`z2`) — the click-free path for a live parameter
    /// change: the transfer function updates while the running state carries.
    fn set_coefficients(&mut self, b0: f32, b1: f32, b2: f32, a1: f32, a2: f32) {
        self.b0 = b0;
        self.b1 = b1;
        self.b2 = b2;
        self.a1 = a1;
        self.a2 = a2;
    }

    /// RBJ low shelf (slope S = 1).
    pub fn low_shelf(sample_rate: f32, f0: f32, gain_db: f32) -> Self {
        let mut biquad = Self::from_coefficients(1.0, 0.0, 0.0, 0.0, 0.0);
        biquad.set_low_shelf(sample_rate, f0, gain_db);
        biquad
    }

    /// Recompute this section's low-shelf coefficients in place (keeps state).
    pub fn set_low_shelf(&mut self, sample_rate: f32, f0: f32, gain_db: f32) {
        let a = 10f32.powf(gain_db / 40.0);
        let w0 = 2.0 * std::f32::consts::PI * f0 / sample_rate;
        let (sin, cos) = w0.sin_cos();
        let alpha = sin / 2.0 * 2f32.sqrt(); // S = 1
        let sqrt_a2a = 2.0 * a.sqrt() * alpha;
        let b0 = a * ((a + 1.0) - (a - 1.0) * cos + sqrt_a2a);
        let b1 = 2.0 * a * ((a - 1.0) - (a + 1.0) * cos);
        let b2 = a * ((a + 1.0) - (a - 1.0) * cos - sqrt_a2a);
        let a0 = (a + 1.0) + (a - 1.0) * cos + sqrt_a2a;
        let a1 = -2.0 * ((a - 1.0) + (a + 1.0) * cos);
        let a2 = (a + 1.0) + (a - 1.0) * cos - sqrt_a2a;
        self.set_coefficients(b0 / a0, b1 / a0, b2 / a0, a1 / a0, a2 / a0);
    }

    /// RBJ peaking EQ.
    pub fn peaking(sample_rate: f32, f0: f32, gain_db: f32, q: f32) -> Self {
        let mut biquad = Self::from_coefficients(1.0, 0.0, 0.0, 0.0, 0.0);
        biquad.set_peaking(sample_rate, f0, gain_db, q);
        biquad
    }

    /// Recompute this section's peaking coefficients in place (keeps state).
    pub fn set_peaking(&mut self, sample_rate: f32, f0: f32, gain_db: f32, q: f32) {
        let a = 10f32.powf(gain_db / 40.0);
        let w0 = 2.0 * std::f32::consts::PI * f0 / sample_rate;
        let (sin, cos) = w0.sin_cos();
        let alpha = sin / (2.0 * q);
        let b0 = 1.0 + alpha * a;
        let b1 = -2.0 * cos;
        let b2 = 1.0 - alpha * a;
        let a0 = 1.0 + alpha / a;
        let a1 = -2.0 * cos;
        let a2 = 1.0 - alpha / a;
        self.set_coefficients(b0 / a0, b1 / a0, b2 / a0, a1 / a0, a2 / a0);
    }

    /// RBJ high shelf (slope S = 1).
    pub fn high_shelf(sample_rate: f32, f0: f32, gain_db: f32) -> Self {
        let mut biquad = Self::from_coefficients(1.0, 0.0, 0.0, 0.0, 0.0);
        biquad.set_high_shelf(sample_rate, f0, gain_db);
        biquad
    }

    /// Recompute this section's high-shelf coefficients in place (keeps state).
    pub fn set_high_shelf(&mut self, sample_rate: f32, f0: f32, gain_db: f32) {
        let a = 10f32.powf(gain_db / 40.0);
        let w0 = 2.0 * std::f32::consts::PI * f0 / sample_rate;
        let (sin, cos) = w0.sin_cos();
        let alpha = sin / 2.0 * 2f32.sqrt(); // S = 1
        let sqrt_a2a = 2.0 * a.sqrt() * alpha;
        let b0 = a * ((a + 1.0) + (a - 1.0) * cos + sqrt_a2a);
        let b1 = -2.0 * a * ((a - 1.0) + (a + 1.0) * cos);
        let b2 = a * ((a + 1.0) + (a - 1.0) * cos - sqrt_a2a);
        let a0 = (a + 1.0) - (a - 1.0) * cos + sqrt_a2a;
        let a1 = 2.0 * ((a - 1.0) - (a + 1.0) * cos);
        let a2 = (a + 1.0) - (a - 1.0) * cos - sqrt_a2a;
        self.set_coefficients(b0 / a0, b1 / a0, b2 / a0, a1 / a0, a2 / a0);
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let y = self.b0 * x + self.z1;
        self.z1 = self.b1 * x - self.a1 * y + self.z2;
        self.z2 = self.b2 * x - self.a2 * y;
        y
    }
}

/// Measure one frequency's amplitude via the Goertzel algorithm — the
/// measurement workhorse shared by this crate's tests.
#[cfg(test)]
pub(crate) fn goertzel(samples: &[f32], sample_rate: f32, freq: f32) -> f32 {
    let n = samples.len();
    let k = (freq * n as f32 / sample_rate).round();
    let w = 2.0 * std::f32::consts::PI * k / n as f32;
    let coef = 2.0 * w.cos();
    let (mut s1, mut s2) = (0.0f32, 0.0f32);
    for &x in samples {
        let s0 = x + coef * s1 - s2;
        s2 = s1;
        s1 = s0;
    }
    let power = s1 * s1 + s2 * s2 - coef * s1 * s2;
    2.0 * (power.max(0.0)).sqrt() / n as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn db_round_trips() {
        for db in [-60.0, -20.0, -6.0, 0.0, 6.0] {
            assert!((lin_to_db(db_to_lin(db)) - db).abs() < 1e-3);
        }
        assert_eq!(db_to_lin(0.0), 1.0);
    }

    #[test]
    fn unity_shelves_pass_through() {
        // gain 0 dB → A = 1 → the transfer function is exactly 1.
        let mut low = Biquad::low_shelf(48_000.0, 100.0, 0.0);
        let mut peak = Biquad::peaking(48_000.0, 1_000.0, 0.0, 0.8);
        let mut high = Biquad::high_shelf(48_000.0, 6_000.0, 0.0);
        for i in 0..256 {
            let x = ((i * 7919) % 101) as f32 / 50.0 - 1.0;
            for filter in [&mut low, &mut peak, &mut high] {
                assert!((filter.process(x) - x).abs() < 1e-4);
            }
        }
    }

    #[test]
    fn low_shelf_boosts_lows_leaves_highs() {
        let sr = 48_000.0;
        let n = 9_600;
        let run = |freq: f32| -> f32 {
            let mut filter = Biquad::low_shelf(sr, 250.0, 12.0);
            let samples: Vec<f32> = (0..n)
                .map(|i| (2.0 * std::f32::consts::PI * freq * i as f32 / sr).sin())
                .map(|x| filter.process(x))
                .collect();
            // Skip the transient before measuring.
            goertzel(&samples[n / 2..], sr, freq)
        };
        let low = run(50.0);
        let high = run(8_000.0);
        assert!(low > db_to_lin(10.0), "50 Hz should gain ~12 dB, got {low}");
        assert!(
            (lin_to_db(high)).abs() < 1.0,
            "8 kHz should stay ~unity, got {} dB",
            lin_to_db(high)
        );
    }
}
