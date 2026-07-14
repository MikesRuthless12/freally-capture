import type { HotkeySettings, LinkSettings, OscSettings } from "../api/types";

/**
 * Draft-editing helpers shared by the unified Settings modal and the
 * standalone per-panel dialogs. Lives outside the panel components so both
 * can share them without tripping react-refresh's only-export-components rule.
 */

/** Mirrors `OscSettings::default()` in osc.rs — the draft seed when a
 * pre-existing settings file lacks the slice. */
export const DEFAULT_OSC: OscSettings = { enabled: false, port: 9000, lan: false };

/** Mirrors `LinkSettings::default()` in link.rs. */
export const DEFAULT_LINK: LinkSettings = { enabled: false, port: 9720, name: "", key: "" };

/**
 * What Save actually persists: every accelerator trimmed, and "" → `null`
 * (unbound). Shared by the standalone Hotkeys dialog and the unified modal's
 * Apply so both hit the same hotkey re-registration path with the same shapes.
 */
export function normalizeHotkeys(draft: HotkeySettings): HotkeySettings {
  return {
    record: draft.record?.trim() || null,
    goLive: draft.goLive?.trim() || null,
    transition: draft.transition?.trim() || null,
    saveReplay: draft.saveReplay?.trim() || null,
    addMarker: draft.addMarker?.trim() || null,
    still: draft.still?.trim() || null,
    panic: draft.panic?.trim() || null,
    timerToggle: draft.timerToggle?.trim() || null,
    timerReset: draft.timerReset?.trim() || null,
    zoom100: draft.zoom100?.trim() || null,
    zoom150: draft.zoom150?.trim() || null,
    zoom200: draft.zoom200?.trim() || null,
    splitTimerSplit: draft.splitTimerSplit?.trim() || null,
    splitTimerUndo: draft.splitTimerUndo?.trim() || null,
    splitTimerSkip: draft.splitTimerSkip?.trim() || null,
    splitTimerReset: draft.splitTimerReset?.trim() || null,
    playlistNext: draft.playlistNext?.trim() || null,
    playlistPrevious: draft.playlistPrevious?.trim() || null,
    replayRoll: draft.replayRoll?.trim() || null,
  };
}
