/**
 * Typed wrappers around the Tauri command bridge.
 *
 * Command names + payloads mirror `src-tauri/src/commands/` (JS sends
 * camelCase argument names; Tauri maps them onto snake_case parameters).
 */
import { invoke } from "@tauri-apps/api/core";

import type {
  AddedItem,
  AutoConfig,
  BuildInfo,
  AppAudioList,
  AudioDevice,
  AudioFilterId,
  AudioFilterKind,
  BackdropSplit,
  BlendMode,
  BugReportContext,
  CalibrationResult,
  CalibrationStatus,
  CameraControl,
  CaptureSource,
  CefStatus,
  CornerSlot,
  CursorFxSetting,
  DisplayInfo,
  EncoderCatalog,
  EulaStatus,
  FfmpegStatus,
  Filter,
  FilterId,
  FilterKind,
  GameCaptureStatus,
  GuideLine,
  Health,
  HotkeyAuditEntry,
  IntegrationsStatus,
  ItemId,
  LinkPeer,
  LoopbackDevices,
  MonitorMode,
  NormRect,
  PtzMoveDirection,
  ScaleMode,
  RecordingFile,
  RundownStatus,
  RecordingStatus,
  ReplayStatus,
  SceneId,
  Settings,
  SourceId,
  SourceSettings,
  StillTarget,
  StreamStatus,
  StudioDto,
  Transform,
  VerticalCanvas,
  VideoDevice,
  VideoFormat,
  WorkbenchMode,
} from "./types";

/** Bridge liveness probe: app version + linked core crates. */
export function health(): Promise<Health> {
  return invoke<Health>("health");
}

/** Read the current settings. */
export function settingsGet(): Promise<Settings> {
  return invoke<Settings>("settings_get");
}

/** One-shot pickup of a freally:// invite that launched the app (cold-start
 * deep link — it fired before the webview's event listener existed). */
export function remotePendingInvite(): Promise<string | null> {
  return invoke<string | null>("remote_pending_invite");
}

// ---------------------------------------------------------------------------
// Live streaming (Phase 5)
// ---------------------------------------------------------------------------

/** Go Live with the configured target (Settings → Stream). */
export function streamStart(): Promise<void> {
  return invoke("stream_start");
}

/** The pre-flight disk item (CAP-M09): whole minutes of recording left at
 * the configured bitrate, or null when free space can't be read. */
export function preflightDisk(): Promise<number | null> {
  return invoke("preflight_disk");
}

/** End the stream (the local recording, if any, is untouched). */
export function streamStop(): Promise<StreamStatus> {
  return invoke<StreamStatus>("stream_stop");
}

/** The current stream status (the `stream` event pushes the same shape). */
export function streamStatus(): Promise<StreamStatus> {
  return invoke<StreamStatus>("stream_status");
}

// ---------------------------------------------------------------------------
// Replay buffer (Phase 6)
// ---------------------------------------------------------------------------

/** Arm the rolling replay buffer (starts its background encode). */
export function replayArm(): Promise<void> {
  return invoke("replay_arm");
}

/** Disarm the replay buffer (drops the un-saved history). */
export function replayDisarm(): Promise<void> {
  return invoke("replay_disarm");
}

/** Save the last N seconds to the recordings folder; resolves to the path. */
export function replaySave(): Promise<string> {
  return invoke<string>("replay_save");
}

/** Float a reaction emoji over the program (TASK-614 — baked into what
 * records and streams). */
export function studioSendReaction(emoji: string): Promise<void> {
  return invoke("studio_send_reaction", { emoji });
}

/** Drop a chapter marker at the current recording position (TASK-610). */
export function recordingAddMarker(): Promise<number> {
  return invoke<number>("recording_add_marker");
}

/** The current replay-buffer status (the `replay` event pushes the same). */
export function replayStatus(): Promise<ReplayStatus> {
  return invoke<ReplayStatus>("replay_status");
}

// ---------------------------------------------------------------------------
// Profiles + scene collections (Phase 5)
// ---------------------------------------------------------------------------

/** Named-snapshot listings: the active name + everything on disk. */
export type NamedList = { active: string; names: string[] };

export function profilesList(): Promise<NamedList> {
  return invoke<NamedList>("profiles_list");
}

/** Save the current settings under `name` and make it the active profile. */
export function profileCreate(name: string): Promise<NamedList> {
  return invoke<NamedList>("profile_create", { name });
}

/** Switch profiles (the outgoing one saves first); returns the new settings. */
export function profileSwitch(name: string): Promise<Settings> {
  return invoke<Settings>("profile_switch", { name });
}

