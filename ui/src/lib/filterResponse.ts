import type { AudioFilter, AudioFilterKind, EqBand } from "../api/types";
import { eqCurveDb } from "./eqResponse";

/**
 * Per-filter graphics math for the plugin showpiece: a **transfer curve**
 * (input→output dB) for dynamics, a **frequency response** for tone filters, or
 * a plain level readout for the rest. All computed from the filter's own
 * parameters — the same values the engine runs — so the drawn shape matches.
 */

export type VizKind = "transfer" | "frequency" | "level";
export type FilterType = AudioFilterKind["type"];

/** Which graphic a filter type gets. Parametric EQ keeps its own editor. */
export function vizKind(type: FilterType): VizKind {
  switch (type) {
    case "compressor":
    case "limiter":
    case "noiseGate":
      return "transfer";
    case "eq":
    case "deEsser":
    case "rumbleGuard":
      return "frequency";
    default:
      return "level";
  }
}

/** The transfer plot's input-dB floor. */
export const TRANSFER_MIN_DB = -60;

/** Output dB for an input dB through a dynamics filter's static curve. */
export function transferOut(filter: AudioFilter, inDb: number): number {
  switch (filter.type) {
    case "compressor": {
      const compressed =
        inDb <= filter.thresholdDb
          ? inDb
          : filter.thresholdDb + (inDb - filter.thresholdDb) / filter.ratio;
      return compressed + filter.outputGainDb;
    }
    case "limiter":
      return Math.min(inDb, filter.thresholdDb);
    case "noiseGate":
      // Below the close threshold the gate shuts (floor); above it, pass.
      return inDb >= filter.closeThresholdDb ? inDb : TRANSFER_MIN_DB;
    default:
      return inDb;
  }
}

/** Sampled `{ inDb, outDb }` transfer points across the input range. */
export function transferCurve(filter: AudioFilter, steps = 48): { inDb: number; outDb: number }[] {
  const points: { inDb: number; outDb: number }[] = [];
  for (let i = 0; i <= steps; i++) {
    const inDb = TRANSFER_MIN_DB + (i / steps) * -TRANSFER_MIN_DB;
    points.push({ inDb, outDb: transferOut(filter, inDb) });
  }
  return points;
}

/** The EQ bands that describe a tone filter's frequency response. */
function filterBands(filter: AudioFilter): EqBand[] {
  switch (filter.type) {
    case "eq":
      return [
        { type: "lowShelf", freqHz: 250, gainDb: filter.lowDb, q: 0.707 },
        { type: "bell", freqHz: 1000, gainDb: filter.midDb, q: 0.8 },
        { type: "highShelf", freqHz: 4000, gainDb: filter.highDb, q: 0.707 },
      ];
    case "deEsser":
      // The max reduction, drawn as a high-shelf cut at the crossover.
      return [{ type: "highShelf", freqHz: filter.freqHz, gainDb: -filter.amountDb, q: 0.707 }];
    case "rumbleGuard":
      return [{ type: "highPass", freqHz: filter.freqHz, gainDb: 0, q: 0.707 }];
    default:
      return [];
  }
}

/** Total frequency response (dB) of a tone filter at each frequency in `freqs`. */
export function frequencyCurve(filter: AudioFilter, freqs: number[], sampleRate = 48000): number[] {
  return eqCurveDb(filterBands(filter), freqs, sampleRate);
}

/** Linear peak → dBFS (floored). */
export function toDb(linear: number): number {
  return 20 * Math.log10(Math.max(1e-5, linear));
}
