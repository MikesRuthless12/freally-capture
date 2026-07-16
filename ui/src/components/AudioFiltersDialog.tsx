import { useEffect, useRef, useState } from "react";
import type { UnlistenFn } from "@tauri-apps/api/event";

import { useDismiss } from "../lib/useDismiss";

import {
  audioArmFilterMeters,
  studioAddAudioFilter,
  studioApplyVoicePreset,
  studioRemoveAudioFilter,
  studioReorderAudioFilter,
  studioSetAudioFilterEnabled,
  studioUpdateAudioFilter,
} from "../api/commands";
import { onAudio } from "../api/events";
import { FilterViz } from "./FilterViz";
import type {
  AudioFilter,
  AudioFilterKind,
  AudioFilterTypeName,
  Collection,
  Source,
  SourceId,
} from "../api/types";
import { kindHasAudio } from "../api/types";
import { useT } from "../i18n/t";
import { ParametricEqEditor } from "./ParametricEqEditor";
import { PickerShell } from "./PickerShell";

/**
 * `type -> i18n key`. Resolved with `t(...)` at RENDER time, never here: a
 * module-level `t()` runs at import, before `initLocale`, and would freeze
 * every name to English for the life of the process.
 */
const FILTER_NAME_KEYS: Record<AudioFilterTypeName, string> = {
  gain: "audiofilters-name-gain",
  noiseGate: "audiofilters-name-noise-gate",
  compressor: "audiofilters-name-compressor",
  limiter: "audiofilters-name-limiter",
  eq: "audiofilters-name-eq",
  denoise: "audiofilters-name-denoise",
  ducker: "audiofilters-name-ducking",
  parametricEq: "audiofilters-name-parametric-eq",
  deEsser: "audiofilters-name-de-esser",
  rumbleGuard: "audiofilters-name-rumble-guard",
};

const FILTER_DEFAULTS: Record<AudioFilterTypeName, AudioFilterKind> = {
  denoise: { type: "denoise", strength: 0.5 },
  noiseGate: {
    type: "noiseGate",
    openThresholdDb: -26,
    closeThresholdDb: -32,
    attackMs: 25,
    holdMs: 200,
    releaseMs: 150,
  },
  compressor: {
    type: "compressor",
    ratio: 4,
    thresholdDb: -18,
    attackMs: 6,
    releaseMs: 60,
    outputGainDb: 0,
  },
  limiter: { type: "limiter", thresholdDb: -3, releaseMs: 60 },
  eq: { type: "eq", lowDb: 0, midDb: 0, highDb: 0 },
  gain: { type: "gain", db: 0 },
  ducker: {
    type: "ducker",
    trigger: null,
    thresholdDb: -30,
    amountDb: 12,
    attackMs: 50,
    releaseMs: 300,
  },
  parametricEq: {
    type: "parametricEq",
    bands: [
      { type: "bell", freqHz: 120, gainDb: 0, q: 1 },
      { type: "bell", freqHz: 1000, gainDb: 0, q: 1 },
      { type: "bell", freqHz: 6000, gainDb: 0, q: 1 },
    ],
  },
  deEsser: { type: "deEsser", freqHz: 6500, thresholdDb: -30, amountDb: 8 },
  rumbleGuard: { type: "rumbleGuard", freqHz: 90 },
};

const fail = (what: string) => (err: unknown) => console.error(`${what} failed:`, err);

type AudioFiltersDialogProps = {
  source: Source;
  collection: Collection | null;
  onClose: () => void;
};

/**
 * A source's ordered audio filter chain — owned classic DSP (the denoiser is
 * spectral suppression, no ML), applied before the fader.
 */
