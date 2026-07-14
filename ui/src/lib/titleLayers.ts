import type { TitleLayer } from "../api/types";

/** A fully-populated text layer (CAP-N16) — seeds the title starter templates
 * and the editor's “+ Text” button. */
export function titleTextLayer(
  overrides: Partial<Extract<TitleLayer, { kind: "text" }>> = {},
): Extract<TitleLayer, { kind: "text" }> {
  return {
    kind: "text",
    x: 0,
    y: 0,
    text: "",
    fontFamily: null,
    fontFile: null,
    sizePx: 48,
    color: { r: 255, g: 255, b: 255, a: 255 },
    align: "left",
    outlinePx: 0,
    outlineColor: { r: 0, g: 0, b: 0, a: 255 },
    shadow: false,
    sourceFile: "",
    binding: "whole",
    csvRow: 1,
    csvColumn: "",
    jsonPointer: "",
    ...overrides,
  };
}
