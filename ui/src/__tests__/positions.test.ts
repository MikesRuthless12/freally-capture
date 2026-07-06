import { describe, expect, it } from "vitest";

import { POSITION_PRESETS } from "../lib/positions";

describe("POSITION_PRESETS", () => {
  it("offers the six one-click seats", () => {
    expect(POSITION_PRESETS.map((preset) => preset.key)).toEqual([
      "topLeft",
      "topRight",
      "middleLeft",
      "middleRight",
      "bottomLeft",
      "bottomRight",
    ]);
  });

  it("every slot lies fully inside the canvas (the model rejects anything else)", () => {
    for (const { key, slot } of POSITION_PRESETS) {
      expect(slot.w, key).toBeGreaterThan(0);
      expect(slot.h, key).toBeGreaterThan(0);
      expect(slot.x, key).toBeGreaterThanOrEqual(0);
      expect(slot.y, key).toBeGreaterThanOrEqual(0);
      expect(slot.x + slot.w, key).toBeLessThanOrEqual(1);
      expect(slot.y + slot.h, key).toBeLessThanOrEqual(1);
    }
  });

  it("middle seats are vertically centered", () => {
    const middle = POSITION_PRESETS.filter((preset) => preset.key.startsWith("middle"));
    for (const { key, slot } of middle) {
      expect(slot.y + slot.h / 2, key).toBeCloseTo(0.5);
    }
  });
});
