import { useState } from "react";

import {
  studioSetAudioMono,
  studioSetAudioMonitor,
  studioSetAudioMuted,
  studioSetAudioPan,
  studioSetAudioSolo,
  studioSetAudioSyncOffset,
  studioSetAudioTracks,
  studioSetAudioVolume,
  studioSetSceneAudioOverride,
} from "../api/commands";
import type {
  AudioSettings,
  AudioSourceLevels,
  MonitorMode,
  SceneAudioOverride,
  SceneId,
  Source,
} from "../api/types";
import { MAX_SYNC_OFFSET_MS, MAX_VOLUME_DB, MIN_VOLUME_DB, TRACK_COUNT } from "../api/types";
import { useT } from "../i18n/t";
import type { MeterColors } from "../lib/meters";
import { DEFAULT_METER_COLORS, meterGradient } from "../lib/meters";
import { NumberField } from "./NumberField";

const fail = (what: string) => (err: unknown) => console.error(`${what} failed:`, err);

/** Linear amplitude → dB, floored to the meter's bottom. */
function linToDb(lin: number): number {
  return 20 * Math.log10(Math.max(lin, 1e-6));
}

/** dB → meter percentage across MIN_VOLUME_DB..0. */
function dbToPercent(db: number): number {
  const clamped = Math.min(0, Math.max(MIN_VOLUME_DB, db));
  return ((clamped - MIN_VOLUME_DB) / -MIN_VOLUME_DB) * 100;
}

/** The dB ticks drawn beside a vertical meter (OBS-style scale). */
const DB_TICKS = [0, -6, -12, -24, -36, -48, -60];

/**
 * The VU meter: a low→mid→high gradient (green→yellow→red by default; see
 * `resolveMeterColors`) revealed to the RMS level with a peak tick. Horizontal
 * (bar) or vertical (column) per `orientation`. Levels arrive ~20 Hz; a short
 * CSS transition smooths the motion between events.
 */
function VuMeter({
  levels,
  dimmed,
  orientation,
  colors,
}: {
  levels?: AudioSourceLevels;
  dimmed: boolean;
  orientation: "horizontal" | "vertical";
  colors: MeterColors;
}) {
  const t = useT();
  const rms = levels ? Math.max(levels.rms[0], levels.rms[1]) : 0;
  const peak = levels ? Math.max(levels.peak[0], levels.peak[1]) : 0;
  const rmsPercent = dbToPercent(linToDb(rms));
  const peakPercent = dbToPercent(linToDb(peak));
  const vertical = orientation === "vertical";
  // The inner gradient always spans the FULL meter, so a colour maps to its dB
  // position (one green→yellow→red sweep). A clipping window sized to the level
  // reveals just the reached part — no tiling/repeat as the meter grows.
  const innerExtent = rmsPercent > 0 ? (10000 / rmsPercent).toFixed(2) : "0";
  return (
    <div
      role="meter"
      aria-label={t("channelstrip-level")}
      aria-valuemin={MIN_VOLUME_DB}
      aria-valuemax={0}
      aria-valuenow={Math.round(linToDb(peak))}
      className={`relative overflow-hidden rounded-sm bg-black/50 ${
        vertical ? "h-full w-2.5" : "h-2 w-full"
      } ${dimmed ? "opacity-40" : ""}`}
    >
      <div
        className={`absolute overflow-hidden ${vertical ? "bottom-0 left-0 w-full" : "inset-y-0 left-0"}`}
        style={{
          [vertical ? "height" : "width"]: `${rmsPercent}%`,
          transition: `${vertical ? "height" : "width"} 75ms linear`,
        }}
      >
        <div
          className={`absolute ${vertical ? "bottom-0 left-0 w-full" : "inset-y-0 left-0"}`}
          style={{
            [vertical ? "height" : "width"]: `${innerExtent}%`,
            background: meterGradient(vertical ? "to top" : "to right", colors),
          }}
        />
      </div>
      {peakPercent > 0 && (
        <div
          className={`absolute bg-white/80 ${vertical ? "left-0 h-px w-full" : "top-0 h-full w-px"}`}
          style={
            vertical
              ? { bottom: `${peakPercent}%`, transition: "bottom 75ms linear" }
              : { left: `${peakPercent}%`, transition: "left 75ms linear" }
          }
        />
      )}
    </div>
  );
}

/** `mode -> i18n key`. Resolved at render — see FILTER_NAME_KEYS. */
const MONITOR_LABEL_KEYS: Record<MonitorMode, string> = {
  off: "channelstrip-monitor-off",
  monitorOnly: "channelstrip-monitor-only",
  monitorAndOutput: "channelstrip-monitor-and-output",
};

