/**
 * Typed wrappers around core → UI push events.
 *
 * Event names + payloads mirror `src-tauri/src/events.rs` and
 * `src-tauri/src/studio.rs`.
 */
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

import type {
  AudioLevelsPayload,
  ExportStatus,
  FfmpegStatus,
  ProgramStatus,
  RecordingStatus,
  ReplayStatus,
  StatsPayload,
  StreamStatus,
  StudioDto,
} from "./types";

/** Subscribe to the ~2 Hz `stats` event. Resolves to an unlisten function. */
export function onStats(handler: (stats: StatsPayload) => void): Promise<UnlistenFn> {
  return listen<StatsPayload>("stats", (event) => handler(event.payload));
}

/** Subscribe to opened freally:// invite links (the OS deep link). The URL
 * is untrusted — parse it with the invite validator; never auto-join. */
export function onRemoteInvite(handler: (url: string) => void): Promise<UnlistenFn> {
  return listen<string>("remote-invite", (event) => handler(event.payload));
}

/** Subscribe to live-stream status (~1 Hz while a session runs). */
export function onStream(handler: (status: StreamStatus) => void): Promise<UnlistenFn> {
  return listen<StreamStatus>("stream", (event) => handler(event.payload));
}

/** Subscribe to replay-buffer status (~1 Hz while armed). */
export function onReplay(handler: (status: ReplayStatus) => void): Promise<UnlistenFn> {
  return listen<ReplayStatus>("replay", (event) => handler(event.payload));
}

/** Subscribe to saved-replay confirmations (the toast's trigger). */
export function onReplaySaved(handler: (payload: { path: string }) => void): Promise<UnlistenFn> {
  return listen<{ path: string }>("replay_saved", (event) => handler(event.payload));
}

/** Subscribe to model changes — the full collection on every mutation. */
export function onStudio(handler: (studio: StudioDto) => void): Promise<UnlistenFn> {
  return listen<StudioDto>("studio", (event) => handler(event.payload));
}

/** Subscribe to compose-loop health + per-source states (≥1 Hz). */
export function onProgram(handler: (status: ProgramStatus) => void): Promise<UnlistenFn> {
  return listen<ProgramStatus>("program", (event) => handler(event.payload));
}

/** Subscribe to mixer levels + audio source states (~20 Hz). */
export function onAudio(handler: (levels: AudioLevelsPayload) => void): Promise<UnlistenFn> {
  return listen<AudioLevelsPayload>("audio", (event) => handler(event.payload));
}

/** Subscribe to the ffmpeg component's install/status changes. */
export function onFfmpeg(handler: (status: FfmpegStatus) => void): Promise<UnlistenFn> {
  return listen<FfmpegStatus>("ffmpeg", (event) => handler(event.payload));
}

/** Subscribe to recording state/progress (~2 Hz while a session runs). */
export function onRecording(handler: (status: RecordingStatus) => void): Promise<UnlistenFn> {
  return listen<RecordingStatus>("recording", (event) => handler(event.payload));
}

/** Subscribe to .frec export progress (frames done/total + terminal state). */
export function onRecordingExport(handler: (status: ExportStatus) => void): Promise<UnlistenFn> {
  return listen<ExportStatus>("recording-export", (event) => handler(event.payload));
}

/** Subscribe to a .frec opened via the OS while the app is already running. */
export function onOpenFrec(handler: (path: string) => void): Promise<UnlistenFn> {
  return listen<string>("open-frec", (event) => handler(event.payload));
}
