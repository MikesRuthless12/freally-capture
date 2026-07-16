import type { AudioFilter } from "../api/types";
import {
  frequencyCurve,
  TRANSFER_MIN_DB,
  toDb,
  transferCurve,
  transferOut,
  vizKind,
} from "../lib/filterResponse";

/** One filter's live meter (linear in/out peaks) from the `audio` event. */
export type FilterMeter = { inPeak: number; outPeak: number } | undefined;

const W = 168;
const H = 108;
const PAD = 8;
const PW = W - PAD * 2;
const PH = H - PAD * 2;

// Frequency plot range (mirrors the EQ analyzer).
const F_MIN = 30;
const F_MAX = 16000;
const FREQS = Array.from({ length: 96 }, (_, i) => F_MIN * Math.pow(F_MAX / F_MIN, i / 95));
// Transfer plot: output dB range.
const OUT_MIN = -60;
const OUT_MAX = 6;

const clamp = (v: number, lo: number, hi: number) => Math.min(hi, Math.max(lo, v));

/** Shared dark analyzer surface + defs (accent gradient + glow). */
function Surface({ id, children }: { id: string; children: React.ReactNode }) {
  return (
    <svg viewBox={`0 0 ${W} ${H}`} className="shrink-0 text-havoc-accent" role="img" aria-hidden>
      <defs>
        <linearGradient id={`${id}-surface`} x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%" stopColor="#111a2e" />
          <stop offset="100%" stopColor="#070b14" />
        </linearGradient>
        <linearGradient id={`${id}-fill`} x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%" stopColor="currentColor" stopOpacity="0.34" />
          <stop offset="100%" stopColor="currentColor" stopOpacity="0.02" />
        </linearGradient>
        <filter id={`${id}-glow`} x="-30%" y="-30%" width="160%" height="160%">
          <feGaussianBlur stdDeviation="1.3" result="b" />
          <feMerge>
            <feMergeNode in="b" />
            <feMergeNode in="SourceGraphic" />
          </feMerge>
        </filter>
      </defs>
      <rect x="1" y="1" width={W - 2} height={H - 2} rx="8" fill={`url(#${id}-surface)`} />
      {children}
    </svg>
  );
}

function TransferPlot({ filter, meter }: { filter: AudioFilter; meter: FilterMeter }) {
  const xOf = (inDb: number) =>
    PAD + ((clamp(inDb, TRANSFER_MIN_DB, 0) - TRANSFER_MIN_DB) / -TRANSFER_MIN_DB) * PW;
  const yOf = (outDb: number) =>
    PAD + (1 - (clamp(outDb, OUT_MIN, OUT_MAX) - OUT_MIN) / (OUT_MAX - OUT_MIN)) * PH;
  const curve = transferCurve(filter)
    .map((p, i) => `${i === 0 ? "M" : "L"}${xOf(p.inDb).toFixed(1)},${yOf(p.outDb).toFixed(1)}`)
    .join(" ");
  const inDb = meter ? toDb(meter.inPeak) : null;
  const dot =
    inDb !== null && inDb > TRANSFER_MIN_DB
      ? { x: xOf(inDb), y: yOf(transferOut(filter, inDb)) }
      : null;
  return (
    <Surface id={`t-${filter.id}`}>
      {/* unity reference diagonal */}
      <line
        x1={xOf(-60)}
        y1={yOf(-60)}
        x2={xOf(0)}
        y2={yOf(0)}
        stroke="white"
        strokeOpacity={0.12}
        strokeDasharray="2 3"
      />
      {[-40, -20, 0].map((d) => (
        <line
          key={d}
          x1={xOf(d)}
          y1={PAD}
          x2={xOf(d)}
          y2={PAD + PH}
          stroke="white"
          strokeOpacity={0.05}
        />
      ))}
      <path
        d={curve}
        fill="none"
        stroke="currentColor"
        strokeWidth={2}
        strokeLinejoin="round"
        filter={`url(#t-${filter.id}-glow)`}
      />
      {dot && (
        <>
          <line
            x1={dot.x}
            y1={PAD}
            x2={dot.x}
            y2={PAD + PH}
            stroke="currentColor"
            strokeOpacity={0.3}
          />
          <circle
            cx={dot.x}
            cy={dot.y}
            r={3.5}
            fill="#ffffff"
            filter={`url(#t-${filter.id}-glow)`}
          />
        </>
      )}
    </Surface>
  );
}

function FrequencyPlot({ filter, meter }: { filter: AudioFilter; meter: FilterMeter }) {
  const G_MIN = -24;
  const G_MAX = 24;
  const xOf = (i: number) => PAD + (i / (FREQS.length - 1)) * PW;
  const yOf = (db: number) => PAD + (1 - (clamp(db, G_MIN, G_MAX) - G_MIN) / (G_MAX - G_MIN)) * PH;
  const curve = frequencyCurve(filter, FREQS)
    .map((db, i) => `${i === 0 ? "M" : "L"}${xOf(i).toFixed(1)},${yOf(db).toFixed(1)}`)
    .join(" ");
  const area = `${curve} L${xOf(FREQS.length - 1).toFixed(1)},${yOf(0).toFixed(1)} L${xOf(0).toFixed(1)},${yOf(0).toFixed(1)} Z`;
  return (
    <Surface id={`f-${filter.id}`}>
      <line x1={PAD} y1={yOf(0)} x2={PAD + PW} y2={yOf(0)} stroke="white" strokeOpacity={0.16} />
      {[100, 1000, 10000].map((hz) => {
        const t = Math.log(hz / F_MIN) / Math.log(F_MAX / F_MIN);
        return (
          <line
            key={hz}
            x1={PAD + t * PW}
            y1={PAD}
            x2={PAD + t * PW}
            y2={PAD + PH}
            stroke="white"
            strokeOpacity={0.05}
          />
        );
      })}
      <path d={area} fill={`url(#f-${filter.id}-fill)`} stroke="none" />
      <path
        d={curve}
        fill="none"
        stroke="currentColor"
        strokeWidth={2}
        strokeLinejoin="round"
        filter={`url(#f-${filter.id}-glow)`}
      />
      {meter && (
        <text
          x={W - PAD}
          y={PAD + 8}
          textAnchor="end"
          fontSize="7"
          fill="currentColor"
          fillOpacity={0.8}
        >
          {toDb(meter.outPeak).toFixed(0)} dB
        </text>
      )}
    </Surface>
  );
}

