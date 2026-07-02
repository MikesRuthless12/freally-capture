/**
 * Typed mirrors of the Rust bridge payloads.
 *
 * Keep in lockstep with `src-tauri/src/commands/mod.rs`,
 * `src-tauri/src/events.rs`, and `src-tauri/src/settings.rs`.
 */

/** One linked core crate, as reported by `health`. */
export type CrateHealth = {
  name: string;
  version: string;
};

/** The `health` command report. */
export type Health = {
  appVersion: string;
  os: string;
  coreOk: boolean;
  crates: CrateHealth[];
};

/** The persisted user settings (`settings.json` in the OS config dir). */
export type Settings = {
  language: string;
  showStatsDock: boolean;
};

/** The `stats` push-event payload (~2 Hz). */
export type StatsPayload = {
  fps: number;
  cpu: number;
  /** True until real sampling lands (P5.5) — the UI labels the data honestly. */
  placeholder: boolean;
};

// ---------------------------------------------------------------------------
// Capture + preview (Phase 1)
// ---------------------------------------------------------------------------

/**
 * "portal" = the Linux ScreenCast portal: the *system dialog* picks the
 * screen/window (the only capture Wayland allows — shown honestly as such).
 */
export type CaptureSourceKind = "display" | "window" | "portal";

/** One capturable screen/window, as listed by `capture_list_sources`. */
export type CaptureSource = {
  id: string;
  kind: CaptureSourceKind;
  label: string;
  /** Pixel size when known; 0 when the OS only reveals it after start. */
  width: number;
  height: number;
};

/** One webcam / capture card. */
export type VideoDevice = {
  id: string;
  name: string;
};

/** One capture format a video device offers. */
export type VideoFormat = {
  width: number;
  height: number;
  fps: number;
  fourcc: string;
};

/** What to preview — mirrors the Rust `PreviewSource` enum (tag = `kind`). */
export type PreviewSource =
  | { kind: "display"; id: string; label: string }
  | { kind: "window"; id: string; label: string }
  | { kind: "portal"; label?: string }
  | { kind: "webcam"; id: string; label: string; format?: VideoFormat };

export type PreviewErrorCode =
  "permission" | "cancelled" | "notFound" | "unsupported" | "stopped" | "backend";

/** The `preview` push-event payload (state changes + a 1 Hz live update). */
export type PreviewStatus = {
  state: "idle" | "waiting" | "live" | "error";
  /** The Sources-rail card this status belongs to. */
  sourceKey?: string;
  label?: string;
  width?: number;
  height?: number;
  /** Measured preview frame rate (frames received in the last second). */
  fps?: number;
  /** Frames overwritten before the preview consumed them. */
  dropped?: number;
  errorCode?: PreviewErrorCode;
  errorMessage?: string;
};
