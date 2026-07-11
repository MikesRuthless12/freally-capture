import { useEffect, useRef, useState } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";

import { studioWorkbenchClose, studioWorkbenchSet } from "../api/commands";
import type { Filter, FilterKind, ItemId, WorkbenchMode } from "../api/types";
import { hexToRgba, rgbaToHex } from "../lib/color";
import { useT } from "../i18n/t";
import { PickerShell } from "./PickerShell";

/** How often the workbench refetches its JPEG (~10 fps is plenty). */
const POLL_MS = 100;
const MODES: WorkbenchMode[] = ["keyed", "source", "matte", "split"];
/** Loupe: magnification and the source region (canvas px) sampled around the cursor. */
const LOUPE_ZOOM = 8;
const LOUPE_SRC = 13;

/**
 * The keying workbench (CAP-M26): a focused tuning view for a chroma/color/luma
 * key. It drives a single-source backend render slot in one of four modes
 * (keyed · raw source · alpha matte · before/after split), samples the raw
 * source with an eyedropper to set the key color, and offers a magnifier loupe
 * for edge inspection — all live, with the key parameters editable in place.
 */
export function WorkbenchDialog({
  item,
  filter,
  sourceName,
  onChange,
  onClose,
}: {
  item: { id: ItemId };
  filter: Filter;
  sourceName: string;
  onChange: (kind: FilterKind) => void;
  onClose: () => void;
}) {
  const t = useT();
  const [mode, setMode] = useState<WorkbenchMode>("keyed");
  const [split, setSplit] = useState(0.5);
  const [eyedropper, setEyedropper] = useState(false);
  const [loupe, setLoupe] = useState(true);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const loupeRef = useRef<HTMLCanvasElement>(null);

  const hasColor = filter.type === "chromaKey" || filter.type === "colorKey";

  // Drive the backend slot; close it when the workbench unmounts.
  useEffect(() => {
    studioWorkbenchSet(item.id, mode, split).catch(() => undefined);
  }, [item.id, mode, split]);
  useEffect(
    () => () => {
      studioWorkbenchClose().catch(() => undefined);
    },
    [],
  );

  // Poll the workbench JPEG onto the canvas (a canvas, not an <img>, so the
  // eyedropper + loupe can read pixels).
  useEffect(() => {
    if (typeof createImageBitmap === "undefined") return;
    let stopped = false;
    let timer: ReturnType<typeof setTimeout> | undefined;
    let lastSeq = "";
    const url = convertFileSrc("workbench-preview", "preview");
    const tick = async () => {
      if (stopped) return;
      try {
        const response = await fetch(url, { cache: "no-store" });
        if (response.status === 200) {
          const seq = response.headers.get("x-frame-seq") ?? "";
          if (seq !== lastSeq) {
            lastSeq = seq;
            const bitmap = await createImageBitmap(await response.blob());
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
        // The studio is restarting — retry.
      }
      if (!stopped) timer = setTimeout(tick, POLL_MS);
    };
    void tick();
    return () => {
      stopped = true;
      if (timer) clearTimeout(timer);
    };
  }, []);

  /** Canvas-pixel coordinates of a pointer event. */
  const atPointer = (event: React.PointerEvent<HTMLCanvasElement>) => {
    const canvas = canvasRef.current;
    if (!canvas) return null;
    const rect = canvas.getBoundingClientRect();
    const x = Math.round(((event.clientX - rect.left) / rect.width) * canvas.width);
    const y = Math.round(((event.clientY - rect.top) / rect.height) * canvas.height);
    return { x, y };
  };

  const onCanvasClick = (event: React.PointerEvent<HTMLCanvasElement>) => {
    if (!eyedropper || !hasColor) return;
    const point = atPointer(event);
    const canvas = canvasRef.current;
    const context = canvas?.getContext("2d");
    if (!point || !context) return;
    const [r, g, b] = context.getImageData(point.x, point.y, 1, 1).data;
    onChange({ ...filter, key: { r, g, b, a: 255 } } as FilterKind);
    setEyedropper(false);
  };

  const onCanvasMove = (event: React.PointerEvent<HTMLCanvasElement>) => {
    if (!loupe) return;
    const point = atPointer(event);
    const source = canvasRef.current;
    const target = loupeRef.current?.getContext("2d");
    if (!point || !source || !target) return;
    const half = Math.floor(LOUPE_SRC / 2);
    target.imageSmoothingEnabled = false;
    target.clearRect(0, 0, LOUPE_SRC * LOUPE_ZOOM, LOUPE_SRC * LOUPE_ZOOM);
    target.drawImage(
      source,
      point.x - half,
      point.y - half,
      LOUPE_SRC,
      LOUPE_SRC,
      0,
      0,
      LOUPE_SRC * LOUPE_ZOOM,
      LOUPE_SRC * LOUPE_ZOOM,
    );
    // Center crosshair on the sampled pixel.
    target.strokeStyle = "#ff3ea5";
    target.strokeRect(half * LOUPE_ZOOM, half * LOUPE_ZOOM, LOUPE_ZOOM, LOUPE_ZOOM);
  };

  const armEyedropper = () => {
    setMode("source"); // sample the raw source, pre-key
    setEyedropper(true);
  };

  return (
    <PickerShell title={t("workbench-title", { name: sourceName })} onClose={onClose} wide>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <div className="flex flex-wrap items-center gap-2">
          <div className="flex gap-0.5 rounded-md border border-white/10 p-0.5" role="group">
            {MODES.map((option) => (
              <button
                key={option}
                type="button"
                aria-pressed={mode === option}
                onClick={() => setMode(option)}
                className={`rounded px-2 py-1 text-[11px] ${
                  mode === option
                    ? "bg-havoc-accent/15 text-havoc-text"
                    : "text-havoc-muted hover:text-havoc-text"
                }`}
              >
                {t(`workbench-mode-${option}`)}
              </button>
            ))}
          </div>
          {hasColor && (
            <button
              type="button"
              aria-pressed={eyedropper}
              onClick={armEyedropper}
              className={`rounded-md border px-2 py-1 text-[11px] ${
                eyedropper
                  ? "border-havoc-accent/60 text-havoc-text"
                  : "border-white/10 text-havoc-muted hover:text-havoc-text"
              }`}
            >
              {t("workbench-eyedropper")}
            </button>
          )}
          <label className="flex items-center gap-1 text-[11px] text-havoc-muted">
            <input type="checkbox" checked={loupe} onChange={(e) => setLoupe(e.target.checked)} />
            {t("workbench-loupe")}
          </label>
        </div>

        <div className="relative flex justify-center rounded-lg border border-white/10 bg-black/40 p-2">
          <canvas
            ref={canvasRef}
            aria-label={t("workbench-preview-alt")}
            onPointerDown={onCanvasClick}
            onPointerMove={onCanvasMove}
            className={`max-h-80 max-w-full object-contain ${
              eyedropper ? "cursor-crosshair" : "cursor-default"
            }`}
          />
          {loupe && (
            <canvas
              ref={loupeRef}
              width={LOUPE_SRC * LOUPE_ZOOM}
              height={LOUPE_SRC * LOUPE_ZOOM}
              aria-hidden="true"
              className="pointer-events-none absolute top-3 right-3 rounded border border-white/10 bg-black/60"
            />
          )}
        </div>

        {mode === "split" && (
          <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
            <span className="w-24 shrink-0">{t("workbench-split")}</span>
            <input
              type="range"
              min={0}
              max={1}
              step={0.01}
              value={split}
              onChange={(event) => setSplit(Number(event.target.value))}
              className="flex-1 accent-havoc-accent"
              aria-label={t("workbench-split")}
            />
          </label>
        )}

        <KeyParams filter={filter} onChange={onChange} />

        {eyedropper && (
          <p className="m-0 text-[10px] leading-snug text-havoc-accent" role="status">
            {t("workbench-eyedropper-hint")}
          </p>
        )}

        <div className="flex justify-end">
          <button
            type="button"
            onClick={onClose}
            className="rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-muted hover:text-havoc-text"
          >
            {t("workbench-close")}
          </button>
        </div>
      </div>
    </PickerShell>
  );
}

/** The key filter's tunable parameters, editable while the matte is visible. */
function KeyParams({ filter, onChange }: { filter: Filter; onChange: (kind: FilterKind) => void }) {
  const t = useT();
  const slider = (
    label: string,
    value: number,
    min: number,
    max: number,
    patch: (v: number) => FilterKind,
  ) => (
    <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
      <span className="w-24 shrink-0">{label}</span>
      <input
        type="range"
        min={min}
        max={max}
        step={0.01}
        value={value}
        onChange={(event) => onChange(patch(Number(event.target.value)))}
        className="flex-1 accent-havoc-accent"
        aria-label={label}
      />
      <span className="w-10 text-right text-havoc-text">{value.toFixed(2)}</span>
    </label>
  );

  if (filter.type === "chromaKey" || filter.type === "colorKey") {
    return (
      <div className="flex flex-col gap-1.5 border-t border-white/5 pt-3">
        <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
          <span className="w-24 shrink-0">{t("filters-key-color-rgb")}</span>
          <input
            type="color"
            value={rgbaToHex(filter.key)}
            onChange={(event) =>
              onChange({
                ...filter,
                key: hexToRgba(event.target.value, filter.key.a),
              } as FilterKind)
            }
            aria-label={t("filters-key-color-rgb")}
            className="h-6 w-10 cursor-pointer rounded border border-white/10 bg-transparent"
          />
        </label>
        {slider(
          t("filters-similarity"),
          filter.similarity,
          0,
          1,
          (v) =>
            ({
              ...filter,
              similarity: v,
            }) as FilterKind,
        )}
        {slider(
          t("filters-smoothness"),
          filter.smoothness,
          0,
          1,
          (v) =>
            ({
              ...filter,
              smoothness: v,
            }) as FilterKind,
        )}
        {filter.type === "chromaKey" &&
          slider(
            t("filters-spill"),
            filter.spill,
            0,
            1,
            (v) => ({ ...filter, spill: v }) as FilterKind,
          )}
      </div>
    );
  }
  if (filter.type === "lumaKey") {
    return (
      <div className="flex flex-col gap-1.5 border-t border-white/5 pt-3">
        {slider(
          t("filters-luma-min"),
          filter.lumaMin,
          0,
          1,
          (v) => ({ ...filter, lumaMin: v }) as FilterKind,
        )}
        {slider(
          t("filters-luma-max"),
          filter.lumaMax,
          0,
          1,
          (v) => ({ ...filter, lumaMax: v }) as FilterKind,
        )}
        {slider(
          t("filters-smoothness"),
          filter.smoothness,
          0,
          1,
          (v) =>
            ({
              ...filter,
              smoothness: v,
            }) as FilterKind,
        )}
      </div>
    );
  }
  return null;
}
