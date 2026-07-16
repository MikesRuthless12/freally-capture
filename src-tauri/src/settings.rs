//! The JSON settings store — `settings.json` in the OS config dir.
//!
//! User configuration lives as plain JSON in the per-user config directory
//! (via `directories`), e.g. `%APPDATA%\Freally\Freally Capture\config\` on
//! Windows, `~/Library/Application Support/` on macOS, `~/.config/` on Linux.
//! Writes are atomic (temp file + rename) so a crash never truncates the
//! file. Stream keys are NOT stored here — they arrive in Phase 5 with their
//! own locally-scoped handling.

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use fcap_encode::mux::{Container, EncPreset, RateControl, RcMode};

/// How the Audio Mixer lays out its channel strips.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MixerLayout {
    /// Strips stacked as horizontal rows (the compact default).
    #[default]
    Horizontal,
    /// OBS-style vertical strips side by side, with tall meters + faders.
    Vertical,
}

/// CAP-N34 loudness normalization: the live rider that steers the program
/// toward a target LUFS with a peak ceiling. Off by default.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct LoudnessSettings {
    pub enabled: bool,
    /// Integrated-loudness target, LUFS (e.g. −14 YouTube, −16, −23 EBU R128).
    pub target_lufs: f32,
    /// Peak ceiling, dBFS (the rider's output limiter).
    pub ceiling_db: f32,
}

impl Default for LoudnessSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            target_lufs: -16.0,
            ceiling_db: -1.0,
        }
    }
}

impl LoudnessSettings {
    fn validate(&self) -> Result<(), String> {
        if !self.target_lufs.is_finite() || !(-30.0..=-5.0).contains(&self.target_lufs) {
            return Err("loudness target must be between -30 and -5 LUFS".to_owned());
        }
        if !self.ceiling_db.is_finite() || !(-9.0..=0.0).contains(&self.ceiling_db) {
            return Err("loudness ceiling must be between -9 and 0 dBFS".to_owned());
        }
        Ok(())
    }
}

/// One CAP-N37 soundboard pad: a local clip with playback + mix options.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct SoundboardPad {
    /// Stable UUID (the media-hub ring key + the engine source id).
    pub id: String,
    pub name: String,
    pub path: String,
    /// Local hotkey accelerator that fires the pad from the dock.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hotkey: Option<String>,
    pub gain_db: f32,
    /// Track bitmask this pad mixes into (bit 0 = track 1).
    pub tracks: u8,
    /// Choke group 1..=8 (a new pad stops the others in its group); 0 = none.
    pub choke_group: u8,
    pub looping: bool,
    /// Duck the rest of the mix while this pad plays.
    pub auto_duck: bool,
}

impl Default for SoundboardPad {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            path: String::new(),
            hotkey: None,
            gain_db: 0.0,
            tracks: 1,
            choke_group: 0,
            looping: false,
            auto_duck: false,
        }
    }
}

/// CAP-N37 soundboard: a grid of local audio-clip pads. Empty until populated.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct SoundboardSettings {
    pub pads: Vec<SoundboardPad>,
}

impl SoundboardSettings {
    fn validate(&self) -> Result<(), String> {
        if self.pads.len() > 64 {
            return Err("too many soundboard pads (64 max)".to_owned());
        }
        for pad in &self.pads {
            if pad.id.len() > 64 || pad.name.len() > 128 || pad.path.len() > 4_096 {
                return Err("invalid soundboard pad".to_owned());
            }
            if pad.name.chars().any(char::is_control) || pad.path.chars().any(char::is_control) {
                return Err("soundboard pad name/path has control characters".to_owned());
            }
            // The pad id is the media-hub ring key AND the engine source id; a
            // non-UUID would decode into a ring nothing drains (a burnt decoder
            // and no sound). The UI mints these with `crypto.randomUUID()`.
            if fcap_scene::SourceId::parse(&pad.id).is_none() {
                return Err("soundboard pad id must be a valid source id".to_owned());
            }
            if pad.choke_group > 8 {
                return Err("soundboard choke group must be 0–8".to_owned());
            }
            if !pad.gain_db.is_finite() || !(-60.0..=12.0).contains(&pad.gain_db) {
                return Err("soundboard pad gain out of range".to_owned());
            }
            if pad.hotkey.as_ref().is_some_and(|hk| hk.len() > 64) {
                return Err("soundboard hotkey is too long".to_owned());
            }
        }
        Ok(())
    }
}

/// Which palette the UI paints with (TASK-906). Applied live through the
/// frontend's CSS-variable theme provider — never a rebuild, never a reload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ThemeMode {
    /// Havoc dark — what every build before 0.96.0 shipped, and still the default.
    #[default]
    Dark,
    Light,
    /// Dark, but with [`ThemeSettings::accent`] replacing the Havoc blue.
    Custom,
}

/// Appearance (TASK-906).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ThemeSettings {
    pub mode: ThemeMode,
    /// `#rrggbb`. Only read when `mode` is [`ThemeMode::Custom`], but persisted
    /// always, so switching to Custom and back does not lose the colour.
    pub accent: String,
}

impl Default for ThemeSettings {
    fn default() -> Self {
        Self {
            mode: ThemeMode::default(),
            // The Havoc accent, so `Custom` starts where `Dark` left off.
            accent: "#4a9eff".to_owned(),
        }
    }
}

impl ThemeSettings {
    pub fn validate(&self) -> Result<(), String> {
        // The accent lands in a CSS custom property. Anything but a plain hex
        // triple could close the declaration and inject a rule.
        let hex = self.accent.strip_prefix('#').unwrap_or("");
        if hex.len() != 6 || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err("the accent colour must be #rrggbb".to_owned());
        }
        Ok(())
    }
}

/// Preview alignment aids (CAP-M04): smart snapping guides, safe-area
/// overlays, and rulers. All are preview-only chrome — none touch the model or
/// the composed output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AlignmentSettings {
    /// Snap a dragged item to canvas + other-item edges/centers (on by default,
    /// like a design tool). Holding a modifier during the drag bypasses it.
    pub smart_guides: bool,
    /// Draw action-safe + title-safe rectangles over the preview.
    pub safe_areas: bool,
    /// Draw px rulers in the gutter around the preview.
    pub rulers: bool,
}

impl Default for AlignmentSettings {
    fn default() -> Self {
        Self {
            smart_guides: true,
            safe_areas: false,
            rulers: false,
        }
    }
}

/// Which palette the Audio Mixer's level meters use (Settings → Accessibility).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MeterPreset {
    /// The green→yellow→red sweep every build so far has drawn.
    #[default]
    Default,
    /// An Okabe–Ito blue→orange→vermillion ramp — distinguishable under the
    /// common red-green color-vision deficiencies, where green vs. red is not.
    Colorblind,
    /// The three zone colours below, user-picked.
    Custom,
}

/// Accessibility (Settings → Accessibility): the mixer VU meter palette.
/// The meter is one low→mid→high sweep; these recolor its three zones.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AccessibilitySettings {
    pub meter_preset: MeterPreset,
    /// `#rrggbb` ×3. Only read when `meter_preset` is [`MeterPreset::Custom`],
    /// but persisted always, so switching presets never loses the picks.
    pub meter_low: String,
    pub meter_mid: String,
    pub meter_high: String,
}

impl Default for AccessibilitySettings {
    fn default() -> Self {
        Self {
            meter_preset: MeterPreset::default(),
            // The default sweep, so Custom starts where Default left off.
            meter_low: "#22c55e".to_owned(),
            meter_mid: "#eab308".to_owned(),
            meter_high: "#ef4444".to_owned(),
        }
    }
}

impl AccessibilitySettings {
    pub fn validate(&self) -> Result<(), String> {
        // Same gate as `ThemeSettings`: these land in a CSS gradient in the
        // webview, and anything but a plain hex triple could close the
        // declaration and inject a rule.
        for color in [&self.meter_low, &self.meter_mid, &self.meter_high] {
            let hex = color.strip_prefix('#').unwrap_or("");
            if hex.len() != 6 || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
                return Err("meter colours must be #rrggbb".to_owned());
            }
        }
        Ok(())
    }
}

/// `Settings::language` sentinel: follow the operating system's preferred
/// languages instead of a fixed choice. Fresh installs start here, so a Japanese
/// user does not have to find a picker to stop reading English. Kept in step
/// with `AUTO_LOCALE` in `ui/src/i18n/locales.ts`.
pub const AUTO_LANGUAGE: &str = "auto";

/// User-facing settings. Every field defaults (`serde(default)`) so missing
/// keys never brick the app, and unknown keys from newer builds are ignored.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Settings {
    /// UI language: a BCP-47 tag the user chose, or [`AUTO_LANGUAGE`] to follow
    /// the operating system. The frontend resolves it against the OS preference
    /// list — see `ui/src/i18n/locales.ts`. `validate` rejects an empty tag, so
    /// "follow the system" is a word rather than `""`.
    pub language: String,
    /// Whether the stats dock is shown.
    pub show_stats_dock: bool,
    /// The audio monitor output device name (`None`/empty = the OS default).
    pub monitor_device: Option<String>,
    /// CAP-N30 program-bus output routes: master / track buses → physical
    /// output devices, each with a trim. Empty by default — only the monitor
    /// bus reaches a device until the operator adds a route.
    #[serde(default)]
    pub audio_outputs: Vec<fcap_scene::AudioOutputRoute>,
    /// CAP-N34 loudness normalization (the live rider). Off by default.
    #[serde(default)]
    pub loudness: LoudnessSettings,
    /// CAP-N47 SMPTE LTC timecode (generator + reader). Off by default.
    #[serde(default)]
    pub ltc: LtcSettings,
    /// CAP-N37 soundboard pads. Empty until populated.
    #[serde(default)]
    pub soundboard: SoundboardSettings,
    /// Audio Mixer strip orientation.
    pub mixer_layout: MixerLayout,
    /// Appearance: palette + custom accent (Phase 9, TASK-906).
    pub theme: ThemeSettings,
    /// Preview alignment aids: smart guides, safe areas, rulers (CAP-M04).
    pub alignment: AlignmentSettings,
    /// Accessibility: the mixer VU meter palette (Settings → Accessibility).
    pub accessibility: AccessibilitySettings,
    /// Recording output configuration (Phase 4).
    pub recording: RecordingSettings,
    /// Remote Guests networking (Phase R).
    pub remote: RemoteSettings,
    /// Live-stream configuration (Phase 5).
    pub stream: StreamSettings,
    /// The rolling replay buffer (Phase 6).
    pub replay: ReplaySettings,
    /// Studio Mode's commit transition (Phase 5).
    pub transition: TransitionSettings,
    /// Global action hotkeys (Phase 5).
    pub hotkeys: HotkeySettings,
    /// The panic button's privacy slate (CAP-M22).
    pub panic_slate: PanicSlateSettings,
    /// The WebSocket remote-control API (Phase 7).
    pub remote_control: RemoteControlSettings,
    /// Browser docks — named URLs opened as dock windows (Phase 7).
    pub browser_docks: Vec<BrowserDockSettings>,
    /// Sandboxed Lua scripts (Phase 7).
    pub scripts: Vec<ScriptSettings>,
    /// The EULA version the user accepted, if any (Phase 8). `None` (fresh
    /// install) or a stale version → the app shows the acceptance gate before
    /// it can be used.
    pub accepted_eula_version: Option<String>,
    /// Whether the first-run wizard has been seen (Phase 9, TASK-903/905).
    /// `false` on a fresh install; set once the user finishes *or skips* it, so
    /// a skipped wizard never comes back uninvited.
    pub completed_onboarding: bool,
    /// Camera control profiles (CAP-M18): device id → control tag → value,
    /// reapplied whenever that device (re)opens. Written server-side by
    /// `set_camera_control` and PRESERVED across `set()` (the counter/EULA
    /// pattern) so an open settings dialog can't clobber a live tweak.
    pub camera_profiles: std::collections::HashMap<String, std::collections::HashMap<String, i64>>,
    /// HDR→SDR tone-map per display capture (CAP-N74): capture id →
    /// operator + paper-white. Written server-side by `set_hdr_tone_map`
    /// and PRESERVED across `set()` like the camera profiles.
    pub hdr_tone_map: std::collections::HashMap<String, HdrToneMapSetting>,
    /// Cursor effects per display/window capture (CAP-N19): capture id →
    /// halo/ripples/keystroke config. Written server-side by `set_cursor_fx`
    /// and PRESERVED across `set()` like the tone-maps.
    #[serde(default)]
    pub cursor_fx: std::collections::HashMap<String, CursorFxSetting>,
    /// Automation: rules + macros (CAP-N01/N02). Every rule ships disabled;
    /// actions are limited to the remote-API allowlist by validation.
    #[serde(default)]
    pub automation: crate::automation::AutomationSettings,
    /// The show rundown (CAP-N09): a timed scene playlist. Auto-advance is
    /// off by default — a show never runs away from its operator unasked.
    #[serde(default)]
    pub rundown: crate::rundown::RundownSettings,
    /// The LAN touch panel + tally service (CAP-N06/N07). Off by default;
    /// loopback unless LAN is turned on; a password is required.
    #[serde(default)]
    pub web_panel: crate::webpanel::WebPanelSettings,
    /// OSC control surface (CAP-N04). Off by default, loopback unless LAN.
    #[serde(default)]
    pub osc: crate::osc::OscSettings,
    /// PTZ cameras (CAP-N08). Empty by default — a camera exists only
    /// because the operator typed its address; nothing is ever discovered.
    #[serde(default)]
    pub ptz: crate::ptz::PtzSettings,
    /// MIDI control surfaces (CAP-N03). No port opens until one is picked.
    #[serde(default)]
    pub midi: crate::midi::MidiSettings,
    /// The Freally Link output (CAP-N12): share the program with one other
    /// Freally instance on the LAN. Off by default; no port opens until
    /// enabled; nothing announces until enabled.
    #[serde(default)]
    pub link: crate::link::LinkSettings,
}

