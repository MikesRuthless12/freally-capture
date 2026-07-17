import { useEffect, useRef, useState } from "react";

import { useDismiss } from "../lib/useDismiss";

import { save } from "@tauri-apps/plugin-dialog";

import {
  bezierExportWipe,
  cursorFxSet,
  settingsGet,
  studioAddFilter,
  studioPasteFilters,
  studioRemoveFilter,
  studioReorderFilter,
  studioSetFilterEnabled,
  studioSetItemBlend,
  studioAutocrop,
  studioAutocropFollow,
  studioAutocropGet,
  studioSetItemScaling,
  studioUpdateFilter,
} from "../api/commands";
import { copyFilters, useClipboard } from "../lib/clipboard";
import { WorkbenchDialog } from "./WorkbenchDialog";
import type {
  BlendMode,
  CursorFxSetting,
  Filter,
  FilterKind,
  FilterTypeName,
  ItemId,
  Rgba,
  ScaleMode,
  SceneId,
  SceneItem,
  Source,
} from "../api/types";
import { BLEND_MODES } from "../api/types";

/** Pixel-perfect scaling (CAP-N70), in UI order, with render-time i18n keys. */
const SCALE_MODES: ScaleMode[] = ["auto", "nearest", "integer", "sharpBilinear"];
const SCALE_MODE_KEYS: Record<ScaleMode, string> = {
  auto: "filters-scaling-auto",
  nearest: "filters-scaling-nearest",
  integer: "filters-scaling-integer",
  sharpBilinear: "filters-scaling-sharp",
};
import { hexToRgba, rgbaToHex } from "../lib/color";
import { useT } from "../i18n/t";
import { NumberField } from "./NumberField";
import { PickerShell } from "./PickerShell";

/**
 * `type -> i18n key`. Resolved with `t(...)` at RENDER time, never here: a
 * module-level `t()` runs at import, before `initLocale`, and would freeze
 * every name to English for the life of the process.
 */
const FILTER_NAME_KEYS: Record<FilterTypeName, string> = {
  chromaKey: "filters-name-chroma-key",
  colorKey: "filters-name-color-key",
  lumaKey: "filters-name-luma-key",
  renderDelay: "filters-name-render-delay",
  colorCorrection: "filters-name-color-correction",
  lut: "filters-name-lut",
  blur: "filters-name-blur",
  mask: "filters-name-mask",
  sharpen: "filters-name-sharpen",
  scroll: "filters-name-scroll",
  perspective: "filters-name-perspective",
  fadeLoop: "filters-name-fade-loop",
  crop: "filters-name-crop",
  flip: "filters-name-flip",
  directionalBlur: "filters-name-directional-blur",
  radialBlur: "filters-name-radial-blur",
  zoomBlur: "filters-name-zoom-blur",
  pixelate: "filters-name-pixelate",
  freeze: "filters-name-freeze",
  userShader: "filters-name-shader",
  bezierMask: "filters-name-bezier-mask",
};

/** A curated built-in gallery of user WGSL effects (CAP-N22): `[i18n key,
 * source]`. Selecting one fills the editor; the user can then tweak it. Each
 * defines `effect(uv, color, p, texel, time)` and annotates its params with
 * `// @param <label> <min> <max> <default>`, which the editor turns into
 * sliders. */
const SHADER_GALLERY: Array<[string, string]> = [
  [
    "filters-shader-gallery-grayscale",
    `// @param Amount 0 1 1
fn effect(uv: vec2<f32>, color: vec4<f32>, p: vec4<f32>, texel: vec4<f32>, time: f32) -> vec4<f32> {
  let gray = vec3<f32>(luma(color.rgb));
  return vec4<f32>(mix(color.rgb, gray, p.x), color.a);
}`,
  ],
  [
    "filters-shader-gallery-invert",
    `fn effect(uv: vec2<f32>, color: vec4<f32>, p: vec4<f32>, texel: vec4<f32>, time: f32) -> vec4<f32> {
  return vec4<f32>(vec3<f32>(1.0) - color.rgb, color.a);
}`,
  ],
  [
    "filters-shader-gallery-scanlines",
    `// @param Darkness 0 1 0.4
// @param Lines 100 1080 480
fn effect(uv: vec2<f32>, color: vec4<f32>, p: vec4<f32>, texel: vec4<f32>, time: f32) -> vec4<f32> {
  let s = 0.5 + 0.5 * sin(uv.y * p.y * 3.14159);
  return vec4<f32>(color.rgb * (1.0 - p.x * (1.0 - s)), color.a);
}`,
  ],
  [
    "filters-shader-gallery-vignette",
    `// @param Strength 0 2 1
fn effect(uv: vec2<f32>, color: vec4<f32>, p: vec4<f32>, texel: vec4<f32>, time: f32) -> vec4<f32> {
  let d = distance(uv, vec2<f32>(0.5));
  let v = clamp(1.0 - d * d * p.x * 2.0, 0.0, 1.0);
  return vec4<f32>(color.rgb * v, color.a);
}`,
  ],
];

