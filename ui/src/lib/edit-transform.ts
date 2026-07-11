/**
 * Numeric Edit Transform math (CAP-M05). Pure conversions between the model's
 * center-based transform and the panel's displayed values (size in canvas px,
 * position relative to a chosen anchor). Same transform model as
 * `transform.ts`/`constrain.ts`. `content` is the item's source size *after*
 * its transform crop (the caller derives it from the runtime source size).
 */

import type { Transform } from "../api/types";
import { boundsOf, MIN_SCALE, type Size } from "./constrain";

/** The item's on-canvas size (content × scale), canvas px. */
export function displayedSize(t: Transform, content: Size): Size {
  return { w: content.w * t.scaleX, h: content.h * t.scaleY };
}

/** Set the on-canvas width/height by adjusting the scales (about the center).
 * A zero-size content axis (fully cropped) keeps its scale. */
export function withSize(t: Transform, content: Size, w: number, h: number): Transform {
  const scaleX = content.w > 0 ? Math.max(w / content.w, MIN_SCALE) : t.scaleX;
  const scaleY = content.h > 0 ? Math.max(h / content.h, MIN_SCALE) : t.scaleY;
  return { ...t, scaleX, scaleY };
}

/** A position anchor: fractions of the item's bounding box (0=min, 0.5=center,
 * 1=max) on each axis. The 3×3 grid the panel exposes. */
export type Anchor = { fx: number; fy: number };

/** The nine anchors, row-major from top-left, matching a 3×3 grid. */
export const ANCHORS: Anchor[] = [
  { fx: 0, fy: 0 },
  { fx: 0.5, fy: 0 },
  { fx: 1, fy: 0 },
  { fx: 0, fy: 0.5 },
  { fx: 0.5, fy: 0.5 },
  { fx: 1, fy: 0.5 },
  { fx: 0, fy: 1 },
  { fx: 0.5, fy: 1 },
  { fx: 1, fy: 1 },
];

export const CENTER_ANCHOR: Anchor = { fx: 0.5, fy: 0.5 };

export function anchorsEqual(a: Anchor, b: Anchor): boolean {
  return a.fx === b.fx && a.fy === b.fy;
}

/** The canvas-px point of `anchor` on the item's (rotation-aware) bounding box. */
export function anchorPoint(t: Transform, content: Size, anchor: Anchor): { x: number; y: number } {
  const box = boundsOf(t, content);
  return {
    x: box.minX + anchor.fx * (box.maxX - box.minX),
    y: box.minY + anchor.fy * (box.maxY - box.minY),
  };
}

/** Move the item so its `anchor` point lands at `(x, y)` (canvas px). */
export function moveAnchorTo(
  t: Transform,
  content: Size,
  anchor: Anchor,
  x: number,
  y: number,
): Transform {
  const point = anchorPoint(t, content, anchor);
  return { ...t, x: t.x + (x - point.x), y: t.y + (y - point.y) };
}