export function collectionsList(): Promise<NamedList> {
  return invoke<NamedList>("collections_list");
}

/** Duplicate the current scenes under `name` and switch to that copy. */
export function collectionCreate(name: string): Promise<NamedList> {
  return invoke<NamedList>("collection_create", { name });
}

/** Switch scene collections (the active one saves first). */
export function collectionSwitch(name: string): Promise<NamedList> {
  return invoke<NamedList>("collection_switch", { name });
}

/** A caveat on one imported source (CAP-M02). Mirrors `ImportNote` in Rust. */
export type ImportNote =
  | "needsReselect"
  | "gameCaptureAsWindow"
  | "referencesFile"
  | "filterDropped"
  | "geometryApproximated";

/** Why an OBS source could not be imported. Mirrors `SkipReason`. */
export type SkipReason = "unsupportedKind" | "group";

/** One imported source and its caveats. Mirrors `ImportedSource`. */
export type ImportedSource = { name: string; obsKind: string; notes: ImportNote[] };

/** One source that was dropped. Mirrors `SkippedSource`. */
export type SkippedSource = { name: string; obsKind: string; reason: SkipReason };

/** The honest per-source account of an OBS import. Mirrors `ImportReport`. */
export type ImportReport = {
  name: string;
  sceneCount: number;
  sourceCount: number;
  itemCount: number;
  notes: ImportedSource[];
  skipped: SkippedSource[];
};

/** Import an OBS scene collection (`scenes.json`) as a new collection and
 * switch to it; resolves to the honest per-source report. */
export function collectionImportObs(path: string): Promise<ImportReport> {
  return invoke<ImportReport>("collection_import_obs", { path });
}

/** What references a file (CAP-M03). Mirrors `FileRefKind` in Rust. */
export type FileRefKind = "image" | "media" | "slideshow" | "font" | "lut" | "mask";

/** One broken file reference, grouped by path. Mirrors `MissingFile`. */
export type MissingFile = { path: string; kind: FileRefKind; sourceName: string; uses: number };

/** The missing-file doctor's scan: every referenced file that isn't on disk. */
export function collectionMissingFiles(): Promise<MissingFile[]> {
  return invoke<MissingFile[]>("collection_missing_files");
}

/** Repoint one broken path to a new one everywhere it appears; returns the
 * number of references changed. */
export function collectionRelink(oldPath: string, newPath: string): Promise<number> {
  return invoke<number>("collection_relink", { oldPath, newPath });
}

/** Bulk relink: find each missing file by name in `folder` and repoint it;
 * returns the number of references changed. */
export function collectionRelinkFolder(folder: string): Promise<number> {
  return invoke<number>("collection_relink_folder", { folder });
}

/** Replace and persist the settings. */
export function settingsSet(settings: Settings): Promise<void> {
  return invoke("settings_set", { settings });
}

/** Open (or focus) a browser dock window on an http(s) URL (TASK-702). */
export function browserDockOpen(name: string, url: string): Promise<void> {
  return invoke("browser_dock_open", { name, url });
}

/**
 * Enumerate screen/window sources. On Wayland this returns exactly one
 * portal entry — the system dialog picks the real source.
 */
export function captureListSources(): Promise<CaptureSource[]> {
  return invoke<CaptureSource[]>("capture_list_sources");
}

/**
 * A one-shot JPEG thumbnail (`data:` URI) of a window source, or `null` when
 * none is available (minimized/GPU-composited window, or an unsupported
 * platform). The picker re-requests this on a timer for a live preview.
 */