const FILTER_DEFAULTS: Record<FilterTypeName, FilterKind> = {
  chromaKey: {
    type: "chromaKey",
    key: { r: 0, g: 255, b: 0, a: 255 },
    similarity: 0.4,
    smoothness: 0.08,
    spill: 0.1,
  },
  colorCorrection: {
    type: "colorCorrection",
    gamma: 0,
    brightness: 0,
    contrast: 0,
    saturation: 1,
    hueShift: 0,
    opacity: 1,
  },
  colorKey: {
    type: "colorKey",
    key: { r: 0, g: 255, b: 0, a: 255 },
    similarity: 0.4,
    smoothness: 0.08,
  },
  lumaKey: { type: "lumaKey", lumaMin: 0, lumaMax: 1, smoothness: 0.08 },
  renderDelay: { type: "renderDelay", delayMs: 100 },
  lut: { type: "lut", path: "", amount: 1 },
  blur: { type: "blur", radius: 8 },
  mask: { type: "mask", path: "", mode: "alpha", invert: false },
  sharpen: { type: "sharpen", amount: 0.25 },
  scroll: { type: "scroll", speedX: 50, speedY: 0 },
  perspective: { type: "perspective", tilt: 55, fade: 0.35 },
  fadeLoop: { type: "fadeLoop", fadeInS: 1, visibleS: 4, fadeOutS: 1, hiddenS: 4 },
  crop: { type: "crop", left: 0, top: 0, right: 0, bottom: 0 },
  flip: { type: "flip", horizontal: true, vertical: false },
  directionalBlur: { type: "directionalBlur", radius: 8, angle: 0 },
  radialBlur: { type: "radialBlur", amount: 0.5, centerX: 0.5, centerY: 0.5 },
  zoomBlur: { type: "zoomBlur", amount: 0.5, centerX: 0.5, centerY: 0.5 },
  pixelate: { type: "pixelate", size: 8 },
  freeze: { type: "freeze" },
  userShader: { type: "userShader", source: SHADER_GALLERY[0][1], params: [1] },
  bezierMask: {
    type: "bezierMask",
    points: [
      [0.25, 0.25],
      [0.75, 0.25],
      [0.75, 0.75],
      [0.25, 0.75],
    ],
    feather: 0.03,
    invert: false,
  },
};

/** Preset shapes for the bezier mask editor (CAP-N28): `[i18n key, points]`. */
const MASK_SHAPES: Array<[string, [number, number][]]> = [
  [
    "filters-mask-shape-rectangle",
    [
      [0.2, 0.2],
      [0.8, 0.2],
      [0.8, 0.8],
      [0.2, 0.8],
    ],
  ],
  [
    "filters-mask-shape-diamond",
    [
      [0.5, 0.15],
      [0.85, 0.5],
      [0.5, 0.85],
      [0.15, 0.5],
    ],
  ],
  [
    "filters-mask-shape-hexagon",
    [
      [0.5, 0.12],
      [0.83, 0.31],
      [0.83, 0.69],
      [0.5, 0.88],
      [0.17, 0.69],
      [0.17, 0.31],
    ],
  ],
  [
    "filters-mask-shape-circle",
    Array.from({ length: 12 }, (_, i) => {
      const a = (i / 12) * Math.PI * 2;
      return [0.5 + 0.36 * Math.cos(a), 0.5 + 0.36 * Math.sin(a)] as [number, number];
    }),
  ],
];

/** Parse `// @param <label> <min> <max> <default>` lines out of a shader
 * (CAP-N22) into slider metadata, in source order, capped at four (the shader
 * uniform carries exactly four params). */
function parseShaderParams(
  source: string,
): Array<{ label: string; min: number; max: number; def: number }> {
  const out: Array<{ label: string; min: number; max: number; def: number }> = [];
  for (const line of source.split("\n")) {
    const m = line.match(
      /^\s*\/\/\s*@param\s+(\S+)\s+(-?\d*\.?\d+)\s+(-?\d*\.?\d+)\s+(-?\d*\.?\d+)/,
    );
    if (m && out.length < 4) {
      out.push({ label: m[1], min: Number(m[2]), max: Number(m[3]), def: Number(m[4]) });
    }
  }
  return out;
}

type FiltersDialogProps = {
  sceneId: SceneId;
  item: SceneItem;
  sourceName: string;
  /** The item's source — display/window captures get the cursor-effects
   * section (CAP-N19). */
  source?: Source;
  onClose: () => void;
};

const fail = (what: string) => (err: unknown) => console.error(`${what} failed:`, err);

