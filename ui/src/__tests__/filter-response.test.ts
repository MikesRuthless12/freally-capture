import { describe, expect, it } from "vitest";

import { frequencyCurve, transferOut, vizKind } from "../lib/filterResponse";
import type { AudioFilter } from "../api/types";

const f = (kind: object): AudioFilter => ({ id: "x", enabled: true, ...kind }) as AudioFilter;

describe("filterResponse", () => {
  it("routes each filter to the right graphic", () => {
    expect(vizKind("compressor")).toBe("transfer");
    expect(vizKind("limiter")).toBe("transfer");
    expect(vizKind("noiseGate")).toBe("transfer");
    expect(vizKind("eq")).toBe("frequency");
    expect(vizKind("deEsser")).toBe("frequency");
    expect(vizKind("rumbleGuard")).toBe("frequency");
    expect(vizKind("gain")).toBe("level");
    expect(vizKind("ducker")).toBe("level");
  });

  it("compressor transfer bends above the threshold", () => {
    const comp = f({
      type: "compressor",
      ratio: 4,
      thresholdDb: -20,
      attackMs: 6,
      releaseMs: 60,
      outputGainDb: 0,
    });
    // Below threshold: unity.
    expect(transferOut(comp, -30)).toBeCloseTo(-30, 5);
    // 20 dB over → 20/4 = 5 dB over threshold = -15.
    expect(transferOut(comp, 0)).toBeCloseTo(-15, 5);
  });

  it("limiter clamps at the ceiling", () => {
    const lim = f({ type: "limiter", thresholdDb: -3, releaseMs: 60 });
    expect(transferOut(lim, -20)).toBeCloseTo(-20, 5);
    expect(transferOut(lim, 0)).toBeCloseTo(-3, 5);
  });

  it("EQ frequency curve boosts the low shelf", () => {
    const eq = f({ type: "eq", lowDb: 9, midDb: 0, highDb: 0 });
    const [low, high] = frequencyCurve(eq, [50, 12000]);
    expect(low).toBeGreaterThan(6);
    expect(Math.abs(high)).toBeLessThan(1);
  });
});
