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

/**
 * The VU meter: a green→yellow→red gradient revealed to the RMS level, with
 * a peak tick. Levels arrive ~20 Hz; a short CSS width transition smooths
 * the motion between events.
 */
function VuMeter({ levels, dimmed }: { levels?: AudioSourceLevels; dimmed: boolean }) {
  const rms = levels ? Math.max(levels.rms[0], levels.rms[1]) : 0;
  const peak = levels ? Math.max(levels.peak[0], levels.peak[1]) : 0;
  const rmsPercent = dbToPercent(linToDb(rms));
  const peakPercent = dbToPercent(linToDb(peak));
  return (
    <div
      role="meter"
      aria-label="Level"
      aria-valuemin={MIN_VOLUME_DB}
      aria-valuemax={0}
      aria-valuenow={Math.round(linToDb(peak))}
      className={`relative h-2 w-full overflow-hidden rounded-sm bg-black/50 ${
        dimmed ? "opacity-40" : ""
      }`}
    >
      <div
        className="h-full transition-[width] duration-75 ease-linear"
        style={{
          width: `${rmsPercent}%`,
          background:
            "linear-gradient(to right, #22c55e 0%, #22c55e 62%, #eab308 78%, #ef4444 95%)",
          backgroundSize: "10rem 100%",
        }}
      />
      {peakPercent > 0 && (
        <div
          className="absolute top-0 h-full w-px bg-white/80 transition-[left] duration-75 ease-linear"
          style={{ left: `${peakPercent}%` }}
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

type ChannelStripProps = {
  source: Source;
  audio: AudioSettings;
  levels?: AudioSourceLevels;
  onOpenFilters: () => void;
  onOpenAdvanced: () => void;
};

/**
 * One mixer strip: name + status, VU meter, fader (dB), mute, monitor cycle,
 * filters, track dots 1–6, and the advanced popover (sync offset, PTT/PTM).
 */
export function ChannelStrip({
  source,
  audio,
  levels,
  onOpenFilters,
  onOpenAdvanced,
}: ChannelStripProps) {
  // Fader drags stream; keep a local draft for instant feedback.
  const [draftDb, setDraftDb] = useState<number | null>(null);
  const volumeDb = draftDb ?? audio.volumeDb;
  const gated = levels?.gated ?? (audio.muted || Boolean(audio.pushToTalk));
  const hasError = levels?.state === "error";

  const commitVolume = (value: number) => {
    setDraftDb(null);
    studioSetAudioVolume(source.id, value).catch(fail("volume"));
  };

  return (
    <div className="flex flex-col gap-1 rounded-lg border border-white/10 bg-white/[0.02] px-2 py-1.5">
      <div className="flex items-center gap-1.5">
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
          {volumeDb <= MIN_VOLUME_DB ? "-inf" : volumeDb.toFixed(1)} dB
        </span>
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
        <button
          type="button"
          onClick={onOpenFilters}
          title="Audio filters (denoise, gate, compressor…)"
          aria-label={`Audio filters for ${source.name}`}
          className="shrink-0 rounded border border-white/10 px-1.5 text-[10px] text-havoc-muted hover:text-havoc-text"
        >
          ƒ{audio.filters.length > 0 ? audio.filters.length : ""}
        </button>
        <button
          type="button"
          onClick={onOpenAdvanced}
          title="Sync offset & push-to-talk hotkeys"
          aria-label={`Advanced audio settings for ${source.name}`}
          className="shrink-0 rounded border border-white/10 px-1.5 text-[10px] text-havoc-muted hover:text-havoc-text"
        >
          ⋯
        </button>
      </div>

      {hasError ? (
        <p className="m-0 truncate text-[10px] text-red-400" title={levels?.errorMessage}>
          {levels?.errorMessage ?? "audio device error"}
        </p>
      ) : (
        <VuMeter levels={levels} dimmed={gated} />
      )}

      <div className="flex items-center gap-2">
        <input
          type="range"
          min={MIN_VOLUME_DB}
          max={MAX_VOLUME_DB}
          step={0.5}
          value={volumeDb}
          onChange={(event) => {
            const value = Number(event.target.value);
            setDraftDb(value);
            studioSetAudioVolume(source.id, value).catch(fail("volume"));
          }}
          onMouseUp={() => draftDb !== null && commitVolume(draftDb)}
          onKeyUp={() => draftDb !== null && commitVolume(draftDb)}
          aria-label={`Volume of ${source.name} in decibels`}
          className="min-w-0 flex-1 accent-havoc-accent"
        />
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
