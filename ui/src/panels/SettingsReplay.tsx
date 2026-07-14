import { useState } from "react";

import { settingsSet } from "../api/commands";
import type { ReplaySettings, Settings } from "../api/types";
import { NumberField } from "../components/NumberField";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

const chipBase =
  "rounded-md border px-2 py-1 text-[11px] transition-colors hover:border-havoc-accent/60";

const LENGTH_PRESETS: Array<[string, number]> = [
  ["replay-length-15s", 15],
  ["replay-length-30s", 30],
  ["replay-length-1min", 60],
  ["replay-length-2min", 120],
  ["replay-length-5min", 300],
];

const QUALITY_PRESETS: Array<[string, number]> = [
  ["replay-quality-low", 3000],
  ["replay-quality-standard", 6000],
  ["replay-quality-high", 12000],
];

/**
 * The Replay editor body (TASK-603): the rolling buffer's length/quality
 * presets. Pure draft editing — every change goes through `onChange`; nothing
 * saves until the caller does (the unified modal's Apply, or the standalone
 * dialog's Save). Changes apply the next time the buffer is armed.
 */
export function ReplaySettingsBody({
  replay,
  onChange,
}: {
  replay: ReplaySettings;
  onChange: (next: ReplaySettings) => void;
}) {
  const t = useT();
  const patch = (part: Partial<ReplaySettings>) => onChange({ ...replay, ...part });

  return (
    <div className="flex flex-col gap-3 text-xs text-havoc-text">
      <div className="flex flex-col gap-1 text-[11px] text-havoc-muted">
        {t("replay-length-presets")}
        <div className="flex flex-wrap gap-1.5">
          {LENGTH_PRESETS.map(([label, seconds]) => (
            <button
              key={label}
              type="button"
              onClick={() => patch({ seconds })}
              aria-pressed={replay.seconds === seconds}
              className={`${chipBase} ${
                replay.seconds === seconds
                  ? "border-havoc-accent/70 bg-havoc-accent/15 text-havoc-text"
                  : "border-white/10 text-havoc-muted"
              }`}
            >
              {t(label)}
            </button>
          ))}
        </div>
      </div>

      <div className="flex flex-col gap-1 text-[11px] text-havoc-muted">
        {t("replay-quality-presets")}
        <div className="flex flex-wrap gap-1.5">
          {QUALITY_PRESETS.map(([label, bitrateKbps]) => (
            <button
              key={label}
              type="button"
              onClick={() => patch({ bitrateKbps })}
              aria-pressed={replay.bitrateKbps === bitrateKbps}
              className={`${chipBase} ${
                replay.bitrateKbps === bitrateKbps
                  ? "border-havoc-accent/70 bg-havoc-accent/15 text-havoc-text"
                  : "border-white/10 text-havoc-muted"
              }`}
            >
              {t(label)}
            </button>
          ))}
        </div>
      </div>

      <div className="grid grid-cols-2 gap-2">
        <NumberField
          label={t("replay-length-seconds")}
          value={replay.seconds}
          min={5}
          max={300}
          step={5}
          onCommit={(value) => patch({ seconds: Math.round(value) })}
        />
        <NumberField
          label={t("replay-video-bitrate")}
          value={replay.bitrateKbps}
          min={500}
          max={60000}
          step={500}
          onCommit={(value) => patch({ bitrateKbps: Math.round(value) })}
        />
        <NumberField
          label={t("replay-fps")}
          value={replay.fps}
          min={1}
          max={240}
          onCommit={(value) => patch({ fps: Math.round(value) })}
        />
        <NumberField
          label={t("replay-audio-track")}
          value={replay.track}
          min={1}
          max={6}
          onCommit={(value) => patch({ track: Math.round(value) })}
        />
      </div>

      <p className="m-0 text-[10px] leading-snug text-havoc-muted">
        {t("replay-note", {
          mb: Math.round((replay.seconds * (replay.bitrateKbps + replay.audioBitrateKbps)) / 8000),
        })}
      </p>
    </div>
  );
}

/**
 * Settings → Replay as a standalone dialog — the Controls dock's "Replay…"
 * button. The unified Settings modal renders `ReplaySettingsBody` instead.
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
  const t = useT();
  const [draft, setDraft] = useState<ReplaySettings | null>(settings?.replay ?? null);
  const [error, setError] = useState<string | null>(null);

  if (!settings || !draft) return null;

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
    <PickerShell title={t("replay-title")} onClose={onClose}>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <ReplaySettingsBody replay={draft} onChange={setDraft} />
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
            {t("replay-cancel")}
          </button>
          <button
            type="button"
            onClick={save}
            className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
          >
            {t("replay-save")}
          </button>
        </div>
      </div>
    </PickerShell>
  );
}