/// One display's HDR→SDR mapping (CAP-N74). `operator` is a wire name the
/// capture layer parses ("clip" | "maxRgb" | "reinhard" | "bt2408").
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HdrToneMapSetting {
    pub operator: String,
    pub paper_white_nits: u32,
}

/// One capture's cursor effects (CAP-N19): halo, click ripples, keystroke
/// ghosting — drawn into the frames on the owned (Windows) cursor path.
/// Colors are `#rrggbb`; the capture layer parses them.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CursorFxSetting {
    pub halo: bool,
    pub halo_color: String,
    /// Halo radius in frame pixels (validated to 8–128).
    pub halo_radius: u32,
    pub ripples: bool,
    pub left_color: String,
    pub right_color: String,
    pub keystrokes: bool,
}

impl Default for CursorFxSetting {
    fn default() -> Self {
        // Everything OFF; the colors are just sane starting points for the
        // pickers (amber halo, blue left / red right — the screencast idiom).
        Self {
            halo: false,
            halo_color: "#ffd54a".to_owned(),
            halo_radius: 24,
            ripples: false,
            left_color: "#4ac1ff".to_owned(),
            right_color: "#ff5a5a".to_owned(),
            keystrokes: false,
        }
    }
}

impl CursorFxSetting {
    pub fn validate(&self) -> Result<(), String> {
        for color in [&self.halo_color, &self.left_color, &self.right_color] {
            if fcap_capture::cursorfx::parse_color(color).is_none() {
                return Err(format!("invalid cursor-effect color: {color}"));
            }
        }
        if !(8..=128).contains(&self.halo_radius) {
            return Err("cursor halo radius must be 8–128 px".to_owned());
        }
        Ok(())
    }

    /// The live capture-registry config — `None` when every effect is off,
    /// so the capture thread samples no input at all.
    pub fn to_config(&self) -> Option<fcap_capture::cursorfx::CursorFxConfig> {
        if !(self.halo || self.ripples || self.keystrokes) {
            return None;
        }
        let parse = fcap_capture::cursorfx::parse_color;
        Some(fcap_capture::cursorfx::CursorFxConfig {
            halo: self.halo,
            halo_color: parse(&self.halo_color)?,
            halo_radius: self.halo_radius,
            ripples: self.ripples,
            left_color: parse(&self.left_color)?,
            right_color: parse(&self.right_color)?,
            keystrokes: self.keystrokes,
        })
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            language: AUTO_LANGUAGE.to_owned(),
            show_stats_dock: true,
            monitor_device: None,
            audio_outputs: Vec::new(),
            loudness: LoudnessSettings::default(),
            ltc: LtcSettings::default(),
            soundboard: SoundboardSettings::default(),
            mixer_layout: MixerLayout::default(),
            theme: ThemeSettings::default(),
            alignment: AlignmentSettings::default(),
            accessibility: AccessibilitySettings::default(),
            recording: RecordingSettings::default(),
            remote: RemoteSettings::default(),
            stream: StreamSettings::default(),
            replay: ReplaySettings::default(),
            transition: TransitionSettings::default(),
            hotkeys: HotkeySettings::default(),
            panic_slate: PanicSlateSettings::default(),
            remote_control: RemoteControlSettings::default(),
            browser_docks: Vec::new(),
            scripts: Vec::new(),
            accepted_eula_version: None,
            completed_onboarding: false,
            camera_profiles: std::collections::HashMap::new(),
            hdr_tone_map: std::collections::HashMap::new(),
            cursor_fx: std::collections::HashMap::new(),
            automation: crate::automation::AutomationSettings::default(),
            rundown: crate::rundown::RundownSettings::default(),
            web_panel: crate::webpanel::WebPanelSettings::default(),
            osc: crate::osc::OscSettings::default(),
            ptz: crate::ptz::PtzSettings::default(),
            midi: crate::midi::MidiSettings::default(),
            link: crate::link::LinkSettings::default(),
        }
    }
}

/// One sandboxed Lua script (TASK-703): a user-chosen `.lua` file, loaded
/// while enabled. The sandbox has no io/os/require — a script can only call
/// the same command surface the remote API exposes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct ScriptSettings {
    pub path: String,
    pub enabled: bool,
}

impl ScriptSettings {
    pub fn validate(&self) -> Result<(), String> {
        if self.path.is_empty() || self.path.len() > 1024 || self.path.chars().any(char::is_control)
        {
            return Err("invalid script path".to_owned());
        }
        Ok(())
    }
}

/// One browser dock: a named URL opened as its own dock window (TASK-702).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct BrowserDockSettings {
    pub name: String,
    pub url: String,
}

impl BrowserDockSettings {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.len() > 64 || self.name.chars().any(char::is_control) {
            return Err("invalid dock name".to_owned());
        }
        // Delegate to the one open-time validator (Url::parse + scheme +
        // bounds) so a dock that saves is a dock that opens — no weaker
        // save-time prefix check that persists a URL Open then rejects.
        crate::docks::validate_dock_url(&self.url).map(|_| ())
    }
}

/// The rolling replay buffer (Phase 6, TASK-603): while armed, a background
/// encode keeps the last N seconds as small encoded segments (bounded disk,
/// tiny memory); Save stitches them into a playable file without touching
/// the stream or the recording. The UI's length/quality presets write these
/// fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ReplaySettings {
    /// How much history Save keeps, in seconds.
    pub seconds: u32,
    /// CBR video bitrate of the buffer's own encode.
    pub bitrate_kbps: u32,
    pub audio_bitrate_kbps: u32,
    pub fps: u32,
    /// The mixer track the buffer records (1-based, like the UI dots).
    pub track: u8,
}

impl Default for ReplaySettings {
    fn default() -> Self {
        Self {
            seconds: 30,
            bitrate_kbps: 6_000,
            audio_bitrate_kbps: 160,
            fps: 60,
            track: 1,
        }
    }
}

impl ReplaySettings {
    pub fn validate(&self) -> Result<(), String> {
        if !(5..=300).contains(&self.seconds) {
            return Err("replay length out of range (5–300 s)".to_owned());
        }
        if !(500..=60_000).contains(&self.bitrate_kbps) {
            return Err("replay bitrate out of range (500–60000 kbps)".to_owned());
        }
        if !(32..=512).contains(&self.audio_bitrate_kbps) {
            return Err("replay audio bitrate out of range (32–512 kbps)".to_owned());
        }
        if !(1..=240).contains(&self.fps) {
            return Err("replay fps out of range (1–240)".to_owned());
        }
        if !(1..=6).contains(&self.track) {
            return Err("the replay track must be 1–6".to_owned());
        }
        Ok(())
    }
}

/// Global action hotkeys (Phase 5). Each is an accelerator string
/// (`"Ctrl+Shift+R"`, `"F13"`) or `None`. Per-source PTT/PTM live in the
/// mixer's AudioSettings, not here.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct HotkeySettings {
    /// Toggle recording start/stop.
    pub record: Option<String>,
    /// Toggle Go Live / End Stream.
    pub go_live: Option<String>,
    /// Commit the Studio-Mode Preview → Program transition.
    pub transition: Option<String>,
    /// Save the replay buffer's last N seconds (Phase 6).
    pub save_replay: Option<String>,
    /// Drop a chapter marker into the active recording (Phase 6).
    pub add_marker: Option<String>,
    /// Grab a still frame of the program (CAP-M08).
    pub still: Option<String>,
    /// Cut program to the privacy slate + hard-mute everything (CAP-M22).
    /// Engage only — restoring is the deliberate two-step in the UI.
    pub panic: Option<String>,
    /// Start/pause every timer source (CAP-M15).
    pub timer_toggle: Option<String>,
    /// Reset every timer source (CAP-M15).
    pub timer_reset: Option<String>,
    /// Punch-in zoom presets (CAP-N71): reset the selected capture's lens.
    pub zoom_100: Option<String>,
    /// Punch-in zoom to 150%.
    pub zoom_150: Option<String>,
    /// Punch-in zoom to 200%.
    pub zoom_200: Option<String>,
    /// Start / split every split-timer source (CAP-N18).
    pub split_timer_split: Option<String>,
    /// Undo the last split (CAP-N18).
    pub split_timer_undo: Option<String>,
    /// Skip the current segment (CAP-N18).
    pub split_timer_skip: Option<String>,
    /// Reset every split timer (CAP-N18).
    pub split_timer_reset: Option<String>,
    /// Jump every playlist to its next item (CAP-N17).
    pub playlist_next: Option<String>,
    /// Jump every playlist back (CAP-N17).
    pub playlist_previous: Option<String>,
    /// Roll every live Instant Replay source (CAP-N10).
    pub replay_roll: Option<String>,
}

impl HotkeySettings {
    pub fn validate(&self) -> Result<(), String> {
        // EVERY field must be in this array — one missing here is silently
        // unvalidated. The UI's curated combobox is NOT the only writer:
        // settings.json is hand-editable and profiles import whole structs.
        for (name, key) in [
            ("record", &self.record),
            ("goLive", &self.go_live),
            ("transition", &self.transition),
            ("saveReplay", &self.save_replay),
            ("addMarker", &self.add_marker),
            ("still", &self.still),
            ("panic", &self.panic),
            ("timerToggle", &self.timer_toggle),
            ("timerReset", &self.timer_reset),
            ("zoom100", &self.zoom_100),
            ("zoom150", &self.zoom_150),
            ("zoom200", &self.zoom_200),
            ("splitTimerSplit", &self.split_timer_split),
            ("splitTimerUndo", &self.split_timer_undo),
            ("splitTimerSkip", &self.split_timer_skip),
            ("splitTimerReset", &self.split_timer_reset),
            ("playlistNext", &self.playlist_next),
            ("playlistPrevious", &self.playlist_previous),
            ("replayRoll", &self.replay_roll),
        ] {
            let Some(key) = key else { continue };
            if key.trim().is_empty() {
                // Legacy for unbound; the UI normalizes "" to null on save.
                continue;
            }
            validate_accelerator(key).map_err(|err| format!("the {name} hotkey {err}"))?;
        }
        Ok(())
    }

    /// Clear any binding that can't be a valid accelerator — called on LOAD so
    /// a file written by an older build (whose Hotkeys field was free-text and
    /// only length-capped) can't carry an unparseable value that would fail
    /// `validate()` and block EVERY future settings save. An unparseable
    /// binding never registered anyway, so dropping it to "unbound" loses
    /// nothing and lets the user rebind from the combobox. Keep this field
    /// list in sync with `validate()`'s (the same all-fields trap).
    pub fn sanitize(&mut self) {
        for field in [
            &mut self.record,
            &mut self.go_live,
            &mut self.transition,
            &mut self.save_replay,
            &mut self.add_marker,
            &mut self.still,
            &mut self.panic,
            &mut self.timer_toggle,
            &mut self.timer_reset,
            &mut self.zoom_100,
            &mut self.zoom_150,
            &mut self.zoom_200,
            &mut self.split_timer_split,
            &mut self.split_timer_undo,
            &mut self.split_timer_skip,
            &mut self.split_timer_reset,
            &mut self.playlist_next,
            &mut self.playlist_previous,
            &mut self.replay_roll,
        ] {
            if let Some(key) = field {
                if !key.trim().is_empty() && validate_accelerator(key).is_err() {
                    *field = None;
                }
            }
        }
    }
}

/// Structural sanity for one accelerator: it must parse with the OS global-
/// shortcut parser — the *exact* gate that decides whether the binding can be
/// registered — or be a two-stroke chord (CAP-N05) of two such strokes. This
/// accepts every real key the old free-text field allowed (Space, Enter,
/// Delete, navigation and punctuation keys, …) and rejects only what can never
/// register, including the garbage-string class ("Ctrl+asekfj…"). Delegating
/// to the real parser means the shape gate can never drift from what actually
/// works — a hand-maintained key allowlist rejected legitimate bindings.
fn validate_accelerator(text: &str) -> Result<(), &'static str> {
    // A generous ceiling before parsing — no legitimate accelerator (or chord
    // of two) approaches this; it just bounds absurd input cheaply.
    if text.len() > 128 {
        return Err("is too long (128 characters max)");
    }
    if let Some(chord) = crate::chords::parse_chord(text) {
        validate_stroke(&chord.leader)?;
        validate_stroke(&chord.follower)?;
        return Ok(());
    }
    if crate::chords::is_chord(text) {
        return Err("is a malformed chord (expected \"Leader, Key\")");
    }
    validate_stroke(text)
}

