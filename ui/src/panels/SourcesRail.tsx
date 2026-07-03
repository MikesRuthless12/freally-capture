import { useEffect, useRef, useState } from "react";

import {
  audioInputDevices,
  audioLoopbackDevices,
  captureListSources,
  openPrivacySettings,
  studioRenameSource,
  studioRetrySource,
  videoDeviceFormats,
  videoDevicesList,
} from "../api/commands";
import type {
  AudioDevice,
  AudioLevelsPayload,
  CaptureSource,
  Collection,
  ItemId,
  ProgramStatus,
  Scene,
  SourceId,
  SourceSettings,
  VideoDevice,
  VideoFormat,
} from "../api/types";
import { kindHasAudio } from "../api/types";
import { EmptyHint, Panel } from "../components/Panel";
import { NumberField } from "../components/NumberField";
import { PickerShell } from "../components/PickerShell";
import { hexToRgba } from "../lib/color";

type SourcesRailProps = {
  collection: Collection | null;
  scene: Scene | null;
  program: ProgramStatus | null;
  audio: AudioLevelsPayload | null;
  os?: string;
  selectedItem: ItemId | null;
  onSelect: (item: ItemId | null) => void;
  onAdd: (settings: SourceSettings, name?: string) => void;
  onAddExisting: (source: SourceId) => void;
  onRemove: (item: ItemId) => void;
  onMove: (item: ItemId, toIndex: number) => void;
  onSetVisible: (item: ItemId, visible: boolean) => void;
  onSetLocked: (item: ItemId, locked: boolean) => void;
  onOpenFilters: (item: ItemId) => void;
  onOpenProperties: (source: SourceId) => void;
};

type PickerMode =
  | "display"
  | "window"
  | "webcam"
  | "image"
  | "media"
  | "color"
  | "text"
  | "audioInput"
  | "audioOutput"
  | "existing";

const KIND_BADGE: Record<string, string> = {
  display: "Display",
  window: "Window",
  portal: "Portal",
  videoDevice: "Camera",
  image: "Image",
  media: "Media",
  color: "Color",
  text: "Text",
  audioInput: "Audio In",
  audioOutput: "Audio Out",
};

const ADD_MENU: Array<[PickerMode, string]> = [
  ["display", "Display Capture"],
  ["window", "Window Capture"],
  ["webcam", "Video Capture Device"],
  ["image", "Image"],
  ["media", "Media (video/image file)"],
  ["color", "Color"],
  ["text", "Text"],
  ["audioInput", "Audio Input Capture"],
  ["audioOutput", "Audio Output Capture"],
  ["existing", "Existing source…"],
];

/**
 * The Sources rail: the active scene's items, top of the stack first.
 * Model order is bottom-first (index = z), so the list renders reversed.
 */
