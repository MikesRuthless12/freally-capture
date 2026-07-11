import { describe, expect, it } from "vitest";

import type { Transform } from "../api/types";
import {
  anchorPoint,
  displayedSize,
  moveAnchorTo,
  withSize,
  CENTER_ANCHOR,
} from "../lib/edit-transform";

const content = { w: 200, h: 100 };
function tf(extra: Partial<Transform> = {}): Transform {
  return {
    x: 500,
    y: 300,
    scaleX: 1,
    scaleY: 1,
    rotation: 0,
    crop: { left: 0, top: 0, right: 0, bottom: 0 },
    ...extra,
  };
}

describe("edit transform math (CAP-M05)", () => {
  it("reports the on-canvas size as content × scale", () => {
    expect(displayedSize(tf({ scaleX: 1.5, scaleY: 2 }), content)).toEqual({ w: 300, h: 200 });
  });

  it("round-trips size ↔ scale", () => {
    const next = withSize(tf(), content, 300, 250);
    expect(next.scaleX).toBeCloseTo(1.5);
    expect(next.scaleY).toBeCloseTo(2.5);
    expect(displayedSize(next, content)).toEqual({ w: 300, h: 250 });
  });

  it("clamps a size to the minimum scale, never zero", () => {
    const next = withSize(tf(), content, 0, 0);
    expect(next.scaleX).toBeGreaterThan(0);
    expect(next.scaleY).toBeGreaterThan(0);
  });

  it("gives the center anchor at the transform center", () => {
    expect(anchorPoint(tf(), content, CENTER_ANCHOR)).toEqual({ x: 500, y: 300 });
  });

  it("gives the top-left anchor at the box min corner", () => {
    // box of a 200×100 item centered at (500,300): x∈[400,600], y∈[250,350].
    expect(anchorPoint(tf(), content, { fx: 0, fy: 0 })).toEqual({ x: 400, y: 250 });
    expect(anchorPoint(tf(), content, { fx: 1, fy: 1 })).toEqual({ x: 600, y: 350 });
  });

  it("moves the top-left anchor to a target, shifting the center", () => {
    const next = moveAnchorTo(tf(), content, { fx: 0, fy: 0 }, 0, 0);
    // top-left → (0,0) means the center moves to (100, 50).
    expect(next.x).toBe(100);
    expect(next.y).toBe(50);
    expect(anchorPoint(next, content, { fx: 0, fy: 0 })).toEqual({ x: 0, y: 0 });
  });
});
