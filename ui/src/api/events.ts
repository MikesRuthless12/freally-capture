/**
 * Typed wrappers around core → UI push events.
 *
 * Event names + payloads mirror `src-tauri/src/events.rs`.
 */
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

import type { StatsPayload } from "./types";

/** Subscribe to the ~2 Hz `stats` event. Resolves to an unlisten function. */
export function onStats(handler: (stats: StatsPayload) => void): Promise<UnlistenFn> {
  return listen<StatsPayload>("stats", (event) => handler(event.payload));
}