const MONITOR_NEXT: Record<MonitorMode, MonitorMode> = {
  off: "monitorOnly",
  monitorOnly: "monitorAndOutput",
  monitorAndOutput: "off",
};

// -- shared control atoms (identical behaviour in both orientations) ---------

function StatusDot({ levels }: { levels?: AudioSourceLevels }) {
  const t = useT();
  const hasError = levels?.state === "error";
  return (
    <span
      title={
        hasError
          ? (levels?.errorMessage ?? t("channelstrip-status-error"))
          : levels?.state === "live"
            ? t("channelstrip-status-live")
            : t("channelstrip-status-waiting-audio")
      }
      aria-label={t("channelstrip-status", {
        state: levels?.state ?? t("channelstrip-status-waiting"),
      })}
      className={`h-1.5 w-1.5 shrink-0 rounded-full ${
        hasError ? "bg-red-400" : levels?.state === "live" ? "bg-emerald-400" : "bg-amber-300"
      }`}
    />
  );
}

function MuteButton({
  source,
  muted,
  onToggle,
}: {
  source: Source;
  muted: boolean;
  onToggle: () => void;
}) {
  const t = useT();
  return (
    <button
      type="button"
      onClick={onToggle}
      title={muted ? t("channelstrip-unmute") : t("channelstrip-mute")}
      aria-label={
        muted
          ? t("channelstrip-unmute-source", { name: source.name })
          : t("channelstrip-mute-source", { name: source.name })
      }
      aria-pressed={muted}
      className={`shrink-0 rounded border px-1.5 text-[10px] font-bold ${
        muted
          ? "border-red-400/60 bg-red-500/20 text-red-300"
          : "border-white/10 text-havoc-muted hover:text-havoc-text"
      }`}
    >
      M
    </button>
  );
}

/** The per-scene mix toggle (TASK-605): while on, this strip's fader/mute
 * override the global mix for the CURRENT scene only. */
function SceneMixButton({
  source,
  active,
  onToggle,
}: {
  source: Source;
  active: boolean;
  onToggle: () => void;
}) {
  const t = useT();
  return (
    <button
      type="button"
      onClick={onToggle}
      title={active ? t("channelstrip-scene-mix-on") : t("channelstrip-scene-mix-off")}
      aria-label={t("channelstrip-scene-mix-label", { name: source.name })}
      aria-pressed={active}
      className={`shrink-0 rounded border px-1.5 text-[10px] font-bold ${
        active
          ? "border-havoc-accent/70 bg-havoc-accent/20 text-havoc-text"
          : "border-white/10 text-havoc-muted hover:text-havoc-text"
      }`}
    >
      S
    </button>
  );
}

/** PFL solo (CAP-M19): monitor hears only soloed strips; program unchanged. */
function SoloButton({ source, audio }: { source: Source; audio: AudioSettings }) {
  const t = useT();
  return (
    <button
      type="button"
      onClick={() => studioSetAudioSolo(source.id, !audio.solo).catch(fail("solo"))}
      title={t("channelstrip-solo-title")}
      aria-label={t("channelstrip-solo-source", { name: source.name })}
      aria-pressed={audio.solo}
      className={`shrink-0 rounded border px-1.5 text-[10px] font-bold ${
        audio.solo
          ? "border-amber-400/60 bg-amber-500/20 text-amber-300"
          : "border-white/10 text-havoc-muted hover:text-havoc-text"
      }`}
    >
      PFL
    </button>
  );
}

function MonitorButton({ source, audio }: { source: Source; audio: AudioSettings }) {
  const t = useT();
  return (
    <button
      type="button"
      onClick={() =>
        studioSetAudioMonitor(source.id, MONITOR_NEXT[audio.monitor]).catch(fail("monitor"))
      }
      title={t("channelstrip-monitor-cycle", { mode: t(MONITOR_LABEL_KEYS[audio.monitor]) })}
      aria-label={t("channelstrip-monitor-mode", {
        name: source.name,
        mode: t(MONITOR_LABEL_KEYS[audio.monitor]),
      })}
      className={`shrink-0 rounded border px-1.5 text-[10px] ${
        audio.monitor === "off"
          ? "border-white/10 text-havoc-muted hover:text-havoc-text"
          : audio.monitor === "monitorOnly"
            ? "border-havoc-accent/60 bg-havoc-accent/15 text-havoc-accent"
            : "border-havoc-accent-2/60 bg-havoc-accent-2/15 text-havoc-accent-2"
      }`}
    >
      🎧
    </button>
  );
}

