import { describe, expect, it } from "vitest";

import { findDuck } from "../lib/ducking";
import type { AudioFilter, Source } from "../api/types";

/** A source carrying just the audio filters the matrix reads. */
function target(filters: AudioFilter[]): Source {
  return { id: "t", name: "Music", kind: "audioOutput", audio: { filters } } as unknown as Source;
}

describe("findDuck", () => {
  it("finds the ducker keyed to a given trigger", () => {
    const filters = [
      { id: "f1", enabled: true, type: "gain", db: 0 },
      {
        id: "f2",
        enabled: true,
        type: "ducker",
        trigger: "mic-a",
        thresholdDb: -30,
        amountDb: 12,
        attackMs: 50,
        releaseMs: 300,
      },
      {
        id: "f3",
        enabled: true,
        type: "ducker",
        trigger: "mic-b",
        thresholdDb: -30,
        amountDb: 6,
        attackMs: 50,
        releaseMs: 300,
      },
    ] as unknown as AudioFilter[];
    expect(findDuck(target(filters), "mic-a" as unknown as Source["id"])?.id).toBe("f2");
    expect(findDuck(target(filters), "mic-b" as unknown as Source["id"])?.amountDb).toBe(6);
  });

  it("returns null when no duck targets that trigger", () => {
    const filters = [
      {
        id: "f1",
        enabled: true,
        type: "ducker",
        trigger: "mic-a",
        thresholdDb: -30,
        amountDb: 12,
        attackMs: 50,
        releaseMs: 300,
      },
    ] as unknown as AudioFilter[];
    expect(findDuck(target(filters), "mic-z" as unknown as Source["id"])).toBeNull();
  });

  it("ignores non-ducker filters and a source with no audio", () => {
    const noAudio = { id: "t", name: "X", kind: "audioOutput" } as unknown as Source;
    expect(findDuck(noAudio, "mic-a" as unknown as Source["id"])).toBeNull();
  });
});
