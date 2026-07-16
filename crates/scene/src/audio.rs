//! Per-source audio: the mixer state + the ordered audio filter chain.
//!
//! Audio-capable [`crate::Source`]s (Audio Input / Audio Output — Media joins
//! them in Phase 4) carry an [`AudioSettings`]: fader volume, mute, monitor
//! routing, the up-to-6 track assignment, the A/V sync offset, push-to-talk /
//! push-to-mute hotkeys, and an ordered [`AudioFilter`] chain. Like video
//! filters, parameters are plain serde data — the DSP in `fcap-audio` mirrors
//! them. Every filter is owned classic DSP; there is no ML anywhere, per the
//! charter.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::source::SourceId;

/// How many mix tracks exist (recording muxes up to all 6 in Phase 4).
pub const TRACK_COUNT: usize = 6;

/// The lowest fader position; treated as −∞ (silence) by the engine.
pub const MIN_VOLUME_DB: f32 = -60.0;

/// The highest fader position (a little headroom over unity).
pub const MAX_VOLUME_DB: f32 = 6.0;

/// The largest A/V sync offset the engine buffers (delays audio only —
/// negative offsets would need the video delayed, which recording integration
/// may add later; the model stays honest and refuses them).
pub const MAX_SYNC_OFFSET_MS: u32 = 950;

/// Stable identity of one audio filter instance (UI list keys, reorder targets).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AudioFilterId(pub Uuid);

impl AudioFilterId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for AudioFilterId {
    fn default() -> Self {
        Self::new()
    }
}

/// Where a source's monitored audio goes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MonitorMode {
    /// Not monitored; the source feeds only its assigned tracks.
    #[default]
    Off,
    /// Heard on the monitor device and excluded from the tracks/program mix.
    MonitorOnly,
    /// Heard on the monitor device *and* mixed into the tracks/program.
    MonitorAndOutput,
}

fn default_gate_open_db() -> f32 {
    -26.0
}

fn default_gate_close_db() -> f32 {
    -32.0
}

fn default_gate_attack_ms() -> f32 {
    25.0
}

fn default_gate_hold_ms() -> f32 {
    200.0
}

fn default_gate_release_ms() -> f32 {
    150.0
}

fn default_comp_ratio() -> f32 {
    4.0
}

fn default_comp_threshold_db() -> f32 {
    -18.0
}

fn default_comp_attack_ms() -> f32 {
    6.0
}

fn default_comp_release_ms() -> f32 {
    60.0
}

fn default_limit_threshold_db() -> f32 {
    -3.0
}

fn default_limit_release_ms() -> f32 {
    60.0
}

fn default_denoise_strength() -> f32 {
    0.5
}

fn default_duck_threshold_db() -> f32 {
    -30.0
}

fn default_duck_amount_db() -> f32 {
    12.0
}

fn default_duck_attack_ms() -> f32 {
    50.0
}

fn default_duck_release_ms() -> f32 {
    300.0
}

fn default_deesser_freq_hz() -> f32 {
    6_500.0
}

fn default_deesser_threshold_db() -> f32 {
    -30.0
}

fn default_deesser_amount_db() -> f32 {
    8.0
}

fn default_rumble_freq_hz() -> f32 {
    90.0
}

/// One band's shape in the parametric EQ (CAP-N35).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EqBandType {
    /// Peaking (boost/cut a band around `freq_hz`).
    Bell,
    LowShelf,
    HighShelf,
    /// Band-reject at `freq_hz`.
    Notch,
    /// 12 dB/oct high-pass (low-cut) at `freq_hz`.
    HighPass,
    /// 12 dB/oct low-pass (high-cut) at `freq_hz`.
    LowPass,
}

