import { useEffect, useState } from "react";

import {
  audioInputDevices,
  audioLoopbackDevices,
  cameraControlSet,
  cameraControlsList,
  cameraProfileReset,
  captureListSources,
  replayRollSource,
  studioPlaylistControl,
  studioPlaylistCue,
  studioRenameSource,
  studioSplitControl,
  studioTimerControl,
  studioTitleFire,
  studioTitleSetText,
  studioUpdateSourceSettings,
  videoDeviceFormats,
  videoDevicesList,
} from "../api/commands";
import type {
  AudioDevice,
  CameraControl,
  CaptureSource,
  CountdownEnd,
  DeinterlaceMode,
  FieldOrder,
  FileBinding,
  InputLayout,
  PlaylistEntry,
  ReplaySpeed,
  Source,
  SourceSettings,
  TextAlign,
  SplitComparison,
  TimerMode,
  TitleAnimation,
  TitleLayer,
  VideoDevice,
  VideoFormat,
  VisStyle,
} from "../api/types";
import { hexToRgba, rgbaToHex } from "../lib/color";
import { INPUT_LAYOUTS, REPLAY_SPEEDS, VIS_STYLES } from "../lib/sourceOptions";
import { titleTextLayer } from "../lib/titleLayers";
import { parseVisTarget, visTargetKey } from "../lib/visTarget";
import { useT } from "../i18n/t";
import { LanIngestFields } from "./LanIngestFields";
import { NumberField } from "./NumberField";
import { PickerShell } from "./PickerShell";

const inputClass =
  "rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text outline-none focus:border-havoc-accent/60";

/** Comma-separated cue seconds with a local draft (NumberField's idiom).
 * Parsing on every keystroke made the field unusable: the controlled value
 * snapped back mid-word, and a trailing comma's empty segment parsed as
 * Number("") === 0, injecting phantom 0 s cues. The draft commits on blur
 * or Enter; empty segments are dropped, not zeroed. */
function CueListField({
  label,
  cues,
  onCommit,
}: {
  label: string;
  cues: number[];
  onCommit: (cues: number[]) => void;
}) {
  const [draft, setDraft] = useState<string | null>(null);
  const commit = () => {
    if (draft === null) return;
    onCommit(
      draft
        .split(",")
        .map((cue) => cue.trim())
        .filter((cue) => cue.length > 0)
        .map(Number)
        .filter((cue) => Number.isFinite(cue) && cue >= 0),
    );
    setDraft(null);
  };
  return (
    <label className="flex flex-1 flex-col gap-1 text-[11px] text-havoc-muted">
      {label}
      <input
        value={draft ?? cues.join(", ")}
        onChange={(event) => setDraft(event.target.value)}
        onBlur={commit}
        onKeyDown={(event) => {
          if (event.key === "Enter") commit();
        }}
        placeholder="10, 42.5"
        className={`${inputClass} font-mono`}
      />
    </label>
  );
}

/** A fresh copy with rows `index` and `index + 1` swapped — the ↑/↓ move. */
function swapAt<T>(list: T[], index: number): T[] {
  const next = [...list];
  [next[index], next[index + 1]] = [next[index + 1], next[index]];
  return next;
}

/** The ↑/↓/× buttons shared by the playlist-item and title-layer rows.
 * `onSwap(at)` swaps rows `at` and `at + 1`; labels are the callers'
 * existing i18n strings. */
function RowControls({
  index,
  count,
  upLabel,
  downLabel,
  removeLabel,
  onSwap,
  onRemove,
}: {
  index: number;
  count: number;
  upLabel: string;
  downLabel: string;
  removeLabel: string;
  onSwap: (at: number) => void;
  onRemove: () => void;
}) {
  return (
    <>
      <button
        type="button"
        disabled={index === 0}
        onClick={() => onSwap(index - 1)}
        aria-label={upLabel}
        className="rounded border border-white/10 px-1.5 py-1 text-[11px] text-havoc-muted enabled:hover:text-havoc-text disabled:opacity-40"
      >
        ↑
      </button>
      <button
        type="button"
        disabled={index === count - 1}
        onClick={() => onSwap(index)}
        aria-label={downLabel}
        className="rounded border border-white/10 px-1.5 py-1 text-[11px] text-havoc-muted enabled:hover:text-havoc-text disabled:opacity-40"
      >
        ↓
      </button>
      <button
        type="button"
        onClick={onRemove}
        aria-label={removeLabel}
        className="rounded border border-white/10 px-1.5 py-1 text-[11px] text-havoc-muted hover:text-red-400"
      >
        ×
      </button>
    </>
  );
}

type PropertiesDialogProps = {
  source: Source;
  /** The scenes a Nested Scene source can point at (cycle-checked on Apply). */
  scenes?: Array<{ id: string; name: string }>;
  /** The audio-capable sources a visualizer (CAP-N15) can listen to. */
  audioSources?: Array<{ id: string; name: string }>;
  onClose: () => void;
};

/** Per-kind source settings + rename. Apply pushes to the engine live. */
export function PropertiesDialog({
  source,
  scenes = [],
  audioSources = [],
  onClose,
}: PropertiesDialogProps) {
  const t = useT();
  const [name, setName] = useState(source.name);
  const [draft, setDraft] = useState<SourceSettings>(() => {
    // A Source is its settings plus identity (+ the audio strip) — peel
    // everything that isn't per-kind settings off.
    const settings: Partial<Source> = { ...source };
    delete settings.id;
    delete settings.name;
    delete settings.audio;
    return settings as SourceSettings;
  });
  const [error, setError] = useState<string | null>(null);

  const apply = () => {
    setError(null);
    const jobs: Promise<unknown>[] = [studioUpdateSourceSettings(source.id, draft)];
    if (name.trim() && name.trim() !== source.name) {
      jobs.push(studioRenameSource(source.id, name.trim()));
    }
    Promise.all(jobs)
      .then(() => onClose())
      .catch((err) => setError(String(err)));
  };

  return (
    <PickerShell title={t("properties-title", { name: source.name })} onClose={onClose} wide>
      <div className="flex flex-col gap-3">
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("properties-name")}
          <input
            value={name}
            onChange={(event) => setName(event.target.value)}
            className={inputClass}
          />
        </label>

        <SettingsEditor
          draft={draft}
          scenes={scenes}
          audioSources={audioSources}
          sourceId={source.id}
          onChange={setDraft}
        />

        {error && (
          <p role="alert" className="m-0 text-xs text-red-400">
            {error}
          </p>
        )}
        <div className="flex justify-end gap-2">
          <button
            type="button"
            onClick={onClose}
            className="rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-muted hover:text-havoc-text"
          >
            {t("properties-cancel")}
          </button>
          <button
            type="button"
            onClick={apply}
            className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
          >
            {t("properties-apply")}
          </button>
        </div>
      </div>
    </PickerShell>
  );
}

