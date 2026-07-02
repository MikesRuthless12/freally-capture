import { describe, expect, it } from "vitest";

import type { Transform } from "../api/types";
import { canvasToLocal, contentSize, corners, hitTest } from "../lib/transform";

/**
 * Parity tests: these numbers mirror `crates/compositor/src/transform.rs`'s
 * unit tests — if either side changes, both must (the on-canvas handles are
 * only pixel-accurate while the two implementations agree).
 */

const plain = (x: number, y: number): Transform => ({
  x,
  y,
  scaleX: 1,
  scaleY: 1,
  rotation: 0,
  crop: { left: 0, top: 0, right: 0, bottom: 0 },
});

describe("transform math (Rust parity)", () => {
  it("identity centers content on its position", () => {
    const c = corners(plain(50, 30), { w: 20, h: 10 });
    expect(c[0].x).toBeCloseTo(40);
    expect(c[0].y).toBeCloseTo(25);
    expect(c[3].x).toBeCloseTo(60);
    expect(c[3].y).toBeCloseTo(35);
  });

  it("rotation is clockwise in screen space", () => {
    const t = { ...plain(0, 0), rotation: 90 };
    const c = corners(t, { w: 10, h: 2 });
    const rightMid = { x: (c[1].x + c[3].x) / 2, y: (c[1].y + c[3].y) / 2 };
    expect(rightMid.x).toBeCloseTo(0, 3);
    expect(rightMid.y).toBeCloseTo(5, 3);
  });

  it("crop shrinks the content", () => {
    expect(contentSize(10, 8, { left: 2, top: 1, right: 4, bottom: 3 })).toEqual({ w: 4, h: 4 });
    expect(contentSize(10, 8, { left: 6, top: 0, right: 6, bottom: 0 })).toBeNull();
  });

  it("canvasToLocal inverts localToCanvas", () => {
    const t: Transform = {
      x: 123,
      y: 456,
      scaleX: 1.5,
      scaleY: 0.75,
      rotation: -37,
      crop: { left: 0, top: 0, right: 0, bottom: 0 },
    };
    const content = { w: 200, h: 100 };
    const local = canvasToLocal(t, content, { x: 123, y: 456 });
    expect(local).not.toBeNull();
    // The item's position IS its content center.
    expect(local!.x).toBeCloseTo(100, 3);
    expect(local!.y).toBeCloseTo(50, 3);
  });

  it("hitTest respects rotation", () => {
    const t = { ...plain(50, 50), rotation: 90 };
    const content = { w: 40, h: 10 };
    // Rotated 90°: the bar now extends vertically.
    expect(hitTest(t, content, { x: 50, y: 68 })).toBe(true);
    expect(hitTest(t, content, { x: 68, y: 50 })).toBe(false);
  });
});
