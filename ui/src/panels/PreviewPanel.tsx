import { useCallback, useEffect, useMemo, useRef, useState, useSyncExternalStore } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";

import { isModalOpen, modalSubscribe } from "../lib/modal";
import { useT } from "../i18n/t";

import {
  nativePreviewActive,
  nativePreviewSetOverlay,
  nativePreviewSetRegion,
  nativePreviewSetSelection,
  studioSetFocus,
  studioSetGuides,
  studioZoomScroll,
} from "../api/commands";
import type {
  AlignmentSettings,
  BackdropSplit,
  Collection,
  GuideLine,
  ItemId,
  ProgramStatus,
  Scene,
  SceneItem,
  Transform,
} from "../api/types";
import {
  boundsOf,
  clampMoveAgainstObstacles,
  clampMoveDelta,
  clampScalesToCanvas,
  edgeDistances,
  intersects,
  MIN_SCALE,
  slideIntoCanvas,
  type Box,
  type Size,
} from "../lib/constrain";
import { safeAreaRects, snapCandidates, snapMove, type Guide, type Rect } from "../lib/guides";
import {
  alignItems,
  alignToCanvas,
  ALIGN_EDGES,
  distributeItems,
  type AlignEdge,
  type DistributeAxis,
  type Measured,
} from "../lib/align";
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
  /** The primary selection — carries the transform handles. */
  selectedItem: ItemId | null;
  /** The full multi-selection (includes the primary). CAP-M04 follow-on. */
  selectedItems: ItemId[];
  onSelect: (item: ItemId | null) => void;
  /** Shift-click: add/remove one item from the multi-selection. */
  onToggleSelect: (item: ItemId) => void;
  /** Marquee result: replace the selection with these items. */
  onSelectMany: (items: ItemId[]) => void;
  onItemTransform: (item: ItemId, transform: Transform) => void;
  /** Batch transform for align/distribute/group-move (one undo step). */
  onItemsTransform: (changes: { item: ItemId; transform: Transform }[], coalesce: boolean) => void;
  /** Preview alignment aids (CAP-M04): smart guides, safe areas, rulers. */
  alignment: AlignmentSettings;
};

/** Grab radius for a custom guide line, display px. */
const GUIDE_GRAB = 6;

/** Normalized canvas regions per backdrop split — mirrors `BackdropSplit::region()`. */
const BACKDROP_REGIONS: Record<BackdropSplit, { x: number; y: number; w: number; h: number }> = {
  full: { x: 0, y: 0, w: 1, h: 1 },
  left: { x: 0, y: 0, w: 0.5, h: 1 },
  right: { x: 0.5, y: 0, w: 0.5, h: 1 },
  top: { x: 0, y: 0, w: 1, h: 0.5 },
  bottom: { x: 0, y: 0.5, w: 1, h: 0.5 },
};
/** Persistent custom-guide color (distinct from the pink live snap guides). */
const GUIDE_COLOR = "#22d3ee";

/** Poll interval for the program frame pipe (~30 fps). */
const FRAME_POLL_MS = 33;
/** Handle hit radius, display px. */
const HANDLE_RADIUS = 7;
/** Rotate handle offset above the top edge, display px. */
const ROTATE_OFFSET = 22;
/** Smart-guide snap radius, display px (converted to canvas px per drag). */
const SNAP_RADIUS = 8;
/** Ruler gutter width/height, display px (reserved outside the native region). */
const RULER_SIZE = 16;

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
  /** The canvas bounds — the item may never move or scale past them. */
  canvas: Size;
  /** Other remote-guest boxes — a guest drag stops dead against them. */
  obstacles: Box[];
  /** For edge drags: which edge (0=left 1=right 2=top 3=bottom). */
  edge?: number;
  /** For corner drags: which corner index holds still (the anchor). */
  fixedIndex?: number;
  /** For rotate: the cursor's start angle minus the item's start rotation. */
  angleOffset?: number;
  /** Smart-guide snap context (move mode only; absent = snapping off). */
  snap?: { candidates: ReturnType<typeof snapCandidates>; threshold: number };
};

/** The CAP-M04-follow-on interactions, kept apart from the single-item
 * `DragState` so the delicate resize/rotate/crop math stays untouched. */
