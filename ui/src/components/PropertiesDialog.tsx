import { useEffect, useState } from "react";

import {
  audioInputDevices,
  audioLoopbackDevices,
  captureListSources,
  studioRenameSource,
  studioUpdateSourceSettings,
  videoDeviceFormats,
  videoDevicesList,
} from "../api/commands";
import type {
  AudioDevice,
  CaptureSource,
  Source,
  SourceSettings,
  TextAlign,
  VideoDevice,
  VideoFormat,
} from "../api/types";
import { hexToRgba, rgbaToHex } from "../lib/color";
import { useT } from "../i18n/t";
import { NumberField } from "./NumberField";
import { PickerShell } from "./PickerShell";

const inputClass =
  "rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text outline-none focus:border-havoc-accent/60";

type PropertiesDialogProps = {
  source: Source;
  /** The scenes a Nested Scene source can point at (cycle-checked on Apply). */
  scenes?: Array<{ id: string; name: string }>;
  onClose: () => void;
};

/** Per-kind source settings + rename. Apply pushes to the engine live. */
export function PropertiesDialog({ source, scenes = [], onClose }: PropertiesDialogProps) {
  const t = useT();
  const [name, setName] = useState(source.name);
  const [draft, setDraft] = useState<SourceSettings>(() => {
    // A Source is its settings plus identity (+ the audio strip) — peel
    // everything that isn't per-kind settings off.
    const settings: Partial<Source> = { ...source };
    delete settings.id;
    delete settings.name;
    delete settings.audio;
    return settings as SourceSettings;
  });
  const [error, setError] = useState<string | null>(null);

  const apply = () => {
    setError(null);
    const jobs: Promise<unknown>[] = [studioUpdateSourceSettings(source.id, draft)];
    if (name.trim() && name.trim() !== source.name) {
      jobs.push(studioRenameSource(source.id, name.trim()));
    }
    Promise.all(jobs)
      .then(() => onClose())
      .catch((err) => setError(String(err)));
  };

  return (
    <PickerShell title={t("properties-title", { name: source.name })} onClose={onClose} wide>
      <div className="flex flex-col gap-3">
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("properties-name")}
          <input
            value={name}
            onChange={(event) => setName(event.target.value)}
            className={inputClass}
          />
        </label>

        <SettingsEditor draft={draft} scenes={scenes} onChange={setDraft} />

        {error && (
          <p role="alert" className="m-0 text-xs text-red-400">
            {error}
          </p>
        )}
        <div className="flex justify-end gap-2">
          <button
            type="button"
            onClick={onClose}
            className="rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-muted hover:text-havoc-text"
          >
            {t("properties-cancel")}
          </button>
          <button
            type="button"
            onClick={apply}
            className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
          >
            {t("properties-apply")}
          </button>
        </div>
      </div>
    </PickerShell>
  );
}

