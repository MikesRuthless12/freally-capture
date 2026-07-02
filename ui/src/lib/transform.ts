/**
 * Item transform math — the TypeScript mirror of
 * `crates/compositor/src/transform.rs`. The on-canvas handles are only
 * pixel-accurate while these two stay in lockstep; change them together.
 *
 * Canvas space is y-down pixels. An item's content is its source after the
 * transform crop; `x`/`y` place the content's center, scales multiply the
 * cropped size, rotation is degrees clockwise about that center.
 */

import type { Crop, Transform } from "../api/types";

export type Vec2 = { x: number; y: number };

/** Content size after the transform crop; null = fully cropped away. */
export function contentSize(
  sourceW: number,
  sourceH: number,
  crop: Crop,
): { w: number; h: number } | null {
  const w = sourceW - crop.left - crop.right;
  const h = sourceH - crop.top - crop.bottom;
  return w > 0 && h > 0 ? { w, h } : null;
}

/** The 2D affine [a, b, tx, c, d, ty]: local content px → canvas px. */
export function affine(t: Transform, content: { w: number; h: number }): number[] {
  const radians = (t.rotation * Math.PI) / 180;
  const sin = Math.sin(radians);
  const cos = Math.cos(radians);
  const a = cos * t.scaleX;
  const b = -sin * t.scaleY;
  const c = sin * t.scaleX;
  const d = cos * t.scaleY;
  const tx = t.x - (a * content.w * 0.5 + b * content.h * 0.5);
  const ty = t.y - (c * content.w * 0.5 + d * content.h * 0.5);
  return [a, b, tx, c, d, ty];
}

/** Map a local content point into canvas px. */
export function localToCanvas(t: Transform, content: { w: number; h: number }, p: Vec2): Vec2 {
  const [a, b, tx, c, d, ty] = affine(t, content);
  return { x: a * p.x + b * p.y + tx, y: c * p.x + d * p.y + ty };
}

/**
 * Map a canvas point into the item's local content space (inverse affine).
 * Returns null for degenerate (zero-scale) transforms.
 */
export function canvasToLocal(
  t: Transform,
  content: { w: number; h: number },
  p: Vec2,
): Vec2 | null {
  const [a, b, tx, c, d, ty] = affine(t, content);
  const det = a * d - b * c;
  if (Math.abs(det) < 1e-9) return null;
  const x = p.x - tx;
  const y = p.y - ty;
  return { x: (d * x - b * y) / det, y: (-c * x + a * y) / det };
}

/**
 * The content's four corners in canvas px, in local corner order
 * `(0,0) (w,0) (0,h) (w,h)` — matches `transform.rs::corners`.
 */
export function corners(t: Transform, content: { w: number; h: number }): Vec2[] {
  return [
    localToCanvas(t, content, { x: 0, y: 0 }),
    localToCanvas(t, content, { x: content.w, y: 0 }),
    localToCanvas(t, content, { x: 0, y: content.h }),
    localToCanvas(t, content, { x: content.w, y: content.h }),
  ];
}

/** True when a canvas point falls inside the item's (rotated) content box. */
export function hitTest(t: Transform, content: { w: number; h: number }, p: Vec2): boolean {
  const local = canvasToLocal(t, content, p);
  if (!local) return false;
  return local.x >= 0 && local.x <= content.w && local.y >= 0 && local.y <= content.h;
}
