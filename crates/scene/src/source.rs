//! Sources — the shared inputs scene items point at.
//!
//! A [`Source`] lives in the collection's shared pool and is referenced by
//! [`crate::SceneItem`]s across any number of scenes: renaming a source or
//! changing its settings updates every scene that shows it. The variants here
//! are exactly the source kinds the engine can run today — new kinds are added
//! alongside their runtime, never ahead of it.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::audio::AudioSettings;

/// Stable identity of a shared [`Source`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SourceId(pub Uuid);

impl SourceId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Parse a canonical UUID string into a source id (e.g. a CAP-N37 soundboard
    /// pad id, whose ring key doubles as its transient engine source id).
    pub fn parse(text: &str) -> Option<Self> {
        Uuid::parse_str(text).ok().map(Self)
    }
}

impl Default for SourceId {
    fn default() -> Self {
        Self::new()
    }
}

/// An RGBA color, 8 bits per channel (straight, not premultiplied).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    pub const WHITE: Rgba = Rgba {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };

    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

/// A requested webcam capture format (mirrors the device's advertised modes).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoDeviceFormat {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub fourcc: String,
}

/// Horizontal alignment of a text source's lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
}

fn default_text_size() -> f32 {
    72.0
}

fn default_true() -> bool {
    true
}

fn default_line_spacing() -> f32 {
    1.0
}

fn default_color_size() -> u32 {
    1920
}

fn default_slide_ms() -> u32 {
    5_000
}

fn default_slide_fade_ms() -> u32 {
    300
}

fn default_chat_width() -> u32 {
    480
}

fn default_chat_lines() -> u32 {
    12
}

fn default_chat_font() -> f32 {
    22.0
}

fn default_color_height() -> u32 {
    1080
}

fn default_browser_width() -> u32 {
    1280
}

fn default_browser_height() -> u32 {
    720
}

fn default_browser_fps() -> u32 {
    30
}

