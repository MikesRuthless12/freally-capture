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
/** Audio Mixer strip orientation. */
export type MixerLayout = "horizontal" | "vertical";

export type Settings = {
  language: string;
  showStatsDock: boolean;
  /** The audio monitor output device name (null/"" = the OS default). */
  monitorDevice: string | null;
  /** Audio Mixer strip orientation. */
  mixerLayout: MixerLayout;
  /** Recording output configuration (Phase 4). */
  recording: RecordingSettings;
  /** Remote Guests networking (Phase R). */
  remote: RemoteSettings;
  /** Live-stream configuration (Phase 5). */
  stream: StreamSettings;
  /** The rolling replay buffer (Phase 6). */
  replay: ReplaySettings;
  /** Studio Mode's commit transition (Phase 5). */
  transition: TransitionSettings;
  /** Global action hotkeys (Phase 5). */
  hotkeys: HotkeySettings;
  /** The WebSocket remote-control API (Phase 7). */
  remoteControl: RemoteControlSettings;
  /** Browser docks — named URLs opened as dock windows (Phase 7). */
  browserDocks: BrowserDockSettings[];
  /** Sandboxed Lua scripts (Phase 7). */
  scripts: ScriptSettings[];
};

/** One browser dock: a named URL opened as its own dock window. */
export type BrowserDockSettings = {
  name: string;
  url: string;
};

/** One sandboxed Lua script: a .lua file path, loaded while enabled. */
export type ScriptSettings = {
  path: string;
  enabled: boolean;
};

/** The WebSocket remote-control API (mirrors `RemoteControlSettings` in
 * settings.rs). Off by default; requires a password; loopback unless `lan`. */
export type RemoteControlSettings = {
  enabled: boolean;
  /** TCP port (1024–65535). */
  port: number;
  /** Accept LAN connections (0.0.0.0) instead of loopback only. */
  lan: boolean;
  /** A secret — masked in the UI; auth is challenge–response, the password
   * itself never crosses the wire. */
  password: string;
};

/** The rolling replay buffer (mirrors `ReplaySettings` in settings.rs). */
export type ReplaySettings = {
  /** How much history Save keeps, in seconds (5–300). */
  seconds: number;
  /** CBR video bitrate of the buffer's own encode. */
  bitrateKbps: number;
  audioBitrateKbps: number;
  fps: number;
  /** The mixer track the buffer records (1-based). */
  track: number;
};

/** The `replay` event payload / `replay_status` result. */
export type ReplayStatus = {
  armed: boolean;
  /** "idle" | "buffering" | "recovering" | "failed". */
  state: "idle" | "buffering" | "recovering" | "failed";
  /** The armed window length (0 when idle). */
  seconds: number;
  error?: string;
  lastSaved?: string;
};

/** Global action hotkeys — accelerator strings ("Ctrl+Shift+R", "F13") or
 * null. Per-source PTT/PTM live in the mixer's audio settings. */
export type HotkeySettings = {
  /** Toggle recording start/stop. */
  record: string | null;
  /** Toggle Go Live / End Stream. */
  goLive: string | null;
  /** Commit the Studio-Mode Preview → Program transition. */
  transition: string | null;
  /** Save the replay buffer's last N seconds (Phase 6). */
  saveReplay: string | null;
  /** Drop a chapter marker into the active recording (Phase 6). */
  addMarker: string | null;
};

/** Studio Mode's commit transition (Phase 5). Stinger lands with the Phase 6
 * packs. */
export type TransitionKind =
  | "cut"
  | "fade"
  | "slideLeft"
  | "slideRight"
  | "slideUp"
  | "slideDown"
  | "swipeLeft"
  | "swipeRight"
  | "lumaLinear"
  | "lumaRadial"
  | "lumaHorizontal"
  | "lumaDiamond"
  | "lumaClock"
  | "lumaImage"
  | "stinger";