/** Per-item blend mode + the ordered filter chain with live parameters. */
export function FiltersDialog({ sceneId, item, sourceName, source, onClose }: FiltersDialogProps) {
  const t = useT();
  const clipboard = useClipboard();
  const [addOpen, setAddOpen] = useState(false);
  // The key filter open in the keying workbench (CAP-M26), if any.
  const [tuning, setTuning] = useState<Filter | null>(null);
  const tuningFilter = tuning && item.filters.find((f) => f.id === tuning.id);
  // Wraps the trigger *and* the menu — see `useDismiss`.
  const addMenuRef = useRef<HTMLDivElement>(null);
  useDismiss(addOpen, addMenuRef, () => setAddOpen(false));

  const update = (filter: Filter, kind: FilterKind) => {
    studioUpdateFilter(sceneId, item.id, filter.id, kind).catch(fail("filter update"));
  };

  // Cursor effects (CAP-N19) ride the CAPTURE, not the item — two items
  // sharing one display share them, like the HDR tone-map.
  const captureId =
    source && (source.kind === "display" || source.kind === "window") && source.captureId
      ? source.captureId
      : null;

  // Auto black-bar crop (CAP-N72): follow-mode state, hydrated from the
  // engine so a reopened dialog shows the truth.
  const [autocropFollow, setAutocropFollow] = useState(false);
  useEffect(() => {
    let alive = true;
    studioAutocropGet(item.id)
      .then((follow) => alive && typeof follow === "boolean" && setAutocropFollow(follow))
      .catch(() => undefined);
    return () => {
      alive = false;
    };
  }, [item.id]);

  return (
    <>
      <PickerShell title={t("filters-title", { name: sourceName })} onClose={onClose} wide>
        <div className="flex flex-col gap-3">
          <div className="flex flex-wrap items-center gap-4">
            <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
              {t("filters-blend-mode")}
              <select
                value={item.blend}
                onChange={(event) =>
                  studioSetItemBlend(sceneId, item.id, event.target.value as BlendMode).catch(
                    fail("blend change"),
                  )
                }
                className="rounded-md border border-white/10 bg-havoc-panel px-2 py-1 text-xs text-havoc-text"
              >
                {BLEND_MODES.map((mode) => (
                  <option key={mode} value={mode}>
                    {mode}
                  </option>
                ))}
              </select>
            </label>
            <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
              {t("filters-scaling")}
              <select
                value={item.scaling ?? "auto"}
                onChange={(event) =>
                  studioSetItemScaling(sceneId, item.id, event.target.value as ScaleMode).catch(
                    fail("scaling change"),
                  )
                }
                title={t("filters-scaling-hint")}
                className="rounded-md border border-white/10 bg-havoc-panel px-2 py-1 text-xs text-havoc-text"
              >
                {SCALE_MODES.map((mode) => (
                  <option key={mode} value={mode}>
                    {t(SCALE_MODE_KEYS[mode])}
                  </option>
                ))}
              </select>
            </label>
            {!item.backdrop && (
              <span className="flex items-center gap-2 text-[11px] text-havoc-muted">
                <button
                  type="button"
                  onClick={() => studioAutocrop(sceneId, item.id).catch(fail("auto-crop"))}
                  title={t("filters-autocrop-title")}
                  className="rounded-md border border-white/10 px-2 py-1 text-xs text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
                >
                  {t("filters-autocrop")}
                </button>
                <label className="flex items-center gap-1">
                  <input
                    type="checkbox"
                    checked={autocropFollow}
                    onChange={(event) => {
                      const next = event.target.checked;
                      setAutocropFollow(next);
                      studioAutocropFollow(sceneId, item.id, next).catch(fail("auto-crop follow"));
                    }}
                  />
                  {t("filters-autocrop-follow")}
                </label>
              </span>
            )}
          </div>

          <div className="flex items-center justify-between">
            <span className="text-[11px] font-semibold tracking-wider text-havoc-muted uppercase">
              {t("filters-chain-header")}
            </span>
            <div className="relative" ref={addMenuRef}>
              <button
                type="button"
                onClick={() => setAddOpen((open) => !open)}
                aria-haspopup="menu"
                aria-expanded={addOpen}
                className="rounded-md border border-white/10 px-2 py-0.5 text-xs text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
              >
                {t("filters-add")}
              </button>
              {addOpen && (
                <div
                  role="menu"
                  aria-label={t("filters-add-menu")}
                  className="absolute right-0 z-20 mt-1 w-44 rounded-lg border border-white/10 bg-havoc-panel p-1 shadow-xl"
                >
                  {(Object.keys(FILTER_DEFAULTS) as FilterTypeName[]).map((type) => (
                    <button
                      key={type}
                      type="button"
                      role="menuitem"
                      onClick={() => {
                        setAddOpen(false);
                        studioAddFilter(sceneId, item.id, FILTER_DEFAULTS[type]).catch(
                          fail("filter add"),
                        );
                      }}
                      className="block w-full rounded-md px-2 py-1.5 text-left text-xs text-havoc-text hover:bg-white/5"
                    >
                      {t(FILTER_NAME_KEYS[type])}
                    </button>
                  ))}
                </div>
              )}
            </div>
          </div>

          <div className="flex items-center gap-2">
            <button
              type="button"
              disabled={item.filters.length === 0}
              onClick={() => copyFilters(item.filters)}
              className="rounded-md border border-white/10 px-2 py-0.5 text-xs text-havoc-muted enabled:hover:text-havoc-text disabled:opacity-40"
            >
              {t("filters-copy", { count: item.filters.length })}
            </button>
            <button
              type="button"
              disabled={!clipboard.filters?.length}
              onClick={() =>
                clipboard.filters &&
                studioPasteFilters(sceneId, item.id, clipboard.filters).catch(fail("filter paste"))
              }
              className="rounded-md border border-white/10 px-2 py-0.5 text-xs text-havoc-muted enabled:hover:text-havoc-text disabled:opacity-40"
            >
              {t("filters-paste", { count: clipboard.filters?.length ?? 0 })}
            </button>
          </div>

          {item.filters.length === 0 ? (
            <p className="m-0 text-xs text-havoc-muted">{t("filters-empty")}</p>
          ) : (
            <ul className="m-0 flex list-none flex-col gap-2 p-0">
              {item.filters.map((filter, index) => (
                <li
                  key={filter.id}
                  className="rounded-lg border border-white/10 bg-white/[0.02] p-2"
                >
                  <div className="flex items-center gap-2">
                    <input
                      type="checkbox"
                      checked={filter.enabled}
                      onChange={(event) =>
                        studioSetFilterEnabled(
                          sceneId,
                          item.id,
                          filter.id,
                          event.target.checked,
                        ).catch(fail("filter toggle"))
                      }
                      aria-label={t("filters-enable", { name: t(FILTER_NAME_KEYS[filter.type]) })}
                    />
                    <span className="flex-1 text-xs font-semibold text-havoc-text">
                      {t(FILTER_NAME_KEYS[filter.type])}
                    </span>
                    {(filter.type === "chromaKey" ||
                      filter.type === "colorKey" ||
                      filter.type === "lumaKey") && (
                      <button
                        type="button"
                        onClick={() => setTuning(filter)}
                        title={t("workbench-tune")}
                        className="rounded px-1.5 text-[10px] text-havoc-muted hover:text-havoc-accent"
                      >
                        {t("workbench-tune")}
                      </button>
                    )}
                    <button
                      type="button"
                      disabled={index === 0}
                      onClick={() =>
                        studioReorderFilter(sceneId, item.id, filter.id, index - 1).catch(
                          fail("filter reorder"),
                        )
                      }
                      title={t("filters-run-earlier")}
                      aria-label={t("filters-move-up", { name: t(FILTER_NAME_KEYS[filter.type]) })}
                      className="rounded px-1 text-[10px] text-havoc-muted enabled:hover:text-havoc-text disabled:opacity-40"
                    >
                      ▲
                    </button>
                    <button
                      type="button"
                      disabled={index === item.filters.length - 1}
                      onClick={() =>
                        studioReorderFilter(sceneId, item.id, filter.id, index + 1).catch(
                          fail("filter reorder"),
                        )
                      }
                      title={t("filters-run-later")}
                      aria-label={t("filters-move-down", {
                        name: t(FILTER_NAME_KEYS[filter.type]),
                      })}
                      className="rounded px-1 text-[10px] text-havoc-muted enabled:hover:text-havoc-text disabled:opacity-40"
                    >
                      ▼
                    </button>
                    <button
                      type="button"
                      onClick={() =>
                        studioRemoveFilter(sceneId, item.id, filter.id).catch(fail("filter remove"))
                      }
                      title={t("filters-remove-title")}
                      aria-label={t("filters-remove", { name: t(FILTER_NAME_KEYS[filter.type]) })}
                      className="rounded px-1 text-xs text-havoc-muted hover:text-red-400"
                    >
                      ×
                    </button>
                  </div>
                  <FilterParams
                    filter={filter}
                    sceneId={sceneId}
                    itemId={item.id}
                    onChange={(kind) => update(filter, kind)}
                  />
                </li>
              ))}
            </ul>
          )}

          {captureId && <CursorFxSection captureId={captureId} />}
        </div>
      </PickerShell>
      {tuningFilter && (
        <WorkbenchDialog
          item={item}
          filter={tuningFilter}
          sourceName={sourceName}
          onChange={(kind) => update(tuningFilter, kind)}
          onClose={() => setTuning(null)}
        />
      )}
    </>
  );
}

