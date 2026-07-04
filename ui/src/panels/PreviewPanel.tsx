import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";

import {
  nativePreviewActive,
  nativePreviewSetRegion,
  nativePreviewSetSelection,
} from "../api/commands";
import type { Collection, ItemId, ProgramStatus, Scene, SceneItem, Transform } from "../api/types";
import {
  canvasToLocal,
  contentSize,
  corners as itemCorners,
  effectiveSourceSize,
  hitTest,
  localToCanvas,
  type Vec2,
} from "../lib/transform";

type PreviewPanelProps = {
  collection: Collection | null;
  scene: Scene | null;
  program: ProgramStatus | null;
  selectedItem: ItemId | null;
  onSelect: (item: ItemId | null) => void;
  onItemTransform: (item: ItemId, transform: Transform) => void;
};

/** Poll interval for the program frame pipe (~30 fps). */
const FRAME_POLL_MS = 33;
/** Handle hit radius, display px. */
const HANDLE_RADIUS = 7;
/** Rotate handle offset above the top edge, display px. */
const ROTATE_OFFSET = 22;
const MIN_SCALE = 0.01;

type DragState = {
  mode: "move" | "scale-corner" | "scale-edge" | "rotate" | "crop-edge";
  itemId: ItemId;
  start: Transform;
  /** Content size (source px after start.crop). */
  content: { w: number; h: number };
  /** Source resolution (pre-crop). */
  source: { w: number; h: number };
  /** Pointer start, program px. */
  pointer: Vec2;
  /** Fixed point (opposite corner/edge midpoint), program px. */
  fixed: Vec2;
  /** For edge drags: which edge (0=left 1=right 2=top 3=bottom). */
  edge?: number;
  /** For rotate: the cursor's start angle minus the item's start rotation. */
  angleOffset?: number;
};

/**
 * The program preview: the composed program frame (polled from the
 * `preview://` pipe) with pixel-accurate selection + transform handles.
 * Handle math mirrors `crates/compositor/src/transform.rs` via
 * `../lib/transform.ts` — drags edit the model, the compositor renders it.
 */
