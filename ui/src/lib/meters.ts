import type { AccessibilitySettings } from "../api/types";

/**
 * The mixer VU meter palette (Settings → Accessibility). The meter is one
 * low→mid→high sweep; these three colours recolor its zones. Lives outside
 * the components so both the meter (ChannelStrip) and the Settings pane can
 * share it without tripping react-refresh's only-export-components rule.
 */
export type MeterColors = { low: string; mid: string; high: string };

/** The green→yellow→red sweep every build so far has drawn. */
export const DEFAULT_METER_COLORS: MeterColors = {
  low: "#22c55e",
  mid: "#eab308",
  high: "#ef4444",
};

/** Okabe–Ito blue→orange→vermillion — readable under red-green CVD. */
const COLORBLIND_METER_COLORS: MeterColors = {
  low: "#0072b2",
  mid: "#e69f00",
  high: "#d55e00",
};

/** `#rrggbb` only — these land in a CSS gradient (same guard as theme.ts). */
const isMeterHex = (value: string) => /^#[0-9a-fA-F]{6}$/.test(value);

/**
 * Settings → Accessibility → the meter palette. Rust validates on save; the
 * per-channel hex guard covers the in-memory path (a corrupt settings file, a
 * stale profile) so a bad value never reaches `style.background`.
 */
export function resolveMeterColors(accessibility: AccessibilitySettings | undefined): MeterColors {
  switch (accessibility?.meterPreset) {
    case "colorblind":
      return COLORBLIND_METER_COLORS;
    case "custom":
      return {
        low: isMeterHex(accessibility.meterLow) ? accessibility.meterLow : DEFAULT_METER_COLORS.low,
        mid: isMeterHex(accessibility.meterMid) ? accessibility.meterMid : DEFAULT_METER_COLORS.mid,
        high: isMeterHex(accessibility.meterHigh)
          ? accessibility.meterHigh
          : DEFAULT_METER_COLORS.high,
      };
    default:
      return DEFAULT_METER_COLORS;
  }
}

/** The low→mid→high gradient a level bar reveals, shared by both meters. */
export function meterGradient(direction: "to right" | "to top", colors: MeterColors): string {
  const { low, mid, high } = colors;
  return `linear-gradient(${direction}, ${low} 0%, ${low} 62%, ${mid} 78%, ${high} 95%)`;
}