// ---------------------------------------------------------------------------
// Cursor effects (CAP-N19)
// ---------------------------------------------------------------------------

const CURSOR_FX_DEFAULTS: CursorFxSetting = {
  halo: false,
  haloColor: "#ffd54a",
  haloRadius: 24,
  ripples: false,
  leftColor: "#4ac1ff",
  rightColor: "#ff5a5a",
  keystrokes: false,
};

/** Halo, click ripples & keystroke ghosting for a display/window capture —
 * drawn into the frames on the owned (Windows) cursor path, applied live. */
function CursorFxSection({ captureId }: { captureId: string }) {
  const t = useT();
  const [fx, setFx] = useState<CursorFxSetting>(CURSOR_FX_DEFAULTS);
  useEffect(() => {
    let alive = true;
    settingsGet()
      .then((settings) => {
        const saved = settings.cursorFx?.[captureId];
        if (alive && saved) setFx(saved);
      })
      .catch(() => undefined);
    return () => {
      alive = false;
    };
  }, [captureId]);

  const apply = (next: CursorFxSetting) => {
    setFx(next);
    cursorFxSet(captureId, next).catch(fail("cursor effects"));
  };

  return (
    <div className="flex flex-col gap-2 rounded-lg border border-white/10 bg-white/[0.02] p-2">
      <span className="text-[11px] font-semibold tracking-wider text-havoc-muted uppercase">
        {t("filters-cursorfx-header")}
      </span>
      <p className="m-0 text-[11px] text-havoc-muted">{t("filters-cursorfx-hint")}</p>
      <div className="flex flex-wrap items-center gap-4">
        <label className="flex items-center gap-1 text-[11px] text-havoc-muted">
          <input
            type="checkbox"
            checked={fx.halo}
            onChange={(event) => apply({ ...fx, halo: event.target.checked })}
          />
          {t("filters-cursorfx-halo")}
        </label>
        {fx.halo && (
          <>
            <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
              {t("filters-cursorfx-halo-color")}
              <input
                type="color"
                value={fx.haloColor}
                onChange={(event) => apply({ ...fx, haloColor: event.target.value })}
                aria-label={t("filters-cursorfx-halo-color")}
                className="h-6 w-10 cursor-pointer rounded border border-white/10 bg-transparent"
              />
            </label>
            <NumberField
              label={t("filters-cursorfx-halo-radius")}
              value={fx.haloRadius}
              min={8}
              max={128}
              step={4}
              onCommit={(haloRadius) => apply({ ...fx, haloRadius: Math.round(haloRadius) })}
              className="w-20"
            />
          </>
        )}
      </div>
      <div className="flex flex-wrap items-center gap-4">
        <label className="flex items-center gap-1 text-[11px] text-havoc-muted">
          <input
            type="checkbox"
            checked={fx.ripples}
            onChange={(event) => apply({ ...fx, ripples: event.target.checked })}
          />
          {t("filters-cursorfx-ripples")}
        </label>
        {fx.ripples && (
          <>
            <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
              {t("filters-cursorfx-left-color")}
              <input
                type="color"
                value={fx.leftColor}
                onChange={(event) => apply({ ...fx, leftColor: event.target.value })}
                aria-label={t("filters-cursorfx-left-color")}
                className="h-6 w-10 cursor-pointer rounded border border-white/10 bg-transparent"
              />
            </label>
            <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
              {t("filters-cursorfx-right-color")}
              <input
                type="color"
                value={fx.rightColor}
                onChange={(event) => apply({ ...fx, rightColor: event.target.value })}
                aria-label={t("filters-cursorfx-right-color")}
                className="h-6 w-10 cursor-pointer rounded border border-white/10 bg-transparent"
              />
            </label>
          </>
        )}
      </div>
      <div className="flex flex-col gap-1">
        <label className="flex items-center gap-1 text-[11px] text-havoc-muted">
          <input
            type="checkbox"
            checked={fx.keystrokes}
            onChange={(event) => apply({ ...fx, keystrokes: event.target.checked })}
          />
          {t("filters-cursorfx-keystrokes")}
        </label>
        {fx.keystrokes && (
          <p className="m-0 text-[11px] text-havoc-muted">
            {t("filters-cursorfx-keystrokes-hint")}
          </p>
        )}
      </div>
    </div>
  );
}