function FiltersButton({
  source,
  audio,
  onOpenFilters,
}: {
  source: Source;
  audio: AudioSettings;
  onOpenFilters: () => void;
}) {
  const t = useT();
  return (
    <button
      type="button"
      onClick={onOpenFilters}
      title={t("channelstrip-audio-filters-title")}
      aria-label={t("channelstrip-audio-filters-label", { name: source.name })}
      className="shrink-0 rounded border border-white/10 px-1.5 text-[10px] text-havoc-muted hover:text-havoc-text"
    >
      ƒ{audio.filters.length > 0 ? audio.filters.length : ""}
    </button>
  );
}

function AdvancedButton({
  source,
  onOpenAdvanced,
}: {
  source: Source;
  onOpenAdvanced: () => void;
}) {
  const t = useT();
  return (
    <button
      type="button"
      onClick={onOpenAdvanced}
      title={t("channelstrip-advanced-title")}
      aria-label={t("channelstrip-advanced-label", { name: source.name })}
      className="shrink-0 rounded border border-white/10 px-1.5 text-[10px] text-havoc-muted hover:text-havoc-text"
    >
      ⋯
    </button>
  );
}

function TrackDots({ source, audio }: { source: Source; audio: AudioSettings }) {
  const t = useT();
  return (
    <div
      className="flex shrink-0 items-center gap-0.5"
      aria-label={t("channelstrip-track-assignment")}
    >
      {Array.from({ length: TRACK_COUNT }, (_, index) => {
        const assigned = (audio.tracks & (1 << index)) !== 0;
        return (
          <button
            key={index}
            type="button"
            onClick={() =>
              studioSetAudioTracks(source.id, audio.tracks ^ (1 << index)).catch(
                fail("track assignment"),
              )
            }
            title={
              assigned
                ? t("channelstrip-track-assigned", { n: index + 1 })
                : t("channelstrip-track", { n: index + 1 })
            }
            aria-label={t("channelstrip-track-label", { n: index + 1, name: source.name })}
            aria-pressed={assigned}
            className={`h-3.5 w-3.5 rounded-full border text-[8px] leading-none ${
              assigned
                ? "border-havoc-accent bg-havoc-accent/30 text-havoc-text"
                : "border-white/15 text-havoc-muted hover:border-white/40"
            }`}
          >
            {index + 1}
          </button>
        );
      })}
    </div>
  );
}

function dbLabel(volumeDb: number): string {
  return volumeDb <= MIN_VOLUME_DB ? "-inf" : volumeDb.toFixed(1);
}

type ChannelStripProps = {
  source: Source;
  audio: AudioSettings;
  levels?: AudioSourceLevels;
  orientation?: "horizontal" | "vertical";
  /** The program scene, for the per-scene mix toggle (TASK-605). */
  sceneId?: SceneId | null;
  /** This source's override in that scene, when one exists. */
  sceneOverride?: SceneAudioOverride | null;
  /** The meter palette (Settings → Accessibility); default sweep when absent. */
  meterColors?: MeterColors;
  onOpenFilters: () => void;
  onOpenAdvanced: () => void;
};

/**
 * One mixer strip: name + status, VU meter, fader (dB), mute, monitor cycle,
 * filters, track dots 1–6, and the advanced popover (sync offset, PTT/PTM).
 * Horizontal = a compact row; vertical = an OBS-style column with a tall
 * meter + fader.
 */