export const TRANSITION_KINDS: Array<[TransitionKind, string]> = [
  ["cut", "Cut"],
  ["fade", "Fade"],
  ["slideLeft", "Slide ←"],
  ["slideRight", "Slide →"],
  ["slideUp", "Slide ↑"],
  ["slideDown", "Slide ↓"],
  ["swipeLeft", "Swipe ←"],
  ["swipeRight", "Swipe →"],
  ["lumaLinear", "Luma wipe (linear)"],
  ["lumaRadial", "Luma wipe (radial)"],
  ["lumaHorizontal", "Luma wipe (horizontal)"],
  ["lumaDiamond", "Luma wipe (diamond)"],
  ["lumaClock", "Luma wipe (clock)"],
  ["lumaImage", "Image wipe (custom)"],
  ["stinger", "Stinger (video)"],
];

export type TransitionSettings = {
  kind: TransitionKind;
  durationMs: number;
  /** The grayscale wipe image for `lumaImage`. */
  lumaImage: string;
  /** The video file for `stinger`. */
  stingerPath: string;
  /** When the scene swap lands under the stinger, ms into the transition. */
  stingerCutMs: number;
};

/** The services the stream target picker offers (`srt`/`whip` are the
 * Phase 6 protocol targets — self-hosted SRT ingest / WebRTC WHIP endpoint). */
export type StreamService =
  "twitch" | "youTube" | "kick" | "facebook" | "trovo" | "custom" | "srt" | "whip";

export const STREAM_SERVICES: Array<[StreamService, string]> = [
  ["twitch", "Twitch"],
  ["youTube", "YouTube"],
  ["kick", "Kick"],
  ["facebook", "Facebook"],
  ["trovo", "Trovo"],
  ["custom", "Custom (RTMP/RTMPS)"],
  ["srt", "SRT (self-hosted)"],
  ["whip", "WHIP (WebRTC)"],
];

/** One stream target (mirrors `StreamTargetSettings` in settings.rs).
 * The stream key is a SECRET — masked in the UI, never logged. */
export type StreamTargetSettings = {
  /** Go Live publishes to every enabled target at once. */
  enabled: boolean;
  service: StreamService;
  /** Which canvas this target publishes. */
  canvas: "main" | "vertical";
  /** Overrides the service's preset ingest when non-empty. */
  ingestUrl: string;
  /** SECRET. */
  streamKey: string;
  /** ffmpeg encoder id, or "auto" = best detected H.264 encoder. */
  encoderId: string;
  /** CBR video bitrate. */
  bitrateKbps: number;
  audioBitrateKbps: number;
  keyframeSec: number;
  fps: number;
  /** The mixer track that goes to this target (1-based). */
  track: number;
  /** Publish at this size instead of the canvas size (0 = canvas). */
  outputWidth: number;
  outputHeight: number;
};

/** Live-stream configuration (mirrors `StreamSettings` in settings.rs):
 * the target list — targets with equal encode settings share one encode. */
export type StreamSettings = {
  targets: StreamTargetSettings[];
  /** Start a local recording automatically on Go Live. */
  autoRecord: boolean;
};

/** One target's slice of the `stream` event payload. */
export type StreamTargetStatus = {
  /** The settings row this target came from. */
  id: number;
  label: string;
  state: "live" | "reconnecting" | "failed" | "ended";
  error?: string;
  reconnects: number;
  framesDropped: number;
  /** Publish bitrate (measured, or the configured rate on a shared lane). */
  kbps: number;
  /** How many other targets share this target's encode. */
  shared: number;
};

/** The `stream` event payload / `stream_status` result. */
export type StreamStatus = {
  /** "idle" | "live" | "reconnecting" | "failed" | "ended". */
  state: "idle" | "live" | "reconnecting" | "failed" | "ended";
  error?: string;
  elapsedSec: number;
  reconnects: number;
  framesDropped: number;
  /** The enabled services, joined (e.g. "Twitch + YouTube"). */
  service: string;
  /** Per-target health + bitrate (empty when idle). */
  targets: StreamTargetStatus[];
};

/** Remote Guests networking — the user's own **opt-in** TURN relay (never
 * author-served). Empty URL = direct P2P only. The credential is a secret. */
