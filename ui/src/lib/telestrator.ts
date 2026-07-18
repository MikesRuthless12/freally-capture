/**
 * Telestrator (CAP-N57): the UI-side drawing config + armed state.
 *
 * A tiny observable store (not in any one component) so the tool palette, the
 * preview overlay, and the menu toggle all share one source of truth. The marks
 * themselves live in Rust (streamed there and baked into the program); this only
 * holds what the operator is about to draw. Config persists in localStorage.
 */

import { hexToRgba } from "./color";

export type TeleToolKind = "pen" | "highlight" | "arrow" | "ellipse";

export type TelestratorConfig = {
  /** Whether drawing is active (the preview captures pointer events). */
  armed: boolean;
  tool: TeleToolKind;
  /** The stroke color as a hex string (#rrggbb). */
  color: string;
  /** Pen/arrow/ellipse line width as a fraction of canvas height. */
  width: number;
  /** Whiteboard mode: marks persist until cleared (else they fade). */
  whiteboard: boolean;
  /** Seconds a mark stays before it fades (ignored in whiteboard mode). */
  fadeSeconds: number;
};

/** The color swatches the palette offers. */
export const TELE_COLORS = [
  "#ff3b30",
  "#ffcc00",
  "#34c759",
  "#00c8ff",
  "#ffffff",
  "#111111",
] as const;

/** Selectable line widths (fraction of canvas height) — thin / medium / thick. */
export const TELE_WIDTHS = [0.004, 0.007, 0.012] as const;

/** A highlighter is translucent and fixed-width regardless of the pen width. */
const HIGHLIGHT_ALPHA = 0.4;
const HIGHLIGHT_WIDTH = 0.035;

const STORE_KEY = "telestrator.config";

const DEFAULT: TelestratorConfig = {
  armed: false,
  tool: "pen",
  color: "#ff3b30",
  width: TELE_WIDTHS[1],
  whiteboard: false,
  fadeSeconds: 4,
};

function load(): TelestratorConfig {
  try {
    const raw = window.localStorage.getItem(STORE_KEY);
    if (!raw) return { ...DEFAULT };
    const parsed = JSON.parse(raw) as Partial<TelestratorConfig>;
    return {
      ...DEFAULT,
      ...parsed,
      // Arming never persists — every session starts with drawing off.
      armed: false,
    };
  } catch {
    return { ...DEFAULT };
  }
}

let config: TelestratorConfig = load();
const listeners = new Set<(c: TelestratorConfig) => void>();

function persist(): void {
  try {
    // Arming is never persisted — every session starts with drawing off.
    const { tool, color, width, whiteboard, fadeSeconds } = config;
    window.localStorage.setItem(
      STORE_KEY,
      JSON.stringify({ tool, color, width, whiteboard, fadeSeconds }),
    );
  } catch {
    // Best-effort — config persistence is not load-bearing.
  }
}

export function telestratorSubscribe(listener: (c: TelestratorConfig) => void): () => void {
  listeners.add(listener);
  return () => {
    listeners.delete(listener);
  };
}

export function telestratorGet(): TelestratorConfig {
  return config;
}

export function telestratorSet(patch: Partial<TelestratorConfig>): void {
  config = { ...config, ...patch };
  persist();
  for (const listener of listeners) listener(config);
}

/** The straight-alpha RGBA a stroke should carry for the current config. */
export function strokeColor(cfg: TelestratorConfig): [number, number, number, number] {
  const { r, g, b } = hexToRgba(cfg.color);
  const a = cfg.tool === "highlight" ? HIGHLIGHT_ALPHA : 1;
  return [r / 255, g / 255, b / 255, a];
}

/** The stroke width (fraction of canvas height) for the current config. */
export function strokeWidth(cfg: TelestratorConfig): number {
  return cfg.tool === "highlight" ? HIGHLIGHT_WIDTH : cfg.width;
}

/** The fade window (seconds) for a new stroke, or null in whiteboard mode. */
export function strokeFade(cfg: TelestratorConfig): number | null {
  return cfg.whiteboard ? null : cfg.fadeSeconds;
}