/// What a source *is*, plus its per-kind settings.
///
/// Serialized with `kind` as the tag — the on-disk scene-collection format.
/// Every settings field defaults so files from older builds keep loading.
/// `rename_all` covers the variant tags only; `rename_all_fields` makes the
/// variant *fields* camelCase too — the UI sends `captureId`/`deviceId`/…,
/// and without it serde silently drops them to their defaults.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum SourceSettings {
    /// A whole display, captured via the Phase 1 capture pipeline.
    Display {
        /// The OS capture id (from `fcap_capture::list_sources`).
        #[serde(default)]
        capture_id: String,
        /// The label the picker showed — re-resolution key if ids shift.
        #[serde(default)]
        label: String,
    },
    /// One window, captured via the Phase 1 capture pipeline.
    Window {
        #[serde(default)]
        capture_id: String,
        #[serde(default)]
        label: String,
    },
    /// The Wayland ScreenCast portal — the *system dialog* picks the actual
    /// screen/window on every (re)start; that honesty is by design.
    Portal {},
    /// A webcam / capture card via `fcap-sources`.
    VideoDevice {
        #[serde(default)]
        device_id: String,
        /// `None` = auto (highest resolution).
        #[serde(default)]
        format: Option<VideoDeviceFormat>,
        /// Deinterlacing (CAP-M17) for interlaced feeds; off by default.
        /// Changing it restarts the device (like a format change).
        #[serde(default)]
        deinterlace: DeinterlaceMode,
        /// Which field is dominant when deinterlacing.
        #[serde(default)]
        field_order: FieldOrder,
    },
    /// A still image file (PNG/JPEG/BMP/GIF-first-frame/WebP…).
    Image {
        #[serde(default)]
        path: String,
    },
    /// A media file (video or image) composed onto the canvas with its
    /// audio in the mixer. `.frec` plays through the owned codec with
    /// nothing fetched; the wire formats (mp4/mkv/webm/…) decode through
    /// the clearly-labeled on-demand ffmpeg component.
    Media {
        #[serde(default)]
        path: String,
        /// Restart from the top at the end.
        #[serde(default, rename = "loop")]
        looping: bool,
        /// Try hardware decode first (falls back to software on its own).
        #[serde(default = "default_true")]
        hw_decode: bool,
        /// Hold on the first frame until recording starts, then play from
        /// the top (the backdrop's "start playback with recording" option).
        #[serde(default)]
        start_with_recording: bool,
        /// True reverse playback: GIFs reverse through the owned decoder;
        /// `.frec`/wire files render a reversed copy once (cached, via the
        /// labeled ffmpeg component) and play that.
        #[serde(default)]
        reverse: bool,
    },
    /// A remote guest's live feed (Remote Guests, P2P/WebRTC) — video frames
    /// *and* mic audio are pushed from the webview's WebRTC session over IPC;
    /// there is no OS device behind this kind. Video composites onto the
    /// canvas; audio joins the mixer as its own strip (like Media).
    RemoteGuest {
        /// The guest's display label (from the session).
        #[serde(default)]
        label: String,
    },
    /// A solid color block.
    Color {
        #[serde(default = "Rgba::default_color")]
        color: Rgba,
        #[serde(default = "default_color_size")]
        width: u32,
        #[serde(default = "default_color_height")]
        height: u32,
    },
    /// A microphone / line-in (audio only — renders nothing on the canvas).
    AudioInput {
        /// The audio device name; `""` = the OS default input.
        #[serde(default)]
        device_id: String,
    },
    /// Desktop / system audio ("what you hear"). Windows captures any output
    /// device via WASAPI loopback; Linux uses a PipeWire/Pulse **monitor**
    /// device; macOS needs a virtual loopback device (e.g. BlackHole) until
    /// ScreenCaptureKit audio lands — the pickers say so honestly.
    AudioOutput {
        /// Windows: an *output* device name (loopback); elsewhere the
        /// monitor / virtual capture device name. `""` = the default.
        #[serde(default)]
        device_id: String,
    },
    /// One application's audio, captured as its own mixer source (Phase 8,
    /// TASK-805). Windows-first via WASAPI process loopback; other OSes surface
    /// the honest per-OS guidance instead of a fake toggle. Audio-only.
    AppAudio {
        /// The target process id (the capture key within a session).
        #[serde(default)]
        pid: u32,
        /// The executable file name (e.g. `chrome.exe`) — a durable, human
        /// label; the pid alone is not stable across relaunches.
        #[serde(default)]
        exe: String,
        /// CAP-N73: the Window-capture source this audio is linked to.
        /// Hiding that window mutes this strip, removing it removes this
        /// too, and the engine re-resolves `pid` from the window's live
        /// process across app restarts.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        linked_window: Option<SourceId>,
    },
    /// An ordered set of images cycling on a timer (Phase 6): per-slide
    /// duration, an optional crossfade (equal-size slides only — different
    /// sizes hard-cut, honestly), loop or hold-last, optional shuffle.
    Slideshow {
        #[serde(default)]
        paths: Vec<String>,
        /// How long each slide holds, ms.
        #[serde(default = "default_slide_ms")]
        slide_ms: u32,
        /// Crossfade length between slides, ms (0 = hard cut).
        #[serde(default = "default_slide_fade_ms")]
        transition_ms: u32,
        /// Restart from the top at the end (else hold the last slide).
        #[serde(default = "default_true", rename = "loop")]
        looping: bool,
        /// Re-shuffle the order each cycle.
        #[serde(default)]
        shuffle: bool,
    },
    /// The live chat overlay (Phase 6, TASK-613): a positionable,
    /// transparent-background record of the incoming livestream chat —
    /// username + message + a per-message 12-hour timestamp. **No API key,
    /// no developer account, no sign-in, ever** (the hard rule): YouTube
    /// reads through the owned InnerTube client exactly like the web
    /// player, Twitch reads anonymous IRC, Kick polls its public endpoint.
    /// Facebook would need the user's own token — opt-in and not
    /// implemented yet; it never gates the others.
    ChatOverlay {
        /// A YouTube channel / watch / live_chat URL (empty = off).
        #[serde(default)]
        youtube: String,
        /// A Twitch channel name (empty = off).
        #[serde(default)]
        twitch: String,
        /// A Kick channel slug (empty = off).
        #[serde(default)]
        kick: String,
        /// Overlay width in canvas pixels.
        #[serde(default = "default_chat_width")]
        width: u32,
        /// How many newest lines stay on screen.
        #[serde(default = "default_chat_lines")]
        max_lines: u32,
        #[serde(default = "default_chat_font")]
        font_size: f32,
    },
    /// Another scene composed as a source — nested scenes (Phase 6). The
    /// referenced scene renders at program-canvas size and follows its own
    /// edits live. Cycle-safe: the model rejects references that would make
    /// a scene contain itself, directly or through other scenes.
    NestedScene {
        #[serde(default)]
        scene: crate::scene::SceneId,
    },
    /// Shaped, rasterized text (rustybuzz shaping, RTL-aware).
    Text {
        #[serde(default)]
        text: String,
        /// System font family; `None` = the platform default face.
        #[serde(default)]
        font_family: Option<String>,
        /// Explicit font file — overrides `font_family` when set.
        #[serde(default)]
        font_file: Option<String>,
        #[serde(default = "default_text_size")]
        size_px: f32,
        #[serde(default = "Rgba::default_text")]
        color: Rgba,
        #[serde(default)]
        align: TextAlign,
        /// Line height multiplier (1.0 = the font's natural spacing).
        #[serde(default = "default_line_spacing")]
        line_spacing: f32,
        /// Render right-to-left paragraphs (auto-detected; this forces it).
        #[serde(default)]
        force_rtl: bool,
        /// Word-wrap width in px; `None`/0 = never wrap.
        #[serde(default)]
        wrap_width: Option<u32>,
        /// Bind the content to a watched local file (CAP-M16); `""` = the
        /// `text` field is used. The render loop polls it and re-renders on
        /// change, tolerating atomic-write gaps (temp+rename) by holding the
        /// last good content.
        #[serde(default)]
        source_file: String,
        /// How the bound file parses into the shown text.
        #[serde(default)]
        binding: FileBinding,
        /// CSV: the 1-based data row ([`FileBinding::CsvCell`]).
        #[serde(default = "default_csv_row")]
        csv_row: u32,
        /// CSV: the column, by header name or 1-based index.
        #[serde(default)]
        csv_column: String,
        /// JSON: an RFC 6901 pointer, e.g. `/teams/0/score`.
        #[serde(default)]
        json_pointer: String,
    },
    /// SMPTE-style color bars (CAP-M21) — verify scenes, encoders,
    /// projectors, and stream targets with no camera plugged in.
    TestBars {
        #[serde(default = "default_color_size")]
        width: u32,
        #[serde(default = "default_color_height")]
        height: u32,
    },
    /// Calibration grid / crosshatch (CAP-M21) — projector alignment,
    /// overscan and geometry checks.
    TestGrid {
        #[serde(default = "default_color_size")]
        width: u32,
        #[serde(default = "default_color_height")]
        height: u32,
    },
    /// Motion sweep (CAP-M21) — a bar crossing at constant speed to expose
    /// judder, tearing, and encoder motion handling.
    TestSweep {
        #[serde(default = "default_color_size")]
        width: u32,
        #[serde(default = "default_color_height")]
        height: u32,
    },
    /// The 1 kHz lineup tone at −20 dBFS (CAP-M21). Audio-only.
    TestTone {},
    /// The combined A/V sync flash+beep pattern (CAP-M21): a white flash and
    /// a 1 kHz beep from one clock — CAP-M20's workbench measures against it.
    TestFlashBeep {
        #[serde(default = "default_color_size")]
        width: u32,
        #[serde(default = "default_color_height")]
        height: u32,
    },
    /// The timer & clock text family (CAP-M15): wall clock / countdown /
    /// stopwatch / time since live / time since recording. The render loop
    /// repaints the face only when its text changes (~1 Hz); run state
    /// (start/pause/reset) is runtime-only and never persisted.
    Timer {
        #[serde(default)]
        mode: TimerMode,
        /// Wall clock: a strftime pattern; `""` = `%H:%M:%S`.
        #[serde(default)]
        format: String,
        /// Wall clock: fixed UTC offset in minutes; `None` = local time.
        #[serde(default)]
        utc_offset_min: Option<i32>,
        /// Countdown: the duration, ms (used while `target` is empty).
        #[serde(default = "default_countdown_ms")]
        countdown_ms: u64,
        /// Countdown: a `"HH:MM"` local wall target; `""` = use the duration.
        #[serde(default)]
        target: String,
        #[serde(default)]
        end_action: CountdownEnd,
        /// The scene [`CountdownEnd::SwitchScene`] jumps to.
        #[serde(default)]
        end_scene: Option<crate::scene::SceneId>,
        /// System font family; `None` = the platform default face.
        #[serde(default)]
        font_family: Option<String>,
        /// Explicit font file — overrides `font_family` when set.
        #[serde(default)]
        font_file: Option<String>,
        #[serde(default = "default_timer_size")]
        size_px: f32,
        #[serde(default = "Rgba::default_text")]
        color: Rgba,
        /// V1-C: an optional line shown above the number in slate mode (e.g.
        /// "Starting Soon"). Empty = no message. Ignored unless `slate` is set.
        #[serde(default)]
        message: String,
        /// V1-C: when set, the timer renders as a full-canvas countdown slate
        /// with this background instead of the inline text face. Boxed so the
        /// (image-path-bearing) slate does not fatten every `SourceSettings`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        slate: Option<Box<CountdownSlate>>,
    },
    /// The viewer-facing performance HUD (CAP-N14): the stats dock's real
    /// numbers — render fps, this process's CPU% and memory, GPU compose
    /// time, dropped frames, and the live publish bitrate — composited for
    /// the audience. The render loop repaints the face only when its text
    /// changes (~2 Hz). GPU utilization is deliberately absent: it is not
    /// measured anywhere, and a viewer HUD must not guess.
    SystemStats {
        #[serde(default = "default_true")]
        show_fps: bool,
        #[serde(default = "default_true")]
        show_cpu: bool,
        #[serde(default = "default_true")]
        show_memory: bool,
        #[serde(default = "default_true")]
        show_render_ms: bool,
        #[serde(default = "default_true")]
        show_dropped: bool,
        #[serde(default = "default_true")]
        show_bitrate: bool,
        /// CAP-N47: the burn-in timecode line (the LTC reader's decode).
        /// Defaults OFF — existing overlays keep their exact face.
        #[serde(default)]
        show_timecode: bool,
        /// System font family; `None` = the platform default face.
        #[serde(default)]
        font_family: Option<String>,
        /// Explicit font file — overrides `font_family` when set.
        #[serde(default)]
        font_file: Option<String>,
        #[serde(default = "default_stats_size")]
        size_px: f32,
        #[serde(default = "Rgba::default_text")]
        color: Rgba,
    },
    /// CAP-N15: classic audio visualization (FFT spectrum bars, an
    /// oscilloscope, or stereo VU meters) of a mixer signal, rendered as a
    /// video source. Listens **post-fader** — the signal that actually
    /// mixes, so a muted strip visualizes flat — to one strip, one track
    /// bus, or the master mix. Classic DSP; no ML.
    AudioVisualizer {
        #[serde(default)]
        style: VisStyle,
        #[serde(default)]
        target: VisTargetKind,
        /// 1-based track bus ([`VisTargetKind::Track`]).
        #[serde(default = "default_vis_track")]
        track: u32,
        /// The bound strip ([`VisTargetKind::Source`]); unset renders an
        /// honest error instead of guessing a signal.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        source: Option<SourceId>,
        #[serde(default = "default_vis_width")]
        width: u32,
        #[serde(default = "default_vis_height")]
        height: u32,
        /// Spectrum bar count (bars style; the renderer clamps to 8–128).
        #[serde(default = "default_vis_bands")]
        bands: u32,
        #[serde(default = "Rgba::default_color")]
        color: Rgba,
        #[serde(default = "default_true")]
        peak_hold: bool,
        /// Bar fall rate, dB/s (the renderer clamps to 6–120).
        #[serde(default = "default_vis_decay")]
        decay: f32,
        /// V1-C: classic level colours — green→yellow→red for VU/bars, a
        /// phosphor-green scope — instead of the flat `color`.
        #[serde(default)]
        classic: bool,
    },
    /// CAP-N18: a LiveSplit-style speedrun split timer. Imports a `.lss`
    /// split file (read-only — nothing is written back), compares the live
    /// run against PB / best segments / average, highlights golds, and
    /// splits from the global hotkeys. **Process-memory auto-splitters are
    /// deliberately excluded** (anti-cheat adjacency) — file + hotkey
    /// splitting only.
    SplitTimer {
        /// The `.lss` file (local only — network paths are refused).
        #[serde(default)]
        path: String,
        #[serde(default)]
        comparison: SplitComparison,
        #[serde(default = "default_split_width")]
        width: u32,
        #[serde(default = "default_split_height")]
        height: u32,
        #[serde(default = "default_split_size")]
        size_px: f32,
        #[serde(default = "Rgba::default_text")]
        color: Rgba,
        /// Ahead-of-comparison delta color.
        #[serde(default = "Rgba::default_ahead")]
        ahead: Rgba,
        /// Behind-comparison delta color.
        #[serde(default = "Rgba::default_behind")]
        behind: Rgba,
        /// Gold-segment highlight color.
        #[serde(default = "Rgba::default_gold")]
        gold: Rgba,
    },
    /// CAP-N17: an ordered, trimmed, **gapless** media playlist — the whole
    /// list plays through one labeled-ffmpeg concat decode, so item
    /// boundaries are frame-exact. Per-item in/out trims + cue points,
    /// loop / shuffle / hold-last, next/previous from the global hotkeys,
    /// and a "now playing" studio variable (CAP-N02) Text sources can
    /// interpolate. Wire formats only (`.frec`/stills play through
    /// Media/Slideshow); items are all-video or all-audio, never mixed.
    Playlist {
        #[serde(default)]
        items: Vec<PlaylistEntry>,
        /// Restart from the top at the end.
        #[serde(default = "default_true", rename = "loop")]
        looping: bool,
        /// One shuffle draw per start (a looping shuffle repeats its order).
        #[serde(default)]
        shuffle: bool,
        /// Hold the last frame at the end (else clear to transparent).
        #[serde(default = "default_true")]
        hold_last: bool,
        #[serde(default = "default_true")]
        hw_decode: bool,
        /// The studio variable fed the playing item's name ("" = off).
        #[serde(default)]
        now_playing_variable: String,
        /// Audio-only lane: hide the on-canvas track-list face (pure sound —
        /// used by background music so it adds no visible card). Video
        /// playlists ignore this.
        #[serde(default)]
        hidden_face: bool,
    },
    /// CAP-N10: plays the replay buffer INTO the program. A roll snapshots
    /// the armed buffer's last `seconds` into a temporary clip (stream
    /// copy — fast) and plays it at `speed` — retimed, never interpolated —
    /// then clears back to transparency ("auto-return to live"). Slow
    /// motion is silent by design; full speed plays the clip's audio
    /// through the source's mixer strip.
    ReplayPlayback {
        /// How much history a roll grabs, seconds (clamped to the buffer).
        #[serde(default = "default_replay_roll_secs")]
        seconds: u32,
        #[serde(default)]
        speed: ReplaySpeed,
        #[serde(default = "default_true")]
        hw_decode: bool,
    },
    /// CAP-N11: a LAN ingest LISTENER — a phone or second PC on the same
    /// network feeds this source over SRT or RTMP (any free mobile
    /// SRT/RTMP camera app, another encoder). Nothing listens until the
    /// source is added; the listener binds this machine and **never dials
    /// out** — LAN only, never the internet. Decoding rides the labeled
    /// on-demand ffmpeg component. SRT can encrypt with a passphrase;
    /// RTMP is unauthenticated by protocol (the pickers say so).
    LanIngest {
        #[serde(default)]
        protocol: IngestProtocol,
        /// The listen port (1024–65535; SRT defaults 9710, RTMP 1935).
        #[serde(default = "default_ingest_port")]
        port: u16,
        /// SRT only. Empty = an open, unencrypted listener — the UI warns.
        #[serde(default)]
        passphrase: String,
    },
    /// CAP-N13: a live input overlay — the pressed state of a FIXED layout
    /// of keys / mouse buttons / gamepad controls, drawn for the viewers.
    /// Privacy by construction (stated in-product too): input is polled
    /// only while a session is live, only for the layout's fixed keys —
    /// no hook, no buffer, nothing logged or stored, no free-text capture.
    /// Keyboard/mouse state is Windows-only today (the picker says so);
    /// gamepads read through the cross-platform `gilrs` library.
    InputOverlay {
        #[serde(default)]
        layout: InputLayout,
        /// The idle key-cap / outline color.
        #[serde(default = "Rgba::default_text")]
        color: Rgba,
        /// The pressed-state fill.
        #[serde(default = "Rgba::default_color")]
        accent: Rgba,
    },
    /// CAP-N16: the title & scoreboard designer — a fixed-size canvas of
    /// text / image / solid-box layers with an animate-in/out pass, CAP-M16
    /// file bindings and CAP-N02 `{{variable}}` interpolation per text
    /// cell, and live fire/edit control from the properties dialog. Fully
    /// local, CPU-composed; deliberately not a browser source.
    Title {
        #[serde(default = "default_color_size")]
        width: u32,
        #[serde(default = "default_color_height")]
        height: u32,
        /// Drawn in list order — later layers on top.
        #[serde(default)]
        layers: Vec<TitleLayer>,
        #[serde(default)]
        animation: TitleAnimation,
        /// The in/out animation length, ms.
        #[serde(default = "default_title_duration_ms")]
        duration_ms: u32,
    },
    /// V1-D: a generated "social & channels" bar — a tidy vertical panel that
    /// lists a creator's social handles, one row each: a brand-coloured badge +
    /// the platform name + the handle. Fully local and CPU-composed: no logos
    /// are fetched or embedded (the coloured badge *is* the design), nothing is
    /// read off disk, and nothing dials the network. A purely static face —
    /// it repaints only when its settings change. Blank-handle rows are skipped.
    SocialBar {
        /// An optional title line above the rows (`""` = no header).
        #[serde(default)]
        header: String,
        /// One account per row, drawn top-to-bottom.
        #[serde(default)]
        rows: Vec<SocialRow>,
        /// System font family; `None` = the bundled default face. This is a
        /// family *name*, never a file path — the source reads nothing off disk.
        #[serde(default)]
        font_family: Option<String>,
        #[serde(default = "default_social_size")]
        size_px: f32,
        /// The header + handle text colour.
        #[serde(default = "Rgba::default_text")]
        color: Rgba,
        /// The panel's semi-transparent rounded background.
        #[serde(default = "Rgba::default_social_bg")]
        background: Rgba,
    },
    /// CAP-N12: another Freally Capture instance's program feed, received
    /// over the owned Freally Link protocol on the operator's own network.
    /// Video composites onto the canvas; the sender's master audio joins
    /// the mixer as this source's strip. The session reconnects with
    /// backoff on its own — an unplugged sender shows a "connecting" face,
    /// never a frozen frame. LAN-intent by design: nothing here dials the
    /// internet by itself, and the sender side is off by default.
    FreallyLink {
        /// The sender's address — an IPv4/hostname, no scheme.
        #[serde(default)]
        host: String,
        #[serde(default = "default_link_port")]
        port: u16,
        /// The label discovery showed (or the typed host:port).
        #[serde(default)]
        label: String,
        /// The sender's pairing key, presented on connect — the sender
        /// serves nothing until it checks out (CAP-N12's gate).
        #[serde(default)]
        key: String,
    },
    /// CAP-N77: an http(s) page rendered offscreen by the browser-host helper
    /// (CEF as an on-demand component — never bundled, never in-process) and
    /// composed with transparency intact. Local files play through Media/Image.
    Browser {
        #[serde(default)]
        url: String,
        #[serde(default = "default_browser_width")]
        width: u32,
        #[serde(default = "default_browser_height")]
        height: u32,
        #[serde(default = "default_browser_fps")]
        fps: u32,
        #[serde(default = "default_true")]
        transparent: bool,
    },
}

