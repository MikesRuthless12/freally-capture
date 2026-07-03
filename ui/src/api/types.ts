/**
 * Typed mirrors of the Rust bridge payloads.
 *
 * Keep in lockstep with `src-tauri/src/commands/`, `src-tauri/src/events.rs`,
 * `src-tauri/src/studio.rs`, `src-tauri/src/settings.rs`, and the scene model
 * in `crates/scene` (serde camelCase on every shape).
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
  /** The audio monitor output device name (null/"" = the OS default). */
  monitorDevice: string | null;
};

/** The `stats` push-event payload (~2 Hz). */
export type StatsPayload = {
  fps: number;
  cpu: number;
  /** True until real sampling lands (P5.5) — the UI labels the data honestly. */
  placeholder: boolean;
};

// ---------------------------------------------------------------------------
// Capture pickers (Phase 1)
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

// ---------------------------------------------------------------------------
// The scene model (Phase 2 — mirrors crates/scene)
// ---------------------------------------------------------------------------

export type SceneId = string;
export type SourceId = string;
export type ItemId = string;
export type FilterId = string;

/** Straight (unpremultiplied) RGBA, 0–255 per channel. */
export type Rgba = { r: number; g: number; b: number; a: number };

/** Pixels cut from each edge of the source (pre-scale). */
export type Crop = { left: number; top: number; right: number; bottom: number };

/**
 * Where an item sits: `x`/`y` place the (cropped) content's **center** in
 * canvas px; scales multiply the cropped size; rotation is degrees clockwise
 * about that center. Mirrors `crates/compositor/src/transform.rs` — the
 * on-canvas handles depend on this exact math.
 */
export type Transform = {
  x: number;
  y: number;
  scaleX: number;
  scaleY: number;
  rotation: number;
  crop: Crop;
};

export type BlendMode =
  "normal" | "additive" | "subtract" | "screen" | "multiply" | "lighten" | "darken";

export const BLEND_MODES: BlendMode[] = [
  "normal",
  "additive",
  "subtract",
  "screen",
  "multiply",
  "lighten",
  "darken",
];

export type TextAlign = "left" | "center" | "right";

export type VideoDeviceFormat = {
  width: number;
  height: number;
  fps: number;
  fourcc: string;
};

/** Per-kind source settings (serde tag = `kind`). */
export type SourceSettings =
  | { kind: "display"; captureId: string; label: string }
  | { kind: "window"; captureId: string; label: string }
  | { kind: "portal" }
  | { kind: "videoDevice"; deviceId: string; format?: VideoDeviceFormat | null }
  | { kind: "image"; path: string }
  | { kind: "color"; color: Rgba; width: number; height: number }
  | { kind: "audioInput"; deviceId: string }
  | { kind: "audioOutput"; deviceId: string }
  | {
      kind: "text";
      text: string;
      fontFamily?: string | null;
      fontFile?: string | null;
      sizePx: number;
      color: Rgba;
      align: TextAlign;
      lineSpacing: number;
      forceRtl: boolean;
      wrapWidth?: number | null;
    };

export type SourceKindName = SourceSettings["kind"];

/** Whether a source kind produces audio (and so carries `AudioSettings`). */
export function kindHasAudio(kind: SourceKindName): boolean {
  return kind === "audioInput" || kind === "audioOutput";
}

/** One shared source: identity + name + flattened settings (+ audio strip). */
export type Source = {
  id: SourceId;
  name: string;
  audio?: AudioSettings | null;
} & SourceSettings;

// ---------------------------------------------------------------------------
// Audio (Phase 3 — mirrors crates/scene/src/audio.rs + src-tauri/src/audio.rs)
// ---------------------------------------------------------------------------

export type AudioFilterId = string;

/** Where a source's monitored audio goes. */
export type MonitorMode = "off" | "monitorOnly" | "monitorAndOutput";

