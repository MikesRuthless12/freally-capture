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

/** What the About panel shows (mirrors `BuildInfo` in `buildinfo.rs`). */
export type BuildInfo = {
  version: string;
  authors: string;
  projectStarted: string;
  /** `null` until 1.0.0 ships. */
  firstStableReleased: string | null;
  copyright: string;
  homepage: string;
  repository: string;
  issues: string;
  os: string;
  arch: string;
  target: string;
};

/** The running version's changelog, for Help → What's New (mirrors
 *  `ReleaseNotes` in `buildinfo.rs`). `notes` is Markdown, or `null` if this
 *  build has no changelog section. */
export type ReleaseNotes = {
  version: string;
  notes: string | null;
};

/** What `autoconfig_suggest` proposes (mirrors `AutoConfig` in autoconfig.rs).
 * `encoderReason` / `qualityReason` are i18n KEYS, not sentences. */
export type AutoConfig = {
  encoderId: string;
  encoderLabel: string;
  hardware: boolean;
  width: number;
  height: number;
  fps: number;
  bitrateKbps: number;
  encoderReason: string;
  qualityReason: string;
  gpus: string[];
  physicalCores: number;
};

/** Which palette the UI paints with (mirrors `ThemeMode` in settings.rs). */
export type ThemeMode = "dark" | "light" | "custom";

/** Appearance (mirrors `ThemeSettings` in settings.rs). */
export type ThemeSettings = {
  mode: ThemeMode;
  /** `#rrggbb` — Rust's `validate()` rejects anything else. */
  accent: string;
};

/** Preview alignment aids (CAP-M04) — mirrors `AlignmentSettings` in settings.rs. */
export type AlignmentSettings = {
  /** Snap a dragged item to canvas + other-item edges/centers. */
  smartGuides: boolean;
  /** Draw action-safe + title-safe rectangles over the preview. */
  safeAreas: boolean;
  /** Draw px rulers in the gutter around the preview. */
  rulers: boolean;
};

/** Which palette the mixer's level meters use (mirrors `MeterPreset` in
 * settings.rs). `colorblind` is an Okabe–Ito blue→orange→vermillion ramp. */
export type MeterPreset = "default" | "colorblind" | "custom";

/** Accessibility (mirrors `AccessibilitySettings` in settings.rs): the mixer
 * VU meter's three zone colours. `#rrggbb` — Rust's `validate()` rejects
 * anything else; the custom colours are only read for the `custom` preset. */
export type AccessibilitySettings = {
  meterPreset: MeterPreset;
  meterLow: string;
  meterMid: string;
  meterHigh: string;
};

/** The persisted user settings (`settings.json` in the OS config dir). */
/** Audio Mixer strip orientation. */
export type MixerLayout = "horizontal" | "vertical";

/** Which mixer bus a CAP-N30 physical-output route carries (flattened onto the
 *  route object, so a route is `{ bus: "master", … }` or
 *  `{ bus: "track", index, … }`). */
export type OutputBus = { bus: "master" } | { bus: "track"; index: number };

/** One CAP-N30 physical-output route: a program bus → an output device with a
 *  trim. `deviceId` "" = the OS default output. */
export type AudioOutputRoute = OutputBus & {
  deviceId: string;
  /** Output trim in dB (−60..=6); the floor is silence, 0 passes through. */
  gainDb: number;
};

/** CAP-N34 loudness normalization (the live program rider). */
export type LoudnessSettings = {
  enabled: boolean;
  /** Integrated-loudness target, LUFS (−14 / −16 / −23). */
  targetLufs: number;
  /** Peak ceiling, dBFS. */
  ceilingDb: number;
};

/** CAP-N47 SMPTE LTC timecode (generator + reader). */
export type LtcSettings = {
  enabled: boolean;
  /** The track bus the generator rides (0-based, 0..=5). */
  track: number;
  /** LTC frame rate: 24, 25 or 30. */
  fps: number;
  /** The source id whose raw input the reader taps ("" = off). */
  readSource: string;
};

/** One CAP-N37 soundboard pad. */
export type SoundboardPad = {
  id: string;
  name: string;
  path: string;
  hotkey?: string | null;
  gainDb: number;
  /** Track bitmask (bit 0 = track 1). */
  tracks: number;
  /** Choke group 1–8 (0 = none). */
  chokeGroup: number;
  looping: boolean;
  autoDuck: boolean;
};

export type SoundboardSettings = { pads: SoundboardPad[] };

