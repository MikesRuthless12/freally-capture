import { useEffect, useState } from "react";

import { ffmpegStatus, settingsSet } from "../api/commands";
import { onFfmpeg } from "../api/events";
import type { Container, FfmpegStatus, RecordingSettings, Settings } from "../api/types";
import { TRACK_COUNT } from "../api/types";
import { NumberField } from "../components/NumberField";
import { PickerShell } from "../components/PickerShell";

const selectClass =
  "rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text outline-none focus:border-havoc-accent/60";
const inputClass = selectClass;

const CONTAINERS: { value: Container; label: string; wire: boolean }[] = [
  {
    value: "frec",
    label: "freally-video (.frec) — lossless, owned, nothing to download",
    wire: false,
  },
  { value: "mkv", label: "MKV — crash-tolerant; remux to mp4 later", wire: true },
  { value: "mp4", label: "MP4 — plays everywhere", wire: true },
  { value: "mov", label: "MOV", wire: true },
  { value: "webm", label: "WebM (AV1 + Opus)", wire: true },
];

const FPS_CHOICES = [24, 30, 50, 60];

/**
 * Settings → Output: where recordings go and what they are — container
 * (owned lossless .frec by default), folder/filename, fps, the up-to-6
 * recorded tracks, splitting, and the separate-local-copy intent. The
 * encoder + rate-control depth lands here too (P4.6).
 */
export function SettingsOutput({
  settings,
  onSaved,
  onClose,
  onOpenComponents,
}: {
  settings: Settings | null;
  onSaved: (next: Settings) => void;
  onClose: () => void;
  onOpenComponents: () => void;
}) {
  const [saveError, setSaveError] = useState<string | null>(null);
  const [ffmpeg, setFfmpeg] = useState<FfmpegStatus | null>(null);

  useEffect(() => {
    let alive = true;
    let unlisten: (() => void) | undefined;
    ffmpegStatus()
      .then((status) => alive && setFfmpeg(status))
      .catch(() => alive && setFfmpeg(null));
    onFfmpeg((status) => setFfmpeg(status)).then((fn) => {
      if (alive) unlisten = fn;
      else fn();
    });
    return () => {
      alive = false;
      unlisten?.();
    };
  }, []);

  if (!settings) {
    return (
      <PickerShell title="Output" onClose={onClose}>
        <p className="m-0 text-xs text-havoc-muted">Settings are still loading…</p>
      </PickerShell>
    );
  }
  const rec = settings.recording;

  const save = (patch: Partial<RecordingSettings>) => {
    const next: Settings = { ...settings, recording: { ...settings.recording, ...patch } };
    setSaveError(null);
    settingsSet(next)
      .then(() => onSaved(next))
      .catch((err) => setSaveError(String(err)));
  };

  const wire = rec.container !== "frec";
  const ffmpegReady = ffmpeg?.state === "ready";

  return (
    <PickerShell title="Output" onClose={onClose} wide>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          Recording format
          <select
            value={rec.container}
            onChange={(event) => save({ container: event.target.value as Container })}
            className={selectClass}
          >
            {CONTAINERS.map((entry) => (
              <option key={entry.value} value={entry.value}>
                {entry.label}
              </option>
            ))}
          </select>
        </label>

        {wire && !ffmpegReady && (
          <div className="flex items-center justify-between gap-2 rounded-lg border border-amber-400/30 bg-amber-400/10 px-2.5 py-2">
            <span className="text-[11px] text-amber-200">
              This format needs the FFmpeg component (wire codecs — not bundled). Lossless .frec
              needs nothing.
            </span>
            <button
              type="button"
              onClick={onOpenComponents}
              className="shrink-0 rounded-md border border-amber-400/40 px-2 py-1 text-[11px] text-amber-200 transition-colors hover:border-amber-300"
            >
              Install…
            </button>
          </div>
        )}

        <div className="grid grid-cols-2 gap-2">
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            Recordings folder
            <input
              type="text"
              value={rec.folder}
              placeholder="OS Videos folder"
              onChange={(event) => save({ folder: event.target.value })}
              className={inputClass}
            />
          </label>
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            Filename prefix
            <input
              type="text"
              value={rec.filenamePrefix}
              onChange={(event) => save({ filenamePrefix: event.target.value })}
              className={inputClass}
            />
          </label>
        </div>

        <div className="grid grid-cols-3 items-end gap-2">
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            Frame rate
            <select
              value={rec.fps}
              onChange={(event) => save({ fps: Number(event.target.value) })}
              className={selectClass}
            >
              {FPS_CHOICES.map((fps) => (
                <option key={fps} value={fps}>
                  {fps} fps
                </option>
              ))}
            </select>
          </label>
          <NumberField
            label="Split every (minutes, 0 = off)"
            value={rec.splitMinutes}
            min={0}
            max={1440}
            onCommit={(value) => save({ splitMinutes: Math.round(value) })}
          />
          <div className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            Audio tracks
            <div className="flex items-center gap-1" role="group" aria-label="Recorded tracks">
              {Array.from({ length: TRACK_COUNT }, (_, index) => {
                const on = (rec.tracksMask & (1 << index)) !== 0;
                const lastOne = on && rec.tracksMask === 1 << index;
                return (
                  <button
                    key={index}
                    type="button"
                    title={
                      lastOne
                        ? "At least one track must record"
                        : `Record track ${index + 1}: ${on ? "on" : "off"}`
                    }
                    aria-pressed={on}
                    disabled={lastOne}
                    onClick={() => save({ tracksMask: rec.tracksMask ^ (1 << index) })}
                    className={`h-6 w-6 rounded-md border text-[10px] transition-colors disabled:cursor-not-allowed ${
                      on
                        ? "border-havoc-accent/60 bg-havoc-accent/25 text-havoc-text"
                        : "border-white/10 bg-white/[0.04] text-havoc-muted hover:text-havoc-text"
                    }`}
                  >
                    {index + 1}
                  </button>
                );
              })}
            </div>
          </div>
        </div>

        <label className="flex items-start gap-2 text-[11px] text-havoc-muted">
          <input
            type="checkbox"
            checked={rec.separateLocalCopy}
            onChange={(event) => save({ separateLocalCopy: event.target.checked })}
            className="mt-0.5"
          />
          <span>
            Record a separate local copy while streaming — takes effect when streaming lands (0.70);
            the local recording never rides a stream&apos;s settings.
          </span>
        </label>

        {saveError && (
          <p role="alert" className="m-0 text-[11px] text-red-300">
            {saveError}
          </p>
        )}
      </div>
    </PickerShell>
  );
}
