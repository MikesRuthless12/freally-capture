import { useEffect, useState } from "react";

import {
  audioInputDevices,
  audioLoopbackDevices,
  cameraControlSet,
  cameraControlsList,
  cameraProfileReset,
  captureListSources,
  studioRenameSource,
  studioTimerControl,
  studioUpdateSourceSettings,
  videoDeviceFormats,
  videoDevicesList,
} from "../api/commands";
import type {
  AudioDevice,
  CameraControl,
  CaptureSource,
  CountdownEnd,
  DeinterlaceMode,
  FieldOrder,
  FileBinding,
  Source,
  SourceSettings,
  TextAlign,
  TimerMode,
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

        <SettingsEditor draft={draft} scenes={scenes} sourceId={source.id} onChange={setDraft} />

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
  sourceId,
  onChange,
}: {
  draft: SourceSettings;
  scenes: Array<{ id: string; name: string }>;
  sourceId: string;
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
    case "timer":
      return <TimerEditor draft={draft} scenes={scenes} sourceId={sourceId} onChange={onChange} />;
    case "testBars":
    case "testGrid":
    case "testSweep":
    case "testFlashBeep":
      return (
        <div className="flex items-end gap-2">
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
    case "testTone":
      return (
        <p className="m-0 text-xs leading-relaxed text-havoc-muted">
          {t("properties-testtone-note")}
        </p>
      );
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
      <div className="flex items-end gap-2">
        <label className="flex flex-1 flex-col gap-1 text-[11px] text-havoc-muted">
          {t("properties-deinterlace")}
          <select
            value={draft.deinterlace}
            onChange={(event) =>
              onChange({ ...draft, deinterlace: event.target.value as DeinterlaceMode })
            }
            className={inputClass}
          >
            <option value="off">{t("properties-deinterlace-off")}</option>
            <option value="discard">{t("properties-deinterlace-discard")}</option>
            <option value="bob">{t("properties-deinterlace-bob")}</option>
            <option value="linear">{t("properties-deinterlace-linear")}</option>
            <option value="blend">{t("properties-deinterlace-blend")}</option>
            <option value="motionAdaptive">{t("properties-deinterlace-adaptive")}</option>
          </select>
        </label>
        {draft.deinterlace !== "off" && (
          <label className="flex flex-1 flex-col gap-1 text-[11px] text-havoc-muted">
            {t("properties-field-order")}
            <select
              value={draft.fieldOrder}
              onChange={(event) =>
                onChange({ ...draft, fieldOrder: event.target.value as FieldOrder })
              }
              className={inputClass}
            >
              <option value="topFirst">{t("properties-field-order-top")}</option>
              <option value="bottomFirst">{t("properties-field-order-bottom")}</option>
            </select>
          </label>
        )}
      </div>
      {draft.deinterlace !== "off" && (
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">
          {t("properties-deinterlace-note")}
        </p>
      )}
      <CameraControlsSection deviceId={draft.deviceId} />
    </div>
  );
}

/** Localized labels for the known control tags; unknown ids show the
 * backend's own name. Values are i18n keys. */
const CAMERA_CONTROL_LABELS: Record<string, string> = {
  brightness: "camera-control-brightness",
  contrast: "camera-control-contrast",
  hue: "camera-control-hue",
  saturation: "camera-control-saturation",
  sharpness: "camera-control-sharpness",
  gamma: "camera-control-gamma",
  whiteBalance: "camera-control-white-balance",
  backlightComp: "camera-control-backlight",
  gain: "camera-control-gain",
  pan: "camera-control-pan",
  tilt: "camera-control-tilt",
  zoom: "camera-control-zoom",
  exposure: "camera-control-exposure",
  iris: "camera-control-iris",
  focus: "camera-control-focus",
};

/**
 * CAP-M18 — the running device's image controls. Values commit on release
 * (each commit also lands in the per-device profile, reapplied on
 * hotplug/restart). An empty list is honest: the device isn't streaming
 * yet, or this backend reports no controls (per-OS reality).
 */
