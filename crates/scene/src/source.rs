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
    },
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

impl Rgba {
    fn default_color() -> Self {
        // The Havoc accent blue — a friendly non-black default block.
        Rgba::new(0x4a, 0x9e, 0xff, 0xff)
    }

    fn default_text() -> Self {
        Rgba::WHITE
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
                | SourceSettings::RemoteGuest { .. }
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