export type Settings = {
  language: string;
  showStatsDock: boolean;
  /** The audio monitor output device name (null/"" = the OS default). */
  monitorDevice: string | null;
  /** CAP-N30 program-bus output routes: master / track buses → devices. */
  audioOutputs?: AudioOutputRoute[];
  /** CAP-N34 loudness normalization (the live rider). */
  loudness?: LoudnessSettings;
  /** CAP-N47 SMPTE LTC timecode (generator + reader). */
  ltc?: LtcSettings;
  /** CAP-N37 soundboard pads. */
  soundboard?: SoundboardSettings;
  /** Audio Mixer strip orientation. */
  mixerLayout: MixerLayout;
  /** Appearance: palette + custom accent (Phase 9). */
  theme: ThemeSettings;
  /** Preview alignment aids: smart guides, safe areas, rulers (CAP-M04). */
  alignment: AlignmentSettings;
  /** Accessibility: the mixer VU meter palette. */
  accessibility?: AccessibilitySettings;
  /** Whether the first-run wizard has been seen (Phase 9). */
  completedOnboarding: boolean;
  /** CAP-M18: device id → control tag → value. Server-owned — written by
   * cameraControlSet and PRESERVED across settingsSet (never edit here). */
  cameraProfiles?: Record<string, Record<string, number>>;
  /** CAP-N74: capture id → HDR tone-map. Server-owned — written by
   * hdrToneMapSet and PRESERVED across settingsSet (never edit here). */
  hdrToneMap?: Record<string, HdrToneMapSetting>;
  /** CAP-N19: capture id → cursor effects. Server-owned — written by
   * cursorFxSet and PRESERVED across settingsSet (never edit here). */
  cursorFx?: Record<string, CursorFxSetting>;
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
  /** The panic button's privacy slate (CAP-M22). */
  panicSlate: PanicSlateSettings;
  /** The WebSocket remote-control API (Phase 7). */
  remoteControl: RemoteControlSettings;
  /** Browser docks — named URLs opened as dock windows (Phase 7). */
  browserDocks: BrowserDockSettings[];
  /** Sandboxed Lua scripts (Phase 7). */
  scripts: ScriptSettings[];
  /** Automation: rules + macros (CAP-N01/N02). Rules ship disabled. */
  automation?: AutomationSettings;
  /** The show rundown (CAP-N09). Auto-advance ships off. */
  rundown?: RundownSettings;
  /** The LAN touch panel + tally service (CAP-N06/N07). Off by default. */
  webPanel?: WebPanelSettings;
  /** The OSC control surface (CAP-N04). Off by default. */
  osc?: OscSettings;
  /** PTZ cameras (CAP-N08). Empty by default. */
  ptz?: PtzSettings;
  /** MIDI control surfaces (CAP-N03). No port opens until one is picked. */
  midi?: MidiSettings;
  /** The Freally Link output (CAP-N12). Off by default. */
  link?: LinkSettings;
};

/** The Freally Link output (CAP-N12): share the program with one other
 * Freally instance on the LAN. Off by default; one receiver at a time;
 * v1 ships motion-JPEG + uncompressed audio over TCP — said in the UI. */
export type LinkSettings = {
  enabled: boolean;
  port: number;
  /** The name discovery advertises ("" = "Freally Capture"). */
  name: string;
  /** The pairing key receivers must present before a frame is served —
   * required (8+ characters) to enable the output. */
  key: string;
};

/** One Freally Link output found by the LAN scan (CAP-N12). */
export type LinkPeer = {
  name: string;
  host: string;
  port: number;
};

/** The fixed studio-command allowlist automation actions draw from
 * (CAP-N01/N02) — mirrors `ALLOWED_COMMANDS` in remote_api.rs. An action can
 * be nothing else: no file paths, no processes, no network. */
export const ALLOWED_COMMANDS = [
  "getStatus",
  "listScenes",
  "setProgramScene",
  "setPreviewScene",
  "setStudioMode",
  "transition",
  "startStream",
  "stopStream",
  "startRecording",
  "stopRecording",
  "pauseRecording",
  "addMarker",
  "armReplay",
  "saveReplay",
  "setAudioMuted",
  "setAudioVolume",
  "setFilterEnabled",
  "runMacro",
] as const;

/** One step of a macro (CAP-N02). */
export type MacroStep =
  | { kind: "action"; command: string; params?: unknown }
  | { kind: "wait"; ms: number }
  | { kind: "setVariable"; name: string; value: string };

/** One named macro (CAP-N02); `hotkey` is an OS-global accelerator. */
export type AutomationMacro = {
  name: string;
  steps: MacroStep[];
  repeat: number;
  /** Plain ("Ctrl+Shift+M") or a chord ("Ctrl+K, 3" — CAP-N05). */
  hotkey?: string;
  /** The hotkey layer this accelerator belongs to (CAP-N05); absent = all. */
  layer?: number;
};

/** What makes a rule fire (CAP-N01). Idle/focus are Windows-only signals —
 * elsewhere those triggers simply never fire, and the UI says so. */
export type AutomationTrigger =
  | { kind: "sceneSwitched"; scene: string }
  | { kind: "streamState"; live: boolean }
  | { kind: "recordingState"; recording: boolean }
  | { kind: "sourceError"; source: string }
  | { kind: "audioLevel"; source: string; thresholdDb: number; above: boolean }
  | { kind: "systemIdle"; seconds: number }
  | { kind: "windowFocus"; exe: string }
  | { kind: "timeOfDay"; at: string }
  | { kind: "fileChanged"; path: string };

/** A condition gate — all must hold for a rule's actions to run. */
export type AutomationCondition =
  | { kind: "variableEquals"; name: string; value: string }
  | { kind: "streaming"; live: boolean }
  | { kind: "recording"; recording: boolean };

/** One automation rule (CAP-N01). Ships disabled. */
export type AutomationRule = {
  name: string;
  enabled: boolean;
  trigger: AutomationTrigger;
  conditions: AutomationCondition[];
  actions: MacroStep[];
  macroName: string;
};

/** Rules + macros (CAP-N01/N02). */
export type AutomationSettings = {
  rules: AutomationRule[];
  macros: AutomationMacro[];
};

/** One rundown step (CAP-N09): a scene, a hold, and optional actions. */
export type RundownStep = {
  name: string;
  /** The scene to cut to; "" = stay on the current one. */
  scene: string;
  /** Seconds to hold before auto-advance; 0 = manual only. */
  holdSecs: number;
  actions: MacroStep[];
};