export type RemoteSettings = {
  turnUrl: string;
  turnUsername: string;
  turnCredential: string;
};

/** Recording containers; `frec` is the owned lossless default. */
export type Container = "frec" | "mkv" | "mp4" | "mov" | "webm";

export type RcMode = "cbr" | "vbr" | "cqp";

export type RateControl = {
  mode: RcMode;
  bitrateKbps: number;
  /** Constant-quality value for `cqp` (0–51). */
  cq: number;
};

/** The quality/speed trade, mapped onto each encoder family's knob. */
export type EncPreset = "quality" | "balanced" | "performance";

/** Recording configuration (mirrors `RecordingSettings` in settings.rs). */
export type RecordingSettings = {
  container: Container;
  /** ffmpeg encoder id, or "auto" = best detected H.264 encoder. */
  encoderId: string;
  rateControl: RateControl;
  preset: EncPreset;
  keyframeSec: number;
  fps: number;
  audioBitrateKbps: number;
  /** Bitmask of the mixer tracks to record (bit 0 = track 1). */
  tracksMask: number;
  /** Output folder ("" = the OS Videos folder). */
  folder: string;
  filenamePrefix: string;
  /** Split into playable segments every N minutes (0 = off). */
  splitMinutes: number;
  /** Also record the vertical canvas (a parallel "… (vertical)" file). */
  recordVertical: boolean;
  /** Encode at this size instead of the canvas (0 = canvas; wire only). */
  outputWidth: number;
  outputHeight: number;
};

/** One file in the recordings folder (`recordings_list`). */
export type RecordingFile = {
  path: string;
  name: string;
  sizeBytes: number;
  modifiedMs: number;
  /** Lowercase extension ("frec", "mkv", …). */
  ext: string;
};

/** The `recording` event + `recording_status` payload. */
export type RecordingStatus =
  | { state: "idle"; lastPaths: string[]; error: string | null }
  | {
      state: "recording";
      durationSec: number;
      path: string;
      container: Container;
      tracks: number;
      framesDuplicated: number;
      framesBehind: number;
      audioBlocksDropped: number;
      /** Chapter markers dropped so far. */
      markers: number;
    }
  | {
      state: "paused";
      durationSec: number;
      path: string;
      container: Container;
      tracks: number;
    }
  | { state: "finalizing"; path: string };