export function SourcesRail({
  collection,
  scene,
  program,
  audio,
  os,
  selectedItem,
  onSelect,
  onAdd,
  onAddExisting,
  onRemove,
  onMove,
  onSetVisible,
  onSetLocked,
  onOpenFilters,
  onOpenProperties,
}: SourcesRailProps) {
  const [menuOpen, setMenuOpen] = useState(false);
  const [picker, setPicker] = useState<PickerMode | null>(null);
  const [renaming, setRenaming] = useState<{ source: SourceId; draft: string } | null>(null);

  const items = scene?.items ?? [];
  const topFirst = [...items].reverse();
  const sourceOf = (id: SourceId) => collection?.sources.find((source) => source.id === id);

  const commitRename = () => {
    if (!renaming) return;
    const { source, draft } = renaming;
    setRenaming(null);
    if (draft.trim()) {
      studioRenameSource(source, draft.trim()).catch((err) =>
        console.error("source rename failed:", err),
      );
    }
  };

  const openPicker = (mode: PickerMode) => {
    setMenuOpen(false);
    setPicker(mode);
  };

  const pick = (settings: SourceSettings, name?: string) => {
    setPicker(null);
    onAdd(settings, name);
  };

  return (
    <Panel
      title="Sources"
      actions={
        <div className="relative">
          <button
            type="button"
            disabled={!scene}
            onClick={() => setMenuOpen((open) => !open)}
            title="Add a source"
            aria-label="Add a source"
            aria-haspopup="menu"
            aria-expanded={menuOpen}
            className="rounded-md border border-white/10 px-2 py-0.5 text-xs text-havoc-muted transition-colors enabled:hover:border-havoc-accent/50 enabled:hover:text-havoc-text disabled:opacity-60"
          >
            +
          </button>
          {menuOpen && (
            <div
              role="menu"
              aria-label="Add a source"
              className="absolute right-0 z-20 mt-1 w-48 rounded-lg border border-white/10 bg-havoc-panel p-1 shadow-xl"
            >
              {ADD_MENU.map(([mode, label]) => (
                <button
                  key={mode}
                  type="button"
                  role="menuitem"
                  onClick={() => openPicker(mode)}
                  className="block w-full rounded-md px-2 py-1.5 text-left text-xs text-havoc-text hover:bg-white/5"
                >
                  {label}
                </button>
              ))}
              <p className="m-0 border-t border-white/5 px-2 py-1.5 text-[10px] leading-snug text-havoc-muted">
                Media (video files) arrives with the recording engine; Browser needs its own
                offscreen-webview work.
              </p>
            </div>
          )}
        </div>
      }
    >
      {topFirst.length === 0 ? (
        <EmptyHint>
          No sources in this scene — add a Display Capture, Window, Webcam, Image, Color, or Text
          with “+”. Drag, scale, and rotate them on the canvas; right side buttons reorder the
          stack.
        </EmptyHint>
      ) : (
        <ul className="m-0 flex list-none flex-col gap-1.5 p-0">
          {topFirst.map((item) => {
            const modelIndex = items.findIndex((candidate) => candidate.id === item.id);
            const source = sourceOf(item.source);
            // Audio sources report through the `audio` event; video through
            // `program`. Same status shape, one dot.
            const status =
              source && kindHasAudio(source.kind)
                ? audio?.sources[item.source]
                : program?.sources[item.source];
            const isSelected = item.id === selectedItem;
            const isRenaming = renaming?.source === item.source;
            return (
              <li key={item.id}>
                <div
                  className={`group flex items-center gap-1 rounded-lg border px-1.5 py-1.5 ${
                    isSelected
                      ? "border-havoc-accent/50 bg-havoc-accent/10"
                      : "border-white/10 bg-white/[0.02]"
                  }`}
                >
                  <button
                    type="button"
                    onClick={() => onSetVisible(item.id, !item.visible)}
                    title={item.visible ? "Hide" : "Show"}
                    aria-label={`${item.visible ? "Hide" : "Show"} ${source?.name ?? "source"}`}
                    aria-pressed={item.visible}
                    className={`shrink-0 rounded px-1 text-xs ${
                      item.visible ? "text-havoc-text" : "text-havoc-muted opacity-50"
                    }`}
                  >
                    {item.visible ? "👁" : "–"}
                  </button>
                  {isRenaming ? (
                    <input
                      autoFocus
                      value={renaming.draft}
                      onChange={(event) =>
                        setRenaming({ source: item.source, draft: event.target.value })
                      }
                      onBlur={commitRename}
                      onKeyDown={(event) => {
                        if (event.key === "Enter") commitRename();
                        if (event.key === "Escape") setRenaming(null);
                      }}
                      aria-label={`Rename ${source?.name ?? "source"}`}
                      className="min-w-0 flex-1 rounded border border-havoc-accent/50 bg-transparent px-1 text-xs text-havoc-text outline-none"
                    />
                  ) : (
                    <button
                      type="button"
                      onClick={() => onSelect(item.id)}
                      onDoubleClick={() =>
                        source && setRenaming({ source: source.id, draft: source.name })
                      }
                      title={source?.name}
                      className="flex min-w-0 flex-1 items-center gap-1.5 truncate text-left text-xs text-havoc-text"
                    >
                      <span className="rounded bg-white/10 px-1 py-px text-[9px] text-havoc-muted uppercase">
                        {KIND_BADGE[source?.kind ?? ""] ?? "?"}
                      </span>
                      <span className="truncate">{source?.name ?? "(missing source)"}</span>
                    </button>
                  )}
                  {status && status.state === "error" ? (
                    <span className="flex shrink-0 items-center gap-1">
                      <button
                        type="button"
                        onClick={() =>
                          studioRetrySource(item.source).catch((err) =>
                            console.error("source retry failed:", err),
                          )
                        }
                        title={`Retry — ${status.errorMessage ?? "error"}`}
                        aria-label={`Retry ${source?.name ?? "source"}`}
                        className="flex items-center gap-1 rounded px-1 text-[10px] text-red-400 hover:text-red-300"
                      >
                        <span
                          aria-label="status: error"
                          className="h-1.5 w-1.5 rounded-full bg-red-400"
                        />
                        ↻
                      </button>
                      {status.errorCode === "permission" && os === "macos" && (
                        <button
                          type="button"
                          onClick={() =>
                            void openPrivacySettings(
                              source?.kind === "videoDevice" ? "camera" : "screenRecording",
                            )
                          }
                          title="Open the macOS privacy settings for this permission"
                          aria-label={`Open privacy settings for ${source?.name ?? "source"}`}
                          className="rounded border border-red-400/40 px-1 text-[9px] text-red-300 hover:border-red-300"
                        >
                          settings
                        </button>
                      )}
                    </span>
                  ) : status ? (
                    <span
                      title={
                        status.state !== "live"
                          ? "starting…"
                          : "width" in status && status.width
                            ? `${status.width}×${status.height}${status.fps ? ` @ ${status.fps}` : ""}`
                            : "live"
                      }
                      aria-label={`status: ${status.state}`}
                      className={`h-1.5 w-1.5 shrink-0 rounded-full ${
                        status.state === "live" ? "bg-emerald-400" : "bg-amber-300"
                      }`}
                    />
                  ) : null}
                  <span className="hidden shrink-0 items-center group-hover:flex">
                    <button
                      type="button"
                      onClick={() => onSetLocked(item.id, !item.locked)}
                      title={item.locked ? "Unlock" : "Lock"}
                      aria-label={`${item.locked ? "Unlock" : "Lock"} ${source?.name ?? "source"}`}
                      aria-pressed={item.locked}
                      className={`rounded px-1 text-[10px] ${
                        item.locked ? "text-amber-300" : "text-havoc-muted hover:text-havoc-text"
                      }`}
                    >
                      {item.locked ? "🔒" : "🔓"}
                    </button>
                    <button
                      type="button"
                      disabled={modelIndex === items.length - 1}
                      onClick={() => onMove(item.id, modelIndex + 1)}
                      title="Raise in the stack"
                      aria-label={`Raise ${source?.name ?? "source"}`}
                      className="rounded px-1 text-[10px] text-havoc-muted enabled:hover:text-havoc-text disabled:opacity-40"
                    >
                      ▲
                    </button>
                    <button
                      type="button"
                      disabled={modelIndex === 0}
                      onClick={() => onMove(item.id, modelIndex - 1)}
                      title="Lower in the stack"
                      aria-label={`Lower ${source?.name ?? "source"}`}
                      className="rounded px-1 text-[10px] text-havoc-muted enabled:hover:text-havoc-text disabled:opacity-40"
                    >
                      ▼
                    </button>
                    <button
                      type="button"
                      onClick={() => onOpenFilters(item.id)}
                      title="Filters & blend"
                      aria-label={`Filters for ${source?.name ?? "source"}`}
                      className="rounded px-1 text-[10px] text-havoc-muted hover:text-havoc-text"
                    >
                      ƒ
                    </button>
                    <button
                      type="button"
                      onClick={() => onOpenProperties(item.source)}
                      title="Properties"
                      aria-label={`Properties of ${source?.name ?? "source"}`}
                      className="rounded px-1 text-[10px] text-havoc-muted hover:text-havoc-text"
                    >
                      ⚙
                    </button>
                    <button
                      type="button"
                      onClick={() => onRemove(item.id)}
                      title="Remove from this scene"
                      aria-label={`Remove ${source?.name ?? "source"}`}
                      className="rounded px-1 text-xs text-havoc-muted hover:text-red-400"
                    >
                      ×
                    </button>
                  </span>
                </div>
              </li>
            );
          })}
        </ul>
      )}

      {picker === "display" || picker === "window" ? (
        <CapturePicker mode={picker} onClose={() => setPicker(null)} onPick={pick} />
      ) : picker === "webcam" ? (
        <WebcamPicker onClose={() => setPicker(null)} onPick={pick} />
      ) : picker === "audioInput" || picker === "audioOutput" ? (
        <AudioPicker mode={picker} onClose={() => setPicker(null)} onPick={pick} />
      ) : picker === "image" ? (
        <ImageForm onClose={() => setPicker(null)} onPick={pick} />
      ) : picker === "media" ? (
        <MediaForm onClose={() => setPicker(null)} onPick={pick} />
      ) : picker === "color" ? (
        <ColorForm onClose={() => setPicker(null)} onPick={pick} />
      ) : picker === "text" ? (
        <TextForm onClose={() => setPicker(null)} onPick={pick} />
      ) : picker === "existing" ? (
        <ExistingPicker
          collection={collection}
          onClose={() => setPicker(null)}
          onPick={(sourceId) => {
            setPicker(null);
            onAddExisting(sourceId);
          }}
        />
      ) : null}
    </Panel>
  );
}

