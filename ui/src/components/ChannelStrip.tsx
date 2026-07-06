import { useState } from "react";

import {
  studioSetAudioMonitor,
  studioSetAudioMuted,
  studioSetAudioSyncOffset,
  studioSetAudioTracks,
  studioSetAudioVolume,
} from "../api/commands";
import type { AudioSettings, AudioSourceLevels, MonitorMode, Source } from "../api/types";
import { MAX_SYNC_OFFSET_MS, MAX_VOLUME_DB, MIN_VOLUME_DB, TRACK_COUNT } from "../api/types";
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

/** The green→yellow→red gradient a level bar reveals, shared by both meters. */
const METER_GRADIENT_H =
  "linear-gradient(to right, #22c55e 0%, #22c55e 62%, #eab308 78%, #ef4444 95%)";
const METER_GRADIENT_V =
  "linear-gradient(to top, #22c55e 0%, #22c55e 62%, #eab308 78%, #ef4444 95%)";

/** The dB ticks drawn beside a vertical meter (OBS-style scale). */
const DB_TICKS = [0, -6, -12, -24, -36, -48, -60];

/**
 * The VU meter: a green→yellow→red gradient revealed to the RMS level with a
 * peak tick. Horizontal (bar) or vertical (column) per `orientation`. Levels
 * arrive ~20 Hz; a short CSS transition smooths the motion between events.
 */