// ---------------------------------------------------------------------------
// Parameter editors
// ---------------------------------------------------------------------------

function Slider({
  label,
  value,
  min,
  max,
  step,
  onChange,
}: {
  label: string;
  value: number;
  min: number;
  max: number;
  step: number;
  onChange: (value: number) => void;
}) {
  return (
    <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
      <span className="w-24 shrink-0">{label}</span>
      <input
        type="range"
        min={min}
        max={max}
        step={step}
        value={value}
        onChange={(event) => onChange(Number(event.target.value))}
        className="flex-1 accent-havoc-accent"
        aria-label={label}
      />
      <span className="w-12 shrink-0 text-right text-havoc-text">
        {Number.isInteger(step) ? value : value.toFixed(2)}
      </span>
    </label>
  );
}

function ColorRow({
  label,
  value,
  onChange,
}: {
  label: string;
  value: Rgba;
  onChange: (value: Rgba) => void;
}) {
  return (
    <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
      <span className="w-24 shrink-0">{label}</span>
      <input
        type="color"
        value={rgbaToHex(value)}
        onChange={(event) => onChange(hexToRgba(event.target.value, value.a))}
        aria-label={label}
        className="h-6 w-10 cursor-pointer rounded border border-white/10 bg-transparent"
      />
    </label>
  );
}

function PathRow({
  label,
  value,
  placeholder,
  onCommit,
}: {
  label: string;
  value: string;
  placeholder: string;
  onCommit: (value: string) => void;
}) {
  const [draft, setDraft] = useState(value);
  return (
    <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
      <span className="w-24 shrink-0">{label}</span>
      <input
        value={draft}
        placeholder={placeholder}
        onChange={(event) => setDraft(event.target.value)}
        onBlur={() => draft !== value && onCommit(draft)}
        onKeyDown={(event) => {
          if (event.key === "Enter" && draft !== value) onCommit(draft);
        }}
        aria-label={label}
        className="flex-1 rounded-md border border-white/10 bg-havoc-panel px-2 py-1 text-xs text-havoc-text outline-none focus:border-havoc-accent/60"
      />
    </label>
  );
}

function CropRow({
  values,
  onChange,
}: {
  values: { left: number; top: number; right: number; bottom: number };
  onChange: (values: { left: number; top: number; right: number; bottom: number }) => void;
}) {
  const t = useT();
  const field = (key: "left" | "top" | "right" | "bottom") => (
    <label key={key} className="flex flex-1 flex-col gap-0.5 text-[10px] text-havoc-muted">
      {t(`filters-crop-${key}`)}
      <input
        type="number"
        min={0}
        value={values[key]}
        onChange={(event) =>
          onChange({ ...values, [key]: Math.max(0, Number(event.target.value) || 0) })
        }
        // The side must be the *translated* word, not the raw key — otherwise a
        // Japanese screen reader announces "クロップ left".
        aria-label={t("filters-crop-aria", { side: t(`filters-crop-${key}`) })}
        className="rounded-md border border-white/10 bg-havoc-panel px-1.5 py-1 text-xs text-havoc-text"
      />
    </label>
  );
  return (
    <div className="flex gap-2">{(["left", "top", "right", "bottom"] as const).map(field)}</div>
  );
}