// ---------------------------------------------------------------------------
// Pickers
// ---------------------------------------------------------------------------

function CapturePicker({
  mode,
  onClose,
  onPick,
}: {
  mode: "display" | "window";
  onClose: () => void;
  onPick: (settings: SourceSettings, name?: string) => void;
}) {
  const [entries, setEntries] = useState<CaptureSource[] | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    captureListSources()
      .then((all) => {
        if (cancelled) return;
        // The portal pseudo-source stands in for both modes (Wayland).
        setEntries(all.filter((s) => s.kind === mode || s.kind === "portal"));
      })
      .catch((err) => {
        if (!cancelled) setError(String(err));
      });
    return () => {
      cancelled = true;
    };
  }, [mode]);

  const title = mode === "display" ? "Add a Display Capture" : "Add a Window Capture";
  const hasPortal = entries?.some((s) => s.kind === "portal") ?? false;

  return (
    <PickerShell title={title} onClose={onClose}>
      {error ? (
        <p className="m-0 text-xs text-red-400">{error}</p>
      ) : entries === null ? (
        <p className="m-0 text-xs text-havoc-muted">Looking for sources…</p>
      ) : entries.length === 0 ? (
        <p className="m-0 text-xs text-havoc-muted">
          Nothing to capture here — no {mode === "display" ? "displays" : "windows"} were found.
        </p>
      ) : (
        <>
          <ul className="m-0 flex list-none flex-col gap-1 p-0">
            {entries.map((entry) => (
              <li key={entry.id}>
                <button
                  type="button"
                  onClick={() =>
                    onPick(
                      entry.kind === "portal"
                        ? { kind: "portal" }
                        : {
                            kind: entry.kind as "display" | "window",
                            captureId: entry.id,
                            label: entry.label,
                          },
                      entry.label,
                    )
                  }
                  className="w-full truncate rounded-md border border-white/10 px-2 py-1.5 text-left text-xs text-havoc-text hover:border-havoc-accent/50"
                >
                  {entry.label}
                  {entry.width > 0 && (
                    <span className="ml-1.5 text-havoc-muted">
                      {entry.width}×{entry.height}
                    </span>
                  )}
                </button>
              </li>
            ))}
          </ul>
          {hasPortal && (
            <p className="mt-2 mb-0 text-[11px] leading-relaxed text-havoc-muted">
              On Wayland, the system dialog picks the screen or window — apps can’t capture globally
              there, so that’s the honest (and only) path.
            </p>
          )}
        </>
      )}
    </PickerShell>
  );
}

