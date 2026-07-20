import type { InputLayout, ReplaySpeed, SocialPlatform, SocialRow, VisStyle } from "../api/types";

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

/** Every audio container the audio pickers open (the "music lane" playlist
 * and the soundboard alike — one list, no drift). */
export const AUDIO_EXTS = ["mp3", "wav", "m4a", "aac", "ogg", "flac", "opus"];

/** V1-C: the countdown slate's seed colours — one definition, so the slate
 * looks identical whether quick-added (Starting Soon) or switched to in
 * Properties. */
export const SLATE_SOLID = { r: 16, g: 20, b: 26, a: 255 };
export const SLATE_GRADIENT_FROM = { r: 18, g: 20, b: 42, a: 255 };
export const SLATE_GRADIENT_TO = { r: 74, g: 26, b: 92, a: 255 };

/** V1-D: the default social-bar row (also the Add form's seed — one
 * definition of the seed colour, shared so the two stay in lockstep). */
export function newSocialRow(): SocialRow {
  return { platform: "youtube", handle: "", label: "", color: { r: 74, g: 158, b: 255, a: 255 } };
}

/** V1-D social platforms, in menu order. Brand names are proper nouns (not
 * localized); `custom` is the only i18n-labelled entry. */
export const SOCIAL_PLATFORMS: Array<[SocialPlatform, string]> = [
  ["youtube", "YouTube"],
  ["twitch", "Twitch"],
  ["kick", "Kick"],
  ["twitter", "X (Twitter)"],
  ["instagram", "Instagram"],
  ["tiktok", "TikTok"],
  ["facebook", "Facebook"],
  ["discord", "Discord"],
  ["custom", "sources-social-custom"],
];
