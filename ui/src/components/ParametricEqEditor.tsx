import { useEffect, useRef, useState } from "react";
import type { UnlistenFn } from "@tauri-apps/api/event";

import { audioArmSpectrum } from "../api/commands";
import { onAudio } from "../api/events";
import type { EqBand, EqBandType, SourceId } from "../api/types";
import { eqCurveDb } from "../lib/eqResponse";
import { useT } from "../i18n/t";

const SR = 48000;
// These mirror crates/audio/src/spectrum.rs so the analyzer bins line up.
const F_MIN = 30;
const F_MAX = 16000;
const SPECTRUM_BINS = 48;
// Curve gain axis, and the spectrum's dBFS axis (drawn on the same box).
const G_MIN = -18;
const G_MAX = 18;
const S_MIN = -84;
const S_MAX = 0;
const MAX_BANDS = 16;

const W = 360;
const H = 180;
const PAD = { l: 6, r: 6, t: 8, b: 18 };
const PLOT_W = W - PAD.l - PAD.r;
const PLOT_H = H - PAD.t - PAD.b;

const BAND_TYPES: EqBandType[] = ["bell", "lowShelf", "highShelf", "notch", "highPass", "lowPass"];
const GAIN_TYPES: EqBandType[] = ["bell", "lowShelf", "highShelf"];
const hasGain = (type: EqBandType) => GAIN_TYPES.includes(type);

const clamp = (value: number, lo: number, hi: number) => Math.min(hi, Math.max(lo, value));

const freqToX = (f: number) =>
  PAD.l + (Math.log(clamp(f, F_MIN, F_MAX) / F_MIN) / Math.log(F_MAX / F_MIN)) * PLOT_W;
const xToFreq = (x: number) => F_MIN * Math.pow(F_MAX / F_MIN, (x - PAD.l) / PLOT_W);
const gainToY = (g: number) =>
  PAD.t + (1 - (clamp(g, G_MIN, G_MAX) - G_MIN) / (G_MAX - G_MIN)) * PLOT_H;
const yToGain = (y: number) => G_MAX - ((y - PAD.t) / PLOT_H) * (G_MAX - G_MIN);
const specToY = (db: number) =>
  PAD.t + (1 - (clamp(db, S_MIN, S_MAX) - S_MIN) / (S_MAX - S_MIN)) * PLOT_H;

const GRID_FREQS = [100, 1000, 10000];
const GRID_GAINS = [-12, 0, 12];
const CURVE_FREQS = Array.from({ length: 120 }, (_, i) => F_MIN * Math.pow(F_MAX / F_MIN, i / 119));

/**
 * CAP-N35 parametric-EQ editor: draggable band nodes over a live spectrum. The
 * curve is computed from the same RBJ math the engine runs; the spectrum arrives
 * on the `audio` event while this source is armed (armed on mount, cleared on
 * unmount).
 */