function WebcamPicker({
  onClose,
  onPick,
}: {
  onClose: () => void;
  onPick: (settings: SourceSettings, name?: string) => void;
}) {
  const [devices, setDevices] = useState<VideoDevice[] | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [selected, setSelected] = useState<VideoDevice | null>(null);
  const [formatsFor, setFormatsFor] = useState<{ deviceId: string; list: VideoFormat[] } | null>(
    null,
  );
  const formatRef = useRef<HTMLSelectElement>(null);

  useEffect(() => {
    let cancelled = false;
    videoDevicesList()
      .then((list) => {
        if (!cancelled) setDevices(list);
      })
      .catch((err) => {
        if (!cancelled) setError(String(err));
      });
    return () => {
      cancelled = true;
    };
  }, []);

  useEffect(() => {
    if (!selected) return;
    let cancelled = false;
    const deviceId = selected.id;
    videoDeviceFormats(deviceId)
      .then((list) => {
        if (!cancelled) setFormatsFor({ deviceId, list });
      })
      .catch(() => {
        // Format probing is best-effort (a live device can't be probed);
        // "Auto" still works.
        if (!cancelled) setFormatsFor({ deviceId, list: [] });
      });
    return () => {
      cancelled = true;
    };
  }, [selected]);

  const formats = selected && formatsFor?.deviceId === selected.id ? formatsFor.list : null;

  const add = () => {
    if (!selected) return;
    const index = formatRef.current ? Number(formatRef.current.value) : -1;
    const format = formats && index >= 0 ? formats[index] : null;
    onPick({ kind: "videoDevice", deviceId: selected.id, format }, selected.name);
  };

  return (
    <PickerShell title="Add a Video Capture Device" onClose={onClose}>
      {error ? (
        <p className="m-0 text-xs text-red-400">{error}</p>
      ) : devices === null ? (
        <p className="m-0 text-xs text-havoc-muted">Looking for cameras…</p>
      ) : devices.length === 0 ? (
        <p className="m-0 text-xs text-havoc-muted">No cameras or capture cards were found.</p>
      ) : (
        <div className="flex flex-col gap-2">
          <ul className="m-0 flex list-none flex-col gap-1 p-0">
            {devices.map((device) => (
              <li key={device.id}>
                <button
                  type="button"
                  onClick={() => setSelected(device)}
                  aria-pressed={selected?.id === device.id}
                  className={`w-full truncate rounded-md border px-2 py-1.5 text-left text-xs text-havoc-text ${
                    selected?.id === device.id
                      ? "border-havoc-accent/60 bg-havoc-accent/10"
                      : "border-white/10 hover:border-havoc-accent/50"
                  }`}
                >
                  {device.name}
                </button>
              </li>
            ))}
          </ul>
          {selected && (
            <>
              <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
                Format
                <select
                  ref={formatRef}
                  defaultValue={-1}
                  className="rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text"
                >
                  <option value={-1}>
                    {formats === null ? "Auto (loading formats…)" : "Auto (highest resolution)"}
                  </option>
                  {(formats ?? []).map((format, index) => (
                    <option
                      key={`${format.width}x${format.height}@${format.fps}-${format.fourcc}`}
                      value={index}
                    >
                      {format.width}×{format.height} @ {format.fps} fps ({format.fourcc})
                    </option>
                  ))}
                </select>
              </label>
              <button
                type="button"
                onClick={add}
                className="self-end rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
              >
                Add camera
              </button>
            </>
          )}
        </div>
      )}
    </PickerShell>
  );
}

