/**
 * The CSS-variable theme provider (TASK-906).
 *
 * `styles/global.css` declares the Havoc palette as Tailwind `@theme` custom
 * properties (`--color-havoc-bg`, `--color-havoc-accent`, …). Every class in the
 * app resolves through those, so overriding them on `<html>` repaints the whole
 * studio on the next frame — no rebuild, no reload, no React re-render.
 *
 * Only the variables that actually differ are set. Anything unset falls through
 * to `global.css`, so the dark theme costs nothing and a future variable added
 * to the stylesheet does not need touching here.
 */

/** Mirrors `ThemeMode` in `src-tauri/src/settings.rs`. */
export type ThemeMode = "dark" | "light" | "custom";

/** A full authored palette (CAP-N65): the six theme colours. Mirrors
 * `CustomPalette` in settings.rs. */
export type CustomPalette = {
  bg: string;
  panel: string;
  accent: string;
  accent2: string;
  text: string;
  muted: string;
};

export type Theme = {
  mode: ThemeMode;
  /** `#rrggbb`. Rust's `validate()` rejects anything else. */
  accent: string;
  /** CAP-N65: a full authored palette, applied when `mode` is `custom`. */
  palette?: CustomPalette;
};

export const DEFAULT_THEME: Theme = { mode: "dark", accent: "#4a9eff" };

/** The shipped dark palette (matches `global.css` @theme) — the editor's
 * starting point when authoring a custom palette. */
export const DARK_PALETTE: CustomPalette = {
  bg: "#0a0a0b",
  panel: "#1a1a1d",
  accent: "#4a9eff",
  accent2: "#00d4ff",
  text: "#e8e8ea",
  muted: "#8b8b93",
};

/** The variables `global.css` defines. Setting a subset is fine. */
type Palette = Partial<{
  "--color-havoc-bg": string;
  "--color-havoc-panel": string;
  "--color-havoc-accent": string;
  "--color-havoc-accent-2": string;
  "--color-havoc-text": string;
  "--color-havoc-muted": string;
}>;

const LIGHT: Palette = {
  "--color-havoc-bg": "#f4f4f6",
  "--color-havoc-panel": "#ffffff",
  "--color-havoc-text": "#16161a",
  "--color-havoc-muted": "#5c5c66",
};

/**
 * A hex triple → the same hue at a second stop, for the accent gradient. The
 * app's gradients run `accent → accent-2`; with one user-chosen colour we
 * lighten it rather than inventing an unrelated second hue.
 */
export function lighten(hex: string, amount = 0.25): string {
  const value = hex.replace("#", "");
  if (value.length !== 6) return hex;
  const channels = [0, 2, 4].map((i) => parseInt(value.slice(i, i + 2), 16));
  const lifted = channels.map((c) => Math.round(c + (255 - c) * amount));
  return "#" + lifted.map((c) => c.toString(16).padStart(2, "0")).join("");
}

/**
 * Reject anything that is not `#rrggbb` before it reaches a CSS declaration.
 * Rust validates on save; this guards the in-memory path (a corrupt settings
 * file, a stale profile) so a bad value can never reach `style.setProperty`.
 */
export function isHexColor(value: string): boolean {
  return /^#[0-9a-fA-F]{6}$/.test(value);
}

function paletteFor(theme: Theme): Palette {
  switch (theme?.mode) {
    case "light":
      return LIGHT;
    case "custom": {
      // A full authored palette (CAP-N65) wins; otherwise fall back to the
      // accent-only Custom (dark base + one chosen accent).
      const p = theme.palette;
      if (p && isFullPalette(p)) {
        return {
          "--color-havoc-bg": p.bg,
          "--color-havoc-panel": p.panel,
          "--color-havoc-accent": p.accent,
          "--color-havoc-accent-2": p.accent2,
          "--color-havoc-text": p.text,
          "--color-havoc-muted": p.muted,
        };
      }
      if (!isHexColor(theme.accent)) return {};
      return {
        "--color-havoc-accent": theme.accent,
        "--color-havoc-accent-2": lighten(theme.accent),
      };
    }
    case "dark":
    default:
      return {};
  }
}

/** Every colour in a custom palette is a valid hex triple. */
export function isFullPalette(p: CustomPalette): boolean {
  return [p.bg, p.panel, p.accent, p.accent2, p.text, p.muted].every(isHexColor);
}

/** WCAG relative luminance of a `#rrggbb` colour (0–1). */
function luminance(hex: string): number {
  const v = hex.replace("#", "");
  const chan = [0, 2, 4].map((i) => parseInt(v.slice(i, i + 2), 16) / 255);
  const lin = (c: number) => (c <= 0.03928 ? c / 12.92 : Math.pow((c + 0.055) / 1.055, 2.4));
  return 0.2126 * lin(chan[0]) + 0.7152 * lin(chan[1]) + 0.0722 * lin(chan[2]);
}

/** WCAG contrast ratio (1–21) between two `#rrggbb` colours. ≥4.5 passes AA for
 * normal text; ≥3 for large text / UI components. */
export function contrastRatio(a: string, b: string): number {
  if (!isHexColor(a) || !isHexColor(b)) return 1;
  const la = luminance(a);
  const lb = luminance(b);
  return (Math.max(la, lb) + 0.05) / (Math.min(la, lb) + 0.05);
}

/** Every variable this module ever sets, so switching themes clears the last. */
const ALL_VARS: Array<keyof Palette> = [
  "--color-havoc-bg",
  "--color-havoc-panel",
  "--color-havoc-accent",
  "--color-havoc-accent-2",
  "--color-havoc-text",
  "--color-havoc-muted",
];

/**
 * Paint `theme` onto `<html>`. Idempotent: every variable this module owns is
 * removed first, so going custom → dark does not leave the old accent behind.
 */
export function applyTheme(
  theme: Theme | undefined | null,
  root: HTMLElement | null = documentRoot(),
): void {
  if (!root) return;
  // A settings file written before 0.96.0 has no `theme`. Rust's `serde(default)`
  // supplies one, but a mocked bridge or a hand-edited file may not — and a throw
  // here lands in App's `.catch`, which leaves `settings` null and silently
  // disables every control that depends on it.
  const resolved: Theme = theme ?? DEFAULT_THEME;
  const palette = paletteFor(resolved);
  for (const name of ALL_VARS) {
    const value = palette[name];
    if (value === undefined) root.style.removeProperty(name);
    else root.style.setProperty(name, value);
  }
  root.dataset.theme = resolved.mode;
}

function documentRoot(): HTMLElement | null {
  return typeof document === "undefined" ? null : document.documentElement;
}
