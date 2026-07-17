import { useEffect, useState } from "react";

import { onEncoderFallback, onStats, onStream } from "../api/events";
import type { EncoderFallback, StatsPayload, StreamStatus } from "../api/types";
import { EmptyHint, Panel } from "../components/Panel";
import { TimelineDialog } from "../components/TimelineDialog";
import { useT } from "../i18n/t";
import { formatBytes, formatDuration } from "../lib/format";

/** One stat readout tile. `title` is an optional hover tooltip. */
function Stat({ label, value, title }: { label: string; value: string; title?: string }) {
  return (
    <div className="rounded-lg border border-white/5 bg-white/[0.03] px-2.5 py-1.5" title={title}>
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
  // Encoder failover note (CAP-M12): sticky for the session — the operator
  // must be able to see AFTER the show why quality/CPU changed mid-way.
  const [fallback, setFallback] = useState<EncoderFallback | null>(null);
  // CAP-N50: the session-timeline dialog.
  const [timelineOpen, setTimelineOpen] = useState(false);

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
    onEncoderFallback((payload) => setFallback(payload))
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
        <Stat
          label={t("stats-disk")}
          value={
            stats?.diskFreeBytes != null
              ? `${formatBytes(stats.diskFreeBytes)} ${t("stats-disk-free")}`
              : "—"
          }
        />
        {stats?.secsUntilFull != null && stats.burnBytesPerSec != null && (
          <Stat
            label={t("stats-disk-left")}
            value={formatDuration(stats.secsUntilFull)}
            title={t("stats-disk-rate", {
              rate: (stats.burnBytesPerSec / 1e6).toFixed(1),
            })}
          />
        )}
        {(stats?.verticalFps ?? 0) > 0 && (
          <Stat label={t("stats-vertical-fps")} value={(stats?.verticalFps ?? 0).toFixed(0)} />
        )}
      </div>
      {stream?.rehearsal === true && targets.length > 0 && (
        <p
          role="status"
          className="m-0 mt-2 rounded-lg border border-violet-500/40 bg-violet-500/10 px-2.5 py-1.5 text-[11px] text-violet-300"
        >
          {t("stats-rehearsal-note")}
        </p>
      )}
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
      {fallback && (
        <p className="m-0 mt-2 rounded-lg border border-amber-400/30 bg-amber-400/10 px-2.5 py-1.5 text-[11px] text-amber-200">
          {t("fallback-note", { from: fallback.from, to: fallback.to })}
        </p>
      )}
      {stats?.placeholder && (
        <div className="mt-2">
          <EmptyHint>{t("stats-starting")}</EmptyHint>
        </div>
      )}
      {/* CAP-N50: the recorded, correlated session timeline (the dock shows
          now; the timeline explains the whole session afterwards). */}
      <button
        type="button"
        onClick={() => setTimelineOpen(true)}
        className="mt-2 self-start rounded-md border border-white/10 px-2 py-1 text-[11px] text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
      >
        {t("stats-timeline-open")}
      </button>
      {timelineOpen && <TimelineDialog onClose={() => setTimelineOpen(false)} />}
    </Panel>
  );
}