/// One parametric-EQ band (CAP-N35). `gain_db` is used only by bell + shelf
/// bands; notch/HP/LP ignore it. Ranges are clamped by the DSP.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EqBand {
    #[serde(rename = "type")]
    pub kind: EqBandType,
    /// Center / corner frequency, 20..=20000 Hz.
    pub freq_hz: f32,
    /// Gain for bell + shelf bands, -30..=30 dB (ignored by notch/HP/LP).
    #[serde(default)]
    pub gain_db: f32,
    /// Resonance / width, 0.1..=18.
    pub q: f32,
}

/// The starting bands for a fresh parametric EQ: three flat bells the operator
/// can drag, so the editor opens with something on the curve rather than blank.
fn default_eq_bands() -> Vec<EqBand> {
    vec![
        EqBand {
            kind: EqBandType::Bell,
            freq_hz: 120.0,
            gain_db: 0.0,
            q: 1.0,
        },
        EqBand {
            kind: EqBandType::Bell,
            freq_hz: 1_000.0,
            gain_db: 0.0,
            q: 1.0,
        },
        EqBand {
            kind: EqBandType::Bell,
            freq_hz: 6_000.0,
            gain_db: 0.0,
            q: 1.0,
        },
    ]
}

/// One audio filter's kind + parameters. Tagged by `type` in JSON.
///
/// Ranges are documented per field; the DSP clamps defensively, the UI keeps
/// its controls inside the same bounds. `rename_all_fields` is load-bearing,
/// exactly as it is for the video [`crate::FilterKind`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum AudioFilterKind {
    /// Plain make-up / trim gain.
    Gain {
        /// -30..=30 dB; 0 = unity.
        #[serde(default)]
        db: f32,
    },
    /// Classic downward noise gate with hysteresis.
    NoiseGate {
        /// Level that opens the gate, dBFS.
        #[serde(default = "default_gate_open_db")]
        open_threshold_db: f32,
        /// Level below which the gate starts closing, dBFS (≤ open).
        #[serde(default = "default_gate_close_db")]
        close_threshold_db: f32,
        /// 1..=500 ms fade-in when opening.
        #[serde(default = "default_gate_attack_ms")]
        attack_ms: f32,
        /// 0..=3000 ms the gate stays open after the level drops.
        #[serde(default = "default_gate_hold_ms")]
        hold_ms: f32,
        /// 1..=3000 ms fade-out when closing.
        #[serde(default = "default_gate_release_ms")]
        release_ms: f32,
    },
    /// Downward compressor (peak-sensing, hard knee).
    Compressor {
        /// 1..=32 : 1.
        #[serde(default = "default_comp_ratio")]
        ratio: f32,
        /// Level compression starts at, dBFS.
        #[serde(default = "default_comp_threshold_db")]
        threshold_db: f32,
        /// 0.1..=500 ms.
        #[serde(default = "default_comp_attack_ms")]
        attack_ms: f32,
        /// 1..=3000 ms.
        #[serde(default = "default_comp_release_ms")]
        release_ms: f32,
        /// -30..=30 dB make-up gain after compression.
        #[serde(default)]
        output_gain_db: f32,
    },
    /// Fast peak limiter (instant attack) with a hard safety ceiling.
    Limiter {
        /// Ceiling, dBFS.
        #[serde(default = "default_limit_threshold_db")]
        threshold_db: f32,
        /// 1..=1000 ms.
        #[serde(default = "default_limit_release_ms")]
        release_ms: f32,
    },
    /// Three-band tone EQ (low shelf / mid peak / high shelf biquads).
    Eq {
        /// -20..=20 dB below ~250 Hz.
        #[serde(default)]
        low_db: f32,
        /// -20..=20 dB around 1 kHz.
        #[serde(default)]
        mid_db: f32,
        /// -20..=20 dB above ~4 kHz.
        #[serde(default)]
        high_db: f32,
    },
    /// Owned classic-DSP spectral noise suppression (STFT noise-floor
    /// tracking + Wiener-style gain) — **no ML**, per the charter.
    Denoise {
        /// 0..=1 — how hard steady noise is suppressed.
        #[serde(default = "default_denoise_strength")]
        strength: f32,
    },
    /// Sidechain ducking: dip this source while the trigger source speaks.
    Ducker {
        /// The source whose level drives the duck (e.g. the mic). `None`
        /// leaves the filter inert until the user picks one.
        #[serde(default)]
        trigger: Option<SourceId>,
        /// Trigger level that engages the duck, dBFS.
        #[serde(default = "default_duck_threshold_db")]
        threshold_db: f32,
        /// 0..=60 dB — how far this source dips while triggered.
        #[serde(default = "default_duck_amount_db")]
        amount_db: f32,
        /// 1..=1000 ms dip-in time.
        #[serde(default = "default_duck_attack_ms")]
        attack_ms: f32,
        /// 1..=5000 ms recovery time.
        #[serde(default = "default_duck_release_ms")]
        release_ms: f32,
    },
    /// Owned N-band parametric EQ (CAP-N35): a cascade of bell / shelf / notch /
    /// high-pass / low-pass biquads with draggable nodes on a live spectrum.
    ParametricEq {
        #[serde(default = "default_eq_bands")]
        bands: Vec<EqBand>,
    },
    /// Split-band de-esser (CAP-N36): a high-band detector drives gain
    /// reduction of just the sibilance band — classic DSP, no ML.
    DeEsser {
        /// Sibilance crossover, 3000..=12000 Hz.
        #[serde(default = "default_deesser_freq_hz")]
        freq_hz: f32,
        /// High-band level that starts de-essing, dBFS.
        #[serde(default = "default_deesser_threshold_db")]
        threshold_db: f32,
        /// Maximum reduction of the sibilance band, 0..=24 dB.
        #[serde(default = "default_deesser_amount_db")]
        amount_db: f32,
    },
    /// Rumble guard (CAP-N36): a clean 2nd-order low-cut for desk thumps and
    /// plosives.
    RumbleGuard {
        /// Low-cut corner, 20..=300 Hz.
        #[serde(default = "default_rumble_freq_hz")]
        freq_hz: f32,
    },
}

