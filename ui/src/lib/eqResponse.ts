import type { EqBand } from "../api/types";

/**
 * The parametric-EQ frequency response, mirrored in TypeScript for the editor
 * curve (CAP-N35). These are the same RBJ Audio-EQ-Cookbook formulas the Rust
 * `Biquad` uses in `crates/audio/src/dsp.rs` — kept in lock-step so the drawn
 * curve matches what the engine actually renders.
 */

export type Biquad = { b0: number; b1: number; b2: number; a1: number; a2: number };

const clampFreq = (f: number) => Math.min(20000, Math.max(20, f));
const clampGain = (g: number) => Math.min(30, Math.max(-30, g));
const clampQ = (q: number) => Math.min(18, Math.max(0.1, q));

/** RBJ coefficients (a0-normalized) for one band at `sampleRate`. */
export function bandCoeffs(band: EqBand, sampleRate: number): Biquad {
  const f0 = clampFreq(band.freqHz);
  const gain = clampGain(band.gainDb);
  const q = clampQ(band.q);
  const w0 = (2 * Math.PI * f0) / sampleRate;
  const cos = Math.cos(w0);
  const sin = Math.sin(w0);
  const norm = (
    b0: number,
    b1: number,
    b2: number,
    a0: number,
    a1: number,
    a2: number,
  ): Biquad => ({
    b0: b0 / a0,
    b1: b1 / a0,
    b2: b2 / a0,
    a1: a1 / a0,
    a2: a2 / a0,
  });

  switch (band.type) {
    case "bell": {
      const a = Math.pow(10, gain / 40);
      const alpha = sin / (2 * q);
      return norm(1 + alpha * a, -2 * cos, 1 - alpha * a, 1 + alpha / a, -2 * cos, 1 - alpha / a);
    }
    case "lowShelf": {
      const a = Math.pow(10, gain / 40);
      const alpha = (sin / 2) * Math.SQRT2;
      const s = 2 * Math.sqrt(a) * alpha;
      return norm(
        a * (a + 1 - (a - 1) * cos + s),
        2 * a * (a - 1 - (a + 1) * cos),
        a * (a + 1 - (a - 1) * cos - s),
        a + 1 + (a - 1) * cos + s,
        -2 * (a - 1 + (a + 1) * cos),
        a + 1 + (a - 1) * cos - s,
      );
    }
    case "highShelf": {
      const a = Math.pow(10, gain / 40);
      const alpha = (sin / 2) * Math.SQRT2;
      const s = 2 * Math.sqrt(a) * alpha;
      return norm(
        a * (a + 1 + (a - 1) * cos + s),
        -2 * a * (a - 1 + (a + 1) * cos),
        a * (a + 1 + (a - 1) * cos - s),
        a + 1 - (a - 1) * cos + s,
        2 * (a - 1 - (a + 1) * cos),
        a + 1 - (a - 1) * cos - s,
      );
    }
    case "notch": {
      const alpha = sin / (2 * q);
      return norm(1, -2 * cos, 1, 1 + alpha, -2 * cos, 1 - alpha);
    }
    case "highPass": {
      const alpha = sin / (2 * q);
      const b0 = (1 + cos) / 2;
      return norm(b0, -(1 + cos), b0, 1 + alpha, -2 * cos, 1 - alpha);
    }
    case "lowPass": {
      const alpha = sin / (2 * q);
      const b0 = (1 - cos) / 2;
      return norm(b0, 1 - cos, b0, 1 + alpha, -2 * cos, 1 - alpha);
    }
  }
}

/** |H(e^jw)| of one biquad at frequency `f`, in dB. */
export function magnitudeDb(c: Biquad, f: number, sampleRate: number): number {
  const w = (2 * Math.PI * f) / sampleRate;
  const cos1 = Math.cos(w);
  const sin1 = Math.sin(w);
  const cos2 = Math.cos(2 * w);
  const sin2 = Math.sin(2 * w);
  const numRe = c.b0 + c.b1 * cos1 + c.b2 * cos2;
  const numIm = -(c.b1 * sin1 + c.b2 * sin2);
  const denRe = 1 + c.a1 * cos1 + c.a2 * cos2;
  const denIm = -(c.a1 * sin1 + c.a2 * sin2);
  const num = Math.hypot(numRe, numIm);
  const den = Math.hypot(denRe, denIm) || 1e-9;
  return 20 * Math.log10(Math.max(1e-9, num / den));
}

/** Total EQ response (sum of the bands' dB) at each frequency in `freqs`. */
export function eqCurveDb(bands: EqBand[], freqs: number[], sampleRate: number): number[] {
  const coeffs = bands.map((band) => bandCoeffs(band, sampleRate));
  return freqs.map((f) => coeffs.reduce((sum, c) => sum + magnitudeDb(c, f, sampleRate), 0));
}
