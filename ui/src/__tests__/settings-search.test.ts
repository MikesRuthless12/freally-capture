import { describe, expect, it } from "vitest";

import { SETTINGS_KEYWORDS, categoryMatches } from "../panels/settingsSearch";

/**
 * CAP-N67 index coverage: every settings category is searchable. The
 * `Record<CategoryId, string[]>` type already forces an entry per category at
 * compile time (a new category with no keywords is a type error); this asserts
 * the values are non-empty and normalized at runtime, and that matching works.
 */
describe("settings search (CAP-N67)", () => {
  const categories = Object.keys(SETTINGS_KEYWORDS) as Array<keyof typeof SETTINGS_KEYWORDS>;

  it("indexes every category with at least one lowercase keyword", () => {
    expect(categories.length).toBeGreaterThanOrEqual(9);
    for (const category of categories) {
      expect(SETTINGS_KEYWORDS[category].length).toBeGreaterThan(0);
      for (const keyword of SETTINGS_KEYWORDS[category]) {
        expect(keyword.trim()).not.toBe("");
        expect(keyword).toBe(keyword.toLowerCase());
      }
    }
  });

  it("matches by keyword and by label; empty query shows all", () => {
    expect(categoryMatches("appearance", "Appearance", "theme")).toBe(true);
    expect(categoryMatches("hotkeys", "Hotkeys", "shortcut")).toBe(true);
    expect(categoryMatches("network", "Network", "osc")).toBe(true);
    expect(categoryMatches("streaming", "Streaming", "stream")).toBe(true);
    expect(categoryMatches("about", "About", "")).toBe(true);
    expect(categoryMatches("replay", "Replay", "zzzznope")).toBe(false);
  });
});
