import { describe, expect, it } from "vitest";

import { eqCurveDb, magnitudeDb, bandCoeffs } from "../lib/eqResponse";
import type { EqBand } from "../api/types";

const SR = 48000;

describe("eqResponse", () => {
  it("a bell boost peaks at its own frequency", () => {
    const band: EqBand = { type: "bell", freqHz: 1000, gainDb: 6, q: 1 };
    const at = magnitudeDb(bandCoeffs(band, SR), 1000, SR);
    expect(at).toBeCloseTo(6, 0);
    const away = magnitudeDb(bandCoeffs(band, SR), 60, SR);
    expect(Math.abs(away)).toBeLessThan(1);
  });

  it("a high-pass cuts the lows and passes the highs", () => {
    const band: EqBand = { type: "highPass", freqHz: 300, gainDb: 0, q: 0.707 };
    expect(magnitudeDb(bandCoeffs(band, SR), 50, SR)).toBeLessThan(-9);
    expect(Math.abs(magnitudeDb(bandCoeffs(band, SR), 4000, SR))).toBeLessThan(1);
  });

  it("bands sum, and an empty EQ is flat", () => {
    const bands: EqBand[] = [
      { type: "bell", freqHz: 100, gainDb: 4, q: 1 },
      { type: "bell", freqHz: 100, gainDb: 4, q: 1 },
    ];
    // Two identical +4 dB bells ≈ +8 dB at 100 Hz.
    expect(eqCurveDb(bands, [100], SR)[0]).toBeCloseTo(8, 0);
    expect(eqCurveDb([], [1000], SR)[0]).toBe(0);
  });
});
