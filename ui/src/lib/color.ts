import type { Rgba } from "../api/types";

/** `#rrggbb` → straight RGBA (alpha preserved separately). */
export function hexToRgba(hex: string, alpha = 255): Rgba {
  const value = hex.replace("#", "");
  return {
    r: parseInt(value.slice(0, 2), 16) || 0,
    g: parseInt(value.slice(2, 4), 16) || 0,
    b: parseInt(value.slice(4, 6), 16) || 0,
    a: alpha,
  };
}

/** RGBA → `#rrggbb` (alpha handled by the caller's control). */
export function rgbaToHex(rgba: Rgba): string {
  const channel = (v: number) => v.toString(16).padStart(2, "0");
  return `#${channel(rgba.r)}${channel(rgba.g)}${channel(rgba.b)}`;
}
