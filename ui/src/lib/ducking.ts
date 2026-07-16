import type { AudioFilter, AudioFilterKind, Source, SourceId } from "../api/types";

/** One ducker in a chain, narrowed to the ducker variant. */
export type Ducker = AudioFilter & Extract<AudioFilterKind, { type: "ducker" }>;

/** The default a matrix-created duck starts from (mirrors the filter dialog). */
export const DUCKER_DEFAULT: Extract<AudioFilterKind, { type: "ducker" }> = {
  type: "ducker",
  trigger: null,
  thresholdDb: -30,
  amountDb: 12,
  attackMs: 50,
  releaseMs: 300,
};

/**
 * Find the duck on `target` whose trigger is `triggerId` (one matrix cell).
 * The CAP-N31 matrix is entirely a view over per-strip Ducker filters, so the
 * whole feature turns on this lookup being right — hence its own tested module.
 */
export function findDuck(target: Source, triggerId: SourceId): Ducker | null {
  const filters = target.audio?.filters ?? [];
  for (const filter of filters) {
    if (filter.type === "ducker" && filter.trigger === triggerId) {
      return filter as Ducker;
    }
  }
  return null;
}

/** The `AudioFilterKind` payload for a duck (drops the instance id/enabled). */
export function duckKind(duck: Ducker): AudioFilterKind {
  return {
    type: "ducker",
    trigger: duck.trigger,
    thresholdDb: duck.thresholdDb,
    amountDb: duck.amountDb,
    attackMs: duck.attackMs,
    releaseMs: duck.releaseMs,
  };
}
