import { useState } from "react";

import {
  studioAddAudioFilter,
  studioRemoveAudioFilter,
  studioReorderAudioFilter,
  studioSetAudioFilterEnabled,
  studioUpdateAudioFilter,
} from "../api/commands";
import type {
  AudioFilter,
  AudioFilterKind,
  AudioFilterTypeName,
  Collection,
  Source,
  SourceId,
} from "../api/types";
import { kindHasAudio } from "../api/types";
import { PickerShell } from "./PickerShell";

const FILTER_NAMES: Record<AudioFilterTypeName, string> = {
  gain: "Gain",
  noiseGate: "Noise Gate",
  compressor: "Compressor",
  limiter: "Limiter",
  eq: "3-Band EQ",
  denoise: "Denoise",
  ducker: "Ducking",
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
  const [addOpen, setAddOpen] = useState(false);
  const filters = source.audio?.filters ?? [];

  // Ducking triggers: every *other* audio source in the collection.
  const triggerOptions =
    collection?.sources.filter(
      (candidate) => candidate.id !== source.id && kindHasAudio(candidate.kind),
    ) ?? [];

  const update = (filter: AudioFilter, kind: AudioFilterKind) => {
    studioUpdateAudioFilter(source.id, filter.id, kind).catch(fail("audio filter update"));
  };

  return (
    <PickerShell title={`Audio filters — ${source.name}`} onClose={onClose} wide>
      <div className="flex flex-col gap-3">
        <div className="flex items-center justify-between">
          <span className="text-[11px] font-semibold tracking-wider text-havoc-muted uppercase">
            Filter chain (top runs first, before the fader)
          </span>
          <div className="relative">
            <button
              type="button"
              onClick={() => setAddOpen((open) => !open)}
              aria-haspopup="menu"
              aria-expanded={addOpen}
              className="rounded-md border border-white/10 px-2 py-0.5 text-xs text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
            >
              + Add filter
            </button>
            {addOpen && (
              <div
                role="menu"
                aria-label="Add an audio filter"
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
                    {FILTER_NAMES[type]}
                  </button>
                ))}
              </div>
            )}
          </div>
        </div>

        {filters.length === 0 ? (
          <p className="m-0 text-xs text-havoc-muted">
            No filters yet — denoise a mic (classic DSP, no ML), gate the room, tame peaks with the
            compressor, or duck music under your voice.
          </p>
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
                    aria-label={`Enable ${FILTER_NAMES[filter.type]}`}
                  />
                  <span className="flex-1 text-xs font-semibold text-havoc-text">
                    {FILTER_NAMES[filter.type]}
                  </span>
                  <button
                    type="button"
                    disabled={index === 0}
                    onClick={() =>
                      studioReorderAudioFilter(source.id, filter.id, index - 1).catch(
                        fail("audio filter reorder"),
                      )
                    }
                    title="Run earlier"
                    aria-label={`Move ${FILTER_NAMES[filter.type]} up`}
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
                    title="Run later"
                    aria-label={`Move ${FILTER_NAMES[filter.type]} down`}
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
                    title="Remove filter"
                    aria-label={`Remove ${FILTER_NAMES[filter.type]}`}
                    className="rounded px-1 text-xs text-havoc-muted hover:text-red-400"
                  >
                    ×
                  </button>
                </div>
                <AudioFilterParams
                  filter={filter}
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
  triggers,
  onChange,
}: {
  filter: AudioFilter;
  triggers: Source[];
  onChange: (kind: AudioFilterKind) => void;
}) {
  switch (filter.type) {
    case "gain":
      return (
        <div className="mt-2">
          <Slider
            label="Gain (dB)"
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
            label="Open at (dB)"
            value={filter.openThresholdDb}
            min={-96}
            max={0}
            step={1}
            onChange={(openThresholdDb) => onChange({ ...filter, openThresholdDb })}
          />
          <Slider
            label="Close at (dB)"
            value={filter.closeThresholdDb}
            min={-96}
            max={0}
            step={1}
            onChange={(closeThresholdDb) => onChange({ ...filter, closeThresholdDb })}
          />
          <Slider
            label="Attack (ms)"
            value={filter.attackMs}
            min={1}
            max={500}
            step={1}
            onChange={(attackMs) => onChange({ ...filter, attackMs })}
          />
          <Slider
            label="Hold (ms)"
            value={filter.holdMs}
            min={0}
            max={3000}
            step={10}
            onChange={(holdMs) => onChange({ ...filter, holdMs })}
          />
          <Slider
            label="Release (ms)"
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
            label="Ratio (:1)"
            value={filter.ratio}
            min={1}
            max={32}
            step={0.5}
            onChange={(ratio) => onChange({ ...filter, ratio })}
          />
          <Slider
            label="Threshold (dB)"
            value={filter.thresholdDb}
            min={-60}
            max={0}
            step={1}
            onChange={(thresholdDb) => onChange({ ...filter, thresholdDb })}
          />
          <Slider
            label="Attack (ms)"
            value={filter.attackMs}
            min={0.1}
            max={500}
            step={0.1}
            onChange={(attackMs) => onChange({ ...filter, attackMs })}
          />
          <Slider
            label="Release (ms)"
            value={filter.releaseMs}
            min={1}
            max={3000}
            step={10}
            onChange={(releaseMs) => onChange({ ...filter, releaseMs })}
          />
          <Slider
            label="Output gain (dB)"
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
            label="Ceiling (dB)"
            value={filter.thresholdDb}
            min={-30}
            max={0}
            step={0.5}
            onChange={(thresholdDb) => onChange({ ...filter, thresholdDb })}
          />
          <Slider
            label="Release (ms)"
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
            label="Low (dB)"
            value={filter.lowDb}
            min={-20}
            max={20}
            step={0.5}
            onChange={(lowDb) => onChange({ ...filter, lowDb })}
          />
          <Slider
            label="Mid (dB)"
            value={filter.midDb}
            min={-20}
            max={20}
            step={0.5}
            onChange={(midDb) => onChange({ ...filter, midDb })}
          />
          <Slider
            label="High (dB)"
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
            label="Strength"
            value={filter.strength}
            min={0}
            max={1}
            step={0.05}
            onChange={(strength) => onChange({ ...filter, strength })}
          />
          <p className="m-0 text-[10px] leading-snug text-havoc-muted">
            Owned classic-DSP spectral suppression — steady noise (fans, hiss) drops while speech
            passes. No ML, no models, per the charter.
          </p>
        </div>
      );
    case "ducker":
      return (
        <div className="mt-2 flex flex-col gap-1.5">
          <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
            <span className="w-28 shrink-0">Duck under</span>
            <select
              value={filter.trigger ?? ""}
              onChange={(event) =>
                onChange({ ...filter, trigger: (event.target.value || null) as SourceId | null })
              }
              aria-label="Ducking trigger source"
              className="flex-1 rounded-md border border-white/10 bg-havoc-panel px-2 py-1 text-xs text-havoc-text"
            >
              <option value="">(pick a trigger — e.g. your mic)</option>
              {triggers.map((trigger) => (
                <option key={trigger.id} value={trigger.id}>
                  {trigger.name}
                </option>
              ))}
            </select>
          </label>
          <Slider
            label="Trigger at (dB)"
            value={filter.thresholdDb}
            min={-96}
            max={0}
            step={1}
            onChange={(thresholdDb) => onChange({ ...filter, thresholdDb })}
          />
          <Slider
            label="Duck by (dB)"
            value={filter.amountDb}
            min={0}
            max={60}
            step={1}
            onChange={(amountDb) => onChange({ ...filter, amountDb })}
          />
          <Slider
            label="Attack (ms)"
            value={filter.attackMs}
            min={1}
            max={1000}
            step={5}
            onChange={(attackMs) => onChange({ ...filter, attackMs })}
          />
          <Slider
            label="Release (ms)"
            value={filter.releaseMs}
            min={1}
            max={5000}
            step={10}
            onChange={(releaseMs) => onChange({ ...filter, releaseMs })}
          />
        </div>
      );
  }
}