export function PreviewPanel({
  collection,
  scene,
  program,
  selectedItem,
  onSelect,
  onItemTransform,
}: PreviewPanelProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [box, setBox] = useState({ left: 0, top: 0, width: 0, height: 0 });
  const dragRef = useRef<DragState | null>(null);
  // The native GPU preview surface (the "OBS feel" path): when active, a
  // native child window paints the program region and the JPEG canvas is
  // suppressed. Off Windows this stays false → the JPEG path renders.
  const [nativeActive, setNativeActive] = useState(false);
  const [hoverCursor, setHoverCursor] = useState("default");
  const lastRegion = useRef("");

  const programW = collection?.canvasWidth ?? 1920;
  const programH = collection?.canvasHeight ?? 1080;
  const running = program?.state === "running";
  const emptyScene = (scene?.items.length ?? 0) === 0;

  // The displayed box: the program frame letterboxed inside the container.
  useEffect(() => {
    const container = containerRef.current;
    if (!container) return;
    const recompute = () => {
      const rect = container.getBoundingClientRect();
      const scale = Math.min(rect.width / programW, rect.height / programH);
      const width = Math.max(1, programW * scale);
      const height = Math.max(1, programH * scale);
      setBox({
        left: (rect.width - width) / 2,
        top: (rect.height - height) / 2,
        width,
        height,
      });
    };
    recompute();
    const observer = new ResizeObserver(recompute);
    observer.observe(container);
    return () => observer.disconnect();
  }, [programW, programH]);

  // Is the native GPU preview surface available? (Windows + created.)
  useEffect(() => {
    let alive = true;
    nativePreviewActive()
      .then((on) => alive && setNativeActive(on))
      .catch(() => alive && setNativeActive(false));
    return () => {
      alive = false;
    };
  }, []);

  // While native, keep the child window positioned over the letterboxed
  // program area (physical px, window-client relative) and shown only when
  // there's a live scene to paint (so the empty/starting HTML hints aren't
  // hidden behind it). A light interval catches layout drift as docks resize.
  useEffect(() => {
    if (!nativeActive) return;
    const report = () => {
      const container = containerRef.current;
      if (!container) return;
      const rect = container.getBoundingClientRect();
      const dpr = window.devicePixelRatio || 1;
      const x = Math.round((rect.left + box.left) * dpr);
      const y = Math.round((rect.top + box.top) * dpr);
      const w = Math.round(box.width * dpr);
      const h = Math.round(box.height * dpr);
      const visible = running && !emptyScene;
      const key = `${x},${y},${w},${h},${visible}`;
      if (key !== lastRegion.current) {
        lastRegion.current = key;
        void nativePreviewSetRegion(x, y, w, h, visible).catch(() => undefined);
      }
    };
    report();
    const timer = setInterval(report, 150);
    return () => {
      clearInterval(timer);
      lastRegion.current = "";
      void nativePreviewSetRegion(0, 0, 0, 0, false).catch(() => undefined);
    };
  }, [nativeActive, box, running, emptyScene]);

  // Tell the native surface which item is selected, so it can draw the box +
  // handles *into* the GPU frame (they're hidden under the opaque surface).
  // Also fires with `null` on deselect. Only matters on the native path.
  useEffect(() => {
    if (!nativeActive) return;
    void nativePreviewSetSelection(selectedItem).catch(() => undefined);
  }, [nativeActive, selectedItem]);

  // Poll the composed frame onto the canvas while the studio runs (skipped
  // when the native surface is painting the region).
  useEffect(() => {
    if (!running || nativeActive) return;
    // jsdom (tests) has neither the scheme nor createImageBitmap.
    if (typeof createImageBitmap === "undefined") return;
    let stopped = false;
    let timer: ReturnType<typeof setTimeout> | undefined;
    let lastSeq = "";
    const url = convertFileSrc("frame", "preview");

    const tick = async () => {
      if (stopped) return;
      try {
        const response = await fetch(url, { cache: "no-store" });
        if (response.status === 200) {
          const seq = response.headers.get("x-frame-seq") ?? "";
          if (seq !== lastSeq) {
            lastSeq = seq;
            const blob = await response.blob();
            const bitmap = await createImageBitmap(blob);
            const canvas = canvasRef.current;
            if (canvas && !stopped) {
              if (canvas.width !== bitmap.width) canvas.width = bitmap.width;
              if (canvas.height !== bitmap.height) canvas.height = bitmap.height;
              canvas.getContext("2d")?.drawImage(bitmap, 0, 0);
            }
            bitmap.close();
          }
        }
      } catch {
        // The studio is restarting or the scheme isn't up yet — just retry.
      }
      if (!stopped) timer = setTimeout(tick, FRAME_POLL_MS);
    };
    void tick();
    return () => {
      stopped = true;
      if (timer !== undefined) clearTimeout(timer);
    };
  }, [running, nativeActive]);

  const displayScale = box.width > 0 ? box.width / programW : 1;
  const toProgram = useCallback(
    (event: { clientX: number; clientY: number }): Vec2 => {
      const container = containerRef.current;
      if (!container || displayScale === 0) return { x: 0, y: 0 };
      const rect = container.getBoundingClientRect();
      return {
        x: (event.clientX - rect.left - box.left) / displayScale,
        y: (event.clientY - rect.top - box.top) / displayScale,
      };
    },
    [box, displayScale],
  );

  /**
   * The item's composed base size: the runtime-reported source resolution
   * with the item's enabled Crop filters folded in — the compositor draws
   * the chain output, so the handles must measure against it too.
   */
  const sourceSize = useCallback(
    (item: SceneItem): { w: number; h: number } | null => {
      const status = program?.sources[item.source];
      if (!status?.width || !status?.height) return null;
      return effectiveSourceSize(status.width, status.height, item.filters);
    },
    [program],
  );

  const selected = scene?.items.find((item) => item.id === selectedItem) ?? null;
  const selectedGeometry = useMemo(() => {
    if (!selected || selected.pendingFit) return null;
    const source = sourceSize(selected);
    if (!source) return null;
    const content = contentSize(source.w, source.h, selected.transform.crop);
    if (!content) return null;
    return { source, content };
  }, [selected, sourceSize]);

  /** The cursor for a pointer position: resize on handles, move on the body. */
  const cursorFor = useCallback(
    (p: Vec2): string => {
      if (!selected || !selectedGeometry || selected.locked) return "default";
      const { content } = selectedGeometry;
      const t = selected.transform;
      const corner = itemCorners(t, content);
      const mids = edgeMidpoints(corner);
      const rotate = rotateHandle(t, content, displayScale);
      const near = (a: Vec2) => distance(a, p) * displayScale <= HANDLE_RADIUS;
      if (near(rotate)) return "grab";
      const c = corner.findIndex(near);
      if (c >= 0) return c === 0 || c === 3 ? "nwse-resize" : "nesw-resize";
      const e = mids.findIndex(near);
      if (e >= 0) return e <= 1 ? "ew-resize" : "ns-resize";
      return hitTest(t, content, p) ? "move" : "default";
    },
    [selected, selectedGeometry, displayScale],
  );

  // -- pointer interactions ---------------------------------------------------

  const beginDrag = (event: React.PointerEvent) => {
    if (!scene) return;
    const p = toProgram(event);

    // Grab a handle of the selected item first.
    if (selected && selectedGeometry && !selected.locked) {
      const { content, source } = selectedGeometry;
      const t = selected.transform;
      const corner = itemCorners(t, content);
      const mids = edgeMidpoints(corner);
      const rotate = rotateHandle(t, content, displayScale);
      const near = (a: Vec2) => distance(a, p) * displayScale <= HANDLE_RADIUS;

      const start: Omit<DragState, "mode" | "fixed"> = {
        itemId: selected.id,
        start: t,
        content,
        source,
        pointer: p,
      };
      if (near(rotate)) {
        const angle = Math.atan2(p.y - t.y, p.x - t.x);
        dragRef.current = {
          ...start,
          mode: "rotate",
          fixed: { x: t.x, y: t.y },
          angleOffset: (angle * 180) / Math.PI - t.rotation,
        };
      } else {
        const cornerHit = corner.findIndex(near);
        if (cornerHit >= 0) {
          dragRef.current = {
            ...start,
            mode: "scale-corner",
            fixed: corner[3 - cornerHit], // (0,0)↔(w,h), (w,0)↔(0,h)
          };
        } else {
          const edgeHit = mids.findIndex(near);
          if (edgeHit >= 0) {
            dragRef.current = {
              ...start,
              mode: event.altKey ? "crop-edge" : "scale-edge",
              edge: edgeHit,
              fixed: mids[edgeHit ^ 1], // 0↔1 (left/right), 2↔3 (top/bottom)
            };
          } else if (hitTest(t, content, p)) {
            dragRef.current = { ...start, mode: "move", fixed: { x: t.x, y: t.y } };
          }
        }
      }
      if (dragRef.current) {
        (event.target as Element).setPointerCapture(event.pointerId);
        event.preventDefault();
        return;
      }
    }

    // Otherwise: select the topmost item under the cursor (top = last).
    for (let index = scene.items.length - 1; index >= 0; index -= 1) {
      const item = scene.items[index];
      if (!item.visible) continue;
      const source = sourceSize(item);
      if (!source) continue;
      const content = contentSize(source.w, source.h, item.transform.crop);
      if (!content) continue;
      if (hitTest(item.transform, content, p)) {
        onSelect(item.id);
        // Immediately allow dragging the newly selected (unlocked) item.
        if (!item.locked) {
          dragRef.current = {
            mode: "move",
            itemId: item.id,
            start: item.transform,
            content,
            source,
            pointer: p,
            fixed: { x: item.transform.x, y: item.transform.y },
          };
          (event.target as Element).setPointerCapture(event.pointerId);
        }
        event.preventDefault();
        return;
      }
    }
    onSelect(null);
  };

  const updateDrag = (event: React.PointerEvent) => {
    const p = toProgram(event);
    const drag = dragRef.current;
    if (!drag) {
      setHoverCursor(cursorFor(p));
      return;
    }
    const next = applyDrag(drag, p, event.shiftKey);
    if (next) onItemTransform(drag.itemId, next);
  };

  const endDrag = (event: React.PointerEvent) => {
    if (dragRef.current) {
      updateDrag(event);
      dragRef.current = null;
    }
  };

  // -- overlay geometry (display px) -------------------------------------------

  const overlay = useMemo(() => {
    if (!selected || !selectedGeometry) return null;
    const { content } = selectedGeometry;
    const t = selected.transform;
    const toDisplay = (v: Vec2): Vec2 => ({ x: v.x * displayScale, y: v.y * displayScale });
    const corner = itemCorners(t, content).map(toDisplay);
    const mids = edgeMidpoints(corner);
    const rotate = toDisplay(rotateHandle(t, content, displayScale));
    return { corner, mids, rotate, locked: selected.locked };
  }, [selected, selectedGeometry, displayScale]);

  return (
    <section
      aria-label="Program preview"
      className="relative flex min-h-0 min-w-0 flex-col overflow-hidden rounded-xl border border-white/10 bg-black/60"
    >
      <div ref={containerRef} className="relative min-h-0 flex-1">
        {running && (
          <>
            {!nativeActive && (
              <canvas
                ref={canvasRef}
                role="img"
                aria-label="Program output"
                className="absolute"
                style={{
                  left: box.left,
                  top: box.top,
                  width: box.width,
                  height: box.height,
                }}
              />
            )}
            <svg
              role="application"
              aria-label="Canvas editor"
              className="absolute touch-none"
              style={{
                left: box.left,
                top: box.top,
                width: box.width,
                height: box.height,
                cursor: hoverCursor,
              }}
              onPointerDown={beginDrag}
              onPointerMove={updateDrag}
              onPointerUp={endDrag}
              onPointerCancel={endDrag}
            >
              {overlay && (
                <g>
                  <polygon
                    points={[
                      overlay.corner[0],
                      overlay.corner[1],
                      overlay.corner[3],
                      overlay.corner[2],
                    ]
                      .map((v) => `${v.x},${v.y}`)
                      .join(" ")}
                    fill="none"
                    stroke="#4a9eff"
                    strokeWidth={1.5}
                    strokeDasharray={overlay.locked ? "4 3" : undefined}
                  />
                  {!overlay.locked && (
                    <>
                      <line
                        x1={overlay.mids[2].x}
                        y1={overlay.mids[2].y}
                        x2={overlay.rotate.x}
                        y2={overlay.rotate.y}
                        stroke="#4a9eff"
                        strokeWidth={1}
                      />
                      {[...overlay.corner, ...overlay.mids].map((v, index) => (
                        <rect
                          key={index}
                          x={v.x - 4}
                          y={v.y - 4}
                          width={8}
                          height={8}
                          fill="#0a0a0b"
                          stroke="#4a9eff"
                          strokeWidth={1.5}
                        />
                      ))}
                      <circle
                        cx={overlay.rotate.x}
                        cy={overlay.rotate.y}
                        r={5}
                        fill="#0a0a0b"
                        stroke="#00d4ff"
                        strokeWidth={1.5}
                      />
                    </>
                  )}
                </g>
              )}
            </svg>
          </>
        )}
        {!running || emptyScene ? (
          <div className="pointer-events-none absolute inset-0 flex items-center justify-center p-6">
            <div className="flex max-w-md flex-col items-center gap-2 text-center">
              <span className="bg-gradient-to-r from-havoc-accent to-havoc-accent-2 bg-clip-text text-sm font-semibold tracking-widest text-transparent uppercase">
                Program
              </span>
              {program?.state === "noGpu" ? (
                <p className="m-0 text-xs text-red-400" role="alert">
                  No usable GPU adapter was found — the compositor can’t run on this machine.
                  <span className="mt-1 block text-havoc-muted">{program.adapter}</span>
                </p>
              ) : !running ? (
                <p className="m-0 text-xs text-havoc-muted" role="status">
                  Starting the compositor…
                </p>
              ) : (
                <p className="m-0 text-xs text-havoc-muted">
                  This scene is empty — add a source in Sources, then drag, scale, and rotate it
                  right here on the canvas.
                </p>
              )}
            </div>
          </div>
        ) : null}
      </div>
      {running && program && (
        <div className="flex shrink-0 items-center gap-3 border-t border-white/5 bg-black/40 px-3 py-1 text-[11px] text-havoc-muted">
          <span className="flex items-center gap-1.5">
            <span className="h-1.5 w-1.5 rounded-full bg-emerald-400" aria-hidden="true" />
            {program.width}×{program.height}
          </span>
          <span>{program.fps} fps</span>
          {program.dropped > 0 && <span className="text-amber-300">{program.dropped} dropped</span>}
          <span className="ml-auto max-w-64 truncate" title={program.adapter}>
            {program.adapter}
          </span>
        </div>
      )}
    </section>
  );
}

