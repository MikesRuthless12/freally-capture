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
import { useT } from "../i18n/t";

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
  outputWidth: 0,
  outputHeight: 0,
});

/**
 * The Streaming editor body (TASK-502 / TASK-601): the Go Live target list.
 * Go Live publishes to every enabled target at once — targets with equal
 * encode settings share one encode. Stream keys are SECRETS — masked password
 * fields with an explicit reveal, never logged anywhere. Pure draft editing —
 * the caller saves (the unified modal's Apply, or the standalone Save).
 */
export function StreamSettingsBody({
  stream,
  onChange,
  onOpenComponents,
}: {
  stream: StreamSettings;
  onChange: (next: StreamSettings) => void;
  onOpenComponents: () => void;
}) {
  const t = useT();
  const [encoders, setEncoders] = useState<EncoderDesc[] | null>(null);
  const [shownKeys, setShownKeys] = useState<Record<number, boolean>>({});

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

  const patchTarget = (index: number, part: Partial<StreamTargetSettings>) =>
    onChange({
      ...stream,
      targets: stream.targets.map((target, at) => (at === index ? { ...target, ...part } : target)),
    });

  const addTarget = () => onChange({ ...stream, targets: [...stream.targets, defaultTarget()] });

  const removeTarget = (index: number) =>
    onChange({ ...stream, targets: stream.targets.filter((_, at) => at !== index) });

  return (
    <div className="flex flex-col gap-3 text-xs text-havoc-text">
      {stream.targets.map((target, index) => (
        <fieldset
          key={index}
          className="flex flex-col gap-3 rounded-lg border border-white/10 bg-white/[0.02] p-3"
        >
          <legend className="flex items-center gap-2 px-1 text-[11px] text-havoc-muted">
            <input
              type="checkbox"
              checked={target.enabled}
              onChange={(event) => patchTarget(index, { enabled: event.target.checked })}
              aria-label={t("stream-target-enabled", { index: index + 1 })}
            />
            {t("stream-target", { index: index + 1 })}
            {stream.targets.length > 1 && (
              <button
                type="button"
                onClick={() => removeTarget(index)}
                className="rounded border border-white/10 px-1.5 text-[10px] text-havoc-muted hover:border-red-400/50 hover:text-red-300"
              >
                {t("stream-remove")}
              </button>
            )}
          </legend>

          <div className="grid grid-cols-2 gap-2">
            <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
              {t("stream-service")}
              <select
                value={target.service}
                onChange={(event) =>
                  patchTarget(index, { service: event.target.value as StreamService })
                }
                className={inputClass}
              >
                {STREAM_SERVICES.map(([value, labelKey]) => (
                  <option key={value} value={value}>
                    {t(labelKey)}
                  </option>
                ))}
              </select>
            </label>
            <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
              {t("stream-canvas")}
              <select
                value={target.canvas}
                onChange={(event) =>
                  patchTarget(index, { canvas: event.target.value as "main" | "vertical" })
                }
                className={inputClass}
              >
                <option value="main">{t("stream-canvas-main")}</option>
                <option value="vertical">{t("stream-canvas-vertical")}</option>
              </select>
            </label>
          </div>

          {(target.service === "custom" ||
            target.service === "srt" ||
            target.service === "whip" ||
            target.ingestUrl) && (
            <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
              {target.service === "srt"
                ? t("stream-ingest-srt")
                : target.service === "whip"
                  ? t("stream-ingest-whip")
                  : t("stream-ingest-url")}{" "}
              {target.service !== "custom" &&
                target.service !== "srt" &&
                target.service !== "whip" &&
                t("stream-ingest-override")}
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
              ? t("stream-key-srt")
              : target.service === "whip"
                ? t("stream-key-whip")
                : target.service === "custom"
                  ? t("stream-key-custom")
                  : t("stream-key-service")}
            <div className="flex gap-2">
              <input
                type={shownKeys[index] ? "text" : "password"}
                value={target.streamKey}
                onChange={(event) => patchTarget(index, { streamKey: event.target.value })}
                autoComplete="off"
                aria-label={t("stream-key-aria", { index: index + 1 })}
                className={`${inputClass} min-w-0 flex-1 font-mono`}
              />
              <button
                type="button"
                onClick={() => setShownKeys((shown) => ({ ...shown, [index]: !shown[index] }))}
                aria-pressed={Boolean(shownKeys[index])}
                className="shrink-0 rounded-md border border-white/10 px-2.5 text-[11px] text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
              >
                {shownKeys[index] ? t("stream-key-hide") : t("stream-key-show")}
              </button>
            </div>
          </label>

          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t("stream-encoder")}
            <select
              value={target.encoderId}
              onChange={(event) => patchTarget(index, { encoderId: event.target.value })}
              className={inputClass}
            >
              <option value="auto">{t("stream-encoder-auto")}</option>
              {(encoders ?? []).map((desc) => (
                <option key={desc.id} value={desc.id} disabled={desc.verified === false}>
                  {desc.label}
                  {desc.verified === false ? ` ${t("stream-encoder-unavailable")}` : ""}
                </option>
              ))}
            </select>
          </label>

          <div className="grid grid-cols-2 gap-2">
            <NumberField
              label={t("stream-video-bitrate")}
              value={target.bitrateKbps}
              min={500}
              max={60000}
              step={500}
              onCommit={(value) => patchTarget(index, { bitrateKbps: Math.round(value) })}
            />
            <NumberField
              label={t("stream-audio-bitrate")}
              value={target.audioBitrateKbps}
              min={32}
              max={512}
              step={32}
              onCommit={(value) => patchTarget(index, { audioBitrateKbps: Math.round(value) })}
            />
            <NumberField
              label={t("stream-fps")}
              value={target.fps}
              min={1}
              max={240}
              onCommit={(value) => patchTarget(index, { fps: Math.round(value) })}
            />
            <NumberField
              label={t("stream-keyframe")}
              value={target.keyframeSec}
              min={0.25}
              max={10}
              step={0.25}
              onCommit={(value) => patchTarget(index, { keyframeSec: value })}
            />
            <NumberField
              label={t("stream-audio-track")}
              value={target.track}
              min={1}
              max={6}
              onCommit={(value) => patchTarget(index, { track: Math.round(value) })}
            />
            <NumberField
              label={t("stream-output-width")}
              value={target.outputWidth}
              min={0}
              max={16384}
              step={2}
              onCommit={(value) => patchTarget(index, { outputWidth: Math.round(value) })}
            />
            <NumberField
              label={t("stream-output-height")}
              value={target.outputHeight}
              min={0}
              max={16384}
              step={2}
              onCommit={(value) => patchTarget(index, { outputHeight: Math.round(value) })}
            />
          </div>
        </fieldset>
      ))}

      {stream.targets.length < MAX_TARGETS && (
        <button
          type="button"
          onClick={addTarget}
          className="self-start rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
        >
          {t("stream-add-target")}
        </button>
      )}

      <p className="m-0 text-[10px] leading-snug text-havoc-muted">{t("stream-go-live-note")}</p>

      <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
        <input
          type="checkbox"
          checked={stream.autoRecord}
          onChange={(event) => onChange({ ...stream, autoRecord: event.target.checked })}
        />
        {t("stream-auto-record")}
      </label>

      <p className="m-0 text-[10px] leading-snug text-havoc-muted">
        {t("stream-ffmpeg-note-before")}{" "}
        <button
          type="button"
          onClick={onOpenComponents}
          className="underline hover:text-havoc-text"
        >
          {t("stream-ffmpeg-note-link")}
        </button>
        {t("stream-ffmpeg-note-after")}
      </p>
    </div>
  );
}

/**
 * Settings → Streaming as a standalone dialog — the Controls dock's
 * "Streaming…" button and the Go Live "needs settings" path. The unified
 * Settings modal renders `StreamSettingsBody` instead.
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
  const t = useT();
  const [draft, setDraft] = useState<StreamSettings | null>(settings?.stream ?? null);
  const [error, setError] = useState<string | null>(null);

  if (!settings || !draft) return null;

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
    <PickerShell title={t("stream-title")} onClose={onClose} wide>
      <div className="flex max-h-[70vh] flex-col gap-3 overflow-y-auto pr-1 text-xs text-havoc-text">
        <StreamSettingsBody
          stream={draft}
          onChange={setDraft}
          onOpenComponents={onOpenComponents}
        />
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
            {t("stream-cancel")}
          </button>
          <button
            type="button"
            onClick={save}
            className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
          >
            {t("stream-save")}
          </button>
        </div>
      </div>
    </PickerShell>
  );
}