/// The Freally Link stream's default TCP port (CAP-N12).
fn default_link_port() -> u16 {
    9720
}

/// One [`SourceSettings::SocialBar`] account row (V1-D).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialRow {
    #[serde(default)]
    pub platform: SocialPlatform,
    /// The handle shown, e.g. `@mychannel`. Blank = this row is skipped.
    #[serde(default)]
    pub handle: String,
    /// The badge label for [`SocialPlatform::Custom`] (ignored otherwise —
    /// bundled platforms show their own name).
    #[serde(default)]
    pub label: String,
    /// The badge colour for [`SocialPlatform::Custom`] (ignored otherwise —
    /// bundled platforms use their brand colour).
    #[serde(default = "Rgba::default_color")]
    pub color: Rgba,
}

/// A bundled social platform (its brand colour + display name baked in) or a
/// user-defined [`SocialPlatform::Custom`] row (V1-D).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SocialPlatform {
    #[default]
    Youtube,
    Twitch,
    Kick,
    Twitter,
    Instagram,
    Tiktok,
    Facebook,
    Discord,
    Custom,
}

impl SocialPlatform {
    /// The platform's brand badge colour. [`SocialPlatform::Custom`] returns
    /// the neutral accent — its real colour rides the row's own `color` field.
    pub fn brand_color(self) -> Rgba {
        match self {
            SocialPlatform::Youtube => Rgba::new(0xFF, 0x00, 0x00, 0xFF),
            SocialPlatform::Twitch => Rgba::new(0x91, 0x46, 0xFF, 0xFF),
            SocialPlatform::Kick => Rgba::new(0x53, 0xFC, 0x18, 0xFF),
            SocialPlatform::Twitter => Rgba::new(0x1D, 0xA1, 0xF2, 0xFF),
            SocialPlatform::Instagram => Rgba::new(0xE1, 0x30, 0x6C, 0xFF),
            SocialPlatform::Tiktok => Rgba::new(0x00, 0xF2, 0xEA, 0xFF),
            SocialPlatform::Facebook => Rgba::new(0x18, 0x77, 0xF2, 0xFF),
            SocialPlatform::Discord => Rgba::new(0x58, 0x65, 0xF2, 0xFF),
            SocialPlatform::Custom => Rgba::default_color(),
        }
    }