function SettingsEditor({
  draft,
  scenes,
  onChange,
}: {
  draft: SourceSettings;
  scenes: Array<{ id: string; name: string }>;
  onChange: (settings: SourceSettings) => void;
}) {
  const t = useT();
  switch (draft.kind) {
    case "chatOverlay":
      return (
        <div className="flex flex-col gap-2">
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t("properties-youtube")}
            <input
              value={draft.youtube}
              onChange={(event) => onChange({ ...draft, youtube: event.target.value })}
              className={`${inputClass} font-mono`}
            />
          </label>
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t("properties-twitch")}
            <input
              value={draft.twitch}
              onChange={(event) => onChange({ ...draft, twitch: event.target.value })}
              className={`${inputClass} font-mono`}
            />
          </label>
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t("properties-kick")}
            <input
              value={draft.kick}
              onChange={(event) => onChange({ ...draft, kick: event.target.value })}
              className={`${inputClass} font-mono`}
            />
          </label>
          <div className="grid grid-cols-3 gap-2">
            <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
              {t("properties-width-px")}
              <input
                type="number"
                min={120}
                max={3840}
                value={draft.width}
                onChange={(event) => onChange({ ...draft, width: Number(event.target.value) })}
                className={inputClass}
              />
            </label>
            <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
              {t("properties-lines")}
              <input
                type="number"
                min={1}
                max={50}
                value={draft.maxLines}
                onChange={(event) => onChange({ ...draft, maxLines: Number(event.target.value) })}
                className={inputClass}
              />
            </label>
            <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
              {t("properties-font-px")}
              <input
                type="number"
                min={10}
                max={96}
                value={draft.fontSize}
                onChange={(event) => onChange({ ...draft, fontSize: Number(event.target.value) })}
                className={inputClass}
              />
            </label>
          </div>
        </div>
      );
    case "slideshow":
      return (
        <div className="flex flex-col gap-2">
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t("properties-images")}
            <textarea
              value={draft.paths.join("\n")}
              onChange={(event) =>
                onChange({
                  ...draft,
                  paths: event.target.value
                    .split("\n")
                    .map((line) => line.trim())
                    .filter(Boolean),
                })
              }
              rows={5}
              className={`${inputClass} font-mono`}
            />
          </label>
          <div className="grid grid-cols-2 gap-2">
            <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
              {t("properties-per-slide")}
              <input
                type="number"
                min={100}
                value={draft.slideMs}
                onChange={(event) => onChange({ ...draft, slideMs: Number(event.target.value) })}
                className={inputClass}
              />
            </label>
            <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
              {t("properties-crossfade")}
              <input
                type="number"
                min={0}
                max={5000}
                value={draft.transitionMs}
                onChange={(event) =>
                  onChange({ ...draft, transitionMs: Number(event.target.value) })
                }
                className={inputClass}
              />
            </label>
          </div>
          <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
            <input
              type="checkbox"
              checked={draft.loop}
              onChange={(event) => onChange({ ...draft, loop: event.target.checked })}
            />
            {t("properties-loop-slideshow")}
          </label>
          <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
            <input
              type="checkbox"
              checked={draft.shuffle}
              onChange={(event) => onChange({ ...draft, shuffle: event.target.checked })}
            />
            {t("properties-shuffle")}
          </label>
        </div>
      );
    case "nestedScene":
      return (
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("properties-nested-scene")}
          <select
            value={draft.scene}
            onChange={(event) => onChange({ ...draft, scene: event.target.value })}
            className={inputClass}
          >
            {scenes.map((entry) => (
              <option key={entry.id} value={entry.id}>
                {entry.name}
              </option>
            ))}
          </select>
        </label>
      );
    case "display":
    case "window":
      return <CaptureRepick draft={draft} onChange={onChange} />;
    case "portal":
      return (
        <p className="m-0 text-xs leading-relaxed text-havoc-muted">
          {t("properties-portal-note")}
        </p>
      );
    case "videoDevice":
      return <VideoDeviceEditor draft={draft} onChange={onChange} />;
    case "audioInput":
    case "audioOutput":
      return <AudioDeviceEditor draft={draft} onChange={onChange} />;
    case "appAudio":
      return (
        <p className="m-0 text-xs leading-relaxed text-havoc-muted">
          {t("properties-appaudio-capturing", {
            exe: draft.exe || t("properties-appaudio-exe-fallback"),
          })}
          {draft.pid ? ` ${t("properties-appaudio-pid", { pid: draft.pid })}` : ""}.{" "}
          {t("properties-appaudio-note")}
        </p>
      );
    case "image":
      return (
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("properties-image-file")}
          <input
            value={draft.path}
            onChange={(event) => onChange({ ...draft, path: event.target.value })}
            placeholder="C:\art\overlay.png"
            className={inputClass}
          />
        </label>
      );
    case "media":
      return (
        <div className="flex flex-col gap-2">
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t("properties-media-file")}
            <input
              value={draft.path}
              onChange={(event) => onChange({ ...draft, path: event.target.value })}
              placeholder="C:\clips\intro.mp4"
              className={inputClass}
            />
          </label>
          <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
            <input
              type="checkbox"
              checked={draft.loop}
              onChange={(event) => onChange({ ...draft, loop: event.target.checked })}
            />
            {t("properties-media-loop")}
          </label>
          <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
            <input
              type="checkbox"
              checked={draft.hwDecode}
              onChange={(event) => onChange({ ...draft, hwDecode: event.target.checked })}
            />
            {t("properties-media-hwdecode")}
          </label>
          <p className="m-0 text-[10px] leading-relaxed text-havoc-muted">
            {t("properties-media-note")}
          </p>
        </div>
      );
    case "color":
      return (
        <div className="flex items-end gap-2">
          <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
            {t("properties-color")}
            <input
              type="color"
              value={rgbaToHex(draft.color)}
              onChange={(event) =>
                onChange({ ...draft, color: hexToRgba(event.target.value, draft.color.a) })
              }
              aria-label={t("properties-color")}
              className="h-7 w-12 cursor-pointer rounded border border-white/10 bg-transparent"
            />
          </label>
          <NumberField
            label={t("properties-width")}
            value={draft.width}
            min={1}
            max={16384}
            onCommit={(width) => onChange({ ...draft, width })}
            className="flex-1"
          />
          <NumberField
            label={t("properties-height")}
            value={draft.height}
            min={1}
            max={16384}
            onCommit={(height) => onChange({ ...draft, height })}
            className="flex-1"
          />
        </div>
      );
    case "text":
      return <TextEditor draft={draft} onChange={onChange} />;
  }
}