export function ChannelStrip({
  source,
  audio,
  levels,
  orientation = "horizontal",
  sceneId = null,
  sceneOverride = null,
  meterColors = DEFAULT_METER_COLORS,
  onOpenFilters,
  onOpenAdvanced,
}: ChannelStripProps) {
  const t = useT();
  // Fader drags stream; keep a local draft for instant feedback.
  const [draftDb, setDraftDb] = useState<number | null>(null);
  // Per-scene mix (TASK-605): when this scene overrides the strip, the
  // fader/mute SHOW and EDIT the override; everything else stays global.
  const overriding = sceneOverride !== null && sceneId !== null;
  const shownDb = overriding ? sceneOverride.volumeDb : audio.volumeDb;
  const shownMuted = overriding ? sceneOverride.muted : audio.muted;
  const volumeDb = draftDb ?? shownDb;
  const gated = levels?.gated ?? (shownMuted || Boolean(audio.pushToTalk));
  const hasError = levels?.state === "error";

  const setVolume = (value: number) => {
    if (overriding && sceneId) {
      studioSetSceneAudioOverride(sceneId, source.id, {
        volumeDb: value,
        muted: sceneOverride.muted,
      }).catch(fail("scene volume"));
    } else {
      studioSetAudioVolume(source.id, value).catch(fail("volume"));
    }
  };
  const toggleMute = () => {
    if (overriding && sceneId) {
      studioSetSceneAudioOverride(sceneId, source.id, {
        volumeDb: sceneOverride.volumeDb,
        muted: !sceneOverride.muted,
      }).catch(fail("scene mute"));
    } else {
      studioSetAudioMuted(source.id, !audio.muted).catch(fail("mute"));
    }
  };
  const toggleSceneMix = () => {
    if (!sceneId) return;
    studioSetSceneAudioOverride(
      sceneId,
      source.id,
      overriding ? null : { volumeDb: audio.volumeDb, muted: audio.muted },
    ).catch(fail("per-scene mix"));
  };
  const onFaderChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const value = Number(event.target.value);
    setDraftDb(value);
    setVolume(value);
  };
  const commitVolume = () => {
    if (draftDb === null) return;
    const value = draftDb;
    setDraftDb(null);
    setVolume(value);
  };

  if (orientation === "vertical") {
    return (
      <div className="flex h-full w-[74px] shrink-0 flex-col items-center gap-1 rounded-lg border border-white/10 bg-white/[0.02] px-1.5 py-1.5">
        <div className="flex w-full items-center justify-center gap-1">
          <StatusDot levels={levels} />
          <span className="min-w-0 truncate text-[10px] text-havoc-text" title={source.name}>
            {source.name}
          </span>
        </div>
        <span className="tabular-nums text-[9px] text-havoc-muted">{dbLabel(volumeDb)} dB</span>
        {hasError ? (
          <p
            className="m-0 flex-1 px-0.5 text-center text-[9px] text-red-400"
            title={levels?.errorMessage}
          >
            {levels?.errorMessage ?? t("channelstrip-device-error")}
          </p>
        ) : (
          <div className="flex min-h-0 flex-1 items-stretch gap-1.5 py-0.5">
            {/* dB tick scale */}
            <div className="flex flex-col justify-between py-0.5 text-[7px] leading-none text-havoc-muted tabular-nums">
              {DB_TICKS.map((tick) => (
                <span key={tick}>{tick}</span>
              ))}
            </div>
            <VuMeter levels={levels} dimmed={gated} orientation="vertical" colors={meterColors} />
            <input
              type="range"
              min={MIN_VOLUME_DB}
              max={MAX_VOLUME_DB}
              step={0.5}
              value={volumeDb}
              onChange={onFaderChange}
              onMouseUp={commitVolume}
              onKeyUp={commitVolume}
              aria-label={t("channelstrip-volume-label", { name: source.name })}
              className="accent-havoc-accent"
              style={{ writingMode: "vertical-lr", direction: "rtl", width: "1rem" }}
            />
          </div>
        )}
        {audio.pushToTalk && (
          <span
            title={t("channelstrip-ptt-hold", { key: audio.pushToTalk })}
            className={`rounded px-1 text-[8px] uppercase ${
              gated ? "bg-white/10 text-havoc-muted" : "bg-emerald-500/20 text-emerald-300"
            }`}
          >
            PTT
          </span>
        )}
        <div className="flex w-full items-center justify-center gap-0.5">
          <MuteButton source={source} muted={shownMuted} onToggle={toggleMute} />
          {sceneId && (
            <SceneMixButton source={source} active={overriding} onToggle={toggleSceneMix} />
          )}
          <SoloButton source={source} audio={audio} />
          <MonitorButton source={source} audio={audio} />
          <FiltersButton source={source} audio={audio} onOpenFilters={onOpenFilters} />
          <AdvancedButton source={source} onOpenAdvanced={onOpenAdvanced} />
        </div>
        <TrackDots source={source} audio={audio} />
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-1 rounded-lg border border-white/10 bg-white/[0.02] px-2 py-1.5">
      <div className="flex items-center gap-1.5">
        <StatusDot levels={levels} />
        <span className="min-w-0 flex-1 truncate text-xs text-havoc-text" title={source.name}>
          {source.name}
        </span>
        {audio.pushToTalk && (
          <span
            title={t("channelstrip-ptt-hold", { key: audio.pushToTalk })}
            className={`rounded px-1 text-[9px] uppercase ${
              gated ? "bg-white/10 text-havoc-muted" : "bg-emerald-500/20 text-emerald-300"
            }`}
          >
            PTT
          </span>
        )}
        <span className="w-12 shrink-0 text-right text-[10px] tabular-nums text-havoc-muted">
          {dbLabel(volumeDb)} dB
        </span>
        <MuteButton source={source} muted={shownMuted} onToggle={toggleMute} />
        {sceneId && (
          <SceneMixButton source={source} active={overriding} onToggle={toggleSceneMix} />
        )}
        <SoloButton source={source} audio={audio} />
        <MonitorButton source={source} audio={audio} />
        <FiltersButton source={source} audio={audio} onOpenFilters={onOpenFilters} />
        <AdvancedButton source={source} onOpenAdvanced={onOpenAdvanced} />
      </div>

      {hasError ? (
        <p className="m-0 truncate text-[10px] text-red-400" title={levels?.errorMessage}>
          {levels?.errorMessage ?? t("channelstrip-audio-device-error")}
        </p>
      ) : (
        <VuMeter levels={levels} dimmed={gated} orientation="horizontal" colors={meterColors} />
      )}

      <div className="flex items-center gap-2">
        <input
          type="range"
          min={MIN_VOLUME_DB}
          max={MAX_VOLUME_DB}
          step={0.5}
          value={volumeDb}
          onChange={onFaderChange}
          onMouseUp={commitVolume}
          onKeyUp={commitVolume}
          aria-label={t("channelstrip-volume-label", { name: source.name })}
          className="min-w-0 flex-1 accent-havoc-accent"
        />
        <TrackDots source={source} audio={audio} />
      </div>
    </div>
  );
}