function AudioPicker({
  mode,
  onClose,
  onPick,
}: {
  mode: "audioInput" | "audioOutput";
  onClose: () => void;
  onPick: (settings: SourceSettings, name?: string) => void;
}) {
  const [devices, setDevices] = useState<AudioDevice[] | null>(null);
  const [guidance, setGuidance] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const isLoopback = mode === "audioOutput";

  useEffect(() => {
    let cancelled = false;
    if (isLoopback) {
      audioLoopbackDevices()
        .then((result) => {
          if (cancelled) return;
          setDevices(result.devices);
          setGuidance(result.guidance ?? null);
        })
        .catch((err) => {
          if (!cancelled) setError(String(err));
        });
    } else {
      audioInputDevices()
        .then((list) => {
          if (!cancelled) setDevices(list);
        })
        .catch((err) => {
          if (!cancelled) setError(String(err));
        });
    }
    return () => {
      cancelled = true;
    };
  }, [isLoopback]);

  const title = isLoopback ? "Add an Audio Output Capture" : "Add an Audio Input Capture";
  // Windows loopback (no guidance) can capture the default output; elsewhere
  // an explicit monitor/virtual device pick is the honest requirement.
  const offerDefault = !isLoopback || (devices !== null && guidance === null);
  const entries: Array<{ id: string; name: string }> = [
    ...(offerDefault
      ? [
          {
            id: "",
            name: isLoopback ? "Default output (what you hear)" : "Default input",
          },
        ]
      : []),
    ...(devices ?? []),
  ];

  return (
    <PickerShell title={title} onClose={onClose}>
      {error ? (
        <p className="m-0 text-xs text-red-400">{error}</p>
      ) : devices === null ? (
        <p className="m-0 text-xs text-havoc-muted">Looking for audio devices…</p>
      ) : (
        <div className="flex flex-col gap-2">
          {entries.length === 0 ? (
            <p className="m-0 text-xs text-havoc-muted">
              {isLoopback
                ? "No desktop-audio capture device was found here."
                : "No microphones or line-ins were found."}
            </p>
          ) : (
            <ul className="m-0 flex list-none flex-col gap-1 p-0">
              {entries.map((device) => (
                <li key={device.id || "(default)"}>
                  <button
                    type="button"
                    onClick={() =>
                      onPick(
                        { kind: mode, deviceId: device.id },
                        device.id === "" ? undefined : device.name,
                      )
                    }
                    className="w-full truncate rounded-md border border-white/10 px-2 py-1.5 text-left text-xs text-havoc-text hover:border-havoc-accent/50"
                  >
                    {device.name}
                  </button>
                </li>
              ))}
            </ul>
          )}
          {guidance && (
            <p className="m-0 rounded-md border border-amber-400/20 bg-amber-400/5 p-2 text-[11px] leading-relaxed text-amber-200/90">
              {guidance}
            </p>
          )}
          {!isLoopback && (
            <p className="m-0 text-[10px] leading-snug text-havoc-muted">
              Mixer strips get a VU meter, fader, mute, monitoring, filters (denoise, gate,
              compressor…), and track assignment. Everything stays on this machine.
            </p>
          )}
        </div>
      )}
    </PickerShell>
  );
}