function CaptureRepick({
  draft,
  onChange,
}: {
  draft: Extract<SourceSettings, { kind: "display" | "window" }>;
  onChange: (settings: SourceSettings) => void;
}) {
  const t = useT();
  const [entries, setEntries] = useState<CaptureSource[] | null>(null);

  useEffect(() => {
    let cancelled = false;
    captureListSources()
      .then((all) => {
        if (!cancelled) setEntries(all.filter((s) => s.kind === draft.kind));
      })
      .catch(() => {
        if (!cancelled) setEntries([]);
      });
    return () => {
      cancelled = true;
    };
  }, [draft.kind]);

  return (
    <div className="flex flex-col gap-1.5">
      <span className="text-[11px] text-havoc-muted">
        {t("properties-repick-capturing", { label: draft.label })}
      </span>
      <span className="text-[11px] text-havoc-muted">
        {entries === null
          ? t("properties-repick-looking")
          : entries.length === 0
            ? draft.kind === "display"
              ? t("properties-repick-none-displays")
              : t("properties-repick-none-windows")
            : t("properties-repick-again")}
      </span>
      {entries && entries.length > 0 && (
        <ul className="m-0 flex max-h-48 list-none flex-col gap-1 overflow-auto p-0">
          {entries.map((entry) => (
            <li key={entry.id}>
              <button
                type="button"
                onClick={() =>
                  onChange({ kind: draft.kind, captureId: entry.id, label: entry.label })
                }
                aria-pressed={entry.id === draft.captureId}
                className={`w-full truncate rounded-md border px-2 py-1.5 text-left text-xs text-havoc-text ${
                  entry.id === draft.captureId
                    ? "border-havoc-accent/60 bg-havoc-accent/10"
                    : "border-white/10 hover:border-havoc-accent/50"
                }`}
              >
                {entry.label}
              </button>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}

function VideoDeviceEditor({
  draft,
  onChange,
}: {
  draft: Extract<SourceSettings, { kind: "videoDevice" }>;
  onChange: (settings: SourceSettings) => void;
}) {
  const t = useT();
  const [devices, setDevices] = useState<VideoDevice[] | null>(null);
  // Keyed by device id so switching devices reads as "loading" without a
  // synchronous reset inside the fetch effect.
  const [formatsFor, setFormatsFor] = useState<{ deviceId: string; list: VideoFormat[] } | null>(
    null,
  );

  useEffect(() => {
    let cancelled = false;
    videoDevicesList()
      .then((list) => {
        if (!cancelled) setDevices(list);
      })
      .catch(() => {
        if (!cancelled) setDevices([]);
      });
    return () => {
      cancelled = true;
    };
  }, []);

  useEffect(() => {
    let cancelled = false;
    const deviceId = draft.deviceId;
    videoDeviceFormats(deviceId)
      .then((list) => {
        if (!cancelled) setFormatsFor({ deviceId, list });
      })
      .catch(() => {
        // A device that's live right now can't be probed — Auto still works.
        if (!cancelled) setFormatsFor({ deviceId, list: [] });
      });
    return () => {
      cancelled = true;
    };
  }, [draft.deviceId]);

  const formats = formatsFor?.deviceId === draft.deviceId ? formatsFor.list : null;

  const formatKey = (format: VideoFormat) =>
    `${format.width}x${format.height}@${format.fps}-${format.fourcc}`;
  const current = draft.format ? formatKey(draft.format) : "";

  return (
    <div className="flex flex-col gap-2">
      <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
        {t("properties-device")}
        <select
          value={draft.deviceId}
          onChange={(event) => onChange({ ...draft, deviceId: event.target.value, format: null })}
          className={inputClass}
        >
          {(devices ?? []).map((device) => (
            <option key={device.id} value={device.id}>
              {device.name}
            </option>
          ))}
          {devices !== null && !devices.some((device) => device.id === draft.deviceId) && (
            <option value={draft.deviceId}>{t("properties-video-current-device")}</option>
          )}
        </select>
      </label>
      <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
        {t("properties-format")}
        <select
          value={current}
          onChange={(event) => {
            const picked = (formats ?? []).find(
              (format) => formatKey(format) === event.target.value,
            );
            onChange({ ...draft, format: picked ?? null });
          }}
          className={inputClass}
        >
          <option value="">
            {formats === null ? t("properties-format-auto-loading") : t("properties-format-auto")}
          </option>
          {(formats ?? []).map((format) => (
            <option key={formatKey(format)} value={formatKey(format)}>
              {format.width}×{format.height} @ {format.fps} fps ({format.fourcc})
            </option>
          ))}
        </select>
      </label>
    </div>
  );
}

function AudioDeviceEditor({
  draft,
  onChange,
}: {
  draft: Extract<SourceSettings, { kind: "audioInput" | "audioOutput" }>;
  onChange: (settings: SourceSettings) => void;
}) {
  const t = useT();
  const [devices, setDevices] = useState<AudioDevice[] | null>(null);
  const [guidance, setGuidance] = useState<string | null>(null);
  const isLoopback = draft.kind === "audioOutput";

  useEffect(() => {
    let cancelled = false;
    if (isLoopback) {
      audioLoopbackDevices()
        .then((result) => {
          if (cancelled) return;
          setDevices(result.devices);
          setGuidance(result.guidance ?? null);
        })
        .catch(() => {
          if (!cancelled) setDevices([]);
        });
    } else {
      audioInputDevices()
        .then((list) => {
          if (!cancelled) setDevices(list);
        })
        .catch(() => {
          if (!cancelled) setDevices([]);
        });
    }
    return () => {
      cancelled = true;
    };
  }, [isLoopback]);

  // Windows loopback (no guidance) has a real "default output" fallback;
  // elsewhere an explicit device pick is the honest requirement.
  const offerDefault = !isLoopback || guidance === null;

  return (
    <div className="flex flex-col gap-2">
      <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
        {isLoopback ? t("properties-audio-capture-of") : t("properties-device")}
        <select
          value={draft.deviceId}
          onChange={(event) => onChange({ ...draft, deviceId: event.target.value })}
          className={inputClass}
        >
          {offerDefault && (
            <option value="">
              {isLoopback
                ? t("properties-audio-default-output")
                : t("properties-audio-default-input")}
            </option>
          )}
          {(devices ?? []).map((device) => (
            <option key={device.id} value={device.id}>
              {device.name}
              {device.isDefault ? ` ${t("properties-audio-default-suffix")}` : ""}
            </option>
          ))}
          {devices !== null &&
            draft.deviceId !== "" &&
            !devices.some((device) => device.id === draft.deviceId) && (
              <option value={draft.deviceId}>
                {t("properties-audio-current-device", { id: draft.deviceId })}
              </option>
            )}
        </select>
      </label>
      {guidance && (
        <p className="m-0 rounded-md border border-amber-400/20 bg-amber-400/5 p-2 text-[10px] leading-snug text-amber-200/90">
          {guidance}
        </p>
      )}
    </div>
  );
}

function TextEditor({
  draft,
  onChange,
}: {
  draft: Extract<SourceSettings, { kind: "text" }>;
  onChange: (settings: SourceSettings) => void;
}) {
  const t = useT();
  return (
    <div className="flex flex-col gap-2">
      <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
        {t("properties-text")}
        <textarea
          value={draft.text}
          onChange={(event) => onChange({ ...draft, text: event.target.value })}
          rows={3}
          className={inputClass}
        />
      </label>
      <div className="flex gap-2">
        <label className="flex flex-1 flex-col gap-1 text-[11px] text-havoc-muted">
          {t("properties-font-family")}
          <input
            value={draft.fontFamily ?? ""}
            onChange={(event) =>
              onChange({ ...draft, fontFamily: event.target.value.trim() || null })
            }
            placeholder="Segoe UI"
            className={inputClass}
          />
        </label>
        <NumberField
          label={t("properties-size-px")}
          value={draft.sizePx}
          min={4}
          max={512}
          onCommit={(sizePx) => onChange({ ...draft, sizePx })}
          className="w-24"
        />
      </div>
      <div className="flex items-end gap-3">
        <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
          {t("properties-color")}
          <input
            type="color"
            value={rgbaToHex(draft.color)}
            onChange={(event) =>
              onChange({ ...draft, color: hexToRgba(event.target.value, draft.color.a) })
            }
            aria-label={t("properties-text-color")}
            className="h-7 w-12 cursor-pointer rounded border border-white/10 bg-transparent"
          />
        </label>
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("properties-align")}
          <select
            value={draft.align}
            onChange={(event) => onChange({ ...draft, align: event.target.value as TextAlign })}
            className={inputClass}
          >
            <option value="left">{t("properties-align-left")}</option>
            <option value="center">{t("properties-align-center")}</option>
            <option value="right">{t("properties-align-right")}</option>
          </select>
        </label>
        <NumberField
          label={t("properties-line-spacing")}
          value={draft.lineSpacing}
          min={0.25}
          max={4}
          step={0.05}
          onCommit={(lineSpacing) => onChange({ ...draft, lineSpacing })}
          className="w-24"
        />
      </div>
      <div className="flex items-center gap-4">
        <label className="flex w-40 flex-col gap-1 text-[11px] text-havoc-muted">
          {t("properties-wrap-width")}
          <input
            type="number"
            min={0}
            value={draft.wrapWidth ?? 0}
            onChange={(event) => {
              const value = Number(event.target.value) || 0;
              onChange({ ...draft, wrapWidth: value > 0 ? value : null });
            }}
            className={inputClass}
          />
        </label>
        <label className="flex items-center gap-1.5 text-[11px] text-havoc-muted">
          <input
            type="checkbox"
            checked={draft.forceRtl}
            onChange={(event) => onChange({ ...draft, forceRtl: event.target.checked })}
          />
          {t("properties-force-rtl")}
        </label>
      </div>
      <p className="m-0 text-[10px] leading-snug text-havoc-muted">{t("properties-text-note")}</p>
    </div>
  );
}
