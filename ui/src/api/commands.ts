/**
 * Typed wrappers around the Tauri command bridge.
 *
 * Command names + payloads mirror `src-tauri/src/commands/` (JS sends
 * camelCase argument names; Tauri maps them onto snake_case parameters).
 */
import { invoke } from "@tauri-apps/api/core";

import type {
  AddedItem,
  AudioDevice,
  AudioFilterId,
  AudioFilterKind,
  BlendMode,
  CaptureSource,
  CornerSlot,
  EncoderCatalog,
  FfmpegStatus,
  FilterId,
  FilterKind,
  Health,
  ItemId,
  LoopbackDevices,
  MonitorMode,
  RecordingFile,
  RecordingStatus,
  SceneId,
  Settings,
  SourceId,
  SourceSettings,
  StudioDto,
  Transform,
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

/** macOS only: open the Privacy pane ("screenRecording" | "camera"). */
export function openPrivacySettings(pane: "screenRecording" | "camera"): Promise<void> {
  return invoke("open_privacy_settings", { pane });
}

// ---------------------------------------------------------------------------
// The studio (Phase 2): scenes, items, sources, filters
// ---------------------------------------------------------------------------

/** The whole current model (initial load). */
export function studioGet(): Promise<StudioDto> {
  return invoke<StudioDto>("studio_get");
}

export function studioAddScene(name: string): Promise<SceneId> {
  return invoke<SceneId>("studio_add_scene", { name });
}

export function studioRenameScene(sceneId: SceneId, name: string): Promise<void> {
  return invoke("studio_rename_scene", { sceneId, name });
}

export function studioRemoveScene(sceneId: SceneId): Promise<void> {
  return invoke("studio_remove_scene", { sceneId });
}

export function studioSelectScene(sceneId: SceneId): Promise<void> {
  return invoke("studio_select_scene", { sceneId });
}

export function studioReorderScene(sceneId: SceneId, toIndex: number): Promise<void> {
  return invoke("studio_reorder_scene", { sceneId, toIndex });
}

/** Add a brand-new source on top of a scene. */
export function studioAddItem(
  sceneId: SceneId,
  settings: SourceSettings,
  name?: string,
): Promise<AddedItem> {
  return invoke<AddedItem>("studio_add_item", { sceneId, name: name ?? null, settings });
}

/** Place an existing pool source on top of a scene (source sharing). */
export function studioAddExistingSource(sceneId: SceneId, sourceId: SourceId): Promise<ItemId> {
  return invoke<ItemId>("studio_add_existing_source", { sceneId, sourceId });
}

export function studioRemoveItem(sceneId: SceneId, itemId: ItemId): Promise<void> {
  return invoke("studio_remove_item", { sceneId, itemId });
}

export function studioReorderItem(
  sceneId: SceneId,
  itemId: ItemId,
  toIndex: number,
): Promise<void> {
  return invoke("studio_reorder_item", { sceneId, itemId, toIndex });
}

export function studioSetItemTransform(
  sceneId: SceneId,
  itemId: ItemId,
  transform: Transform,
): Promise<void> {
  return invoke("studio_set_item_transform", { sceneId, itemId, transform });
}

export function studioSetItemVisible(
  sceneId: SceneId,
  itemId: ItemId,
  visible: boolean,
): Promise<void> {
  return invoke("studio_set_item_visible", { sceneId, itemId, visible });
}

export function studioSetItemLocked(
  sceneId: SceneId,
  itemId: ItemId,
  locked: boolean,
): Promise<void> {
  return invoke("studio_set_item_locked", { sceneId, itemId, locked });
}

export function studioSetItemBlend(
  sceneId: SceneId,
  itemId: ItemId,
  blend: BlendMode,
): Promise<void> {
  return invoke("studio_set_item_blend", { sceneId, itemId, blend });
}

/**
 * Arrange the scene as a centered screen with up to four corner cameras.
 * `center` becomes the backdrop (bottom of z-order); each corner item fits
 * into its slot on top. Placement resolves on each source's next sized frame.
 */
export function studioApplyLayout(
  sceneId: SceneId,
  center: ItemId | null,
  corners: CornerSlot[],
): Promise<void> {
  return invoke("studio_apply_layout", { sceneId, center, corners });
}

export function studioRenameSource(sourceId: SourceId, name: string): Promise<void> {
  return invoke("studio_rename_source", { sourceId, name });
}

/** Replace a source's settings; the engine restarts it on its next tick. */
export function studioUpdateSourceSettings(
  sourceId: SourceId,
  settings: SourceSettings,
): Promise<void> {
  return invoke("studio_update_source_settings", { sourceId, settings });
}

/** Restart an errored source with unchanged settings (replugged camera…). */
export function studioRetrySource(sourceId: SourceId): Promise<void> {
  return invoke("studio_retry_source", { sourceId });
}

export function studioAddFilter(
  sceneId: SceneId,
  itemId: ItemId,
  kind: FilterKind,
): Promise<FilterId> {
  return invoke<FilterId>("studio_add_filter", { sceneId, itemId, kind });
}

export function studioRemoveFilter(
  sceneId: SceneId,
  itemId: ItemId,
  filterId: FilterId,
): Promise<void> {
  return invoke("studio_remove_filter", { sceneId, itemId, filterId });
}

export function studioReorderFilter(
  sceneId: SceneId,
  itemId: ItemId,
  filterId: FilterId,
  toIndex: number,
): Promise<void> {
  return invoke("studio_reorder_filter", { sceneId, itemId, filterId, toIndex });
}

export function studioUpdateFilter(
  sceneId: SceneId,
  itemId: ItemId,
  filterId: FilterId,
  kind: FilterKind,
): Promise<void> {
  return invoke("studio_update_filter", { sceneId, itemId, filterId, kind });
}

export function studioSetFilterEnabled(
  sceneId: SceneId,
  itemId: ItemId,
  filterId: FilterId,
  enabled: boolean,
): Promise<void> {
  return invoke("studio_set_filter_enabled", { sceneId, itemId, filterId, enabled });
}

// ---------------------------------------------------------------------------
// Audio (Phase 3): devices + the per-source mixer state
// ---------------------------------------------------------------------------

/** Capture devices (microphones / line-in). */
export function audioInputDevices(): Promise<AudioDevice[]> {
  return invoke<AudioDevice[]>("audio_input_devices");
}

/** Playback devices (the monitor picker). */
export function audioOutputDevices(): Promise<AudioDevice[]> {
  return invoke<AudioDevice[]>("audio_output_devices");
}

/** Desktop-audio capture candidates + the honest per-OS guidance. */
export function audioLoopbackDevices(): Promise<LoopbackDevices> {
  return invoke<LoopbackDevices>("audio_loopback_devices");
}

export function studioSetAudioVolume(sourceId: SourceId, volumeDb: number): Promise<void> {
  return invoke("studio_set_audio_volume", { sourceId, volumeDb });
}

export function studioSetAudioMuted(sourceId: SourceId, muted: boolean): Promise<void> {
  return invoke("studio_set_audio_muted", { sourceId, muted });
}

export function studioSetAudioMonitor(sourceId: SourceId, monitor: MonitorMode): Promise<void> {
  return invoke("studio_set_audio_monitor", { sourceId, monitor });
}

/** Set the track-assignment bitmask (bit 0 = track 1). */
export function studioSetAudioTracks(sourceId: SourceId, tracks: number): Promise<void> {
  return invoke("studio_set_audio_tracks", { sourceId, tracks });
}

export function studioSetAudioSyncOffset(sourceId: SourceId, syncOffsetMs: number): Promise<void> {
  return invoke("studio_set_audio_sync_offset", { sourceId, syncOffsetMs });
}

/** Bind/clear PTT + PTM hotkeys (accelerator strings, e.g. "Ctrl+Shift+T"). */
export function studioSetAudioHotkeys(
  sourceId: SourceId,
  pushToTalk: string | null,
  pushToMute: string | null,
): Promise<void> {
  return invoke("studio_set_audio_hotkeys", { sourceId, pushToTalk, pushToMute });
}

export function studioAddAudioFilter(
  sourceId: SourceId,
  kind: AudioFilterKind,
): Promise<AudioFilterId> {
  return invoke<AudioFilterId>("studio_add_audio_filter", { sourceId, kind });
}

export function studioRemoveAudioFilter(
  sourceId: SourceId,
  filterId: AudioFilterId,
): Promise<void> {
  return invoke("studio_remove_audio_filter", { sourceId, filterId });
}

export function studioReorderAudioFilter(
  sourceId: SourceId,
  filterId: AudioFilterId,
  toIndex: number,
): Promise<void> {
  return invoke("studio_reorder_audio_filter", { sourceId, filterId, toIndex });
}

export function studioUpdateAudioFilter(
  sourceId: SourceId,
  filterId: AudioFilterId,
  kind: AudioFilterKind,
): Promise<void> {
  return invoke("studio_update_audio_filter", { sourceId, filterId, kind });
}

export function studioSetAudioFilterEnabled(
  sourceId: SourceId,
  filterId: AudioFilterId,
  enabled: boolean,
): Promise<void> {
  return invoke("studio_set_audio_filter_enabled", { sourceId, filterId, enabled });
}

// ---------------------------------------------------------------------------
// Encoders + the on-demand ffmpeg component (Phase 4)
// ---------------------------------------------------------------------------

/**
 * Detect the encoder catalog (hardware probe; verified against the
 * installed ffmpeg component when present). First call can take ~1 s.
 */
export function encodersList(): Promise<EncoderCatalog> {
  return invoke<EncoderCatalog>("encoders_list");
}

/** The ffmpeg component's current status. */
export function ffmpegStatus(): Promise<FfmpegStatus> {
  return invoke<FfmpegStatus>("ffmpeg_status");
}

/** Start the on-demand fetch + verify (progress rides the `ffmpeg` event). */
export function ffmpegInstall(): Promise<void> {
  return invoke("ffmpeg_install");
}

/** Cancel an in-flight fetch (the partial download is removed). */
export function ffmpegCancel(): Promise<void> {
  return invoke("ffmpeg_cancel");
}

/** Remove the installed component. */
export function ffmpegRemove(): Promise<void> {
  return invoke("ffmpeg_remove");
}

// ---------------------------------------------------------------------------
// Recording (Phase 4)
// ---------------------------------------------------------------------------

/** Start recording with the persisted Settings → Output configuration. */
export function recordingStart(): Promise<void> {
  return invoke("recording_start");
}

/** Stop + finalize; resolves to the finished file paths. */
export function recordingStop(): Promise<string[]> {
  return invoke<string[]>("recording_stop");
}

/** Pause the running recording (the file stays one contiguous timeline). */
export function recordingPause(): Promise<void> {
  return invoke("recording_pause");
}

/** Resume a paused recording. */
export function recordingResume(): Promise<void> {
  return invoke("recording_resume");
}

/** The current recording status snapshot. */
export function recordingStatus(): Promise<RecordingStatus> {
  return invoke<RecordingStatus>("recording_status");
}

/** The recordings folder's media files, newest first. */
export function recordingsList(): Promise<RecordingFile[]> {
  return invoke<RecordingFile[]>("recordings_list");
}

/** Remux an mkv recording to a sibling mp4 (stream copy, no re-encode). */
export function recordingRemux(path: string): Promise<string> {
  return invoke<string>("recording_remux", { path });
}
