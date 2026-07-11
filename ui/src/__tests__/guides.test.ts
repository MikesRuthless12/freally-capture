import { describe, expect, it } from "vitest";

import type { Box, Size } from "../lib/constrain";
import { snapCandidates, snapMove } from "../lib/guides";

const canvas: Size = { w: 1920, h: 1080 };

/** A box centered at (cx, cy) with the given half-extents. */
function box(cx: number, cy: number, hw: number, hh: number): Box {
  return { minX: cx - hw, maxX: cx + hw, minY: cy - hh, maxY: cy + hh };
}

describe("smart alignment guides (CAP-M04)", () => {
  it("snaps a near edge to the canvas edge", () => {
    // Left edge sits at x=3, within an 8px threshold of the canvas left (0).
    const moving = box(103, 540, 100, 100);
    const { dx, guides } = snapMove(moving, snapCandidates(canvas, []), 8);
    expect(dx).toBe(-3); // pulled left so minX lands on 0
    expect(guides).toContainEqual(
      expect.objectContaining({ orientation: "v", position: 0, target: "canvas" }),
    );
  });

  it("snaps the center to the canvas center on both axes", () => {
    // Center at (964, 545): 4px off the canvas center (960, 540).
    const moving = box(964, 545, 200, 120);
    const { dx, dy, guides } = snapMove(moving, snapCandidates(canvas, []), 8);
    expect(dx).toBe(-4);
    expect(dy).toBe(-5);
    expect(guides).toContainEqual(expect.objectContaining({ orientation: "v", position: 960 }));
    expect(guides).toContainEqual(expect.objectContaining({ orientation: "h", position: 540 }));
  });

  it("snaps to another item's edge and spans a guide across both", () => {
    const other = box(500, 300, 100, 100); // right edge at x=600
    const moving = box(703, 700, 100, 50); // left edge at x=603, 3px from 600
    const { dx, guides } = snapMove(moving, snapCandidates(canvas, [other]), 8);
    expect(dx).toBe(-3); // moving.minX (603) → 600
    const v = guides.find((g) => g.orientation === "v");
    expect(v?.position).toBe(600);
    expect(v?.target).toBe("item");
    // The guide spans from the other item's top (200) to the moving item's
    // bottom (750) — the union of both vertical extents.
    expect(v?.from).toBeLessThanOrEqual(200);
    expect(v?.to).toBeGreaterThanOrEqual(750);
  });

  it("does not snap when nothing is within the threshold", () => {
    const moving = box(1000, 600, 100, 100);
    const { dx, dy, guides } = snapMove(moving, snapCandidates(canvas, []), 8);
    expect(dx).toBe(0);
    expect(dy).toBe(0);
    expect(guides).toHaveLength(0);
  });

  it("a zero threshold disables snapping entirely", () => {
    const moving = box(103, 540, 100, 100); // would otherwise snap
    const result = snapMove(moving, snapCandidates(canvas, []), 0);
    expect(result).toEqual({ dx: 0, dy: 0, guides: [] });
  });

  it("picks the nearest of several candidate lines", () => {
    // minX=598 (2px from an item edge at 600) vs center far away — take the 2px.
    const other = box(500, 300, 100, 100); // right edge 600
    const moving = box(698, 700, 100, 50); // minX=598
    const { dx } = snapMove(moving, snapCandidates(canvas, [other]), 8);
    expect(dx).toBe(2); // 598 → 600
  });
});