/** The show rundown (CAP-N09). Auto-advance ships off. */
export type RundownSettings = {
  steps: RundownStep[];
  autoAdvance: boolean;
};

/** The rundown's live state — "next up + remaining time". */
export type RundownStatus = {
  at?: number;
  remainingSecs?: number;
  nextUp?: string;
  running: boolean;
};

/** The LAN touch panel + tally service (CAP-N06/N07). Off by default;
 * password required; loopback unless `lan`. */
export type WebPanelSettings = {
  enabled: boolean;
  port: number;
  lan: boolean;
  password: string;
};

/** The OSC control surface (CAP-N04). Off by default; LAN-only. */
export type OscSettings = {
  enabled: boolean;
  port: number;
  lan: boolean;
};

/** A learned MIDI control (CAP-N03). */
export type MidiControl =
  { kind: "note"; channel: number; note: number } | { kind: "cc"; channel: number; cc: number };

/** What a MIDI control drives (CAP-N03). Actions ride the fixed allowlist. */
export type MidiTarget =
  | { kind: "action"; command: string; params?: unknown }
  | { kind: "macro"; name: string }
  | { kind: "volume"; source: string }
  | { kind: "mute"; source: string }
  | { kind: "scene"; scene: string };

/** One learned MIDI binding. */
export type MidiBinding = {
  control: MidiControl;
  target: MidiTarget;
  /** Light the pad's LED / drive the motor fader from the studio's state. */
  feedback: boolean;
};

/** MIDI control surfaces (CAP-N03). No port opens until one is picked. */
export type MidiSettings = {
  input: string;
  output: string;
  bindings: MidiBinding[];
};

/** Which way a PTZ head is driven (CAP-N08). */
export type PtzMoveDirection =
  "up" | "down" | "left" | "right" | "upLeft" | "upRight" | "downLeft" | "downRight" | "stop";

/** One named PTZ preset (a VISCA memory slot). */
export type PtzPreset = { name: string; slot: number };

/** "When this scene goes on program, recall this preset." */
export type PtzSceneRecall = { scene: string; slot: number };

/** One PTZ camera the operator entered (CAP-N08). Never auto-discovered. */
export type PtzCamera = {
  name: string;
  host: string;
  port: number;
  presets: PtzPreset[];
  sceneRecalls: PtzSceneRecall[];
};

/** PTZ cameras (CAP-N08). Empty by default. */
export type PtzSettings = { cameras: PtzCamera[] };

/** One display's HDR→SDR tone-map (CAP-N74). */
export type HdrToneMapSetting = {
  /** "clip" | "maxRgb" | "reinhard" | "bt2408". */
  operator: string;
  /** 80–1000. */
  paperWhiteNits: number;
};

/** One capture's cursor effects (CAP-N19) — drawn into the frames on the
 * owned (Windows) cursor path. Colors are `#rrggbb`. */
export type CursorFxSetting = {
  halo: boolean;
  haloColor: string;
  /** 8–128 px. */
  haloRadius: number;
  ripples: boolean;
  leftColor: string;
  rightColor: string;
  keystrokes: boolean;
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
  /** Grab a still frame of the program (CAP-M08). */
  still: string | null;
  /** Cut to the privacy slate + hard-mute (CAP-M22). Engage only. */
  panic: string | null;
  /** Start/pause every timer source (CAP-M15). */
  timerToggle: string | null;
  /** Reset every timer source (CAP-M15). */
  timerReset: string | null;
  /** Punch-in zoom presets (CAP-N71). */
  zoom100: string | null;
  zoom150: string | null;
  zoom200: string | null;
  /** Split-timer keys (CAP-N18) — each drives every live split timer. */
  splitTimerSplit: string | null;
  splitTimerUndo: string | null;
  splitTimerSkip: string | null;
  splitTimerReset: string | null;
  /** Playlist transport (CAP-N17) — drives every live playlist. */
  playlistNext: string | null;
  playlistPrevious: string | null;
  /** Roll every live Instant Replay source (CAP-N10). */
  replayRoll: string | null;
};

/** The panic button's privacy slate (CAP-M22). */
export type PanicSlateSettings = {
  /** `#rrggbb`. */
  color: string;
  /** Optional image path ("" = colour only), drawn native-size centered. */
  image: string;
};

/** What a still-frame grab captures (CAP-M08). Mirrors `StillTarget` in studio.rs. */
export type StillTarget =
  { kind: "program" } | { kind: "source"; item: ItemId; preFilter: boolean };

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
  | "stinger"
  | "move";

/**
 * `[value, i18n key]` — the second element is a catalog key, not English. Call
 * sites render `t(key)`. These labels reach a `<select>`, so leaving them as
 * English literals here would have quietly excluded them from translation while
 * the parity lint stayed green.
 *
 * `ui/src/__tests__/i18n.test.ts` asserts every key below exists in `en.ftl` —
 * the lint cannot, because `t(key)` on a variable is invisible to it.
 */
