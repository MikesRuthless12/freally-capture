/**
 * Align a single item's bounding box to the canvas (CAP-M04), plus multi-item
 * align-to-each-other and distribute (the CAP-M04 follow-on). Pure geometry over
 * the same transform model as `transform.ts`/`constrain.ts`.
 */

import type { ItemId, SceneItem, Transform } from "../api/types";
import { boundsOf, type Box, type Size } from "./constrain";
import { contentSize, effectiveSourceSize } from "./transform";

export type AlignEdge = "left" | "hcenter" | "right" | "top" | "vcenter" | "bottom";

/** Every align edge, in a natural toolbar order. */
export const ALIGN_EDGES: AlignEdge[] = ["left", "hcenter", "right", "top", "vcenter", "bottom"];

/**
 * The transform that aligns `item`'s (rotation-aware) bounding box to a canvas
 * edge or center. `sourceW`/`sourceH` are the item's runtime source resolution
 * (from the program status). Returns `null` when the source size is unknown or
 * the item is fully cropped away — nothing to measure, so nothing to align.
 */
export function alignToCanvas(
  item: SceneItem,
  sourceW: number,
  sourceH: number,
  canvas: Size,
  edge: AlignEdge,
): Transform | null {
  const source = effectiveSourceSize(sourceW, sourceH, item.filters);
  const content = contentSize(source.w, source.h, item.transform.crop);
  if (!content) return null;
  const box = boundsOf(item.transform, content);
  let dx = 0;
  let dy = 0;
  switch (edge) {
    case "left":
      dx = -box.minX;
      break;
    case "hcenter":
      dx = (canvas.w - (box.minX + box.maxX)) / 2;
      break;
    case "right":
      dx = canvas.w - box.maxX;
      break;
    case "top":
      dy = -box.minY;
      break;
    case "vcenter":
      dy = (canvas.h - (box.minY + box.maxY)) / 2;
      break;
    case "bottom":
      dy = canvas.h - box.maxY;
      break;
  }
  return { ...item.transform, x: item.transform.x + dx, y: item.transform.y + dy };
}

/** One selected item measured for multi-item alignment: its id, current
 * transform, and rotation-aware bounding box (canvas px). */
export type Measured = { id: ItemId; transform: Transform; box: Box };

/** The distribute axis: `"h"` equalizes horizontal gaps, `"v"` vertical. */
export type DistributeAxis = "h" | "v";

/**
 * Align a group of items to *each other* along `edge` — every box's left (or
 * right/top/bottom) meets the group's, or centers land on the group's shared
 * center line. Returns only the items that move (keyed by id); the extreme item
 * on that edge stays put. A no-op for fewer than two items.
 */
export function alignItems(items: Measured[], edge: AlignEdge): Map<ItemId, Transform> {
  const out = new Map<ItemId, Transform>();
  if (items.length < 2) return out;
  const minX = Math.min(...items.map((m) => m.box.minX));
  const maxX = Math.max(...items.map((m) => m.box.maxX));
  const minY = Math.min(...items.map((m) => m.box.minY));
  const maxY = Math.max(...items.map((m) => m.box.maxY));
  const groupCx = (minX + maxX) / 2;
  const groupCy = (minY + maxY) / 2;
  for (const m of items) {
    let dx = 0;
    let dy = 0;
    switch (edge) {
      case "left":
        dx = minX - m.box.minX;
        break;
      case "right":
        dx = maxX - m.box.maxX;
        break;
      case "hcenter":
        dx = groupCx - (m.box.minX + m.box.maxX) / 2;
        break;
      case "top":
        dy = minY - m.box.minY;
        break;
      case "bottom":
        dy = maxY - m.box.maxY;
        break;
      case "vcenter":
        dy = groupCy - (m.box.minY + m.box.maxY) / 2;
        break;
    }
    if (dx !== 0 || dy !== 0) {
      out.set(m.id, { ...m.transform, x: m.transform.x + dx, y: m.transform.y + dy });
    }
  }
  return out;
}

/**
 * Distribute items so the gaps *between* their boxes are equal along `axis`.
 * The two extreme items stay fixed and the rest are re-spaced between them —
 * the classic "distribute spacing". Needs at least three items; a no-op below
 * that. Returns only the items that move.
 */
export function distributeItems(items: Measured[], axis: DistributeAxis): Map<ItemId, Transform> {
  const out = new Map<ItemId, Transform>();
  if (items.length < 3) return out;
  const lo = (b: Box) => (axis === "h" ? b.minX : b.minY);
  const hi = (b: Box) => (axis === "h" ? b.maxX : b.maxY);
  const sorted = [...items].sort((a, z) => lo(a.box) - lo(z.box));
  const first = sorted[0];
  const last = sorted[sorted.length - 1];
  const span = hi(last.box) - lo(first.box);
  const totalSize = sorted.reduce((sum, m) => sum + (hi(m.box) - lo(m.box)), 0);
  const gap = (span - totalSize) / (sorted.length - 1);

  let cursor = hi(first.box) + gap; // where the next item's low edge belongs
  for (let k = 1; k < sorted.length - 1; k += 1) {
    const m = sorted[k];
    const size = hi(m.box) - lo(m.box);
    const delta = cursor - lo(m.box);
    if (delta !== 0) {
      out.set(
        m.id,
        axis === "h"
          ? { ...m.transform, x: m.transform.x + delta }
          : { ...m.transform, y: m.transform.y + delta },
      );
    }
    cursor += size + gap;
  }
  return out;
}