// ---------------------------------------------------------------------------
// Drag math (program-pixel space; mirrors transform.rs semantics)
// ---------------------------------------------------------------------------

function distance(a: Vec2, b: Vec2): number {
  return Math.hypot(a.x - b.x, a.y - b.y);
}

/** Edge midpoints from the corner array: [left, right, top, bottom]. */
function edgeMidpoints(corner: Vec2[]): Vec2[] {
  const mid = (a: Vec2, b: Vec2): Vec2 => ({ x: (a.x + b.x) / 2, y: (a.y + b.y) / 2 });
  return [
    mid(corner[0], corner[2]), // left
    mid(corner[1], corner[3]), // right
    mid(corner[0], corner[1]), // top
    mid(corner[2], corner[3]), // bottom
  ];
}

/** The rotate handle, program px: above the top edge midpoint. */
function rotateHandle(t: Transform, content: { w: number; h: number }, displayScale: number): Vec2 {
  const radians = (t.rotation * Math.PI) / 180;
  const top = localToCanvas(t, content, { x: content.w / 2, y: 0 });
  const offset = ROTATE_OFFSET / Math.max(displayScale, 1e-6);
  return { x: top.x + Math.sin(radians) * offset, y: top.y - Math.cos(radians) * offset };
}

/** Compute the dragged transform. Returns null when the drag is degenerate. */
function applyDrag(drag: DragState, p: Vec2, shift: boolean): Transform | null {
  const t = drag.start;
  switch (drag.mode) {
    case "move": {
      return {
        ...t,
        x: drag.fixed.x + (p.x - drag.pointer.x),
        y: drag.fixed.y + (p.y - drag.pointer.y),
      };
    }
    case "rotate": {
      const angle = (Math.atan2(p.y - t.y, p.x - t.x) * 180) / Math.PI;
      let rotation = angle - (drag.angleOffset ?? 0);
      if (shift) rotation = Math.round(rotation / 15) * 15;
      // Normalize to (-180, 180] for readable numbers.
      rotation = ((((rotation + 180) % 360) + 360) % 360) - 180;
      return { ...t, rotation };
    }
    case "scale-corner": {
      // Opposite corners stay opposite through the shared center.
      const center = { x: (drag.fixed.x + p.x) / 2, y: (drag.fixed.y + p.y) / 2 };
      const radians = (t.rotation * Math.PI) / 180;
      const sin = Math.sin(radians);
      const cos = Math.cos(radians);
      // R⁻¹ · (p − center)
      const dx = p.x - center.x;
      const dy = p.y - center.y;
      const local = { x: cos * dx + sin * dy, y: -sin * dx + cos * dy };
      let scaleX: number;
      let scaleY: number;
      if (shift) {
        scaleX = Math.max(Math.abs(local.x) / (drag.content.w / 2), MIN_SCALE);
        scaleY = Math.max(Math.abs(local.y) / (drag.content.h / 2), MIN_SCALE);
      } else {
        // Aspect-preserving: scale BOTH axes by the same factor relative to
        // the item's *current* scales — a stretched item must not snap to
        // uniform scale the moment a corner is grabbed.
        const halfDiag = Math.hypot(
          (drag.content.w / 2) * t.scaleX,
          (drag.content.h / 2) * t.scaleY,
        );
        const k = distance(p, center) / Math.max(halfDiag, 1e-6);
        scaleX = Math.max(t.scaleX * k, MIN_SCALE);
        scaleY = Math.max(t.scaleY * k, MIN_SCALE);
      }
      return { ...t, x: center.x, y: center.y, scaleX, scaleY };
    }
    case "scale-edge": {
      const edge = drag.edge ?? 0;
      const radians = (t.rotation * Math.PI) / 180;
      const horizontal = edge <= 1;
      // The outward axis from the fixed edge toward the dragged edge.
      const sign = edge === 0 || edge === 2 ? -1 : 1;
      const axis: Vec2 = horizontal
        ? { x: Math.cos(radians) * sign, y: Math.sin(radians) * sign }
        : { x: -Math.sin(radians) * sign, y: Math.cos(radians) * sign };
      const extent = Math.max(
        (p.x - drag.fixed.x) * axis.x + (p.y - drag.fixed.y) * axis.y,
        MIN_SCALE * (horizontal ? drag.content.w : drag.content.h),
      );
      const scale = extent / (horizontal ? drag.content.w : drag.content.h);
      const center = {
        x: drag.fixed.x + (axis.x * extent) / 2,
        y: drag.fixed.y + (axis.y * extent) / 2,
      };
      return horizontal
        ? { ...t, x: center.x, y: center.y, scaleX: scale }
        : { ...t, x: center.x, y: center.y, scaleY: scale };
    }
    case "crop-edge": {
      // Alt-drag: crop the dragged edge; the opposite edge stays put and the
      // scale is untouched (OBS behavior).
      const edge = drag.edge ?? 0;
      const horizontal = edge <= 1;
      const local = canvasToLocal(t, drag.content, p);
      if (!local) return null;
      const crop = { ...t.crop };
      if (horizontal) {
        // How far the cursor sits from the fixed (opposite) edge, source px.
        const fromFixed = edge === 0 ? drag.content.w - local.x : local.x;
        const cut = Math.round(drag.content.w - fromFixed);
        if (edge === 0) {
          crop.left = clampCrop(drag.start.crop.left + cut, drag.source.w, crop.right);
        } else {
          crop.right = clampCrop(drag.start.crop.right + cut, drag.source.w, crop.left);
        }
      } else {
        const fromFixed = edge === 2 ? drag.content.h - local.y : local.y;
        const cut = Math.round(drag.content.h - fromFixed);
        if (edge === 2) {
          crop.top = clampCrop(drag.start.crop.top + cut, drag.source.h, crop.bottom);
        } else {
          crop.bottom = clampCrop(drag.start.crop.bottom + cut, drag.source.h, crop.top);
        }
      }
      // Keep the fixed edge glued: recompute the center from the fixed
      // midpoint and the new content extent along the drag axis.
      const content = {
        w: drag.source.w - crop.left - crop.right,
        h: drag.source.h - crop.top - crop.bottom,
      };
      if (content.w <= 0 || content.h <= 0) return null;
      const radians = (t.rotation * Math.PI) / 180;
      const sign = edge === 0 || edge === 2 ? -1 : 1;
      const axis: Vec2 = horizontal
        ? { x: Math.cos(radians) * sign, y: Math.sin(radians) * sign }
        : { x: -Math.sin(radians) * sign, y: Math.cos(radians) * sign };
      const extent = horizontal ? content.w * t.scaleX : content.h * t.scaleY;
      return {
        ...t,
        crop,
        x: drag.fixed.x + (axis.x * extent) / 2,
        y: drag.fixed.y + (axis.y * extent) / 2,
      };
    }
  }
}

function clampCrop(value: number, sourceExtent: number, oppositeCrop: number): number {
  return Math.max(0, Math.min(value, sourceExtent - oppositeCrop - 1));
}