export const TRANSITION_KINDS: Array<[TransitionKind, string]> = [
  ["cut", "transition-kind-cut"],
  ["fade", "transition-kind-fade"],
  ["slideLeft", "transition-kind-slide-left"],
  ["slideRight", "transition-kind-slide-right"],
  ["slideUp", "transition-kind-slide-up"],
  ["slideDown", "transition-kind-slide-down"],
  ["swipeLeft", "transition-kind-swipe-left"],
  ["swipeRight", "transition-kind-swipe-right"],
  ["lumaLinear", "transition-kind-luma-linear"],
  ["lumaRadial", "transition-kind-luma-radial"],
  ["lumaHorizontal", "transition-kind-luma-horizontal"],
  ["lumaDiamond", "transition-kind-luma-diamond"],
  ["lumaClock", "transition-kind-luma-clock"],
  ["lumaImage", "transition-kind-image"],
  ["stinger", "transition-kind-stinger"],
  ["move", "transition-kind-move"],
];

/** How a track-matte stinger packs its transparency (CAP-N29). */
export type StingerMatte = "none" | "horizontal" | "vertical";

/** `[value, i18n key]` for the track-matte picker (see `TRANSITION_KINDS`). */
export const STINGER_MATTES: Array<[StingerMatte, string]> = [
  ["none", "stinger-matte-none"],
  ["horizontal", "stinger-matte-horizontal"],
  ["vertical", "stinger-matte-vertical"],
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
  /** How a track-matte stinger carries transparency (CAP-N29). */
  stingerMatte: StingerMatte;
  /** dB the program ducks under the stinger's own audio (CAP-N29); 0 = off. */
  stingerDuckDb: number;
};

/** The services the stream target picker offers (`srt`/`whip` are the
 * Phase 6 protocol targets — self-hosted SRT ingest / WebRTC WHIP endpoint). */
export type StreamService =
  "twitch" | "youTube" | "kick" | "facebook" | "trovo" | "custom" | "srt" | "whip";

/**
 * `[value, i18n key]` — see [`TRANSITION_KINDS`]. The brand names round-trip
 * through the catalog unchanged so that every locale carries every key (parity),
 * but `Custom (RTMP/RTMPS)`, `SRT (self-hosted)` and `WHIP (WebRTC)` each hide a
 * translatable word behind a protocol name.
 */