export function captureWindowThumbnail(id: string, maxDim?: number): Promise<string | null> {
  return invoke<string | null>("capture_window_thumbnail", { id, maxDim });
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

/**
 * Undo the newest scene edit (CAP-M01). Resolves to the reversed edit's label
 * (a stable `history.<label>` key), or `null` when there was nothing to undo.
 * The restored model arrives on the `studio` event like any mutation.
 */
export function studioUndo(): Promise<string | null> {
  return invoke<string | null>("studio_undo");
}

/** Redo the most recently undone scene edit. Mirror of {@link studioUndo}. */
export function studioRedo(): Promise<string | null> {
  return invoke<string | null>("studio_redo");
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

/** CAP-M18: the controls a RUNNING device reports (empty = not streaming
 * or a backend without control support). */
export function cameraControlsList(deviceId: string): Promise<CameraControl[]> {
  return invoke<CameraControl[]>("camera_controls_list", { deviceId });
}

/** CAP-M18: set one control now AND save it into the device's profile. */
export function cameraControlSet(deviceId: string, control: string, value: number): Promise<void> {
  return invoke("camera_control_set", { deviceId, control, value: Math.round(value) });
}

/** CAP-M18: clear the saved profile + restore the backend defaults. */
export function cameraProfileReset(deviceId: string): Promise<void> {
  return invoke("camera_profile_reset", { deviceId });
}

/** CAP-M14: every hotkey binding in the studio, with conflict analysis. */
export function hotkeyAudit(): Promise<HotkeyAuditEntry[]> {
  return invoke<HotkeyAuditEntry[]>("hotkey_audit");
}

/** CAP-M14: save the composed cheat sheet to Downloads; returns the path. */
export function hotkeyCheatsheetSave(content: string): Promise<string> {
  return invoke<string>("hotkey_cheatsheet_save", { content });
}

/** CAP-M15: drive a timer source's run state. Runtime-only (no undo). */
export function studioTimerControl(
  sourceId: SourceId,
  action: "start" | "pause" | "toggle" | "reset",
): Promise<void> {
  return invoke("studio_timer_control", { sourceId, action });
}

/** CAP-N18: drive a split-timer source's run. Runtime-only (no undo). */
export function studioSplitControl(
  sourceId: SourceId,
  action: "split" | "undo" | "skip" | "reset",
): Promise<void> {
  return invoke("studio_split_control", { sourceId, action });
}

/** CAP-N17: jump a playlist to its next/previous item. Runtime-only. */
export function studioPlaylistControl(
  sourceId: SourceId,
  action: "next" | "previous",
): Promise<void> {
  return invoke("studio_playlist_control", { sourceId, action });
}

/** CAP-N17: jump a playlist to a cue — `seconds` into item `item`'s file. */
export function studioPlaylistCue(
  sourceId: SourceId,
  item: number,
  seconds: number,
): Promise<void> {
  return invoke("studio_playlist_cue", { sourceId, item, seconds });
}

/** CAP-N16: fire a title's animate-in/out. Runtime-only. */
export function studioTitleFire(sourceId: SourceId, action: "in" | "out"): Promise<void> {
  return invoke("studio_title_fire", { sourceId, action });
}

/** CAP-N16: push live text into title layer `layer` (no session restart). */
export function studioTitleSetText(
  sourceId: SourceId,
  layer: number,
  value: string,
): Promise<void> {
  return invoke("studio_title_set_text", { sourceId, layer, value });
}

/** CAP-N10: roll one Instant Replay source (needs the armed buffer). */
export function replayRollSource(sourceId: SourceId): Promise<void> {
  return invoke("replay_roll_source", { sourceId });
}

/** CAP-N11: this machine's best-effort LAN address (for the ingest URL/QR).
 * The probe sends nothing — it only asks the OS which interface faces the LAN. */
export function localLanIp(): Promise<string> {
  return invoke<string>("local_lan_ip");
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

/** Apply several item transforms as one undo step — align/distribute/group-move
 * (CAP-M04 follow-on). `coalesce` folds a streaming group drag into one step. */
export function studioSetItemTransforms(
  sceneId: SceneId,
  changes: { item: ItemId; transform: Transform }[],
  coalesce: boolean,
): Promise<void> {
  return invoke("studio_set_item_transforms", { sceneId, changes, coalesce });
}

/** Replace a scene's custom alignment guides (CAP-M04 follow-on). */
export function studioSetGuides(sceneId: SceneId, guides: GuideLine[]): Promise<void> {
  return invoke("studio_set_guides", { sceneId, guides });
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

/** Pixel-perfect scaling (CAP-N70): smooth / nearest / integer / sharp-bilinear. */
export function studioSetItemScaling(
  sceneId: SceneId,
  itemId: ItemId,
  scaling: ScaleMode,
): Promise<void> {
  return invoke("studio_set_item_scaling", { sceneId, itemId, scaling });
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

/**
 * Seat one item into a normalized canvas slot — the one-click position
 * presets. Placement resolves on the source's next sized frame (the same
 * mechanism as `studioApplyLayout`'s corners).
 */
export function studioSetItemSlot(sceneId: SceneId, itemId: ItemId, slot: NormRect): Promise<void> {
  return invoke("studio_set_item_slot", { sceneId, itemId, slot });
}

/**
 * Center-view routing (host-only in a remote session): pass an item to seat
 * that capture in the center (the displaced center swaps onto its old seat;
 * cams never overlap the shared view; one screen view at a time). Pass
 * `null` to retire the center onto the cam rail.
 */
export function studioSetCenterView(sceneId: SceneId, itemId: ItemId | null): Promise<void> {
  return invoke("studio_set_center_view", { sceneId, itemId });
}

/**
 * Highlight Speaker (Focus/Spotlight): pass an item to promote it to fill the
 * canvas (the other video items hide); pass `null` to restore the exact
 * pre-focus layout.
 */
export function studioSetFocus(sceneId: SceneId, itemId: ItemId | null): Promise<void> {
  return invoke("studio_set_focus", { sceneId, itemId });
}

/** Studio Mode on/off (on = a preview pane opens on the program scene). */
export function studioSetStudioMode(on: boolean): Promise<void> {
  return invoke("studio_set_studio_mode", { on });
}

/** Group items so they move / show / hide together (Phase 6). */
export function studioCreateGroup(
  sceneId: SceneId,
  name: string,
  itemIds: ItemId[],
): Promise<string> {
  return invoke<string>("studio_create_group", { sceneId, name, itemIds });
}

/** Dissolve a group — its items stay exactly where they are. */
export function studioUngroup(sceneId: SceneId, groupId: string): Promise<void> {
  return invoke("studio_ungroup", { sceneId, groupId });
}

/** A group's eye toggle — hides/shows every member together. */
export function studioSetGroupVisible(
  sceneId: SceneId,
  groupId: string,
  visible: boolean,
): Promise<void> {
  return invoke("studio_set_group_visible", { sceneId, groupId, visible });
}

/** Set (or clear, with `null`) a source's per-scene mixer override. */
export function studioSetSceneAudioOverride(
  sceneId: SceneId,
  sourceId: SourceId,
  over: { volumeDb: number; muted: boolean } | null,
): Promise<void> {
  return invoke("studio_set_scene_audio_override", { sceneId, sourceId, over });
}

/** Configure (or clear, with `null`) the second (vertical) output canvas. */
export function studioSetVertical(vertical: VerticalCanvas | null): Promise<void> {
  return invoke("studio_set_vertical", { vertical });
}

/** Point the Studio-Mode preview pane at a scene. */
export function studioSetPreviewScene(sceneId: SceneId): Promise<void> {
  return invoke("studio_set_preview_scene", { sceneId });
}

/** Commit Preview → Program through the configured transition. */
export function studioTransition(): Promise<void> {
  return invoke("studio_transition");
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

/** Engage (true) or restore (false) the panic slate (CAP-M22). Restoring is
 * only ever called from the deliberate two-step confirm. */
export function studioPanicSet(on: boolean): Promise<void> {
  return invoke("studio_panic_set", { on });
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

/** Paste a copied filter chain onto an item (CAP-M05) — each filter is appended
 * with a fresh id, keeping its kind + enabled state. Resolves to how many were
 * added; the whole paste is a single undo step. */
export function studioPasteFilters(
  sceneId: SceneId,
  itemId: ItemId,
  filters: Filter[],
): Promise<number> {
  return invoke<number>("studio_paste_filters", { sceneId, itemId, filters });
}

/** Open/update the keying workbench (CAP-M26): render `itemId` in `mode`, with
 * `split` (0..1) the before/after divider for Split mode. Preview-only; the
 * frame arrives on the `workbench-preview` pipe. */
export function studioWorkbenchSet(
  itemId: ItemId,
  mode: WorkbenchMode,
  split: number,
): Promise<void> {
  return invoke("studio_workbench_set", { itemId, mode, split });
}

/** Close the keying workbench (clears its preview slot). */
export function studioWorkbenchClose(): Promise<void> {
  return invoke("studio_workbench_close");
}

/** Open/close the multiview monitor (CAP-M06): while on, the render loop keeps
 * every scene's sources live and publishes per-scene thumbnails to
 * `/multiview/<id>`. */
export function studioMultiviewSet(on: boolean): Promise<void> {
  return invoke("studio_multiview_set", { on });
}

/** Grab a still frame (CAP-M08): a lossless PNG of the program or a single
 * source, saved into the recordings folder. Resolves once queued; the saved
 * path arrives on the `still-saved` event (or `still-error`). */
export function captureStill(target: StillTarget): Promise<void> {
  return invoke("studio_capture_still", { target });
}

/** Confirm the quit guard (CAP-M23): ordered shutdown — end stream →
 * finalize recordings → flush replay → save — then the app exits. */
export function quitConfirmed(): Promise<void> {
  return invoke("quit_confirmed");
}

/** Cancel the quit guard: the next close asks again (instead of reading as
 * the hung-webview double-close escape). */
export function quitGuardCancel(): Promise<void> {
  return invoke("quit_guard_cancel");
}

// -- Redacted diagnostics bundle (CAP-M24) -------------------------------------

/** The EXACT text the diagnostics zip will contain (allowlist-built,
 * scrubbed) — shown before export, in the crash-report tradition. */
export function diagnosticsPreview(): Promise<string> {
  return invoke("diagnostics_preview");
}

/** Zip the bundle into Downloads and resolve to its path. Strictly local —
 * nothing is sent anywhere; the user attaches it by hand. */
export function diagnosticsExport(): Promise<string> {
  return invoke("diagnostics_export");
}

// -- Recording salvage (CAP-M11) ----------------------------------------------

/** Interrupted recordings found at startup (unclean exit) awaiting repair. */
export function salvagePending(): Promise<string[]> {
  return invoke("salvage_pending");
}

/** Repair one pending file into a `(repaired)` sibling; resolves to the new
 * path. Long recordings copy for minutes — keep the UI responsive. */
export function salvageRepair(path: string): Promise<string> {
  return invoke("salvage_repair", { path });
}

/** Decline the salvage offer (the files themselves stay on disk). */
export function salvageDismiss(): Promise<void> {
  return invoke("salvage_dismiss");
}

// -- Projectors + aux windows (CAP-M07) ---------------------------------------

/** Enumerate the connected displays for the projector "open on…" picker. */
export function listDisplays(): Promise<DisplayInfo[]> {
  return invoke<DisplayInfo[]>("list_displays");
}

/** Open (or focus) an auxiliary window on a display. `label` says what it shows
 * (`projector-program`, `projector-preview`, `multiview`); `display` positions
 * it on that monitor; `fullscreen` fullscreens it, else it floats on top. */
export function auxWindowOpen(
  label: string,
  title: string,
  display: number | null,
  fullscreen: boolean,
): Promise<void> {
  return invoke("aux_window_open", { label, title, display, fullscreen });
}

/** Close an auxiliary window by label. */
export function auxWindowClose(label: string): Promise<void> {
  return invoke("aux_window_close", { label });
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

/** Apps currently making sound (Windows) + the honest per-OS guidance. */
export function appAudioApps(): Promise<AppAudioList> {
  return invoke<AppAudioList>("app_audio_apps");
}

/** Optional-integration status: NDI (detected runtime) + VST (scoped). */
export function integrationsStatus(): Promise<IntegrationsStatus> {
  return invoke<IntegrationsStatus>("integrations_status");
}

/** Game-capture status: honest anti-cheat/AV risk + the working fallback. */
export function gameCaptureStatus(): Promise<GameCaptureStatus> {
  return invoke<GameCaptureStatus>("game_capture_status");
}

/** The EULA text + version + whether the current version is already accepted. */
export function eulaStatus(): Promise<EulaStatus> {
  return invoke<EulaStatus>("eula_status");
}

/** Record acceptance of the current EULA version (persisted). */
export function eulaAccept(): Promise<void> {
  return invoke("eula_accept");
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

/** CAP-M19: stereo balance, −1..=1 (0 = untouched). */
export function studioSetAudioPan(sourceId: SourceId, pan: number): Promise<void> {
  return invoke("studio_set_audio_pan", { sourceId, pan });
}

/** CAP-M19: PFL solo — monitor routing only. */
export function studioSetAudioSolo(sourceId: SourceId, solo: boolean): Promise<void> {
  return invoke("studio_set_audio_solo", { sourceId, solo });
}

/** CAP-M19: mono downmix. */
export function studioSetAudioMono(sourceId: SourceId, mono: boolean): Promise<void> {
  return invoke("studio_set_audio_mono", { sourceId, mono });
}

export function studioSetAudioSyncOffset(sourceId: SourceId, syncOffsetMs: number): Promise<void> {
  return invoke("studio_set_audio_sync_offset", { sourceId, syncOffsetMs });
}

/** CAP-M20: arm both sync-workbench probes (one shared zero instant backend-side). */
export function calibrationStart(videoSource: SourceId, audioSource: SourceId): Promise<void> {
  return invoke("calibration_start", { videoSource, audioSource });
}

/** CAP-M20: disarm both probes (cancel / dialog closed). */
export function calibrationStop(): Promise<void> {
  return invoke("calibration_stop");
}

/** CAP-M20: live "seeing flashes / hearing beeps" feedback while measuring. */
export function calibrationStatus(): Promise<CalibrationStatus> {
  return invoke<CalibrationStatus>("calibration_status");
}

/** CAP-M20: take the series, disarm, and estimate the offset. */
export function calibrationFinish(): Promise<CalibrationResult> {
  return invoke<CalibrationResult>("calibration_finish");
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

/** The CEF (browser-source runtime) component status. */
export function cefStatus(): Promise<CefStatus> {
  return invoke<CefStatus>("cef_status");
}

/** Start the on-demand CEF fetch + verify (progress rides the `cef` event). */
export function cefInstall(): Promise<void> {
  return invoke("cef_install");
}

/** Cancel an in-flight CEF fetch (the partial download is removed). */
export function cefCancel(): Promise<void> {
  return invoke("cef_cancel");
}

/** Remove the installed CEF runtime. */
export function cefRemove(): Promise<void> {
  return invoke("cef_remove");
}

// ---------------------------------------------------------------------------
// Native preview surface (the "OBS feel" path)
// ---------------------------------------------------------------------------

/**
 * Whether the native GPU preview surface is active (Windows + created). When
 * true the preview panel hides its JPEG `<canvas>` — the native child window
 * paints the region directly.
 */
export function nativePreviewActive(): Promise<boolean> {
  return invoke<boolean>("native_preview_active");
}

/**
 * Report the preview region's on-screen rectangle in **physical pixels**
 * (relative to the window client area) + whether it's currently visible. The
 * native child window follows it. A no-op off Windows.
 */
export function nativePreviewSetRegion(
  x: number,
  y: number,
  width: number,
  height: number,
  visible: boolean,
): Promise<void> {
  return invoke("native_preview_set_region", { x, y, width, height, visible });
}

/**
 * Report which scene item is selected so the native GPU preview can draw its
 * selection box + handles into the frame (they'd otherwise be hidden under the
 * opaque native surface). `null` clears it. A no-op off the native path.
 */
export function nativePreviewSetSelection(item: ItemId | null): Promise<void> {
  return invoke("native_preview_set_selection", { item });
}

/** Alignment overlay drawn into the native GPU frame (CAP-M04): safe-area
 * rectangles + guide lines, in canvas px. Mirrors {@link nativePreviewSetSelection} —
 * the SVG path renders the same model; a no-op off the native path. */
export type PreviewOverlay = {
  safeAreas: { x: number; y: number; w: number; h: number }[];
  guides: { orientation: "v" | "h"; position: number; from: number; to: number }[];
};

export function nativePreviewSetOverlay(overlay: PreviewOverlay): Promise<void> {
  return invoke("native_preview_set_overlay", { overlay });
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

/** Export a .frec recording to a sibling mp4/mkv/mov/webm (decode + re-encode
 * through the ffmpeg component, so it plays in any player). Progress rides the
 * `recording-export` event. */
export function recordingExport(path: string, container: string): Promise<void> {
  return invoke("recording_export", { path, container });
}

/** Cancel the running .frec export. */
export function recordingExportCancel(): Promise<void> {
  return invoke("recording_export_cancel");
}

/** A `.frec` the app was opened with (OS double-click), if any — one-shot. */
export function openFrecPending(): Promise<string | null> {
  return invoke<string | null>("open_frec_pending");
}

/** Pause or resume an embedded Media source (video) live on the stream. */
export function studioMediaSetPaused(sourceId: SourceId, paused: boolean): Promise<void> {
  return invoke("studio_media_set_paused", { sourceId, paused });
}

/** Whether an embedded Media source is currently paused. */
export function studioMediaPaused(sourceId: SourceId): Promise<boolean> {
  return invoke<boolean>("studio_media_paused", { sourceId });
}

/** A Media source's transport state; duration 0 = not known yet. */
export type MediaTransport = { position: number; duration: number };

/** Jump a Media source's playback to `seconds` (the transport scrubber). */
export function studioMediaSeek(sourceId: SourceId, seconds: number): Promise<void> {
  return invoke("studio_media_seek", { sourceId, seconds });
}

/** Poll a Media source's playhead + duration for the scrubber. */
export function studioMediaTransport(sourceId: SourceId): Promise<MediaTransport> {
  return invoke<MediaTransport>("studio_media_transport", { sourceId });
}

/** The MIDI ports on this machine (CAP-N03): `[inputs, outputs]`. */
export function midiPorts(): Promise<[string[], string[]]> {
  return invoke<[string[], string[]]>("midi_ports");
}

/** Arm MIDI-learn: the next pad/knob touched is reported, not fired. */
export function midiLearn(on: boolean): Promise<void> {
  return invoke("midi_learn", { on });
}

/** Drive a PTZ head (CAP-N08); "stop" halts it (VISCA heads keep moving). */
export function ptzMove(
  camera: string,
  direction: PtzMoveDirection,
  panSpeed: number,
  tiltSpeed: number,
): Promise<void> {
  return invoke("ptz_move", { camera, direction, panSpeed, tiltSpeed });
}

/** Zoom a PTZ camera (positive = in, negative = out, 0 = stop). */
export function ptzZoom(camera: string, speed: number): Promise<void> {
  return invoke("ptz_zoom", { camera, speed });
}

/** Recall a PTZ preset slot. */
export function ptzPresetRecall(camera: string, slot: number): Promise<void> {
  return invoke("ptz_preset_recall", { camera, slot });
}

/** Store the camera's current position into a preset slot. */
export function ptzPresetStore(camera: string, slot: number): Promise<void> {
  return invoke("ptz_preset_store", { camera, slot });
}

/** The LAN panel's URL (CAP-N06), or null while it is off. */
export function panelUrl(): Promise<string | null> {
  return invoke<string | null>("panel_url");
}

/** Scan the LAN for Freally Link outputs (~2 s; CAP-N12). User-initiated. */
export function linkDiscover(): Promise<LinkPeer[]> {
  return invoke<LinkPeer[]>("link_discover");
}

/** The Freally Link output's address (CAP-N12), or null while it is off. */
export function linkUrl(): Promise<string | null> {
  return invoke<string | null>("link_url");
}

/** Switch the active hotkey layer (CAP-N05; sticky — see the docs). */
export function hotkeySetLayer(layer: number): Promise<void> {
  return invoke("hotkey_set_layer", { layer });
}

/** The active hotkey layer (0 = base). */
export function hotkeyLayer(): Promise<number> {
  return invoke<number>("hotkey_layer");
}

/** Start the show rundown at a step (CAP-N09). */
export function rundownStart(index: number): Promise<void> {
  return invoke("rundown_start", { index });
}

/** Advance the rundown to the next step; stops at the end. */
export function rundownAdvance(): Promise<void> {
  return invoke("rundown_advance");
}

/** Stop the rundown (the scene stays put). */
export function rundownStop(): Promise<void> {
  return invoke("rundown_stop");
}

/** The rundown's live state — next up + remaining time. */
export function rundownStatus(): Promise<RundownStatus> {
  return invoke<RundownStatus>("rundown_status");
}

/** Run a macro by name (CAP-N02) — the same path hotkeys and rules use. */
export function automationRunMacro(name: string): Promise<void> {
  return invoke("automation_run_macro", { name });
}

/** Every studio variable (CAP-N02). */
export function automationVariables(): Promise<Record<string, string>> {
  return invoke<Record<string, string>>("automation_variables");
}

/** Set one studio variable by hand (CAP-N02). */
export function automationSetVariable(name: string, value: string): Promise<void> {
  return invoke("automation_set_variable", { name, value });
}

/** The passthrough monitor's measured capture→publish latency in ms
 * (CAP-N69); null until a monitor has been open long enough to measure. */
export function studioPassthroughLatency(sourceId: SourceId): Promise<number | null> {
  return invoke<number | null>("studio_passthrough_latency", { sourceId });
}

/** Set one display's HDR→SDR tone-map (CAP-N74) — persisted and applied to
 * the very next captured frame (no restart). */
export function hdrToneMapSet(
  captureId: string,
  operator: string,
  paperWhiteNits: number,
): Promise<void> {
  return invoke("hdr_tone_map_set", { captureId, operator, paperWhiteNits });
}

/** Set one capture's cursor effects (CAP-N19) — persisted and applied to the
 * very next captured frame (no restart). Windows captures only (macOS/Linux
 * composite their own cursor). */
export function cursorFxSet(captureId: string, fx: CursorFxSetting): Promise<void> {
  return invoke("cursor_fx_set", { captureId, fx });
}

/** Add a Window Capture together with its app's audio as one linked pair
 * (CAP-N73, Windows). Hidden mutes the audio; removal removes it; the pid
 * re-resolves across app restarts. */
export function studioAddLinkedWindow(
  sceneId: SceneId,
  captureId: string,
  label: string,
): Promise<AddedItem> {
  return invoke<AddedItem>("studio_add_linked_window", { sceneId, captureId, label });
}

/** One-shot auto black-bar crop (CAP-N72): scan the item's next frame and
 * apply the detected crop as an undoable edit. */
export function studioAutocrop(sceneId: SceneId, itemId: ItemId): Promise<void> {
  return invoke("studio_autocrop", { sceneId, itemId });
}

/** Arm/disarm re-detection on resolution change (follow; off by default). */
export function studioAutocropFollow(
  sceneId: SceneId,
  itemId: ItemId,
  follow: boolean,
): Promise<void> {
  return invoke("studio_autocrop_follow", { sceneId, itemId, follow });
}

/** Whether follow-mode auto-crop is armed for an item. */
export function studioAutocropGet(itemId: ItemId): Promise<boolean> {
  return invoke<boolean>("studio_autocrop_get", { itemId });
}

/** A punch-in lens's target state (CAP-N71). */
export type ZoomLensState = { zoom: number; follow: boolean };

/** Punch-in zoom (CAP-N71): set a lens's absolute target zoom (1..8) about
 * an optional normalized content anchor. Runtime-only — never persisted. */
export function studioZoomSet(
  itemId: ItemId,
  zoom: number,
  anchor?: { x: number; y: number },
): Promise<void> {
  return invoke("studio_zoom_set", {
    itemId,
    zoom,
    anchorX: anchor?.x ?? null,
    anchorY: anchor?.y ?? null,
  });
}

/** Multiply a lens's target zoom (the canvas wheel) about an anchor. */
export function studioZoomScroll(
  itemId: ItemId,
  factor: number,
  anchorX: number,
  anchorY: number,
): Promise<void> {
  return invoke("studio_zoom_scroll", { itemId, factor, anchorX, anchorY });
}

/** Toggle follow-the-cursor panning while zoomed (Windows-first). */
export function studioZoomFollow(itemId: ItemId, follow: boolean): Promise<void> {
  return invoke("studio_zoom_follow", { itemId, follow });
}

/** A lens's current target (for UI state after a reload). */
export function studioZoomGet(itemId: ItemId): Promise<ZoomLensState> {
  return invoke<ZoomLensState>("studio_zoom_get", { itemId });
}

/** Set (or clear, with `null`) a scene's backdrop wallpaper — an image, an
 * animated GIF, or a looping video, pinned under everything else. */
export function studioSetSceneBackdrop(sceneId: SceneId, path: string | null): Promise<void> {
  return invoke("studio_set_scene_backdrop", { sceneId, path });
}

/** Move the backdrop between the whole canvas and a half split (the capture
 * is seated into the other half). */
export function studioSetBackdropSplit(sceneId: SceneId, split: BackdropSplit): Promise<void> {
  return invoke("studio_set_backdrop_split", { sceneId, split });
}

/** Toggle a video backdrop's "start playback with recording" hold. */
export function studioSetBackdropSync(
  sceneId: SceneId,
  startWithRecording: boolean,
): Promise<void> {
  return invoke("studio_set_backdrop_sync", { sceneId, startWithRecording });
}

/** Export a `.frec` the user opened via the OS to a sibling wire file. */
export function openFrecExport(path: string, container: string): Promise<void> {
  return invoke("open_frec_export", { path, container });
}

/** The anonymous bug-report context: app/OS info + any pending crash. */
export function bugReportContext(): Promise<BugReportContext> {
  return invoke<BugReportContext>("bug_report_context");
}

/** Open a pre-filled GitHub issue ("github"), Gmail web compose ("gmail"), or
 * the OS mail client ("email") with the anonymous report — the user still
 * clicks send. Nothing auto-sends. */
export function bugReportSubmit(
  target: "github" | "gmail" | "email",
  description: string,
  includeCrash: boolean,
): Promise<void> {
  return invoke("bug_report_submit", { target, description, includeCrash });
}

/** Probe the machine and suggest encoder/fps/bitrate. Reads hardware; changes nothing. */
export function autoconfigSuggest(): Promise<AutoConfig> {
  return invoke<AutoConfig>("autoconfig_suggest");
}

/** Record that the first-run wizard was finished OR skipped. */
export function settingsCompleteOnboarding(): Promise<void> {
  return invoke("settings_complete_onboarding");
}

/** What the About panel shows: version, authors, dates, links. Read-only. */
export function buildInfo(): Promise<BuildInfo> {
  return invoke<BuildInfo>("build_info");
}

/** Dismiss + delete the pending crash report. */
export function bugReportClearCrash(): Promise<void> {
  return invoke("bug_report_clear_crash");
}
