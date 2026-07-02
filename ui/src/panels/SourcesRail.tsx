import { useEffect, useRef, useState } from "react";

import { captureListSources, videoDeviceFormats, videoDevicesList } from "../api/commands";
import type {
  CaptureSource,
  PreviewSource,
  PreviewStatus,
  VideoDevice,
  VideoFormat,
} from "../api/types";
import { EmptyHint, Panel } from "../components/Panel";

/** One entry in the rail: what the user added + the key preview events use. */
export type AddedSource = {
  key: string;
  label: string;
  source: PreviewSource;
};

type SourcesRailProps = {
  sources: AddedSource[];
  activeKey: string | null;
  status: PreviewStatus;
  onAdd: (source: PreviewSource, label: string) => void;
  onSelect: (key: string) => void;
  onRemove: (key: string) => void;
};

type PickerMode = "display" | "window" | "webcam";

const KIND_BADGE: Record<string, string> = {
  display: "Display",
  window: "Window",
  portal: "Portal",
  webcam: "Camera",
};

/** The Sources rail: add Display/Window/Webcam sources; one previews live. */
export function SourcesRail({
  sources,
  activeKey,
  status,
  onAdd,
  onSelect,
  onRemove,
}: SourcesRailProps) {
  const [menuOpen, setMenuOpen] = useState(false);
  const [picker, setPicker] = useState<PickerMode | null>(null);

  const openPicker = (mode: PickerMode) => {
    setMenuOpen(false);
    setPicker(mode);
  };

  return (
    <Panel
      title="Sources"
      actions={
        <div className="relative">
          <button
            type="button"
            onClick={() => setMenuOpen((open) => !open)}
            title="Add a source"
            aria-label="Add a source"
            aria-haspopup="menu"
            aria-expanded={menuOpen}
            className="rounded-md border border-white/10 px-2 py-0.5 text-xs text-havoc-muted transition-colors hover:border-havoc-accent/50 hover:text-havoc-text"
          >
            +
          </button>
          {menuOpen && (
            <div
              role="menu"
              aria-label="Add a source"
              className="absolute right-0 z-20 mt-1 w-44 rounded-lg border border-white/10 bg-havoc-panel p-1 shadow-xl"
            >
              {(
                [
                  ["display", "Display Capture"],
                  ["window", "Window Capture"],
                  ["webcam", "Video Capture Device"],
                ] as const
              ).map(([mode, label]) => (
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
            </div>
          )}
        </div>
      }
    >
      {sources.length === 0 ? (
        <EmptyHint>
          No sources yet — add a Display Capture, Window Capture, or Webcam with “+”. One source
          previews at a time; scenes and mixing land with the compositor (0.40.0).
        </EmptyHint>
      ) : (
        <ul className="m-0 flex list-none flex-col gap-1.5 p-0">
          {sources.map((added) => {
            const isActive = added.key === activeKey;
            const state = isActive ? status.state : undefined;
            return (
              <li key={added.key}>
                <div
                  className={`flex items-center gap-2 rounded-lg border px-2 py-1.5 ${
                    isActive
                      ? "border-havoc-accent/50 bg-havoc-accent/10"
                      : "border-white/10 bg-white/[0.02]"
                  }`}
                >
                  <button
                    type="button"
                    onClick={() => onSelect(added.key)}
                    title={isActive ? "Previewing" : "Preview this source"}
                    className="min-w-0 flex-1 truncate text-left text-xs text-havoc-text"
                  >
                    <span className="mr-1.5 rounded bg-white/10 px-1 py-px text-[10px] text-havoc-muted uppercase">
                      {KIND_BADGE[added.source.kind]}
                    </span>
                    {added.label}
                  </button>
                  {isActive && state && state !== "idle" && (
                    <span
                      className={`shrink-0 text-[10px] uppercase ${
                        state === "live"
                          ? "text-emerald-400"
                          : state === "error"
                            ? "text-red-400"
                            : "text-amber-300"
                      }`}
                    >
                      {state}
                    </span>
                  )}
                  <button
                    type="button"
                    onClick={() => onRemove(added.key)}
                    title="Remove this source"
                    aria-label={`Remove ${added.label}`}
                    className="shrink-0 rounded px-1 text-xs text-havoc-muted hover:text-red-400"
                  >
                    ×
                  </button>
                </div>
              </li>
            );
          })}
        </ul>
      )}
      {picker === "display" || picker === "window" ? (
        <CapturePicker
          mode={picker}
          onClose={() => setPicker(null)}
          onPick={(source, label) => {
            setPicker(null);
            onAdd(source, label);
          }}
        />
      ) : picker === "webcam" ? (
        <WebcamPicker
          onClose={() => setPicker(null)}
          onPick={(source, label) => {
            setPicker(null);
            onAdd(source, label);
          }}
        />
      ) : null}
    </Panel>
  );
}

function PickerShell({
  title,
  onClose,
  children,
}: {
  title: string;
  onClose: () => void;
  children: React.ReactNode;
}) {
  useEffect(() => {
    const onKey = (event: KeyboardEvent) => {
      if (event.key === "Escape") onClose();
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [onClose]);

  return (
    <div className="fixed inset-0 z-30 flex items-center justify-center bg-black/60 p-6">
      <div
        role="dialog"
        aria-label={title}
        className="flex max-h-[70vh] w-[26rem] max-w-full flex-col rounded-xl border border-white/10 bg-havoc-panel shadow-2xl"
      >
        <header className="flex items-center justify-between border-b border-white/5 px-4 py-2.5">
          <h3 className="m-0 text-xs font-semibold tracking-wider text-havoc-muted uppercase">
            {title}
          </h3>
          <button
            type="button"
            onClick={onClose}
            aria-label="Close"
            className="rounded px-1.5 text-sm text-havoc-muted hover:text-havoc-text"
          >
            ×
          </button>
        </header>
        <div className="min-h-0 flex-1 overflow-auto p-3">{children}</div>
      </div>
    </div>
  );
}

function CapturePicker({
  mode,
  onClose,
  onPick,
}: {
  mode: "display" | "window";
  onClose: () => void;
  onPick: (source: PreviewSource, label: string) => void;
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
                        ? { kind: "portal", label: entry.label }
                        : {
                            kind: entry.kind as "display" | "window",
                            id: entry.id,
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
  onPick: (source: PreviewSource, label: string) => void;
}) {
  const [devices, setDevices] = useState<VideoDevice[] | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [selected, setSelected] = useState<VideoDevice | null>(null);
  // Keyed by device id so switching devices reads as "loading" without a
  // synchronous reset inside the fetch effect.
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
        // Format probing is best-effort; "Auto" still works.
        if (!cancelled) setFormatsFor({ deviceId, list: [] });
      });
    return () => {
      cancelled = true;
    };
  }, [selected]);

  /** `null` = still loading for the selected device. */
  const formats = selected && formatsFor?.deviceId === selected.id ? formatsFor.list : null;

  const add = () => {
    if (!selected) return;
    const index = formatRef.current ? Number(formatRef.current.value) : -1;
    const format = formats && index >= 0 ? formats[index] : undefined;
    onPick({ kind: "webcam", id: selected.id, label: selected.name, format }, selected.name);
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