function CameraControlsSection({ deviceId }: { deviceId: string }) {
  const t = useT();
  const [controls, setControls] = useState<CameraControl[] | null>(null);

  useEffect(() => {
    let alive = true;
    const fetchControls = () =>
      cameraControlsList(deviceId)
        .then((list) => {
          if (alive) setControls(list);
        })
        .catch(() => {
          if (alive) setControls([]);
        });
    fetchControls();
    // The device may still be warming up when the dialog opens.
    const retry = window.setTimeout(fetchControls, 1_500);
    return () => {
      alive = false;
      window.clearTimeout(retry);
    };
  }, [deviceId]);

  const refresh = () => {
    cameraControlsList(deviceId)
      .then(setControls)
      .catch(() => setControls([]));
  };

  const reset = () => {
    cameraProfileReset(deviceId)
      .then(() => window.setTimeout(refresh, 400))
      .catch((err) => console.error(err));
  };

  /** Commit one control, holding the committed value in view. Without this
   * the row would snap back to the stale fetched value on release (the
   * capture thread applies the write a frame later). */
  const commit = (control: CameraControl, value: number) => {
    setControls((current) =>
      (current ?? []).map((entry) => (entry.id === control.id ? { ...entry, value } : entry)),
    );
    cameraControlSet(deviceId, control.id, value).catch((err) => console.error(err));
  };

  return (
    <div className="flex flex-col gap-2 border-t border-white/5 pt-2">
      <div className="flex items-center justify-between">
        <p className="m-0 text-[11px] font-semibold uppercase tracking-wide text-havoc-muted">
          {t("camera-controls-title")}
        </p>
        <div className="flex gap-2">
          <button
            type="button"
            onClick={refresh}
            className="rounded border border-white/10 px-2 py-0.5 text-[10px] text-havoc-muted hover:text-havoc-text"
          >
            {t("camera-controls-refresh")}
          </button>
          <button
            type="button"
            disabled={!controls || controls.length === 0}
            onClick={reset}
            className="rounded border border-white/10 px-2 py-0.5 text-[10px] text-havoc-muted enabled:hover:text-havoc-text disabled:opacity-50"
          >
            {t("camera-controls-reset")}
          </button>
        </div>
      </div>
      {!controls || controls.length === 0 ? (
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">
          {t("camera-controls-empty")}
        </p>
      ) : (
        <>
          {controls.map((control) => (
            <CameraControlRow
              key={control.id}
              control={control}
              onCommit={(value) => commit(control, value)}
            />
          ))}
          <p className="m-0 text-[10px] leading-snug text-havoc-muted">
            {t("camera-controls-note")}
          </p>
        </>
      )}
    </div>
  );
}