    /// The platform's on-badge display name. These are proper nouns — the same
    /// in every language, so they are NOT localized. [`SocialPlatform::Custom`]
    /// returns `""`: the row's own `label` is shown instead.
    pub fn display_name(self) -> &'static str {
        match self {
            SocialPlatform::Youtube => "YouTube",
            SocialPlatform::Twitch => "Twitch",
            SocialPlatform::Kick => "Kick",
            SocialPlatform::Twitter => "X",
            SocialPlatform::Instagram => "Instagram",
            SocialPlatform::Tiktok => "TikTok",
            SocialPlatform::Facebook => "Facebook",
            SocialPlatform::Discord => "Discord",
            SocialPlatform::Custom => "",
        }
    }
}

/// Which protocol a [`SourceSettings::LanIngest`] listener speaks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum IngestProtocol {
    /// SRT in listener mode — supports passphrase encryption (preferred).
    #[default]
    Srt,
    /// An RTMP server for one publisher — no authentication in the protocol.
    Rtmp,
}

fn default_ingest_port() -> u16 {
    9710
}

/// Which fixed layout a [`SourceSettings::InputOverlay`] draws.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum InputLayout {
    /// The WASD cluster + Shift/Space + a mouse.
    #[default]
    Wasd,
    /// A compact full keyboard + a mouse.
    Keyboard,
    /// A dual-stick gamepad (sticks follow the axes, triggers fill).
    Gamepad,
    /// An arcade fight stick: 8-way gated ball top + eight buttons.
    Fightstick,
}

