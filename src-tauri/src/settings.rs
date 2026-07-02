//! The JSON settings store — `settings.json` in the OS config dir.
//!
//! User configuration lives as plain JSON in the per-user config directory
//! (via `directories`), e.g. `%APPDATA%\Freally\Freally Capture\config\` on
//! Windows, `~/Library/Application Support/` on macOS, `~/.config/` on Linux.
//! Writes are atomic (temp file + rename) so a crash never truncates the
//! file. Stream keys are NOT stored here — they arrive in Phase 5 with their
//! own locally-scoped handling.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

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
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            language: "en".to_owned(),
            show_stats_dock: true,
        }
    }
}

/// Thread-safe handle to the settings file, managed as Tauri state.
pub struct SettingsStore {
    path: PathBuf,
    current: Mutex<Settings>,
}

impl SettingsStore {
    /// Open the store in the OS config dir, materializing the file with
    /// defaults on first run.
    pub fn load_default() -> Self {
        let dirs = ProjectDirs::from("com", "Freally", "Freally Capture")
            .expect("no home directory — cannot locate the OS config dir");
        Self::load_from(dirs.config_dir().join("settings.json"))
    }

    /// Open the store at an explicit path (missing file → defaults; corrupt
    /// file → defaults, with the corrupt content left in place until the next
    /// successful save overwrites it).
    pub fn load_from(path: PathBuf) -> Self {
        let current = read_settings(&path);
        let store = Self {
            path,
            current: Mutex::new(current),
        };
        if !store.path.exists() {
            // First run: write the defaults so the file is discoverable.
            if let Err(err) = store.persist() {
                eprintln!(
                    "settings: could not create {} ({err}); running with in-memory defaults",
                    store.path.display()
                );
            }
        }
        store
    }

    /// A snapshot of the current settings.
    pub fn get(&self) -> Settings {
        self.lock().clone()
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
        let json =
            serde_json::to_string_pretty(&*self.lock()).expect("settings always serialize to JSON");
        write_atomic(&self.path, &json)
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

/// Write via a sibling temp file + rename so the settings file is always
/// either the old or the new complete content, never a truncated mix.
fn write_atomic(path: &Path, content: &str) -> io::Result<()> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, content)?;
    fs::rename(&tmp, path)
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
    fn unknown_and_missing_keys_are_tolerated() {
        let path = temp_path("partial");
        fs::write(&path, r#"{ "language": "fr", "someFutureKey": 42 }"#).expect("write partial");
        let store = SettingsStore::load_from(path.clone());
        let settings = store.get();
        assert_eq!(settings.language, "fr");
        assert!(settings.show_stats_dock, "missing keys take their defaults");
        let _ = fs::remove_file(&path);
    }
}
