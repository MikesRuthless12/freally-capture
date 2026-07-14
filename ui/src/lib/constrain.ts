/**
 * Canvas constraints for preview interactions — Mike's rules:
 *
 * 1. a video item can never be MOVED outside the canvas frame (never cut off);
 * 2. it can never be SIZED bigger than the canvas frame;
 * 3. while moving/resizing, the UI shows how many px remain to each edge;
 * 4. two remote-guest captures can never overlap — a drag stops dead the
 *    moment one guest's bounds touch another's.
 *
 * All pure math over the same transform model as `transform.ts` (canvas px,
 * y-down; an item's bounding box is its rotated, cropped, scaled content).
 */

import type { Transform } from "../api/types";
import { contentSize, corners as itemCorners } from "./transform";

/** Scales this small are treated as degenerate (mirrors the drag minimum). */
export const MIN_SCALE = 0.01;

const EPS = 1e-6;

export type Size = { w: number; h: number };

/** An axis-aligned box in canvas px. */
export type Box = { minX: number; minY: number; maxX: number; maxY: number };

/** The item's axis-aligned bounding box (rotation, crop, scale included). */
export function boundsOf(t: Transform, content: Size): Box {
  const pts = itemCorners(t, content);
  return {
    minX: Math.min(...pts.map((p) => p.x)),
    minY: Math.min(...pts.map((p) => p.y)),
    maxX: Math.max(...pts.map((p) => p.x)),
    maxY: Math.max(...pts.map((p) => p.y)),
  };
}

export function intersects(a: Box, b: Box): boolean {
  return (
    a.maxX > b.minX + EPS && b.maxX > a.minX + EPS && a.maxY > b.minY + EPS && b.maxY > a.minY + EPS
  );
}

/** Clamp a move delta so the box never leaves the canvas. An oversized box
 * (legacy state only — rule 2 prevents new ones) pins flush to the origin. */
export function clampMoveDelta(
  box: Box,
  dx: number,
  dy: number,
  canvas: Size,
): { dx: number; dy: number } {
  const axis = (min: number, max: number, d: number, limit: number): number => {
    if (max - min >= limit) return -min;
    return Math.min(Math.max(d, -min), limit - max);
  };
  return { dx: axis(box.minX, box.maxX, dx, canvas.w), dy: axis(box.minY, box.maxY, dy, canvas.h) };
}

/**
 * Clamp the scales so the rotated bounding box fits the canvas.
 *
 * The bbox is linear in the scales: `bboxW = sx·A + sy·B`, `bboxH = sx·C +
 * sy·D` with A=w·|cos| B=h·|sin| C=w·|sin| D=h·|cos|. Aspect-preserving drags
 * shrink both scales by one factor; free drags clamp each axis on its own
 * (holding the other), so pushing one edge into the wall never shrinks the
 * other axis.
 */
export function clampScalesToCanvas(
  t: Transform,
  content: Size,
  canvas: Size,
  preserveAspect: boolean,
): { scaleX: number; scaleY: number } {
  const radians = (t.rotation * Math.PI) / 180;
  const cos = Math.abs(Math.cos(radians));
  const sin = Math.abs(Math.sin(radians));
  const a = content.w * cos;
  const b = content.h * sin;
  const c = content.w * sin;
  const d = content.h * cos;
  let sx = t.scaleX;
  let sy = t.scaleY;
  if (preserveAspect) {
    const bboxW = sx * a + sy * b;
    const bboxH = sx * c + sy * d;
    let factor = 1;
    if (bboxW > EPS) factor = Math.min(factor, canvas.w / bboxW);
    if (bboxH > EPS) factor = Math.min(factor, canvas.h / bboxH);
    sx = Math.max(sx * factor, MIN_SCALE);
    sy = Math.max(sy * factor, MIN_SCALE);
  } else {
    let sxMax = Infinity;
    if (a > EPS) sxMax = Math.min(sxMax, (canvas.w - sy * b) / a);
    if (c > EPS) sxMax = Math.min(sxMax, (canvas.h - sy * d) / c);
    sx = Math.min(Math.max(sx, MIN_SCALE), Math.max(sxMax, MIN_SCALE));
    let syMax = Infinity;
    if (b > EPS) syMax = Math.min(syMax, (canvas.w - sx * a) / b);
    if (d > EPS) syMax = Math.min(syMax, (canvas.h - sx * c) / d);
    sy = Math.min(Math.max(sy, MIN_SCALE), Math.max(syMax, MIN_SCALE));
  }
  return { scaleX: sx, scaleY: sy };
}

/** Slide a transform's position the minimum distance to sit inside the
 * canvas (used after scale/rotate/crop changes; the size already fits). */
export function slideIntoCanvas(t: Transform, content: Size, canvas: Size): Transform {
  const box = boundsOf(t, content);
  const { dx, dy } = clampMoveDelta(box, 0, 0, canvas);
  return dx === 0 && dy === 0 ? t : { ...t, x: t.x + dx, y: t.y + dy };
}

/** Apply a pasted transform, slid back into the canvas when the live source
 * size is known and left verbatim otherwise — shared by the Edit-Transform
 * dialog's Paste and the menu bar's Paste Transform so they can't diverge. */
export function constrainPaste(clip: Transform, source: Size | null, canvas: Size): Transform {
  const content = source ? contentSize(source.w, source.h, clip.crop) : null;
  return content ? slideIntoCanvas(clip, content, canvas) : clip;
}

/**
 * Clamp a move delta against obstacle boxes: the moving box stops the moment
 * it touches an obstacle's outside bounds (approach from any side), and can
 * slide along it. A box that already overlaps an obstacle (legacy state) is
 * never trapped — movement away stays free.
 */
export function clampMoveAgainstObstacles(
  start: Box,
  dx: number,
  dy: number,
  obstacles: Box[],
): { dx: number; dy: number } {
  const spans = (aMin: number, aMax: number, bMin: number, bMax: number) =>
    aMax > bMin + EPS && bMax > aMin + EPS;
  let cdx = dx;
  for (const o of obstacles) {
    if (!spans(start.minY, start.maxY, o.minY, o.maxY)) continue;
    if (cdx > 0 && start.maxX <= o.minX + EPS) cdx = Math.min(cdx, o.minX - start.maxX);
    else if (cdx < 0 && start.minX >= o.maxX - EPS) cdx = Math.max(cdx, o.maxX - start.minX);
  }
  const movedMinX = start.minX + cdx;
  const movedMaxX = start.maxX + cdx;
  let cdy = dy;
  for (const o of obstacles) {
    if (!spans(movedMinX, movedMaxX, o.minX, o.maxX)) continue;
    if (cdy > 0 && start.maxY <= o.minY + EPS) cdy = Math.min(cdy, o.minY - start.maxY);
    else if (cdy < 0 && start.minY >= o.maxY - EPS) cdy = Math.max(cdy, o.maxY - start.minY);
  }
  return { dx: cdx, dy: cdy };
}

/** px remaining to each canvas edge (the OBS-style readout; ≥ 0 = inside). */
export function edgeDistances(
  box: Box,
  canvas: Size,
): { left: number; top: number; right: number; bottom: number } {
  return {
    left: Math.round(box.minX),
    top: Math.round(box.minY),
    right: Math.round(canvas.w - box.maxX),
    bottom: Math.round(canvas.h - box.maxY),
  };
}
