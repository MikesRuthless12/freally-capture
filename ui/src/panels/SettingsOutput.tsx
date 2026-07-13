import { useEffect, useRef, useState } from "react";

import { encodersList, ffmpegStatus, settingsSet } from "../api/commands";
import { onFfmpeg } from "../api/events";
import type {
  Container,
  EncPreset,
  EncoderCatalog,
  FfmpegStatus,
  RcMode,
  RecordingSettings,
  Settings,
  VideoCodec,
} from "../api/types";
import { TRACK_COUNT } from "../api/types";
import { NumberField } from "../components/NumberField";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

const selectClass =
  "rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text outline-none focus:border-havoc-accent/60";
const inputClass = selectClass;

const CONTAINERS: { value: Container; label: string; wire: boolean }[] = [
  {
    value: "frec",
    label: "output-container-frec",
    wire: false,
  },
  { value: "mkv", label: "output-container-mkv", wire: true },
  { value: "mp4", label: "output-container-mp4", wire: true },
  { value: "mov", label: "output-container-mov", wire: true },
  { value: "webm", label: "output-container-webm", wire: true },
];

const FPS_CHOICES = [24, 30, 50, 60, 120, 144, 240];

/** The codecs each wire container legally holds (mirrors recording.rs). */
function containerAccepts(container: Container, codec: VideoCodec): boolean {
  if (container === "webm") return codec === "av1";
  if (container === "mov") return codec === "h264" || codec === "hevc";
  return true;
}

/** One-click record presets (✨ REC-9). */
const RECORD_PRESETS: {
  label: string;
  title: string;
  patch: Partial<RecordingSettings>;
}[] = [
  {
    label: "output-preset-lossless-label",
    title: "output-preset-lossless-title",
    patch: { container: "frec" },
  },
  {
    label: "output-preset-high-label",
    title: "output-preset-high-title",
    patch: {
      container: "mp4",
      encoderId: "auto",
      rateControl: { mode: "cqp", bitrateKbps: 40000, cq: 16 },
      preset: "quality",
    },
  },
  {
    label: "output-preset-balanced-label",
    title: "output-preset-balanced-title",
    patch: {
      container: "mkv",
      encoderId: "auto",
      rateControl: { mode: "cqp", bitrateKbps: 8000, cq: 23 },
      preset: "balanced",
    },
  },
];