/// One stroke must parse as an OS accelerator — the authoritative check
/// (the same `parse::<Shortcut>()` the registrar uses in `hotkeys.rs`).
fn validate_stroke(stroke: &str) -> Result<(), &'static str> {
    stroke
        .parse::<tauri_plugin_global_shortcut::Shortcut>()
        .map(|_| ())
        .map_err(|_| "is not a valid shortcut (modifiers + one key, e.g. Ctrl+Shift+R)")
}

/// The panic button's privacy slate (CAP-M22): a solid colour, optionally
/// with an image drawn at its native size, centered (make it canvas-sized
/// for an exact fill). What program shows while panicked.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct PanicSlateSettings {
    /// `#rrggbb`.
    pub color: String,
    /// Optional image path ("" = colour only).
    pub image: String,
}

impl Default for PanicSlateSettings {
    fn default() -> Self {
        Self {
            // A neutral dark — deliberately NOT pure black, so the slate is
            // distinguishable from a dead feed at a glance.
            color: "#10141a".to_owned(),
            image: String::new(),
        }
    }
}

impl PanicSlateSettings {
    pub fn validate(&self) -> Result<(), String> {
        let hex = self.color.strip_prefix('#').unwrap_or("");
        if hex.len() != 6 || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err("the panic slate colour must be #rrggbb".to_owned());
        }
        if self.image.len() > 1024 || self.image.chars().any(char::is_control) {
            return Err("invalid panic slate image path".to_owned());
        }
        Ok(())
    }
}

/// How Studio Mode commits Preview → Program. The Phase 6 pack adds the
/// custom luma-wipe image and the stinger video (with its cut point).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct TransitionSettings {
    pub kind: fcap_scene::TransitionKind,
    pub duration_ms: u32,
    /// The grayscale wipe image for [`fcap_scene::TransitionKind::LumaImage`].
    pub luma_image: String,
    /// The video file for [`fcap_scene::TransitionKind::Stinger`].
    pub stinger_path: String,
    /// When the scene swap lands under the stinger, ms into the transition.
    pub stinger_cut_ms: u32,
    /// How a track-matte stinger packs its transparency (CAP-N29). `None` (the
    /// default) uses the file as-is (opaque / straight alpha).
    pub stinger_matte: fcap_scene::StingerMatte,
    /// Optional program-audio duck while a stinger plays (CAP-N29), in dB of
    /// attenuation, driven by the stinger clip's own audio envelope. `0.0`
    /// (the default) is off — the program mix is untouched.
    pub stinger_duck_db: f32,
}

impl Default for TransitionSettings {
    fn default() -> Self {
        Self {
            kind: fcap_scene::TransitionKind::Fade,
            duration_ms: 300,
            luma_image: String::new(),
            stinger_path: String::new(),
            stinger_cut_ms: 500,
            stinger_matte: fcap_scene::StingerMatte::None,
            stinger_duck_db: 0.0,
        }
    }
}

impl TransitionSettings {
    pub fn validate(&self) -> Result<(), String> {
        if !(50..=5_000).contains(&self.duration_ms) {
            return Err("transition duration out of range (50–5000 ms)".to_owned());
        }
        for path in [&self.luma_image, &self.stinger_path] {
            if path.len() > 512 || path.chars().any(char::is_control) {
                return Err("invalid transition file path".to_owned());
            }
        }
        if self.stinger_cut_ms > 5_000 {
            return Err("stinger cut point out of range (0–5000 ms)".to_owned());
        }
        if !(0.0..=60.0).contains(&self.stinger_duck_db) || !self.stinger_duck_db.is_finite() {
            return Err("stinger audio duck out of range (0–60 dB)".to_owned());
        }
        Ok(())
    }
}

/// Which canvas a stream target publishes (Phase 6 multi-canvas).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StreamCanvas {
    /// The program canvas.
    #[default]
    Main,
    /// The second (vertical) canvas — needs one configured in the studio.
    Vertical,
}

/// One stream target (Settings → Stream). The **stream key is a secret**:
/// redacted from `Debug`, masked in the UI, never logged.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct StreamTargetSettings {
    /// Go Live publishes to every enabled target at once.
    pub enabled: bool,
    pub service: fcap_stream::StreamService,
    /// Which canvas this target publishes.
    pub canvas: StreamCanvas,
    /// Overrides the service's preset ingest when non-empty (regional or
    /// custom `rtmp://`/`rtmps://`).
    pub ingest_url: String,
    /// SECRET.
    pub stream_key: String,
    /// ffmpeg encoder id, or "auto" = best detected H.264 encoder.
    pub encoder_id: String,
    /// CBR video bitrate — the streaming-side rate control is always CBR.
    pub bitrate_kbps: u32,
    pub audio_bitrate_kbps: u32,
    pub keyframe_sec: f32,
    pub fps: u32,
    /// The mixer track that goes to this target (1-based, like the UI dots).
    pub track: u8,
    /// Publish at this size instead of the canvas size (0 = canvas).
    pub output_width: u32,
    pub output_height: u32,
}

impl Default for StreamTargetSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            service: fcap_stream::StreamService::Twitch,
            canvas: StreamCanvas::Main,
            ingest_url: String::new(),
            stream_key: String::new(),
            encoder_id: "auto".to_owned(),
            bitrate_kbps: 6_000,
            audio_bitrate_kbps: 160,
            keyframe_sec: 2.0,
            fps: 60,
            track: 1,
            output_width: 0,
            output_height: 0,
        }
    }
}

// Manual Debug so a debug-printed Settings can never leak a stream key.
impl std::fmt::Debug for StreamTargetSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StreamTargetSettings")
            .field("enabled", &self.enabled)
            .field("service", &self.service)
            .field("ingest_url", &self.ingest_url)
            .field(
                "stream_key",
                &if self.stream_key.is_empty() {
                    ""
                } else {
                    "[redacted]"
                },
            )
            .field("encoder_id", &self.encoder_id)
            .field("bitrate_kbps", &self.bitrate_kbps)
            .field("fps", &self.fps)
            .field("track", &self.track)
            .finish()
    }
}

impl StreamTargetSettings {
    pub fn validate(&self) -> Result<(), String> {
        if !self.ingest_url.is_empty() {
            let scheme_ok = match self.service.protocol() {
                fcap_stream::StreamProtocol::Rtmp => {
                    self.ingest_url.starts_with("rtmp://")
                        || self.ingest_url.starts_with("rtmps://")
                }
                fcap_stream::StreamProtocol::Srt => self.ingest_url.starts_with("srt://"),
                fcap_stream::StreamProtocol::Whip => {
                    self.ingest_url.starts_with("https://")
                        || self.ingest_url.starts_with("http://")
                }
            };
            if !scheme_ok {
                return Err(
                    "the ingest URL scheme doesn't match the service (rtmp(s):// / srt:// / https:// for WHIP)"
                        .to_owned(),
                );
            }
        }
        for field in [&self.ingest_url, &self.stream_key] {
            if field.len() > 512 || field.chars().any(char::is_control) {
                return Err("invalid stream target".to_owned());
            }
        }
        if self.encoder_id.len() > 64
            || !self
                .encoder_id
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
        {
            return Err("invalid stream encoder id".to_owned());
        }
        if !(500..=60_000).contains(&self.bitrate_kbps) {
            return Err("stream bitrate out of range (500–60000 kbps)".to_owned());
        }
        if !(32..=512).contains(&self.audio_bitrate_kbps) {
            return Err("stream audio bitrate out of range (32–512 kbps)".to_owned());
        }
        if !(0.25..=10.0).contains(&self.keyframe_sec) {
            return Err("stream keyframe interval out of range (0.25–10 s)".to_owned());
        }
        if !(1..=240).contains(&self.fps) {
            return Err("stream fps out of range (1–240)".to_owned());
        }
        if !(1..=6).contains(&self.track) {
            return Err("the stream track must be 1–6".to_owned());
        }
        for size in [self.output_width, self.output_height] {
            if size != 0 && !(16..=16_384).contains(&size) {
                return Err("output size out of range (16–16384, 0 = canvas)".to_owned());
            }
        }
        Ok(())
    }
}

/// Live-stream configuration (Settings → Stream): the target list — Go Live
/// publishes to every enabled target at once, and targets with **equal
/// encode settings share one encode** (Phase 6 multistream) — plus the
/// auto-record toggle.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(from = "StreamSettingsWire", rename_all = "camelCase")]
pub struct StreamSettings {
    pub targets: Vec<StreamTargetSettings>,
    /// TASK-508: start a local recording automatically on Go Live.
    pub auto_record: bool,
    /// CAP-M09: the pre-flight checklist refuses "Go Live anyway" until
    /// every blocking item is green.
    pub preflight_hold: bool,
}

impl Default for StreamSettings {
    fn default() -> Self {
        Self {
            targets: vec![StreamTargetSettings::default()],
            auto_record: false,
            preflight_hold: false,
        }
    }
}

/// The on-disk shape, tolerant of the 0.70.0 single-target layout: when no
/// `targets` list exists yet, the old flat fields migrate into `targets[0]`.
#[derive(Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct StreamSettingsWire {
    targets: Option<Vec<StreamTargetSettings>>,
    auto_record: bool,
    preflight_hold: bool,
    // The 0.70.0 flat single-target fields.
    service: fcap_stream::StreamService,
    ingest_url: String,
    stream_key: String,
    encoder_id: String,
    bitrate_kbps: u32,
    audio_bitrate_kbps: u32,
    keyframe_sec: f32,
    fps: u32,
    track: u8,
}

impl Default for StreamSettingsWire {
    fn default() -> Self {
        let target = StreamTargetSettings::default();
        Self {
            targets: None,
            auto_record: false,
            preflight_hold: false,
            service: target.service,
            ingest_url: target.ingest_url,
            stream_key: target.stream_key,
            encoder_id: target.encoder_id,
            bitrate_kbps: target.bitrate_kbps,
            audio_bitrate_kbps: target.audio_bitrate_kbps,
            keyframe_sec: target.keyframe_sec,
            fps: target.fps,
            track: target.track,
        }
    }
}

impl From<StreamSettingsWire> for StreamSettings {
    fn from(wire: StreamSettingsWire) -> Self {
        let targets = match wire.targets {
            Some(targets) => targets,
            None => vec![StreamTargetSettings {
                enabled: true,
                service: wire.service,
                canvas: StreamCanvas::Main,
                ingest_url: wire.ingest_url,
                stream_key: wire.stream_key,
                encoder_id: wire.encoder_id,
                bitrate_kbps: wire.bitrate_kbps,
                audio_bitrate_kbps: wire.audio_bitrate_kbps,
                keyframe_sec: wire.keyframe_sec,
                fps: wire.fps,
                track: wire.track,
                output_width: 0,
                output_height: 0,
            }],
        };
        StreamSettings {
            targets,
            auto_record: wire.auto_record,
            preflight_hold: wire.preflight_hold,
        }
    }
}

impl StreamSettings {
    /// The most simultaneous targets Go Live will drive (encoder sessions
    /// and upload bandwidth are finite; this bound keeps the UI honest).
    pub const MAX_TARGETS: usize = 6;

    pub fn validate(&self) -> Result<(), String> {
        if self.targets.len() > Self::MAX_TARGETS {
            return Err(format!(
                "at most {} stream targets are supported",
                Self::MAX_TARGETS
            ));
        }
        for target in &self.targets {
            target.validate()?;
        }
        Ok(())
    }
}

/// Remote Guests (Phase R) networking. The TURN relay is strictly **opt-in**
/// and the *user's own* (e.g. Oracle Always Free coturn) — never author-run
/// infrastructure. Empty URL = direct P2P only (STUN), the free default.
#[derive(Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct RemoteSettings {
    /// `turn:`/`turns:` URL of the user's own relay; empty = direct only.
    pub turn_url: String,
    pub turn_username: String,
    /// A secret: redacted from Debug, never logged.
    pub turn_credential: String,
}

// Manual Debug so a debug-printed Settings can never leak the credential.
impl std::fmt::Debug for RemoteSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RemoteSettings")
            .field("turn_url", &self.turn_url)
            .field("turn_username", &self.turn_username)
            .field(
                "turn_credential",
                &if self.turn_credential.is_empty() {
                    ""
                } else {
                    "[redacted]"
                },
            )
            .finish()
    }
}

impl RemoteSettings {
    pub fn validate(&self) -> Result<(), String> {
        if !self.turn_url.is_empty()
            && !self.turn_url.starts_with("turn:")
            && !self.turn_url.starts_with("turns:")
        {
            return Err("TURN URL must start with turn: or turns:".to_owned());
        }
        if self.turn_url.len() > 512 || self.turn_url.chars().any(char::is_control) {
            return Err("invalid TURN URL".to_owned());
        }
        for field in [&self.turn_username, &self.turn_credential] {
            if field.len() > 256 || field.chars().any(char::is_control) {
                return Err("invalid TURN credentials".to_owned());
            }
        }
        Ok(())
    }
}

