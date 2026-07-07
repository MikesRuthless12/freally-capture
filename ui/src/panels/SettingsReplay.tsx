import { useState } from "react";

import { settingsSet } from "../api/commands";
import type { ReplaySettings, Settings } from "../api/types";
import { NumberField } from "../components/NumberField";
import { PickerShell } from "../components/PickerShell";

const chipBase =
  "rounded-md border px-2 py-1 text-[11px] transition-colors hover:border-havoc-accent/60";

const LENGTH_PRESETS: Array<[string, number]> = [
  ["15 s", 15],
  ["30 s", 30],
  ["1 min", 60],
  ["2 min", 120],
  ["5 min", 300],
];

const QUALITY_PRESETS: Array<[string, number]> = [
  ["Low (3 Mbps)", 3000],
  ["Standard (6 Mbps)", 6000],
  ["High (12 Mbps)", 12000],
];

/**
 * Settings → Replay (TASK-603): the rolling buffer's length/quality presets.
 * Changes apply the next time the buffer is armed.
 */
export function SettingsReplay({
  settings,
  onSaved,
  onClose,
}: {
  settings: Settings | null;
  onSaved: (next: Settings) => void;
  onClose: () => void;
}) {
  const [draft, setDraft] = useState<ReplaySettings | null>(settings?.replay ?? null);
  const [error, setError] = useState<string | null>(null);

  if (!settings || !draft) return null;

  const patch = (part: Partial<ReplaySettings>) => setDraft({ ...draft, ...part });

  const save = () => {
    setError(null);
    const next = { ...settings, replay: draft };
    settingsSet(next)
      .then(() => {
        onSaved(next);
        onClose();
      })
      .catch((err) => setError(String(err)));
  };

  return (
    <PickerShell title="Settings — Replay Buffer" onClose={onClose}>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <div className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          Length presets
          <div className="flex flex-wrap gap-1.5">
            {LENGTH_PRESETS.map(([label, seconds]) => (
              <button
                key={label}
                type="button"
                onClick={() => patch({ seconds })}
                aria-pressed={draft.seconds === seconds}
                className={`${chipBase} ${
                  draft.seconds === seconds
                    ? "border-havoc-accent/70 bg-havoc-accent/15 text-havoc-text"
                    : "border-white/10 text-havoc-muted"
                }`}
              >
                {label}
              </button>
            ))}
          </div>
        </div>

        <div className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          Quality presets
          <div className="flex flex-wrap gap-1.5">
            {QUALITY_PRESETS.map(([label, bitrateKbps]) => (
              <button
                key={label}
                type="button"
                onClick={() => patch({ bitrateKbps })}
                aria-pressed={draft.bitrateKbps === bitrateKbps}
                className={`${chipBase} ${
                  draft.bitrateKbps === bitrateKbps
                    ? "border-havoc-accent/70 bg-havoc-accent/15 text-havoc-text"
                    : "border-white/10 text-havoc-muted"
                }`}
              >
                {label}
              </button>
            ))}
          </div>
        </div>

        <div className="grid grid-cols-2 gap-2">
          <NumberField
            label="Length (seconds)"
            value={draft.seconds}
            min={5}
            max={300}
            step={5}
            onCommit={(value) => patch({ seconds: Math.round(value) })}
          />
          <NumberField
            label="Video bitrate (kbps)"
            value={draft.bitrateKbps}
            min={500}
            max={60000}
            step={500}
            onCommit={(value) => patch({ bitrateKbps: Math.round(value) })}
          />
          <NumberField
            label="FPS"
            value={draft.fps}
            min={1}
            max={240}
            onCommit={(value) => patch({ fps: Math.round(value) })}
          />
          <NumberField
            label="Audio track (1–6)"
            value={draft.track}
            min={1}
            max={6}
            onCommit={(value) => patch({ track: Math.round(value) })}
          />
        </div>

        <p className="m-0 text-[10px] leading-snug text-havoc-muted">
          While armed, the buffer runs its own lightweight encode into a bounded on-disk ring —
          about {Math.round((draft.seconds * (draft.bitrateKbps + draft.audioBitrateKbps)) / 8000)}{" "}
          MB at these settings. Saving stitches the ring without re-encoding and never touches the
          stream or the recording. Changes apply the next time you arm.
        </p>

        {error && (
          <p role="alert" className="m-0 text-[11px] text-red-300">
            {error}
          </p>
        )}
        <div className="flex justify-end gap-2">
          <button
            type="button"
            onClick={onClose}
            className="rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-muted hover:text-havoc-text"
          >
            Cancel
          </button>
          <button
            type="button"
            onClick={save}
            className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
          >
            Save
          </button>
        </div>
      </div>
    </PickerShell>
  );
}