/** One audio filter's parameters (serde tag = `type`; owned classic DSP). */
export type AudioFilterKind =
  | { type: "gain"; db: number }
  | {
      type: "noiseGate";
      openThresholdDb: number;
      closeThresholdDb: number;
      attackMs: number;
      holdMs: number;
      releaseMs: number;
    }
  | {
      type: "compressor";
      ratio: number;
      thresholdDb: number;
      attackMs: number;
      releaseMs: number;
      outputGainDb: number;
    }
  | { type: "limiter"; thresholdDb: number; releaseMs: number }
  | { type: "eq"; lowDb: number; midDb: number; highDb: number }
  | { type: "denoise"; strength: number }
  | {
      type: "ducker";
      trigger?: SourceId | null;
      thresholdDb: number;
      amountDb: number;
      attackMs: number;
      releaseMs: number;
    };

export type AudioFilterTypeName = AudioFilterKind["type"];

/** One audio filter instance in a source's chain. */
export type AudioFilter = { id: AudioFilterId; enabled: boolean } & AudioFilterKind;

/** The fader floor/ceiling (mirrors MIN/MAX_VOLUME_DB in crates/scene). */
export const MIN_VOLUME_DB = -60;
export const MAX_VOLUME_DB = 6;
export const TRACK_COUNT = 6;
export const MAX_SYNC_OFFSET_MS = 950;

/** A source's whole mixer state (lives on the shared source). */
export type AudioSettings = {
  volumeDb: number;
  muted: boolean;
  monitor: MonitorMode;
  /** Bitmask of the up-to-6 tracks (bit 0 = track 1). */
  tracks: number;
  syncOffsetMs: number;
  /** Hotkey accelerator: silent unless held. */
  pushToTalk?: string | null;
  /** Hotkey accelerator: silent while held. */
  pushToMute?: string | null;
  filters: AudioFilter[];
};

/** One selectable audio device. */
export type AudioDevice = {
  id: string;
  name: string;
  isDefault: boolean;
};

/** The Audio Output Capture picker payload (+ honest per-OS guidance). */
export type LoopbackDevices = {
  devices: AudioDevice[];
  guidance?: string;
};

/** One source's live levels/status in the `audio` event. */
export type AudioSourceLevels = {
  state: SourceRuntimeState;
  errorCode?: string;
  errorMessage?: string;
  /** Linear peak per channel [L, R] since the last event. */
  peak: [number, number];
  /** Linear RMS per channel [L, R] since the last event. */
  rms: [number, number];
  /** The strip mixes silence right now (mute or a PTT/PTM gate). */
  gated: boolean;
};

/** The `audio` push event (~20 Hz): per-source levels + program mix health. */
export type AudioLevelsPayload = {
  sources: Record<string, AudioSourceLevels>;
  master: { peak: [number, number]; rms: [number, number] };
  lufs: { momentary?: number; shortTerm?: number };
  monitorError?: string;
  /** Capture samples dropped across sources (ring overflows). */
  dropped: number;
};

/** One filter's parameters (serde tag = `type`). */
export type FilterKind =
  | { type: "chromaKey"; key: Rgba; similarity: number; smoothness: number; spill: number }
  | {
      type: "colorCorrection";
      gamma: number;
      brightness: number;
      contrast: number;
      saturation: number;
      hueShift: number;
      opacity: number;
    }
  | { type: "lut"; path: string; amount: number }
  | { type: "blur"; radius: number }
  | { type: "mask"; path: string; mode: "alpha" | "luma"; invert: boolean }
  | { type: "sharpen"; amount: number }
  | { type: "scroll"; speedX: number; speedY: number }
  | { type: "crop"; left: number; top: number; right: number; bottom: number };

export type FilterTypeName = FilterKind["type"];

/** One filter instance in an item's chain. */
export type Filter = { id: FilterId; enabled: boolean } & FilterKind;

