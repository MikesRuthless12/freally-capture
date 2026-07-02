import { useEffect, useState } from "react";

import { health, settingsGet, settingsSet } from "./api/commands";
import type { Health, Settings } from "./api/types";
import { ControlsDock } from "./panels/ControlsDock";
import { MixerDock } from "./panels/MixerDock";
import { PreviewPanel } from "./panels/PreviewPanel";
import { ScenesRail } from "./panels/ScenesRail";
import { SourcesRail } from "./panels/SourcesRail";
import { StatsDock } from "./panels/StatsDock";

/** The Freally Capture studio shell: preview + rails + bottom docks. */
export default function App() {
  const [core, setCore] = useState<Health | null>(null);
  const [coreError, setCoreError] = useState(false);
  const [settings, setSettings] = useState<Settings | null>(null);
  // The stats dock renders only after settings settle (loaded or failed), so
  // a persisted "off" never flashes visible on launch.
  const [settingsSettled, setSettingsSettled] = useState(false);
  const [saveError, setSaveError] = useState<string | null>(null);

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
    return () => {
      cancelled = true;
    };
  }, []);

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
          <PreviewPanel />
          <SourcesRail />
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
