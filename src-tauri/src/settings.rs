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

/// User-facing settings. Every field defaults (`serde(default)`) so missing
/// keys never brick the app, and unknown keys from newer builds are ignored.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Settings {
    /// UI language (BCP-47). The language picker + full 18-language i18n land
    /// in Phase 9; the field exists from day one for forward compatibility.
    pub language: String,
    /// Whether the stats dock is shown.
    pub show_stats_dock: bool,
    /// The audio monitor output device name (`None`/empty = the OS default).
    pub monitor_device: Option<String>,
    /// Audio Mixer strip orientation.
    pub mixer_layout: MixerLayout,
    /// Recording output configuration (Phase 4).
    pub recording: RecordingSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            language: "en".to_owned(),
            show_stats_dock: true,
            monitor_device: None,
            mixer_layout: MixerLayout::default(),
            recording: RecordingSettings::default(),
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
    /// Filename prefix; the timestamp is appended.
    pub filename_prefix: String,
    /// Split into playable segments every N minutes (0 = off).
    pub split_minutes: u32,
}

impl Default for RecordingSettings {
    fn default() -> Self {
        Self {
            container: Container::Frec,
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
            split_minutes: 0,
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
        self.recording.validate()?;
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

    /// Replace the settings and persist them atomically.
    pub fn set(&self, next: Settings) -> io::Result<()> {
        {
            let mut guard = self.lock();
            *guard = next;
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
    match fs::read_to_string(path) {
        Ok(text) => match serde_json::from_str(&text) {
            Ok(settings) => settings,
            Err(err) => {
                eprintln!(
                    "settings: {} is not valid settings JSON ({err}); using defaults",
                    path.display()
                );
                Settings::default()
            }
        },
        Err(err) if err.kind() == io::ErrorKind::NotFound => Settings::default(),
        Err(err) => {
            eprintln!(
                "settings: cannot read {} ({err}); using defaults",
                path.display()
            );
            Settings::default()
        }
    }
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
            mixer_layout: MixerLayout::Vertical,
            recording: RecordingSettings {
                container: Container::Mkv,
                split_minutes: 30,
                ..RecordingSettings::default()
            },
        };
        store.set(next.clone()).expect("save settings");

        let reloaded = SettingsStore::load_from(path.clone());
        assert_eq!(reloaded.get(), next);
        let _ = fs::remove_file(&path);
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
        ];
        for (why, bad) in cases {
            assert!(bad.validate().is_err(), "should reject: {why}");
        }
    }
}