/// A [`SourceSettings::ReplayPlayback`] roll's speed — retimed frames, no
/// interpolation, no ML.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReplaySpeed {
    Full,
    #[default]
    Half,
    Quarter,
}

fn default_replay_roll_secs() -> u32 {
    15
}

/// One CAP-N17 playlist entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistEntry {
    #[serde(default)]
    pub path: String,
    /// In-trim, seconds (0 = the top).
    #[serde(default, rename = "in")]
    pub in_s: f32,
    /// Out-trim, seconds (0 = the end).
    #[serde(default, rename = "out")]
    pub out_s: f32,
    /// Cue points, seconds into the FILE (independent of the trims).
    #[serde(default)]
    pub cues: Vec<f32>,
}

/// One CAP-N16 title layer, drawn in list order (later layers on top).
/// Text layers carry the full text surface plus outline/shadow and the
/// CAP-M16 binding fields; image layers load once at session start; rects
/// are the bars and plates behind the text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum TitleLayer {
    Text {
        #[serde(default)]
        x: i32,
        #[serde(default)]
        y: i32,
        #[serde(default)]
        text: String,
        /// System font family; `None` = the bundled default face.
        #[serde(default)]
        font_family: Option<String>,
        /// Explicit font file — overrides `font_family` when set.
        #[serde(default)]
        font_file: Option<String>,
        #[serde(default = "default_title_text_size")]
        size_px: f32,
        #[serde(default = "Rgba::default_text")]
        color: Rgba,
        #[serde(default)]
        align: TextAlign,
        /// Outline stroke width visible OUTSIDE the fill, px (0 = none).
        #[serde(default)]
        outline_px: f32,
        #[serde(default = "Rgba::default_outline")]
        outline_color: Rgba,
        /// A soft drop shadow, scaled with the type size.
        #[serde(default)]
        shadow: bool,
        /// CAP-M16: a watched local file this cell binds to (`""` = `text`).
        #[serde(default)]
        source_file: String,
        #[serde(default)]
        binding: FileBinding,
        /// CSV: the 1-based data row ([`FileBinding::CsvCell`]).
        #[serde(default = "default_csv_row")]
        csv_row: u32,
        /// CSV: the column, by header name or 1-based index.
        #[serde(default)]
        csv_column: String,
        /// JSON: an RFC 6901 pointer, e.g. `/teams/0/score`.
        #[serde(default)]
        json_pointer: String,
    },
    Image {
        #[serde(default)]
        x: i32,
        #[serde(default)]
        y: i32,
        #[serde(default)]
        path: String,
    },
    Rect {
        #[serde(default)]
        x: i32,
        #[serde(default)]
        y: i32,
        #[serde(default = "default_title_rect_width")]
        width: u32,
        #[serde(default = "default_title_rect_height")]
        height: u32,
        #[serde(default = "Rgba::default_color")]
        color: Rgba,
    },
}

