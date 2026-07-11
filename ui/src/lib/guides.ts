/**
 * Smart alignment guides (CAP-M04): while an item is dragged, snap its box to
 * the canvas edges/centers and to *other items'* edges/centers, and report the
 * guide lines to draw. Pure math over canvas-px boxes (same space as
 * `constrain.ts`); the caller renders the returned guides (SVG on the JPEG
 * path, native GPU on the Windows surface) and converts the snap threshold from
 * display px to canvas px.
 */

import type { Box, Size } from "./constrain";

/** A rectangle in canvas px. */
export type Rect = { x: number; y: number; w: number; h: number };

/** Inset fractions per edge for the two broadcast safe areas. */
const ACTION_SAFE_INSET = 0.05; // 90% box — graphics/action stay inside
const TITLE_SAFE_INSET = 0.1; // 80% box — text stays inside

/**
 * The action-safe (90%) and title-safe (80%) rectangles for a canvas — the
 * standard broadcast overscan guides, centered.
 */
export function safeAreaRects(canvas: Size): { action: Rect; title: Rect } {
  const inset = (frac: number): Rect => ({
    x: canvas.w * frac,
    y: canvas.h * frac,
    w: canvas.w * (1 - 2 * frac),
    h: canvas.h * (1 - 2 * frac),
  });
  return { action: inset(ACTION_SAFE_INSET), title: inset(TITLE_SAFE_INSET) };
}

/** A snap line to draw while dragging, in canvas px. */
export type Guide = {
  /** "v" = a vertical line at constant x; "h" = horizontal at constant y. */
  orientation: "v" | "h";
  /** The line's position — x for "v", y for "h" — in canvas px. */
  position: number;
  /** Draw between these two coordinates on the *other* axis, canvas px. */
  from: number;
  to: number;
  /** Whether the matched line came from the canvas or another item. */
  target: "canvas" | "item";
};

/** One snap line candidate: a position plus the extent to draw a guide over. */
type Candidate = { pos: number; lo: number; hi: number; target: "canvas" | "item" };

/**
 * The snap lines the moving item can catch: the canvas' left/center/right and
 * top/middle/bottom, plus every other item's box edges and centers.
 */
export function snapCandidates(canvas: Size, others: Box[]): { v: Candidate[]; h: Candidate[] } {
  const v: Candidate[] = [
    { pos: 0, lo: 0, hi: canvas.h, target: "canvas" },
    { pos: canvas.w / 2, lo: 0, hi: canvas.h, target: "canvas" },
    { pos: canvas.w, lo: 0, hi: canvas.h, target: "canvas" },
  ];
  const h: Candidate[] = [
    { pos: 0, lo: 0, hi: canvas.w, target: "canvas" },
    { pos: canvas.h / 2, lo: 0, hi: canvas.w, target: "canvas" },
    { pos: canvas.h, lo: 0, hi: canvas.w, target: "canvas" },
  ];
  for (const b of others) {
    const cx = (b.minX + b.maxX) / 2;
    const cy = (b.minY + b.maxY) / 2;
    v.push(
      { pos: b.minX, lo: b.minY, hi: b.maxY, target: "item" },
      { pos: cx, lo: b.minY, hi: b.maxY, target: "item" },
      { pos: b.maxX, lo: b.minY, hi: b.maxY, target: "item" },
    );
    h.push(
      { pos: b.minY, lo: b.minX, hi: b.maxX, target: "item" },
      { pos: cy, lo: b.minX, hi: b.maxX, target: "item" },
      { pos: b.maxY, lo: b.minX, hi: b.maxX, target: "item" },
    );
  }
  return { v, h };
}

function bestSnap(
  probes: number[],
  candidates: Candidate[],
  threshold: number,
): { offset: number; candidate: Candidate } | null {
  let best: { offset: number; candidate: Candidate } | null = null;
  for (const probe of probes) {
    for (const candidate of candidates) {
      const offset = candidate.pos - probe;
      if (Math.abs(offset) <= threshold && (!best || Math.abs(offset) < Math.abs(best.offset))) {
        best = { offset, candidate };
      }
    }
  }
  return best;
}

/**
 * Snap a moving box (already offset by the raw drag delta) against the
 * candidates. Returns the additional `dx`/`dy` to apply so an edge or center
 * lines up, plus the guides to draw (built from the *snapped* extents so a
 * guide spans both the moving item and its match). `threshold` is canvas px;
 * pass 0 (or empty candidates) to disable snapping. Each axis snaps to its
 * single nearest line — the classic smart-guide behaviour.
 */
export function snapMove(
  box: Box,
  candidates: { v: Candidate[]; h: Candidate[] },
  threshold: number,
): { dx: number; dy: number; guides: Guide[] } {
  if (threshold <= 0) return { dx: 0, dy: 0, guides: [] };
  const midX = (box.minX + box.maxX) / 2;
  const midY = (box.minY + box.maxY) / 2;
  const bestX = bestSnap([box.minX, midX, box.maxX], candidates.v, threshold);
  const bestY = bestSnap([box.minY, midY, box.maxY], candidates.h, threshold);

  const dx = bestX?.offset ?? 0;
  const dy = bestY?.offset ?? 0;
  const snapped: Box = {
    minX: box.minX + dx,
    maxX: box.maxX + dx,
    minY: box.minY + dy,
    maxY: box.maxY + dy,
  };

  const guides: Guide[] = [];
  if (bestX) {
    const c = bestX.candidate;
    guides.push({
      orientation: "v",
      position: c.pos,
      from: Math.min(c.lo, snapped.minY),
      to: Math.max(c.hi, snapped.maxY),
      target: c.target,
    });
  }
  if (bestY) {
    const c = bestY.candidate;
    guides.push({
      orientation: "h",
      position: c.pos,
      from: Math.min(c.lo, snapped.minX),
      to: Math.max(c.hi, snapped.maxX),
      target: c.target,
    });
  }
  return { dx, dy, guides };
}
