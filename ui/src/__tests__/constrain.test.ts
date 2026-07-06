import { describe, expect, it } from "vitest";

import type { Transform } from "../api/types";
import {
  boundsOf,
  clampMoveAgainstObstacles,
  clampMoveDelta,
  clampScalesToCanvas,
  edgeDistances,
  intersects,
  slideIntoCanvas,
  type Box,
} from "../lib/constrain";

const CANVAS = { w: 1920, h: 1080 };

function transform(partial: Partial<Transform> = {}): Transform {
  return {
    x: 960,
    y: 540,
    scaleX: 1,
    scaleY: 1,
    rotation: 0,
    crop: { left: 0, top: 0, right: 0, bottom: 0 },
    ...partial,
  };
}

describe("boundsOf", () => {
  it("boxes an unrotated centered item", () => {
    const box = boundsOf(transform(), { w: 400, h: 200 });
    expect(box).toEqual({ minX: 760, minY: 440, maxX: 1160, maxY: 640 });
  });

  it("a 90° rotation swaps the extents", () => {
    const box = boundsOf(transform({ rotation: 90 }), { w: 400, h: 200 });
    expect(box.maxX - box.minX).toBeCloseTo(200);
    expect(box.maxY - box.minY).toBeCloseTo(400);
  });
});

describe("clampMoveDelta (canvas rule: never cut off)", () => {
  const box: Box = { minX: 100, minY: 100, maxX: 500, maxY: 300 };

  it("free movement inside stays untouched", () => {
    expect(clampMoveDelta(box, 50, -20, CANVAS)).toEqual({ dx: 50, dy: -20 });
  });

  it("stops flush at the left/top edges", () => {
    expect(clampMoveDelta(box, -500, -500, CANVAS)).toEqual({ dx: -100, dy: -100 });
  });

  it("stops flush at the right/bottom edges", () => {
    expect(clampMoveDelta(box, 5000, 5000, CANVAS)).toEqual({ dx: 1420, dy: 780 });
  });
});

describe("clampScalesToCanvas (canvas rule: never oversized)", () => {
  it("free (per-axis) clamping caps one axis without touching the other", () => {
    const t = transform({ scaleX: 3, scaleY: 0.5 }); // 1000-wide content → 3000 px
    const { scaleX, scaleY } = clampScalesToCanvas(t, { w: 1000, h: 1000 }, CANVAS, false);
    expect(scaleX).toBeCloseTo(1.92); // 1920 / 1000
    expect(scaleY).toBeCloseTo(0.5);
  });

  it("aspect-preserving clamping shrinks both axes by one factor", () => {
    const t = transform({ scaleX: 4, scaleY: 2 });
    const { scaleX, scaleY } = clampScalesToCanvas(t, { w: 1000, h: 1000 }, CANVAS, true);
    expect(scaleY / scaleX).toBeCloseTo(0.5); // the 2:1 ratio survives
    expect(1000 * scaleY).toBeLessThanOrEqual(CANVAS.h + 1);
  });

  it("a rotated item is capped by the swapped canvas extent", () => {
    // At 90°, the content WIDTH spans the canvas HEIGHT.
    const t = transform({ rotation: 90, scaleX: 2, scaleY: 1 });
    const { scaleX } = clampScalesToCanvas(t, { w: 1000, h: 500 }, CANVAS, false);
    expect(1000 * scaleX).toBeLessThanOrEqual(CANVAS.h + 1);
  });

  it("cropped content clamps against the cropped size", () => {
    // 1000-wide source cropped to 500 wide: scale 4 fits (2000 > 1920? no — 500*4 = 2000 > 1920 → clamps).
    const t = transform({ scaleX: 4, crop: { left: 250, top: 0, right: 250, bottom: 0 } });
    const { scaleX } = clampScalesToCanvas(t, { w: 500, h: 1000 }, CANVAS, false);
    expect(500 * scaleX).toBeLessThanOrEqual(CANVAS.w + 1);
  });
});

describe("slideIntoCanvas", () => {
  it("slides an out-of-frame item back inside", () => {
    const t = slideIntoCanvas(transform({ x: -100 }), { w: 400, h: 200 }, CANVAS);
    const box = boundsOf(t, { w: 400, h: 200 });
    expect(box.minX).toBeGreaterThanOrEqual(0);
  });
});

describe("clampMoveAgainstObstacles (guests never overlap)", () => {
  const mover: Box = { minX: 0, minY: 0, maxX: 100, maxY: 100 };
  const wall: Box = { minX: 200, minY: 0, maxX: 300, maxY: 100 };

  it("stops dead at the obstacle's near edge (from the left)", () => {
    expect(clampMoveAgainstObstacles(mover, 250, 0, [wall]).dx).toBe(100);
  });

  it("stops approaching from the right", () => {
    const fromRight: Box = { minX: 400, minY: 0, maxX: 500, maxY: 100 };
    expect(clampMoveAgainstObstacles(fromRight, -300, 0, [wall]).dx).toBe(-100);
  });

  it("stops vertically too", () => {
    const above: Box = { minX: 200, minY: -300, maxX: 300, maxY: -200 };
    expect(clampMoveAgainstObstacles(above, 0, 500, [wall]).dy).toBe(200);
  });

  it("slides along the obstacle when there's no overlap on the other axis", () => {
    const below: Box = { minX: 0, minY: 200, maxX: 100, maxY: 300 };
    const clamped = clampMoveAgainstObstacles(below, 50, 0, [wall]);
    expect(clamped).toEqual({ dx: 50, dy: 0 }); // different rows — free to move
  });

  it("never traps a box that already overlaps (it can move away)", () => {
    const stuck: Box = { minX: 250, minY: 0, maxX: 350, maxY: 100 };
    expect(clampMoveAgainstObstacles(stuck, -400, 0, [wall]).dx).toBe(-400);
  });
});

describe("edgeDistances (the OBS-style px readout)", () => {
  it("reports px remaining to each edge", () => {
    const box: Box = { minX: 10, minY: 20, maxX: 1900, maxY: 1000 };
    expect(edgeDistances(box, CANVAS)).toEqual({ left: 10, top: 20, right: 20, bottom: 80 });
  });
});

describe("intersects", () => {
  it("touching edges do not count as overlap", () => {
    const a: Box = { minX: 0, minY: 0, maxX: 100, maxY: 100 };
    const b: Box = { minX: 100, minY: 0, maxX: 200, maxY: 100 };
    expect(intersects(a, b)).toBe(false);
    expect(intersects(a, { ...b, minX: 99 })).toBe(true);
  });
});