/// The WebSocket remote-control API (Phase 7, TASK-701). **Off by default**;
/// requires a password to enable; binds loopback unless `lan` is explicitly
/// set. Disabled means the port is closed.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct RemoteControlSettings {
    pub enabled: bool,
    /// TCP port (1024–65535).
    pub port: u16,
    /// Accept LAN connections (0.0.0.0) instead of loopback only.
    pub lan: bool,
    /// A secret: redacted from Debug, never logged. Auth is challenge–
    /// response — the password itself never crosses the wire.
    pub password: String,
}

impl Default for RemoteControlSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            port: fcap_stream::remote::DEFAULT_REMOTE_PORT,
            lan: false,
            password: String::new(),
        }
    }
}

// Manual Debug so a debug-printed Settings can never leak the password.
impl std::fmt::Debug for RemoteControlSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RemoteControlSettings")
            .field("enabled", &self.enabled)
            .field("port", &self.port)
            .field("lan", &self.lan)
            .field(
                "password",
                &if self.password.is_empty() {
                    ""
                } else {
                    "[redacted]"
                },
            )
            .finish()
    }
}

impl RemoteControlSettings {
    pub fn validate(&self) -> Result<(), String> {
        if self.port < 1024 {
            return Err("remote-control port must be 1024–65535".to_owned());
        }
        if self.password.len() > 256 || self.password.chars().any(char::is_control) {
            return Err("invalid remote-control password".to_owned());
        }
        if self.enabled && self.password.trim().is_empty() {
            return Err("the remote-control API requires a password".to_owned());
        }
        Ok(())
    }
}

/// CAP-N38 audio-only recording format. WAV is the owned writer; FLAC/Opus go
/// through the labeled ffmpeg component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AudioRecFormat {
    #[default]
    Wav,
    Flac,
    Opus,
}

impl AudioRecFormat {
    /// The ffmpeg codec name to transcode the owned WAV to, or `None` for WAV.
    pub fn codec(self) -> Option<&'static str> {
        match self {
            AudioRecFormat::Wav => None,
            AudioRecFormat::Flac => Some("flac"),
            AudioRecFormat::Opus => Some("opus"),
        }
    }
}

/// CAP-N47: SMPTE LTC timecode — classic audio timecode, fully offline. The
/// generator rides one track bus (assign that track in Output settings to
/// record it; feed it to an output route to jam-sync external recorders);
/// the reader taps one source's raw input and drives the stats overlay's
/// timecode line + LTC-stamped markers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct LtcSettings {
    /// Generate LTC (time-of-day, non-drop) onto `track`.
    pub enabled: bool,
    /// The track bus the generator rides (0-based, 0..=5).
    pub track: u8,
    /// LTC frame rate: 24, 25 or 30.
    pub fps: u32,
    /// The source (uuid) whose raw input the reader taps ("" = off).
    pub read_source: String,
}

impl Default for LtcSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            track: 5, // track 6 — the conventional spare
            fps: 30,
            read_source: String::new(),
        }
    }
}

impl LtcSettings {
    pub fn validate(&self) -> Result<(), String> {
        if self.track as usize >= fcap_scene::TRACK_COUNT {
            return Err("LTC track out of range (1–6)".to_owned());
        }
        if !matches!(self.fps, 24 | 25 | 30) {
            return Err("LTC frame rate must be 24, 25 or 30".to_owned());
        }
        if !self.read_source.is_empty()
            && (self.read_source.len() != 36
                || !self
                    .read_source
                    .bytes()
                    .all(|b| b.is_ascii_hexdigit() || b == b'-'))
        {
            return Err("invalid LTC reader source id".to_owned());
        }
        Ok(())
    }
}

/// CAP-N45: one step of the post-record pipeline. A **closed** action set —
/// there is deliberately no "run a command" variant and never will be; the
/// sandbox IS the design (the ShareX-style chain without the shell).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(
    tag = "action",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum PipelineStep {
    /// Integrity-check the file (CAP-N46). A FAIL stops the chain — later
    /// steps must not polish or move a broken file.
    Verify,
    /// Stream-copy remux an mkv to a playable mp4 sibling (the chain then
    /// continues on the mp4).
    Remux,
    /// Loudness-normalize to the app's target (CAP-N34; wire files only).
    Normalize,
    /// Rename with a CAP-M25 token template (same folder).
    Rename { template: String },
    /// Move to a folder (the chain continues on the moved file).
    Move { folder: String },
    /// Copy to a folder (the chain continues on the ORIGINAL).
    Copy { folder: String },
    /// Show the file in the OS file manager.
    Reveal,
    /// Emit a `recordingPipeline` event to the sandboxed Lua scripts.
    LuaEvent,
}

impl PipelineStep {
    /// Stable id for the UI/i18n ("verify", "remux", …).
    pub fn id(&self) -> &'static str {
        match self {
            PipelineStep::Verify => "verify",
            PipelineStep::Remux => "remux",
            PipelineStep::Normalize => "normalize",
            PipelineStep::Rename { .. } => "rename",
            PipelineStep::Move { .. } => "move",
            PipelineStep::Copy { .. } => "copy",
            PipelineStep::Reveal => "reveal",
            PipelineStep::LuaEvent => "luaEvent",
        }
    }
}

/// Recording configuration (Settings → Output). Independent of any future
/// stream settings by design — the local copy never rides a stream's knobs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct RecordingSettings {
    /// Output container; `frec` is the owned lossless default.
    pub container: Container,
    /// CAP-N38 audio-only recording format (WAV owned; FLAC/Opus via ffmpeg).
    #[serde(default)]
    pub audio_format: AudioRecFormat,
    /// ffmpeg encoder id, or "auto" = best detected H.264 encoder.
    pub encoder_id: String,
    pub rate_control: RateControl,
    /// The quality/speed trade, mapped onto each encoder family's knob.
    pub preset: EncPreset,
    /// Keyframe interval in seconds.
    pub keyframe_sec: f32,
    /// Recording frame rate (CFR).
    pub fps: u32,
    pub audio_bitrate_kbps: u32,
    /// Bitmask of the mixer tracks to record (bit 0 = track 1; ≥ 1 bit).
    pub tracks_mask: u8,
    /// Output folder ("" = the OS Videos folder).
    pub folder: String,
    /// Filename prefix — the `{prefix}` token's value (CAP-M25).
    pub filename_prefix: String,
    /// Token filename templates (CAP-M25) for recordings, replay saves and
    /// stills — see `filename::TOKENS`.
    pub template: String,
    pub replay_template: String,
    pub still_template: String,
    /// Per-output folders ("" = the recordings folder) (CAP-M25).
    pub replay_folder: String,
    pub still_folder: String,
    /// The persisted `{counter}` token value. Server-owned: bumped when a
    /// template uses it and preserved across `set`, so a settings dialog
    /// open during a recording can never write a stale count back.
    pub counter: u32,
    /// Split into playable segments every N minutes (0 = off).
    pub split_minutes: u32,
    /// Also record the second (vertical) canvas when one is configured —
    /// a parallel `… (vertical)` file with the same settings (Phase 6).
    pub record_vertical: bool,
    /// Encode at this size instead of the canvas size (0 = canvas). Wire
    /// containers only — the lossless .frec always records the canvas.
    pub output_width: u32,
    pub output_height: u32,
    /// CAP-N40 ISO recording: the scene-source ids (uuid strings) recorded
    /// as their own clean per-source files alongside the program (each remote
    /// guest's feed is just a source). Empty = off.
    pub iso_sources: Vec<String>,
    /// ISO lanes record the source post-filter (as processed) when true, or
    /// raw pre-filter when false.
    pub iso_post_filter: bool,
    /// The ISO lanes' own container + encoder — independent of the program
    /// recording's (rate control, preset, keyframe and audio bitrate are
    /// shared with the program settings above).
    pub iso_container: Container,
    pub iso_encoder_id: String,
    /// CAP-N42: record the program with real transparency (`.frec` only —
    /// wire codecs flatten). The recorder gets its own transparent-clear
    /// render; the preview/stream keep the normal opaque program.
    pub alpha_frec: bool,
    /// CAP-N43: event-driven split triggers for the owned `.frec` splitter
    /// (wire containers split by time only — the ffmpeg segment muxer can't
    /// cut on demand; the UI says so). Each starts a new part file exactly
    /// on the event's frame boundary, minimum part length one second.
    pub split_on_scene: bool,
    pub split_on_marker: bool,
    pub split_on_rundown: bool,
    /// CAP-N44: studio events (scene switch, replay save, stream reconnect,
    /// dropped-frame burst, alarm, rule firing) drop typed chapter markers
    /// automatically, alongside the manual marker hotkey.
    pub auto_markers: bool,
    /// CAP-N45: the post-record pipeline — after a recording finalizes, run
    /// these steps on the main file(s), in order, in the background. Steps
    /// come from the closed [`PipelineStep`] set only. Settings are
    /// per-profile by construction, so pipelines are too.
    pub pipeline_enabled: bool,
    pub pipeline: Vec<PipelineStep>,
}

impl Default for RecordingSettings {
    fn default() -> Self {
        Self {
            container: Container::Frec,
            audio_format: AudioRecFormat::default(),
            encoder_id: "auto".to_owned(),
            rate_control: RateControl {
                mode: RcMode::Cqp,
                bitrate_kbps: 8_000,
                cq: 23,
            },
            preset: EncPreset::Balanced,
            keyframe_sec: 2.0,
            fps: 60,
            audio_bitrate_kbps: 192,
            tracks_mask: 0b1,
            folder: String::new(),
            filename_prefix: "Freally Capture".to_owned(),
            template: "{prefix} {date} {time}".to_owned(),
            replay_template: "Replay {date} {time}".to_owned(),
            still_template: "Still {date} {time}".to_owned(),
            replay_folder: String::new(),
            still_folder: String::new(),
            counter: 0,
            split_minutes: 0,
            record_vertical: false,
            output_width: 0,
            output_height: 0,
            iso_sources: Vec::new(),
            iso_post_filter: true,
            iso_container: Container::Frec,
            iso_encoder_id: "auto".to_owned(),
            alpha_frec: false,
            split_on_scene: false,
            split_on_marker: false,
            split_on_rundown: false,
            auto_markers: false,
            pipeline_enabled: false,
            pipeline: Vec::new(),
        }
    }
}

impl RecordingSettings {
    pub fn validate(&self) -> Result<(), String> {
        if self.encoder_id.len() > 64
            || !self
                .encoder_id
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
        {
            return Err("invalid encoder id".to_owned());
        }
        if !(100..=300_000).contains(&self.rate_control.bitrate_kbps) {
            return Err("bitrate out of range (100–300000 kbps)".to_owned());
        }
        if self.rate_control.cq > 51 {
            return Err("CQ out of range (0–51)".to_owned());
        }
        if !(0.25..=10.0).contains(&self.keyframe_sec) {
            return Err("keyframe interval out of range (0.25–10 s)".to_owned());
        }
        if !(1..=240).contains(&self.fps) {
            return Err("recording fps out of range (1–240)".to_owned());
        }
        if !(32..=512).contains(&self.audio_bitrate_kbps) {
            return Err("audio bitrate out of range (32–512 kbps)".to_owned());
        }
        if self.tracks_mask == 0 || self.tracks_mask > 0b11_1111 {
            return Err("at least one of the 6 tracks must record".to_owned());
        }
        if self.split_minutes > 24 * 60 {
            return Err("split interval over 24 h".to_owned());
        }
        for size in [self.output_width, self.output_height] {
            if size != 0 && !(16..=16_384).contains(&size) {
                return Err("output size out of range (16–16384, 0 = canvas)".to_owned());
            }
        }
        if self.folder.len() > 1024 || self.folder.chars().any(char::is_control) {
            return Err("invalid recording folder".to_owned());
        }
        let prefix_ok = !self.filename_prefix.is_empty()
            && self.filename_prefix.len() <= 80
            && !self.filename_prefix.chars().any(|c| {
                c.is_control() || matches!(c, '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|')
            });
        if !prefix_ok {
            return Err("invalid filename prefix".to_owned());
        }
        // CAP-M25: templates must be sized, filename-safe outside their
        // tokens, and use only known tokens (catch typos at save time).
        for (template, what) in [
            (&self.template, "recording"),
            (&self.replay_template, "replay"),
            (&self.still_template, "still"),
        ] {
            if template.is_empty() || template.chars().count() > 200 {
                return Err(format!(
                    "the {what} filename template must be 1–200 characters"
                ));
            }
            if template.chars().any(|c| {
                c.is_control() || matches!(c, '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|')
            }) {
                return Err(format!(
                    "the {what} filename template has characters not allowed in filenames"
                ));
            }
            crate::filename::template_tokens_valid(template)
                .map_err(|err| format!("{what} filename template: {err}"))?;
        }
        for folder in [&self.replay_folder, &self.still_folder] {
            if folder.len() > 1024 || folder.chars().any(char::is_control) {
                return Err("invalid output folder".to_owned());
            }
        }
        // CAP-N40: ISO lanes — bounded count, well-formed ids, no duplicates,
        // and an encoder id shaped like the program one.
        if self.iso_sources.len() > 8 {
            return Err("at most 8 ISO sources can record at once".to_owned());
        }
        let mut iso_seen = std::collections::HashSet::new();
        for id in &self.iso_sources {
            let shaped = id.len() == 36 && id.bytes().all(|b| b.is_ascii_hexdigit() || b == b'-');
            if !shaped {
                return Err("invalid ISO source id".to_owned());
            }
            if !iso_seen.insert(id) {
                return Err("duplicate ISO source".to_owned());
            }
        }
        if self.iso_encoder_id.len() > 64
            || !self
                .iso_encoder_id
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
        {
            return Err("invalid ISO encoder id".to_owned());
        }
        // CAP-N45: a bounded pipeline of well-formed steps. (Folders are
        // additionally is_remote-guarded at RUN time — validation shape here,
        // the NTLM rule there.)
        if self.pipeline.len() > 10 {
            return Err("the post-record pipeline is limited to 10 steps".to_owned());
        }
        for step in &self.pipeline {
            match step {
                PipelineStep::Rename { template } => {
                    if template.is_empty() || template.chars().count() > 200 {
                        return Err("pipeline rename template must be 1–200 characters".to_owned());
                    }
                    if template.chars().any(crate::filename::is_reserved) {
                        return Err(
                            "pipeline rename template has characters not allowed in filenames"
                                .to_owned(),
                        );
                    }
                    crate::filename::template_tokens_valid(template)
                        .map_err(|err| format!("pipeline rename template: {err}"))?;
                }
                PipelineStep::Move { folder } | PipelineStep::Copy { folder }
                    if folder.trim().is_empty()
                        || folder.len() > 1024
                        || folder.chars().any(char::is_control) =>
                {
                    return Err("invalid pipeline folder".to_owned());
                }
                _ => {}
            }
        }
        Ok(())
    }
}

