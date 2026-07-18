import type { CategoryId } from "./Settings";

/**
 * CAP-N67 settings search index. Kept out of Settings.tsx so that component
 * file exports only components (Fast Refresh). The `Record<CategoryId, …>` type
 * forces an entry per category at compile time — a new category with no
 * keywords is a type error — and `__tests__/settings-search.test.ts` guards the
 * values at runtime.
 */
export const SETTINGS_KEYWORDS: Record<CategoryId, string[]> = {
  general: ["language", "locale", "stats dock", "alignment", "guides", "safe area", "rulers"],
  appearance: ["theme", "dark", "light", "accent", "color", "palette", "contrast", "fctheme"],
  streaming: ["stream", "twitch", "youtube", "rtmp", "bitrate", "stream key", "ingest", "encoder"],
  output: ["recording", "record", "encoder", "container", "fps", "format", "folder", "resolution"],
  replay: ["replay", "buffer", "instant replay", "clip"],
  hotkeys: ["hotkey", "shortcut", "keybinding", "bind", "push to talk", "ptt"],
  network: ["lan", "web panel", "osc", "midi", "tally", "remote api", "port", "control"],
  accessibility: ["meter", "vu", "contrast", "palette", "colour blind"],
  about: ["version", "update", "portable", "license", "diagnostics"],
};

/** Does `category` match the (lowercased) search query? Empty query matches all. */
export function categoryMatches(category: CategoryId, label: string, query: string): boolean {
  const q = query.trim().toLowerCase();
  if (!q) return true;
  return (
    label.toLowerCase().includes(q) || SETTINGS_KEYWORDS[category].some((kw) => kw.includes(q))
  );
}