/** One placement of a source in a scene. */
export type SceneItem = {
  id: ItemId;
  source: SourceId;
  visible: boolean;
  locked: boolean;
  blend: BlendMode;
  transform: Transform;
  /** True until the first frame auto-fits the item (engine-managed). */
  pendingFit: boolean;
  filters: Filter[];
};

/** One scene: ordered items, index = z-order, `items[0]` bottom-most. */
export type Scene = {
  id: SceneId;
  name: string;
  items: SceneItem[];
};

/** The whole model (the on-disk scene-collection format). */
export type Collection = {
  formatVersion: number;
  canvasWidth: number;
  canvasHeight: number;
  sources: Source[];
  scenes: Scene[];
  activeScene: SceneId;
};

/** The `studio` event / `studio_get` payload. */
export type StudioDto = {
  revision: number;
  collection: Collection;
};

/** What `studio_add_item` created. */
export type AddedItem = {
  sourceId: SourceId;
  itemId: ItemId;
};

// ---------------------------------------------------------------------------
// The `program` event (compose-loop health)
// ---------------------------------------------------------------------------

export type SourceRuntimeState = "waiting" | "live" | "error";

export type SourceRuntimeErrorCode =
  "permission" | "cancelled" | "notFound" | "unsupported" | "stopped" | "backend";

/** Live status of one source (keyed by source id). */
export type SourceRuntime = {
  state: SourceRuntimeState;
  width?: number;
  height?: number;
  fps?: number;
  errorCode?: SourceRuntimeErrorCode;
  errorMessage?: string;
};

/** The `program` push event: compose fps + per-source states (≥1 Hz). */
export type ProgramStatus = {
  /** "noGpu" is honest: no adapter at all — the canvas cannot compose. */
  state: "starting" | "running" | "noGpu";
  width: number;
  height: number;
  fps: number;
  renderMicros: number;
  adapter: string;
  dropped: number;
  sources: Record<string, SourceRuntime>;
};

// ---------------------------------------------------------------------------
// Encoders + the on-demand ffmpeg component (Phase 4 — mirrors
// crates/encode and src-tauri/src/commands/recording.rs)
// ---------------------------------------------------------------------------

export type VideoCodec = "h264" | "hevc" | "av1";

export type EncoderEngine = "nvenc" | "quickSync" | "amf" | "videoToolbox" | "vaapi" | "software";

export type GpuVendor = "nvidia" | "amd" | "intel" | "apple" | "other";

/** One physical GPU, as encoder detection saw it. */
export type GpuInfo = {
  name: string;
  vendor: GpuVendor;
  backend: string;
};

/** One encoder the picker can offer (`id` is the stable ffmpeg name). */
export type EncoderDesc = {
  id: string;
  codec: VideoCodec;
  engine: EncoderEngine;
  label: string;
  hardware: boolean;
  /** The honest capability note the picker shows. */
  note: string;
  /**
   * null until verified against the installed ffmpeg component;
   * false = refused here (greyed out, auto-pick skips it).
   */
  verified: boolean | null;
};

/** Everything `encoders_list` found. */
export type EncoderCatalog = {
  gpus: GpuInfo[];
  encoders: EncoderDesc[];
};

/** The build an install would fetch (pinned URL + size). */
export type FfmpegBuild = {
  version: string;
  source: string;
  url: string;
  sizeBytes: number;
};

/**
 * The `ffmpeg` event + `ffmpeg_status` payload: the clearly-labeled,
 * on-demand wire-codec component's state machine.
 */
export type FfmpegStatus =
  | { state: "missing"; build: FfmpegBuild | null }
  | {
      state: "downloading";
      receivedBytes: number;
      totalBytes: number | null;
      bytesPerSec: number;
    }
  | { state: "verifying" }
  | { state: "extracting" }
  | { state: "ready"; version: string; path: string }
  | { state: "error"; message: string; build: FfmpegBuild | null };