impl Settings {
    /// Reject values a well-behaved frontend never sends — keeps a buggy (or
    /// compromised) webview from persisting junk. BCP-47 tags are short ASCII.
    pub fn validate(&self) -> Result<(), String> {
        if self.language.is_empty()
            || self.language.len() > 35
            || !self
                .language
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-')
        {
            return Err("invalid language tag".to_owned());
        }
        if let Some(device) = &self.monitor_device {
            // Device names are what the OS reports — bound the size and shape.
            if device.len() > 256 || device.chars().any(char::is_control) {
                return Err("invalid monitor device name".to_owned());
            }
        }
        // CAP-N30 output routes: bounded (≤ one per program bus), sane device
        // names + trims, and no two routes fighting over the same bus.
        if self.audio_outputs.len() > fcap_scene::TRACK_COUNT + 1 {
            return Err("too many audio output routes".to_owned());
        }
        let mut seen_buses = std::collections::HashSet::new();
        for route in &self.audio_outputs {
            if route.device_id.len() > 256 || route.device_id.chars().any(char::is_control) {
                return Err("invalid audio output device name".to_owned());
            }
            if let fcap_scene::OutputBus::Track { index } = route.bus {
                if index as usize >= fcap_scene::TRACK_COUNT {
                    return Err("audio output route names a nonexistent track".to_owned());
                }
            }
            if !route.gain_db.is_finite()
                || route.gain_db < fcap_scene::MIN_VOLUME_DB
                || route.gain_db > fcap_scene::MAX_VOLUME_DB
            {
                return Err("audio output trim out of range".to_owned());
            }
            if !seen_buses.insert(route.bus) {
                return Err("duplicate audio output route for one bus".to_owned());
            }
        }
        self.loudness.validate()?;
        self.ltc.validate()?;
        self.soundboard.validate()?;
        self.theme.validate()?;
        self.accessibility.validate()?;
        self.recording.validate()?;
        self.remote.validate()?;
        self.stream.validate()?;
        self.replay.validate()?;
        self.transition.validate()?;
        self.hotkeys.validate()?;
        self.panic_slate.validate()?;
        self.remote_control.validate()?;
        if self.browser_docks.len() > 16 {
            return Err("too many browser docks (16 max)".to_owned());
        }
        for dock in &self.browser_docks {
            dock.validate()?;
        }
        if self.scripts.len() > 16 {
            return Err("too many scripts (16 max)".to_owned());
        }
        for script in &self.scripts {
            script.validate()?;
        }
        // Camera profiles (CAP-M18): bounded maps of short ASCII-ish keys —
        // a hand-edited file can't grow them without limit.
        if self.camera_profiles.len() > 64 {
            return Err("too many camera profiles (64 devices max)".to_owned());
        }
        // Automation (CAP-N01/N02): bounded, allowlisted actions only.
        self.automation.validate()?;
        // The rundown (CAP-N09) shares that allowlist for its step actions.
        self.rundown.validate()?;
        // The LAN panel + tally (CAP-N06/N07): off by default, password-gated.
        self.web_panel.validate()?;
        // OSC (CAP-N04): off by default, loopback-first.
        self.osc.validate()?;
        // PTZ cameras (CAP-N08): bounded, user-entered addresses only.
        self.ptz.validate()?;
        // MIDI (CAP-N03): bounded, allowlisted actions only.
        self.midi.validate()?;
        // Freally Link (CAP-N12): off by default, sane port + name.
        self.link.validate()?;
        // HDR tone-maps (CAP-N74): bounded, known operators, sane nits.
        if self.hdr_tone_map.len() > 64 {
            return Err("too many HDR tone-map entries (64 displays max)".to_owned());
        }
        for (capture, tone) in &self.hdr_tone_map {
            if capture.len() > 256 || capture.chars().any(char::is_control) {
                return Err("invalid HDR tone-map capture id".to_owned());
            }
            if fcap_capture::tonemap::ToneMapOperator::from_name(&tone.operator).is_none() {
                return Err(format!("unknown tone-map operator: {}", tone.operator));
            }
            if !(80..=1000).contains(&tone.paper_white_nits) {
                return Err("paper white must be 80–1000 nits".to_owned());
            }
        }
        // Cursor effects (CAP-N19): bounded, parseable colors, sane radius.
        if self.cursor_fx.len() > 64 {
            return Err("too many cursor-effect entries (64 captures max)".to_owned());
        }
        for (capture, fx) in &self.cursor_fx {
            if capture.is_empty() || capture.len() > 512 || capture.chars().any(char::is_control) {
                return Err("invalid cursor-effect capture id".to_owned());
            }
            fx.validate()?;
        }
        for (device, profile) in &self.camera_profiles {
            if device.len() > 256 || device.chars().any(char::is_control) {
                return Err("invalid camera profile device id".to_owned());
            }
            if profile.len() > 32 {
                return Err("too many camera controls in a profile (32 max)".to_owned());
            }
            for control in profile.keys() {
                if control.len() > 32 || !control.bytes().all(|b| b.is_ascii_alphanumeric()) {
                    return Err("invalid camera control tag".to_owned());
                }
            }
        }
        Ok(())
    }
}

/// Thread-safe handle to the settings file, managed as Tauri state.
pub struct SettingsStore {
    /// `None` only when no OS config dir could be resolved (no home
    /// directory) — the store then lives in memory for the session.
    path: Option<PathBuf>,
    current: Mutex<Settings>,
}

impl SettingsStore {
    /// Open the store in the OS config dir, materializing the file with
    /// defaults on first run. With no resolvable home directory the store
    /// degrades to in-memory defaults instead of failing startup.
    pub fn load_default() -> Self {
        match ProjectDirs::from("com", "Freally", "Freally Capture") {
            Some(dirs) => Self::load_from(dirs.config_dir().join("settings.json")),
            None => {
                eprintln!(
                    "settings: no home directory — running with in-memory defaults (nothing persists)"
                );
                Self {
                    path: None,
                    current: Mutex::new(Settings::default()),
                }
            }
        }
    }

    /// Open the store at an explicit path (missing file → defaults; corrupt
    /// file → defaults, with the corrupt content left in place until the next
    /// successful save overwrites it).
    pub fn load_from(path: PathBuf) -> Self {
        let current = read_settings(&path);
        let first_run = !path.exists();
        let store = Self {
            path: Some(path),
            current: Mutex::new(current),
        };
        if first_run {
            // First run: write the defaults so the file is discoverable.
            if let Err(err) = store.persist() {
                eprintln!("settings: could not create the settings file ({err}); running with in-memory defaults");
            }
        }
        store
    }

    /// A snapshot of the current settings.
    pub fn get(&self) -> Settings {
        self.lock().clone()
    }

    /// Just the monitor-device field — so the audio bridge's per-tick poll
    /// doesn't clone the whole [`Settings`] every 50 ms to compare one string.
    pub fn monitor_device(&self) -> Option<String> {
        self.lock().monitor_device.clone()
    }

    /// Just the CAP-N30 output routes — the audio bridge polls these each tick
    /// to reconcile the engine's physical outputs without a full clone.
    pub fn audio_outputs(&self) -> Vec<fcap_scene::AudioOutputRoute> {
        self.lock().audio_outputs.clone()
    }

    /// Just the CAP-N34 loudness settings — polled by the audio bridge.
    pub fn loudness(&self) -> LoudnessSettings {
        self.lock().loudness
    }

    /// Just the CAP-N47 LTC settings — polled by the audio bridge.
    pub fn ltc(&self) -> LtcSettings {
        self.lock().ltc.clone()
    }

    /// A CAP-N37 soundboard pad by id (for the trigger command).
    pub fn soundboard_pad(&self, id: &str) -> Option<SoundboardPad> {
        self.lock()
            .soundboard
            .pads
            .iter()
            .find(|pad| pad.id == id)
            .cloned()
    }

    /// Replace the settings and persist them atomically.
    ///
    /// Two fields are deliberately **not** replaceable through here, because
    /// they are machine-level facts rather than user-editable preferences:
    /// `accepted_eula_version` and `completed_onboarding`. A caller would
    /// otherwise silently reset them and re-show the gate or the wizard on the
    /// next launch — `settings_set`, because a stale UI snapshot carries the
    /// pre-acceptance value, and `profile_switch`, because a profile
    /// snapshotted earlier carries it too. Only [`Self::accept_eula`] and
    /// [`Self::complete_onboarding`] may write them.
    pub fn set(&self, next: Settings) -> io::Result<()> {
        {
            let mut guard = self.lock();
            let accepted = guard.accepted_eula_version.clone();
            let onboarded = guard.completed_onboarding;
            let counter = guard.recording.counter;
            // Camera profiles (CAP-M18) are written server-side while the
            // user drags a slider — a stale dialog snapshot must not clobber
            // them (the counter/EULA pattern).
            let camera_profiles = std::mem::take(&mut guard.camera_profiles);
            let hdr_tone_map = std::mem::take(&mut guard.hdr_tone_map);
            let cursor_fx = std::mem::take(&mut guard.cursor_fx);
            *guard = next;
            guard.accepted_eula_version = accepted;
            guard.completed_onboarding = onboarded;
            guard.recording.counter = counter;
            guard.camera_profiles = camera_profiles;
            guard.hdr_tone_map = hdr_tone_map;
            guard.cursor_fx = cursor_fx;
        }
        self.persist()
    }

    /// Advance the `{counter}` filename token and return the new value — the
    /// only path allowed to write it (CAP-M25). A persist failure keeps the
    /// in-memory bump (the recording must not be blocked on a settings write).
    /// Write one camera control into a device's profile (CAP-M18) and
    /// persist. The only writer besides `reset_camera_profile`.
    pub fn set_camera_control(&self, device_id: &str, control: &str, value: i64) {
        {
            let mut guard = self.lock();
            guard
                .camera_profiles
                .entry(device_id.to_string())
                .or_default()
                .insert(control.to_string(), value);
        }
        if let Err(err) = self.persist() {
            eprintln!("settings: could not persist a camera profile: {err}");
        }
    }

    /// Write one display's HDR→SDR tone-map (CAP-N74) and persist; also the
    /// reader's counterpart `hdr_tone_map(capture_id)`. The only writer.
    pub fn set_hdr_tone_map(&self, capture_id: &str, setting: HdrToneMapSetting) {
        {
            let mut guard = self.lock();
            guard.hdr_tone_map.insert(capture_id.to_string(), setting);
        }
        if let Err(err) = self.persist() {
            eprintln!("settings: could not persist an HDR tone-map: {err}");
        }
    }

    /// One display's saved tone-map, if any (CAP-N74).
    pub fn hdr_tone_map(&self, capture_id: &str) -> Option<HdrToneMapSetting> {
        self.lock().hdr_tone_map.get(capture_id).cloned()
    }

    /// Write one capture's cursor effects (CAP-N19) and persist; also the
    /// reader's counterpart `cursor_fx(capture_id)`. The only writer.
    pub fn set_cursor_fx(&self, capture_id: &str, setting: CursorFxSetting) {
        {
            let mut guard = self.lock();
            guard.cursor_fx.insert(capture_id.to_string(), setting);
        }
        if let Err(err) = self.persist() {
            eprintln!("settings: could not persist cursor effects: {err}");
        }
    }

    /// One capture's saved cursor effects, if any (CAP-N19).
    pub fn cursor_fx(&self, capture_id: &str) -> Option<CursorFxSetting> {
        self.lock().cursor_fx.get(capture_id).cloned()
    }

