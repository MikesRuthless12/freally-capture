/**
 * Typed wrappers around core → UI push events.
 *
 * Event names + payloads mirror `src-tauri/src/events.rs` and
 * `src-tauri/src/studio.rs`.
 */
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

import type { ProgramStatus, StatsPayload, StudioDto } from "./types";

/** Subscribe to the ~2 Hz `stats` event. Resolves to an unlisten function. */
export function onStats(handler: (stats: StatsPayload) => void): Promise<UnlistenFn> {
  return listen<StatsPayload>("stats", (event) => handler(event.payload));
}

/** Subscribe to model changes — the full collection on every mutation. */
export function onStudio(handler: (studio: StudioDto) => void): Promise<UnlistenFn> {
  return listen<StudioDto>("studio", (event) => handler(event.payload));
}

/** Subscribe to compose-loop health + per-source states (≥1 Hz). */
export function onProgram(handler: (status: ProgramStatus) => void): Promise<UnlistenFn> {
  return listen<ProgramStatus>("program", (event) => handler(event.payload));
}