function VuMeter({
  levels,
  dimmed,
  orientation,
}: {
  levels?: AudioSourceLevels;
  dimmed: boolean;
  orientation: "horizontal" | "vertical";
}) {
  const rms = levels ? Math.max(levels.rms[0], levels.rms[1]) : 0;
  const peak = levels ? Math.max(levels.peak[0], levels.peak[1]) : 0;
  const rmsPercent = dbToPercent(linToDb(rms));
  const peakPercent = dbToPercent(linToDb(peak));
  const vertical = orientation === "vertical";
  return (
    <div
      role="meter"
      aria-label="Level"
      aria-valuemin={MIN_VOLUME_DB}
      aria-valuemax={0}
      aria-valuenow={Math.round(linToDb(peak))}
      className={`relative overflow-hidden rounded-sm bg-black/50 ${
        vertical ? "h-full w-2.5" : "h-2 w-full"
      } ${dimmed ? "opacity-40" : ""}`}
    >
      <div
        className={vertical ? "absolute bottom-0 w-full" : "h-full"}
        style={{
          [vertical ? "height" : "width"]: `${rmsPercent}%`,
          background: vertical ? METER_GRADIENT_V : METER_GRADIENT_H,
          backgroundSize: vertical ? "100% 12rem" : "10rem 100%",
          transition: `${vertical ? "height" : "width"} 75ms linear`,
        }}
      />
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

const MONITOR_LABEL: Record<MonitorMode, string> = {
  off: "Monitor off",
  monitorOnly: "Monitor only (not in the mix)",
  monitorAndOutput: "Monitor and output",
};

const MONITOR_NEXT: Record<MonitorMode, MonitorMode> = {
  off: "monitorOnly",
  monitorOnly: "monitorAndOutput",
  monitorAndOutput: "off",
};

// -- shared control atoms (identical behaviour in both orientations) ---------

function StatusDot({ levels }: { levels?: AudioSourceLevels }) {
  const hasError = levels?.state === "error";
  return (
    <span
      title={
        hasError
          ? (levels?.errorMessage ?? "error")
          : levels?.state === "live"
            ? "live"
            : "waiting for audio"
      }
      aria-label={`status: ${levels?.state ?? "waiting"}`}
      className={`h-1.5 w-1.5 shrink-0 rounded-full ${
        hasError ? "bg-red-400" : levels?.state === "live" ? "bg-emerald-400" : "bg-amber-300"
      }`}
    />
  );
}

function MuteButton({ source, audio }: { source: Source; audio: AudioSettings }) {
  return (
    <button
      type="button"
      onClick={() => studioSetAudioMuted(source.id, !audio.muted).catch(fail("mute"))}
      title={audio.muted ? "Unmute" : "Mute"}
      aria-label={`${audio.muted ? "Unmute" : "Mute"} ${source.name}`}
      aria-pressed={audio.muted}
      className={`shrink-0 rounded border px-1.5 text-[10px] font-bold ${
        audio.muted
          ? "border-red-400/60 bg-red-500/20 text-red-300"
          : "border-white/10 text-havoc-muted hover:text-havoc-text"
      }`}
    >
      M
    </button>
  );
}

function MonitorButton({ source, audio }: { source: Source; audio: AudioSettings }) {
  return (
    <button
      type="button"
      onClick={() =>
        studioSetAudioMonitor(source.id, MONITOR_NEXT[audio.monitor]).catch(fail("monitor"))
      }
      title={`${MONITOR_LABEL[audio.monitor]} — click to cycle`}
      aria-label={`Monitor mode of ${source.name}: ${MONITOR_LABEL[audio.monitor]}`}
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
  return (
    <button
      type="button"
      onClick={onOpenFilters}
      title="Audio filters (denoise, gate, compressor…)"
      aria-label={`Audio filters for ${source.name}`}
      className="shrink-0 rounded border border-white/10 px-1.5 text-[10px] text-havoc-muted hover:text-havoc-text"
    >
      ƒ{audio.filters.length > 0 ? audio.filters.length : ""}
    </button>
  );
}

function AdvancedButton({ source, onOpenAdvanced }: { source: Source; onOpenAdvanced: () => void }) {
  return (
    <button
      type="button"
      onClick={onOpenAdvanced}
      title="Sync offset & push-to-talk hotkeys"
      aria-label={`Advanced audio settings for ${source.name}`}
      className="shrink-0 rounded border border-white/10 px-1.5 text-[10px] text-havoc-muted hover:text-havoc-text"
    >
      ⋯
    </button>
  );
}

function TrackDots({ source, audio }: { source: Source; audio: AudioSettings }) {
  return (
    <div className="flex shrink-0 items-center gap-0.5" aria-label="Track assignment">
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
            title={`Track ${index + 1}${assigned ? " (assigned)" : ""}`}
            aria-label={`Track ${index + 1} for ${source.name}`}
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
  onOpenFilters,
  onOpenAdvanced,
}: ChannelStripProps) {
  // Fader drags stream; keep a local draft for instant feedback.
  const [draftDb, setDraftDb] = useState<number | null>(null);
  const volumeDb = draftDb ?? audio.volumeDb;
  const gated = levels?.gated ?? (audio.muted || Boolean(audio.pushToTalk));
  const hasError = levels?.state === "error";

  const onFaderChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const value = Number(event.target.value);
    setDraftDb(value);
    studioSetAudioVolume(source.id, value).catch(fail("volume"));
  };
  const commitVolume = () => {
    if (draftDb === null) return;
    const value = draftDb;
    setDraftDb(null);
    studioSetAudioVolume(source.id, value).catch(fail("volume"));
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
            {levels?.errorMessage ?? "device error"}
          </p>
        ) : (
          <div className="flex min-h-0 flex-1 items-stretch gap-1.5 py-0.5">
            {/* dB tick scale */}
            <div className="flex flex-col justify-between py-0.5 text-[7px] leading-none text-havoc-muted tabular-nums">
              {DB_TICKS.map((tick) => (
                <span key={tick}>{tick}</span>
              ))}
            </div>
            <VuMeter levels={levels} dimmed={gated} orientation="vertical" />
            <input
              type="range"
              min={MIN_VOLUME_DB}
              max={MAX_VOLUME_DB}
              step={0.5}
              value={volumeDb}
              onChange={onFaderChange}
              onMouseUp={commitVolume}
              onKeyUp={commitVolume}
              aria-label={`Volume of ${source.name} in decibels`}
              className="accent-havoc-accent"
              style={{ writingMode: "vertical-lr", direction: "rtl", width: "1rem" }}
            />
          </div>
        )}
        {audio.pushToTalk && (
          <span
            title={`Push-to-talk: hold ${audio.pushToTalk}`}
            className={`rounded px-1 text-[8px] uppercase ${
              gated ? "bg-white/10 text-havoc-muted" : "bg-emerald-500/20 text-emerald-300"
            }`}
          >
            PTT
          </span>
        )}
        <div className="flex w-full items-center justify-center gap-0.5">
          <MuteButton source={source} audio={audio} />
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
            title={`Push-to-talk: hold ${audio.pushToTalk}`}
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
        <MuteButton source={source} audio={audio} />
        <MonitorButton source={source} audio={audio} />
        <FiltersButton source={source} audio={audio} onOpenFilters={onOpenFilters} />
        <AdvancedButton source={source} onOpenAdvanced={onOpenAdvanced} />
      </div>

      {hasError ? (
        <p className="m-0 truncate text-[10px] text-red-400" title={levels?.errorMessage}>
          {levels?.errorMessage ?? "audio device error"}
        </p>
      ) : (
        <VuMeter levels={levels} dimmed={gated} orientation="horizontal" />
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
          aria-label={`Volume of ${source.name} in decibels`}
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
  const [ptt, setPtt] = useState(audio.pushToTalk ?? "");
  const [ptm, setPtm] = useState(audio.pushToMute ?? "");
  const hotkeyClass =
    "rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text outline-none focus:border-havoc-accent/60";

  return (
    <div className="flex flex-col gap-2">
      <NumberField
        label={`Sync offset (ms, 0–${MAX_SYNC_OFFSET_MS} — delays this audio)`}
        value={audio.syncOffsetMs}
        min={0}
        max={MAX_SYNC_OFFSET_MS}
        step={5}
        onCommit={(value) =>
          studioSetAudioSyncOffset(source.id, Math.round(value)).catch(fail("sync offset"))
        }
      />
      <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
        Push-to-talk hotkey (silent unless held)
        <input
          value={ptt}
          onChange={(event) => setPtt(event.target.value)}
          placeholder="e.g. Ctrl+Shift+T or F13"
          aria-label="Push-to-talk hotkey"
          className={hotkeyClass}
        />
      </label>
      <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
        Push-to-mute hotkey (silent while held)
        <input
          value={ptm}
          onChange={(event) => setPtm(event.target.value)}
          placeholder="e.g. Ctrl+Shift+M"
          aria-label="Push-to-mute hotkey"
          className={hotkeyClass}
        />
      </label>
      <p className="m-0 text-[10px] leading-snug text-havoc-muted">
        Hotkeys work while other apps are focused. On Linux/Wayland, global hotkeys may be
        unavailable — that's a compositor limit, said honestly.
      </p>
      <button
        type="button"
        onClick={() => onSetHotkeys(ptt.trim() || null, ptm.trim() || null)}
        className="self-end rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
      >
        Apply
      </button>
    </div>
  );
}