    /// Drop a device's whole camera profile (CAP-M18) and persist.
    pub fn reset_camera_profile(&self, device_id: &str) {
        {
            let mut guard = self.lock();
            guard.camera_profiles.remove(device_id);
        }
        if let Err(err) = self.persist() {
            eprintln!("settings: could not persist a camera profile reset: {err}");
        }
    }

    /// One device's saved camera profile as (tag, value) pairs (CAP-M18).
    pub fn camera_profile(&self, device_id: &str) -> Vec<(String, i64)> {
        self.lock()
            .camera_profiles
            .get(device_id)
            .map(|profile| profile.iter().map(|(k, v)| (k.clone(), *v)).collect())
            .unwrap_or_default()
    }

    pub fn bump_recording_counter(&self) -> u32 {
        let value = {
            let mut guard = self.lock();
            guard.recording.counter = guard.recording.counter.wrapping_add(1);
            guard.recording.counter
        };
        if let Err(err) = self.persist() {
            eprintln!("settings: could not persist the filename counter: {err}");
        }
        value
    }

    /// Record acceptance of `version` — the only path allowed to write the EULA
    /// field. Idempotent.
    pub fn accept_eula(&self, version: &str) -> io::Result<()> {
        {
            let mut guard = self.lock();
            guard.accepted_eula_version = Some(version.to_owned());
        }
        self.persist()
    }

    /// Mark the first-run wizard as seen — the only path allowed to write it.
    /// Called when the user finishes **or skips**, so a skipped wizard never
    /// returns uninvited. Idempotent.
    pub fn complete_onboarding(&self) -> io::Result<()> {
        {
            let mut guard = self.lock();
            guard.completed_onboarding = true;
        }
        self.persist()
    }

