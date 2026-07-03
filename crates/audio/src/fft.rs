//! An owned radix-2 FFT (iterative Cooley–Tukey).
//!
//! Sized for the denoiser's 512-point frames — tiny, dependency-free, and
//! plenty fast at audio rates (a 512-point transform is ~2 µs). Public-domain
//! math, authored here.

/// In-place complex FFT over `re`/`im` (equal power-of-two lengths).
/// `inverse` applies the conjugate transform and the 1/N scale.
pub fn fft_in_place(re: &mut [f32], im: &mut [f32], inverse: bool) {
    let n = re.len();
    assert_eq!(n, im.len(), "re/im must match");
    assert!(n.is_power_of_two(), "FFT length must be a power of two");
    if n <= 1 {
        return;
    }

    // Bit-reversal permutation.
    let bits = n.trailing_zeros();
    for i in 0..n {
        let j = i.reverse_bits() >> (usize::BITS - bits);
        if j > i {
            re.swap(i, j);
            im.swap(i, j);
        }
    }

    // Butterflies, stage by stage. Twiddles in f64 for accuracy.
    let sign = if inverse { 1.0f64 } else { -1.0f64 };
    let mut len = 2;
    while len <= n {
        let angle = sign * 2.0 * std::f64::consts::PI / len as f64;
        let (w_im, w_re) = angle.sin_cos();
        let mut start = 0;
        while start < n {
            let mut cur_re = 1.0f64;
            let mut cur_im = 0.0f64;
            for k in 0..len / 2 {
                let a = start + k;
                let b = a + len / 2;
                let t_re = cur_re as f32 * re[b] - cur_im as f32 * im[b];
                let t_im = cur_re as f32 * im[b] + cur_im as f32 * re[b];
                re[b] = re[a] - t_re;
                im[b] = im[a] - t_im;
                re[a] += t_re;
                im[a] += t_im;
                let next_re = cur_re * w_re - cur_im * w_im;
                cur_im = cur_re * w_im + cur_im * w_re;
                cur_re = next_re;
            }
            start += len;
        }
        len <<= 1;
    }

    if inverse {
        let scale = 1.0 / n as f32;
        for value in re.iter_mut().chain(im.iter_mut()) {
            *value *= scale;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn impulse_transforms_flat() {
        let mut re = vec![0.0f32; 64];
        let mut im = vec![0.0f32; 64];
        re[0] = 1.0;
        fft_in_place(&mut re, &mut im, false);
        for k in 0..64 {
            assert!((re[k] - 1.0).abs() < 1e-5 && im[k].abs() < 1e-5, "bin {k}");
        }
    }

    #[test]
    fn sine_lands_in_its_bin() {
        let n = 512;
        let bin = 7;
        let mut re: Vec<f32> = (0..n)
            .map(|i| (2.0 * std::f32::consts::PI * bin as f32 * i as f32 / n as f32).sin())
            .collect();
        let mut im = vec![0.0f32; n];
        fft_in_place(&mut re, &mut im, false);
        let mag = |k: usize| ((re[k] * re[k] + im[k] * im[k]) as f64).sqrt() / (n as f64 / 2.0);
        assert!((mag(bin) - 1.0).abs() < 1e-3, "bin {bin}: {}", mag(bin));
        for k in 0..n / 2 {
            if k != bin {
                assert!(mag(k) < 1e-3, "leakage at bin {k}: {}", mag(k));
            }
        }
    }

    #[test]
    fn forward_inverse_round_trips() {
        let n = 512;
        // Deterministic pseudo-noise (no rand dependency).
        let mut state = 0x2545F491u32;
        let signal: Vec<f32> = (0..n)
            .map(|_| {
                state = state.wrapping_mul(1664525).wrapping_add(1013904223);
                (state >> 8) as f32 / (1 << 24) as f32 * 2.0 - 1.0
            })
            .collect();
        let mut re = signal.clone();
        let mut im = vec![0.0f32; n];
        fft_in_place(&mut re, &mut im, false);
        fft_in_place(&mut re, &mut im, true);
        for i in 0..n {
            assert!((re[i] - signal[i]).abs() < 1e-4, "sample {i}");
            assert!(im[i].abs() < 1e-4, "imag {i}");
        }
    }
}
