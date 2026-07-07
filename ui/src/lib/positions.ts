import type { NormRect } from "../api/types";

/** Mirrors `crates/scene` SLOT_MARGIN / SLOT_SIZE — one corner-cam slot. */
const MARGIN = 0.02;
const SIZE = 0.3;
const FAR = 1 - MARGIN - SIZE;
const MID = 0.5 - SIZE / 2;

export type PositionPreset = {
  key: string;
  /** Human name (tooltips / aria). */
  label: string;
  /** Compact arrow glyph for the one-click button. */
  glyph: string;
  slot: NormRect;
};

/** The six one-click seats: top/middle/bottom × left/right. */
export const POSITION_PRESETS: PositionPreset[] = [
  {
    key: "topLeft",
    label: "top left",
    glyph: "↖",
    slot: { x: MARGIN, y: MARGIN, w: SIZE, h: SIZE },
  },
  {
    key: "topRight",
    label: "top right",
    glyph: "↗",
    slot: { x: FAR, y: MARGIN, w: SIZE, h: SIZE },
  },
  {
    key: "middleLeft",
    label: "middle left",
    glyph: "←",
    slot: { x: MARGIN, y: MID, w: SIZE, h: SIZE },
  },
  {
    key: "middleRight",
    label: "middle right",
    glyph: "→",
    slot: { x: FAR, y: MID, w: SIZE, h: SIZE },
  },
  {
    key: "bottomLeft",
    label: "bottom left",
    glyph: "↙",
    slot: { x: MARGIN, y: FAR, w: SIZE, h: SIZE },
  },
  {
    key: "bottomRight",
    label: "bottom right",
    glyph: "↘",
    slot: { x: FAR, y: FAR, w: SIZE, h: SIZE },
  },
];
