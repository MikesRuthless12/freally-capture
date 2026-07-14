import type { InputLayout, ReplaySpeed, VisStyle } from "../api/types";

/** The three CAP-N10 roll speeds, in menu order. Values are i18n keys. */
export const REPLAY_SPEEDS: Array<[ReplaySpeed, string]> = [
  ["full", "sources-replay-speed-full"],
  ["half", "sources-replay-speed-half"],
  ["quarter", "sources-replay-speed-quarter"],
];

/** The three CAP-N15 visualizer faces, in menu order. Values are i18n keys. */
export const VIS_STYLES: Array<[VisStyle, string]> = [
  ["bars", "sources-visualizer-style-bars"],
  ["scope", "sources-visualizer-style-scope"],
  ["vu", "sources-visualizer-style-vu"],
];

/** The four CAP-N13 layout presets, in menu order. Values are i18n keys. */
export const INPUT_LAYOUTS: Array<[InputLayout, string]> = [
  ["wasd", "sources-input-layout-wasd"],
  ["keyboard", "sources-input-layout-keyboard"],
  ["gamepad", "sources-input-layout-gamepad"],
  ["fightstick", "sources-input-layout-fightstick"],
];
