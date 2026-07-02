import { useCallback, useEffect, useRef, useState } from "react";

import { health, previewStart, previewStop, settingsGet, settingsSet } from "./api/commands";
import { onPreview } from "./api/events";
import type { Health, PreviewSource, PreviewStatus, Settings } from "./api/types";
import { ControlsDock } from "./panels/ControlsDock";
import { MixerDock } from "./panels/MixerDock";
import { PreviewPanel } from "./panels/PreviewPanel";
import { ScenesRail } from "./panels/ScenesRail";
import { SourcesRail, type AddedSource } from "./panels/SourcesRail";
import { StatsDock } from "./panels/StatsDock";

let nextSourceKey = 0;

/** The Freally Capture studio shell: preview + rails + bottom docks. */
export default function App() {
  const [core, setCore] = useState<Health | null>(null);
  const [coreError, setCoreError] = useState(false);
  const [settings, setSettings] = useState<Settings | null>(null);
  // The stats dock renders only after settings settle (loaded or failed), so
  // a persisted "off" never flashes visible on launch.
  const [settingsSettled, setSettingsSettled] = useState(false);
  const [saveError, setSaveError] = useState<string | null>(null);
  const [sources, setSources] = useState<AddedSource[]>([]);
  const [activeKey, setActiveKey] = useState<string | null>(null);
  const [previewStatus, setPreviewStatus] = useState<PreviewStatus>({ state: "idle" });
  // Mirrors activeKey for the event listener (registered once on mount):
  // stale events from a superseded pump carry the old sourceKey and must not
  // overwrite the newly selected source's status. Updated synchronously in
  // the add/select/remove handlers (events can beat React's re-render).
  const activeKeyRef = useRef<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    health()
      .then((report) => {
        if (!cancelled) setCore(report);
      })
      .catch(() => {
        // Plain browser / test runs have no Tauri core behind the bridge.
        if (!cancelled) setCoreError(true);
      });
    settingsGet()
      .then((loaded) => {
        if (!cancelled) {
          setSettings(loaded);
          setSettingsSettled(true);
        }
      })
      .catch(() => {
        // Without the core, the UI just keeps its defaults (nothing persists).
        if (!cancelled) setSettingsSettled(true);
      });
    const unlisten = onPreview((status) => {
      if (cancelled) return;
      // Keyed events must match the active card; unkeyed ones (idle) always apply.
      if (status.sourceKey && status.sourceKey !== activeKeyRef.current) return;
      setPreviewStatus(status);
    }).catch(() => undefined);
    return () => {
      cancelled = true;
      void unlisten.then((fn) => fn?.());
    };
  }, []);

  const addSource = useCallback((source: PreviewSource, label: string) => {
    const key = `src-${nextSourceKey++}`;
    setSources((current) => [...current, { key, label, source }]);
    setActiveKey(key);
    // Synchronously, not just at re-render: the pump's first event can
    // arrive before React commits, and it must not be filtered out.
    activeKeyRef.current = key;
    previewStart(source, key).catch((err) => console.error("preview start failed:", err));
  }, []);

  const selectSource = useCallback(
    (key: string) => {
      const entry = sources.find((added) => added.key === key);
      if (!entry) return;
      // Clicking the active card is a no-op while it works — but when it
      // errored (e.g. a permission was just granted), a click retries it.
      if (key === activeKey && previewStatus.state !== "error") return;
      setActiveKey(key);
      activeKeyRef.current = key;
      previewStart(entry.source, key).catch((err) => console.error("preview start failed:", err));
    },
    [sources, activeKey, previewStatus.state],
  );

  const removeSource = useCallback(
    (key: string) => {
      setSources((current) => current.filter((added) => added.key !== key));
      if (key === activeKey) {
        setActiveKey(null);
        activeKeyRef.current = null;
        previewStop().catch(() => undefined);
      }
    },
    [activeKey],
  );

  const activeSource = sources.find((added) => added.key === activeKey)?.source;
  const activeKind = activeSource?.kind;
  // Cameras are exclusive — the picker must not probe one that's live.
  const liveWebcamId = activeSource?.kind === "webcam" ? activeSource.id : undefined;

  const showStats = settingsSettled && (settings?.showStatsDock ?? true);

  const toggleStatsDock = () => {
    if (!settings) return;
    const previous = settings;
    const next = { ...settings, showStatsDock: !settings.showStatsDock };
    setSettings(next);
    setSaveError(null);
    settingsSet(next).catch((err) => {
      // Roll back so the UI never claims a state the disk doesn't have.
      setSettings(previous);
      setSaveError("Couldn't save settings — the change won't survive a restart.");
      console.error("could not persist settings:", err);
    });
  };

  return (
    <div className="flex h-full flex-col gap-2 p-2">
      <header className="flex shrink-0 items-center justify-between rounded-xl border border-white/10 bg-white/[0.03] px-4 py-2">
        <span className="bg-gradient-to-r from-havoc-accent to-havoc-accent-2 bg-clip-text text-sm font-bold tracking-wide text-transparent">
          Freally Capture
        </span>
        <div className="flex items-center gap-3">
          {saveError && (
            <span role="alert" className="text-xs text-amber-400">
              {saveError}
            </span>
          )}
          <button
            type="button"
            onClick={toggleStatsDock}
            disabled={!settings}
            title={showStats ? "Hide the stats dock" : "Show the stats dock"}
            className="rounded-md border border-white/10 px-2 py-0.5 text-xs text-havoc-muted transition-colors enabled:hover:border-havoc-accent/50 enabled:hover:text-havoc-text disabled:opacity-50"
          >
            Stats {showStats ? "on" : "off"}
          </button>
          <span className="text-xs text-havoc-muted">
            {core
              ? `v${core.appVersion} · core ${core.coreOk ? "OK" : "ERROR"}`
              : coreError
                ? "core unreachable (browser mode)"
                : "connecting to core…"}
          </span>
        </div>
      </header>

      <main className="flex min-h-0 flex-1 flex-col gap-2">
        <div className="grid min-h-0 flex-1 grid-cols-[240px_minmax(0,1fr)_240px] gap-2">
          <ScenesRail />
          <PreviewPanel status={previewStatus} os={core?.os} activeKind={activeKind} />
          <SourcesRail
            sources={sources}
            activeKey={activeKey}
            status={previewStatus}
            liveWebcamId={liveWebcamId}
            onAdd={addSource}
            onSelect={selectSource}
            onRemove={removeSource}
          />
        </div>
        <div
          className={`grid h-44 shrink-0 gap-2 ${
            showStats ? "grid-cols-[2fr_1fr_1fr]" : "grid-cols-[3fr_1fr]"
          }`}
        >
          <MixerDock />
          <ControlsDock />
          {showStats && <StatsDock />}
        </div>
      </main>
    </div>
  );
}