function SettingsEditor({
  draft,
  scenes,
  audioSources,
  sourceId,
  onChange,
}: {
  draft: SourceSettings;
  scenes: Array<{ id: string; name: string }>;
  audioSources: Array<{ id: string; name: string }>;
  sourceId: string;
  onChange: (settings: SourceSettings) => void;
}) {
  const t = useT();
  switch (draft.kind) {
    case "chatOverlay":
      return (
        <div className="flex flex-col gap-2">
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t("properties-youtube")}
            <input
              value={draft.youtube}
              onChange={(event) => onChange({ ...draft, youtube: event.target.value })}
              className={`${inputClass} font-mono`}
            />
          </label>
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t("properties-twitch")}
            <input
              value={draft.twitch}
              onChange={(event) => onChange({ ...draft, twitch: event.target.value })}
              className={`${inputClass} font-mono`}
            />
          </label>
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t("properties-kick")}
            <input
              value={draft.kick}
              onChange={(event) => onChange({ ...draft, kick: event.target.value })}
              className={`${inputClass} font-mono`}
            />
          </label>
          <div className="grid grid-cols-3 gap-2">
            <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
              {t("properties-width-px")}
              <input
                type="number"
                min={120}
                max={3840}
                value={draft.width}
                onChange={(event) => onChange({ ...draft, width: Number(event.target.value) })}
                className={inputClass}
              />
            </label>
            <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
              {t("properties-lines")}
              <input
                type="number"
                min={1}
                max={50}
                value={draft.maxLines}
                onChange={(event) => onChange({ ...draft, maxLines: Number(event.target.value) })}
                className={inputClass}
              />
            </label>
            <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
              {t("properties-font-px")}
              <input
                type="number"
                min={10}
                max={96}
                value={draft.fontSize}
                onChange={(event) => onChange({ ...draft, fontSize: Number(event.target.value) })}
                className={inputClass}
              />
            </label>
          </div>
        </div>
      );
    case "slideshow":
      return (
        <div className="flex flex-col gap-2">
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t("properties-images")}
            <textarea
              value={draft.paths.join("\n")}
              onChange={(event) =>
                onChange({
                  ...draft,
                  paths: event.target.value
                    .split("\n")
                    .map((line) => line.trim())
                    .filter(Boolean),
                })
              }
              rows={5}
              className={`${inputClass} font-mono`}
            />
          </label>
          <div className="grid grid-cols-2 gap-2">
            <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
              {t("properties-per-slide")}
              <input
                type="number"
                min={100}
                value={draft.slideMs}
                onChange={(event) => onChange({ ...draft, slideMs: Number(event.target.value) })}
                className={inputClass}
              />
            </label>
            <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
              {t("properties-crossfade")}
              <input
                type="number"
                min={0}
                max={5000}
                value={draft.transitionMs}
                onChange={(event) =>
                  onChange({ ...draft, transitionMs: Number(event.target.value) })
                }
                className={inputClass}
              />
            </label>
          </div>
          <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
            <input
              type="checkbox"
              checked={draft.loop}
              onChange={(event) => onChange({ ...draft, loop: event.target.checked })}
            />
            {t("properties-loop-slideshow")}
          </label>
          <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
            <input
              type="checkbox"
              checked={draft.shuffle}
              onChange={(event) => onChange({ ...draft, shuffle: event.target.checked })}
            />
            {t("properties-shuffle")}
          </label>
        </div>
      );
    case "nestedScene":
      return (
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("properties-nested-scene")}
          <select
            value={draft.scene}
            onChange={(event) => onChange({ ...draft, scene: event.target.value })}
            className={inputClass}
          >
            {scenes.map((entry) => (
              <option key={entry.id} value={entry.id}>
                {entry.name}
              </option>
            ))}
          </select>
        </label>
      );
    case "display":
    case "window":
      return <CaptureRepick draft={draft} onChange={onChange} />;
    case "portal":
      return (
        <p className="m-0 text-xs leading-relaxed text-havoc-muted">
          {t("properties-portal-note")}
        </p>
      );
    case "videoDevice":
      return <VideoDeviceEditor draft={draft} onChange={onChange} />;
    case "audioInput":
    case "audioOutput":
      return <AudioDeviceEditor draft={draft} onChange={onChange} />;
    case "appAudio":
      return (
        <p className="m-0 text-xs leading-relaxed text-havoc-muted">
          {t("properties-appaudio-capturing", {
            exe: draft.exe || t("properties-appaudio-exe-fallback"),
          })}
          {draft.pid ? ` ${t("properties-appaudio-pid", { pid: draft.pid })}` : ""}.{" "}
          {t("properties-appaudio-note")}
        </p>
      );
    case "image":
      return (
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("properties-image-file")}
          <input
            value={draft.path}
            onChange={(event) => onChange({ ...draft, path: event.target.value })}
            placeholder="C:\art\overlay.png"
            className={inputClass}
          />
        </label>
      );
    case "media":
      return (
        <div className="flex flex-col gap-2">
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t("properties-media-file")}
            <input
              value={draft.path}
              onChange={(event) => onChange({ ...draft, path: event.target.value })}
              placeholder="C:\clips\intro.mp4"
              className={inputClass}
            />
          </label>
          <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
            <input
              type="checkbox"
              checked={draft.loop}
              onChange={(event) => onChange({ ...draft, loop: event.target.checked })}
            />
            {t("properties-media-loop")}
          </label>
          <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
            <input
              type="checkbox"
              checked={draft.hwDecode}
              onChange={(event) => onChange({ ...draft, hwDecode: event.target.checked })}
            />
            {t("properties-media-hwdecode")}
          </label>
          <p className="m-0 text-[10px] leading-relaxed text-havoc-muted">
            {t("properties-media-note")}
          </p>
        </div>
      );
    case "color":
      return (
        <div className="flex items-end gap-2">
          <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
            {t("properties-color")}
            <input
              type="color"
              value={rgbaToHex(draft.color)}
              onChange={(event) =>
                onChange({ ...draft, color: hexToRgba(event.target.value, draft.color.a) })
              }
              aria-label={t("properties-color")}
              className="h-7 w-12 cursor-pointer rounded border border-white/10 bg-transparent"
            />
          </label>
          <NumberField
            label={t("properties-width")}
            value={draft.width}
            min={1}
            max={16384}
            onCommit={(width) => onChange({ ...draft, width })}
            className="flex-1"
          />
          <NumberField
            label={t("properties-height")}
            value={draft.height}
            min={1}
            max={16384}
            onCommit={(height) => onChange({ ...draft, height })}
            className="flex-1"
          />
        </div>
      );
    case "text":
      return <TextEditor draft={draft} onChange={onChange} />;
    case "title":
      return <TitleEditor draft={draft} sourceId={sourceId} onChange={onChange} />;
    case "timer":
      return <TimerEditor draft={draft} scenes={scenes} sourceId={sourceId} onChange={onChange} />;
    case "systemStats":
      return (
        <div className="flex flex-col gap-2">
          {(
            [
              ["showFps", "properties-stats-show-fps"],
              ["showCpu", "properties-stats-show-cpu"],
              ["showMemory", "properties-stats-show-memory"],
              ["showRenderMs", "properties-stats-show-render"],
              ["showDropped", "properties-stats-show-dropped"],
              ["showBitrate", "properties-stats-show-bitrate"],
              ["showTimecode", "properties-stats-show-timecode"],
            ] as const
          ).map(([field, label]) => (
            <label key={field} className="flex items-center gap-2 text-[11px] text-havoc-muted">
              <input
                type="checkbox"
                checked={draft[field]}
                onChange={(event) => onChange({ ...draft, [field]: event.target.checked })}
              />
              {t(label)}
            </label>
          ))}
          <div className="flex items-end gap-2">
            <NumberField
              label={t("properties-stats-size")}
              value={draft.sizePx}
              min={8}
              max={512}
              onCommit={(sizePx) => onChange({ ...draft, sizePx })}
              className="flex-1"
            />
            <label className="flex items-center gap-2 pb-1 text-[11px] text-havoc-muted">
              {t("properties-color")}
              <input
                type="color"
                value={rgbaToHex(draft.color)}
                onChange={(event) =>
                  onChange({ ...draft, color: hexToRgba(event.target.value, draft.color.a) })
                }
                aria-label={t("properties-color")}
                className="h-7 w-12 cursor-pointer rounded border border-white/10 bg-transparent"
              />
            </label>
          </div>
          <p className="m-0 text-[10px] leading-snug text-havoc-muted">
            {t("properties-stats-note")}
          </p>
        </div>
      );
    case "audioVisualizer": {
      const target = visTargetKey(draft);
      const bound = draft.source ? audioSources.some((entry) => entry.id === draft.source) : true;
      return (
        <div className="flex flex-col gap-2">
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t("sources-visualizer-style-label")}
            <select
              value={draft.style}
              onChange={(event) => onChange({ ...draft, style: event.target.value as VisStyle })}
              className={inputClass}
            >
              {VIS_STYLES.map(([value, label]) => (
                <option key={value} value={value}>
                  {t(label)}
                </option>
              ))}
            </select>
          </label>
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t("sources-visualizer-target-label")}
            <select
              value={target}
              onChange={(event) => {
                const next = parseVisTarget(event.target.value);
                onChange({
                  ...draft,
                  target: next.target,
                  track: next.track ?? draft.track,
                  source: next.source,
                });
              }}
              className={inputClass}
            >
              <option value="master">{t("sources-visualizer-target-master")}</option>
              {[1, 2, 3, 4, 5, 6].map((n) => (
                <option key={n} value={`track:${n}`}>
                  {t("sources-visualizer-target-track", { n })}
                </option>
              ))}
              {audioSources.map((entry) => (
                <option key={entry.id} value={`source:${entry.id}`}>
                  {entry.name}
                </option>
              ))}
              {!bound && draft.source && (
                <option value={`source:${draft.source}`} disabled>
                  {t("properties-vis-missing-source")}
                </option>
              )}
            </select>
          </label>
          <div className="flex items-end gap-2">
            <NumberField
              label={t("properties-width")}
              value={draft.width}
              min={1}
              max={16384}
              onCommit={(width) => onChange({ ...draft, width })}
              className="flex-1"
            />
            <NumberField
              label={t("properties-height")}
              value={draft.height}
              min={1}
              max={16384}
              onCommit={(height) => onChange({ ...draft, height })}
              className="flex-1"
            />
          </div>
          <div className="flex items-end gap-2">
            {draft.style === "bars" && (
              <NumberField
                label={t("properties-vis-bands")}
                value={draft.bands}
                min={8}
                max={128}
                onCommit={(bands) => onChange({ ...draft, bands })}
                className="flex-1"
              />
            )}
            <NumberField
              label={t("properties-vis-decay")}
              value={draft.decay}
              min={6}
              max={120}
              onCommit={(decay) => onChange({ ...draft, decay })}
              className="flex-1"
            />
            <label className="flex items-center gap-2 pb-1 text-[11px] text-havoc-muted">
              {t("properties-color")}
              <input
                type="color"
                value={rgbaToHex(draft.color)}
                onChange={(event) =>
                  onChange({ ...draft, color: hexToRgba(event.target.value, draft.color.a) })
                }
                aria-label={t("properties-color")}
                className="h-7 w-12 cursor-pointer rounded border border-white/10 bg-transparent"
              />
            </label>
          </div>
          <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
            <input
              type="checkbox"
              checked={draft.peakHold}
              onChange={(event) => onChange({ ...draft, peakHold: event.target.checked })}
            />
            {t("properties-vis-peak-hold")}
          </label>
          <p className="m-0 text-[10px] leading-snug text-havoc-muted">
            {t("sources-visualizer-note")}
          </p>
        </div>
      );
    }
    case "inputOverlay":
      return (
        <div className="flex flex-col gap-2">
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t("sources-input-layout-label")}
            <select
              value={draft.layout}
              onChange={(event) =>
                onChange({ ...draft, layout: event.target.value as InputLayout })
              }
              className={inputClass}
            >
              {INPUT_LAYOUTS.map(([value, label]) => (
                <option key={value} value={value}>
                  {t(label)}
                </option>
              ))}
            </select>
          </label>
          <div className="flex items-end gap-3">
            <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
              {t("sources-input-color-label")}
              <input
                type="color"
                value={rgbaToHex(draft.color)}
                onChange={(event) =>
                  onChange({ ...draft, color: hexToRgba(event.target.value, draft.color.a) })
                }
                aria-label={t("sources-input-color-label")}
                className="h-7 w-12 cursor-pointer rounded border border-white/10 bg-transparent"
              />
            </label>
            <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
              {t("sources-input-accent-label")}
              <input
                type="color"
                value={rgbaToHex(draft.accent)}
                onChange={(event) =>
                  onChange({ ...draft, accent: hexToRgba(event.target.value, draft.accent.a) })
                }
                aria-label={t("sources-input-accent-label")}
                className="h-7 w-12 cursor-pointer rounded border border-white/10 bg-transparent"
              />
            </label>
          </div>
          <p className="m-0 text-[10px] leading-snug text-havoc-muted">
            {t("sources-input-privacy-note")}
          </p>
          <p className="m-0 text-[10px] leading-snug text-havoc-muted">
            {t("sources-input-os-note")}
          </p>
        </div>
      );
    case "playlist":
      return (
        <div className="flex flex-col gap-2">
          <span className="text-[11px] text-havoc-muted">{t("properties-playlist-items")}</span>
          <div className="flex max-h-64 flex-col gap-2 overflow-y-auto pr-1">
            {draft.items.map((item, index) => {
              const update = (next: Partial<PlaylistEntry>) =>
                onChange({
                  ...draft,
                  items: draft.items.map((entry, at) =>
                    at === index ? { ...entry, ...next } : entry,
                  ),
                });
              return (
                <div
                  key={index}
                  className="flex flex-col gap-1 rounded-md border border-white/10 p-2"
                >
                  <div className="flex items-center gap-1">
                    <input
                      value={item.path}
                      onChange={(event) => update({ path: event.target.value })}
                      placeholder="C:\vt\clip.mp4"
                      className={`${inputClass} flex-1 font-mono`}
                    />
                    <RowControls
                      index={index}
                      count={draft.items.length}
                      upLabel={t("properties-playlist-up")}
                      downLabel={t("properties-playlist-down")}
                      removeLabel={t("properties-playlist-remove")}
                      onSwap={(at) => onChange({ ...draft, items: swapAt(draft.items, at) })}
                      onRemove={() =>
                        onChange({
                          ...draft,
                          items: draft.items.filter((_, at) => at !== index),
                        })
                      }
                    />
                  </div>
                  <div className="flex items-end gap-2">
                    <NumberField
                      label={t("properties-playlist-in")}
                      value={item.in}
                      min={0}
                      max={359999}
                      onCommit={(value) => update({ in: value })}
                      className="flex-1"
                    />
                    <NumberField
                      label={t("properties-playlist-out")}
                      value={item.out}
                      min={0}
                      max={359999}
                      onCommit={(value) => update({ out: value })}
                      className="flex-1"
                    />
                    <CueListField
                      label={t("properties-playlist-cues")}
                      cues={item.cues}
                      onCommit={(cues) => update({ cues })}
                    />
                  </div>
                  {item.cues.length > 0 && (
                    <div className="flex flex-wrap gap-1">
                      {item.cues.map((cue, cueIndex) => (
                        <button
                          key={cueIndex}
                          type="button"
                          onClick={() =>
                            studioPlaylistCue(sourceId, index, cue).catch((err) =>
                              console.error("playlist cue failed:", err),
                            )
                          }
                          className="rounded border border-white/10 px-2 py-0.5 text-[10px] text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
                        >
                          ▶ {cue}s
                        </button>
                      ))}
                    </div>
                  )}
                </div>
              );
            })}
          </div>
          <button
            type="button"
            onClick={() =>
              onChange({
                ...draft,
                items: [...draft.items, { path: "", in: 0, out: 0, cues: [] }],
              })
            }
            className="self-start rounded-md border border-white/10 px-2 py-1 text-[11px] text-havoc-muted hover:text-havoc-text"
          >
            {t("properties-playlist-add-item")}
          </button>
          <div className="flex flex-wrap gap-3">
            {(
              [
                ["loop", "properties-playlist-loop"],
                ["shuffle", "properties-playlist-shuffle"],
                ["holdLast", "properties-playlist-hold-last"],
                ["hwDecode", "properties-playlist-hw"],
              ] as const
            ).map(([field, label]) => (
              <label key={field} className="flex items-center gap-2 text-[11px] text-havoc-muted">
                <input
                  type="checkbox"
                  checked={draft[field]}
                  onChange={(event) => onChange({ ...draft, [field]: event.target.checked })}
                />
                {t(label)}
              </label>
            ))}
          </div>
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t("properties-playlist-variable")}
            <input
              value={draft.nowPlayingVariable}
              onChange={(event) => onChange({ ...draft, nowPlayingVariable: event.target.value })}
              placeholder="nowPlaying"
              className={`${inputClass} font-mono`}
            />
          </label>
          <div className="flex gap-2">
            <button
              type="button"
              onClick={() =>
                studioPlaylistControl(sourceId, "previous").catch((err) =>
                  console.error("playlist control failed:", err),
                )
              }
              className="rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-text hover:border-havoc-accent/50"
            >
              {t("properties-playlist-previous")}
            </button>
            <button
              type="button"
              onClick={() =>
                studioPlaylistControl(sourceId, "next").catch((err) =>
                  console.error("playlist control failed:", err),
                )
              }
              className="rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-text hover:border-havoc-accent/50"
            >
              {t("properties-playlist-next")}
            </button>
          </div>
          <p className="m-0 text-[10px] leading-snug text-havoc-muted">
            {t("properties-playlist-note")}
          </p>
        </div>
      );
    case "replayPlayback":
      return (
        <div className="flex flex-col gap-2">
          <div className="flex items-end gap-2">
            <NumberField
              label={t("sources-replay-seconds-label")}
              value={draft.seconds}
              min={2}
              max={300}
              onCommit={(seconds) => onChange({ ...draft, seconds })}
              className="flex-1"
            />
            <label className="flex flex-1 flex-col gap-1 text-[11px] text-havoc-muted">
              {t("sources-replay-speed-label")}
              <select
                value={draft.speed}
                onChange={(event) =>
                  onChange({ ...draft, speed: event.target.value as ReplaySpeed })
                }
                className={inputClass}
              >
                {REPLAY_SPEEDS.map(([value, label]) => (
                  <option key={value} value={value}>
                    {t(label)}
                  </option>
                ))}
              </select>
            </label>
          </div>
          <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
            <input
              type="checkbox"
              checked={draft.hwDecode}
              onChange={(event) => onChange({ ...draft, hwDecode: event.target.checked })}
            />
            {t("properties-playlist-hw")}
          </label>
          <button
            type="button"
            onClick={() =>
              replayRollSource(sourceId).catch((err) => console.error("replay roll failed:", err))
            }
            className="self-start rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
          >
            {t("properties-replay-roll")}
          </button>
          <p className="m-0 text-[10px] leading-snug text-havoc-muted">
            {t("properties-replay-note")}
          </p>
        </div>
      );
    case "freallyLink":
      return (
        <div className="flex flex-col gap-2">
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t("sources-link-host")}
            <input
              value={draft.host}
              onChange={(event) => onChange({ ...draft, host: event.target.value })}
              placeholder="192.168.1.20"
              className={`${inputClass} font-mono`}
            />
          </label>
          <NumberField
            label={t("sources-link-port")}
            value={draft.port}
            min={1}
            max={65535}
            onCommit={(port) => onChange({ ...draft, port })}
          />
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t("sources-link-key")}
            <input
              value={draft.key}
              onChange={(event) => onChange({ ...draft, key: event.target.value })}
              className={`${inputClass} font-mono`}
            />
          </label>
          <p className="m-0 text-[10px] leading-snug text-havoc-muted">
            {t("sources-link-key-hint")}
          </p>
          <p className="m-0 text-[10px] leading-snug text-havoc-muted">
            {t("properties-link-note")}
          </p>
        </div>
      );
    case "splitTimer":
      return (
        <div className="flex flex-col gap-2">
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t("sources-splits-file-label")}
            <input
              value={draft.path}
              onChange={(event) => onChange({ ...draft, path: event.target.value })}
              placeholder="C:\runs\any-percent.lss"
              className={`${inputClass} font-mono`}
            />
          </label>
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t("sources-splits-comparison-label")}
            <select
              value={draft.comparison}
              onChange={(event) =>
                onChange({ ...draft, comparison: event.target.value as SplitComparison })
              }
              className={inputClass}
            >
              <option value="personalBest">{t("sources-splits-comparison-pb")}</option>
              <option value="bestSegments">{t("sources-splits-comparison-best")}</option>
              <option value="average">{t("sources-splits-comparison-average")}</option>
            </select>
          </label>
          <div className="flex items-end gap-2">
            <NumberField
              label={t("properties-width")}
              value={draft.width}
              min={1}
              max={16384}
              onCommit={(width) => onChange({ ...draft, width })}
              className="flex-1"
            />
            <NumberField
              label={t("properties-height")}
              value={draft.height}
              min={1}
              max={16384}
              onCommit={(height) => onChange({ ...draft, height })}
              className="flex-1"
            />
            <NumberField
              label={t("properties-splits-size")}
              value={draft.sizePx}
              min={8}
              max={96}
              onCommit={(sizePx) => onChange({ ...draft, sizePx })}
              className="flex-1"
            />
          </div>
          <div className="flex flex-wrap items-center gap-3">
            {(
              [
                ["color", "properties-color"],
                ["ahead", "properties-splits-ahead"],
                ["behind", "properties-splits-behind"],
                ["gold", "properties-splits-gold"],
              ] as const
            ).map(([field, label]) => (
              <label key={field} className="flex items-center gap-2 text-[11px] text-havoc-muted">
                {t(label)}
                <input
                  type="color"
                  value={rgbaToHex(draft[field])}
                  onChange={(event) =>
                    onChange({ ...draft, [field]: hexToRgba(event.target.value, draft[field].a) })
                  }
                  aria-label={t(label)}
                  className="h-7 w-10 cursor-pointer rounded border border-white/10 bg-transparent"
                />
              </label>
            ))}
          </div>
          <div className="flex gap-2">
            {(
              [
                ["split", "properties-splits-split"],
                ["undo", "properties-splits-undo"],
                ["skip", "properties-splits-skip"],
                ["reset", "properties-splits-reset"],
              ] as const
            ).map(([action, label]) => (
              <button
                key={action}
                type="button"
                onClick={() =>
                  studioSplitControl(sourceId, action).catch((err) =>
                    console.error("split control failed:", err),
                  )
                }
                className="rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-text hover:border-havoc-accent/50"
              >
                {t(label)}
              </button>
            ))}
          </div>
          <p className="m-0 text-[10px] leading-snug text-havoc-muted">
            {t("properties-splits-note")}
          </p>
        </div>
      );
    case "lanIngest":
      return <LanIngestEditor draft={draft} onChange={onChange} />;
    case "testBars":
    case "testGrid":
    case "testSweep":
    case "testFlashBeep":
      return (
        <div className="flex items-end gap-2">
          <NumberField
            label={t("properties-width")}
            value={draft.width}
            min={1}
            max={16384}
            onCommit={(width) => onChange({ ...draft, width })}
            className="flex-1"
          />
          <NumberField
            label={t("properties-height")}
            value={draft.height}
            min={1}
            max={16384}
            onCommit={(height) => onChange({ ...draft, height })}
            className="flex-1"
          />
        </div>
      );
    case "testTone":
      return (
        <p className="m-0 text-xs leading-relaxed text-havoc-muted">
          {t("properties-testtone-note")}
        </p>
      );
  }
}

