import { useEffect, useMemo, useRef, useState } from "react";

import { forensicTimeline } from "../api/commands";
import type { ForensicEvent, TimelineStatus } from "../api/types";
import { useT } from "../i18n/t";
import { PickerShell } from "./PickerShell";

/** Refresh cadence while the dialog is open and a session still records. */
const POLL_MS = 2_000;
const GRAPH_W = 720;
const GRAPH_H = 240;
const AXIS_H = 18;

const EVENT_COLOR: Record<ForensicEvent["kind"], string> = {
  alarm: "#f87171",
  "alarm-clear": "#9ca3af",
  fallback: "#fbbf24",
  reconnect: "#fbbf24",
  target: "#60a5fa",
  scene: "#22d3ee",
  recording: "#a78bfa",
  stream: "#a78bfa",
  marker: "#34d399",
};

const TARGET_COLORS = ["#60a5fa", "#34d399", "#f472b6", "#fbbf24", "#c084fc", "#f97316"];

function formatClock(ms: number): string {
  const total = Math.floor(ms / 1000);
  const h = Math.floor(total / 3600);
  const m = Math.floor((total % 3600) / 60);
  const s = total % 60;
  const mm = String(m).padStart(2, "0");
  const ss = String(s).padStart(2, "0");
  return h > 0 ? `${h}:${mm}:${ss}` : `${mm}:${ss}`;
}

/**
 * CAP-N50: the forensic session timeline — fps, encoder queue depth, and
 * per-target bitrate correlated with every discrete moment (scene switches,
 * alarms, reconnects, fallbacks) on one zoomable graph. Wheel zooms around
 * the cursor; drag pans; Fit resets.
 */