function CameraControlRow({
  control,
  onCommit,
}: {
  control: CameraControl;
  onCommit: (value: number) => void;
}) {
  const t = useT();
  const [drag, setDrag] = useState<number | null>(null);
  const shown = drag ?? control.value;
  const label = CAMERA_CONTROL_LABELS[control.id]
    ? t(CAMERA_CONTROL_LABELS[control.id])
    : control.name;
  const commit = () => {
    if (drag !== null) {
      onCommit(drag);
      setDrag(null);
    }
  };
  // A control whose backend reports no range (Windows: exposure/focus/zoom)
  // gets a stepper — a slider would need bounds we honestly don't have.
  const ranged = control.min !== undefined && control.max !== undefined;
  return (
    <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
      <span className="w-24 shrink-0 truncate" title={control.name}>
        {label}
      </span>
      {ranged ? (
        <>
          <input
            type="range"
            min={control.min}
            max={control.max}
            step={control.step}
            value={shown}
            disabled={!control.writable}
            onChange={(event) => setDrag(Number(event.target.value))}
            onPointerUp={commit}
            onBlur={commit}
            aria-label={label}
            className="min-w-0 flex-1 accent-havoc-accent disabled:opacity-40"
          />
          <span className="w-14 shrink-0 text-right tabular-nums">{shown}</span>
        </>
      ) : (
        <input
          type="number"
          step={control.step}
          value={shown}
          disabled={!control.writable}
          onChange={(event) => setDrag(Number(event.target.value))}
          onBlur={commit}
          onKeyDown={(event) => {
            if (event.key === "Enter") commit();
          }}
          aria-label={label}
          className={`${inputClass} min-w-0 flex-1 disabled:opacity-40`}
        />
      )}
    </label>
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

/** The five CAP-M15 faces + the countdown end actions. Values are i18n keys. */
const TIMER_EDITOR_MODES: Array<[TimerMode, string]> = [
  ["wallClock", "sources-timer-wall-clock"],
  ["countdown", "sources-timer-countdown"],
  ["stopwatch", "sources-timer-stopwatch"],
  ["sinceLive", "sources-timer-since-live"],
  ["sinceRecording", "sources-timer-since-recording"],
];
const TIMER_END_ACTIONS: Array<[CountdownEnd, string]> = [
  ["none", "properties-timer-end-none"],
  ["flash", "properties-timer-end-flash"],
  ["switchScene", "properties-timer-end-switch"],
];

function TimerEditor({
  draft,
  scenes,
  sourceId,
  onChange,
}: {
  draft: Extract<SourceSettings, { kind: "timer" }>;
  scenes: Array<{ id: string; name: string }>;
  sourceId: string;
  onChange: (settings: SourceSettings) => void;
}) {
  const t = useT();
  // A wall-clock target runs by itself; Start/Pause/Reset drive the rest.
  const runControls =
    draft.mode === "stopwatch" || (draft.mode === "countdown" && draft.target.trim() === "");
  const control = (action: "start" | "pause" | "reset") => {
    studioTimerControl(sourceId, action).catch((err) => console.error(err));
  };
  return (
    <div className="flex flex-col gap-2">
      <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
        {t("sources-timer-mode-label")}
        <select
          value={draft.mode}
          onChange={(event) => onChange({ ...draft, mode: event.target.value as TimerMode })}
          className={inputClass}
        >
          {TIMER_EDITOR_MODES.map(([value, label]) => (
            <option key={value} value={value}>
              {t(label)}
            </option>
          ))}
        </select>
      </label>
      {draft.mode === "wallClock" && (
        <>
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t("properties-timer-format")}
            <input
              value={draft.format}
              onChange={(event) => onChange({ ...draft, format: event.target.value })}
              placeholder="%H:%M:%S"
              className={`${inputClass} font-mono`}
            />
          </label>
          <p className="m-0 text-[10px] leading-snug text-havoc-muted">
            {t("properties-timer-format-note")}
          </p>
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t("properties-timer-utc")}
            <input
              type="number"
              min={-840}
              max={840}
              value={draft.utcOffsetMin ?? ""}
              onChange={(event) =>
                onChange({
                  ...draft,
                  utcOffsetMin:
                    event.target.value === "" ? null : Math.round(Number(event.target.value)),
                })
              }
              placeholder={t("properties-timer-utc-placeholder")}
              className={inputClass}
            />
          </label>
        </>
      )}
      {draft.mode === "countdown" && (
        <>
          <div className="flex items-end gap-2">
            <NumberField
              label={t("properties-timer-duration")}
              value={Math.round(draft.countdownMs / 1000)}
              min={1}
              max={86_400}
              onCommit={(seconds) =>
                onChange({ ...draft, countdownMs: Math.round(seconds) * 1000 })
              }
              className="flex-1"
            />
            <label className="flex flex-1 flex-col gap-1 text-[11px] text-havoc-muted">
              {t("properties-timer-target")}
              <input
                value={draft.target}
                onChange={(event) => onChange({ ...draft, target: event.target.value })}
                placeholder="19:30"
                className={`${inputClass} font-mono`}
              />
            </label>
          </div>
          <p className="m-0 text-[10px] leading-snug text-havoc-muted">
            {t("properties-timer-target-note")}
          </p>
          <div className="flex items-end gap-2">
            <label className="flex flex-1 flex-col gap-1 text-[11px] text-havoc-muted">
              {t("properties-timer-end")}
              <select
                value={draft.endAction}
                onChange={(event) =>
                  onChange({ ...draft, endAction: event.target.value as CountdownEnd })
                }
                className={inputClass}
              >
                {TIMER_END_ACTIONS.map(([value, label]) => (
                  <option key={value} value={value}>
                    {t(label)}
                  </option>
                ))}
              </select>
            </label>
            {draft.endAction === "switchScene" && (
              <label className="flex flex-1 flex-col gap-1 text-[11px] text-havoc-muted">
                {t("properties-timer-end-scene")}
                <select
                  value={draft.endScene ?? ""}
                  onChange={(event) => onChange({ ...draft, endScene: event.target.value || null })}
                  className={inputClass}
                >
                  <option value="">—</option>
                  {scenes.map((scene) => (
                    <option key={scene.id} value={scene.id}>
                      {scene.name}
                    </option>
                  ))}
                </select>
              </label>
            )}
          </div>
        </>
      )}
      <div className="flex items-end gap-2">
        <NumberField
          label={t("properties-timer-size")}
          value={draft.sizePx}
          min={4}
          max={512}
          onCommit={(sizePx) => onChange({ ...draft, sizePx })}
          className="flex-1"
        />
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
      </div>
      {runControls && (
        <div className="flex gap-2">
          {(
            [
              ["start", "properties-timer-start"],
              ["pause", "properties-timer-pause"],
              ["reset", "properties-timer-reset"],
            ] as const
          ).map(([action, label]) => (
            <button
              key={action}
              type="button"
              onClick={() => control(action)}
              className="rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-muted hover:text-havoc-text"
            >
              {t(label)}
            </button>
          ))}
        </div>
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
      <div className="flex flex-col gap-2 border-t border-white/5 pt-2">
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("properties-text-file")}
          <input
            value={draft.sourceFile}
            onChange={(event) => onChange({ ...draft, sourceFile: event.target.value })}
            placeholder="C:\\data\\score.csv"
            className={`${inputClass} font-mono`}
          />
        </label>
        {draft.sourceFile.trim() !== "" && (
          <>
            <div className="flex items-end gap-2">
              <label className="flex flex-1 flex-col gap-1 text-[11px] text-havoc-muted">
                {t("properties-text-binding")}
                <select
                  value={draft.binding}
                  onChange={(event) =>
                    onChange({ ...draft, binding: event.target.value as FileBinding })
                  }
                  className={inputClass}
                >
                  <option value="whole">{t("properties-text-binding-whole")}</option>
                  <option value="csvCell">{t("properties-text-binding-csv")}</option>
                  <option value="jsonPointer">{t("properties-text-binding-json")}</option>
                </select>
              </label>
              {draft.binding === "csvCell" && (
                <>
                  <NumberField
                    label={t("properties-text-csv-row")}
                    value={draft.csvRow}
                    min={1}
                    max={100000}
                    onCommit={(csvRow) => onChange({ ...draft, csvRow: Math.round(csvRow) })}
                    className="w-24"
                  />
                  <label className="flex flex-1 flex-col gap-1 text-[11px] text-havoc-muted">
                    {t("properties-text-csv-column")}
                    <input
                      value={draft.csvColumn}
                      onChange={(event) => onChange({ ...draft, csvColumn: event.target.value })}
                      placeholder={t("properties-text-csv-column-placeholder")}
                      className={`${inputClass} font-mono`}
                    />
                  </label>
                </>
              )}
              {draft.binding === "jsonPointer" && (
                <label className="flex flex-1 flex-col gap-1 text-[11px] text-havoc-muted">
                  {t("properties-text-json-pointer")}
                  <input
                    value={draft.jsonPointer}
                    onChange={(event) => onChange({ ...draft, jsonPointer: event.target.value })}
                    placeholder="/teams/0/score"
                    className={`${inputClass} font-mono`}
                  />
                </label>
              )}
            </div>
            <p className="m-0 text-[10px] leading-snug text-havoc-muted">
              {t("properties-text-file-note")}
            </p>
          </>
        )}
      </div>
    </div>
  );
}