/** The `stats` push-event payload (~2 Hz). */
export type StatsPayload = {
  /** Composed frames per second (the program render rate). */
  fps: number;
  /** The second (vertical) canvas's compose rate (0 = none running). */
  verticalFps: number;
  /** This process's CPU usage, percent of the whole machine. */
  cpu: number;
  /** This process's resident memory, MiB. */
  memoryMb: number;
  /** Frames the capture pipeline dropped since the session began. */
  dropped: number;
  /** Mean GPU compose time per frame, milliseconds. */
  renderMs: number;
  /** True only in the brief pre-compose-loop startup window. */
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

/** One of the four corners the screen-plus-corners layout can seat a camera in. */
export type Corner = "topLeft" | "topRight" | "bottomLeft" | "bottomRight";

/** The corners in host-first fill order (top-right, then the rest). */
export const CORNERS: Corner[] = ["topRight", "topLeft", "bottomRight", "bottomLeft"];

/** A rectangle in normalized canvas coordinates (0..1, origin top-left). */
export type NormRect = { x: number; y: number; w: number; h: number };

/** One corner assignment for the screen-plus-corners layout. */
export type CornerSlot = { itemId: ItemId; corner: Corner };

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
  | { kind: "media"; path: string; loop: boolean; hwDecode: boolean }
  | { kind: "remoteGuest"; label: string }
  | { kind: "color"; color: Rgba; width: number; height: number }
  | { kind: "nestedScene"; scene: SceneId }
  | {
      kind: "chatOverlay";
      youtube: string;
      twitch: string;
      kick: string;
      width: number;
      maxLines: number;
      fontSize: number;
    }
  | {
      kind: "slideshow";
      paths: string[];
      slideMs: number;
      transitionMs: number;
      loop: boolean;
      shuffle: boolean;
    }
  | { kind: "audioInput"; deviceId: string }
  | { kind: "audioOutput"; deviceId: string }
  | { kind: "appAudio"; pid: number; exe: string }
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
  return (
    kind === "audioInput" ||
    kind === "audioOutput" ||
    kind === "appAudio" ||
    kind === "media" ||
    kind === "remoteGuest"
  );
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

/** One app currently making sound (the App Audio picker rows). */
export type AppAudioApp = {
  pid: number;
  name: string;
  exe: string;
};

/** The App Audio picker payload + the honest per-OS guidance. */
export type AppAudioList = {
  apps: AppAudioApp[];
  supported: boolean;
  guidance: string;
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
  | { type: "colorKey"; key: Rgba; similarity: number; smoothness: number }
  | { type: "lumaKey"; lumaMin: number; lumaMax: number; smoothness: number }
  | { type: "renderDelay"; delayMs: number }
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
  /** When set, the first-frame fit targets this normalized slot (a layout corner). */
  pendingSlot?: NormRect;
  filters: Filter[];
};

/** One scene: ordered items, index = z-order, `items[0]` bottom-most. */
/** One item's pre-focus placement (Highlight Speaker restore buffer). */
export type FocusRestore = {
  item: ItemId;
  transform: Transform;
  visible: boolean;
};

/** Highlight Speaker: `item` fills the canvas; `prior` restores on toggle-off. */
export type FocusState = {
  item: ItemId;
  prior: FocusRestore[];
};

/** A named set of items that move / show / hide together (Phase 6). */
export type SourceGroup = {
  id: string;
  name: string;
  items: ItemId[];
  /** ANDs with each member's own eye toggle. */
  visible: boolean;
};

/** One source's per-scene mixer override (Phase 6). */
export type SceneAudioOverride = {
  source: SourceId;
  volumeDb: number;
  muted: boolean;
};

export type Scene = {
  id: SceneId;
  name: string;
  items: SceneItem[];
  focus?: FocusState | null;
  /** Source groups (Phase 6). */
  groups?: SourceGroup[];
  /** Per-scene mixer overrides (Phase 6). */
  audioOverrides?: SceneAudioOverride[];
};

/** The second output canvas (Phase 6): its own size + the scene it shows. */
export type VerticalCanvas = {
  width: number;
  height: number;
  /** The scene this canvas composes (independent of the program scene). */
  scene: SceneId;
};

/** The whole model (the on-disk scene-collection format). */
export type Collection = {
  formatVersion: number;
  canvasWidth: number;
  canvasHeight: number;
  sources: Source[];
  scenes: Scene[];
  activeScene: SceneId;
  /** The optional second (vertical) output canvas (Phase 6). */
  vertical?: VerticalCanvas | null;
};

/** The `studio` event / `studio_get` payload. */
export type StudioDto = {
  revision: number;
  collection: Collection;
  /** Studio Mode (Phase 5): present while enabled. */
  studioMode?: StudioModeDto;
};

/** The Studio-Mode slice of the model (session state, never persisted). */
export type StudioModeDto = {
  previewScene: SceneId;
  /** A Preview→Program blend is running right now. */
  transitioning: boolean;
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

/** The anonymous bug-report context (mirrors `BugReportContextDto`). */
export type BugReportContext = {
  appVersion: string;
  os: string;
  arch: string;
  /** The anonymous system line always included in a report. */
  diagnostics: string;
  /** The scrubbed crash text from the previous run, if the app crashed. */
  pendingCrash: string | null;
};

/**
 * The `recording-export` event: a .frec → wire-container export's progress and
 * terminal state (mirrors `ExportStatusDto` in commands/recording.rs).
 */
export type ExportStatus =
  | { state: "exporting"; framesDone: number; framesTotal: number }
  | { state: "done"; path: string }
  | { state: "error"; message: string }
  | { state: "cancelled" };