/**
 * Settings → Output: where recordings go and what they are — container
 * (owned lossless .frec by default), folder/filename, fps, the up-to-6
 * recorded tracks, and splitting, plus the encoder + rate-control depth (P4.6).
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
  const t = useT();
  const [saveError, setSaveError] = useState<string | null>(null);
  const [ffmpeg, setFfmpeg] = useState<FfmpegStatus | null>(null);
  const [catalog, setCatalog] = useState<EncoderCatalog | null>(null);
  // The latest known settings, so back-to-back edits accumulate instead of
  // each rebuilding from the render-time prop (which lags the async save +
  // parent re-render, letting the second edit revert the first). Adopt the
  // prop only when it actually changes (an effect, not every render, so an
  // unrelated re-render can't clobber an in-flight optimistic patch).
  const settingsRef = useRef<Settings | null>(settings);
  useEffect(() => {
    settingsRef.current = settings;
  }, [settings]);

  useEffect(() => {
    let alive = true;
    let unlisten: (() => void) | undefined;
    ffmpegStatus()
      .then((status) => alive && setFfmpeg(status))
      .catch(() => alive && setFfmpeg(null));
    encodersList()
      .then((found) => alive && setCatalog(found))
      .catch(() => alive && setCatalog(null));
    onFfmpeg((status) => {
      setFfmpeg(status);
      // A fresh install re-verifies the catalog — refetch it.
      if (status.state === "ready" || status.state === "missing") {
        encodersList()
          .then((found) => setCatalog(found))
          .catch(() => undefined);
      }
    })
      .then((fn) => {
        if (alive) unlisten = fn;
        else fn();
      })
      .catch(() => undefined);
    return () => {
      alive = false;
      unlisten?.();
    };
  }, []);

  if (!settings) {
    return (
      <PickerShell title={t("output-title")} onClose={onClose}>
        <p className="m-0 text-xs text-havoc-muted">{t("output-loading")}</p>
      </PickerShell>
    );
  }
  const rec = settings.recording;

  const save = (patch: Partial<RecordingSettings>) => {
    const base = settingsRef.current ?? settings;
    const next: Settings = { ...base, recording: { ...base.recording, ...patch } };
    settingsRef.current = next; // accumulate before the async round-trip
    setSaveError(null);
    settingsSet(next)
      .then(() => onSaved(next))
      .catch((err) => setSaveError(String(err)));
  };

  const wire = rec.container !== "frec";
  const ffmpegReady = ffmpeg?.state === "ready";

  return (
    <PickerShell title={t("output-title")} onClose={onClose} wide>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("output-recording-format")}
          <select
            value={rec.container}
            onChange={(event) => save({ container: event.target.value as Container })}
            className={selectClass}
          >
            {CONTAINERS.map((entry) => (
              <option key={entry.value} value={entry.value}>
                {t(entry.label)}
              </option>
            ))}
          </select>
        </label>

        {wire && !ffmpegReady && (
          <div className="flex items-center justify-between gap-2 rounded-lg border border-amber-400/30 bg-amber-400/10 px-2.5 py-2">
            <span className="text-[11px] text-amber-200">{t("output-ffmpeg-warning")}</span>
            <button
              type="button"
              onClick={onOpenComponents}
              className="shrink-0 rounded-md border border-amber-400/40 px-2 py-1 text-[11px] text-amber-200 transition-colors hover:border-amber-300"
            >
              {t("output-install")}
            </button>
          </div>
        )}

        <div className="grid grid-cols-2 gap-2">
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t("output-recordings-folder")}
            <input
              type="text"
              value={rec.folder}
              placeholder={t("output-folder-placeholder")}
              onChange={(event) => save({ folder: event.target.value })}
              className={inputClass}
            />
          </label>
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t("output-filename-prefix")}
            <input
              type="text"
              value={rec.filenamePrefix}
              onChange={(event) => save({ filenamePrefix: event.target.value })}
              className={inputClass}
            />
          </label>
        </div>

        {/* Templates commit on blur/Enter, NOT per keystroke: the backend
            validator rejects every intermediate state of typing a token
            ("{" is an unclosed brace), and a controlled per-keystroke save
            would snap the input back — making tokens untypeable. */}
        <div className="grid grid-cols-3 gap-2">
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t("output-recording-template")}
            <input
              type="text"
              defaultValue={rec.template}
              onBlur={(event) => {
                if (event.target.value !== rec.template) save({ template: event.target.value });
              }}
              onKeyDown={(event) => {
                if (event.key === "Enter") event.currentTarget.blur();
              }}
              className={inputClass}
            />
          </label>
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t("output-replay-template")}
            <input
              type="text"
              defaultValue={rec.replayTemplate}
              onBlur={(event) => {
                if (event.target.value !== rec.replayTemplate)
                  save({ replayTemplate: event.target.value });
              }}
              onKeyDown={(event) => {
                if (event.key === "Enter") event.currentTarget.blur();
              }}
              className={inputClass}
            />
          </label>
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t("output-still-template")}
            <input
              type="text"
              defaultValue={rec.stillTemplate}
              onBlur={(event) => {
                if (event.target.value !== rec.stillTemplate)
                  save({ stillTemplate: event.target.value });
              }}
              onKeyDown={(event) => {
                if (event.key === "Enter") event.currentTarget.blur();
              }}
              className={inputClass}
            />
          </label>
        </div>
        <p className="m-0 text-[11px] text-havoc-muted">{t("output-template-tokens")}</p>

        <div className="grid grid-cols-2 gap-2">
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t("output-replay-folder")}
            <input
              type="text"
              value={rec.replayFolder}
              placeholder={t("output-same-folder-placeholder")}
              onChange={(event) => save({ replayFolder: event.target.value })}
              className={inputClass}
            />
          </label>
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t("output-still-folder")}
            <input
              type="text"
              value={rec.stillFolder}
              placeholder={t("output-same-folder-placeholder")}
              onChange={(event) => save({ stillFolder: event.target.value })}
              className={inputClass}
            />
          </label>
        </div>

        <div className="grid grid-cols-3 items-end gap-2">
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t("output-frame-rate")}
            <select
              value={rec.fps}
              onChange={(event) => save({ fps: Number(event.target.value) })}
              className={selectClass}
            >
              {FPS_CHOICES.map((fps) => (
                <option key={fps} value={fps}>
                  {t("output-fps-option", { fps })}
                </option>
              ))}
            </select>
          </label>
          <NumberField
            label={t("output-split-every")}
            value={rec.splitMinutes}
            min={0}
            max={1440}
            onCommit={(value) => save({ splitMinutes: Math.round(value) })}
          />
          <NumberField
            label={t("output-output-width")}
            value={rec.outputWidth}
            min={0}
            max={16384}
            step={2}
            onCommit={(value) => save({ outputWidth: Math.round(value) })}
          />
          <NumberField
            label={t("output-output-height")}
            value={rec.outputHeight}
            min={0}
            max={16384}
            step={2}
            onCommit={(value) => save({ outputHeight: Math.round(value) })}
          />
          <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
            <input
              type="checkbox"
              checked={rec.recordVertical}
              onChange={(event) => save({ recordVertical: event.target.checked })}
            />
            {t("output-record-vertical")}
          </label>
          <div className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t("output-audio-tracks")}
            <div
              className="flex items-center gap-1"
              role="group"
              aria-label={t("output-recorded-tracks-group")}
            >
              {Array.from({ length: TRACK_COUNT }, (_, index) => {
                const on = (rec.tracksMask & (1 << index)) !== 0;
                const lastOne = on && rec.tracksMask === 1 << index;
                return (
                  <button
                    key={index}
                    type="button"
                    title={
                      lastOne
                        ? t("output-track-last-one")
                        : on
                          ? t("output-record-track-on", { index: index + 1 })
                          : t("output-record-track-off", { index: index + 1 })
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

        {wire && (
          <section className="flex flex-col gap-2 rounded-lg border border-white/10 bg-white/[0.03] p-2.5">
            <h4 className="m-0 text-[11px] font-semibold tracking-wider text-havoc-muted uppercase">
              {t("output-encoder-heading")}
            </h4>
            <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
              {t("output-video-encoder")}
              <select
                value={rec.encoderId}
                onChange={(event) => save({ encoderId: event.target.value })}
                className={selectClass}
              >
                <option value="auto">{t("output-encoder-auto")}</option>
                {(catalog?.encoders ?? [])
                  .filter((encoder) => containerAccepts(rec.container, encoder.codec))
                  .map((encoder) => (
                    <option
                      key={encoder.id}
                      value={encoder.id}
                      disabled={encoder.verified === false}
                    >
                      {encoder.label}
                      {encoder.verified === false ? ` ${t("output-encoder-unavailable")}` : ""}
                    </option>
                  ))}
              </select>
            </label>
            {(() => {
              const chosen = catalog?.encoders.find((encoder) => encoder.id === rec.encoderId);
              return chosen ? (
                <p className="m-0 text-[10px] leading-snug text-havoc-muted">{chosen.note}</p>
              ) : null;
            })()}
            <div className="grid grid-cols-3 gap-2">
              <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
                {t("output-preset")}
                <select
                  value={rec.preset}
                  onChange={(event) => save({ preset: event.target.value as EncPreset })}
                  className={selectClass}
                >
                  <option value="quality">{t("output-preset-quality")}</option>
                  <option value="balanced">{t("output-preset-balanced-option")}</option>
                  <option value="performance">{t("output-preset-performance")}</option>
                </select>
              </label>
              <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
                {t("output-rate-control")}
                <select
                  value={rec.rateControl.mode}
                  onChange={(event) =>
                    save({
                      rateControl: { ...rec.rateControl, mode: event.target.value as RcMode },
                    })
                  }
                  className={selectClass}
                >
                  <option value="cqp">{t("output-rc-cqp")}</option>
                  <option value="cbr">{t("output-rc-cbr")}</option>
                  <option value="vbr">{t("output-rc-vbr")}</option>
                </select>
              </label>
              {rec.rateControl.mode === "cqp" ? (
                <NumberField
                  label={t("output-cq")}
                  value={rec.rateControl.cq}
                  min={0}
                  max={51}
                  onCommit={(value) =>
                    save({ rateControl: { ...rec.rateControl, cq: Math.round(value) } })
                  }
                />
              ) : (
                <NumberField
                  label={t("output-bitrate")}
                  value={rec.rateControl.bitrateKbps}
                  min={100}
                  max={300000}
                  step={500}
                  onCommit={(value) =>
                    save({
                      rateControl: { ...rec.rateControl, bitrateKbps: Math.round(value) },
                    })
                  }
                />
              )}
            </div>
            <div className="grid grid-cols-2 gap-2">
              <NumberField
                label={t("output-keyframe")}
                value={rec.keyframeSec}
                min={0.25}
                max={10}
                step={0.25}
                onCommit={(value) => save({ keyframeSec: value })}
              />
              <NumberField
                label={t("output-audio-bitrate")}
                value={rec.audioBitrateKbps}
                min={32}
                max={512}
                step={32}
                onCommit={(value) => save({ audioBitrateKbps: Math.round(value) })}
              />
            </div>
          </section>
        )}

        <div className="flex items-center gap-1.5">
          <span className="text-[11px] text-havoc-muted">{t("output-presets")}</span>
          {RECORD_PRESETS.map((preset) => (
            <button
              key={preset.label}
              type="button"
              title={t(preset.title)}
              onClick={() => save(preset.patch)}
              className="rounded-md border border-white/10 bg-white/[0.04] px-2 py-1 text-[11px] text-havoc-muted transition-colors hover:border-havoc-accent/50 hover:text-havoc-text"
            >
              {t(preset.label)}
            </button>
          ))}
        </div>

        {saveError && (
          <p role="alert" className="m-0 text-[11px] text-red-300">
            {saveError}
          </p>
        )}
      </div>
    </PickerShell>
  );
}
