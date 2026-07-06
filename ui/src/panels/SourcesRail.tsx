import { useCallback, useEffect, useRef, useState } from "react";

import {
  audioInputDevices,
  audioLoopbackDevices,
  captureListSources,
  captureWindowThumbnail,
  openPrivacySettings,
  studioApplyLayout,
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
  Corner,
  CornerSlot,
  ItemId,
  ProgramStatus,
  Scene,
  SceneId,
  SourceId,
  SourceSettings,
  VideoDevice,
  VideoFormat,
} from "../api/types";
import { CORNERS } from "../api/types";
import { EmptyHint, Panel } from "../components/Panel";
import { NumberField } from "../components/NumberField";
import { PickerShell } from "../components/PickerShell";
import { hexToRgba } from "../lib/color";
import { spikeHost, spikeJoin, spikeStop } from "../remote/spike";

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
  | "remoteGuest"
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
  remoteGuest: "Guest",
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
  ["remoteGuest", "Remote Guest (P2P spike)"],
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
  const [showLayout, setShowLayout] = useState(false);
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
        <div className="flex items-center gap-1">
          <button
            type="button"
            disabled={!scene}
            onClick={() => setShowLayout(true)}
            title="Arrange: screen + corners"
            aria-label="Arrange: screen + corners"
            className="rounded-md border border-white/10 px-2 py-0.5 text-xs text-havoc-muted transition-colors enabled:hover:border-havoc-accent/50 enabled:hover:text-havoc-text disabled:opacity-60"
          >
            ▦
          </button>
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
                  Browser Source arrives later — it needs its own offscreen-webview work.
                </p>
              </div>
            )}
          </div>
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
            // Audio-*only* sources report through the `audio` event; every
            // video source (incl. Media, which also has audio but is
            // video-primary) reports its pipeline state — errors, retry —
            // through `program`. Same status shape, one dot.
            const audioOnly = source?.kind === "audioInput" || source?.kind === "audioOutput";
            const status = audioOnly ? audio?.sources[item.source] : program?.sources[item.source];
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
      ) : picker === "remoteGuest" && scene ? (
        <RemoteGuestForm sceneId={scene.id} onClose={() => setPicker(null)} />
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
      {showLayout && (
        <LayoutPicker collection={collection} scene={scene} onClose={() => setShowLayout(false)} />
      )}
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
  const loadedRef = useRef(false);

  const refresh = useCallback(() => {
    captureListSources()
      .then((all) => {
        // The portal pseudo-source stands in for both modes (Wayland).
        setEntries(all.filter((s) => s.kind === mode || s.kind === "portal"));
        setError(null);
        loadedRef.current = true;
      })
      .catch((err) => {
        // Only surface a failure before the first successful load; ignore
        // transient refresh errors so the last good list stays put.
        if (!loadedRef.current) setError(String(err));
      });
  }, [mode]);

  useEffect(() => {
    refresh();
    // Re-scan while the picker is open so a window you restore (e.g. from the
    // system tray) shows up within a couple seconds without reopening — the ↻
    // button in the header does the same on demand.
    const timer = window.setInterval(refresh, 2000);
    return () => window.clearInterval(timer);
  }, [refresh]);

  const title = mode === "display" ? "Add a Display Capture" : "Add a Window Capture";
  const hasPortal = entries?.some((s) => s.kind === "portal") ?? false;
  // Window mode shows a live thumbnail grid; everything else (displays, and the
  // Wayland portal entry in either mode) stays a text row.
  const windowTiles =
    mode === "window" ? (entries ?? []).filter((entry) => entry.kind === "window") : [];
  const listEntries = (entries ?? []).filter(
    (entry) => !(mode === "window" && entry.kind === "window"),
  );

  return (
    <PickerShell title={title} onClose={onClose} onRefresh={refresh}>
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
          {windowTiles.length > 0 && (
            <div className="grid grid-cols-2 gap-2">
              {windowTiles.map((entry, index) => (
                <WindowThumbTile
                  key={entry.id}
                  entry={entry}
                  index={index}
                  onPick={() =>
                    onPick({ kind: "window", captureId: entry.id, label: entry.label }, entry.label)
                  }
                />
              ))}
            </div>
          )}
          {listEntries.length > 0 && (
            <ul
              className={`m-0 flex list-none flex-col gap-1 p-0 ${windowTiles.length > 0 ? "mt-2" : ""}`}
            >
              {listEntries.map((entry) => (
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
          )}
          {hasPortal && (
            <p className="mt-2 mb-0 text-[11px] leading-relaxed text-havoc-muted">
              On Wayland, the system dialog picks the screen or window — apps can’t capture globally
              there, so that’s the honest (and only) path.
            </p>
          )}
          {mode === "window" && windowTiles.length > 0 && (
            <p className="mt-2 mb-0 text-[10px] leading-snug text-havoc-muted">
              Previews update live. A minimized window shows its last frame (or none) until you
              restore it.
            </p>
          )}
        </>
      )}
    </PickerShell>
  );
}

/**
 * One window tile with a *live* preview: it re-requests the thumbnail on a ~1 s
 * timer while the picker is open. Visible windows update; a minimized one keeps
 * its last frame (or shows a placeholder until/unless it's restored).
 */
function WindowThumbTile({
  entry,
  index,
  onPick,
}: {
  entry: CaptureSource;
  index: number;
  onPick: () => void;
}) {
  const [thumb, setThumb] = useState<string | null>(null);
  const [tried, setTried] = useState(false);

  useEffect(() => {
    let cancelled = false;
    let inFlight = false;
    let interval: number | undefined;
    const tick = () => {
      if (inFlight) return;
      inFlight = true;
      captureWindowThumbnail(entry.id, 320)
        .then((uri) => {
          if (!cancelled && uri) setThumb(uri);
        })
        .catch(() => {
          // Keep the last good frame; the placeholder covers the "never" case.
        })
        .finally(() => {
          inFlight = false;
          if (!cancelled) setTried(true);
        });
    };
    // Each grab briefly spins up the real capture backend, so stagger the first
    // one by tile index (no burst when the picker opens), then refresh on a
    // gentle interval for a live-ish preview.
    const startDelay = Math.min(index * 250, 1500);
    const kickoff = window.setTimeout(() => {
      tick();
      interval = window.setInterval(tick, 3000);
    }, startDelay);
    return () => {
      cancelled = true;
      window.clearTimeout(kickoff);
      if (interval !== undefined) window.clearInterval(interval);
    };
  }, [entry.id, index]);

  return (
    <button
      type="button"
      onClick={onPick}
      title={entry.label}
      className="group flex flex-col overflow-hidden rounded-md border border-white/10 text-left transition-colors hover:border-havoc-accent/50"
    >
      <div className="flex aspect-video w-full items-center justify-center bg-black/40">
        {thumb ? (
          <img src={thumb} alt="" className="h-full w-full object-contain" />
        ) : (
          <span className="text-[10px] text-havoc-muted">{tried ? "no preview" : "loading…"}</span>
        )}
      </div>
      <span className="truncate px-1.5 py-1 text-[11px] text-havoc-text">{entry.label}</span>
    </button>
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

/**
 * Remote Guest transport spike (TASK-R1). HOST: start a session, share the
 * session id, and the guest's webcam lands in the scene when they call.
 * GUEST: paste the host's session id and share this machine's webcam.
 * Media flows P2P (WebRTC); only signaling touches the PeerJS broker —
 * nothing runs until a session is explicitly started here.
 */
function RemoteGuestForm({ sceneId, onClose }: { sceneId: SceneId; onClose: () => void }) {
  const [status, setStatus] = useState("idle — nothing touches the network yet");
  const [peerId, setPeerId] = useState<string | null>(null);
  const [hostId, setHostId] = useState("");

  return (
    <PickerShell title="Remote Guest (P2P spike)" onClose={onClose}>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <div className="flex flex-col gap-1.5">
          <p className="m-0 text-[11px] font-semibold uppercase tracking-wide text-havoc-muted">
            Host — receive a guest
          </p>
          <button
            type="button"
            onClick={() => spikeHost(sceneId, setStatus, setPeerId)}
            className="self-start rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
          >
            Start hosting
          </button>
          {peerId && (
            <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
              Session id
              <input
                readOnly
                value={peerId}
                onFocus={(event) => event.target.select()}
                aria-label="Session id"
                className="flex-1 rounded border border-white/10 bg-black/30 px-2 py-1 font-mono text-[11px] text-havoc-text"
              />
            </label>
          )}
        </div>
        <div className="flex flex-col gap-1.5 border-t border-white/5 pt-2">
          <p className="m-0 text-[11px] font-semibold uppercase tracking-wide text-havoc-muted">
            Guest — join a host
          </p>
          <div className="flex gap-2">
            <input
              value={hostId}
              onChange={(event) => setHostId(event.target.value)}
              placeholder="host session id"
              aria-label="Host session id"
              className="flex-1 rounded border border-white/10 bg-black/30 px-2 py-1 font-mono text-[11px] text-havoc-text"
            />
            <button
              type="button"
              disabled={!hostId.trim()}
              onClick={() => {
                spikeJoin(hostId, setStatus).catch((err) => setStatus(`join failed: ${err}`));
              }}
              className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25 disabled:opacity-60"
            >
              Join with webcam
            </button>
          </div>
        </div>
        <div className="flex items-center justify-between gap-2 border-t border-white/5 pt-2">
          <p className="m-0 flex-1 text-[11px] leading-snug text-havoc-muted">{status}</p>
          <button
            type="button"
            onClick={() => {
              spikeStop();
              setPeerId(null);
              setStatus("stopped");
            }}
            className="rounded-md border border-white/10 px-2 py-1 text-[11px] text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
          >
            Stop session
          </button>
        </div>
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

/** A source's assigned slot in the screen-plus-corners layout. */
type LayoutChoice = "off" | "center" | Corner;

const SLOT_OPTIONS: Array<[LayoutChoice, string]> = [
  ["off", "Off"],
  ["center", "Center (screen)"],
  ["topLeft", "Top-Left"],
  ["topRight", "Top-Right"],
  ["bottomLeft", "Bottom-Left"],
  ["bottomRight", "Bottom-Right"],
];

/**
 * Arrange the scene as a centered screen with up to four corner cameras — the
 * explainer / podcast layout. Screen-kind sources auto-seat to the center,
 * cameras fill the corners; the user can reassign any of them (and drag on the
 * canvas afterward). Audio-only sources are skipped — they don't compose.
 */
function LayoutPicker({
  collection,
  scene,
  onClose,
}: {
  collection: Collection | null;
  scene: Scene | null;
  onClose: () => void;
}) {
  const sourceOf = (id: SourceId) => collection?.sources.find((source) => source.id === id);
  const visual = (scene?.items ?? []).filter((item) => {
    const kind = sourceOf(item.source)?.kind;
    return kind !== "audioInput" && kind !== "audioOutput";
  });

  const [choice, setChoice] = useState<Record<string, LayoutChoice>>(() => {
    const map: Record<string, LayoutChoice> = {};
    let centerTaken = false;
    let cornerIdx = 0;
    for (const item of visual) {
      const kind = sourceOf(item.source)?.kind;
      if (!centerTaken && (kind === "display" || kind === "window" || kind === "portal")) {
        map[item.id] = "center";
        centerTaken = true;
      } else if ((kind === "videoDevice" || kind === "media") && cornerIdx < CORNERS.length) {
        map[item.id] = CORNERS[cornerIdx];
        cornerIdx += 1;
      } else {
        map[item.id] = "off";
      }
    }
    return map;
  });

  const apply = () => {
    if (!scene) return;
    // Dedupe by slot — the first source assigned to a slot wins it.
    let center: ItemId | null = null;
    const taken = new Set<Corner>();
    const corners: CornerSlot[] = [];
    for (const item of visual) {
      const slot = choice[item.id] ?? "off";
      if (slot === "off") continue;
      if (slot === "center") {
        center ??= item.id;
      } else if (!taken.has(slot)) {
        taken.add(slot);
        corners.push({ itemId: item.id, corner: slot });
      }
    }
    studioApplyLayout(scene.id, center, corners).catch((err) =>
      console.error("apply layout failed:", err),
    );
    onClose();
  };

  return (
    <PickerShell title="Arrange: Screen + corners" onClose={onClose}>
      {visual.length === 0 ? (
        <p className="m-0 text-xs text-havoc-muted">
          Add a screen capture and one or more cameras to this scene first, then arrange them here.
        </p>
      ) : (
        <div className="flex flex-col gap-2">
          <p className="m-0 text-[11px] leading-relaxed text-havoc-muted">
            Put a screen in the center and up to four cameras in the corners — your explainer /
            podcast layout. Each corner holds a webcam, a captured call window, or a media clip. You
            can drag any of them on the canvas afterward.
          </p>
          <ul className="m-0 flex list-none flex-col gap-1 p-0">
            {visual.map((item) => {
              const source = sourceOf(item.source);
              return (
                <li key={item.id} className="flex items-center gap-2">
                  <span className="rounded bg-white/10 px-1 py-px text-[9px] text-havoc-muted uppercase">
                    {KIND_BADGE[source?.kind ?? ""] ?? "?"}
                  </span>
                  <span className="min-w-0 flex-1 truncate text-xs text-havoc-text">
                    {source?.name ?? "(missing source)"}
                  </span>
                  <select
                    value={choice[item.id] ?? "off"}
                    onChange={(event) =>
                      setChoice((prev) => ({
                        ...prev,
                        [item.id]: event.target.value as LayoutChoice,
                      }))
                    }
                    aria-label={`Slot for ${source?.name ?? "source"}`}
                    className="rounded-md border border-white/10 bg-havoc-panel px-2 py-1 text-xs text-havoc-text"
                  >
                    {SLOT_OPTIONS.map(([value, label]) => (
                      <option key={value} value={value}>
                        {label}
                      </option>
                    ))}
                  </select>
                </li>
              );
            })}
          </ul>
          <button
            type="button"
            onClick={apply}
            className="self-end rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
          >
            Apply layout
          </button>
        </div>
      )}
    </PickerShell>
  );
}