function FilterParams({
  filter,
  sceneId,
  itemId,
  onChange,
}: {
  filter: Filter;
  sceneId: SceneId;
  itemId: ItemId;
  onChange: (kind: FilterKind) => void;
}) {
  const t = useT();
  switch (filter.type) {
    case "colorKey":
      return (
        <div className="mt-2 flex flex-col gap-1.5">
          <ColorRow
            label={t("filters-key-color-rgb")}
            value={filter.key}
            onChange={(key) => onChange({ ...filter, key })}
          />
          <Slider
            label={t("filters-similarity")}
            value={filter.similarity}
            min={0}
            max={1}
            step={0.01}
            onChange={(similarity) => onChange({ ...filter, similarity })}
          />
          <Slider
            label={t("filters-smoothness")}
            value={filter.smoothness}
            min={0}
            max={1}
            step={0.01}
            onChange={(smoothness) => onChange({ ...filter, smoothness })}
          />
        </div>
      );
    case "lumaKey":
      return (
        <div className="mt-2 flex flex-col gap-1.5">
          <Slider
            label={t("filters-luma-min")}
            value={filter.lumaMin}
            min={0}
            max={1}
            step={0.01}
            onChange={(lumaMin) => onChange({ ...filter, lumaMin })}
          />
          <Slider
            label={t("filters-luma-max")}
            value={filter.lumaMax}
            min={0}
            max={1}
            step={0.01}
            onChange={(lumaMax) => onChange({ ...filter, lumaMax })}
          />
          <Slider
            label={t("filters-smoothness")}
            value={filter.smoothness}
            min={0}
            max={1}
            step={0.01}
            onChange={(smoothness) => onChange({ ...filter, smoothness })}
          />
        </div>
      );
    case "renderDelay":
      return (
        <div className="mt-2 flex flex-col gap-1.5">
          <Slider
            label={t("filters-delay")}
            value={filter.delayMs}
            min={0}
            max={500}
            step={10}
            onChange={(delayMs) => onChange({ ...filter, delayMs: Math.round(delayMs) })}
          />
        </div>
      );
    case "chromaKey":
      return (
        <div className="mt-2 flex flex-col gap-1.5">
          <ColorRow
            label={t("filters-key-color")}
            value={filter.key}
            onChange={(key) => onChange({ ...filter, key })}
          />
          <Slider
            label={t("filters-similarity")}
            value={filter.similarity}
            min={0}
            max={1}
            step={0.01}
            onChange={(similarity) => onChange({ ...filter, similarity })}
          />
          <Slider
            label={t("filters-smoothness")}
            value={filter.smoothness}
            min={0}
            max={1}
            step={0.01}
            onChange={(smoothness) => onChange({ ...filter, smoothness })}
          />
          <Slider
            label={t("filters-spill")}
            value={filter.spill}
            min={0}
            max={1}
            step={0.01}
            onChange={(spill) => onChange({ ...filter, spill })}
          />
        </div>
      );
    case "colorCorrection":
      return (
        <div className="mt-2 flex flex-col gap-1.5">
          <Slider
            label={t("filters-gamma")}
            value={filter.gamma}
            min={-3}
            max={3}
            step={0.01}
            onChange={(gamma) => onChange({ ...filter, gamma })}
          />
          <Slider
            label={t("filters-brightness")}
            value={filter.brightness}
            min={-1}
            max={1}
            step={0.01}
            onChange={(brightness) => onChange({ ...filter, brightness })}
          />
          <Slider
            label={t("filters-contrast")}
            value={filter.contrast}
            min={-1}
            max={1}
            step={0.01}
            onChange={(contrast) => onChange({ ...filter, contrast })}
          />
          <Slider
            label={t("filters-saturation")}
            value={filter.saturation}
            min={0}
            max={4}
            step={0.01}
            onChange={(saturation) => onChange({ ...filter, saturation })}
          />
          <Slider
            label={t("filters-hue-shift")}
            value={filter.hueShift}
            min={-180}
            max={180}
            step={1}
            onChange={(hueShift) => onChange({ ...filter, hueShift })}
          />
          <Slider
            label={t("filters-opacity")}
            value={filter.opacity}
            min={0}
            max={1}
            step={0.01}
            onChange={(opacity) => onChange({ ...filter, opacity })}
          />
        </div>
      );
    case "lut":
      return (
        <div className="mt-2 flex flex-col gap-1.5">
          <PathRow
            label={t("filters-cube-file")}
            value={filter.path}
            placeholder="C:\luts\warm.cube"
            onCommit={(path) => onChange({ ...filter, path })}
          />
          <Slider
            label={t("filters-amount")}
            value={filter.amount}
            min={0}
            max={1}
            step={0.01}
            onChange={(amount) => onChange({ ...filter, amount })}
          />
        </div>
      );
    case "blur":
      return (
        <div className="mt-2">
          <Slider
            label={t("filters-radius")}
            value={filter.radius}
            min={0}
            max={64}
            step={0.5}
            onChange={(radius) => onChange({ ...filter, radius })}
          />
        </div>
      );
    case "mask":
      return (
        <div className="mt-2 flex flex-col gap-1.5">
          <PathRow
            label={t("filters-mask-image")}
            value={filter.path}
            placeholder="C:\masks\rounded.png"
            onCommit={(path) => onChange({ ...filter, path })}
          />
          <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
            <span className="w-24 shrink-0">{t("filters-mask-mode")}</span>
            <select
              value={filter.mode}
              onChange={(event) =>
                onChange({ ...filter, mode: event.target.value as "alpha" | "luma" })
              }
              className="rounded-md border border-white/10 bg-havoc-panel px-2 py-1 text-xs text-havoc-text"
            >
              <option value="alpha">{t("filters-mask-alpha")}</option>
              <option value="luma">{t("filters-mask-luma")}</option>
            </select>
            <label className="flex items-center gap-1">
              <input
                type="checkbox"
                checked={filter.invert}
                onChange={(event) => onChange({ ...filter, invert: event.target.checked })}
              />
              {t("filters-mask-invert")}
            </label>
          </label>
        </div>
      );
    case "sharpen":
      return (
        <div className="mt-2">
          <Slider
            label={t("filters-amount")}
            value={filter.amount}
            min={0}
            max={2}
            step={0.01}
            onChange={(amount) => onChange({ ...filter, amount })}
          />
        </div>
      );
    case "scroll":
      return (
        <div className="mt-2 flex flex-col gap-1.5">
          <Slider
            label={t("filters-speed-x")}
            value={filter.speedX}
            min={-500}
            max={500}
            step={1}
            onChange={(speedX) => onChange({ ...filter, speedX })}
          />
          <Slider
            label={t("filters-speed-y")}
            value={filter.speedY}
            min={-500}
            max={500}
            step={1}
            onChange={(speedY) => onChange({ ...filter, speedY })}
          />
        </div>
      );
    case "perspective":
      return (
        <div className="mt-2 flex flex-col gap-1.5">
          <Slider
            label={t("filters-tilt")}
            value={filter.tilt}
            min={0}
            max={80}
            step={1}
            onChange={(tilt) => onChange({ ...filter, tilt })}
          />
          <Slider
            label={t("filters-far-fade")}
            value={filter.fade}
            min={0}
            max={1}
            step={0.01}
            onChange={(fade) => onChange({ ...filter, fade })}
          />
        </div>
      );
    case "fadeLoop":
      return (
        <div className="mt-2 flex flex-col gap-1.5">
          <Slider
            label={t("filters-fade-in-s")}
            value={filter.fadeInS}
            min={0}
            max={10}
            step={0.1}
            onChange={(fadeInS) => onChange({ ...filter, fadeInS })}
          />
          <Slider
            label={t("filters-visible-s")}
            value={filter.visibleS}
            min={0}
            max={60}
            step={0.5}
            onChange={(visibleS) => onChange({ ...filter, visibleS })}
          />
          <Slider
            label={t("filters-fade-out-s")}
            value={filter.fadeOutS}
            min={0}
            max={10}
            step={0.1}
            onChange={(fadeOutS) => onChange({ ...filter, fadeOutS })}
          />
          <Slider
            label={t("filters-hidden-s")}
            value={filter.hiddenS}
            min={0}
            max={60}
            step={0.5}
            onChange={(hiddenS) => onChange({ ...filter, hiddenS })}
          />
        </div>
      );
    case "crop":
      return (
        <div className="mt-2">
          <CropRow values={filter} onChange={(values) => onChange({ ...filter, ...values })} />
        </div>
      );
    case "flip":
      return (
        <div className="mt-2 flex items-center gap-4 text-xs text-havoc-muted">
          <label className="flex items-center gap-1">
            <input
              type="checkbox"
              checked={filter.horizontal}
              onChange={(event) => onChange({ ...filter, horizontal: event.target.checked })}
            />
            {t("filters-flip-horizontal")}
          </label>
          <label className="flex items-center gap-1">
            <input
              type="checkbox"
              checked={filter.vertical}
              onChange={(event) => onChange({ ...filter, vertical: event.target.checked })}
            />
            {t("filters-flip-vertical")}
          </label>
        </div>
      );
    case "directionalBlur":
      return (
        <div className="mt-2 flex flex-col gap-1.5">
          <Slider
            label={t("filters-radius")}
            value={filter.radius}
            min={0}
            max={64}
            step={0.5}
            onChange={(radius) => onChange({ ...filter, radius })}
          />
          <Slider
            label={t("filters-angle")}
            value={filter.angle}
            min={0}
            max={360}
            step={1}
            onChange={(angle) => onChange({ ...filter, angle })}
          />
        </div>
      );
    case "radialBlur":
    case "zoomBlur":
      return (
        <div className="mt-2 flex flex-col gap-1.5">
          <Slider
            label={t("filters-amount")}
            value={filter.amount}
            min={0}
            max={1}
            step={0.01}
            onChange={(amount) => onChange({ ...filter, amount })}
          />
          <Slider
            label={t("filters-center-x")}
            value={filter.centerX}
            min={0}
            max={1}
            step={0.01}
            onChange={(centerX) => onChange({ ...filter, centerX })}
          />
          <Slider
            label={t("filters-center-y")}
            value={filter.centerY}
            min={0}
            max={1}
            step={0.01}
            onChange={(centerY) => onChange({ ...filter, centerY })}
          />
        </div>
      );
    case "pixelate":
      return (
        <div className="mt-2">
          <Slider
            label={t("filters-block-size")}
            value={filter.size}
            min={1}
            max={128}
            step={1}
            onChange={(size) => onChange({ ...filter, size })}
          />
        </div>
      );
    case "freeze":
      return (
        <p className="mt-2 text-[11px] leading-snug text-havoc-muted">{t("filters-freeze-hint")}</p>
      );
    case "userShader": {
      const params = parseShaderParams(filter.source);
      return (
        <div className="mt-2 flex flex-col gap-1.5">
          <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
            <span className="shrink-0">{t("filters-shader-gallery")}</span>
            <select
              value=""
              onChange={(event) => {
                const preset = SHADER_GALLERY.find(([key]) => key === event.target.value);
                if (preset) {
                  onChange({
                    ...filter,
                    source: preset[1],
                    params: parseShaderParams(preset[1]).map((p) => p.def),
                  });
                }
              }}
              className="min-w-0 flex-1 rounded border border-white/10 bg-havoc-panel px-1.5 py-1 text-[11px] text-havoc-text"
            >
              <option value="">{t("filters-shader-gallery-pick")}</option>
              {SHADER_GALLERY.map(([key]) => (
                <option key={key} value={key}>
                  {t(key)}
                </option>
              ))}
            </select>
          </label>
          <textarea
            value={filter.source}
            spellCheck={false}
            rows={8}
            onChange={(event) => onChange({ ...filter, source: event.target.value })}
            aria-label={t("filters-shader-source")}
            className="w-full rounded border border-white/10 bg-havoc-panel px-1.5 py-1 font-mono text-[10px] leading-snug text-havoc-text"
          />
          {params.map((p, index) => (
            <Slider
              key={`${p.label}-${index}`}
              label={p.label}
              value={filter.params[index] ?? p.def}
              min={p.min}
              max={p.max}
              step={Math.max((p.max - p.min) / 100, 0.001)}
              onChange={(value) =>
                onChange({
                  ...filter,
                  params: params.map((pp, j) =>
                    j === index ? value : (filter.params[j] ?? pp.def),
                  ),
                })
              }
            />
          ))}
          <p className="text-[11px] leading-snug text-havoc-muted">{t("filters-shader-hint")}</p>
        </div>
      );
    }
    case "bezierMask":
      return (
        <BezierMaskEditor filter={filter} sceneId={sceneId} itemId={itemId} onChange={onChange} />
      );
  }
}