const inputClass =
  "rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text outline-none focus:border-havoc-accent/60";

function ImageForm({
  onClose,
  onPick,
}: {
  onClose: () => void;
  onPick: (settings: SourceSettings, name?: string) => void;
}) {
  const [path, setPath] = useState("");
  return (
    <PickerShell title="Add an Image" onClose={onClose}>
      <div className="flex flex-col gap-2">
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          Image file (PNG, JPEG, BMP, GIF, WebP…)
          <input
            value={path}
            onChange={(event) => setPath(event.target.value)}
            placeholder="C:\art\overlay.png"
            className={inputClass}
          />
        </label>
        <button
          type="button"
          disabled={!path.trim()}
          onClick={() => onPick({ kind: "image", path: path.trim() })}
          className="self-end rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text enabled:hover:bg-havoc-accent/25 disabled:opacity-50"
        >
          Add image
        </button>
      </div>
    </PickerShell>
  );
}

function MediaForm({
  onClose,
  onPick,
}: {
  onClose: () => void;
  onPick: (settings: SourceSettings, name?: string) => void;
}) {
  const [path, setPath] = useState("");
  const [loop, setLoop] = useState(false);
  return (
    <PickerShell title="Add Media" onClose={onClose}>
      <div className="flex flex-col gap-2">
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          Media file (mp4, mkv, webm, mov, .frec, or an image)
          <input
            value={path}
            onChange={(event) => setPath(event.target.value)}
            placeholder="C:\clips\intro.mp4"
            className={inputClass}
          />
        </label>
        <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
          <input
            type="checkbox"
            checked={loop}
            onChange={(event) => setLoop(event.target.checked)}
          />
          Loop (restart from the top at the end)
        </label>
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">
          .frec plays through the owned freally-video codec — nothing to download. The wire formats
          (mp4/mkv/webm/…) decode through the on-demand FFmpeg component; its audio lands in the
          mixer as its own strip.
        </p>
        <button
          type="button"
          disabled={!path.trim()}
          onClick={() => onPick({ kind: "media", path: path.trim(), loop, hwDecode: true })}
          className="self-end rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text enabled:hover:bg-havoc-accent/25 disabled:opacity-50"
        >
          Add media
        </button>
      </div>
    </PickerShell>
  );
}