type AuxDrag =
  | { kind: "marquee"; start: Vec2; current: Vec2 }
  | {
      kind: "group";
      pointer: Vec2;
      canvas: Size;
      members: { id: ItemId; start: Transform; content: Size }[];
      groupBox: Box;
      snap?: { candidates: ReturnType<typeof snapCandidates>; threshold: number };
    }
  | { kind: "guide"; orientation: "v" | "h"; index: number; position: number };

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
  selectedItems,
  onSelect,
  onToggleSelect,
  onSelectMany,
  onItemTransform,
  onItemsTransform,
  alignment,
}: PreviewPanelProps) {
  const t = useT();
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [box, setBox] = useState({ left: 0, top: 0, width: 0, height: 0 });
  const dragRef = useRef<DragState | null>(null);
  // CAP-M04 follow-on: marquee / group-move / guide-drag, separate from dragRef.
  const auxRef = useRef<AuxDrag | null>(null);
  const [marquee, setMarquee] = useState<{ start: Vec2; current: Vec2 } | null>(null);
  const [guideDrag, setGuideDrag] = useState<{
    orientation: "v" | "h";
    position: number;
    index: number;
  } | null>(null);
  // Live smart-guide lines while a move drag is snapping (canvas px).
  const [liveGuides, setLiveGuides] = useState<Guide[]>([]);
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
  // Custom alignment guides for the active scene (CAP-M04 follow-on).
  const customGuides = useMemo<GuideLine[]>(() => scene?.guides ?? [], [scene]);
  // A modal dialog is a webview element; the native preview child window would
  // paint over it, so suppress the overlay while any modal is open.
  const modalOpen = useSyncExternalStore(modalSubscribe, isModalOpen);

  // The displayed box: the program frame letterboxed inside the container.
  // When rulers are on, a gutter is reserved on the top+left edges so the ruler
  // strips sit *outside* the box (and so outside the native preview region,
  // which would otherwise occlude them on Windows).
  useEffect(() => {
    const container = containerRef.current;
    if (!container) return;
    const gutter = alignment.rulers ? RULER_SIZE : 0;
    const recompute = () => {
      const rect = container.getBoundingClientRect();
      const availW = Math.max(1, rect.width - gutter);
      const availH = Math.max(1, rect.height - gutter);
      const scale = Math.min(availW / programW, availH / programH);
      const width = Math.max(1, programW * scale);
      const height = Math.max(1, programH * scale);
      setBox({
        left: gutter + (availW - width) / 2,
        top: gutter + (availH - height) / 2,
        width,
        height,
      });
    };
    recompute();
    const observer = new ResizeObserver(recompute);
    observer.observe(container);
    return () => observer.disconnect();
  }, [programW, programH, alignment.rulers]);

  // Is the native GPU preview viable? (Windows DX12 + overlay, not failed.) It
  // can flip false mid-session (surface build error, device lost), so re-poll
  // rather than checking once — when it drops, the JPEG canvas + poll return.
  useEffect(() => {
    let alive = true;
    const poll = () =>
      nativePreviewActive()
        .then((on) => alive && setNativeActive(on))
        .catch(() => alive && setNativeActive(false));
    void poll();
    const timer = setInterval(poll, 1000);
    return () => {
      alive = false;
      clearInterval(timer);
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
      const visible = running && !emptyScene && !modalOpen;
      const key = `${x},${y},${w},${h},${visible}`;
      if (key !== lastRegion.current) {
        lastRegion.current = key;
        void nativePreviewSetRegion(x, y, w, h, visible).catch(() => undefined);
      }
    };
    report();
    const timer = setInterval(report, 150);
    // Only stop the interval here — do NOT hide the region. This effect re-runs
    // on every `box` change (a resize spawns a new box object each tick), and
    // hiding on each re-run flashes the native preview blank. The hide lives in
    // the deactivate/unmount effect below, which only fires when nativeActive flips.
    return () => clearInterval(timer);
  }, [nativeActive, box, running, emptyScene, modalOpen]);

  // Hide the native overlay when it stops being viable or the panel unmounts,
  // decoupled from per-layout reporting so an ordinary resize never flashes it.
  useEffect(() => {
    if (!nativeActive) return;
    return () => {
      lastRegion.current = "";
      void nativePreviewSetRegion(0, 0, 0, 0, false).catch(() => undefined);
    };
  }, [nativeActive]);

  // Tell the native surface which item is selected, so it can draw the box +
  // handles *into* the GPU frame (they're hidden under the opaque surface).
  // Also fires with `null` on deselect. Only matters on the native path.
  useEffect(() => {
    if (!nativeActive) return;
    void nativePreviewSetSelection(selectedItem).catch(() => undefined);
  }, [nativeActive, selectedItem]);

  // Push the alignment overlay (safe areas + live snap guides) into the native
  // GPU frame, where the SVG below is occluded. Off the native path this is
  // skipped and the SVG renders the same model instead.
  useEffect(() => {
    if (!nativeActive) return;
    const safeAreas = alignment.safeAreas
      ? Object.values(safeAreaRects({ w: programW, h: programH }))
      : [];
    // Custom guides span the whole canvas; the dragged one tracks live.
    const custom = customGuides.map((guide, index) => ({
      orientation: guide.orientation,
      position: guideDrag && guideDrag.index === index ? guideDrag.position : guide.position,
      from: 0,
      to: guide.orientation === "v" ? programH : programW,
    }));
    const guides = [
      ...custom,
      ...liveGuides.map((guide) => ({
        orientation: guide.orientation,
        position: guide.position,
        from: guide.from,
        to: guide.to,
      })),
    ];
    void nativePreviewSetOverlay({ safeAreas, guides }).catch(() => undefined);
  }, [nativeActive, alignment.safeAreas, programW, programH, liveGuides, customGuides, guideDrag]);

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

  // OBS-style px-to-edge readout, shown only during a drag/resize.
  const [dragReadout, setDragReadout] = useState<{
    left: number;
    top: number;
    right: number;
    bottom: number;
  } | null>(null);

  /** Other visible remote-guest boxes — a guest may never overlap a guest. */
  const guestObstacles = useCallback(
    (moving: SceneItem): Box[] => {
      if (!scene || !collection) return [];
      const kindOf = (item: SceneItem) =>
        collection.sources.find((source) => source.id === item.source)?.kind;
      if (kindOf(moving) !== "remoteGuest") return [];
      return scene.items
        .filter((item) => item.id !== moving.id && item.visible && kindOf(item) === "remoteGuest")
        .flatMap((item) => {
          const source = sourceSize(item);
          if (!source) return [];
          const content = contentSize(source.w, source.h, item.transform.crop);
          if (!content) return [];
          return [boundsOf(item.transform, content)];
        });
    },
    [scene, collection, sourceSize],
  );

  /** Smart-guide snap context for a move drag: every *other* visible item's
   * box as a snap target, plus the canvas. Absent when snapping is off. */
  const buildSnap = useCallback(
    (moving: SceneItem): DragState["snap"] => {
      if (!alignment.smartGuides || !scene) return undefined;
      const others: Box[] = scene.items
        .filter((item) => item.id !== moving.id && item.visible)
        .flatMap((item) => {
          const source = sourceSize(item);
          if (!source) return [];
          const content = contentSize(source.w, source.h, item.transform.crop);
          if (!content) return [];
          return [boundsOf(item.transform, content)];
        });
      const canvas: Size = { w: programW, h: programH };
      const candidates = snapCandidates(canvas, others);
      // Custom guides are snap targets too (CAP-M04 follow-on).
      for (const guide of scene.guides ?? []) {
        if (guide.orientation === "v") {
          candidates.v.push({ pos: guide.position, lo: 0, hi: programH, target: "item" });
        } else {
          candidates.h.push({ pos: guide.position, lo: 0, hi: programW, target: "item" });
        }
      }
      return {
        candidates,
        threshold: SNAP_RADIUS / Math.max(displayScale, 1e-6),
      };
    },
    [alignment.smartGuides, scene, sourceSize, programW, programH, displayScale],
  );

  /** An item's rotation-aware bounding box in canvas px, or null if unsized. */
  const itemBox = useCallback(
    (item: SceneItem): Box | null => {
      const source = sourceSize(item);
      if (!source) return null;
      const content = contentSize(source.w, source.h, item.transform.crop);
      if (!content) return null;
      return boundsOf(item.transform, content);
    },
    [sourceSize],
  );

  /** The multi-selection measured for align/distribute (unsized items dropped). */
  const measuredSelection = useCallback((): Measured[] => {
    if (!scene) return [];
    return selectedItems.flatMap((id) => {
      const item = scene.items.find((i) => i.id === id);
      if (!item) return [];
      const box = itemBox(item);
      return box ? [{ id, transform: item.transform, box }] : [];
    });
  }, [scene, selectedItems, itemBox]);

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

  // Restore the move affordance the instant an item is selected (keyboard, or a
  // click that doesn't move) — updateDrag refines the per-handle cursor on the
  // first pointer move. Keyed on the item id only, so an ordinary scene tick
  // never resets a hovered handle cursor out from under the pointer.
  useEffect(() => {
    setHoverCursor(selected && !selected.locked ? "move" : "default");
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [selectedItem]);

  // -- pointer interactions ---------------------------------------------------

  /** The index of the custom guide within grab range of `p`, or null. */
  const hitGuide = (p: Vec2): number | null => {
    const threshold = GUIDE_GRAB / Math.max(displayScale, 1e-6);
    let best: number | null = null;
    let bestDist = threshold;
    customGuides.forEach((guide, index) => {
      const dist =
        guide.orientation === "v" ? Math.abs(p.x - guide.position) : Math.abs(p.y - guide.position);
      if (dist <= bestDist) {
        bestDist = dist;
        best = index;
      }
    });
    return best;
  };

  /** Snap context for a group move: every *non-selected* visible item, the
   * canvas, and custom guides. */
  const buildGroupSnap = (ids: ItemId[]): Extract<AuxDrag, { kind: "group" }>["snap"] => {
    if (!alignment.smartGuides || !scene) return undefined;
    const set = new Set(ids);
    const others: Box[] = scene.items
      .filter((item) => !set.has(item.id) && item.visible)
      .flatMap((item) => {
        const box = itemBox(item);
        return box ? [box] : [];
      });
    const candidates = snapCandidates({ w: programW, h: programH }, others);
    for (const guide of customGuides) {
      if (guide.orientation === "v") {
        candidates.v.push({ pos: guide.position, lo: 0, hi: programH, target: "item" });
      } else {
        candidates.h.push({ pos: guide.position, lo: 0, hi: programW, target: "item" });
      }
    }
    return { candidates, threshold: SNAP_RADIUS / Math.max(displayScale, 1e-6) };
  };

  /** Assemble a group-move drag from the current multi-selection at pointer `p`. */
  const beginGroupMove = (p: Vec2): Extract<AuxDrag, { kind: "group" }> | null => {
    if (!scene) return null;
    const members = selectedItems.flatMap((id) => {
      const item = scene.items.find((i) => i.id === id);
      if (!item || item.locked) return [];
      const source = sourceSize(item);
      if (!source) return [];
      const content = contentSize(source.w, source.h, item.transform.crop);
      if (!content) return [];
      return [{ id, start: item.transform, content, box: boundsOf(item.transform, content) }];
    });
    if (members.length === 0) return null;
    return {
      kind: "group",
      pointer: p,
      canvas: { w: programW, h: programH },
      members: members.map(({ id, start, content }) => ({ id, start, content })),
      groupBox: unionBoxes(members.map((m) => m.box)),
      snap: buildGroupSnap(members.map((m) => m.id)),
    };
  };

  /** Advance a marquee / guide / group aux drag. */
  const updateAux = (aux: AuxDrag, p: Vec2, shift: boolean) => {
    if (aux.kind === "marquee") {
      aux.current = p;
      setMarquee({ start: aux.start, current: p });
      return;
    }
    if (aux.kind === "guide") {
      aux.position = aux.orientation === "v" ? p.x : p.y;
      setGuideDrag({ orientation: aux.orientation, position: aux.position, index: aux.index });
      return;
    }
    let dx = p.x - aux.pointer.x;
    let dy = p.y - aux.pointer.y;
    ({ dx, dy } = clampMoveDelta(aux.groupBox, dx, dy, aux.canvas));
    if (aux.snap && !shift) {
      const moved: Box = {
        minX: aux.groupBox.minX + dx,
        maxX: aux.groupBox.maxX + dx,
        minY: aux.groupBox.minY + dy,
        maxY: aux.groupBox.maxY + dy,
      };
      const snap = snapMove(moved, aux.snap.candidates, aux.snap.threshold);
      let sdx = snap.dx;
      let sdy = snap.dy;
      ({ dx: sdx, dy: sdy } = clampMoveDelta(moved, sdx, sdy, aux.canvas));
      dx += sdx;
      dy += sdy;
      setLiveGuides(snap.guides);
    }
    onItemsTransform(
      aux.members.map((m) => ({
        item: m.id,
        transform: { ...m.start, x: m.start.x + dx, y: m.start.y + dy },
      })),
      true,
    );
  };

  /** Commit an aux drag on pointer-up. */
  const finishAux = (aux: AuxDrag, p: Vec2, shift: boolean) => {
    if (aux.kind === "marquee") {
      const dragged = Math.abs(p.x - aux.start.x) > 3 || Math.abs(p.y - aux.start.y) > 3;
      if (!dragged) {
        onSelect(null); // a click on empty space clears the selection
        return;
      }
      const rect: Box = {
        minX: Math.min(aux.start.x, p.x),
        maxX: Math.max(aux.start.x, p.x),
        minY: Math.min(aux.start.y, p.y),
        maxY: Math.max(aux.start.y, p.y),
      };
      const hits = (scene?.items ?? [])
        .filter((item) => item.visible)
        .filter((item) => {
          const box = itemBox(item);
          return box ? intersects(rect, box) : false;
        })
        .map((item) => item.id);
      onSelectMany(hits);
      return;
    }
    if (aux.kind === "guide") {
      if (!scene) return;
      const pos = aux.orientation === "v" ? p.x : p.y;
      const within =
        aux.orientation === "v" ? pos >= 0 && pos <= programW : pos >= 0 && pos <= programH;
      // Dragged off the canvas → delete the guide.
      const next = within
        ? customGuides.map((g, i) => (i === aux.index ? { ...g, position: Math.round(pos) } : g))
        : customGuides.filter((_, i) => i !== aux.index);
      studioSetGuides(scene.id, next).catch((err) => console.error("set guides failed:", err));
      return;
    }
    updateAux(aux, p, shift); // group: land the final position
  };

  /** Add a custom guide down the middle of the canvas; drag it from there. */
  const addGuide = (orientation: "v" | "h") => {
    if (!scene) return;
    const position = orientation === "v" ? programW / 2 : programH / 2;
    studioSetGuides(scene.id, [...customGuides, { orientation, position }]).catch((err) =>
      console.error("add guide failed:", err),
    );
  };

  /** Align the multi-selection to each other (one undo step). */
  const arrangeSelected = (edge: AlignEdge) => {
    const changes = alignItems(measuredSelection(), edge);
    if (changes.size > 0) {
      onItemsTransform(
        [...changes].map(([item, transform]) => ({ item, transform })),
        false,
      );
    }
  };

  /** Distribute the multi-selection evenly along an axis (one undo step). */
  const distributeSelected = (axis: DistributeAxis) => {
    const changes = distributeItems(measuredSelection(), axis);
    if (changes.size > 0) {
      onItemsTransform(
        [...changes].map(([item, transform]) => ({ item, transform })),
        false,
      );
    }
  };

  const beginDrag = (event: React.PointerEvent) => {
    if (!scene) return;
    const p = toProgram(event);

    // Shift-click toggles an item in/out of the multi-selection — handled first,
    // so it works on the primary item too (whose handles would otherwise win).
    if (event.shiftKey) {
      for (let index = scene.items.length - 1; index >= 0; index -= 1) {
        const item = scene.items[index];
        // The backdrop wallpaper is not clickable — it can never block
        // selecting or dragging the capture above it.
        if (item.backdrop || !item.visible) continue;
        const source = sourceSize(item);
        if (!source) continue;
        const content = contentSize(source.w, source.h, item.transform.crop);
        if (!content) continue;
        if (hitTest(item.transform, content, p)) {
          onToggleSelect(item.id);
          event.preventDefault();
          return;
        }
      }
      return; // shift on empty space keeps the current selection
    }

    // Grab a handle of the selected item — only in a single selection, so that
    // in a multi-selection clicking the primary group-moves like any member.
    if (selected && selectedGeometry && !selected.locked && selectedItems.length <= 1) {
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
        canvas: { w: programW, h: programH },
        obstacles: guestObstacles(selected),
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
            fixedIndex: 3 - cornerHit,
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
            dragRef.current = {
              ...start,
              mode: "move",
              fixed: { x: t.x, y: t.y },
              snap: buildSnap(selected),
            };
          }
        }
      }
      if (dragRef.current) {
        (event.target as Element).setPointerCapture(event.pointerId);
        event.preventDefault();
        return;
      }
    }

    // A custom guide line under the cursor grabs for a drag (CAP-M04 follow-on).
    const guideIndex = hitGuide(p);
    if (guideIndex !== null) {
      const guide = customGuides[guideIndex];
      auxRef.current = {
        kind: "guide",
        orientation: guide.orientation,
        index: guideIndex,
        position: guide.position,
      };
      setGuideDrag({ orientation: guide.orientation, position: guide.position, index: guideIndex });
      (event.target as Element).setPointerCapture(event.pointerId);
      event.preventDefault();
      return;
    }

    // Otherwise: hit-test items top-down (top = last). The backdrop
    // wallpaper is skipped — clicks land on the capture or empty canvas.
    for (let index = scene.items.length - 1; index >= 0; index -= 1) {
      const item = scene.items[index];
      if (item.backdrop || !item.visible) continue;
      const source = sourceSize(item);
      if (!source) continue;
      const content = contentSize(source.w, source.h, item.transform.crop);
      if (!content) continue;
      if (hitTest(item.transform, content, p)) {
        // Clicking an item already in a multi-selection drags the whole group.
        if (selectedItems.length > 1 && selectedItems.includes(item.id)) {
          const group = beginGroupMove(p);
          if (group) {
            auxRef.current = group;
            (event.target as Element).setPointerCapture(event.pointerId);
            event.preventDefault();
            return;
          }
        }
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
            canvas: { w: programW, h: programH },
            obstacles: guestObstacles(item),
            snap: buildSnap(item),
          };
          (event.target as Element).setPointerCapture(event.pointerId);
        }
        event.preventDefault();
        return;
      }
    }

    // Empty space → a rubber-band marquee select.
    auxRef.current = { kind: "marquee", start: p, current: p };
    setMarquee({ start: p, current: p });
    (event.target as Element).setPointerCapture(event.pointerId);
    event.preventDefault();
  };

  const updateDrag = (event: React.PointerEvent) => {
    const p = toProgram(event);
    const aux = auxRef.current;
    if (aux) {
      updateAux(aux, p, event.shiftKey);
      return;
    }
    const drag = dragRef.current;
    if (!drag) {
      setHoverCursor(cursorFor(p));
      return;
    }
    let next = applyDrag(drag, p, event.shiftKey);
    if (!next) return;
    // Smart guides: snap the moved box onto the nearest canvas/other-item line
    // (Shift bypasses). Snapping runs after the canvas clamp, so an item never
    // leaves the frame; slideIntoCanvas re-seats it if a snap nudged it out.
    if (drag.mode === "move" && drag.snap && !event.shiftKey) {
      const c = contentSize(drag.source.w, drag.source.h, next.crop) ?? drag.content;
      const snap = snapMove(boundsOf(next, c), drag.snap.candidates, drag.snap.threshold);
      if (snap.dx !== 0 || snap.dy !== 0) {
        next = slideIntoCanvas(
          { ...next, x: next.x + snap.dx, y: next.y + snap.dy },
          c,
          drag.canvas,
        );
      }
      setLiveGuides(snap.guides);
    }
    onItemTransform(drag.itemId, next);
    const content = contentSize(drag.source.w, drag.source.h, next.crop) ?? drag.content;
    setDragReadout(edgeDistances(boundsOf(next, content), drag.canvas));
  };

  const endDrag = (event: React.PointerEvent) => {
    const aux = auxRef.current;
    if (aux) {
      finishAux(aux, toProgram(event), event.shiftKey);
      auxRef.current = null;
      setMarquee(null);
      setGuideDrag(null);
      setLiveGuides([]);
      return;
    }
    if (dragRef.current) {
      updateDrag(event);
      dragRef.current = null;
      setDragReadout(null);
      setLiveGuides([]);
    }
  };

  // Mouse zoom (wheel over the canvas): punch into the top-most visible
  // unlocked item under the cursor, zooming about the pointer. With no item
  // hit, the backdrop wallpaper zooms instead — its transform is zoom/pan
  // within its region, mirrored here for smooth steps and clamped again
  // engine-side, so blank canvas can never show no matter what.
  const zoomAt = (event: React.WheelEvent) => {
    if (!scene || dragRef.current) return;
    const p = toProgram(event);
    const factor = event.deltaY < 0 ? 1.1 : 1 / 1.1;
    for (let index = scene.items.length - 1; index >= 0; index -= 1) {
      const item = scene.items[index];
      if (item.backdrop || !item.visible || item.locked) continue;
      const source = sourceSize(item);
      if (!source) continue;
      const content = contentSize(source.w, source.h, item.transform.crop);
      if (!content) continue;
      if (!hitTest(item.transform, content, p)) continue;
      // Punch-in lens (CAP-N71): the wheel zooms a runtime, spring-smoothed
      // lens about the cursor — the item's saved transform (and the undo
      // history) never change.
      const local = canvasToLocal(item.transform, content, p);
      if (!local) return;
      studioZoomScroll(item.id, factor, local.x / content.w, local.y / content.h).catch((err) =>
        console.error("zoom lens failed:", err),
      );
      return;
    }
    const backdrop = scene.items.find((item) => item.backdrop && item.visible);
    if (!backdrop) return;
    const source = sourceSize(backdrop);
    if (!source) return;
    const region = BACKDROP_REGIONS[backdrop.backdrop ?? "full"];
    const rx = region.x * programW;
    const ry = region.y * programH;
    const rw = region.w * programW;
    const rh = region.h * programH;
    if (p.x < rx || p.x > rx + rw || p.y < ry || p.y > ry + rh) return;
    const contain = backdrop.backdrop !== "full";
    const base = contain
      ? Math.min(rw / source.w, rh / source.h)
      : Math.max(rw / source.w, rh / source.h);
    const t = backdrop.transform;
    const zoom0 = Math.min(Math.max(t.scaleX, 1), 8);
    const zoom1 = Math.min(Math.max(zoom0 * factor, 1), 8);
    if (zoom1 === zoom0 && factor !== 1) return; // already at a zoom stop
    const clampPan = (pan: number, max: number) => Math.min(Math.max(pan, -max), max);
    const bound = (zoom: number, size: number, span: number) =>
      Math.max((size * base * zoom - span) * 0.5, 0);
    const cx = rx + rw * 0.5 + clampPan(t.x, bound(zoom0, source.w, rw));
    const cy = ry + rh * 0.5 + clampPan(t.y, bound(zoom0, source.h, rh));
    const f = zoom1 / zoom0;
    const pan1x = clampPan(p.x + (cx - p.x) * f - (rx + rw * 0.5), bound(zoom1, source.w, rw));
    const pan1y = clampPan(p.y + (cy - p.y) * f - (ry + rh * 0.5), bound(zoom1, source.h, rh));
    onItemTransform(backdrop.id, {
      ...t,
      x: pan1x,
      y: pan1y,
      scaleX: zoom1,
      scaleY: zoom1,
    });
  };

  // Highlight Speaker, "click the slot": double-click an item to spotlight it
  // (fill the canvas); double-click it again — or empty space — to restore.
  const toggleFocusAt = (event: React.MouseEvent) => {
    if (!scene) return;
    const p = toProgram(event);
    for (let index = scene.items.length - 1; index >= 0; index -= 1) {
      const item = scene.items[index];
      // The backdrop wallpaper can't be spotlighted (and never eats the
      // double-click meant for the capture above it).
      if (item.backdrop || !item.visible) continue;
      const source = sourceSize(item);
      if (!source) continue;
      const content = contentSize(source.w, source.h, item.transform.crop);
      if (!content) continue;
      if (hitTest(item.transform, content, p)) {
        const next = scene.focus?.item === item.id ? null : item.id;
        studioSetFocus(scene.id, next).catch((err) => console.error("focus toggle failed:", err));
        return;
      }
    }
    if (scene.focus) {
      studioSetFocus(scene.id, null).catch((err) => console.error("focus restore failed:", err));
    }
  };

  // Align the selected item's bounding box to a canvas edge/center (CAP-M04).
  const alignSelected = useCallback(
    (edge: AlignEdge) => {
      if (!selected || !program) return;
      const status = program.sources[selected.source];
      if (!status?.width || !status?.height) return;
      const next = alignToCanvas(
        selected,
        status.width,
        status.height,
        { w: programW, h: programH },
        edge,
      );
      if (next) onItemTransform(selected.id, next);
    },
    [selected, program, programW, programH, onItemTransform],
  );

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

  // Alignment overlay in display px for the SVG path (safe-area rects + live
  // smart-guide lines). Occluded on the native path — pushed there separately.
  const alignmentOverlay = useMemo(() => {
    const s = displayScale;
    const safe: Rect[] = alignment.safeAreas
      ? Object.values(safeAreaRects({ w: programW, h: programH })).map((r) => ({
          x: r.x * s,
          y: r.y * s,
          w: r.w * s,
          h: r.h * s,
        }))
      : [];
    const guides = liveGuides.map((guide) => ({
      orientation: guide.orientation,
      position: guide.position * s,
      from: guide.from * s,
      to: guide.to * s,
    }));
    return { safe, guides };
  }, [alignment.safeAreas, programW, programH, liveGuides, displayScale]);

  // Outlines for the non-primary members of a multi-selection (display px). The
  // primary still draws its full handle overlay. CAP-M04 follow-on.
  const selectionBoxes = useMemo(() => {
    if (!scene || selectedItems.length < 2) return [];
    return selectedItems.flatMap((id) => {
      if (id === selectedItem) return [];
      const item = scene.items.find((i) => i.id === id);
      if (!item) return [];
      const box = itemBox(item);
      if (!box) return [];
      return [
        {
          x: box.minX * displayScale,
          y: box.minY * displayScale,
          w: (box.maxX - box.minX) * displayScale,
          h: (box.maxY - box.minY) * displayScale,
        },
      ];
    });
  }, [scene, selectedItems, selectedItem, itemBox, displayScale]);

  // Custom guides in display px, with the live position for the dragged one.
  const displayGuides = useMemo(
    () =>
      customGuides.map((guide, index) => ({
        orientation: guide.orientation,
        position:
          (guideDrag && guideDrag.index === index ? guideDrag.position : guide.position) *
          displayScale,
      })),
    [customGuides, guideDrag, displayScale],
  );

  return (
    <section
      aria-label={t("preview-program-label")}
      className="relative flex min-h-0 min-w-0 flex-col overflow-hidden rounded-xl border border-white/10 bg-black/60"
    >
      <div ref={containerRef} className="relative min-h-0 flex-1">
        {running && (
          <>
            {!nativeActive && (
              <canvas
                ref={canvasRef}
                role="img"
                aria-label={t("preview-program-output")}
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
              aria-label={t("preview-canvas-editor")}
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
              onWheel={zoomAt}
              onDoubleClick={toggleFocusAt}
            >
              {alignmentOverlay.safe.map((r, index) => (
                <rect
                  key={`safe-${index}`}
                  x={r.x}
                  y={r.y}
                  width={r.w}
                  height={r.h}
                  fill="none"
                  stroke={index === 0 ? "#ffffff66" : "#ffffff3a"}
                  strokeWidth={1}
                  strokeDasharray="6 4"
                />
              ))}
              {alignmentOverlay.guides.map((guide, index) =>
                guide.orientation === "v" ? (
                  <line
                    key={`guide-${index}`}
                    x1={guide.position}
                    y1={guide.from}
                    x2={guide.position}
                    y2={guide.to}
                    stroke="#ff3ea5"
                    strokeWidth={1}
                  />
                ) : (
                  <line
                    key={`guide-${index}`}
                    x1={guide.from}
                    y1={guide.position}
                    x2={guide.to}
                    y2={guide.position}
                    stroke="#ff3ea5"
                    strokeWidth={1}
                  />
                ),
              )}
              {displayGuides.map((guide, index) =>
                guide.orientation === "v" ? (
                  <line
                    key={`cg-${index}`}
                    x1={guide.position}
                    y1={0}
                    x2={guide.position}
                    y2={box.height}
                    stroke={GUIDE_COLOR}
                    strokeWidth={1}
                  />
                ) : (
                  <line
                    key={`cg-${index}`}
                    x1={0}
                    y1={guide.position}
                    x2={box.width}
                    y2={guide.position}
                    stroke={GUIDE_COLOR}
                    strokeWidth={1}
                  />
                ),
              )}
              {selectionBoxes.map((b, index) => (
                <rect
                  key={`sel-${index}`}
                  x={b.x}
                  y={b.y}
                  width={b.w}
                  height={b.h}
                  fill="none"
                  stroke="#4a9eff"
                  strokeWidth={1}
                  strokeDasharray="3 3"
                  opacity={0.8}
                />
              ))}
              {marquee && (
                <rect
                  x={Math.min(marquee.start.x, marquee.current.x) * displayScale}
                  y={Math.min(marquee.start.y, marquee.current.y) * displayScale}
                  width={Math.abs(marquee.current.x - marquee.start.x) * displayScale}
                  height={Math.abs(marquee.current.y - marquee.start.y) * displayScale}
                  fill="#4a9eff22"
                  stroke="#4a9eff"
                  strokeWidth={1}
                  strokeDasharray="4 2"
                />
              )}
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
        {running && alignment.rulers && <Rulers box={box} canvasW={programW} canvasH={programH} />}
        {dragReadout && (
          <div
            className="pointer-events-none absolute z-10 rounded-md border border-white/10 bg-black/75 px-2 py-1 font-mono text-[10px] leading-tight text-havoc-text"
            style={{ left: box.left + 8, top: box.top + 8 }}
            role="status"
            aria-label={t("preview-px-to-edge-label")}
          >
            {t("preview-px-to-edge", {
              left: dragReadout.left,
              top: dragReadout.top,
              right: dragReadout.right,
              bottom: dragReadout.bottom,
            })}
          </div>
        )}
        {!running || emptyScene ? (
          <div className="pointer-events-none absolute inset-0 flex items-center justify-center p-6">
            <div className="flex max-w-md flex-col items-center gap-2 text-center">
              <span className="bg-gradient-to-r from-havoc-accent to-havoc-accent-2 bg-clip-text text-sm font-semibold tracking-widest text-transparent uppercase">
                {t("preview-program-heading")}
              </span>
              {program?.state === "noGpu" ? (
                <p className="m-0 text-xs text-red-400" role="alert">
                  {t("preview-no-gpu")}
                  <span className="mt-1 block text-havoc-muted">{program.adapter}</span>
                </p>
              ) : !running ? (
                <p className="m-0 text-xs text-havoc-muted" role="status">
                  {t("preview-starting-compositor")}
                </p>
              ) : (
                <p className="m-0 text-xs text-havoc-muted">{t("preview-empty-scene")}</p>
              )}
            </div>
          </div>
        ) : null}
      </div>
      {running && program && (
        <div className="flex shrink-0 items-center gap-3 border-t border-white/5 bg-black/40 px-3 py-1 text-[11px] text-havoc-muted">
          {selectedItems.length >= 2 ? (
            <ArrangeBar
              onArrange={arrangeSelected}
              onDistribute={distributeSelected}
              canDistribute={selectedItems.length >= 3}
              t={t}
            />
          ) : (
            selected && !selected.locked && <AlignBar onAlign={alignSelected} t={t} />
          )}
          <GuideButtons onAdd={addGuide} t={t} />
          <span className="flex items-center gap-1.5">
            <span className="h-1.5 w-1.5 rounded-full bg-emerald-400" aria-hidden="true" />
            {program.width}×{program.height}
          </span>
          <span>{t("preview-fps", { fps: program.fps })}</span>
          {program.dropped > 0 && (
            <span className="text-amber-300">
              {t("preview-dropped", { dropped: program.dropped })}
            </span>
          )}
          <span className="ml-auto max-w-64 truncate" title={program.adapter}>
            {program.adapter}
          </span>
        </div>
      )}
    </section>
  );
}

// ---------------------------------------------------------------------------
// Align-to-canvas bar (CAP-M04): lives in the footer, outside the native region
// ---------------------------------------------------------------------------

/** A 12×12 pictogram: the canvas frame + the item bar snapped to `edge`. */
function alignGlyph(edge: AlignEdge) {
  const bar: Record<AlignEdge, { x: number; y: number; w: number; h: number }> = {
    left: { x: 2, y: 3, w: 3, h: 6 },
    hcenter: { x: 4.5, y: 3, w: 3, h: 6 },
    right: { x: 7, y: 3, w: 3, h: 6 },
    top: { x: 3, y: 2, w: 6, h: 3 },
    vcenter: { x: 3, y: 4.5, w: 6, h: 3 },
    bottom: { x: 3, y: 7, w: 6, h: 3 },
  };
  const b = bar[edge];
  return (
    <svg width={14} height={14} viewBox="0 0 12 12" aria-hidden="true">
      <rect x={1} y={1} width={10} height={10} fill="none" stroke="currentColor" opacity={0.45} />
      <rect x={b.x} y={b.y} width={b.w} height={b.h} fill="currentColor" />
    </svg>
  );
}

/** Six align-to-canvas buttons (left/center/right, top/middle/bottom). */
function AlignBar({
  onAlign,
  t,
}: {
  onAlign: (edge: AlignEdge) => void;
  t: (key: string) => string;
}) {
  return (
    <div className="flex items-center gap-0.5" role="group" aria-label={t("align-group")}>
      {ALIGN_EDGES.map((edge) => (
        <button
          key={edge}
          type="button"
          title={t(`align-${edge}`)}
          aria-label={t(`align-${edge}`)}
          onClick={() => onAlign(edge)}
          className="flex h-5 w-5 items-center justify-center rounded text-havoc-muted hover:text-havoc-text"
        >
          {alignGlyph(edge)}
        </button>
      ))}
    </div>
  );
}

/** A 12×12 pictogram: three bars distributed along `axis`. */
function distributeGlyph(axis: DistributeAxis) {
  return (
    <svg width={14} height={14} viewBox="0 0 12 12" aria-hidden="true">
      {axis === "h" ? (
        <>
          <rect x={1} y={3} width={2} height={6} fill="currentColor" />
          <rect x={5} y={3} width={2} height={6} fill="currentColor" />
          <rect x={9} y={3} width={2} height={6} fill="currentColor" />
        </>
      ) : (
        <>
          <rect x={3} y={1} width={6} height={2} fill="currentColor" />
          <rect x={3} y={5} width={6} height={2} fill="currentColor" />
          <rect x={3} y={9} width={6} height={2} fill="currentColor" />
        </>
      )}
    </svg>
  );
}

/** Align-to-each-other + distribute, shown when 2+ items are selected (CAP-M04
 * follow-on). Distribute needs 3+, so it's disabled with only two. */
function ArrangeBar({
  onArrange,
  onDistribute,
  canDistribute,
  t,
}: {
  onArrange: (edge: AlignEdge) => void;
  onDistribute: (axis: DistributeAxis) => void;
  canDistribute: boolean;
  t: (key: string) => string;
}) {
  return (
    <div className="flex items-center gap-0.5" role="group" aria-label={t("arrange-group")}>
      {ALIGN_EDGES.map((edge) => (
        <button
          key={edge}
          type="button"
          title={t(`arrange-${edge}`)}
          aria-label={t(`arrange-${edge}`)}
          onClick={() => onArrange(edge)}
          className="flex h-5 w-5 items-center justify-center rounded text-havoc-muted hover:text-havoc-text"
        >
          {alignGlyph(edge)}
        </button>
      ))}
      <span className="mx-0.5 h-4 w-px bg-white/10" aria-hidden="true" />
      {(["h", "v"] as const).map((axis) => (
        <button
          key={axis}
          type="button"
          disabled={!canDistribute}
          title={t(`distribute-${axis}`)}
          aria-label={t(`distribute-${axis}`)}
          onClick={() => onDistribute(axis)}
          className="flex h-5 w-5 items-center justify-center rounded text-havoc-muted enabled:hover:text-havoc-text disabled:opacity-40"
        >
          {distributeGlyph(axis)}
        </button>
      ))}
    </div>
  );
}

/** Add a vertical / horizontal custom guide (CAP-M04 follow-on). */
function GuideButtons({
  onAdd,
  t,
}: {
  onAdd: (orientation: "v" | "h") => void;
  t: (key: string) => string;
}) {
  return (
    <div className="flex items-center gap-0.5" role="group" aria-label={t("guides-group")}>
      {(["v", "h"] as const).map((orientation) => (
        <button
          key={orientation}
          type="button"
          title={t(`guides-add-${orientation}`)}
          aria-label={t(`guides-add-${orientation}`)}
          onClick={() => onAdd(orientation)}
          className="flex h-5 w-5 items-center justify-center rounded text-havoc-muted hover:text-havoc-text"
        >
          <svg width={14} height={14} viewBox="0 0 12 12" aria-hidden="true">
            <rect
              x={1}
              y={1}
              width={10}
              height={10}
              fill="none"
              stroke="currentColor"
              opacity={0.4}
            />
            {orientation === "v" ? (
              <line x1={6} y1={1} x2={6} y2={11} stroke={GUIDE_COLOR} strokeWidth={1.5} />
            ) : (
              <line x1={1} y1={6} x2={11} y2={6} stroke={GUIDE_COLOR} strokeWidth={1.5} />
            )}
          </svg>
        </button>
      ))}
    </div>
  );
}

// ---------------------------------------------------------------------------
// Rulers (CAP-M04): px ticks in the reserved top/left gutter
// ---------------------------------------------------------------------------

const RULER_FRACS = [0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1];

/** Px rulers along the top and left edges of the preview box, drawn in the
 * reserved gutter (outside the box, so never occluded by the native surface).
 * Ticks every 10% of the canvas; labelled every 20%. */
function Rulers({
  box,
  canvasW,
  canvasH,
}: {
  box: { left: number; top: number; width: number; height: number };
  canvasW: number;
  canvasH: number;
}) {
  return (
    <svg className="pointer-events-none absolute inset-0 h-full w-full" aria-hidden="true">
      {RULER_FRACS.map((frac) => {
        const major = Math.round(frac * 10) % 2 === 0;
        const len = major ? 6 : 3;
        const x = box.left + frac * box.width;
        const y = box.top + frac * box.height;
        return (
          <g key={frac}>
            <line
              x1={x}
              y1={RULER_SIZE - len}
              x2={x}
              y2={RULER_SIZE}
              stroke="#ffffff55"
              strokeWidth={1}
            />
            {major && (
              <text x={x} y={RULER_SIZE - 8} textAnchor="middle" fontSize={7} fill="#ffffff99">
                {Math.round(frac * canvasW)}
              </text>
            )}
            <line
              x1={RULER_SIZE - len}
              y1={y}
              x2={RULER_SIZE}
              y2={y}
              stroke="#ffffff55"
              strokeWidth={1}
            />
            {major && (
              <text
                x={RULER_SIZE - 8}
                y={y}
                textAnchor="middle"
                fontSize={7}
                fill="#ffffff99"
                transform={`rotate(-90 ${RULER_SIZE - 8} ${y})`}
              >
                {Math.round(frac * canvasH)}
              </text>
            )}
          </g>
        );
      })}
    </svg>
  );
}

// ---------------------------------------------------------------------------
// Drag math (program-pixel space; mirrors transform.rs semantics)
// ---------------------------------------------------------------------------

function distance(a: Vec2, b: Vec2): number {
  return Math.hypot(a.x - b.x, a.y - b.y);
}

/** The smallest box containing all of `boxes` (caller guarantees non-empty). */
function unionBoxes(boxes: Box[]): Box {
  return boxes.reduce((acc, b) => ({
    minX: Math.min(acc.minX, b.minX),
    minY: Math.min(acc.minY, b.minY),
    maxX: Math.max(acc.maxX, b.maxX),
    maxY: Math.max(acc.maxY, b.maxY),
  }));
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

/** A guest transform that would overlap another guest is refused — the drag
 * freezes at the last valid state ("hits the outside bounds → no further"). */
function guestBlocked(
  drag: DragState,
  next: Transform,
  content: { w: number; h: number },
): boolean {
  if (drag.obstacles.length === 0) return false;
  const box = boundsOf(next, content);
  return drag.obstacles.some((obstacle) => intersects(box, obstacle));
}

/** Compute the dragged transform. Returns null when the drag is degenerate
 * (or refused — a guest may never overlap a guest). Every mode ends inside
 * the canvas: moves clamp, sizes cap, the rest slides back in. */
function applyDrag(drag: DragState, p: Vec2, shift: boolean): Transform | null {
  const t = drag.start;
  switch (drag.mode) {
    case "move": {
      const startBox = boundsOf(t, drag.content);
      let dx = p.x - drag.pointer.x;
      let dy = p.y - drag.pointer.y;
      ({ dx, dy } = clampMoveDelta(startBox, dx, dy, drag.canvas));
      ({ dx, dy } = clampMoveAgainstObstacles(startBox, dx, dy, drag.obstacles));
      return { ...t, x: drag.fixed.x + dx, y: drag.fixed.y + dy };
    }
    case "rotate": {
      const angle = (Math.atan2(p.y - t.y, p.x - t.x) * 180) / Math.PI;
      let rotation = angle - (drag.angleOffset ?? 0);
      if (shift) rotation = Math.round(rotation / 15) * 15;
      // Normalize to (-180, 180] for readable numbers.
      rotation = ((((rotation + 180) % 360) + 360) % 360) - 180;
      // A rotation can widen the bounding box past the frame: shrink to fit
      // (aspect kept), then slide back inside.
      let next: Transform = { ...t, rotation };
      next = { ...next, ...clampScalesToCanvas(next, drag.content, drag.canvas, true) };
      next = slideIntoCanvas(next, drag.content, drag.canvas);
      return guestBlocked(drag, next, drag.content) ? null : next;
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
      // Cap the size at the frame. When the cap engages, re-anchor on the
      // fixed corner so growth stops dead instead of drifting.
      const raw = { scaleX, scaleY };
      const capped = clampScalesToCanvas(
        { ...t, scaleX, scaleY },
        drag.content,
        drag.canvas,
        !shift,
      );
      let next: Transform = { ...t, x: center.x, y: center.y, ...capped };
      if (capped.scaleX !== raw.scaleX || capped.scaleY !== raw.scaleY) {
        const f = drag.fixedIndex ?? 3;
        const signX = f === 0 || f === 2 ? 1 : -1;
        const signY = f === 0 || f === 1 ? 1 : -1;
        const hx = ((drag.content.w * capped.scaleX) / 2) * signX;
        const hy = ((drag.content.h * capped.scaleY) / 2) * signY;
        next = {
          ...next,
          x: drag.fixed.x + cos * hx - sin * hy,
          y: drag.fixed.y + sin * hx + cos * hy,
        };
      }
      next = slideIntoCanvas(next, drag.content, drag.canvas);
      return guestBlocked(drag, next, drag.content) ? null : next;
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
      const rawScale = extent / (horizontal ? drag.content.w : drag.content.h);
      // Cap the dragged axis at the frame (the other axis stays untouched).
      const capped = clampScalesToCanvas(
        horizontal ? { ...t, scaleX: rawScale } : { ...t, scaleY: rawScale },
        drag.content,
        drag.canvas,
        false,
      );
      const scale = horizontal ? capped.scaleX : capped.scaleY;
      const cappedExtent = scale * (horizontal ? drag.content.w : drag.content.h);
      const center = {
        x: drag.fixed.x + (axis.x * cappedExtent) / 2,
        y: drag.fixed.y + (axis.y * cappedExtent) / 2,
      };
      let next: Transform = horizontal
        ? { ...t, x: center.x, y: center.y, scaleX: scale }
        : { ...t, x: center.x, y: center.y, scaleY: scale };
      next = slideIntoCanvas(next, drag.content, drag.canvas);
      return guestBlocked(drag, next, drag.content) ? null : next;
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
      const next: Transform = {
        ...t,
        crop,
        x: drag.fixed.x + (axis.x * extent) / 2,
        y: drag.fixed.y + (axis.y * extent) / 2,
      };
      // Cropping only shrinks, but the glued edge can sit outside a canvas
      // the item was flush against — slide back in against the NEW content.
      return slideIntoCanvas(next, content, drag.canvas);
    }
  }
}

function clampCrop(value: number, sourceExtent: number, oppositeCrop: number): number {
  return Math.max(0, Math.min(value, sourceExtent - oppositeCrop - 1));
}
