import { describe, expect, it } from "vitest";

import { formatTimecode, landsOnKeyframe, nearestKeyframe, stepFrames } from "../trim";

describe("landsOnKeyframe (mirrors trim.rs lands_on_keyframe)", () => {
  const kfs = [0, 2, 4];
  it("accepts exact keyframes and within half a frame", () => {
    expect(landsOnKeyframe(2, kfs, 60)).toBe(true);
    expect(landsOnKeyframe(2.008, kfs, 60)).toBe(true);
    expect(landsOnKeyframe(0, kfs, 60)).toBe(true);
  });
  it("rejects positions between keyframes", () => {
    expect(landsOnKeyframe(2.02, kfs, 60)).toBe(false);
    expect(landsOnKeyframe(1, kfs, 60)).toBe(false);
    expect(landsOnKeyframe(5, [], 60)).toBe(false);
  });
});

describe("nearestKeyframe", () => {
  it("finds the closest on either side", () => {
    expect(nearestKeyframe(2.9, [0, 2, 4])).toBe(2);
    expect(nearestKeyframe(3.1, [0, 2, 4])).toBe(4);
    expect(nearestKeyframe(99, [0, 2, 4])).toBe(4);
    expect(nearestKeyframe(1, [])).toBe(null);
  });
});

describe("stepFrames", () => {
  it("steps by whole frames and clamps to the file", () => {
    expect(stepFrames(1, 1, 50, 10)).toBeCloseTo(1.02);
    expect(stepFrames(1, -1, 50, 10)).toBeCloseTo(0.98);
    expect(stepFrames(0, -5, 50, 10)).toBe(0);
    expect(stepFrames(9.999, 5, 50, 10)).toBe(10);
  });
});

describe("formatTimecode", () => {
  it("renders HH:MM:SS.ff", () => {
    expect(formatTimecode(0, 60)).toBe("00:00:00.00");
    expect(formatTimecode(3661.5, 60)).toBe("01:01:01.30");
    expect(formatTimecode(59.983, 60)).toBe("00:00:59.59");
  });
});