impl AudioFilterKind {
    /// Machine name of this filter type (mirrors the serde tag).
    pub fn type_name(&self) -> &'static str {
        match self {
            AudioFilterKind::Gain { .. } => "gain",
            AudioFilterKind::NoiseGate { .. } => "noiseGate",
            AudioFilterKind::Compressor { .. } => "compressor",
            AudioFilterKind::Limiter { .. } => "limiter",
            AudioFilterKind::Eq { .. } => "eq",
            AudioFilterKind::Denoise { .. } => "denoise",
            AudioFilterKind::Ducker { .. } => "ducker",
            AudioFilterKind::ParametricEq { .. } => "parametricEq",
            AudioFilterKind::DeEsser { .. } => "deEsser",
            AudioFilterKind::RumbleGuard { .. } => "rumbleGuard",
        }
    }

    /// Human display name.
    pub fn display_name(&self) -> &'static str {
        match self {
            AudioFilterKind::Gain { .. } => "Gain",
            AudioFilterKind::NoiseGate { .. } => "Noise Gate",
            AudioFilterKind::Compressor { .. } => "Compressor",
            AudioFilterKind::Limiter { .. } => "Limiter",
            AudioFilterKind::Eq { .. } => "3-Band EQ",
            AudioFilterKind::Denoise { .. } => "Denoise",
            AudioFilterKind::Ducker { .. } => "Ducking",
            AudioFilterKind::ParametricEq { .. } => "Parametric EQ",
            AudioFilterKind::DeEsser { .. } => "De-esser",
            AudioFilterKind::RumbleGuard { .. } => "Rumble Guard",
        }
    }
}

fn default_enabled() -> bool {
    true
}

/// One audio filter instance in a source's chain.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioFilter {
    #[serde(default)]
    pub id: AudioFilterId,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(flatten)]
    pub kind: AudioFilterKind,
}

impl AudioFilter {
    /// A new enabled filter with a fresh id.
    pub fn new(kind: AudioFilterKind) -> Self {
        Self {
            id: AudioFilterId::new(),
            enabled: true,
            kind,
        }
    }
}