/** The interactive bezier-mask editor (CAP-N28): drag the handles, double-click
 * to add a point, right-click a handle to remove it; plus preset shapes,
 * feather, invert, and export-as-wipe. Coordinates are normalized 0..1 in item
 * space, exactly what the rasterizer expects. */
function BezierMaskEditor({
  filter,
  sceneId,
  itemId,
  onChange,
}: {
  filter: Extract<Filter, { type: "bezierMask" }>;
  sceneId: SceneId;
  itemId: ItemId;
  onChange: (kind: FilterKind) => void;
}) {
  const t = useT();
  const svgRef = useRef<SVGSVGElement>(null);
  const [drag, setDrag] = useState<number | null>(null);
  const points = filter.points;

  const toNorm = (event: { clientX: number; clientY: number }): [number, number] => {
    const rect = svgRef.current?.getBoundingClientRect();
    if (!rect) return [0, 0];
    return [
      Math.min(1, Math.max(0, (event.clientX - rect.left) / rect.width)),
      Math.min(1, Math.max(0, (event.clientY - rect.top) / rect.height)),
    ];
  };
  const setPoints = (next: [number, number][]) => onChange({ ...filter, points: next });

  const exportWipe = async () => {
    const path = await save({
      defaultPath: "wipe.png",
      filters: [{ name: t("studio-preview-filter-images"), extensions: ["png"] }],
    });
    if (typeof path === "string") {
      bezierExportWipe(sceneId, itemId, filter.id, path).catch((err) =>
        console.error("export wipe failed:", err),
      );
    }
  };

  return (
    <div className="mt-2 flex flex-col gap-1.5">
      <svg
        ref={svgRef}
        viewBox="0 0 1 1"
        preserveAspectRatio="none"
        className="aspect-video w-full touch-none rounded border border-white/10 bg-black/40"
        onPointerMove={(event) => {
          if (drag === null) return;
          const p = toNorm(event);
          setPoints(points.map((point, index) => (index === drag ? p : point)));
        }}
        onPointerUp={() => setDrag(null)}
        onDoubleClick={(event) => setPoints([...points, toNorm(event)])}
      >
        <polygon
          points={points.map(([x, y]) => `${x},${y}`).join(" ")}
          fill="rgba(80,180,255,0.18)"
          stroke="rgba(125,211,252,0.9)"
          strokeWidth={0.006}
        />
        {points.map(([x, y], index) => (
          <circle
            key={index}
            cx={x}
            cy={y}
            r={0.022}
            fill={drag === index ? "#7dd3fc" : "#e5e7eb"}
            style={{ cursor: "pointer" }}
            onPointerDown={(event) => {
              event.currentTarget.setPointerCapture(event.pointerId);
              setDrag(index);
            }}
            onContextMenu={(event) => {
              event.preventDefault();
              if (points.length > 3) setPoints(points.filter((_, j) => j !== index));
            }}
          />
        ))}
      </svg>
      <p className="text-[10px] leading-snug text-havoc-muted">{t("filters-mask-editor-hint")}</p>

      <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
        <span className="shrink-0">{t("filters-mask-shape")}</span>
        <select
          value=""
          onChange={(event) => {
            const shape = MASK_SHAPES.find(([key]) => key === event.target.value);
            if (shape) setPoints(shape[1].map((p) => [...p] as [number, number]));
          }}
          className="min-w-0 flex-1 rounded border border-white/10 bg-havoc-panel px-1.5 py-1 text-[11px] text-havoc-text"
        >
          <option value="">{t("filters-mask-shape-pick")}</option>
          {MASK_SHAPES.map(([key]) => (
            <option key={key} value={key}>
              {t(key)}
            </option>
          ))}
        </select>
      </label>

      <Slider
        label={t("filters-mask-feather")}
        value={filter.feather}
        min={0}
        max={0.3}
        step={0.005}
        onChange={(feather) => onChange({ ...filter, feather })}
      />
      <label className="flex items-center gap-2 text-[11px] text-havoc-text">
        <input
          type="checkbox"
          checked={filter.invert}
          onChange={(event) => onChange({ ...filter, invert: event.target.checked })}
        />
        {t("filters-mask-invert")}
      </label>
      <button
        type="button"
        onClick={exportWipe}
        className="self-start rounded-md border border-white/10 px-2 py-1 text-[11px] text-havoc-text hover:border-havoc-accent/50"
      >
        {t("filters-mask-export-wipe")}
      </button>
    </div>
  );
}
