import { useEffect, useState } from "react";

import { encodersList, settingsSet } from "../api/commands";
import type {
  EncoderDesc,
  Settings,
  StreamService,
  StreamSettings,
  StreamTargetSettings,
} from "../api/types";
import { STREAM_SERVICES } from "../api/types";
import { NumberField } from "../components/NumberField";
import { PickerShell } from "../components/PickerShell";

const inputClass =
  "rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text outline-none focus:border-havoc-accent/60";

const MAX_TARGETS = 6;

const defaultTarget = (): StreamTargetSettings => ({
  enabled: true,
  service: "twitch",
  canvas: "main",
  ingestUrl: "",
  streamKey: "",
  encoderId: "auto",
  bitrateKbps: 6000,
  audioBitrateKbps: 160,
  keyframeSec: 2,
  fps: 60,
  track: 1,
});

/**
 * Settings → Stream (TASK-502 / TASK-601): the Go Live target list. Go Live
 * publishes to every enabled target at once — targets with equal encode
 * settings share one encode. Stream keys are SECRETS — masked password
 * fields with an explicit reveal, never logged anywhere.
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
  const [shownKeys, setShownKeys] = useState<Record<number, boolean>>({});
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

  const patchTarget = (index: number, part: Partial<StreamTargetSettings>) =>
    setDraft({
      ...draft,
      targets: draft.targets.map((target, at) => (at === index ? { ...target, ...part } : target)),
    });

  const addTarget = () => setDraft({ ...draft, targets: [...draft.targets, defaultTarget()] });

  const removeTarget = (index: number) =>
    setDraft({ ...draft, targets: draft.targets.filter((_, at) => at !== index) });

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
      <div className="flex max-h-[70vh] flex-col gap-3 overflow-y-auto pr-1 text-xs text-havoc-text">
        {draft.targets.map((target, index) => (
          <fieldset
            key={index}
            className="flex flex-col gap-3 rounded-lg border border-white/10 bg-white/[0.02] p-3"
          >
            <legend className="flex items-center gap-2 px-1 text-[11px] text-havoc-muted">
              <input
                type="checkbox"
                checked={target.enabled}
                onChange={(event) => patchTarget(index, { enabled: event.target.checked })}
                aria-label={`Target ${index + 1} enabled`}
              />
              Target {index + 1}
              {draft.targets.length > 1 && (
                <button
                  type="button"
                  onClick={() => removeTarget(index)}
                  className="rounded border border-white/10 px-1.5 text-[10px] text-havoc-muted hover:border-red-400/50 hover:text-red-300"
                >
                  Remove
                </button>
              )}
            </legend>

            <div className="grid grid-cols-2 gap-2">
              <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
                Service
                <select
                  value={target.service}
                  onChange={(event) =>
                    patchTarget(index, { service: event.target.value as StreamService })
                  }
                  className={inputClass}
                >
                  {STREAM_SERVICES.map(([value, label]) => (
                    <option key={value} value={value}>
                      {label}
                    </option>
                  ))}
                </select>
              </label>
              <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
                Canvas
                <select
                  value={target.canvas}
                  onChange={(event) =>
                    patchTarget(index, { canvas: event.target.value as "main" | "vertical" })
                  }
                  className={inputClass}
                >
                  <option value="main">Main (program)</option>
                  <option value="vertical">Vertical (9:16 — enable it in the studio)</option>
                </select>
              </label>
            </div>

            {(target.service === "custom" ||
              target.service === "srt" ||
              target.service === "whip" ||
              target.ingestUrl) && (
              <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
                {target.service === "srt"
                  ? "SRT ingest URL"
                  : target.service === "whip"
                    ? "WHIP endpoint URL"
                    : "Ingest URL"}{" "}
                {target.service !== "custom" &&
                  target.service !== "srt" &&
                  target.service !== "whip" &&
                  "(override — empty = the service preset)"}
                <input
                  value={target.ingestUrl}
                  onChange={(event) => patchTarget(index, { ingestUrl: event.target.value })}
                  placeholder={
                    target.service === "srt"
                      ? "srt://relay.example.net:8890"
                      : target.service === "whip"
                        ? "https://sfu.example.net/whip/room"
                        : "rtmps://ingest.example.com/live"
                  }
                  className={`${inputClass} font-mono`}
                />
              </label>
            )}

            <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
              {target.service === "srt"
                ? "streamid (optional — appended as ?streamid=…; treated as a secret)"
                : target.service === "whip"
                  ? "Bearer token (optional — sent as the Authorization header; a secret)"
                  : `Stream key (from your ${
                      target.service === "custom" ? "server" : "creator dashboard"
                    } — treated as a secret)`}
              <div className="flex gap-2">
                <input
                  type={shownKeys[index] ? "text" : "password"}
                  value={target.streamKey}
                  onChange={(event) => patchTarget(index, { streamKey: event.target.value })}
                  autoComplete="off"
                  aria-label={`Stream key ${index + 1}`}
                  className={`${inputClass} min-w-0 flex-1 font-mono`}
                />
                <button
                  type="button"
                  onClick={() => setShownKeys((shown) => ({ ...shown, [index]: !shown[index] }))}
                  aria-pressed={Boolean(shownKeys[index])}
                  className="shrink-0 rounded-md border border-white/10 px-2.5 text-[11px] text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
                >
                  {shownKeys[index] ? "Hide" : "Show"}
                </button>
              </div>
            </label>

            <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
              Encoder (H.264 — what RTMP, SRT and WHIP all carry)
              <select
                value={target.encoderId}
                onChange={(event) => patchTarget(index, { encoderId: event.target.value })}
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
                value={target.bitrateKbps}
                min={500}
                max={60000}
                step={500}
                onCommit={(value) => patchTarget(index, { bitrateKbps: Math.round(value) })}
              />
              <NumberField
                label="Audio bitrate (kbps)"
                value={target.audioBitrateKbps}
                min={32}
                max={512}
                step={32}
                onCommit={(value) => patchTarget(index, { audioBitrateKbps: Math.round(value) })}
              />
              <NumberField
                label="FPS"
                value={target.fps}
                min={1}
                max={240}
                onCommit={(value) => patchTarget(index, { fps: Math.round(value) })}
              />
              <NumberField
                label="Keyframe interval (s)"
                value={target.keyframeSec}
                min={0.25}
                max={10}
                step={0.25}
                onCommit={(value) => patchTarget(index, { keyframeSec: value })}
              />
              <NumberField
                label="Audio track (1–6)"
                value={target.track}
                min={1}
                max={6}
                onCommit={(value) => patchTarget(index, { track: Math.round(value) })}
              />
            </div>
          </fieldset>
        ))}

        {draft.targets.length < MAX_TARGETS && (
          <button
            type="button"
            onClick={addTarget}
            className="self-start rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
          >
            + Add target
          </button>
        )}

        <p className="m-0 text-[10px] leading-snug text-havoc-muted">
          Go Live publishes to every enabled target at once, direct to each platform. Targets with
          identical encoder settings share a single encode.
        </p>

        <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
          <input
            type="checkbox"
            checked={draft.autoRecord}
            onChange={(event) => setDraft({ ...draft, autoRecord: event.target.checked })}
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
