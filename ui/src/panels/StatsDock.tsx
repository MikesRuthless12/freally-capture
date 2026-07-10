import { useEffect, useState } from "react";

import { onStats, onStream } from "../api/events";
import type { StatsPayload, StreamStatus } from "../api/types";
import { EmptyHint, Panel } from "../components/Panel";
import { useT } from "../i18n/t";

/** One stat readout tile. */
function Stat({ label, value }: { label: string; value: string }) {
  return (
    <div className="rounded-lg border border-white/5 bg-white/[0.03] px-2.5 py-1.5">
      <div className="text-[10px] tracking-wider uppercase text-havoc-muted">{label}</div>
      <div className="text-sm font-semibold tabular-nums">{value}</div>
    </div>
  );
}

const targetDot: Record<string, string> = {
  live: "bg-red-500",
  reconnecting: "animate-pulse bg-amber-400",
  failed: "bg-red-800",
  ended: "bg-white/30",
};

/** The stats dock — the core's ~2 Hz `stats` push event, plus per-target
 * stream health + bitrate from the ~1 Hz `stream` event (TASK-601). */
export function StatsDock() {
  const t = useT();
  const [stats, setStats] = useState<StatsPayload | null>(null);
  const [stream, setStream] = useState<StreamStatus | null>(null);

  useEffect(() => {
    let disposed = false;
    const cleanups: Array<() => void> = [];

    onStats((payload) => setStats(payload))
      .then((fn) => {
        if (disposed) fn();
        else cleanups.push(fn);
      })
      .catch(() => {
        // Not running inside Tauri (plain browser / tests): no events arrive.
      });
    onStream((payload) => setStream(payload))
      .then((fn) => {
        if (disposed) fn();
        else cleanups.push(fn);
      })
      .catch(() => undefined);

    return () => {
      disposed = true;
      cleanups.forEach((fn) => fn());
    };
  }, []);

  const targets = stream && stream.state !== "idle" ? stream.targets : [];

  return (
    <Panel title={t("stats")}>
      <div className="grid grid-cols-3 gap-2">
        <Stat label={t("stats-fps")} value={stats ? (stats.fps ?? 0).toFixed(0) : "—"} />
        <Stat label={t("stats-cpu")} value={stats ? `${(stats.cpu ?? 0).toFixed(1)}%` : "—"} />
        <Stat
          label={t("stats-memory")}
          value={stats ? `${(stats.memoryMb ?? 0).toFixed(0)} MB` : "—"}
        />
        <Stat label={t("stats-dropped")} value={stats ? (stats.dropped ?? 0).toFixed(0) : "—"} />
        <Stat
          label={t("stats-render")}
          value={stats ? `${(stats.renderMs ?? 0).toFixed(1)} ms` : "—"}
        />
        <Stat
          label={t("stats-gpu")}
          value={stats && (stats.fps ?? 0) > 0 ? t("stats-gpu-compositing") : t("stats-gpu-idle")}
        />
        {(stats?.verticalFps ?? 0) > 0 && (
          <Stat label={t("stats-vertical-fps")} value={(stats?.verticalFps ?? 0).toFixed(0)} />
        )}
      </div>
      {targets.length > 0 && (
        <ul className="mt-2 flex flex-col gap-1" aria-label={t("stats-targets-label")}>
          {targets.map((target) => (
            <li
              key={target.id}
              className="flex items-center gap-2 rounded-lg border border-white/5 bg-white/[0.03] px-2.5 py-1.5 text-[11px]"
            >
              <span
                aria-hidden
                className={`inline-block h-2 w-2 shrink-0 rounded-full ${
                  targetDot[target.state] ?? "bg-white/30"
                }`}
              />
              <span className="min-w-0 flex-1 truncate">
                {target.label}
                {target.shared > 0 && (
                  <span className="text-havoc-muted">{t("stats-shared-encode")}</span>
                )}
              </span>
              <span className="text-havoc-muted">{target.state}</span>
              <span className="font-semibold tabular-nums">
                {target.state === "live" || target.state === "reconnecting"
                  ? `${target.kbps} kbps`
                  : "—"}
              </span>
            </li>
          ))}
        </ul>
      )}
      {stats?.placeholder && (
        <div className="mt-2">
          <EmptyHint>{t("stats-starting")}</EmptyHint>
        </div>
      )}
    </Panel>
  );
}