export const STREAM_SERVICES: Array<[StreamService, string]> = [
  ["twitch", "stream-service-twitch"],
  ["youTube", "stream-service-youtube"],
  ["kick", "stream-service-kick"],
  ["facebook", "stream-service-facebook"],
  ["trovo", "stream-service-trovo"],
  ["custom", "stream-service-custom"],
  ["srt", "stream-service-srt"],
  ["whip", "stream-service-whip"],
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
  /** CAP-M09: refuse "Go Live anyway" until every blocking item is green. */
  preflightHold: boolean;
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
/** CAP-N38 audio-only recording format. */
export type AudioRecFormat = "wav" | "flac" | "opus";

/** CAP-N38 audio-only recording status (polled). */
export type AudioRecStatus =
  | { state: "idle" }
  | { state: "recording"; durationSec: number; path: string; tracks: number; paused: boolean };

export type RecordingSettings = {
  container: Container;
  /** CAP-N38 audio-only recording format (WAV owned; FLAC/Opus via ffmpeg). */
  audioFormat?: AudioRecFormat;
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
  /** The `{prefix}` token's value (CAP-M25). */
  filenamePrefix: string;
  /** Token filename templates (CAP-M25) for recordings, replays, stills. */
  template: string;
  replayTemplate: string;
  stillTemplate: string;
  /** Per-output folders ("" = the recordings folder) (CAP-M25). */
  replayFolder: string;
  stillFolder: string;
  /** The persisted `{counter}` token value — server-owned, read-only here. */
  counter: number;
  /** Split into playable segments every N minutes (0 = off). */
  splitMinutes: number;
  /** Also record the vertical canvas (a parallel "… (vertical)" file). */
  recordVertical: boolean;
  /** Encode at this size instead of the canvas (0 = canvas; wire only). */
  outputWidth: number;
  outputHeight: number;
  /** CAP-N40 ISO recording: source ids recorded as their own clean files. */
  isoSources: string[];
  /** ISO lanes record post-filter (as processed) or raw pre-filter. */
  isoPostFilter: boolean;
  /** The ISO lanes' own container + encoder (independent of the program's). */
  isoContainer: Container;
  isoEncoderId: string;
  /** CAP-N42: record the program with real transparency (.frec only). */
  alphaFrec: boolean;
  /** CAP-N43: event-driven split triggers (the owned .frec splitter only). */
  splitOnScene: boolean;
  splitOnMarker: boolean;
  splitOnRundown: boolean;
  /** CAP-N44: studio events drop typed chapter markers automatically. */
  autoMarkers: boolean;
  /** CAP-N45: the post-record pipeline (closed action set, per-profile). */
  pipelineEnabled: boolean;
  pipeline: PipelineStep[];
};

/** CAP-N45: one post-record pipeline step (a CLOSED action set — there is
 * deliberately no "run a command" variant). */
export type PipelineStep =
  | { action: "verify" }
  | { action: "remux" }
  | { action: "normalize" }
  | { action: "rename"; template: string }
  | { action: "move"; folder: string }
  | { action: "copy"; folder: string }
  | { action: "reveal" }
  | { action: "luaEvent" };

/** CAP-N45: one step's live status in the queue view. */
export type PipelineStepStatus = {
  action: string;
  status: "pending" | "running" | "ok" | "warn" | "fail" | "skipped";
  detail: string;
};

/** CAP-N45: one queued/finished pipeline job (`pipeline_status` + event). */
export type PipelineJob = {
  id: number;
  file: string;
  steps: PipelineStepStatus[];
  done: boolean;
};

/** CAP-N46: one integrity check in a `recording_verify` report. */
export type VerifyCheck = {
  /** Stable id: "container" | "video-continuity" | "audio-continuity" |
   * "av-interleave" | "duration". */
  id: string;
  status: "pass" | "warn" | "fail" | "skipped";
  detail: string;
};

/** CAP-N46: the integrity report — verdict is the worst check. */
export type VerifyReport = {
  verdict: "pass" | "warn" | "fail" | "skipped";
  checks: VerifyCheck[];
};

/** CAP-N41: the trim window's probe payload (`recording_trim_info`). */
export type TrimInfo = {
  durationSecs: number;
  fps: number;
  width: number;
  height: number;
  hasAudio: boolean;
  /** Keyframe presentation times, seconds, ascending. */
  keyframesSecs: number[];
};

/** One file in the recordings folder (`recordings_list`). */
export type RecordingFile = {
  path: string;
  name: string;
  sizeBytes: number;
  modifiedMs: number;
  /** Lowercase extension ("frec", "mkv", …). */
  ext: string;
  /** CAP-N42: a .frec flagged as carrying real transparency (null otherwise). */
  frecAlpha: boolean | null;
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
      /** CAP-N40: ISO lanes recording alongside the program (0 = none). */
      isoLanes: number;
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
  /** Free space on the recording drive, bytes; null if unknown (~5 s cadence). */
  diskFreeBytes: number | null;
  /** Recording write rate, bytes/sec; null when not recording. */
  burnBytesPerSec: number | null;
  /** Seconds until the recording drive fills at the current rate; null when not recording. */
  secsUntilFull: number | null;
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
  /** 3D tilt (CAP-N23), degrees; optional so older transforms round-trip. */
  rotationX?: number;
  rotationY?: number;
  /** Perspective strength for the 3D tilt, 0..=1 (0 = orthographic). */
  perspective?: number;
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

/** Which face a Timer source shows (CAP-M15). */
export type TimerMode = "wallClock" | "countdown" | "stopwatch" | "sinceLive" | "sinceRecording";

/** What a countdown does at zero (CAP-M15). */
export type CountdownEnd = "none" | "flash" | "switchScene";

/** How a Text source's bound file parses (CAP-M16). */
export type FileBinding = "whole" | "csvCell" | "jsonPointer";

/** One camera control a running device reports (CAP-M18). */
export type CameraControl = {
  /** Stable tag: "exposure" | "whiteBalance" | "focus" | "zoom" | "gain" | … */
  id: string;
  /** The backend's own display name (fallback label). */
  name: string;
  /** Absent when the backend reports no range (Windows: exposure/focus/zoom):
   * the UI shows a stepper rather than a meaningless slider. */
  min?: number;
  max?: number;
  step: number;
  default: number;
  value: number;
  writable: boolean;
};

/** A device source's deinterlace mode (CAP-M17). */
export type DeinterlaceMode = "off" | "discard" | "bob" | "linear" | "blend" | "motionAdaptive";

/** Which field an interlaced feed shot first (CAP-M17). */
export type FieldOrder = "topFirst" | "bottomFirst";

/** CAP-M14: one audited hotkey binding. */
export type HotkeyAuditEntry = {
  accelerator: string;
  action: string;
  feature: string;
  /** The audio source's name (PTT/PTM rows only). */
  source?: string;
  registered: boolean;
  /** How many OTHER bindings share this key. */
  sharedWith: number;
  valid: boolean;
};

export type VideoDeviceFormat = {
  width: number;
  height: number;
  fps: number;
  fourcc: string;
};

/** CAP-N18: which reference the split timer compares against. */
export type SplitComparison = "personalBest" | "bestSegments" | "average";

/** CAP-N17: one playlist entry (trims in seconds; 0 = the edge). */
export type PlaylistEntry = {
  path: string;
  /** In-trim, seconds (0 = the top). */
  in: number;
  /** Out-trim, seconds (0 = the end). */
  out: number;
  /** Cue points, seconds into the FILE (independent of the trims). */
  cues: number[];
};

/** CAP-N10: a replay roll's speed — retimed, never interpolated. */
export type ReplaySpeed = "full" | "half" | "quarter";

/** CAP-N11: which protocol the LAN ingest listener speaks. */
export type IngestProtocol = "srt" | "rtmp";

/** CAP-N16: how a Title animates in and out. */
export type TitleAnimation = "none" | "fade" | "slideLeft" | "slideUp" | "wipe";

/** CAP-N16: one title layer (serde tag = `kind`), drawn in list order —
 * later layers on top. */
export type TitleLayer =
  | {
      kind: "text";
      x: number;
      y: number;
      text: string;
      fontFamily?: string | null;
      fontFile?: string | null;
      sizePx: number;
      color: Rgba;
      align: TextAlign;
      /** Outline stroke width visible OUTSIDE the fill, px (0 = none). */
      outlinePx: number;
      outlineColor: Rgba;
      /** A soft drop shadow, scaled with the type size. */
      shadow: boolean;
      /** CAP-M16: a watched local file this cell binds to ("" = the text field). */
      sourceFile: string;
      binding: FileBinding;
      /** CSV: 1-based data row. */
      csvRow: number;
      /** CSV: column by header name or 1-based index. */
      csvColumn: string;
      /** JSON: an RFC 6901 pointer, e.g. /teams/0/score. */
      jsonPointer: string;
    }
  | { kind: "image"; x: number; y: number; path: string }
  | { kind: "rect"; x: number; y: number; width: number; height: number; color: Rgba };

/** CAP-N15: which face the audio visualizer draws. */
export type VisStyle = "bars" | "scope" | "vu";

/** CAP-N13: which fixed layout the input overlay draws. */
export type InputLayout = "wasd" | "keyboard" | "gamepad" | "fightstick";

/** CAP-N15: what the audio visualizer listens to. */
export type VisTargetKind = "master" | "track" | "source";

/** Per-kind source settings (serde tag = `kind`). */
export type SourceSettings =
  | { kind: "display"; captureId: string; label: string }
  | { kind: "window"; captureId: string; label: string }
  | { kind: "portal" }
  | {
      kind: "videoDevice";
      deviceId: string;
      format?: VideoDeviceFormat | null;
      /** CAP-M17: deinterlacing for interlaced feeds (changing restarts the device). */
      deinterlace: DeinterlaceMode;
      fieldOrder: FieldOrder;
    }
  | { kind: "image"; path: string }
  | {
      kind: "media";
      path: string;
      loop: boolean;
      hwDecode: boolean;
      /** Hold on the first frame until recording starts, then play from the
       * top (the backdrop's "start playback with recording" option). */
      startWithRecording?: boolean;
      /** True reverse playback (GIFs reverse natively; video renders a
       * reversed copy once, via the labeled ffmpeg component). */
      reverse?: boolean;
    }
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
      /** CAP-M16: a watched local file the content binds to; "" = the text field. */
      sourceFile: string;
      binding: FileBinding;
      /** CSV: 1-based data row. */
      csvRow: number;
      /** CSV: column by header name or 1-based index. */
      csvColumn: string;
      /** JSON: an RFC 6901 pointer, e.g. /teams/0/score. */
      jsonPointer: string;
    }
  | { kind: "testBars"; width: number; height: number }
  | { kind: "testGrid"; width: number; height: number }
  | { kind: "testSweep"; width: number; height: number }
  | { kind: "testTone" }
  | { kind: "testFlashBeep"; width: number; height: number }
  | {
      kind: "timer";
      mode: TimerMode;
      format: string;
      utcOffsetMin?: number | null;
      countdownMs: number;
      target: string;
      endAction: CountdownEnd;
      endScene?: SceneId | null;
      fontFamily?: string | null;
      fontFile?: string | null;
      sizePx: number;
      color: Rgba;
    }
  | {
      kind: "systemStats";
      showFps: boolean;
      showCpu: boolean;
      showMemory: boolean;
      showRenderMs: boolean;
      showDropped: boolean;
      showBitrate: boolean;
      /** CAP-N47: the burn-in timecode line (the LTC reader's decode). */
      showTimecode: boolean;
      fontFamily?: string | null;
      fontFile?: string | null;
      sizePx: number;
      color: Rgba;
    }
  | {
      kind: "audioVisualizer";
      style: VisStyle;
      target: VisTargetKind;
      /** 1-based track bus (target = "track"). */
      track: number;
      /** The bound strip (target = "source"). */
      source?: SourceId | null;
      width: number;
      height: number;
      /** Spectrum bar count (bars style; the renderer clamps to 8–128). */
      bands: number;
      color: Rgba;
      peakHold: boolean;
      /** Bar fall rate, dB/s (the renderer clamps to 6–120). */
      decay: number;
    }
  | {
      kind: "splitTimer";
      /** The `.lss` file (local only — network paths are refused). */
      path: string;
      comparison: SplitComparison;
      width: number;
      height: number;
      sizePx: number;
      color: Rgba;
      /** Ahead-of-comparison delta color. */
      ahead: Rgba;
      /** Behind-comparison delta color. */
      behind: Rgba;
      /** Gold-segment highlight color. */
      gold: Rgba;
    }
  | {
      kind: "playlist";
      items: PlaylistEntry[];
      loop: boolean;
      /** One shuffle draw per start (a looping shuffle repeats its order). */
      shuffle: boolean;
      /** Hold the last frame at the end (else clear to transparent). */
      holdLast: boolean;
      hwDecode: boolean;
      /** The studio variable fed the playing item's name ("" = off). */
      nowPlayingVariable: string;
    }
  | {
      kind: "replayPlayback";
      /** How much history a roll grabs, seconds (clamped to the buffer). */
      seconds: number;
      speed: ReplaySpeed;
      hwDecode: boolean;
    }
  | {
      kind: "lanIngest";
      protocol: IngestProtocol;
      /** The listen port (1024–65535; SRT defaults 9710, RTMP 1935). */
      port: number;
      /** SRT only. Empty = an open, unencrypted listener (the form warns). */
      passphrase: string;
    }
  | {
      kind: "inputOverlay";
      /** CAP-N13: read only while live, only the layout's fixed keys — never logged. */
      layout: InputLayout;
      /** The idle key-cap / outline color. */
      color: Rgba;
      /** The pressed-state fill. */
      accent: Rgba;
    }
  | {
      kind: "title";
      width: number;
      height: number;
      /** Drawn in list order — later layers on top. */
      layers: TitleLayer[];
      animation: TitleAnimation;
      /** The in/out animation length, ms. */
      durationMs: number;
    }
  | {
      kind: "freallyLink";
      /** The sending instance's address — an IPv4/hostname, no scheme. */
      host: string;
      port: number;
      /** The label discovery showed (or the typed host:port). */
      label: string;
      /** The sender's pairing key, presented on connect. */
      key: string;
    };

export type SourceKindName = SourceSettings["kind"];

/** Whether a source kind produces audio (and so carries `AudioSettings`). */
export function kindHasAudio(kind: SourceKindName): boolean {
  return (
    kind === "audioInput" ||
    kind === "audioOutput" ||
    kind === "appAudio" ||
    kind === "media" ||
    kind === "remoteGuest" ||
    kind === "lanIngest" ||
    kind === "freallyLink" ||
    kind === "testTone" ||
    kind === "testFlashBeep"
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

/** CAP-M20: live probe status while the sync workbench measures. */
export type CalibrationStatus = {
  videoSamples: number;
  audioSamples: number;
  flashSeen: boolean;
  beepHeard: boolean;
};

/** CAP-M20: a completed measurement. Positive = video later than audio. */
export type CalibrationMeasurement = {
  offsetMs: number;
  cycles: number;
  jitterMs: number;
};

/** CAP-M20: why a measurement failed (serde tag = `kind`). */
export type CalibrationFailure =
  | { kind: "noFlash" }
  | { kind: "noBeep" }
  | { kind: "tooFewCycles"; paired: number }
  | { kind: "notThePattern" }
  | { kind: "unstable"; jitterMs: number };

/** CAP-M20: the finish verdict — exactly one side is present. */
export type CalibrationResult = {
  measurement?: CalibrationMeasurement;
  error?: CalibrationFailure;
};

/** CAP-N35 parametric-EQ band shape. */
export type EqBandType = "bell" | "lowShelf" | "highShelf" | "notch" | "highPass" | "lowPass";

/** One parametric-EQ band (CAP-N35). `gainDb` applies to bell + shelf only. */
export type EqBand = {
  type: EqBandType;
  /** Center / corner frequency, 20..=20000 Hz. */
  freqHz: number;
  /** Gain for bell + shelf bands, −30..=30 dB. */
  gainDb: number;
  /** Resonance / width, 0.1..=18. */
  q: number;
};

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
    }
  | { type: "parametricEq"; bands: EqBand[] }
  | { type: "deEsser"; freqHz: number; thresholdDb: number; amountDb: number }
  | { type: "rumbleGuard"; freqHz: number };

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
  /** Stereo balance (CAP-M19), −1 (left) ..= 1 (right); 0 = untouched. */
  pan: number;
  /** PFL solo (CAP-M19): monitor hears only soloed strips; program unchanged. */
  solo: boolean;
  /** Mono downmix before the balance (CAP-M19). */
  mono: boolean;
  /** Join the gain-sharing auto-mixer (CAP-N32). Off by default. */
  automix?: boolean;
  /** Produce an N−1 mix-minus return for this source (CAP-N39). Off by default. */
  mixMinus?: boolean;
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

/** Optional-integration status: NDI (detected runtime) + VST (scoped). */
export type IntegrationsStatus = {
  ndiAvailable: boolean;
  ndiVersion?: string | null;
  ndiGuidance: string;
  vstAvailable: boolean;
  vstStatus: string;
};

/** First-run EULA gate payload. */
export type EulaStatus = {
  version: string;
  text: string;
  accepted: boolean;
};

/** Game-capture status: the honest anti-cheat/AV risk + the working fallback. */
export type GameCaptureStatus = {
  support: "hookPlanned" | "portalOnly" | "windowCaptureOnly";
  hookPossible: boolean;
  risk: string;
  fallback: "windowCapture" | "portal";
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
  /** CAP-N30 program-bus routes that could not open their device. */
  outputErrors?: { bus: string; message: string }[];
  /** CAP-N35 live spectrum of the armed source (a parametric-EQ editor open). */
  spectrum?: { source: string; magnitudes: number[] };
  /** Per-filter live meters (linear in/out peaks) for the strip whose filter
   *  editor is open — the plugin meters. */
  filterMeters?: { source: string; meters: { id: string; inPeak: number; outPeak: number }[] };
  /** Capture samples dropped across sources (ring overflows). */
  dropped: number;
  /** CAP-N47: the LTC reader's live decode (`HH:MM:SS:FF`), when locked. */
  ltc?: string | null;
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
  | { type: "crop"; left: number; top: number; right: number; bottom: number }
  | { type: "flip"; horizontal: boolean; vertical: boolean }
  | { type: "directionalBlur"; radius: number; angle: number }
  | { type: "radialBlur"; amount: number; centerX: number; centerY: number }
  | { type: "zoomBlur"; amount: number; centerX: number; centerY: number }
  | { type: "pixelate"; size: number }
  | { type: "freeze" }
  | { type: "userShader"; source: string; params: number[] }
  | { type: "bezierMask"; points: [number, number][]; feather: number; invert: boolean };

export type FilterTypeName = FilterKind["type"];

/** One filter instance in an item's chain. */
export type Filter = { id: FilterId; enabled: boolean } & FilterKind;

/** Keying-workbench render mode (CAP-M26): raw source, keyed, alpha matte, or a
 * before/after split. Mirrors `WorkbenchMode` in studio.rs. */
export type WorkbenchMode = "source" | "keyed" | "matte" | "split";

/** One connected display for the projector "open on…" picker (CAP-M07).
 * Mirrors `DisplayInfo` in projector.rs. */
export type DisplayInfo = {
  index: number;
  name: string;
  width: number;
  height: number;
  x: number;
  y: number;
  primary: boolean;
};

/** Where a scene's backdrop wallpaper sits: the whole canvas (cover-fit) or
 * one half (fit-contained there, the capture free to take the other half).
 * Mirrors `BackdropSplit` in Rust. */
export type BackdropSplit = "full" | "left" | "right" | "top" | "bottom";

/** Pixel-perfect scaling (CAP-N70). Mirrors `ScaleMode` in Rust: `auto` is
 * the ordinary smooth bilinear; `integer` also snaps the drawn scale to
 * whole multiples. */
export type ScaleMode = "auto" | "nearest" | "integer" | "sharpBilinear";

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
  /** Pixel-perfect scaling (CAP-N70); absent = "auto" (smooth bilinear). */
  scaling?: ScaleMode;
  /** Present on the scene's backdrop wallpaper (pinned bottom layer): which
   * canvas region it fills. The compositor owns its placement; the item's
   * transform is only clamped zoom/pan within that region, and the canvas
   * skips it for clicks and drags. */
  backdrop?: BackdropSplit | null;
  /** Show/hide fade-in duration in ms (CAP-N21); 0/absent = appear instantly. */
  revealMs?: number;
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
  /** Custom alignment guides the user dragged out (CAP-M04 follow-on). */
  guides?: GuideLine[];
};

/** One custom alignment guide line in canvas px (CAP-M04 follow-on). Mirrors
 * `GuideLine` in Rust: `"v"` is a vertical line at a constant x, `"h"` a
 * horizontal line at a constant y. */
export type GuideLine = { orientation: "v" | "h"; position: number };

/** The second output canvas (Phase 6): its own size + the scene it shows. */
export type VerticalCanvas = {
  width: number;
  height: number;
  /** The scene this canvas composes (independent of the program scene). */
  scene: SceneId;
};

export type DskId = string;

/** One downstream-keyer layer (CAP-N24): a source composited over the program,
 *  above every scene, surviving scene cuts. Mirrors `DownstreamKeyer` in Rust. */
export type DownstreamKeyer = {
  id: DskId;
  source: SourceId;
  enabled: boolean;
  opacity: number;
  transform: Transform;
};

/** One per-scene-pair transition rule (CAP-N21): `from`→`to` uses `kind` for
 * `durationMs` instead of the default (the stinger/luma file stays global). */
export type TransitionOverride = {
  from: SceneId;
  to: SceneId;
  kind: TransitionKind;
  durationMs: number;
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
  /** Downstream keyer layers (CAP-N24), bottom-to-top; omitted when empty. */
  downstream?: DownstreamKeyer[];
  /** Per-scene-pair transition rules (CAP-N21); omitted when empty. */
  transitionOverrides?: TransitionOverride[];
};

/** The `studio` event / `studio_get` payload. */
export type StudioDto = {
  revision: number;
  collection: Collection;
  /** Studio Mode (Phase 5): present while enabled. */
  studioMode?: StudioModeDto;
  /** Undo/redo availability + the viewable history list (CAP-M01). */
  history: HistoryState;
  /** Panic slate engaged (CAP-M22) — absent when false. */
  panic?: boolean;
};

/**
 * Undo/redo state (CAP-M01). Labels are stable keys the UI localizes
 * (`history.<label>`), not user-facing strings.
 */
export type HistoryState = {
  canUndo: boolean;
  canRedo: boolean;
  /** Label the next undo would reverse (`null` when nothing to undo). */
  undoLabel: string | null;
  /** Label the next redo would replay. */
  redoLabel: string | null;
  /** Every undoable edit, oldest → newest (the newest is the next undo). */
  undo: string[];
  /** Every undone edit still redoable; the last is the next redo. */
  redo: string[];
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
  /** An HDR display feeds this source (CAP-N74) — offer the tone-map. */
  hdr?: boolean;
  errorCode?: SourceRuntimeErrorCode;
  errorMessage?: string;
  /** Ms since the last delivered frame (capture sources only) (CAP-M13). */
  lastFrameMs?: number;
  /** Capture frames overwritten before the compositor took them (CAP-M13). */
  dropped?: number;
  /** Pipeline restarts (manual retry + auto-recover) (CAP-M13). */
  retries?: number;
};

/** The `alarm` push event (CAP-M10): a broadcast-safety watchdog raised or
 * cleared. Non-modal — a dismissible banner + the a11y announcer. */
export type AlarmKind = "silentAudio" | "clipping" | "black" | "frozen" | "lowDisk";
export type Alarm = {
  kind: AlarmKind;
  active: boolean;
  /** lowDisk only: the forecast in whole minutes. */
  minutesLeft?: number;
};

/** The `encoder-fallback` push event (CAP-M12): a mid-session encoder swap
 * kept the stream/recording alive — surfaced as a toast + stats note. */
export type EncoderFallback = {
  scope: "stream" | "recording";
  /** Human labels (catalog-resolved), not raw encoder ids. */
  from: string;
  to: string;
};

/** The `quit-guard` push event (CAP-M23): what quitting now interrupts. */
export type QuitConsequences = {
  streaming: boolean;
  recording: boolean;
  replay: boolean;
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

/** The CEF (browser-source runtime) component status (mirrors `CefStatusDto`). */
export type CefStatus =
  | { state: "missing"; supported: boolean }
  | { state: "resolving" }
  | {
      state: "downloading";
      receivedBytes: number;
      totalBytes: number | null;
      bytesPerSec: number;
    }
  | { state: "verifying" }
  | { state: "extracting" }
  | { state: "ready"; version: string; path: string }
  | { state: "error"; message: string; supported: boolean };

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