function LevelPlot({ filter, meter }: { filter: AudioFilter; meter: FilterMeter }) {
  // A friendly "what this does" readout + a big live in→out arc of level.
  let headline = "";
  if (filter.type === "gain") headline = `${filter.db >= 0 ? "+" : ""}${filter.db.toFixed(1)} dB`;
  else if (filter.type === "denoise") headline = `${Math.round(filter.strength * 100)}%`;
  else if (filter.type === "ducker") headline = `−${filter.amountDb.toFixed(0)} dB`;
  const inDb = meter ? toDb(meter.inPeak) : OUT_MIN;
  const outDb = meter ? toDb(meter.outPeak) : OUT_MIN;
  const barY = (db: number) =>
    PAD + (1 - (clamp(db, OUT_MIN, OUT_MAX) - OUT_MIN) / (OUT_MAX - OUT_MIN)) * PH;
  return (
    <Surface id={`l-${filter.id}`}>
      <text
        x={W / 2}
        y={H / 2 - 4}
        textAnchor="middle"
        fontSize="20"
        fontWeight="700"
        fill="currentColor"
        fillOpacity={0.9}
        filter={`url(#l-${filter.id}-glow)`}
      >
        {headline}
      </text>
      {/* live in/out level bars along the bottom */}
      <rect
        x={PAD}
        y={barY(inDb)}
        width={12}
        height={PAD + PH - barY(inDb)}
        rx={2}
        fill="white"
        fillOpacity={0.25}
      />
      <rect
        x={PAD + 16}
        y={barY(outDb)}
        width={12}
        height={PAD + PH - barY(outDb)}
        rx={2}
        fill="currentColor"
      />
      <text x={PAD + 6} y={H - 1} textAnchor="middle" fontSize="6" fill="white" fillOpacity={0.4}>
        in
      </text>
      <text
        x={PAD + 22}
        y={H - 1}
        textAnchor="middle"
        fontSize="6"
        fill="currentColor"
        fillOpacity={0.7}
      >
        out
      </text>
    </Surface>
  );
}

/** The live in / out / gain-reduction meter strip beside a graph. */
function MeterStrip({ meter }: { meter: FilterMeter }) {
  const inDb = meter ? toDb(meter.inPeak) : OUT_MIN;
  const outDb = meter ? toDb(meter.outPeak) : OUT_MIN;
  const gr =
    meter && meter.inPeak > 1e-4 ? Math.min(0, toDb(meter.outPeak) - toDb(meter.inPeak)) : 0;
  const pct = (db: number) => `${clamp(((db - OUT_MIN) / (OUT_MAX - OUT_MIN)) * 100, 0, 100)}%`;
  const grPct = `${clamp((-gr / 24) * 100, 0, 100)}%`;
  return (
    <div className="flex min-w-0 flex-1 flex-col justify-center gap-1.5 text-[9px] text-havoc-muted">
      <Bar label="IN" widthPct={pct(inDb)} className="bg-white/30" />
      <Bar label="OUT" widthPct={pct(outDb)} className="bg-havoc-accent" />
      <div className="flex items-center gap-1">
        <span className="w-6 shrink-0 tracking-wide">GR</span>
        <div className="relative h-2 flex-1 overflow-hidden rounded-full bg-white/[0.04]">
          <div
            className="absolute right-0 h-full rounded-full bg-amber-400/80"
            style={{ width: grPct }}
          />
        </div>
        <span className="w-9 shrink-0 text-right tabular-nums text-havoc-text">
          {gr.toFixed(1)}
        </span>
      </div>
    </div>
  );
}

function Bar({
  label,
  widthPct,
  className,
}: {
  label: string;
  widthPct: string;
  className: string;
}) {
  return (
    <div className="flex items-center gap-1">
      <span className="w-6 shrink-0 tracking-wide">{label}</span>
      <div className="relative h-2 flex-1 overflow-hidden rounded-full bg-white/[0.04]">
        <div
          className={`absolute left-0 h-full rounded-full transition-[width] duration-75 ${className}`}
          style={{ width: widthPct }}
        />
      </div>
    </div>
  );
}

/**
 * The plugin showpiece graphic for one filter: a param-driven transfer or
 * frequency-response curve (or a level readout) on a dark analyzer surface,
 * with the live meter beside it. `meter` arrives on the ~20 Hz `audio` event
 * while the filter editor is open, so the graph and bars move with the audio.
 */
export function FilterViz({ filter, meter }: { filter: AudioFilter; meter: FilterMeter }) {
  const kind = vizKind(filter.type);
  return (
    <div className="mb-2 flex items-stretch gap-2 rounded-lg border border-white/10 bg-black/20 p-1.5">
      {kind === "transfer" && <TransferPlot filter={filter} meter={meter} />}
      {kind === "frequency" && <FrequencyPlot filter={filter} meter={meter} />}
      {kind === "level" && <LevelPlot filter={filter} meter={meter} />}
      <MeterStrip meter={meter} />
    </div>
  );
}