function CaptureRepick({
  draft,
  onChange,
}: {
  draft: Extract<SourceSettings, { kind: "display" | "window" }>;
  onChange: (settings: SourceSettings) => void;
}) {
  const t = useT();
  const [entries, setEntries] = useState<CaptureSource[] | null>(null);

  useEffect(() => {
    let cancelled = false;
    captureListSources()
      .then((all) => {
        if (!cancelled) setEntries(all.filter((s) => s.kind === draft.kind));
      })
      .catch(() => {
        if (!cancelled) setEntries([]);
      });
    return () => {
      cancelled = true;
    };
  }, [draft.kind]);

  return (
    <div className="flex flex-col gap-1.5">
      <span className="text-[11px] text-havoc-muted">
        {t("properties-repick-capturing", { label: draft.label })}
      </span>
      <span className="text-[11px] text-havoc-muted">
        {entries === null
          ? t("properties-repick-looking")
          : entries.length === 0
            ? draft.kind === "display"
              ? t("properties-repick-none-displays")
              : t("properties-repick-none-windows")
            : t("properties-repick-again")}
      </span>
      {entries && entries.length > 0 && (
        <ul className="m-0 flex max-h-48 list-none flex-col gap-1 overflow-auto p-0">
          {entries.map((entry) => (
            <li key={entry.id}>
              <button
                type="button"
                onClick={() =>
                  onChange({ kind: draft.kind, captureId: entry.id, label: entry.label })
                }
                aria-pressed={entry.id === draft.captureId}
                className={`w-full truncate rounded-md border px-2 py-1.5 text-left text-xs text-havoc-text ${
                  entry.id === draft.captureId
                    ? "border-havoc-accent/60 bg-havoc-accent/10"
                    : "border-white/10 hover:border-havoc-accent/50"
                }`}
              >
                {entry.label}
              </button>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}

function VideoDeviceEditor({
  draft,
  onChange,
}: {
  draft: Extract<SourceSettings, { kind: "videoDevice" }>;
  onChange: (settings: SourceSettings) => void;
}) {
  const t = useT();
  const [devices, setDevices] = useState<VideoDevice[] | null>(null);
  // Keyed by device id so switching devices reads as "loading" without a
  // synchronous reset inside the fetch effect.
  const [formatsFor, setFormatsFor] = useState<{ deviceId: string; list: VideoFormat[] } | null>(
    null,
  );

  useEffect(() => {
    let cancelled = false;
    videoDevicesList()
      .then((list) => {
        if (!cancelled) setDevices(list);
      })
      .catch(() => {
        if (!cancelled) setDevices([]);
      });
    return () => {
      cancelled = true;
    };
  }, []);

  useEffect(() => {
    let cancelled = false;
    const deviceId = draft.deviceId;
    videoDeviceFormats(deviceId)
      .then((list) => {
        if (!cancelled) setFormatsFor({ deviceId, list });
      })
      .catch(() => {
        // A device that's live right now can't be probed — Auto still works.
        if (!cancelled) setFormatsFor({ deviceId, list: [] });
      });
    return () => {
      cancelled = true;
    };
  }, [draft.deviceId]);

  const formats = formatsFor?.deviceId === draft.deviceId ? formatsFor.list : null;

  const formatKey = (format: VideoFormat) =>
    `${format.width}x${format.height}@${format.fps}-${format.fourcc}`;
  const current = draft.format ? formatKey(draft.format) : "";

  return (
    <div className="flex flex-col gap-2">
      <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
        {t("properties-device")}
        <select
          value={draft.deviceId}
          onChange={(event) => onChange({ ...draft, deviceId: event.target.value, format: null })}
          className={inputClass}
        >
          {(devices ?? []).map((device) => (
            <option key={device.id} value={device.id}>
              {device.name}
            </option>
          ))}
          {devices !== null && !devices.some((device) => device.id === draft.deviceId) && (
            <option value={draft.deviceId}>{t("properties-video-current-device")}</option>
          )}
        </select>
      </label>
      <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
        {t("properties-format")}
        <select
          value={current}
          onChange={(event) => {
            const picked = (formats ?? []).find(
              (format) => formatKey(format) === event.target.value,
            );
            onChange({ ...draft, format: picked ?? null });
          }}
          className={inputClass}
        >
          <option value="">
            {formats === null ? t("properties-format-auto-loading") : t("properties-format-auto")}
          </option>
          {(formats ?? []).map((format) => (
            <option key={formatKey(format)} value={formatKey(format)}>
              {format.width}×{format.height} @ {format.fps} fps ({format.fourcc})
            </option>
          ))}
        </select>
      </label>
      <div className="flex items-end gap-2">
        <label className="flex flex-1 flex-col gap-1 text-[11px] text-havoc-muted">
          {t("properties-deinterlace")}
          <select
            value={draft.deinterlace}
            onChange={(event) =>
              onChange({ ...draft, deinterlace: event.target.value as DeinterlaceMode })
            }
            className={inputClass}
          >
            <option value="off">{t("properties-deinterlace-off")}</option>
            <option value="discard">{t("properties-deinterlace-discard")}</option>
            <option value="bob">{t("properties-deinterlace-bob")}</option>
            <option value="linear">{t("properties-deinterlace-linear")}</option>
            <option value="blend">{t("properties-deinterlace-blend")}</option>
            <option value="motionAdaptive">{t("properties-deinterlace-adaptive")}</option>
          </select>
        </label>
        {draft.deinterlace !== "off" && (
          <label className="flex flex-1 flex-col gap-1 text-[11px] text-havoc-muted">
            {t("properties-field-order")}
            <select
              value={draft.fieldOrder}
              onChange={(event) =>
                onChange({ ...draft, fieldOrder: event.target.value as FieldOrder })
              }
              className={inputClass}
            >
              <option value="topFirst">{t("properties-field-order-top")}</option>
              <option value="bottomFirst">{t("properties-field-order-bottom")}</option>
            </select>
          </label>
        )}
      </div>
      {draft.deinterlace !== "off" && (
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">
          {t("properties-deinterlace-note")}
        </p>
      )}
      <CameraControlsSection deviceId={draft.deviceId} />
    </div>
  );
}

/** Localized labels for the known control tags; unknown ids show the
 * backend's own name. Values are i18n keys. */
const CAMERA_CONTROL_LABELS: Record<string, string> = {
  brightness: "camera-control-brightness",
  contrast: "camera-control-contrast",
  hue: "camera-control-hue",
  saturation: "camera-control-saturation",
  sharpness: "camera-control-sharpness",
  gamma: "camera-control-gamma",
  whiteBalance: "camera-control-white-balance",
  backlightComp: "camera-control-backlight",
  gain: "camera-control-gain",
  pan: "camera-control-pan",
  tilt: "camera-control-tilt",
  zoom: "camera-control-zoom",
  exposure: "camera-control-exposure",
  iris: "camera-control-iris",
  focus: "camera-control-focus",
};

/**
 * CAP-M18 — the running device's image controls. Values commit on release
 * (each commit also lands in the per-device profile, reapplied on
 * hotplug/restart). An empty list is honest: the device isn't streaming
 * yet, or this backend reports no controls (per-OS reality).
 */
function CameraControlsSection({ deviceId }: { deviceId: string }) {
  const t = useT();
  const [controls, setControls] = useState<CameraControl[] | null>(null);

  useEffect(() => {
    let alive = true;
    const fetchControls = () =>
      cameraControlsList(deviceId)
        .then((list) => {
          if (alive) setControls(list);
        })
        .catch(() => {
          if (alive) setControls([]);
        });
    fetchControls();
    // The device may still be warming up when the dialog opens.
    const retry = window.setTimeout(fetchControls, 1_500);
    return () => {
      alive = false;
      window.clearTimeout(retry);
    };
  }, [deviceId]);

  const refresh = () => {
    cameraControlsList(deviceId)
      .then(setControls)
      .catch(() => setControls([]));
  };

  const reset = () => {
    cameraProfileReset(deviceId)
      .then(() => window.setTimeout(refresh, 400))
      .catch((err) => console.error(err));
  };

  /** Commit one control, holding the committed value in view. Without this
   * the row would snap back to the stale fetched value on release (the
   * capture thread applies the write a frame later). */
  const commit = (control: CameraControl, value: number) => {
    setControls((current) =>
      (current ?? []).map((entry) => (entry.id === control.id ? { ...entry, value } : entry)),
    );
    cameraControlSet(deviceId, control.id, value).catch((err) => console.error(err));
  };

  return (
    <div className="flex flex-col gap-2 border-t border-white/5 pt-2">
      <div className="flex items-center justify-between">
        <p className="m-0 text-[11px] font-semibold uppercase tracking-wide text-havoc-muted">
          {t("camera-controls-title")}
        </p>
        <div className="flex gap-2">
          <button
            type="button"
            onClick={refresh}
            className="rounded border border-white/10 px-2 py-0.5 text-[10px] text-havoc-muted hover:text-havoc-text"
          >
            {t("camera-controls-refresh")}
          </button>
          <button
            type="button"
            disabled={!controls || controls.length === 0}
            onClick={reset}
            className="rounded border border-white/10 px-2 py-0.5 text-[10px] text-havoc-muted enabled:hover:text-havoc-text disabled:opacity-50"
          >
            {t("camera-controls-reset")}
          </button>
        </div>
      </div>
      {!controls || controls.length === 0 ? (
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">
          {t("camera-controls-empty")}
        </p>
      ) : (
        <>
          {controls.map((control) => (
            <CameraControlRow
              key={control.id}
              control={control}
              onCommit={(value) => commit(control, value)}
            />
          ))}
          <p className="m-0 text-[10px] leading-snug text-havoc-muted">
            {t("camera-controls-note")}
          </p>
        </>
      )}
    </div>
  );
}

function CameraControlRow({
  control,
  onCommit,
}: {
  control: CameraControl;
  onCommit: (value: number) => void;
}) {
  const t = useT();
  const [drag, setDrag] = useState<number | null>(null);
  const shown = drag ?? control.value;
  const label = CAMERA_CONTROL_LABELS[control.id]
    ? t(CAMERA_CONTROL_LABELS[control.id])
    : control.name;
  const commit = () => {
    if (drag !== null) {
      onCommit(drag);
      setDrag(null);
    }
  };
  // A control whose backend reports no range (Windows: exposure/focus/zoom)
  // gets a stepper — a slider would need bounds we honestly don't have.
  const ranged = control.min !== undefined && control.max !== undefined;
  return (
    <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
      <span className="w-24 shrink-0 truncate" title={control.name}>
        {label}
      </span>
      {ranged ? (
        <>
          <input
            type="range"
            min={control.min}
            max={control.max}
            step={control.step}
            value={shown}
            disabled={!control.writable}
            onChange={(event) => setDrag(Number(event.target.value))}
            onPointerUp={commit}
            onBlur={commit}
            aria-label={label}
            className="min-w-0 flex-1 accent-havoc-accent disabled:opacity-40"
          />
          <span className="w-14 shrink-0 text-right tabular-nums">{shown}</span>
        </>
      ) : (
        <input
          type="number"
          step={control.step}
          value={shown}
          disabled={!control.writable}
          onChange={(event) => setDrag(Number(event.target.value))}
          onBlur={commit}
          onKeyDown={(event) => {
            if (event.key === "Enter") commit();
          }}
          aria-label={label}
          className={`${inputClass} min-w-0 flex-1 disabled:opacity-40`}
        />
      )}
    </label>
  );
}

function AudioDeviceEditor({
  draft,
  onChange,
}: {
  draft: Extract<SourceSettings, { kind: "audioInput" | "audioOutput" }>;
  onChange: (settings: SourceSettings) => void;
}) {
  const t = useT();
  const [devices, setDevices] = useState<AudioDevice[] | null>(null);
  const [guidance, setGuidance] = useState<string | null>(null);
  const isLoopback = draft.kind === "audioOutput";

  useEffect(() => {
    let cancelled = false;
    if (isLoopback) {
      audioLoopbackDevices()
        .then((result) => {
          if (cancelled) return;
          setDevices(result.devices);
          setGuidance(result.guidance ?? null);
        })
        .catch(() => {
          if (!cancelled) setDevices([]);
        });
    } else {
      audioInputDevices()
        .then((list) => {
          if (!cancelled) setDevices(list);
        })
        .catch(() => {
          if (!cancelled) setDevices([]);
        });
    }
    return () => {
      cancelled = true;
    };
  }, [isLoopback]);

  // Windows loopback (no guidance) has a real "default output" fallback;
  // elsewhere an explicit device pick is the honest requirement.
  const offerDefault = !isLoopback || guidance === null;

  return (
    <div className="flex flex-col gap-2">
      <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
        {isLoopback ? t("properties-audio-capture-of") : t("properties-device")}
        <select
          value={draft.deviceId}
          onChange={(event) => onChange({ ...draft, deviceId: event.target.value })}
          className={inputClass}
        >
          {offerDefault && (
            <option value="">
              {isLoopback
                ? t("properties-audio-default-output")
                : t("properties-audio-default-input")}
            </option>
          )}
          {(devices ?? []).map((device) => (
            <option key={device.id} value={device.id}>
              {device.name}
              {device.isDefault ? ` ${t("properties-audio-default-suffix")}` : ""}
            </option>
          ))}
          {devices !== null &&
            draft.deviceId !== "" &&
            !devices.some((device) => device.id === draft.deviceId) && (
              <option value={draft.deviceId}>
                {t("properties-audio-current-device", { id: draft.deviceId })}
              </option>
            )}
        </select>
      </label>
      {guidance && (
        <p className="m-0 rounded-md border border-amber-400/20 bg-amber-400/5 p-2 text-[10px] leading-snug text-amber-200/90">
          {guidance}
        </p>
      )}
    </div>
  );
}

/** The five CAP-M15 faces + the countdown end actions. Values are i18n keys. */
const TIMER_EDITOR_MODES: Array<[TimerMode, string]> = [
  ["wallClock", "sources-timer-wall-clock"],
  ["countdown", "sources-timer-countdown"],
  ["stopwatch", "sources-timer-stopwatch"],
  ["sinceLive", "sources-timer-since-live"],
  ["sinceRecording", "sources-timer-since-recording"],
];
const TIMER_END_ACTIONS: Array<[CountdownEnd, string]> = [
  ["none", "properties-timer-end-none"],
  ["flash", "properties-timer-end-flash"],
  ["switchScene", "properties-timer-end-switch"],
];

function TimerEditor({
  draft,
  scenes,
  sourceId,
  onChange,
}: {
  draft: Extract<SourceSettings, { kind: "timer" }>;
  scenes: Array<{ id: string; name: string }>;
  sourceId: string;
  onChange: (settings: SourceSettings) => void;
}) {
  const t = useT();
  // A wall-clock target runs by itself; Start/Pause/Reset drive the rest.
  const runControls =
    draft.mode === "stopwatch" || (draft.mode === "countdown" && draft.target.trim() === "");
  const control = (action: "start" | "pause" | "reset") => {
    studioTimerControl(sourceId, action).catch((err) => console.error(err));
  };
  return (
    <div className="flex flex-col gap-2">
      <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
        {t("sources-timer-mode-label")}
        <select
          value={draft.mode}
          onChange={(event) => onChange({ ...draft, mode: event.target.value as TimerMode })}
          className={inputClass}
        >
          {TIMER_EDITOR_MODES.map(([value, label]) => (
            <option key={value} value={value}>
              {t(label)}
            </option>
          ))}
        </select>
      </label>
      {draft.mode === "wallClock" && (
        <>
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t("properties-timer-format")}
            <input
              value={draft.format}
              onChange={(event) => onChange({ ...draft, format: event.target.value })}
              placeholder="%H:%M:%S"
              className={`${inputClass} font-mono`}
            />
          </label>
          <p className="m-0 text-[10px] leading-snug text-havoc-muted">
            {t("properties-timer-format-note")}
          </p>
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t("properties-timer-utc")}
            <input
              type="number"
              min={-840}
              max={840}
              value={draft.utcOffsetMin ?? ""}
              onChange={(event) =>
                onChange({
                  ...draft,
                  utcOffsetMin:
                    event.target.value === "" ? null : Math.round(Number(event.target.value)),
                })
              }
              placeholder={t("properties-timer-utc-placeholder")}
              className={inputClass}
            />
          </label>
        </>
      )}
      {draft.mode === "countdown" && (
        <>
          <div className="flex items-end gap-2">
            <NumberField
              label={t("properties-timer-duration")}
              value={Math.round(draft.countdownMs / 1000)}
              min={1}
              max={86_400}
              onCommit={(seconds) =>
                onChange({ ...draft, countdownMs: Math.round(seconds) * 1000 })
              }
              className="flex-1"
            />
            <label className="flex flex-1 flex-col gap-1 text-[11px] text-havoc-muted">
              {t("properties-timer-target")}
              <input
                value={draft.target}
                onChange={(event) => onChange({ ...draft, target: event.target.value })}
                placeholder="19:30"
                className={`${inputClass} font-mono`}
              />
            </label>
          </div>
          <p className="m-0 text-[10px] leading-snug text-havoc-muted">
            {t("properties-timer-target-note")}
          </p>
          <div className="flex items-end gap-2">
            <label className="flex flex-1 flex-col gap-1 text-[11px] text-havoc-muted">
              {t("properties-timer-end")}
              <select
                value={draft.endAction}
                onChange={(event) =>
                  onChange({ ...draft, endAction: event.target.value as CountdownEnd })
                }
                className={inputClass}
              >
                {TIMER_END_ACTIONS.map(([value, label]) => (
                  <option key={value} value={value}>
                    {t(label)}
                  </option>
                ))}
              </select>
            </label>
            {draft.endAction === "switchScene" && (
              <label className="flex flex-1 flex-col gap-1 text-[11px] text-havoc-muted">
                {t("properties-timer-end-scene")}
                <select
                  value={draft.endScene ?? ""}
                  onChange={(event) => onChange({ ...draft, endScene: event.target.value || null })}
                  className={inputClass}
                >
                  <option value="">—</option>
                  {scenes.map((scene) => (
                    <option key={scene.id} value={scene.id}>
                      {scene.name}
                    </option>
                  ))}
                </select>
              </label>
            )}
          </div>
        </>
      )}
      <div className="flex items-end gap-2">
        <NumberField
          label={t("properties-timer-size")}
          value={draft.sizePx}
          min={4}
          max={512}
          onCommit={(sizePx) => onChange({ ...draft, sizePx })}
          className="flex-1"
        />
        <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
          {t("properties-color")}
          <input
            type="color"
            value={rgbaToHex(draft.color)}
            onChange={(event) =>
              onChange({ ...draft, color: hexToRgba(event.target.value, draft.color.a) })
            }
            aria-label={t("properties-color")}
            className="h-7 w-12 cursor-pointer rounded border border-white/10 bg-transparent"
          />
        </label>
      </div>
      {runControls && (
        <div className="flex gap-2">
          {(
            [
              ["start", "properties-timer-start"],
              ["pause", "properties-timer-pause"],
              ["reset", "properties-timer-reset"],
            ] as const
          ).map(([action, label]) => (
            <button
              key={action}
              type="button"
              onClick={() => control(action)}
              className="rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-muted hover:text-havoc-text"
            >
              {t(label)}
            </button>
          ))}
        </div>
      )}
    </div>
  );
}

/** CAP-N16: the compact layer editor + live fire/edit controls. A separate
 * control dock is out of v1 — the live controls live here, said in-product
 * by the note below the buttons. */
function TitleEditor({
  draft,
  sourceId,
  onChange,
}: {
  draft: Extract<SourceSettings, { kind: "title" }>;
  sourceId: string;
  onChange: (settings: SourceSettings) => void;
}) {
  const t = useT();
  const updateLayer = (index: number, next: TitleLayer) =>
    onChange({
      ...draft,
      layers: draft.layers.map((layer, at) => (at === index ? next : layer)),
    });
  const addLayer = (layer: TitleLayer) => onChange({ ...draft, layers: [...draft.layers, layer] });
  const kindLabel = (layer: TitleLayer) =>
    layer.kind === "text"
      ? t("properties-title-kind-text")
      : layer.kind === "image"
        ? t("properties-title-kind-image")
        : t("properties-title-kind-rect");
  return (
    <div className="flex flex-col gap-2">
      <div className="flex items-end gap-2">
        <NumberField
          label={t("properties-width")}
          value={draft.width}
          min={16}
          max={16384}
          onCommit={(width) => onChange({ ...draft, width })}
          className="flex-1"
        />
        <NumberField
          label={t("properties-height")}
          value={draft.height}
          min={16}
          max={16384}
          onCommit={(height) => onChange({ ...draft, height })}
          className="flex-1"
        />
        <label className="flex flex-1 flex-col gap-1 text-[11px] text-havoc-muted">
          {t("properties-title-animation")}
          <select
            value={draft.animation}
            onChange={(event) =>
              onChange({ ...draft, animation: event.target.value as TitleAnimation })
            }
            className={inputClass}
          >
            <option value="none">{t("properties-title-anim-none")}</option>
            <option value="fade">{t("properties-title-anim-fade")}</option>
            <option value="slideLeft">{t("properties-title-anim-slide-left")}</option>
            <option value="slideUp">{t("properties-title-anim-slide-up")}</option>
            <option value="wipe">{t("properties-title-anim-wipe")}</option>
          </select>
        </label>
        <NumberField
          label={t("properties-title-duration")}
          value={draft.durationMs}
          min={0}
          max={10000}
          onCommit={(durationMs) => onChange({ ...draft, durationMs: Math.round(durationMs) })}
          className="w-24"
        />
      </div>
      <div className="flex gap-2">
        <button
          type="button"
          onClick={() =>
            studioTitleFire(sourceId, "in").catch((err) => console.error("title fire failed:", err))
          }
          className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
        >
          {t("properties-title-fire-in")}
        </button>
        <button
          type="button"
          onClick={() =>
            studioTitleFire(sourceId, "out").catch((err) =>
              console.error("title fire failed:", err),
            )
          }
          className="rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-text hover:border-havoc-accent/50"
        >
          {t("properties-title-fire-out")}
        </button>
      </div>
      <span className="text-[11px] text-havoc-muted">{t("properties-title-layers")}</span>
      <div className="flex max-h-72 flex-col gap-2 overflow-y-auto pr-1">
        {draft.layers.map((layer, index) => (
          <div key={index} className="flex flex-col gap-1 rounded-md border border-white/10 p-2">
            <div className="flex items-center gap-1">
              <span className="rounded bg-havoc-accent/15 px-1.5 py-0.5 text-[10px] font-semibold uppercase text-havoc-muted">
                {kindLabel(layer)}
              </span>
              {layer.kind === "text" ? (
                <>
                  <input
                    value={layer.text}
                    onChange={(event) => updateLayer(index, { ...layer, text: event.target.value })}
                    className={`${inputClass} flex-1`}
                  />
                  <button
                    type="button"
                    onClick={() =>
                      studioTitleSetText(sourceId, index, layer.text).catch((err) =>
                        console.error("title set-text failed:", err),
                      )
                    }
                    title={t("properties-title-set-live-note")}
                    className="rounded border border-havoc-accent/60 bg-havoc-accent/15 px-1.5 py-1 text-[11px] text-havoc-text hover:bg-havoc-accent/25"
                  >
                    {t("properties-title-set-live")}
                  </button>
                </>
              ) : layer.kind === "image" ? (
                <input
                  value={layer.path}
                  onChange={(event) => updateLayer(index, { ...layer, path: event.target.value })}
                  placeholder="C:\overlays\badge.png"
                  className={`${inputClass} flex-1 font-mono`}
                />
              ) : (
                <span className="flex-1" />
              )}
              <RowControls
                index={index}
                count={draft.layers.length}
                upLabel={t("properties-title-up")}
                downLabel={t("properties-title-down")}
                removeLabel={t("properties-title-remove")}
                onSwap={(at) => onChange({ ...draft, layers: swapAt(draft.layers, at) })}
                onRemove={() =>
                  onChange({ ...draft, layers: draft.layers.filter((_, at) => at !== index) })
                }
              />
            </div>
            <div className="flex items-end gap-2">
              <NumberField
                label={t("properties-title-x")}
                value={layer.x}
                min={-16384}
                max={16384}
                onCommit={(x) => updateLayer(index, { ...layer, x: Math.round(x) })}
                className="w-20"
              />
              <NumberField
                label={t("properties-title-y")}
                value={layer.y}
                min={-16384}
                max={16384}
                onCommit={(y) => updateLayer(index, { ...layer, y: Math.round(y) })}
                className="w-20"
              />
              {layer.kind === "rect" && (
                <>
                  <NumberField
                    label={t("properties-width")}
                    value={layer.width}
                    min={1}
                    max={16384}
                    onCommit={(width) => updateLayer(index, { ...layer, width })}
                    className="w-20"
                  />
                  <NumberField
                    label={t("properties-height")}
                    value={layer.height}
                    min={1}
                    max={16384}
                    onCommit={(height) => updateLayer(index, { ...layer, height })}
                    className="w-20"
                  />
                </>
              )}
              {layer.kind === "text" && (
                <>
                  <NumberField
                    label={t("properties-size-px")}
                    value={layer.sizePx}
                    min={4}
                    max={512}
                    onCommit={(sizePx) => updateLayer(index, { ...layer, sizePx })}
                    className="w-20"
                  />
                  <NumberField
                    label={t("properties-title-outline")}
                    value={layer.outlinePx}
                    min={0}
                    max={32}
                    onCommit={(outlinePx) => updateLayer(index, { ...layer, outlinePx })}
                    className="w-20"
                  />
                  <label className="flex items-center gap-1 pb-1 text-[11px] text-havoc-muted">
                    {t("properties-title-outline-color")}
                    <input
                      type="color"
                      value={rgbaToHex(layer.outlineColor)}
                      onChange={(event) =>
                        updateLayer(index, {
                          ...layer,
                          outlineColor: hexToRgba(event.target.value, layer.outlineColor.a),
                        })
                      }
                      aria-label={t("properties-title-outline-color")}
                      className="h-7 w-8 cursor-pointer rounded border border-white/10 bg-transparent"
                    />
                  </label>
                  <label className="flex items-center gap-1 pb-1.5 text-[11px] text-havoc-muted">
                    <input
                      type="checkbox"
                      checked={layer.shadow}
                      onChange={(event) =>
                        updateLayer(index, { ...layer, shadow: event.target.checked })
                      }
                    />
                    {t("properties-title-shadow")}
                  </label>
                </>
              )}
              {(layer.kind === "text" || layer.kind === "rect") && (
                <label className="flex items-center gap-1 pb-1 text-[11px] text-havoc-muted">
                  {t("properties-color")}
                  <input
                    type="color"
                    value={rgbaToHex(layer.color)}
                    onChange={(event) =>
                      updateLayer(index, {
                        ...layer,
                        color: hexToRgba(event.target.value, layer.color.a),
                      })
                    }
                    aria-label={t("properties-color")}
                    className="h-7 w-8 cursor-pointer rounded border border-white/10 bg-transparent"
                  />
                </label>
              )}
            </div>
            {layer.kind === "text" && (
              <div className="flex items-end gap-2">
                <label className="flex flex-1 flex-col gap-1 text-[11px] text-havoc-muted">
                  {t("properties-text-file")}
                  <input
                    value={layer.sourceFile}
                    onChange={(event) =>
                      updateLayer(index, { ...layer, sourceFile: event.target.value })
                    }
                    placeholder="C:\data\score.csv"
                    className={`${inputClass} font-mono`}
                  />
                </label>
                {layer.sourceFile.trim() !== "" && (
                  <>
                    <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
                      {t("properties-text-binding")}
                      <select
                        value={layer.binding}
                        onChange={(event) =>
                          updateLayer(index, {
                            ...layer,
                            binding: event.target.value as FileBinding,
                          })
                        }
                        className={inputClass}
                      >
                        <option value="whole">{t("properties-text-binding-whole")}</option>
                        <option value="csvCell">{t("properties-text-binding-csv")}</option>
                        <option value="jsonPointer">{t("properties-text-binding-json")}</option>
                      </select>
                    </label>
                    {layer.binding === "csvCell" && (
                      <>
                        <NumberField
                          label={t("properties-text-csv-row")}
                          value={layer.csvRow}
                          min={1}
                          max={100000}
                          onCommit={(csvRow) =>
                            updateLayer(index, { ...layer, csvRow: Math.round(csvRow) })
                          }
                          className="w-20"
                        />
                        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
                          {t("properties-text-csv-column")}
                          <input
                            value={layer.csvColumn}
                            onChange={(event) =>
                              updateLayer(index, { ...layer, csvColumn: event.target.value })
                            }
                            placeholder={t("properties-text-csv-column-placeholder")}
                            className={`${inputClass} w-24 font-mono`}
                          />
                        </label>
                      </>
                    )}
                    {layer.binding === "jsonPointer" && (
                      <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
                        {t("properties-text-json-pointer")}
                        <input
                          value={layer.jsonPointer}
                          onChange={(event) =>
                            updateLayer(index, { ...layer, jsonPointer: event.target.value })
                          }
                          placeholder="/teams/0/score"
                          className={`${inputClass} w-36 font-mono`}
                        />
                      </label>
                    )}
                  </>
                )}
              </div>
            )}
          </div>
        ))}
      </div>
      <div className="flex gap-2">
        <button
          type="button"
          onClick={() => addLayer(titleTextLayer({ text: t("sources-text-default") }))}
          className="rounded-md border border-white/10 px-2 py-1 text-[11px] text-havoc-muted hover:text-havoc-text"
        >
          {t("properties-title-add-text")}
        </button>
        <button
          type="button"
          onClick={() => addLayer({ kind: "image", x: 0, y: 0, path: "" })}
          className="rounded-md border border-white/10 px-2 py-1 text-[11px] text-havoc-muted hover:text-havoc-text"
        >
          {t("properties-title-add-image")}
        </button>
        <button
          type="button"
          onClick={() =>
            addLayer({
              kind: "rect",
              x: 0,
              y: 0,
              width: 400,
              height: 120,
              color: { r: 74, g: 158, b: 255, a: 230 },
            })
          }
          className="rounded-md border border-white/10 px-2 py-1 text-[11px] text-havoc-muted hover:text-havoc-text"
        >
          {t("properties-title-add-rect")}
        </button>
      </div>
      <p className="m-0 text-[10px] leading-snug text-havoc-muted">{t("properties-title-note")}</p>
    </div>
  );
}

function TextEditor({
  draft,
  onChange,
}: {
  draft: Extract<SourceSettings, { kind: "text" }>;
  onChange: (settings: SourceSettings) => void;
}) {
  const t = useT();
  return (
    <div className="flex flex-col gap-2">
      <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
        {t("properties-text")}
        <textarea
          value={draft.text}
          onChange={(event) => onChange({ ...draft, text: event.target.value })}
          rows={3}
          className={inputClass}
        />
      </label>
      <div className="flex gap-2">
        <label className="flex flex-1 flex-col gap-1 text-[11px] text-havoc-muted">
          {t("properties-font-family")}
          <input
            value={draft.fontFamily ?? ""}
            onChange={(event) =>
              onChange({ ...draft, fontFamily: event.target.value.trim() || null })
            }
            placeholder="Segoe UI"
            className={inputClass}
          />
        </label>
        <NumberField
          label={t("properties-size-px")}
          value={draft.sizePx}
          min={4}
          max={512}
          onCommit={(sizePx) => onChange({ ...draft, sizePx })}
          className="w-24"
        />
      </div>
      <div className="flex items-end gap-3">
        <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
          {t("properties-color")}
          <input
            type="color"
            value={rgbaToHex(draft.color)}
            onChange={(event) =>
              onChange({ ...draft, color: hexToRgba(event.target.value, draft.color.a) })
            }
            aria-label={t("properties-text-color")}
            className="h-7 w-12 cursor-pointer rounded border border-white/10 bg-transparent"
          />
        </label>
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("properties-align")}
          <select
            value={draft.align}
            onChange={(event) => onChange({ ...draft, align: event.target.value as TextAlign })}
            className={inputClass}
          >
            <option value="left">{t("properties-align-left")}</option>
            <option value="center">{t("properties-align-center")}</option>
            <option value="right">{t("properties-align-right")}</option>
          </select>
        </label>
        <NumberField
          label={t("properties-line-spacing")}
          value={draft.lineSpacing}
          min={0.25}
          max={4}
          step={0.05}
          onCommit={(lineSpacing) => onChange({ ...draft, lineSpacing })}
          className="w-24"
        />
      </div>
      <div className="flex items-center gap-4">
        <label className="flex w-40 flex-col gap-1 text-[11px] text-havoc-muted">
          {t("properties-wrap-width")}
          <input
            type="number"
            min={0}
            value={draft.wrapWidth ?? 0}
            onChange={(event) => {
              const value = Number(event.target.value) || 0;
              onChange({ ...draft, wrapWidth: value > 0 ? value : null });
            }}
            className={inputClass}
          />
        </label>
        <label className="flex items-center gap-1.5 text-[11px] text-havoc-muted">
          <input
            type="checkbox"
            checked={draft.forceRtl}
            onChange={(event) => onChange({ ...draft, forceRtl: event.target.checked })}
          />
          {t("properties-force-rtl")}
        </label>
      </div>
      <p className="m-0 text-[10px] leading-snug text-havoc-muted">{t("properties-text-note")}</p>
      <div className="flex flex-col gap-2 border-t border-white/5 pt-2">
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("properties-text-file")}
          <input
            value={draft.sourceFile}
            onChange={(event) => onChange({ ...draft, sourceFile: event.target.value })}
            placeholder="C:\\data\\score.csv"
            className={`${inputClass} font-mono`}
          />
        </label>
        {draft.sourceFile.trim() !== "" && (
          <>
            <div className="flex items-end gap-2">
              <label className="flex flex-1 flex-col gap-1 text-[11px] text-havoc-muted">
                {t("properties-text-binding")}
                <select
                  value={draft.binding}
                  onChange={(event) =>
                    onChange({ ...draft, binding: event.target.value as FileBinding })
                  }
                  className={inputClass}
                >
                  <option value="whole">{t("properties-text-binding-whole")}</option>
                  <option value="csvCell">{t("properties-text-binding-csv")}</option>
                  <option value="jsonPointer">{t("properties-text-binding-json")}</option>
                </select>
              </label>
              {draft.binding === "csvCell" && (
                <>
                  <NumberField
                    label={t("properties-text-csv-row")}
                    value={draft.csvRow}
                    min={1}
                    max={100000}
                    onCommit={(csvRow) => onChange({ ...draft, csvRow: Math.round(csvRow) })}
                    className="w-24"
                  />
                  <label className="flex flex-1 flex-col gap-1 text-[11px] text-havoc-muted">
                    {t("properties-text-csv-column")}
                    <input
                      value={draft.csvColumn}
                      onChange={(event) => onChange({ ...draft, csvColumn: event.target.value })}
                      placeholder={t("properties-text-csv-column-placeholder")}
                      className={`${inputClass} font-mono`}
                    />
                  </label>
                </>
              )}
              {draft.binding === "jsonPointer" && (
                <label className="flex flex-1 flex-col gap-1 text-[11px] text-havoc-muted">
                  {t("properties-text-json-pointer")}
                  <input
                    value={draft.jsonPointer}
                    onChange={(event) => onChange({ ...draft, jsonPointer: event.target.value })}
                    placeholder="/teams/0/score"
                    className={`${inputClass} font-mono`}
                  />
                </label>
              )}
            </div>
            <p className="m-0 text-[10px] leading-snug text-havoc-muted">
              {t("properties-text-file-note")}
            </p>
          </>
        )}
      </div>
    </div>
  );
}

/**
 * CAP-N11: the LAN ingest listener's settings + the live connect URL/QR.
 * Applying a change restarts the listener (the sender must reconnect) —
 * the note says so.
 */
function LanIngestEditor({
  draft,
  onChange,
}: {
  draft: Extract<SourceSettings, { kind: "lanIngest" }>;
  onChange: (settings: SourceSettings) => void;
}) {
  const t = useT();
  return (
    <div className="flex flex-col gap-2">
      <LanIngestFields value={draft} onChange={(next) => onChange({ ...draft, ...next })} />
      <p className="m-0 text-[10px] leading-snug text-havoc-muted">{t("properties-lan-note")}</p>
    </div>
  );
}