export function AudioFiltersDialog({ source, collection, onClose }: AudioFiltersDialogProps) {
  const t = useT();
  const [addOpen, setAddOpen] = useState(false);
  // Wraps the trigger *and* the menu — see `useDismiss`.
  const addMenuRef = useRef<HTMLDivElement>(null);
  useDismiss(addOpen, addMenuRef, () => setAddOpen(false));
  const filters = source.audio?.filters ?? [];

  // Arm per-filter live metering for this strip while the dialog is open; the
  // meters ride the ~20 Hz `audio` event so every plugin graph moves.
  const [meters, setMeters] = useState<Record<string, { inPeak: number; outPeak: number }>>({});
  useEffect(() => {
    audioArmFilterMeters(source.id).catch(() => {});
    // `listen` resolves on a microtask; if this effect is torn down first
    // (StrictMode remount, fast close) capture the unlisten and call it, so the
    // ~20 Hz listener can't leak past the dialog.
    let alive = true;
    let unlisten: UnlistenFn | undefined;
    onAudio((levels) => {
      if (levels.filterMeters && levels.filterMeters.source === source.id) {
        const next: Record<string, { inPeak: number; outPeak: number }> = {};
        for (const meter of levels.filterMeters.meters) {
          next[meter.id] = { inPeak: meter.inPeak, outPeak: meter.outPeak };
        }
        setMeters(next);
      }
    })
      .then((un) => {
        if (alive) unlisten = un;
        else un();
      })
      .catch(() => {});
    return () => {
      alive = false;
      audioArmFilterMeters(null).catch(() => {});
      unlisten?.();
    };
  }, [source.id]);

  // Ducking triggers: every *other* audio source in the collection.
  const triggerOptions =
    collection?.sources.filter(
      (candidate) => candidate.id !== source.id && kindHasAudio(candidate.kind),
    ) ?? [];

  const update = (filter: AudioFilter, kind: AudioFilterKind) => {
    studioUpdateAudioFilter(source.id, filter.id, kind).catch(fail("audio filter update"));
  };

  return (
    <PickerShell title={t("audiofilters-title", { name: source.name })} onClose={onClose} wide>
      <div className="flex flex-col gap-3">
        <div className="flex items-center justify-between">
          <span className="text-[11px] font-semibold tracking-wider text-havoc-muted uppercase">
            {t("audiofilters-chain-header")}
          </span>
          <div className="ml-auto flex items-center gap-2">
            <label className="flex items-center gap-1 text-[10px] text-havoc-muted">
              {t("audiofilters-voice-preset")}
              <select
                value=""
                onChange={(event) => {
                  const preset = event.target.value;
                  if (preset) {
                    studioApplyVoicePreset(source.id, preset).catch(fail("voice preset"));
                  }
                }}
                aria-label={t("audiofilters-voice-preset")}
                className="rounded-md border border-white/10 bg-havoc-panel px-1.5 py-0.5 text-[10px] text-havoc-text"
              >
                <option value="">{t("audiofilters-voice-preset-pick")}</option>
                <option value="broadcast">{t("audiofilters-voice-broadcast")}</option>
                <option value="podcast">{t("audiofilters-voice-podcast")}</option>
                <option value="clean">{t("audiofilters-voice-clean")}</option>
                <option value="none">{t("audiofilters-voice-none")}</option>
              </select>
            </label>
            <div className="relative" ref={addMenuRef}>
              <button
                type="button"
                onClick={() => setAddOpen((open) => !open)}
                aria-haspopup="menu"
                aria-expanded={addOpen}
                className="rounded-md border border-white/10 px-2 py-0.5 text-xs text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
              >
                {t("audiofilters-add")}
              </button>
              {addOpen && (
                <div
                  role="menu"
                  aria-label={t("audiofilters-add-menu")}
                  className="absolute right-0 z-20 mt-1 w-44 rounded-lg border border-white/10 bg-havoc-panel p-1 shadow-xl"
                >
                  {(Object.keys(FILTER_DEFAULTS) as AudioFilterTypeName[]).map((type) => (
                    <button
                      key={type}
                      type="button"
                      role="menuitem"
                      onClick={() => {
                        setAddOpen(false);
                        studioAddAudioFilter(source.id, FILTER_DEFAULTS[type]).catch(
                          fail("audio filter add"),
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
        </div>

        {filters.length === 0 ? (
          <p className="m-0 text-xs text-havoc-muted">{t("audiofilters-empty")}</p>
        ) : (
          <ul className="m-0 flex list-none flex-col gap-2 p-0">
            {filters.map((filter, index) => (
              <li key={filter.id} className="rounded-lg border border-white/10 bg-white/[0.02] p-2">
                <div className="flex items-center gap-2">
                  <input
                    type="checkbox"
                    checked={filter.enabled}
                    onChange={(event) =>
                      studioSetAudioFilterEnabled(source.id, filter.id, event.target.checked).catch(
                        fail("audio filter toggle"),
                      )
                    }
                    aria-label={t("audiofilters-enable", {
                      name: t(FILTER_NAME_KEYS[filter.type]),
                    })}
                  />
                  <span className="flex-1 text-xs font-semibold text-havoc-text">
                    {t(FILTER_NAME_KEYS[filter.type])}
                  </span>
                  <button
                    type="button"
                    disabled={index === 0}
                    onClick={() =>
                      studioReorderAudioFilter(source.id, filter.id, index - 1).catch(
                        fail("audio filter reorder"),
                      )
                    }
                    title={t("audiofilters-run-earlier")}
                    aria-label={t("audiofilters-move-up", {
                      name: t(FILTER_NAME_KEYS[filter.type]),
                    })}
                    className="rounded px-1 text-[10px] text-havoc-muted enabled:hover:text-havoc-text disabled:opacity-40"
                  >
                    ▲
                  </button>
                  <button
                    type="button"
                    disabled={index === filters.length - 1}
                    onClick={() =>
                      studioReorderAudioFilter(source.id, filter.id, index + 1).catch(
                        fail("audio filter reorder"),
                      )
                    }
                    title={t("audiofilters-run-later")}
                    aria-label={t("audiofilters-move-down", {
                      name: t(FILTER_NAME_KEYS[filter.type]),
                    })}
                    className="rounded px-1 text-[10px] text-havoc-muted enabled:hover:text-havoc-text disabled:opacity-40"
                  >
                    ▼
                  </button>
                  <button
                    type="button"
                    onClick={() =>
                      studioRemoveAudioFilter(source.id, filter.id).catch(
                        fail("audio filter remove"),
                      )
                    }
                    title={t("audiofilters-remove-title")}
                    aria-label={t("audiofilters-remove", {
                      name: t(FILTER_NAME_KEYS[filter.type]),
                    })}
                    className="rounded px-1 text-xs text-havoc-muted hover:text-red-400"
                  >
                    ×
                  </button>
                </div>
                {filter.type !== "parametricEq" && (
                  <div className="mt-2">
                    <FilterViz filter={filter} meter={meters[filter.id]} />
                  </div>
                )}
                <AudioFilterParams
                  filter={filter}
                  sourceId={source.id}
                  triggers={triggerOptions}
                  onChange={(kind) => update(filter, kind)}
                />
              </li>
            ))}
          </ul>
        )}
      </div>
    </PickerShell>
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
      <span className="w-28 shrink-0">{label}</span>
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

function AudioFilterParams({
  filter,
  sourceId,
  triggers,
  onChange,
}: {
  filter: AudioFilter;
  sourceId: SourceId;
  triggers: Source[];
  onChange: (kind: AudioFilterKind) => void;
}) {
  const t = useT();
  switch (filter.type) {
    case "gain":
      return (
        <div className="mt-2">
          <Slider
            label={t("audiofilters-gain-db")}
            value={filter.db}
            min={-30}
            max={30}
            step={0.5}
            onChange={(db) => onChange({ ...filter, db })}
          />
        </div>
      );
    case "noiseGate":
      return (
        <div className="mt-2 flex flex-col gap-1.5">
          <Slider
            label={t("audiofilters-open-db")}
            value={filter.openThresholdDb}
            min={-96}
            max={0}
            step={1}
            onChange={(openThresholdDb) => onChange({ ...filter, openThresholdDb })}
          />
          <Slider
            label={t("audiofilters-close-db")}
            value={filter.closeThresholdDb}
            min={-96}
            max={0}
            step={1}
            onChange={(closeThresholdDb) => onChange({ ...filter, closeThresholdDb })}
          />
          <Slider
            label={t("audiofilters-attack-ms")}
            value={filter.attackMs}
            min={1}
            max={500}
            step={1}
            onChange={(attackMs) => onChange({ ...filter, attackMs })}
          />
          <Slider
            label={t("audiofilters-hold-ms")}
            value={filter.holdMs}
            min={0}
            max={3000}
            step={10}
            onChange={(holdMs) => onChange({ ...filter, holdMs })}
          />
          <Slider
            label={t("audiofilters-release-ms")}
            value={filter.releaseMs}
            min={1}
            max={3000}
            step={10}
            onChange={(releaseMs) => onChange({ ...filter, releaseMs })}
          />
        </div>
      );
    case "compressor":
      return (
        <div className="mt-2 flex flex-col gap-1.5">
          <Slider
            label={t("audiofilters-ratio")}
            value={filter.ratio}
            min={1}
            max={32}
            step={0.5}
            onChange={(ratio) => onChange({ ...filter, ratio })}
          />
          <Slider
            label={t("audiofilters-threshold-db")}
            value={filter.thresholdDb}
            min={-60}
            max={0}
            step={1}
            onChange={(thresholdDb) => onChange({ ...filter, thresholdDb })}
          />
          <Slider
            label={t("audiofilters-attack-ms")}
            value={filter.attackMs}
            min={0.1}
            max={500}
            step={0.1}
            onChange={(attackMs) => onChange({ ...filter, attackMs })}
          />
          <Slider
            label={t("audiofilters-release-ms")}
            value={filter.releaseMs}
            min={1}
            max={3000}
            step={10}
            onChange={(releaseMs) => onChange({ ...filter, releaseMs })}
          />
          <Slider
            label={t("audiofilters-output-gain-db")}
            value={filter.outputGainDb}
            min={-30}
            max={30}
            step={0.5}
            onChange={(outputGainDb) => onChange({ ...filter, outputGainDb })}
          />
        </div>
      );
    case "limiter":
      return (
        <div className="mt-2 flex flex-col gap-1.5">
          <Slider
            label={t("audiofilters-ceiling-db")}
            value={filter.thresholdDb}
            min={-30}
            max={0}
            step={0.5}
            onChange={(thresholdDb) => onChange({ ...filter, thresholdDb })}
          />
          <Slider
            label={t("audiofilters-release-ms")}
            value={filter.releaseMs}
            min={1}
            max={1000}
            step={5}
            onChange={(releaseMs) => onChange({ ...filter, releaseMs })}
          />
        </div>
      );
    case "eq":
      return (
        <div className="mt-2 flex flex-col gap-1.5">
          <Slider
            label={t("audiofilters-low-db")}
            value={filter.lowDb}
            min={-20}
            max={20}
            step={0.5}
            onChange={(lowDb) => onChange({ ...filter, lowDb })}
          />
          <Slider
            label={t("audiofilters-mid-db")}
            value={filter.midDb}
            min={-20}
            max={20}
            step={0.5}
            onChange={(midDb) => onChange({ ...filter, midDb })}
          />
          <Slider
            label={t("audiofilters-high-db")}
            value={filter.highDb}
            min={-20}
            max={20}
            step={0.5}
            onChange={(highDb) => onChange({ ...filter, highDb })}
          />
        </div>
      );
    case "denoise":
      return (
        <div className="mt-2 flex flex-col gap-1.5">
          <Slider
            label={t("audiofilters-strength")}
            value={filter.strength}
            min={0}
            max={1}
            step={0.05}
            onChange={(strength) => onChange({ ...filter, strength })}
          />
          <p className="m-0 text-[10px] leading-snug text-havoc-muted">
            {t("audiofilters-denoise-note")}
          </p>
        </div>
      );
    case "ducker":
      return (
        <div className="mt-2 flex flex-col gap-1.5">
          <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
            <span className="w-28 shrink-0">{t("audiofilters-duck-under")}</span>
            <select
              value={filter.trigger ?? ""}
              onChange={(event) =>
                onChange({ ...filter, trigger: (event.target.value || null) as SourceId | null })
              }
              aria-label={t("audiofilters-ducking-trigger")}
              className="flex-1 rounded-md border border-white/10 bg-havoc-panel px-2 py-1 text-xs text-havoc-text"
            >
              <option value="">{t("audiofilters-pick-trigger")}</option>
              {triggers.map((trigger) => (
                <option key={trigger.id} value={trigger.id}>
                  {trigger.name}
                </option>
              ))}
            </select>
          </label>
          <Slider
            label={t("audiofilters-trigger-at-db")}
            value={filter.thresholdDb}
            min={-96}
            max={0}
            step={1}
            onChange={(thresholdDb) => onChange({ ...filter, thresholdDb })}
          />
          <Slider
            label={t("audiofilters-duck-by-db")}
            value={filter.amountDb}
            min={0}
            max={60}
            step={1}
            onChange={(amountDb) => onChange({ ...filter, amountDb })}
          />
          <Slider
            label={t("audiofilters-attack-ms")}
            value={filter.attackMs}
            min={1}
            max={1000}
            step={5}
            onChange={(attackMs) => onChange({ ...filter, attackMs })}
          />
          <Slider
            label={t("audiofilters-release-ms")}
            value={filter.releaseMs}
            min={1}
            max={5000}
            step={10}
            onChange={(releaseMs) => onChange({ ...filter, releaseMs })}
          />
        </div>
      );
    case "parametricEq":
      return (
        <ParametricEqEditor
          sourceId={sourceId}
          bands={filter.bands}
          onChange={(bands) => onChange({ type: "parametricEq", bands })}
        />
      );
    case "deEsser":
      return (
        <div className="mt-2 flex flex-col gap-1.5">
          <Slider
            label={t("audiofilters-deesser-freq")}
            value={filter.freqHz}
            min={3000}
            max={12000}
            step={100}
            onChange={(freqHz) => onChange({ ...filter, freqHz })}
          />
          <Slider
            label={t("audiofilters-trigger-at-db")}
            value={filter.thresholdDb}
            min={-60}
            max={0}
            step={1}
            onChange={(thresholdDb) => onChange({ ...filter, thresholdDb })}
          />
          <Slider
            label={t("audiofilters-deesser-amount")}
            value={filter.amountDb}
            min={0}
            max={24}
            step={0.5}
            onChange={(amountDb) => onChange({ ...filter, amountDb })}
          />
        </div>
      );
    case "rumbleGuard":
      return (
        <div className="mt-2">
          <Slider
            label={t("audiofilters-rumble-freq")}
            value={filter.freqHz}
            min={20}
            max={300}
            step={5}
            onChange={(freqHz) => onChange({ ...filter, freqHz })}
          />
        </div>
      );
  }
}