/** The "⋯" popover body: sync offset + PTT/PTM hotkeys. */
export function AdvancedAudioFields({
  source,
  audio,
  onSetHotkeys,
}: {
  source: Source;
  audio: AudioSettings;
  onSetHotkeys: (pushToTalk: string | null, pushToMute: string | null) => void;
}) {
  const t = useT();
  const [ptt, setPtt] = useState(audio.pushToTalk ?? "");
  const [ptm, setPtm] = useState(audio.pushToMute ?? "");
  const hotkeyClass =
    "rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text outline-none focus:border-havoc-accent/60";

  return (
    <div className="flex flex-col gap-2">
      <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
        {t("channelstrip-pan-label")}
        <input
          type="range"
          min={-100}
          max={100}
          step={1}
          value={Math.round(audio.pan * 100)}
          onChange={(event) =>
            studioSetAudioPan(source.id, Number(event.target.value) / 100).catch(fail("pan"))
          }
          onDoubleClick={() => studioSetAudioPan(source.id, 0).catch(fail("pan"))}
          aria-label={t("channelstrip-pan-aria", { name: source.name })}
          className="min-w-0 flex-1 accent-havoc-accent"
        />
      </label>
      <label className="flex items-center gap-1.5 text-[11px] text-havoc-muted">
        <input
          type="checkbox"
          checked={audio.mono}
          onChange={(event) =>
            studioSetAudioMono(source.id, event.target.checked).catch(fail("mono"))
          }
        />
        {t("channelstrip-mono-label")}
      </label>
      <NumberField
        label={t("channelstrip-sync-offset", { max: MAX_SYNC_OFFSET_MS })}
        value={audio.syncOffsetMs}
        min={0}
        max={MAX_SYNC_OFFSET_MS}
        step={5}
        onCommit={(value) =>
          studioSetAudioSyncOffset(source.id, Math.round(value)).catch(fail("sync offset"))
        }
      />
      <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
        {t("channelstrip-ptt-hotkey")}
        <input
          value={ptt}
          onChange={(event) => setPtt(event.target.value)}
          placeholder={t("channelstrip-ptt-placeholder")}
          aria-label={t("channelstrip-ptt-aria")}
          className={hotkeyClass}
        />
      </label>
      <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
        {t("channelstrip-ptm-hotkey")}
        <input
          value={ptm}
          onChange={(event) => setPtm(event.target.value)}
          placeholder={t("channelstrip-ptm-placeholder")}
          aria-label={t("channelstrip-ptm-aria")}
          className={hotkeyClass}
        />
      </label>
      <p className="m-0 text-[10px] leading-snug text-havoc-muted">
        {t("channelstrip-hotkeys-note")}
      </p>
      <button
        type="button"
        onClick={() => onSetHotkeys(ptt.trim() || null, ptm.trim() || null)}
        className="self-end rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
      >
        {t("channelstrip-apply")}
      </button>
    </div>
  );
}
