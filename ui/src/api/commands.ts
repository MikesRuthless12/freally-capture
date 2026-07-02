/**
 * Typed wrappers around the Tauri command bridge.
 *
 * Command names + payloads mirror `src-tauri/src/commands/mod.rs`.
 */
import { invoke } from "@tauri-apps/api/core";

import type {
  CaptureSource,
  Health,
  PreviewSource,
  Settings,
  VideoDevice,
  VideoFormat,
} from "./types";

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

/**
 * Enumerate screen/window sources. On Wayland this returns exactly one
 * portal entry — the system dialog picks the real source.
 */
export function captureListSources(): Promise<CaptureSource[]> {
  return invoke<CaptureSource[]>("capture_list_sources");
}

/** Enumerate webcams / capture cards. */
export function videoDevicesList(): Promise<VideoDevice[]> {
  return invoke<VideoDevice[]>("video_devices_list");
}

/** List a video device's formats (opens the device briefly). */
export function videoDeviceFormats(deviceId: string): Promise<VideoFormat[]> {
  return invoke<VideoFormat[]>("video_device_formats", { deviceId });
}

/**
 * Start previewing a source. Resolves immediately; progress arrives on the
 * `preview` event keyed by `sourceKey`.
 */
export function previewStart(source: PreviewSource, sourceKey: string): Promise<void> {
  return invoke("preview_start", { source, sourceKey });
}

/** Stop the running preview (idempotent). */
export function previewStop(): Promise<void> {
  return invoke("preview_stop");
}

/** macOS only: open the Privacy pane ("screenRecording" | "camera"). */
export function openPrivacySettings(pane: "screenRecording" | "camera"): Promise<void> {
  return invoke("open_privacy_settings", { pane });
}