export function TimelineDialog({ onClose }: { onClose: () => void }) {
  const t = useT();
  const [data, setData] = useState<TimelineStatus | null>(null);
  // The visible window in session ms; null = fit the whole session.
  const [window_, setWindow] = useState<{ t0: number; t1: number } | null>(null);
  const dragging = useRef<{ startX: number; t0: number; t1: number } | null>(null);
  const svgRef = useRef<SVGSVGElement>(null);

  useEffect(() => {
    let alive = true;
    let timer: ReturnType<typeof setInterval> | undefined;
    const refresh = () => {
      forensicTimeline()
        .then((next) => {
          if (!alive) return;
          setData(next);
          if (!next.active && timer) {
            clearInterval(timer);
            timer = undefined;
          }
        })
        .catch(() => undefined);
    };
    refresh();
    timer = setInterval(refresh, POLL_MS);
    return () => {
      alive = false;
      if (timer) clearInterval(timer);
    };
  }, []);

  const session = data?.session;
  const span = useMemo(() => {
    if (!session || session.samples.length === 0) return null;
    const end = session.endedTMs ?? session.samples[session.samples.length - 1].tMs;
    return { t0: 0, t1: Math.max(end, 1000) };
  }, [session]);
  const view = window_ ?? span;

  // Y-axis scales + target identities are stable across zoom/pan, so they
  // depend only on the session — a drag must not rescan every sample.
  const scale = useMemo(() => {
    if (!session) return null;
    const targetIds = [...new Set(session.samples.flatMap((s) => s.targets.map((x) => x.id)))];
    return {
      maxKbps: Math.max(100, ...session.samples.flatMap((s) => s.targets.map((x) => x.kbps))),
      maxFps: Math.max(70, ...session.samples.map((s) => s.fps)),
      maxBehind: Math.max(10, ...session.samples.map((s) => s.framesBehind)),
      targets: targetIds.map((id, index) => ({
        id,
        color: TARGET_COLORS[index % TARGET_COLORS.length],
        label:
          session.samples.flatMap((s) => s.targets).find((x) => x.id === id)?.label ?? `#${id}`,
      })),
    };
  }, [session]);

  const graph = useMemo(() => {
    if (!session || !view || !scale) return null;
    const { maxKbps, maxFps, maxBehind } = scale;
    const inView = session.samples.filter((s) => s.tMs >= view.t0 && s.tMs <= view.t1);
    const x = (tMs: number) => ((tMs - view.t0) / (view.t1 - view.t0)) * GRAPH_W;
    const line = (pick: (s: (typeof inView)[number]) => number, max: number) =>
      inView
        .map(
          (s, i) =>
            `${i === 0 ? "M" : "L"}${x(s.tMs).toFixed(1)},${(GRAPH_H - (pick(s) / max) * GRAPH_H).toFixed(1)}`,
        )
        .join(" ");
    const targetLines = scale.targets.map((target) => {
      const d = inView
        .map((s) => ({ tMs: s.tMs, kbps: s.targets.find((x) => x.id === target.id)?.kbps }))
        .filter((p): p is { tMs: number; kbps: number } => p.kbps !== undefined)
        .map(
          (p, i) =>
            `${i === 0 ? "M" : "L"}${x(p.tMs).toFixed(1)},${(GRAPH_H - (p.kbps / maxKbps) * GRAPH_H).toFixed(1)}`,
        )
        .join(" ");
      return { ...target, d };
    });
    const events = session.events.filter((e) => e.tMs >= view.t0 && e.tMs <= view.t1);
    return {
      fpsLine: line((s) => s.fps, maxFps),
      behindLine: line((s) => s.framesBehind, maxBehind),
      targetLines,
      events,
      x,
    };
  }, [session, view, scale]);

  const zoom = (clientX: number, factor: number) => {
    if (!view || !svgRef.current) return;
    const rect = svgRef.current.getBoundingClientRect();
    const frac = Math.min(1, Math.max(0, (clientX - rect.left) / rect.width));
    const at = view.t0 + frac * (view.t1 - view.t0);
    const width = Math.max(2_000, (view.t1 - view.t0) * factor);
    let t0 = at - frac * width;
    let t1 = t0 + width;
    if (span) {
      if (t0 < span.t0) {
        t1 += span.t0 - t0;
        t0 = span.t0;
      }
      if (t1 > span.t1) {
        t0 = Math.max(span.t0, t0 - (t1 - span.t1));
        t1 = span.t1;
      }
    }
    setWindow({ t0, t1 });
  };

  return (
    <PickerShell title={t("timeline-title")} onClose={onClose} wide>
      {!session || session.samples.length === 0 ? (
        <p className="m-0 p-4 text-xs text-havoc-muted">{t("timeline-empty")}</p>
      ) : (
        <div className="flex flex-col gap-2 p-3">
          <div className="flex items-center gap-3 text-[11px] text-havoc-muted">
            {data?.active === true && (
              <span className="font-semibold text-red-300">{t("timeline-live")}</span>
            )}
            {session.rehearsal && (
              <span className="rounded bg-violet-500/15 px-1.5 py-px font-semibold text-violet-300">
                {t("livebutton-badge-rehearsal")}
              </span>
            )}
            <span className="text-emerald-300">— {t("timeline-legend-fps")}</span>
            <span className="text-rose-300">— {t("timeline-legend-behind")}</span>
            {graph?.targetLines.map((line) => (
              <span key={line.id} style={{ color: line.color }}>
                — {line.label} (kbps)
              </span>
            ))}
            <button
              type="button"
              onClick={() => setWindow(null)}
              className="ml-auto rounded border border-white/10 px-2 py-0.5 text-[11px] text-havoc-muted hover:text-havoc-text"
            >
              {t("timeline-fit")}
            </button>
          </div>
          <svg
            ref={svgRef}
            viewBox={`0 0 ${GRAPH_W} ${GRAPH_H + AXIS_H}`}
            className="w-full cursor-grab rounded-lg border border-white/10 bg-black/30 select-none"
            role="img"
            aria-label={t("timeline-title")}
            onWheel={(event) => zoom(event.clientX, event.deltaY > 0 ? 1.25 : 0.8)}
            onPointerDown={(event) => {
              if (!view) return;
              dragging.current = { startX: event.clientX, t0: view.t0, t1: view.t1 };
              (event.target as Element).setPointerCapture?.(event.pointerId);
            }}
            onPointerMove={(event) => {
              const drag = dragging.current;
              if (!drag || !svgRef.current || !span) return;
              const rect = svgRef.current.getBoundingClientRect();
              const dt = ((drag.startX - event.clientX) / rect.width) * (drag.t1 - drag.t0);
              let t0 = drag.t0 + dt;
              let t1 = drag.t1 + dt;
              if (t0 < span.t0) {
                t1 += span.t0 - t0;
                t0 = span.t0;
              }
              if (t1 > span.t1) {
                t0 -= t1 - span.t1;
                t1 = span.t1;
              }
              setWindow({ t0: Math.max(span.t0, t0), t1: Math.min(span.t1, t1) });
            }}
            onPointerUp={() => {
              dragging.current = null;
            }}
          >
            {graph && view && (
              <>
                <path d={graph.fpsLine} fill="none" stroke="#34d399" strokeWidth="1.5" />
                <path d={graph.behindLine} fill="none" stroke="#fda4af" strokeWidth="1" />
                {graph.targetLines.map((line) => (
                  <path
                    key={line.id}
                    d={line.d}
                    fill="none"
                    stroke={line.color}
                    strokeWidth="1.25"
                    strokeDasharray="4 2"
                  />
                ))}
                {graph.events.map((event, index) => (
                  <g key={`${event.tMs}-${index}`}>
                    <line
                      x1={graph.x(event.tMs)}
                      x2={graph.x(event.tMs)}
                      y1={0}
                      y2={GRAPH_H}
                      stroke={EVENT_COLOR[event.kind] ?? "#9ca3af"}
                      strokeWidth="1"
                      opacity="0.7"
                    />
                    <circle
                      cx={graph.x(event.tMs)}
                      cy={GRAPH_H - 6}
                      r="3.5"
                      fill={EVENT_COLOR[event.kind] ?? "#9ca3af"}
                    >
                      <title>{`${formatClock(event.tMs)} — ${event.kind}: ${event.label}`}</title>
                    </circle>
                  </g>
                ))}
                {/* Time axis: start, middle, end of the visible window. */}
                {[view.t0, (view.t0 + view.t1) / 2, view.t1].map((tick, index) => (
                  <text
                    key={index}
                    x={index === 0 ? 2 : index === 1 ? GRAPH_W / 2 : GRAPH_W - 2}
                    y={GRAPH_H + AXIS_H - 5}
                    fill="#9ca3af"
                    fontSize="10"
                    textAnchor={index === 0 ? "start" : index === 1 ? "middle" : "end"}
                  >
                    {formatClock(tick)}
                  </text>
                ))}
              </>
            )}
          </svg>
        </div>
      )}
    </PickerShell>
  );
}
