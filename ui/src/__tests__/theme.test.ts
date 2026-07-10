import { afterEach, describe, expect, it } from "vitest";

import { applyTheme, DEFAULT_THEME, isHexColor, lighten } from "../theme/theme";

const root = () => document.documentElement;
const cssVar = (name: string) => root().style.getPropertyValue(name);

afterEach(() => {
  root().removeAttribute("style");
  root().removeAttribute("data-theme");
});

describe("isHexColor", () => {
  it("accepts only a plain #rrggbb triple", () => {
    expect(isHexColor("#4a9eff")).toBe(true);
    expect(isHexColor("#FFFFFF")).toBe(true);
  });

  /**
   * The accent is written straight into a CSS custom property. Anything that can
   * close the declaration injects a rule into the page. Rust's `validate()`
   * rejects these on save; this is the in-memory guard for a corrupt settings
   * file or a stale profile.
   */
  it("rejects anything that could escape a CSS declaration", () => {
    for (const bad of [
      "",
      "4a9eff",
      "#4a9ef",
      "#4a9efff",
      "#gggggg",
      "red",
      "#4a9eff;color:red",
      "#4a9eff}body{display:none",
      "var(--x)",
      "url(evil)",
    ]) {
      expect(isHexColor(bad), `${bad} must be rejected`).toBe(false);
    }
  });
});

describe("lighten", () => {
  it("moves a colour toward white without leaving hex", () => {
    expect(lighten("#000000", 0.5)).toBe("#808080");
    expect(lighten("#ffffff", 0.5)).toBe("#ffffff");
    expect(lighten("#4a9eff")).toMatch(/^#[0-9a-f]{6}$/);
  });

  it("returns the input untouched when it is not a triple", () => {
    expect(lighten("nope")).toBe("nope");
  });
});

describe("applyTheme", () => {
  it("dark sets no overrides — global.css already is the dark theme", () => {
    applyTheme({ mode: "dark", accent: "#4a9eff" });
    expect(cssVar("--color-havoc-bg")).toBe("");
    expect(cssVar("--color-havoc-accent")).toBe("");
    expect(root().dataset.theme).toBe("dark");
  });

  it("light overrides the surface colours", () => {
    applyTheme({ mode: "light", accent: "#4a9eff" });
    expect(cssVar("--color-havoc-bg")).toBe("#f4f4f6");
    expect(cssVar("--color-havoc-text")).toBe("#16161a");
    expect(root().dataset.theme).toBe("light");
  });

  it("custom sets the accent and derives its second stop", () => {
    applyTheme({ mode: "custom", accent: "#ff0000" });
    expect(cssVar("--color-havoc-accent")).toBe("#ff0000");
    expect(cssVar("--color-havoc-accent-2")).toBe(lighten("#ff0000"));
  });

  /**
   * Switching custom → dark must remove the accent, not leave it behind. This is
   * the bug every hand-rolled theme switcher ships: it sets variables but never
   * clears the ones the next theme doesn't define.
   */
  it("clears the variables a previous theme set", () => {
    applyTheme({ mode: "light", accent: "#4a9eff" });
    expect(cssVar("--color-havoc-bg")).toBe("#f4f4f6");

    applyTheme({ mode: "custom", accent: "#00ff00" });
    expect(cssVar("--color-havoc-bg"), "light's background must be gone").toBe("");
    expect(cssVar("--color-havoc-accent")).toBe("#00ff00");

    applyTheme(DEFAULT_THEME);
    expect(cssVar("--color-havoc-accent"), "custom's accent must be gone").toBe("");
    expect(cssVar("--color-havoc-bg")).toBe("");
  });

  it("ignores a custom accent that is not a hex triple", () => {
    applyTheme({ mode: "custom", accent: "red;color:blue" });
    expect(cssVar("--color-havoc-accent")).toBe("");
  });

  it("does nothing without a document", () => {
    expect(() => applyTheme(DEFAULT_THEME, null)).not.toThrow();
  });

  /**
   * A `settings.json` written before 0.96.0 has no `theme`. Rust's
   * `serde(default)` supplies one, but a hand-edited file or a mocked bridge may
   * not — and this used to throw inside `App`'s settings callback, land in its
   * `.catch`, and leave `settings` null. The studio then looked alive and
   * refused to save anything, because every control checks `settings`.
   */
  it("falls back to the default theme when settings predate it", () => {
    expect(() => applyTheme(undefined)).not.toThrow();
    expect(root().dataset.theme).toBe("dark");

    root().removeAttribute("data-theme");
    expect(() => applyTheme(null)).not.toThrow();
    expect(root().dataset.theme).toBe("dark");
  });
});