fn default_tracks() -> u8 {
    0b1 // track 1
}

const TRACK_MASK: u8 = 0b0011_1111;

/// A source's whole mixer state. Lives on the shared [`crate::Source`], so —
/// like renames — volume, mute, filters, and routing follow the source into
/// every scene that shows it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AudioSettings {
    /// Fader position, [`MIN_VOLUME_DB`]..=[`MAX_VOLUME_DB`]; the floor is
    /// treated as −∞ (silence).
    pub volume_db: f32,
    pub muted: bool,
    pub monitor: MonitorMode,
    /// Bitmask of the up-to-6 tracks this source mixes into (bit 0 = track 1).
    pub tracks: u8,
    /// Stereo balance (CAP-M19), −1 (full left) ..= 1 (full right). 0 leaves
    /// the signal untouched (unity — existing mixes are bit-identical).
    pub pan: f32,
    /// PFL solo (CAP-M19): while ANY strip is soloed, the monitor bus hears
    /// only soloed strips. The program/track mix never changes.
    pub solo: bool,
    /// Downmix to mono before the balance (CAP-M19).
    pub mono: bool,
    /// Join the gain-sharing auto-mixer (CAP-N32): the mixer keeps the total
    /// gain of all automixed strips ~constant and hands it to whoever is
    /// speaking. Off by default — a strip joins only when the operator asks.
    #[serde(default)]
    pub automix: bool,
    /// Produce a mix-minus (N−1) return for this source (CAP-N39): the program
    /// minus this source itself, so a remote guest hears everyone but their own
    /// echo. Off by default.
    #[serde(default)]
    pub mix_minus: bool,
    /// Delays this source's audio to line it up with video, ms
    /// (0..=[`MAX_SYNC_OFFSET_MS`]).
    pub sync_offset_ms: u32,
    /// Hotkey accelerator (e.g. `"Ctrl+Shift+T"`): silent **unless** held.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_to_talk: Option<String>,
    /// Hotkey accelerator: silent **while** held.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_to_mute: Option<String>,
    /// The ordered filter chain (applied before the fader).
    pub filters: Vec<AudioFilter>,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            volume_db: 0.0,
            muted: false,
            monitor: MonitorMode::Off,
            tracks: default_tracks(),
            pan: 0.0,
            solo: false,
            mono: false,
            automix: false,
            mix_minus: false,
            sync_offset_ms: 0,
            push_to_talk: None,
            push_to_mute: None,
            filters: Vec::new(),
        }
    }
}

impl AudioSettings {
    /// Pull every field back inside its documented range (load-time repair —
    /// a hand-edited file can never ask the engine for a 10-second delay
    /// buffer or a +100 dB fader).
    pub fn clamp(&mut self) {
        self.volume_db = if self.volume_db.is_finite() {
            self.volume_db.clamp(MIN_VOLUME_DB, MAX_VOLUME_DB)
        } else {
            0.0
        };
        self.tracks &= TRACK_MASK;
        self.pan = if self.pan.is_finite() {
            self.pan.clamp(-1.0, 1.0)
        } else {
            0.0
        };
        self.sync_offset_ms = self.sync_offset_ms.min(MAX_SYNC_OFFSET_MS);
    }

    /// Whether track `index` (0-based) is assigned.
    pub fn track_enabled(&self, index: usize) -> bool {
        index < TRACK_COUNT && self.tracks & (1 << index) != 0
    }
}

/// Which mixer bus a physical-output route (CAP-N30) carries. The monitor bus
/// keeps its own dedicated device (`Settings.monitor_device`); these routes
/// send the **program buses** — the master mix and the six track buses — to
/// *additional* physical outputs (a hardware-recorder feed, a confidence
/// speaker in another room, or a headphone cue built on a spare track bus).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "bus", rename_all = "camelCase")]
pub enum OutputBus {
    /// The program (master) mix.
    Master,
    /// One of the six track buses (0-based `index`; 0 = "Track 1").
    Track { index: u8 },
}