    fn persist(&self) -> io::Result<()> {
        let Some(path) = &self.path else {
            // In-memory mode (no home directory) — announced at startup.
            return Ok(());
        };
        // Hold the lock across the write so concurrent saves serialize and
        // the file always reflects a complete snapshot.
        let guard = self.lock();
        let json =
            serde_json::to_string_pretty(&*guard).expect("settings always serialize to JSON");
        write_atomic(path, &json)
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Settings> {
        // A poisoned lock only means another thread panicked mid-update; the
        // settings value itself is always a complete struct, so recover it.
        self.current
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

fn read_settings(path: &Path) -> Settings {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Settings::default(),
        Err(err) => {
            eprintln!(
                "settings: cannot read {} ({err}); using defaults",
                path.display()
            );
            return Settings::default();
        }
    };

    // Parsed to a `Value` first so a migration can ask what the file actually
    // *said*, which `Settings` can no longer tell it: `serde(default)` has by
    // then filled every missing key, erasing the difference between "absent"
    // and "explicitly false".
    let raw: serde_json::Value = match serde_json::from_str(&text) {
        Ok(raw) => raw,
        Err(err) => {
            eprintln!(
                "settings: {} is not valid settings JSON ({err}); using defaults",
                path.display()
            );
            return Settings::default();
        }
    };

    match Settings::deserialize(&raw) {
        Ok(mut settings) => {
            migrate(&mut settings, &raw);
            settings
        }
        Err(err) => {
            eprintln!(
                "settings: {} is not valid settings JSON ({err}); using defaults",
                path.display()
            );
            Settings::default()
        }
    }
}

/// Bring a settings file written by an older build up to date.
fn migrate(settings: &mut Settings, raw: &serde_json::Value) {
    // `completedOnboarding` arrived in 0.96.0 and defaults to `false`, so every
    // *existing* user would be greeted by the first-run wizard on upgrade — and
    // its template step would add a second display capture on top of the scene
    // they had already arranged. A file that predates the field but has already
    // accepted an EULA has plainly been run before. Its first run is long past.
    //
    // Keyed on the field being *absent*, not falsy: a 0.96.0 user who quit
    // mid-wizard wrote `false` on purpose and must see it again.
    let predates_onboarding = raw.get("completedOnboarding").is_none();
    if predates_onboarding && settings.accepted_eula_version.is_some() {
        settings.completed_onboarding = true;
    }

    // 0.301.0 tightened hotkey validation from "any string ≤64 chars" to a
    // real accelerator parse. A pre-0.301.0 free-text binding that never
    // parsed (so never registered) would now fail whole-struct validation and
    // block every settings save — drop it to unbound instead.
    settings.hotkeys.sanitize();
}

/// Write via a unique sibling temp file + fsync + rename so the file is
/// always either the old or the new complete content — across app crashes
/// and (best-effort) power loss — and concurrent writers (e.g. a second app
/// instance) never collide on the temp path. Shared with the studio's
/// scene-collection persistence.
pub(crate) fn write_atomic(path: &Path, content: &str) -> io::Result<()> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let tmp = path.with_extension(format!("{}.{nanos}.tmp", std::process::id()));

    let result = (|| {
        let mut file = fs::File::create(&tmp)?;
        file.write_all(content.as_bytes())?;
        // Flush data before the rename so a power cut can't leave a
        // truncated file behind the metadata commit. (Directory fsync is
        // not portable to Windows; the file sync is the practical bound.)
        file.sync_all()?;
        drop(file);
        fs::rename(&tmp, path)
    })();

    if result.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    result
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;

    /// A unique temp path per test so parallel tests never collide.
    fn temp_path(tag: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "freally-capture-test-{}-{}-{tag}.json",
            std::process::id(),
            nanos
        ))
    }

    #[test]
    fn missing_file_yields_defaults_and_materializes() {
        let path = temp_path("missing");
        let store = SettingsStore::load_from(path.clone());
        assert_eq!(store.get(), Settings::default());
        assert!(path.exists(), "first load should write the defaults file");
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn set_persists_across_loads() {
        let path = temp_path("roundtrip");
        let store = SettingsStore::load_from(path.clone());
        let next = Settings {
            language: "de".to_owned(),
            show_stats_dock: false,
            monitor_device: Some("Speakers (Realtek)".to_owned()),
            audio_outputs: vec![fcap_scene::AudioOutputRoute {
                bus: fcap_scene::OutputBus::Track { index: 3 },
                device_id: "Headphones (USB)".to_owned(),
                gain_db: -3.0,
            }],
            loudness: LoudnessSettings {
                enabled: true,
                target_lufs: -14.0,
                ceiling_db: -1.5,
            },
            ltc: LtcSettings {
                enabled: true,
                track: 4,
                fps: 25,
                read_source: "33333333-3333-3333-3333-333333333333".to_owned(),
            },
            soundboard: SoundboardSettings {
                pads: vec![SoundboardPad {
                    id: "11111111-1111-1111-1111-111111111111".to_owned(),
                    name: "Applause".to_owned(),
                    path: "C:/sfx/applause.wav".to_owned(),
                    hotkey: Some("Ctrl+Shift+1".to_owned()),
                    gain_db: -3.0,
                    tracks: 0b11,
                    choke_group: 1,
                    looping: false,
                    auto_duck: true,
                }],
            },
            mixer_layout: MixerLayout::Vertical,
            theme: ThemeSettings {
                mode: ThemeMode::Custom,
                accent: "#00d4ff".to_owned(),
            },
            alignment: AlignmentSettings {
                smart_guides: false,
                safe_areas: true,
                rulers: true,
            },
            accessibility: AccessibilitySettings {
                meter_preset: MeterPreset::Custom,
                meter_low: "#0072b2".to_owned(),
                meter_mid: "#e69f00".to_owned(),
                meter_high: "#d55e00".to_owned(),
            },
            recording: RecordingSettings {
                container: Container::Mkv,
                split_minutes: 30,
                iso_sources: vec!["22222222-2222-2222-2222-222222222222".to_owned()],
                iso_post_filter: false,
                iso_container: Container::Mkv,
                alpha_frec: true,
                split_on_scene: true,
                split_on_marker: true,
                auto_markers: true,
                pipeline_enabled: true,
                pipeline: vec![
                    PipelineStep::Verify,
                    PipelineStep::Remux,
                    PipelineStep::Rename {
                        template: "{prefix} {date}".to_owned(),
                    },
                    PipelineStep::Copy {
                        folder: "D:/archive".to_owned(),
                    },
                    PipelineStep::LuaEvent,
                ],
                ..RecordingSettings::default()
            },
            remote: RemoteSettings {
                turn_url: "turns:relay.example.net:5349".to_owned(),
                turn_username: "me".to_owned(),
                turn_credential: "s3cret".to_owned(),
            },
            stream: StreamSettings {
                targets: vec![
                    StreamTargetSettings {
                        service: fcap_stream::StreamService::YouTube,
                        stream_key: "yt-key".to_owned(),
                        bitrate_kbps: 4_500,
                        ..StreamTargetSettings::default()
                    },
                    StreamTargetSettings {
                        service: fcap_stream::StreamService::Twitch,
                        stream_key: "tw-key".to_owned(),
                        track: 2,
                        ..StreamTargetSettings::default()
                    },
                ],
                auto_record: true,
                preflight_hold: true,
            },
            replay: ReplaySettings {
                seconds: 60,
                ..ReplaySettings::default()
            },
            transition: TransitionSettings {
                kind: fcap_scene::TransitionKind::SlideLeft,
                duration_ms: 500,
                ..TransitionSettings::default()
            },
            hotkeys: HotkeySettings {
                record: Some("Ctrl+Shift+R".to_owned()),
                go_live: None,
                transition: Some("F13".to_owned()),
                save_replay: Some("Ctrl+Shift+S".to_owned()),
                add_marker: None,
                still: Some("Ctrl+Shift+P".to_owned()),
                panic: Some("Ctrl+Shift+F12".to_owned()),
                timer_toggle: Some("Ctrl+Shift+T".to_owned()),
                timer_reset: None,
                zoom_100: Some("Ctrl+Shift+0".to_owned()),
                zoom_150: None,
                zoom_200: Some("Ctrl+Shift+2".to_owned()),
                split_timer_split: Some("Numpad1".to_owned()),
                split_timer_undo: Some("Numpad8".to_owned()),
                split_timer_skip: None,
                split_timer_reset: Some("Numpad3".to_owned()),
                playlist_next: Some("Ctrl+Alt+Right".to_owned()),
                playlist_previous: None,
                replay_roll: Some("Ctrl+Shift+I".to_owned()),
            },
            panic_slate: PanicSlateSettings {
                color: "#221100".to_owned(),
                image: "C:/art/brb.png".to_owned(),
            },
            remote_control: RemoteControlSettings {
                enabled: true,
                port: 4460,
                lan: false,
                password: "deck-pass".to_owned(),
            },
            browser_docks: vec![BrowserDockSettings {
                name: "Twitch Chat".to_owned(),
                url: "https://www.twitch.tv/popout/someone/chat".to_owned(),
            }],
            scripts: vec![ScriptSettings {
                path: "C:/scripts/go-live.lua".to_owned(),
                enabled: true,
            }],
            accepted_eula_version: Some("2026-07-08".to_owned()),
            completed_onboarding: true,
            camera_profiles: std::collections::HashMap::from([(
                "cam-0".to_owned(),
                std::collections::HashMap::from([("exposure".to_owned(), -6_i64)]),
            )]),
            hdr_tone_map: std::collections::HashMap::from([(
                r"display:\\.\DISPLAY1".to_owned(),
                HdrToneMapSetting {
                    operator: "maxRgb".to_owned(),
                    paper_white_nits: 240,
                },
            )]),
            cursor_fx: std::collections::HashMap::from([(
                r"display:\\.\DISPLAY1".to_owned(),
                CursorFxSetting {
                    halo: true,
                    halo_color: "#ffd54a".to_owned(),
                    halo_radius: 32,
                    ripples: true,
                    left_color: "#4ac1ff".to_owned(),
                    right_color: "#ff5a5a".to_owned(),
                    keystrokes: true,
                },
            )]),
            // Automation (CAP-N01/N02): a real macro + rule round-trips too.
            automation: crate::automation::AutomationSettings {
                macros: vec![crate::automation::Macro {
                    name: "Intro".to_owned(),
                    steps: vec![crate::automation::MacroStep::Action {
                        command: "startRecording".to_owned(),
                        params: serde_json::Value::Null,
                    }],
                    repeat: 1,
                    hotkey: Some("Ctrl+Shift+I".to_owned()),
                    layer: None,
                }],
                rules: vec![crate::automation::Rule {
                    name: "on-live".to_owned(),
                    enabled: true,
                    trigger: crate::automation::Trigger::StreamState { live: true },
                    conditions: Vec::new(),
                    actions: Vec::new(),
                    macro_name: "Intro".to_owned(),
                }],
            },
            // MIDI (CAP-N03) round-trips too.
            midi: crate::midi::MidiSettings {
                input: "Launchpad".to_owned(),
                output: "Launchpad".to_owned(),
                bindings: vec![crate::midi::MidiBinding {
                    control: crate::midi::MidiControl::Note {
                        channel: 0,
                        note: 60,
                    },
                    target: crate::midi::MidiTarget::Action {
                        command: "startRecording".to_owned(),
                        params: serde_json::Value::Null,
                    },
                    feedback: true,
                }],
            },
            // PTZ (CAP-N08) round-trips too.
            ptz: crate::ptz::PtzSettings {
                cameras: vec![crate::ptz::PtzCamera {
                    name: "Wide".to_owned(),
                    host: "192.168.1.50".to_owned(),
                    port: 52381,
                    presets: vec![crate::ptz::PtzPreset {
                        name: "Two-shot".to_owned(),
                        slot: 3,
                    }],
                    scene_recalls: vec![crate::ptz::SceneRecall {
                        scene: "Interview".to_owned(),
                        slot: 3,
                    }],
                }],
            },
            // OSC (CAP-N04) round-trips too.
            osc: crate::osc::OscSettings {
                enabled: true,
                port: 9000,
                lan: true,
            },
            // The LAN panel + tally (CAP-N06/N07) round-trips too.
            web_panel: crate::webpanel::WebPanelSettings {
                enabled: true,
                port: 4457,
                lan: true,
                password: "panel-pass".to_owned(),
            },
            // The Freally Link output (CAP-N12) round-trips too.
            link: crate::link::LinkSettings {
                enabled: true,
                port: 9725,
                name: "Studio PC".to_owned(),
                key: "studio-link-key".to_owned(),
            },
            // The show rundown (CAP-N09) round-trips too.
            rundown: crate::rundown::RundownSettings {
                steps: vec![crate::rundown::RundownStep {
                    name: "Start-soon loop".to_owned(),
                    scene: "Intro".to_owned(),
                    hold_secs: 300,
                    actions: Vec::new(),
                }],
                auto_advance: true,
            },
        };
        // `set` never writes the EULA field or the onboarding flag (see its
        // docs) — go through their real writers first, so the round-trip below
        // compares the whole struct.
        store.accept_eula("2026-07-08").expect("accept eula");
        store.complete_onboarding().expect("complete onboarding");
        // Camera profiles (CAP-M18) are preserved by `set` too — write the
        // fixture's profile through the real writer.
        store.set_camera_control("cam-0", "exposure", -6);
        // HDR tone-maps (CAP-N74) are preserved by `set` too — same pattern.
        store.set_hdr_tone_map(
            r"display:\\.\DISPLAY1",
            HdrToneMapSetting {
                operator: "maxRgb".to_owned(),
                paper_white_nits: 240,
            },
        );
        // Cursor effects (CAP-N19) are preserved by `set` too — same pattern.
        store.set_cursor_fx(
            r"display:\\.\DISPLAY1",
            CursorFxSetting {
                halo: true,
                halo_color: "#ffd54a".to_owned(),
                halo_radius: 32,
                ripples: true,
                left_color: "#4ac1ff".to_owned(),
                right_color: "#ff5a5a".to_owned(),
                keystrokes: true,
            },
        );
        store.set(next.clone()).expect("save settings");

        let reloaded = SettingsStore::load_from(path.clone());
        assert_eq!(reloaded.get(), next);
        let _ = fs::remove_file(&path);
    }

    /// A settings dialog saved mid-tweak must not clobber a camera profile
    /// written server-side after the dialog snapshotted (CAP-M18).
    #[test]
    fn set_preserves_camera_profiles() {
        let path = temp_path("camera-profiles");
        let store = SettingsStore::load_from(path.clone());
        store.set_camera_control("cam-1", "zoom", 150);

        let stale: Settings = serde_json::from_str(r#"{"language":"fr"}"#).expect("parses");
        assert!(stale.camera_profiles.is_empty());
        store.set(stale).expect("save settings");

        assert_eq!(
            store.camera_profile("cam-1"),
            vec![("zoom".to_owned(), 150)],
            "the live tweak survives the stale dialog save"
        );
        store.reset_camera_profile("cam-1");
        assert!(store.camera_profile("cam-1").is_empty());

        // The HDR tone-map (CAP-N74) survives a stale save the same way.
        store.set_hdr_tone_map(
            "display:X",
            HdrToneMapSetting {
                operator: "bt2408".to_owned(),
                paper_white_nits: 200,
            },
        );
        let stale: Settings = serde_json::from_str(r#"{"language":"de"}"#).expect("parses");
        store.set(stale).expect("save settings");
        assert_eq!(
            store.hdr_tone_map("display:X").map(|tone| tone.operator),
            Some("bt2408".to_owned()),
            "the tone-map survives the stale dialog save"
        );

        // Cursor effects (CAP-N19) survive a stale save the same way.
        store.set_cursor_fx(
            "display:X",
            CursorFxSetting {
                halo: true,
                ..CursorFxSetting::default()
            },
        );
        let stale: Settings = serde_json::from_str(r#"{"language":"it"}"#).expect("parses");
        store.set(stale).expect("save settings");
        assert_eq!(
            store.cursor_fx("display:X").map(|fx| fx.halo),
            Some(true),
            "the cursor effects survive the stale dialog save"
        );
        let _ = fs::remove_file(&path);
    }

    /// The UI's settings payload omits `acceptedEulaVersion`, so `serde(default)`
    /// hands `settings_set` a `None`. Saving any dialog must not re-arm the EULA
    /// gate on the next launch.
    #[test]
    fn set_cannot_clear_an_accepted_eula() {
        let path = temp_path("eula-set");
        let store = SettingsStore::load_from(path.clone());
        store.accept_eula("2026-07-08").expect("accept eula");

        // Exactly what `settings_set` receives from the UI: the field absent.
        let from_ui: Settings =
            serde_json::from_str(r#"{"language":"fr"}"#).expect("UI payload parses");
        assert_eq!(from_ui.accepted_eula_version, None);
        store.set(from_ui).expect("save settings");

        assert_eq!(
            store.get().language,
            "fr",
            "the rest of the payload applies"
        );
        assert_eq!(
            store.get().accepted_eula_version.as_deref(),
            Some("2026-07-08"),
            "acceptance must survive a settings save"
        );
        // And it survives a restart.
        let reloaded = SettingsStore::load_from(path.clone());
        assert_eq!(
            reloaded.get().accepted_eula_version.as_deref(),
            Some("2026-07-08"),
            "acceptance must survive a relaunch"
        );
        let _ = fs::remove_file(&path);
    }

    /// The first-run wizard is a machine-level fact, exactly like EULA
    /// acceptance — and it fails the same way. A stale `settings_set` payload or
    /// a profile snapshotted before the wizard ran would reset it, and the
    /// wizard would greet the user on every launch, forever.
    #[test]
    fn set_cannot_un_complete_the_onboarding() {
        let path = temp_path("onboarding-set");
        let store = SettingsStore::load_from(path.clone());
        assert!(!store.get().completed_onboarding, "fresh install");
        store.complete_onboarding().expect("complete onboarding");

        // Exactly what a stale UI payload looks like: the field absent.
        let from_ui: Settings =
            serde_json::from_str(r#"{"language":"fr"}"#).expect("UI payload parses");
        assert!(!from_ui.completed_onboarding);
        store.set(from_ui).expect("save settings");

        assert!(
            store.get().completed_onboarding,
            "must survive a settings save"
        );
        assert_eq!(
            store.get().language,
            "fr",
            "the rest of the payload applies"
        );

        let reloaded = SettingsStore::load_from(path.clone());
        assert!(
            reloaded.get().completed_onboarding,
            "must survive a relaunch"
        );
        let _ = fs::remove_file(&path);
    }

    /// A profile snapshotted before acceptance carries `None`; switching to it
    /// must not re-arm the gate either.
    #[test]
    fn profile_load_cannot_clear_an_accepted_eula() {
        let path = temp_path("eula-profile");
        let store = SettingsStore::load_from(path.clone());
        store.accept_eula("2026-07-08").expect("accept eula");

        let stale_profile = Settings {
            accepted_eula_version: None,
            ..Settings::default()
        };
        store.set(stale_profile).expect("load profile");

        assert_eq!(
            store.get().accepted_eula_version.as_deref(),
            Some("2026-07-08"),
            "acceptance must survive a profile switch"
        );
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn remote_control_settings_validate() {
        // Off by default, and the default validates.
        let defaults = RemoteControlSettings::default();
        assert!(!defaults.enabled);
        assert!(defaults.validate().is_ok());
        // Enabling without a password is refused.
        let no_password = RemoteControlSettings {
            enabled: true,
            ..Default::default()
        };
        assert!(no_password.validate().is_err());
        // A privileged port is refused; a sane config passes.
        let privileged = RemoteControlSettings {
            port: 80,
            ..Default::default()
        };
        assert!(privileged.validate().is_err());
        let ok = RemoteControlSettings {
            enabled: true,
            port: 4456,
            lan: false,
            password: "deck-pass".to_owned(),
        };
        assert!(ok.validate().is_ok());
        // The password never appears in Debug output.
        assert!(!format!("{ok:?}").contains("deck-pass"));
    }

    #[test]
    fn corrupt_file_falls_back_to_defaults() {
        let path = temp_path("corrupt");
        fs::write(&path, "definitely not json {").expect("write corrupt file");
        let store = SettingsStore::load_from(path.clone());
        assert_eq!(store.get(), Settings::default());
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn validate_bounds_the_monitor_device() {
        let ok = Settings {
            monitor_device: Some("Headphones (USB)".to_owned()),
            ..Settings::default()
        };
        assert!(ok.validate().is_ok());

        for bad in ["x".repeat(300), "spk\u{0007}".to_owned()] {
            let settings = Settings {
                monitor_device: Some(bad.clone()),
                ..Settings::default()
            };
            assert!(settings.validate().is_err(), "should reject {bad:?}");
        }
    }

    #[test]
    fn validate_bounds_the_audio_output_routes() {
        use fcap_scene::{AudioOutputRoute, OutputBus};

        // A sane route on the master and a track passes.
        let ok = Settings {
            audio_outputs: vec![
                AudioOutputRoute {
                    bus: OutputBus::Master,
                    device_id: "Recorder".to_owned(),
                    gain_db: 0.0,
                },
                AudioOutputRoute {
                    bus: OutputBus::Track { index: 4 },
                    device_id: String::new(), // OS default is legal
                    gain_db: -6.0,
                },
            ],
            ..Settings::default()
        };
        assert!(ok.validate().is_ok());

        // Two routes fighting over the same bus is rejected.
        let dup = Settings {
            audio_outputs: vec![
                AudioOutputRoute {
                    bus: OutputBus::Master,
                    device_id: "A".to_owned(),
                    gain_db: 0.0,
                },
                AudioOutputRoute {
                    bus: OutputBus::Master,
                    device_id: "B".to_owned(),
                    gain_db: 0.0,
                },
            ],
            ..Settings::default()
        };
        assert!(dup.validate().is_err(), "one bus, at most one route");

        // A nonexistent track, an out-of-range trim, and a junk device name.
        for bad in [
            AudioOutputRoute {
                bus: OutputBus::Track { index: 9 },
                device_id: String::new(),
                gain_db: 0.0,
            },
            AudioOutputRoute {
                bus: OutputBus::Master,
                device_id: String::new(),
                gain_db: 999.0,
            },
            AudioOutputRoute {
                bus: OutputBus::Master,
                device_id: "spk\u{0007}".to_owned(),
                gain_db: 0.0,
            },
        ] {
            let settings = Settings {
                audio_outputs: vec![bad.clone()],
                ..Settings::default()
            };
            assert!(settings.validate().is_err(), "should reject {bad:?}");
        }
    }

    #[test]
    fn validate_bounds_the_turn_relay() {
        let ok = Settings {
            remote: RemoteSettings {
                turn_url: "turn:relay.example.net:3478".to_owned(),
                turn_username: "me".to_owned(),
                turn_credential: "s3cret".to_owned(),
            },
            ..Settings::default()
        };
        assert!(ok.validate().is_ok());
        assert!(
            Settings::default().validate().is_ok(),
            "empty = direct-only is fine"
        );

        for bad_url in [
            "http://relay",
            "stun:only",
            "x".repeat(600).as_str(),
            "turn:a\u{0007}",
        ] {
            let settings = Settings {
                remote: RemoteSettings {
                    turn_url: bad_url.to_owned(),
                    ..RemoteSettings::default()
                },
                ..Settings::default()
            };
            assert!(settings.validate().is_err(), "should reject {bad_url:?}");
        }
    }

    #[test]
    fn validate_bounds_the_hotkey_accelerators() {
        let with = |accelerator: &str| Settings {
            hotkeys: HotkeySettings {
                record: Some(accelerator.to_owned()),
                ..HotkeySettings::default()
            },
            ..Settings::default()
        };

        // The curated pool's shapes, a structurally-valid value from OUTSIDE
        // the pool (still honest), a chord (CAP-N05), legacy "" (unbound), AND
        // the navigation/punctuation keys the old free-text field allowed and
        // may have persisted — validation must not have regressed on them
        // (they register fine, so they must save).
        for good in [
            "Ctrl+D",
            "Ctrl+Shift+R",
            "Ctrl+Alt+Right",
            "Numpad1",
            "F13",
            "Super+F20",
            "Ctrl+K, 3",
            "Space",
            "Enter",
            "Delete",
            "Home",
            "PageDown",
            "",
        ] {
            assert!(with(good).validate().is_ok(), "should accept {good:?}");
        }

        // The garbage-string class the combobox exists to prevent — an
        // unparseable key and an unknown modifier. The UI is not the only
        // writer, so the store must refuse these too.
        for bad in ["Ctrl+asekfjkakjfsdaajklfksdjdjksfjkasdf", "Wibble+R"] {
            assert!(with(bad).validate().is_err(), "should reject {bad:?}");
        }

        // The error names the offending field, so the dialog can say WHICH
        // binding to fix rather than shrugging generically.
        let err = with("Ctrl+asekfjkakjfsdaajklfksdjdjksfjkasdf")
            .validate()
            .expect_err("garbage must not validate");
        assert!(err.contains("record"), "error should name the field: {err}");
    }

    #[test]
    fn loading_clears_an_unparseable_legacy_hotkey_instead_of_blocking_saves() {
        // A pre-0.301.0 file could store a free-text binding that never parsed
        // (never registered). It must not fail validation forever — load-time
        // sanitize drops it to unbound so settings stay saveable.
        let mut hotkeys = HotkeySettings {
            record: Some("totally not a shortcut".to_owned()),
            go_live: Some("Ctrl+D".to_owned()),
            ..HotkeySettings::default()
        };
        hotkeys.sanitize();
        assert_eq!(hotkeys.record, None, "the garbage binding is dropped");
        assert_eq!(
            hotkeys.go_live.as_deref(),
            Some("Ctrl+D"),
            "a valid binding is untouched"
        );
        let settings = Settings {
            hotkeys,
            ..Settings::default()
        };
        assert!(
            settings.validate().is_ok(),
            "a sanitized struct validates and can be saved"
        );
    }

    #[test]
    fn validate_bounds_the_meter_colours() {
        assert!(Settings::default().validate().is_ok());

        // Anything but a plain hex triple could escape the CSS gradient the
        // colours land in — the same injection gate as the theme accent.
        for bad in ["", "#12345", "#gggggg", "red", "#22c55e;}"] {
            let settings = Settings {
                accessibility: AccessibilitySettings {
                    meter_low: bad.to_owned(),
                    ..AccessibilitySettings::default()
                },
                ..Settings::default()
            };
            assert!(settings.validate().is_err(), "should reject {bad:?}");
        }
    }

    #[test]
    fn debug_never_prints_the_stream_key() {
        let settings = StreamSettings {
            targets: vec![StreamTargetSettings {
                stream_key: "live_secret_key".to_owned(),
                ..StreamTargetSettings::default()
            }],
            ..StreamSettings::default()
        };
        let printed = format!("{settings:?}");
        assert!(
            !printed.contains("live_secret_key"),
            "key leaked: {printed}"
        );
        assert!(printed.contains("[redacted]"));
    }

    #[test]
    fn validate_bounds_the_stream_settings() {
        assert!(Settings::default().validate().is_ok());
        let with = |target: StreamTargetSettings| StreamSettings {
            targets: vec![target],
            ..StreamSettings::default()
        };
        for (bad, why) in [
            (
                with(StreamTargetSettings {
                    ingest_url: "http://nope".to_owned(),
                    ..StreamTargetSettings::default()
                }),
                "scheme",
            ),
            (
                with(StreamTargetSettings {
                    bitrate_kbps: 100,
                    ..StreamTargetSettings::default()
                }),
                "bitrate",
            ),
            (
                with(StreamTargetSettings {
                    track: 0,
                    ..StreamTargetSettings::default()
                }),
                "track",
            ),
            (
                with(StreamTargetSettings {
                    stream_key: "x\u{0007}".to_owned(),
                    ..StreamTargetSettings::default()
                }),
                "control chars",
            ),
            (
                StreamSettings {
                    targets: vec![StreamTargetSettings::default(); 7],
                    ..StreamSettings::default()
                },
                "too many targets",
            ),
        ] {
            assert!(bad.validate().is_err(), "should reject: {why}");
        }
    }

    #[test]
    fn a_single_target_070_stream_settings_file_migrates_into_the_list() {
        // The exact flat shape 0.70.0 wrote — no `targets` key.
        let legacy = r#"{
            "stream": {
                "service": "youTube",
                "ingestUrl": "",
                "streamKey": "yt-legacy-key",
                "encoderId": "auto",
                "bitrateKbps": 4500,
                "audioBitrateKbps": 160,
                "keyframeSec": 2.0,
                "fps": 60,
                "track": 3,
                "autoRecord": true
            }
        }"#;
        let settings: Settings = serde_json::from_str(legacy).expect("parses");
        assert_eq!(settings.stream.targets.len(), 1);
        let target = &settings.stream.targets[0];
        assert!(target.enabled);
        assert_eq!(target.service, fcap_stream::StreamService::YouTube);
        assert_eq!(target.stream_key, "yt-legacy-key");
        assert_eq!(target.bitrate_kbps, 4_500);
        assert_eq!(target.track, 3);
        assert!(settings.stream.auto_record);
    }

    #[test]
    fn debug_never_prints_the_turn_credential() {
        let settings = RemoteSettings {
            turn_url: "turn:relay".to_owned(),
            turn_username: "me".to_owned(),
            turn_credential: "hunter2".to_owned(),
        };
        let printed = format!("{settings:?}");
        assert!(!printed.contains("hunter2"), "credential leaked: {printed}");
        assert!(printed.contains("[redacted]"));
    }

    /// The accent is written into a CSS custom property. A value that can close
    /// the declaration injects a rule into the page, so `validate` is a security
    /// boundary, not a formatting nicety.
    #[test]
    fn the_accent_colour_must_be_a_plain_hex_triple() {
        let ok = Settings {
            theme: ThemeSettings {
                mode: ThemeMode::Custom,
                accent: "#00d4ff".to_owned(),
            },
            ..Settings::default()
        };
        assert!(ok.validate().is_ok());

        for bad in [
            "",
            "4a9eff",                    // no `#`
            "#4a9ef",                    // too short
            "#4a9efff",                  // too long
            "#gggggg",                   // not hex
            "#4a9eff;color:red",         // closes the declaration
            "red",                       // a keyword, not a triple
            "#4a9eff}body{display:none", // escapes the rule
            "var(--x)",
        ] {
            let settings = Settings {
                theme: ThemeSettings {
                    mode: ThemeMode::Custom,
                    accent: bad.to_owned(),
                },
                ..Settings::default()
            };
            assert!(settings.validate().is_err(), "{bad:?} must be rejected");
        }
    }

    /// Switching to Custom and back must not lose the colour, and a fresh install
    /// must look exactly like every build before it.
    #[test]
    fn the_theme_defaults_to_havoc_dark() {
        let theme = ThemeSettings::default();
        assert_eq!(theme.mode, ThemeMode::Dark);
        assert_eq!(theme.accent, "#4a9eff");
        assert!(theme.validate().is_ok());
    }

    /// A fresh install must follow the OS, not force English on a Japanese user.
    /// `validate` rejects an empty tag, so the sentinel has to be a word — and it
    /// has to survive `validate`, or the first `settings_set` would be refused.
    #[test]
    fn a_fresh_install_defaults_to_following_the_system_language() {
        let settings = Settings::default();
        assert_eq!(settings.language, AUTO_LANGUAGE);
        assert!(settings.validate().is_ok(), "the sentinel must validate");
    }

    #[test]
    fn validate_bounds_the_language_tag() {
        let ok = Settings {
            language: "pt-BR".to_owned(),
            ..Settings::default()
        };
        assert!(ok.validate().is_ok());

        for bad in ["", "a".repeat(36).as_str(), "en\u{202e}", "en;rm"] {
            let settings = Settings {
                language: bad.to_owned(),
                ..Settings::default()
            };
            assert!(settings.validate().is_err(), "should reject {bad:?}");
        }
    }

    #[test]
    fn unknown_and_missing_keys_are_tolerated() {
        let path = temp_path("partial");
        fs::write(&path, r#"{ "language": "fr", "someFutureKey": 42 }"#).expect("write partial");
        let store = SettingsStore::load_from(path.clone());
        let settings = store.get();
        assert_eq!(settings.language, "fr");
        assert!(settings.show_stats_dock, "missing keys take their defaults");
        assert_eq!(
            settings.recording,
            RecordingSettings::default(),
            "recording settings default in (frec, track 1)"
        );
        let _ = fs::remove_file(&path);
    }

    /// Upgrading from 0.95.1: the file predates `completedOnboarding`, and
    /// `serde(default)` would make it `false`. Without the migration every
    /// existing user is greeted by the first-run wizard, whose template step
    /// would drop a second display capture onto the scene they already built.
    #[test]
    fn upgrading_a_settings_file_does_not_re_run_the_first_run_wizard() {
        let path = temp_path("onboarding-migrate");
        fs::write(
            &path,
            r#"{ "language": "fr", "acceptedEulaVersion": "2026-07-08" }"#,
        )
        .expect("write a 0.95.1 settings file");

        let settings = SettingsStore::load_from(path.clone()).get();
        assert!(
            settings.completed_onboarding,
            "a file that already accepted an EULA has been run before"
        );
        assert_eq!(
            settings.language, "fr",
            "the migration touches nothing else"
        );
        let _ = fs::remove_file(&path);
    }

    /// The same old file, but the EULA was never accepted — the app was never
    /// really used. That is a first run, and it gets the wizard.
    #[test]
    fn an_old_file_that_never_accepted_the_eula_is_still_a_first_run() {
        let path = temp_path("onboarding-migrate-no-eula");
        fs::write(&path, r#"{ "language": "fr" }"#).expect("write settings");

        let settings = SettingsStore::load_from(path.clone()).get();
        assert!(!settings.completed_onboarding, "never accepted, never ran");
        let _ = fs::remove_file(&path);
    }

    /// Keyed on *absent*, not falsy. A 0.96.0 user who quit halfway through the
    /// wizard wrote `false` deliberately, and must be shown it again.
    #[test]
    fn an_explicit_false_survives_the_migration() {
        let path = temp_path("onboarding-migrate-explicit");
        fs::write(
            &path,
            r#"{ "acceptedEulaVersion": "2026-07-08", "completedOnboarding": false }"#,
        )
        .expect("write a 0.96.0 settings file");

        let settings = SettingsStore::load_from(path.clone()).get();
        assert!(
            !settings.completed_onboarding,
            "an explicit false is a decision, not a missing key"
        );
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn recording_settings_validate_their_bounds() {
        assert!(RecordingSettings::default().validate().is_ok());

        let cases: Vec<(&str, RecordingSettings)> = vec![
            (
                "path separators in the prefix",
                RecordingSettings {
                    filename_prefix: "a/b".to_owned(),
                    ..RecordingSettings::default()
                },
            ),
            (
                "at least one track records",
                RecordingSettings {
                    tracks_mask: 0,
                    ..RecordingSettings::default()
                },
            ),
            (
                "bitrate floor",
                RecordingSettings {
                    rate_control: RateControl {
                        mode: RcMode::Cbr,
                        bitrate_kbps: 5,
                        cq: 23,
                    },
                    ..RecordingSettings::default()
                },
            ),
            (
                "encoder ids are ffmpeg names only",
                RecordingSettings {
                    encoder_id: "x264; rm -rf".to_owned(),
                    ..RecordingSettings::default()
                },
            ),
            (
                "zero fps",
                RecordingSettings {
                    fps: 0,
                    ..RecordingSettings::default()
                },
            ),
            (
                "ISO source ids are uuid-shaped",
                RecordingSettings {
                    iso_sources: vec!["../../etc/passwd".to_owned()],
                    ..RecordingSettings::default()
                },
            ),
            (
                "ISO sources have no duplicates",
                RecordingSettings {
                    iso_sources: vec![
                        "22222222-2222-2222-2222-222222222222".to_owned(),
                        "22222222-2222-2222-2222-222222222222".to_owned(),
                    ],
                    ..RecordingSettings::default()
                },
            ),
            (
                "ISO lane count is bounded",
                RecordingSettings {
                    iso_sources: (0..9)
                        .map(|n| format!("22222222-2222-2222-2222-22222222222{n}"))
                        .collect(),
                    ..RecordingSettings::default()
                },
            ),
            (
                "ISO encoder ids are ffmpeg names only",
                RecordingSettings {
                    iso_encoder_id: "x264; rm -rf".to_owned(),
                    ..RecordingSettings::default()
                },
            ),
            (
                "pipeline length is bounded",
                RecordingSettings {
                    pipeline: vec![PipelineStep::Reveal; 11],
                    ..RecordingSettings::default()
                },
            ),
            (
                "pipeline rename templates use known tokens",
                RecordingSettings {
                    pipeline: vec![PipelineStep::Rename {
                        template: "{not-a-token}".to_owned(),
                    }],
                    ..RecordingSettings::default()
                },
            ),
            (
                "pipeline folders reject control characters",
                RecordingSettings {
                    pipeline: vec![PipelineStep::Move {
                        folder: "D:/archive\u{0007}".to_owned(),
                    }],
                    ..RecordingSettings::default()
                },
            ),
        ];
        for (why, bad) in cases {
            assert!(bad.validate().is_err(), "should reject: {why}");
        }

        // A well-formed ISO config passes.
        assert!(RecordingSettings {
            iso_sources: vec!["22222222-2222-2222-2222-222222222222".to_owned()],
            iso_post_filter: false,
            iso_container: Container::Mkv,
            ..RecordingSettings::default()
        }
        .validate()
        .is_ok());
    }
}