/// How a [`SourceSettings::Title`] animates in and out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TitleAnimation {
    /// A hard cut in/out.
    #[default]
    None,
    Fade,
    /// Enters from the right edge moving left (exits back right).
    SlideLeft,
    /// Enters from the bottom edge moving up (exits back down).
    SlideUp,
    /// A left-to-right reveal.
    Wipe,
}

fn default_title_duration_ms() -> u32 {
    400
}

fn default_title_text_size() -> f32 {
    48.0
}

fn default_title_rect_width() -> u32 {
    400
}

fn default_title_rect_height() -> u32 {
    120
}

/// Which reference a [`SourceSettings::SplitTimer`] compares against.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SplitComparison {
    #[default]
    PersonalBest,
    BestSegments,
    Average,
}

fn default_split_width() -> u32 {
    420
}

fn default_split_height() -> u32 {
    380
}

fn default_split_size() -> f32 {
    18.0
}

/// Which face a [`SourceSettings::AudioVisualizer`] draws.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VisStyle {
    #[default]
    Bars,
    Scope,
    Vu,
}

/// What a [`SourceSettings::AudioVisualizer`] listens to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VisTargetKind {
    /// The program (master) mix.
    #[default]
    Master,
    /// One track bus (the `track` field, 1-based).
    Track,
    /// One mixer strip (the `source` field).
    Source,
}

fn default_vis_track() -> u32 {
    1
}

fn default_vis_width() -> u32 {
    800
}

fn default_vis_height() -> u32 {
    240
}

fn default_vis_bands() -> u32 {
    48
}

fn default_vis_decay() -> f32 {
    30.0
}

/// Which face a [`SourceSettings::Timer`] shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TimerMode {
    #[default]
    WallClock,
    Countdown,
    Stopwatch,
    SinceLive,
    SinceRecording,
}

/// What a countdown does when it reaches zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CountdownEnd {
    #[default]
    None,
    /// The face flashes for a few seconds.
    Flash,
    /// The program switches to `end_scene`.
    SwitchScene,
}

/// The full-canvas background a countdown *slate* paints behind its number
/// (V1-C, "Starting Soon"). `None` on a [`SourceSettings::Timer`] keeps the
/// classic inline text face; any value here makes the timer render as a
/// canvas-filling pre-show slate (an optional message line above a big
/// countdown). `Transparent` lays the slate out but paints no fill, so
/// whatever is composited beneath it — an image/gif/video source or a scene
/// backdrop — shows through, which is how image/video/music backgrounds come
/// for free without embedding a decoder here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CountdownSlate {
    Transparent,
    Solid {
        color: Rgba,
    },
    Gradient {
        from: Rgba,
        to: Rgba,
    },
    /// A still image (png/jpg/…) contain-fit (centred, letterboxed, never
    /// cropped) behind the countdown. Animated
    /// backgrounds (GIF/video) are the compositor's job — set a looping scene
    /// backdrop under a `Transparent` slate — because a single uploaded face
    /// texture cannot play video by itself.
    Image {
        path: String,
    },
}

fn default_countdown_ms() -> u64 {
    5 * 60 * 1_000
}

fn default_csv_row() -> u32 {
    1
}

/// A device source's deinterlace mode (CAP-M17) — the classic algorithms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DeinterlaceMode {
    #[default]
    Off,
    Discard,
    Bob,
    Linear,
    Blend,
    MotionAdaptive,
}

