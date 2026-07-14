import { useEffect, useRef, useState } from "react";

import { useDismiss } from "../lib/useDismiss";

import {
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
  crop: "filters-name-crop",
  flip: "filters-name-flip",
};

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
  crop: { type: "crop", left: 0, top: 0, right: 0, bottom: 0 },
  flip: { type: "flip", horizontal: true, vertical: false },
};

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
                  <FilterParams filter={filter} onChange={(kind) => update(filter, kind)} />
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
  onChange,
}: {
  filter: Filter;
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
  }
}