export function ParametricEqEditor({
  sourceId,
  bands,
  onChange,
}: {
  sourceId: SourceId;
  bands: EqBand[];
  onChange: (bands: EqBand[]) => void;
}) {
  const t = useT();
  const svgRef = useRef<SVGSVGElement>(null);
  const [dragging, setDragging] = useState<number | null>(null);
  const [spectrum, setSpectrum] = useState<number[] | null>(null);

  // Arm the live spectrum for this source while the editor is mounted.
  useEffect(() => {
    audioArmSpectrum(sourceId).catch(() => {});
    // `listen` resolves on a microtask; if teardown wins the race (StrictMode
    // remount, fast close) capture and call the unlisten so it can't leak.
    let alive = true;
    let unlisten: UnlistenFn | undefined;
    onAudio((levels) => {
      if (levels.spectrum && levels.spectrum.source === sourceId) {
        setSpectrum(levels.spectrum.magnitudes);
      }
    })
      .then((un) => {
        if (alive) unlisten = un;
        else un();
      })
      .catch(() => {});
    return () => {
      alive = false;
      audioArmSpectrum(null).catch(() => {});
      unlisten?.();
    };
  }, [sourceId]);

  const setBand = (index: number, patch: Partial<EqBand>) => {
    onChange(bands.map((band, i) => (i === index ? { ...band, ...patch } : band)));
  };
  const addBand = () => {
    if (bands.length >= MAX_BANDS) return;
    onChange([...bands, { type: "bell", freqHz: 1000, gainDb: 0, q: 1 }]);
  };
  const removeBand = (index: number) => onChange(bands.filter((_, i) => i !== index));

  const pointerToPlot = (event: React.PointerEvent) => {
    const rect = svgRef.current?.getBoundingClientRect();
    if (!rect) return null;
    return {
      x: ((event.clientX - rect.left) / rect.width) * W,
      y: ((event.clientY - rect.top) / rect.height) * H,
    };
  };

  const onNodeMove = (event: React.PointerEvent) => {
    if (dragging === null) return;
    const plot = pointerToPlot(event);
    if (!plot) return;
    const band = bands[dragging];
    const patch: Partial<EqBand> = { freqHz: Math.round(clamp(xToFreq(plot.x), F_MIN, F_MAX)) };
    if (hasGain(band.type)) {
      patch.gainDb = Math.round(clamp(yToGain(plot.y), G_MIN, G_MAX) * 2) / 2;
    }
    setBand(dragging, patch);
  };

  const curve = eqCurveDb(bands, CURVE_FREQS, SR);
  const curvePath = curve
    .map(
      (db, i) =>
        `${i === 0 ? "M" : "L"}${freqToX(CURVE_FREQS[i]).toFixed(1)},${gainToY(db).toFixed(1)}`,
    )
    .join(" ");
  const specPath = spectrum
    ? spectrum
        .map((db, i) => {
          const x = PAD.l + (i / (SPECTRUM_BINS - 1)) * PLOT_W;
          return `${i === 0 ? "M" : "L"}${x.toFixed(1)},${specToY(db).toFixed(1)}`;
        })
        .join(" ") +
      ` L${(PAD.l + PLOT_W).toFixed(1)},${(PAD.t + PLOT_H).toFixed(1)} L${PAD.l},${(PAD.t + PLOT_H).toFixed(1)} Z`
    : null;

  const specLine = spectrum
    ? spectrum
        .map((db, i) => {
          const x = PAD.l + (i / (SPECTRUM_BINS - 1)) * PLOT_W;
          return `${i === 0 ? "M" : "L"}${x.toFixed(1)},${specToY(db).toFixed(1)}`;
        })
        .join(" ")
    : null;

  return (
    <div className="mt-2 flex flex-col gap-2">
      <svg
        ref={svgRef}
        viewBox={`0 0 ${W} ${H}`}
        className="w-full text-havoc-accent"
        style={{ touchAction: "none" }}
        onPointerMove={onNodeMove}
        onPointerUp={() => setDragging(null)}
        onPointerLeave={() => setDragging(null)}
        role="img"
        aria-label={t("eq-graph-aria")}
      >
        <defs>
          {/* A fixed dark analyzer surface — reads the same in light + dark. */}
          <linearGradient id="eqSurface" x1="0" y1="0" x2="0" y2="1">
            <stop offset="0%" stopColor="#111a2e" />
            <stop offset="100%" stopColor="#070b14" />
          </linearGradient>
          {/* Spectrum fill: the theme accent fading to nothing. */}
          <linearGradient id="eqSpectrum" x1="0" y1="0" x2="0" y2="1">
            <stop offset="0%" stopColor="currentColor" stopOpacity="0.42" />
            <stop offset="100%" stopColor="currentColor" stopOpacity="0.02" />
          </linearGradient>
          <filter id="eqGlow" x="-30%" y="-30%" width="160%" height="160%">
            <feGaussianBlur stdDeviation="1.4" result="blur" />
            <feMerge>
              <feMergeNode in="blur" />
              <feMergeNode in="SourceGraphic" />
            </feMerge>
          </filter>
        </defs>

        {/* Plot surface */}
        <rect x="1" y="1" width={W - 2} height={H - 2} rx="8" fill="url(#eqSurface)" />

        {/* Grid + axis labels (neutral, subtle) */}
        {GRID_FREQS.map((f) => (
          <g key={f}>
            <line
              x1={freqToX(f)}
              y1={PAD.t}
              x2={freqToX(f)}
              y2={PAD.t + PLOT_H}
              stroke="white"
              strokeOpacity={0.06}
            />
            <text x={freqToX(f) + 3} y={H - 5} fill="white" fillOpacity={0.35} fontSize="7.5">
              {f >= 1000 ? `${f / 1000}k` : f}
            </text>
          </g>
        ))}
        {GRID_GAINS.map((g) => (
          <g key={g}>
            <line
              x1={PAD.l}
              y1={gainToY(g)}
              x2={PAD.l + PLOT_W}
              y2={gainToY(g)}
              stroke="white"
              strokeOpacity={g === 0 ? 0.18 : 0.06}
            />
            <text x={PAD.l + 2} y={gainToY(g) - 2} fill="white" fillOpacity={0.3} fontSize="7">
              {g > 0 ? `+${g}` : g}
            </text>
          </g>
        ))}

        {/* Live spectrum: gradient area + a bright trace line on top */}
        {specPath && <path d={specPath} fill="url(#eqSpectrum)" stroke="none" />}
        {specLine && (
          <path
            d={specLine}
            fill="none"
            stroke="currentColor"
            strokeOpacity={0.55}
            strokeWidth={0.9}
          />
        )}

        {/* EQ response curve — a glowing accent line */}
        <path
          d={curvePath}
          fill="none"
          stroke="currentColor"
          strokeWidth={2}
          strokeLinejoin="round"
          strokeLinecap="round"
          filter="url(#eqGlow)"
        />

        {/* Draggable band nodes: soft halo + solid core with a ring */}
        {bands.map((band, index) => {
          const cx = freqToX(band.freqHz);
          const cy = hasGain(band.type) ? gainToY(band.gainDb) : gainToY(0);
          const active = dragging === index;
          return (
            <g
              key={index}
              className="cursor-grab"
              onPointerDown={(event) => {
                (event.target as SVGGElement).setPointerCapture(event.pointerId);
                setDragging(index);
              }}
            >
              {/* Fat invisible hit target for easy grabbing */}
              <circle cx={cx} cy={cy} r={12} fill="transparent" />
              <circle
                cx={cx}
                cy={cy}
                r={active ? 11 : 8}
                fill="currentColor"
                opacity={active ? 0.28 : 0.16}
              />
              <circle
                cx={cx}
                cy={cy}
                r={active ? 6 : 5}
                fill="currentColor"
                stroke="#ffffff"
                strokeWidth={1.5}
                filter="url(#eqGlow)"
              />
              <text
                x={cx}
                y={cy + 2.5}
                textAnchor="middle"
                fontSize="6.5"
                fontWeight="700"
                fill="#0a0e17"
                pointerEvents="none"
              >
                {index + 1}
              </text>
            </g>
          );
        })}
      </svg>

      {/* Per-band controls */}
      <ul className="m-0 flex list-none flex-col gap-1 p-0">
        {bands.map((band, index) => (
          <li
            key={index}
            className="flex flex-wrap items-center gap-1.5 text-[10px] text-havoc-muted"
          >
            <select
              value={band.type}
              onChange={(event) => setBand(index, { type: event.target.value as EqBandType })}
              aria-label={t("eq-band-type")}
              className="rounded border border-white/10 bg-havoc-panel px-1 py-0.5 text-[10px] text-havoc-text"
            >
              {BAND_TYPES.map((type) => (
                <option key={type} value={type}>
                  {t(`eq-type-${type}`)}
                </option>
              ))}
            </select>
            <NumField
              label={t("eq-freq")}
              value={Math.round(band.freqHz)}
              min={20}
              max={20000}
              onChange={(freqHz) => setBand(index, { freqHz })}
            />
            <NumField
              label={t("eq-gain")}
              value={band.gainDb}
              min={-30}
              max={30}
              step={0.5}
              disabled={!hasGain(band.type)}
              onChange={(gainDb) => setBand(index, { gainDb })}
            />
            <NumField
              label={t("eq-q")}
              value={band.q}
              min={0.1}
              max={18}
              step={0.1}
              onChange={(q) => setBand(index, { q })}
            />
            <button
              type="button"
              onClick={() => removeBand(index)}
              aria-label={t("eq-remove-band")}
              className="rounded px-1 text-havoc-muted hover:text-red-400"
            >
              ×
            </button>
          </li>
        ))}
      </ul>
      <button
        type="button"
        onClick={addBand}
        disabled={bands.length >= MAX_BANDS}
        className="self-start rounded border border-white/10 px-2 py-0.5 text-[10px] text-havoc-muted transition-colors hover:border-havoc-accent/50 hover:text-havoc-text disabled:opacity-40"
      >
        {t("eq-add-band")}
      </button>
    </div>
  );
}

function NumField({
  label,
  value,
  min,
  max,
  step = 1,
  disabled,
  onChange,
}: {
  label: string;
  value: number;
  min: number;
  max: number;
  step?: number;
  disabled?: boolean;
  onChange: (value: number) => void;
}) {
  return (
    <label className="flex items-center gap-0.5">
      <span>{label}</span>
      <input
        type="number"
        value={value}
        min={min}
        max={max}
        step={step}
        disabled={disabled}
        onChange={(event) => onChange(clamp(Number(event.target.value), min, max))}
        aria-label={label}
        className="w-14 rounded border border-white/10 bg-havoc-panel px-1 py-0.5 text-[10px] text-havoc-text disabled:opacity-40"
      />
    </label>
  );
}
