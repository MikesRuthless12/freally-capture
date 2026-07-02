/**
 * Typed wrappers around the Tauri command bridge.
 *
 * Command names + payloads mirror `src-tauri/src/commands/mod.rs`.
 */
import { invoke } from "@tauri-apps/api/core";

import type { Health, Settings } from "./types";

/** Bridge liveness probe: app version + linked core crates. */
export function health(): Promise<Health> {
  return invoke<Health>("health");
}

/** Read the current settings. */
export function settingsGet(): Promise<Settings> {
  return invoke<Settings>("settings_get");
}

/** Replace and persist the settings. */
export function settingsSet(settings: Settings): Promise<void> {
  return invoke("settings_set", { settings });
}
