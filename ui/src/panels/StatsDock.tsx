import { useEffect, useState } from "react";

import { onStats } from "../api/events";
import type { StatsPayload } from "../api/types";
import { EmptyHint, Panel } from "../components/Panel";

/** One stat readout tile. */
function Stat({ label, value }: { label: string; value: string }) {
  return (
    <div className="rounded-lg border border-white/5 bg-white/[0.03] px-2.5 py-1.5">
      <div className="text-[10px] tracking-wider uppercase text-havoc-muted">{label}</div>
      <div className="text-sm font-semibold tabular-nums">{value}</div>
    </div>
  );
}

/** The stats dock — renders the core's ~2 Hz `stats` push event. */
export function StatsDock() {
  const [stats, setStats] = useState<StatsPayload | null>(null);

  useEffect(() => {
    let disposed = false;
    let unlisten: (() => void) | undefined;

    onStats((payload) => setStats(payload))
      .then((fn) => {
        if (disposed) {
          fn();
        } else {
          unlisten = fn;
        }
      })
      .catch(() => {
        // Not running inside Tauri (plain browser / tests): no events arrive.
      });

    return () => {
      disposed = true;
      unlisten?.();
    };
  }, []);

  return (
    <Panel title="Stats">
      <div className="grid grid-cols-3 gap-2">
        <Stat label="FPS" value={stats ? (stats.fps ?? 0).toFixed(0) : "—"} />
        <Stat label="CPU" value={stats ? `${(stats.cpu ?? 0).toFixed(1)}%` : "—"} />
        <Stat label="Memory" value={stats ? `${(stats.memoryMb ?? 0).toFixed(0)} MB` : "—"} />
        <Stat label="Dropped" value={stats ? (stats.dropped ?? 0).toFixed(0) : "—"} />
        <Stat label="Render" value={stats ? `${(stats.renderMs ?? 0).toFixed(1)} ms` : "—"} />
        <Stat label="GPU" value={stats && (stats.fps ?? 0) > 0 ? "compositing" : "idle"} />
      </div>
      {stats?.placeholder && (
        <div className="mt-2">
          <EmptyHint>Starting the compositor…</EmptyHint>
        </div>
      )}
    </Panel>
  );
}
