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
    <PickerShell title={`Properties — ${source.name}`} onClose={onClose} wide>
      <div className="flex flex-col gap-3">
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          Name
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
            Cancel
          </button>
          <button
            type="button"
            onClick={apply}
            className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
          >
            Apply
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
  switch (draft.kind) {
    case "chatOverlay":
      return (
        <div className="flex flex-col gap-2">
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            YouTube — channel / watch / live_chat URL (no key, no sign-in, ever)
            <input
              value={draft.youtube}
              onChange={(event) => onChange({ ...draft, youtube: event.target.value })}
              className={`${inputClass} font-mono`}
            />
          </label>
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            Twitch — channel name (anonymous)
            <input
              value={draft.twitch}
              onChange={(event) => onChange({ ...draft, twitch: event.target.value })}
              className={`${inputClass} font-mono`}
            />
          </label>
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            Kick — channel slug (public endpoint)
            <input
              value={draft.kick}
              onChange={(event) => onChange({ ...draft, kick: event.target.value })}
              className={`${inputClass} font-mono`}
            />
          </label>
          <div className="grid grid-cols-3 gap-2">
            <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
              Width (px)
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
              Lines
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
              Font (px)
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
            Image files (one path per line, shown in order)
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
              Per-slide (ms)
              <input
                type="number"
                min={100}
                value={draft.slideMs}
                onChange={(event) => onChange({ ...draft, slideMs: Number(event.target.value) })}
                className={inputClass}
              />
            </label>
            <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
              Crossfade (ms, 0 = cut)
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
            Loop (off = hold the last slide)
          </label>
          <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
            <input
              type="checkbox"
              checked={draft.shuffle}
              onChange={(event) => onChange({ ...draft, shuffle: event.target.checked })}
            />
            Shuffle each cycle
          </label>
        </div>
      );
    case "nestedScene":
      return (
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          Scene this source composes (a scene that already contains this one is rejected)
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
          The Wayland ScreenCast portal picks the screen or window in the <em>system</em> dialog
          every time this source starts — there is nothing to configure here, by design.
        </p>
      );
    case "videoDevice":
      return <VideoDeviceEditor draft={draft} onChange={onChange} />;
    case "audioInput":
    case "audioOutput":
      return <AudioDeviceEditor draft={draft} onChange={onChange} />;
    case "image":
      return (
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          Image file
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
            Media file (mp4, mkv, webm, mov, .frec, or an image)
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
            Loop (restart from the top at the end)
          </label>
          <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
            <input
              type="checkbox"
              checked={draft.hwDecode}
              onChange={(event) => onChange({ ...draft, hwDecode: event.target.checked })}
            />
            Hardware decode (falls back to software on its own)
          </label>
          <p className="m-0 text-[10px] leading-relaxed text-havoc-muted">
            .frec plays through the owned freally-video codec — nothing to download. Other video
            formats decode through the on-demand FFmpeg component. The file&apos;s audio gets its
            own mixer strip; the strip&apos;s sync offset fine-tunes A/V alignment. A clip with no
            audio leaves its strip silent.
          </p>
        </div>
      );
    case "color":
      return (
        <div className="flex items-end gap-2">
          <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
            Color
            <input
              type="color"
              value={rgbaToHex(draft.color)}
              onChange={(event) =>
                onChange({ ...draft, color: hexToRgba(event.target.value, draft.color.a) })
              }
              aria-label="Color"
              className="h-7 w-12 cursor-pointer rounded border border-white/10 bg-transparent"
            />
          </label>
          <NumberField
            label="Width"
            value={draft.width}
            min={1}
            max={16384}
            onCommit={(width) => onChange({ ...draft, width })}
            className="flex-1"
          />
          <NumberField
            label="Height"
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
        Capturing: <span className="text-havoc-text">{draft.label}</span>
      </span>
      <span className="text-[11px] text-havoc-muted">
        {entries === null
          ? "Looking for sources…"
          : entries.length === 0
            ? `No ${draft.kind === "display" ? "displays" : "windows"} found to re-pick.`
            : "Pick again:"}
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
        Device
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
            <option value={draft.deviceId}>(current device)</option>
          )}
        </select>
      </label>
      <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
        Format
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
            {formats === null ? "Auto (loading formats…)" : "Auto (highest resolution)"}
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
        {isLoopback ? "Capture the audio of" : "Device"}
        <select
          value={draft.deviceId}
          onChange={(event) => onChange({ ...draft, deviceId: event.target.value })}
          className={inputClass}
        >
          {offerDefault && (
            <option value="">
              {isLoopback ? "Default output (what you hear)" : "Default input"}
            </option>
          )}
          {(devices ?? []).map((device) => (
            <option key={device.id} value={device.id}>
              {device.name}
              {device.isDefault ? " (default)" : ""}
            </option>
          ))}
          {devices !== null &&
            draft.deviceId !== "" &&
            !devices.some((device) => device.id === draft.deviceId) && (
              <option value={draft.deviceId}>(current device: {draft.deviceId})</option>
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
  return (
    <div className="flex flex-col gap-2">
      <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
        Text
        <textarea
          value={draft.text}
          onChange={(event) => onChange({ ...draft, text: event.target.value })}
          rows={3}
          className={inputClass}
        />
      </label>
      <div className="flex gap-2">
        <label className="flex flex-1 flex-col gap-1 text-[11px] text-havoc-muted">
          Font family (system; blank = default)
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
          label="Size (px)"
          value={draft.sizePx}
          min={4}
          max={512}
          onCommit={(sizePx) => onChange({ ...draft, sizePx })}
          className="w-24"
        />
      </div>
      <div className="flex items-end gap-3">
        <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
          Color
          <input
            type="color"
            value={rgbaToHex(draft.color)}
            onChange={(event) =>
              onChange({ ...draft, color: hexToRgba(event.target.value, draft.color.a) })
            }
            aria-label="Text color"
            className="h-7 w-12 cursor-pointer rounded border border-white/10 bg-transparent"
          />
        </label>
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          Align
          <select
            value={draft.align}
            onChange={(event) => onChange({ ...draft, align: event.target.value as TextAlign })}
            className={inputClass}
          >
            <option value="left">left</option>
            <option value="center">center</option>
            <option value="right">right</option>
          </select>
        </label>
        <NumberField
          label="Line spacing"
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
          Wrap width (px; 0 = off)
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
          Force right-to-left
        </label>
      </div>
      <p className="m-0 text-[10px] leading-snug text-havoc-muted">
        Rendering uses real shaping (Arabic joining, ligatures) and bidi line ordering. The bundled
        Noto Sans family (incl. Arabic/Hebrew) is the default; system families work too. CJK uses
        system fonts for now.
      </p>
    </div>
  );
}