/// Which field an interlaced feed shot first (CAP-M17).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FieldOrder {
    #[default]
    TopFirst,
    BottomFirst,
}

/// How a Text source's bound file (CAP-M16) parses into the shown text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FileBinding {
    /// The whole file, trailing whitespace trimmed.
    #[default]
    Whole,
    /// One CSV cell (`csv_row` × `csv_column`).
    CsvCell,
    /// One JSON value at `json_pointer`.
    JsonPointer,
}

fn default_timer_size() -> f32 {
    96.0
}

fn default_stats_size() -> f32 {
    28.0
}

/// The social bar's default handle/label type size (V1-D).
fn default_social_size() -> f32 {
    32.0
}

impl Rgba {
    fn default_color() -> Self {
        // The Havoc accent blue — a friendly non-black default block.
        Rgba::new(0x4a, 0x9e, 0xff, 0xff)
    }

    fn default_text() -> Self {
        Rgba::WHITE
    }

    /// Social bar (V1-D): a dark, semi-transparent panel so the video shows
    /// through behind the handles.
    fn default_social_bg() -> Self {
        Rgba::new(0x0a, 0x0a, 0x0f, 0xb8)
    }

    /// Split timer (CAP-N18): ahead-of-comparison green.
    fn default_ahead() -> Self {
        Rgba::new(0x22, 0xc5, 0x5e, 0xff)
    }

    /// Split timer: behind-comparison red (the countdown-flash red).
    fn default_behind() -> Self {
        Rgba::new(0xef, 0x44, 0x44, 0xff)
    }

    /// Split timer: gold-segment amber.
    fn default_gold() -> Self {
        Rgba::new(0xfb, 0xbf, 0x24, 0xff)
    }

    /// Title (CAP-N16): text-outline black.
    fn default_outline() -> Self {
        Rgba::new(0, 0, 0, 0xff)
    }
}

impl SourceSettings {
    /// Machine name of the kind (stable; used for labels + telemetry-free logs).
    pub fn kind_name(&self) -> &'static str {
        match self {
            SourceSettings::Display { .. } => "display",
            SourceSettings::Window { .. } => "window",
            SourceSettings::Portal {} => "portal",
            SourceSettings::VideoDevice { .. } => "videoDevice",
            SourceSettings::Image { .. } => "image",
            SourceSettings::Media { .. } => "media",
            SourceSettings::RemoteGuest { .. } => "remoteGuest",
            SourceSettings::Color { .. } => "color",
            SourceSettings::AudioInput { .. } => "audioInput",
            SourceSettings::AudioOutput { .. } => "audioOutput",
            SourceSettings::AppAudio { .. } => "appAudio",
            SourceSettings::NestedScene { .. } => "nestedScene",
            SourceSettings::Slideshow { .. } => "slideshow",
            SourceSettings::ChatOverlay { .. } => "chatOverlay",
            SourceSettings::Text { .. } => "text",
            SourceSettings::TestBars { .. } => "testBars",
            SourceSettings::TestGrid { .. } => "testGrid",
            SourceSettings::TestSweep { .. } => "testSweep",
            SourceSettings::TestTone {} => "testTone",
            SourceSettings::TestFlashBeep { .. } => "testFlashBeep",
            SourceSettings::Timer { .. } => "timer",
            SourceSettings::SystemStats { .. } => "systemStats",
            SourceSettings::AudioVisualizer { .. } => "audioVisualizer",
            SourceSettings::SplitTimer { .. } => "splitTimer",
            SourceSettings::Playlist { .. } => "playlist",
            SourceSettings::ReplayPlayback { .. } => "replayPlayback",
            SourceSettings::LanIngest { .. } => "lanIngest",
            SourceSettings::InputOverlay { .. } => "inputOverlay",
            SourceSettings::Title { .. } => "title",
            SourceSettings::SocialBar { .. } => "socialBar",
            SourceSettings::FreallyLink { .. } => "freallyLink",
            SourceSettings::Browser { .. } => "browser",
        }
    }

    /// Human default name for a new source of this kind.
    pub fn default_name(&self) -> &'static str {
        match self {
            SourceSettings::Display { .. } => "Display Capture",
            SourceSettings::Window { .. } => "Window Capture",
            SourceSettings::Portal {} => "Screen Capture (Portal)",
            SourceSettings::VideoDevice { .. } => "Video Capture Device",
            SourceSettings::Image { .. } => "Image",
            SourceSettings::Media { .. } => "Media",
            SourceSettings::RemoteGuest { .. } => "Remote Guest",
            SourceSettings::Color { .. } => "Color",
            SourceSettings::AudioInput { .. } => "Audio Input Capture",
            SourceSettings::AudioOutput { .. } => "Audio Output Capture",
            SourceSettings::AppAudio { .. } => "Application Audio",
            SourceSettings::NestedScene { .. } => "Nested Scene",
            SourceSettings::Slideshow { .. } => "Image Slideshow",
            SourceSettings::ChatOverlay { .. } => "Live Chat",
            SourceSettings::Text { .. } => "Text",
            SourceSettings::TestBars { .. } => "SMPTE Bars",
            SourceSettings::TestGrid { .. } => "Calibration Grid",
            SourceSettings::TestSweep { .. } => "Motion Sweep",
            SourceSettings::TestTone {} => "1 kHz Tone",
            SourceSettings::TestFlashBeep { .. } => "A/V Sync Pattern",
            SourceSettings::Timer { .. } => "Timer",
            SourceSettings::SystemStats { .. } => "System Stats",
            SourceSettings::AudioVisualizer { .. } => "Audio Visualizer",
            SourceSettings::SplitTimer { .. } => "Split Timer",
            SourceSettings::Playlist { .. } => "Media Playlist",
            SourceSettings::ReplayPlayback { .. } => "Instant Replay",
            SourceSettings::LanIngest { .. } => "LAN Ingest",
            SourceSettings::InputOverlay { .. } => "Input Overlay",
            SourceSettings::Title { .. } => "Title",
            SourceSettings::SocialBar { .. } => "Social Bar",
            SourceSettings::FreallyLink { .. } => "Freally Link",
            SourceSettings::Browser { .. } => "Browser",
        }
    }

    /// Whether this kind produces audio (and so carries [`AudioSettings`]).
    /// Media has both video and audio — it mixes *and* composes; so does the
    /// flash+beep test pattern.
    pub fn has_audio(&self) -> bool {
        matches!(
            self,
            SourceSettings::AudioInput { .. }
                | SourceSettings::AudioOutput { .. }
                | SourceSettings::AppAudio { .. }
                | SourceSettings::Media { .. }
                | SourceSettings::Playlist { .. }
                | SourceSettings::ReplayPlayback { .. }
                | SourceSettings::LanIngest { .. }
                | SourceSettings::RemoteGuest { .. }
                | SourceSettings::FreallyLink { .. }
                | SourceSettings::TestTone {}
                | SourceSettings::TestFlashBeep { .. }
        )
    }

    /// Whether this kind is audio-*only* (renders nothing on the canvas —
    /// the studio's video pipeline skips it; the mixer owns it entirely).
    pub fn is_audio_only(&self) -> bool {
        matches!(
            self,
            SourceSettings::AudioInput { .. }
                | SourceSettings::AudioOutput { .. }
                | SourceSettings::AppAudio { .. }
                | SourceSettings::TestTone {}
        )
    }

    /// Whether this kind shows a shared screen (the Desktop/Window view).
    /// The center-view rules treat these specially: one screen view at a
    /// time — centering one hides the other visible ones.
    pub fn is_screen_view(&self) -> bool {
        matches!(
            self,
            SourceSettings::Display { .. }
                | SourceSettings::Window { .. }
                | SourceSettings::Portal {}
        )
    }
}