function ColorForm({
  onClose,
  onPick,
}: {
  onClose: () => void;
  onPick: (settings: SourceSettings, name?: string) => void;
}) {
  const [hex, setHex] = useState("#4a9eff");
  const [width, setWidth] = useState(1920);
  const [height, setHeight] = useState(1080);
  return (
    <PickerShell title="Add a Color" onClose={onClose}>
      <div className="flex flex-col gap-2">
        <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
          Color
          <input
            type="color"
            value={hex}
            onChange={(event) => setHex(event.target.value)}
            aria-label="Color"
            className="h-7 w-12 cursor-pointer rounded border border-white/10 bg-transparent"
          />
        </label>
        <div className="flex gap-2">
          <NumberField
            label="Width"
            value={width}
            min={1}
            max={16384}
            onCommit={setWidth}
            className="flex-1"
          />
          <NumberField
            label="Height"
            value={height}
            min={1}
            max={16384}
            onCommit={setHeight}
            className="flex-1"
          />
        </div>
        <button
          type="button"
          onClick={() => onPick({ kind: "color", color: hexToRgba(hex), width, height })}
          className="self-end rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
        >
          Add color
        </button>
      </div>
    </PickerShell>
  );
}

function TextForm({
  onClose,
  onPick,
}: {
  onClose: () => void;
  onPick: (settings: SourceSettings, name?: string) => void;
}) {
  const [text, setText] = useState("Text");
  const [hex, setHex] = useState("#ffffff");
  const [size, setSize] = useState(72);
  return (
    <PickerShell title="Add Text" onClose={onClose}>
      <div className="flex flex-col gap-2">
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          Text
          <textarea
            value={text}
            onChange={(event) => setText(event.target.value)}
            rows={3}
            className={inputClass}
          />
        </label>
        <div className="flex items-end gap-2">
          <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
            Color
            <input
              type="color"
              value={hex}
              onChange={(event) => setHex(event.target.value)}
              aria-label="Text color"
              className="h-7 w-12 cursor-pointer rounded border border-white/10 bg-transparent"
            />
          </label>
          <NumberField
            label="Size (px)"
            value={size}
            min={4}
            max={512}
            onCommit={setSize}
            className="flex-1"
          />
        </div>
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">
          Font family, alignment, wrapping, and RTL live in the source’s Properties. The bundled
          Noto Sans (incl. Arabic/Hebrew) is the default — identical on every machine.
        </p>
        <button
          type="button"
          disabled={!text.trim()}
          onClick={() =>
            onPick(
              {
                kind: "text",
                text,
                fontFamily: null,
                fontFile: null,
                sizePx: size,
                color: hexToRgba(hex),
                align: "left",
                lineSpacing: 1.0,
                forceRtl: false,
                wrapWidth: null,
              },
              text.length > 24 ? `${text.slice(0, 24)}…` : text,
            )
          }
          className="self-end rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text enabled:hover:bg-havoc-accent/25 disabled:opacity-50"
        >
          Add text
        </button>
      </div>
    </PickerShell>
  );
}

function ExistingPicker({
  collection,
  onClose,
  onPick,
}: {
  collection: Collection | null;
  onClose: () => void;
  onPick: (source: SourceId) => void;
}) {
  const sources = collection?.sources ?? [];
  return (
    <PickerShell title="Add an existing source" onClose={onClose}>
      {sources.length === 0 ? (
        <p className="m-0 text-xs text-havoc-muted">
          No sources exist yet — add one to any scene first. Existing sources are shared: renaming
          or reconfiguring one updates every scene that shows it.
        </p>
      ) : (
        <ul className="m-0 flex list-none flex-col gap-1 p-0">
          {sources.map((source) => (
            <li key={source.id}>
              <button
                type="button"
                onClick={() => onPick(source.id)}
                className="flex w-full items-center gap-1.5 truncate rounded-md border border-white/10 px-2 py-1.5 text-left text-xs text-havoc-text hover:border-havoc-accent/50"
              >
                <span className="rounded bg-white/10 px-1 py-px text-[9px] text-havoc-muted uppercase">
                  {KIND_BADGE[source.kind] ?? "?"}
                </span>
                <span className="truncate">{source.name}</span>
              </button>
            </li>
          ))}
        </ul>
      )}
    </PickerShell>
  );
}