impl OutputBus {
    /// A stable, compact key for wire/UI use: `"master"` or `"track1"`..`"track6"`.
    pub fn key(&self) -> String {
        match self {
            OutputBus::Master => "master".to_owned(),
            OutputBus::Track { index } => format!("track{}", index.saturating_add(1)),
        }
    }
}

/// One physical-output route (CAP-N30): a program [`OutputBus`] → an output
/// device, with a trim. `device_id` empty = the OS default output. Off by
/// default — the mixer ships with no routes, so nothing but the monitor bus
/// reaches a device unless the operator adds one.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioOutputRoute {
    /// The bus this route sends (flattened onto the wire object, so a route is
    /// `{ "bus": "master", … }` or `{ "bus": "track", "index": 2, … }`).
    #[serde(flatten)]
    pub bus: OutputBus,
    /// The output device name ("" = the OS default), as the OS reports it.
    #[serde(default)]
    pub device_id: String,
    /// Output trim, [`MIN_VOLUME_DB`]..=[`MAX_VOLUME_DB`] dB; the floor is
    /// silence. 0 dB passes the bus through unchanged.
    #[serde(default)]
    pub gain_db: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_the_neutral_strip() {
        let settings = AudioSettings::default();
        assert_eq!(settings.volume_db, 0.0);
        assert!(!settings.muted);
        assert_eq!(settings.monitor, MonitorMode::Off);
        assert!(settings.track_enabled(0), "track 1 assigned by default");
        assert!(!settings.track_enabled(1));
        assert_eq!(settings.sync_offset_ms, 0);
        assert!(settings.filters.is_empty());
    }

    #[test]
    fn clamp_repairs_out_of_range_values() {
        let mut settings = AudioSettings {
            volume_db: f32::NAN,
            tracks: 0xFF,
            sync_offset_ms: 100_000,
            ..AudioSettings::default()
        };
        settings.clamp();
        assert_eq!(settings.volume_db, 0.0);
        assert_eq!(settings.tracks, TRACK_MASK);
        assert_eq!(settings.sync_offset_ms, MAX_SYNC_OFFSET_MS);

        settings.volume_db = 40.0;
        settings.clamp();
        assert_eq!(settings.volume_db, MAX_VOLUME_DB);
        settings.volume_db = -100.0;
        settings.clamp();
        assert_eq!(settings.volume_db, MIN_VOLUME_DB);
    }

    #[test]
    fn track_enabled_is_bounded() {
        let settings = AudioSettings {
            tracks: TRACK_MASK,
            ..AudioSettings::default()
        };
        assert!(settings.track_enabled(5));
        assert!(!settings.track_enabled(6), "there is no track 7");
    }

    #[test]
    fn output_bus_keys_are_stable() {
        assert_eq!(OutputBus::Master.key(), "master");
        assert_eq!(OutputBus::Track { index: 0 }.key(), "track1");
        assert_eq!(OutputBus::Track { index: 5 }.key(), "track6");
    }

    #[test]
    fn output_route_serializes_flat() {
        // The bus flattens onto the route object so the wire stays flat.
        let master = AudioOutputRoute {
            bus: OutputBus::Master,
            device_id: "Speakers".to_owned(),
            gain_db: -3.0,
        };
        let json = serde_json::to_value(&master).unwrap();
        assert_eq!(json["bus"], "master");
        assert_eq!(json["deviceId"], "Speakers");
        let track = AudioOutputRoute {
            bus: OutputBus::Track { index: 2 },
            device_id: String::new(),
            gain_db: 0.0,
        };
        let json = serde_json::to_value(&track).unwrap();
        assert_eq!(json["bus"], "track");
        assert_eq!(json["index"], 2);
        // Round-trips.
        let back: AudioOutputRoute = serde_json::from_value(json).unwrap();
        assert_eq!(back, track);
    }
}