/// One shared source: identity + display name + settings (+ the mixer state
/// for audio-capable kinds).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub id: SourceId,
    pub name: String,
    /// Present exactly when [`SourceSettings::has_audio`] — enforced by
    /// [`crate::Collection::sanitize`] on load.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<AudioSettings>,
    #[serde(flatten)]
    pub settings: SourceSettings,
}

impl Source {
    /// A new source with a fresh id. Empty names fall back to the kind's
    /// default name; audio-capable kinds start with the neutral mixer strip.
    pub fn new(name: impl Into<String>, settings: SourceSettings) -> Self {
        let name = name.into();
        let name = if name.trim().is_empty() {
            settings.default_name().to_string()
        } else {
            name
        };
        Self {
            id: SourceId::new(),
            name,
            audio: settings.has_audio().then(AudioSettings::default),
            settings,
        }
    }
}

#[cfg(test)]
mod social_bar_tests {
    use super::*;

    /// The V1-D social bar is a purely visual, audio-free static face.
    #[test]
    fn social_bar_is_a_silent_static_face() {
        let settings = SourceSettings::SocialBar {
            header: "Follow me".into(),
            rows: vec![SocialRow {
                platform: SocialPlatform::Youtube,
                handle: "@mychannel".into(),
                label: String::new(),
                color: Rgba::default_color(),
            }],
            font_family: None,
            size_px: 32.0,
            color: Rgba::WHITE,
            background: Rgba::default_social_bg(),
        };
        assert_eq!(settings.kind_name(), "socialBar");
        assert_eq!(settings.default_name(), "Social Bar");
        assert!(!settings.has_audio(), "a social bar makes no sound");
        assert!(!settings.is_audio_only());
        assert!(!settings.is_screen_view());
    }

    /// The serde tag values the TS mirror pins must stay stable (camelCase),
    /// and older files with absent fields still load via the field defaults.
    #[test]
    fn social_platform_wire_values_and_defaults_round_trip() {
        // camelCase of the single-capital variants is clean and predictable.
        for (platform, wire) in [
            (SocialPlatform::Youtube, "\"youtube\""),
            (SocialPlatform::Twitter, "\"twitter\""),
            (SocialPlatform::Tiktok, "\"tiktok\""),
            (SocialPlatform::Custom, "\"custom\""),
        ] {
            assert_eq!(serde_json::to_string(&platform).unwrap(), wire);
        }
        // A minimal on-disk row (only a platform + handle) fills the rest in.
        let row: SocialRow =
            serde_json::from_str(r#"{"platform":"twitch","handle":"@me"}"#).unwrap();
        assert_eq!(row.platform, SocialPlatform::Twitch);
        assert_eq!(row.color, Rgba::default_color());
        // A bare `{"kind":"socialBar"}` loads as an empty bar (no panic).
        let bar: SourceSettings = serde_json::from_str(r#"{"kind":"socialBar"}"#).unwrap();
        assert_eq!(bar.kind_name(), "socialBar");
    }

    /// Every bundled platform has its documented brand colour; Custom defers to
    /// the row's own colour (returning the neutral accent as a placeholder).
    #[test]
    fn brand_colours_match_the_bundled_palette() {
        assert_eq!(
            SocialPlatform::Youtube.brand_color(),
            Rgba::new(0xFF, 0, 0, 0xFF)
        );
        assert_eq!(
            SocialPlatform::Kick.brand_color(),
            Rgba::new(0x53, 0xFC, 0x18, 0xFF)
        );
        assert_eq!(SocialPlatform::Custom.brand_color(), Rgba::default_color());
        assert_eq!(SocialPlatform::Tiktok.display_name(), "TikTok");
        assert_eq!(SocialPlatform::Custom.display_name(), "");
    }
}
