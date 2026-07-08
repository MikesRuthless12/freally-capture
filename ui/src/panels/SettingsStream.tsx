import { useEffect, useState } from "react";

import { encodersList, settingsSet } from "../api/commands";
import type { EncoderDesc, Settings, StreamService, StreamSettings } from "../api/types";
import { STREAM_SERVICES } from "../api/types";
import { NumberField } from "../components/NumberField";
import { PickerShell } from "../components/PickerShell";

const inputClass =
  "rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text outline-none focus:border-havoc-accent/60";

/**
 * Settings → Stream (TASK-502): the Go Live target. Single service this
 * phase (multistream lands in Phase 6). The stream key is a SECRET — a
 * masked password field with an explicit reveal, never logged anywhere.
 */
export function SettingsStream({
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
  const [draft, setDraft] = useState<StreamSettings | null>(settings?.stream ?? null);
  const [encoders, setEncoders] = useState<EncoderDesc[] | null>(null);
  const [showKey, setShowKey] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    encodersList()
      .then((catalog) => {
        if (cancelled) return;
        // RTMP/FLV carries H.264 — only those encoders are offerable.
        setEncoders(catalog.encoders.filter((desc) => desc.codec === "h264"));
      })
      .catch(() => {
        if (!cancelled) setEncoders([]);
      });
    return () => {
      cancelled = true;
    };
  }, []);

  if (!settings || !draft) return null;

  const patch = (part: Partial<StreamSettings>) => setDraft({ ...draft, ...part });

  const save = () => {
    setError(null);
    const next = { ...settings, stream: draft };
    settingsSet(next)
      .then(() => {
        onSaved(next);
        onClose();
      })
      .catch((err) => setError(String(err)));
  };

  return (
    <PickerShell title="Settings — Stream" onClose={onClose} wide>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          Service
          <select
            value={draft.service}
            onChange={(event) => patch({ service: event.target.value as StreamService })}
            className={inputClass}
          >
            {STREAM_SERVICES.map(([value, label]) => (
              <option key={value} value={value}>
                {label}
              </option>
            ))}
          </select>
        </label>

        {(draft.service === "custom" || draft.ingestUrl) && (
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            Ingest URL {draft.service !== "custom" && "(override — empty = the service preset)"}
            <input
              value={draft.ingestUrl}
              onChange={(event) => patch({ ingestUrl: event.target.value })}
              placeholder="rtmps://ingest.example.com/live"
              className={`${inputClass} font-mono`}
            />
          </label>
        )}

        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          Stream key (from your {draft.service === "custom" ? "server" : "creator dashboard"} —
          treated as a secret)
          <div className="flex gap-2">
            <input
              type={showKey ? "text" : "password"}
              value={draft.streamKey}
              onChange={(event) => patch({ streamKey: event.target.value })}
              autoComplete="off"
              aria-label="Stream key"
              className={`${inputClass} min-w-0 flex-1 font-mono`}
            />
            <button
              type="button"
              onClick={() => setShowKey((shown) => !shown)}
              aria-pressed={showKey}
              className="shrink-0 rounded-md border border-white/10 px-2.5 text-[11px] text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
            >
              {showKey ? "Hide" : "Show"}
            </button>
          </div>
        </label>

        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          Encoder (H.264 — what RTMP carries)
          <select
            value={draft.encoderId}
            onChange={(event) => patch({ encoderId: event.target.value })}
            className={inputClass}
          >
            <option value="auto">Auto — the best detected H.264 encoder</option>
            {(encoders ?? []).map((desc) => (
              <option key={desc.id} value={desc.id} disabled={desc.verified === false}>
                {desc.label}
                {desc.verified === false ? " (unavailable here)" : ""}
              </option>
            ))}
          </select>
        </label>

        <div className="grid grid-cols-2 gap-2">
          <NumberField
            label="Video bitrate (kbps, CBR)"
            value={draft.bitrateKbps}
            min={500}
            max={60000}
            step={500}
            onCommit={(value) => patch({ bitrateKbps: Math.round(value) })}
          />
          <NumberField
            label="Audio bitrate (kbps)"
            value={draft.audioBitrateKbps}
            min={32}
            max={512}
            step={32}
            onCommit={(value) => patch({ audioBitrateKbps: Math.round(value) })}
          />
          <NumberField
            label="FPS"
            value={draft.fps}
            min={1}
            max={240}
            onCommit={(value) => patch({ fps: Math.round(value) })}
          />
          <NumberField
            label="Keyframe interval (s)"
            value={draft.keyframeSec}
            min={0.25}
            max={10}
            step={0.25}
            onCommit={(value) => patch({ keyframeSec: value })}
          />
          <NumberField
            label="Audio track (1–6)"
            value={draft.track}
            min={1}
            max={6}
            onCommit={(value) => patch({ track: Math.round(value) })}
          />
        </div>

        <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
          <input
            type="checkbox"
            checked={draft.autoRecord}
            onChange={(event) => patch({ autoRecord: event.target.checked })}
          />
          Start recording when I go live (the recording still stops independently)
        </label>

        <p className="m-0 text-[10px] leading-snug text-havoc-muted">
          Streaming wire codecs run through the labeled on-demand ffmpeg component —{" "}
          <button
            type="button"
            onClick={onOpenComponents}
            className="underline hover:text-havoc-text"
          >
            manage it here
          </button>
          . The local recording keeps running no matter what the stream does.
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
