import { useCallback, useEffect, useState } from "react";

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
      if (!cancelled) setPreviewStatus(status);
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
    previewStart(source, key).catch((err) => console.error("preview start failed:", err));
  }, []);

  const selectSource = useCallback(
    (key: string) => {
      const entry = sources.find((added) => added.key === key);
      if (!entry || key === activeKey) return;
      setActiveKey(key);
      previewStart(entry.source, key).catch((err) => console.error("preview start failed:", err));
    },
    [sources, activeKey],
  );

  const removeSource = useCallback(
    (key: string) => {
      setSources((current) => current.filter((added) => added.key !== key));
      if (key === activeKey) {
        setActiveKey(null);
        previewStop().catch(() => undefined);
      }
    },
    [activeKey],
  );

  const activeKind = sources.find((added) => added.key === activeKey)?.source.kind;

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
