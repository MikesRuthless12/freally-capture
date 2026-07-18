//! CAP-N64: one-file **backup & restore** (a whole-studio archive + migration).
//!
//! Export *everything* that makes this install yours — settings, profiles,
//! scene collections, the active-workspace pointer — to a single versioned
//! `.fcapbackup` zip, and restore it (selectively) on the same or a new
//! machine. Strictly local; nothing is sent anywhere.
//!
//! **Secrets are stripped by default.** Stream keys, the TURN credential, and
//! the LAN-panel / remote-control passwords are blanked in the archived
//! settings, so a backup you email or drop in cloud storage never carries a
//! live secret. (The roadmap's "include stream keys under an encrypted opt-in"
//! is intentionally deferred rather than shipped with home-rolled crypto or a
//! new AEAD dependency — see PHASE-8-PROGRESS; the safe default is to exclude
//! them, and the operator re-enters keys after a restore.) On restore,
//! [`crate::settings::SettingsStore::set`] preserves this machine's server-owned
//! fields (EULA acceptance, onboarding, the recording counter, camera profiles),
//! so restoring a backup never re-prompts the EULA or loses local calibration.

use std::io::{Read, Seek, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::{AppHandle, Manager, Runtime, State};

use crate::profiles::WorkspaceState;
use crate::settings::{Settings, SettingsStore};

const BACKUP_FORMAT: u32 = 1;
const MANIFEST_ENTRY: &str = "backup-manifest.json";

/// Refuse a backup file larger than this (decompression-bomb guard); a studio
/// config is KB–low-MB.
const MAX_BACKUP_BYTES: u64 = 512 * 1024 * 1024;

/// The one source of truth for where secrets live in the settings JSON (Pointer,
/// camelCase). Both [`strip_secrets`] (blank them for an exported archive) and
/// [`graft_secrets`] (keep *this* machine's on restore) consult these, so the two
/// can never drift apart. Kept in step with what `diagnostics.rs` treats as
/// sensitive — including the TURN username, which pairs with its credential.
///
/// `SECRET_ARRAYS` are per-item arrays (each object's listed keys are secret);
/// `SECRET_SCALARS` are single string values.
const SECRET_ARRAYS: &[(&str, &[&str])] = &[
    ("/stream/targets", &["streamKey", "ingestUrl"]),
    ("/browserDocks", &["url"]),
];
const SECRET_SCALARS: &[&str] = &[
    "/remote/turnCredential",
    "/remote/turnUsername",
    "/remoteControl/password",
    "/webPanel/password",
];

/// The archive's self-description.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupManifest {
    #[serde(default)]
    pub backup_format: u32,
    pub app_version: String,
    pub created: String,
    pub has_settings: bool,
    pub collections: Vec<String>,
    pub profiles: Vec<String>,
    /// Always false in this build (secrets are stripped) — reserved.
    #[serde(default)]
    pub stream_keys_included: bool,
}

/// Returned to the UI after an export.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupReport {
    pub path: String,
    pub settings: bool,
    pub collections: usize,
    pub profiles: usize,
    pub total_bytes: u64,
}

/// What to pull in on restore (the selective / migration picker).
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RestoreSelection {
    pub settings: bool,
    pub collections: bool,
    pub profiles: bool,
}

/// Returned after a restore.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestoreReport {
    pub settings: bool,
    pub collections: usize,
    pub profiles: usize,
    /// Collections/profiles are read at launch — a restart loads the restored
    /// ones. Settings apply live.
    pub restart_recommended: bool,
}

/// Blank every secret in the settings JSON so a backup never carries one. Both
/// the stream key AND the ingest URL are blanked (an ingest URL can embed the
/// key), plus the TURN credential + username, the LAN-panel / remote-control
/// passwords, and browser-dock URLs (which can carry an auth token) — mirroring
/// what the diagnostics bundle treats as sensitive.
fn strip_secrets(mut settings: Value) -> Value {
    for (array_pointer, keys) in SECRET_ARRAYS {
        if let Some(items) = settings
            .pointer_mut(array_pointer)
            .and_then(Value::as_array_mut)
        {
            for item in items {
                if let Some(object) = item.as_object_mut() {
                    for key in *keys {
                        object.insert((*key).to_string(), Value::String(String::new()));
                    }
                }
            }
        }
    }
    for pointer in SECRET_SCALARS {
        if let Some(slot) = settings.pointer_mut(pointer) {
            *slot = Value::String(String::new());
        }
    }
    settings
}

/// A backup never carries secrets (they were blanked on export), so restoring its
/// settings must not overwrite the machine's live keys/passwords with the blanks.
/// Copy every secret value from `current` (this machine's settings) into the
/// incoming `restored` at the exact same locations, leaving all non-secret
/// preferences to come from the backup. Arrays are matched by index (same-machine
/// restore aligns; a different machine simply keeps its own values). This is the
/// inverse of [`strip_secrets`] over the same field list.
fn graft_secrets(mut restored: Value, current: &Value) -> Value {
    for (array_pointer, keys) in SECRET_ARRAYS {
        let Some(current_items) = current.pointer(array_pointer).and_then(Value::as_array) else {
            continue;
        };
        if let Some(items) = restored
            .pointer_mut(array_pointer)
            .and_then(Value::as_array_mut)
        {
            for (item, current_item) in items.iter_mut().zip(current_items) {
                if let (Some(object), Some(current_object)) =
                    (item.as_object_mut(), current_item.as_object())
                {
                    for key in *keys {
                        if let Some(value) = current_object.get(*key) {
                            object.insert((*key).to_string(), value.clone());
                        }
                    }
                }
            }
        }
    }
    for pointer in SECRET_SCALARS {
        if let Some(value) = current.pointer(pointer).cloned() {
            if let Some(slot) = restored.pointer_mut(pointer) {
                *slot = value;
            }
        }
    }
    restored
}

/// The `.json` file stems in a config subdir (`collections` / `profiles`).
fn json_stems(dir: &Path) -> Vec<String> {
    std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            (path.extension().and_then(|e| e.to_str()) == Some("json"))
                .then(|| path.file_stem()?.to_str().map(str::to_owned))
                .flatten()
        })
        .collect()
}

fn add_file<W: Write + Seek>(
    zip: &mut zip::ZipWriter<W>,
    options: zip::write::SimpleFileOptions,
    name: &str,
    bytes: &[u8],
) -> Result<(), String> {
    zip.start_file(name, options)
        .map_err(|err| format!("zip: {err}"))?;
    zip.write_all(bytes).map_err(|err| format!("zip: {err}"))?;
    Ok(())
}

/// Build the backup zip from `config_dir` into `writer`.
fn write_backup<W: Write + Seek>(
    writer: W,
    config_dir: &Path,
    created: &str,
) -> Result<BackupReport, String> {
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);
    let mut zip = zip::ZipWriter::new(writer);
    let mut total_bytes: u64 = 0;
    let mut had_settings = false;

    // settings.json — with all secrets stripped.
    let settings_path = config_dir.join("settings.json");
    if let Ok(text) = std::fs::read_to_string(&settings_path) {
        if let Ok(value) = serde_json::from_str::<Value>(&text) {
            let stripped = serde_json::to_vec_pretty(&strip_secrets(value))
                .map_err(|err| format!("serialize: {err}"))?;
            add_file(&mut zip, options, "settings.json", &stripped)?;
            total_bytes += stripped.len() as u64;
            had_settings = true;
        }
    }

    // The Default scene collection + every named one.
    let mut collections = Vec::new();
    let default_collection = config_dir.join("scene-collection.json");
    if let Ok(bytes) = std::fs::read(&default_collection) {
        add_file(&mut zip, options, "scene-collection.json", &bytes)?;
        total_bytes += bytes.len() as u64;
        collections.push("Default".to_string());
    }
    let collections_dir = config_dir.join("collections");
    for name in json_stems(&collections_dir) {
        if let Ok(bytes) = std::fs::read(collections_dir.join(format!("{name}.json"))) {
            add_file(
                &mut zip,
                options,
                &format!("collections/{name}.json"),
                &bytes,
            )?;
            total_bytes += bytes.len() as u64;
            collections.push(name);
        }
    }

    // Named profiles (the Default profile IS settings.json, already added).
    let mut profiles = Vec::new();
    let profiles_dir = config_dir.join("profiles");
    for name in json_stems(&profiles_dir) {
        if let Ok(bytes) = std::fs::read(profiles_dir.join(format!("{name}.json"))) {
            // A profile file is a full Settings snapshot → strip its secrets too.
            if let Ok(value) = serde_json::from_slice::<Value>(&bytes) {
                let stripped = serde_json::to_vec_pretty(&strip_secrets(value))
                    .map_err(|err| format!("serialize: {err}"))?;
                add_file(
                    &mut zip,
                    options,
                    &format!("profiles/{name}.json"),
                    &stripped,
                )?;
                total_bytes += stripped.len() as u64;
                profiles.push(name);
            }
        }
    }

    // The active-workspace pointer (which profile/collection is live).
    if let Ok(bytes) = std::fs::read(config_dir.join("workspace.json")) {
        add_file(&mut zip, options, "workspace.json", &bytes)?;
        total_bytes += bytes.len() as u64;
    }

    let manifest = BackupManifest {
        backup_format: BACKUP_FORMAT,
        app_version: env!("CARGO_PKG_VERSION").to_string(),
        created: created.to_string(),
        has_settings: had_settings,
        collections: collections.clone(),
        profiles: profiles.clone(),
        stream_keys_included: false,
    };
    let manifest_json =
        serde_json::to_vec_pretty(&manifest).map_err(|err| format!("serialize: {err}"))?;
    add_file(&mut zip, options, MANIFEST_ENTRY, &manifest_json)?;
    zip.finish().map_err(|err| format!("zip: {err}"))?;

    Ok(BackupReport {
        path: String::new(),
        settings: had_settings,
        collections: collections.len(),
        profiles: profiles.len(),
        total_bytes,
    })
}

/// A zip entry name is safe to extract under the config dir: a known top-level
/// file, or a single-segment `collections/`/`profiles/` `.json`. Rejects any
/// `..`, absolute path, or nested path — a crafted archive can never escape.
fn safe_target(config_dir: &Path, entry: &str, selection: &RestoreSelection) -> Option<PathBuf> {
    if entry.contains("..") || entry.starts_with('/') || entry.starts_with('\\') {
        return None;
    }
    match entry {
        "settings.json" | "workspace.json" | "scene-collection.json" => {
            let want = if entry == "settings.json" {
                selection.settings
            } else {
                selection.collections
            };
            want.then(|| config_dir.join(entry))
        }
        other => {
            let (dir, name, want) = if let Some(name) = other.strip_prefix("collections/") {
                ("collections", name, selection.collections)
            } else if let Some(name) = other.strip_prefix("profiles/") {
                ("profiles", name, selection.profiles)
            } else {
                return None;
            };
            // exactly `<sub>/<stem>.json`, no further separators. Also reject any
            // `:` — on Windows a name like `C:foo.json` is drive-relative (joins
            // to the CWD, not `config_dir`) and `foo:bar.json` names an NTFS
            // alternate data stream; neither is a plain child file.
            if !want
                || name.contains('/')
                || name.contains('\\')
                || name.contains(':')
                || !name.ends_with(".json")
            {
                return None;
            }
            Some(config_dir.join(dir).join(name))
        }
    }
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

/// Export the whole studio config to `dest` as a `.fcapbackup` (secrets
/// stripped). Runs off the UI thread.
#[tauri::command]
pub async fn backup_export<R: Runtime>(
    _app: AppHandle<R>,
    dest: String,
) -> Result<BackupReport, String> {
    let config_dir = crate::paths::config_dir().ok_or("no config directory to back up")?;
    let created = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    tauri::async_runtime::spawn_blocking(move || {
        let file = std::fs::File::create(&dest)
            .map_err(|err| format!("could not create {dest}: {err}"))?;
        let mut report = write_backup(std::io::BufWriter::new(file), &config_dir, &created)?;
        report.path = dest.clone();
        println!(
            "backup: exported → {} ({} collections, {} profiles)",
            dest, report.collections, report.profiles
        );
        Ok(report)
    })
    .await
    .map_err(|err| format!("backup task failed: {err}"))?
}

/// Read a backup's manifest (for the restore picker's preview).
#[tauri::command]
pub fn backup_inspect(path: String) -> Result<BackupManifest, String> {
    let file =
        std::fs::File::open(&path).map_err(|err| format!("could not open {path:?}: {err}"))?;
    let mut archive = zip::ZipArchive::new(std::io::BufReader::new(file))
        .map_err(|err| format!("not a backup: {err}"))?;
    let entry = archive
        .by_name(MANIFEST_ENTRY)
        .map_err(|_| "this file is not a Freally Capture backup".to_string())?;
    let mut text = String::new();
    entry
        .take(MAX_BACKUP_BYTES)
        .read_to_string(&mut text)
        .map_err(|err| format!("read manifest: {err}"))?;
    serde_json::from_str(&text).map_err(|err| format!("bad backup manifest: {err}"))
}

/// Restore the selected parts of a backup onto this machine. Settings apply
/// live (server-owned fields preserved); collections/profiles are written to
/// disk and load on the next launch.
#[tauri::command]
pub fn backup_restore<R: Runtime>(
    app: AppHandle<R>,
    _state: State<'_, WorkspaceState>,
    path: String,
    selection: RestoreSelection,
) -> Result<RestoreReport, String> {
    let size = std::fs::metadata(&path)
        .map_err(|err| format!("could not read {path:?}: {err}"))?
        .len();
    if size > MAX_BACKUP_BYTES {
        return Err(format!(
            "that backup is too large ({} MB)",
            size / (1024 * 1024)
        ));
    }
    let config_dir = crate::paths::config_dir().ok_or("no config directory to restore into")?;
    let file =
        std::fs::File::open(&path).map_err(|err| format!("could not open {path:?}: {err}"))?;
    let mut archive = zip::ZipArchive::new(std::io::BufReader::new(file))
        .map_err(|err| format!("not a backup: {err}"))?;

    let mut collections = 0usize;
    let mut profiles = 0usize;
    let mut restored_settings = false;
    let mut restored_collection_paths: Vec<PathBuf> = Vec::new();

    for i in 0..archive.len() {
        let entry = archive.by_index(i).map_err(|err| format!("zip: {err}"))?;
        let name = entry.name().to_string();
        if name == MANIFEST_ENTRY {
            continue;
        }
        let Some(target) = safe_target(&config_dir, &name, &selection) else {
            continue;
        };
        let mut bytes = Vec::new();
        entry
            .take(MAX_BACKUP_BYTES)
            .read_to_end(&mut bytes)
            .map_err(|err| format!("read {name}: {err}"))?;

        if name == "settings.json" {
            // A backup never carries secrets (they were blanked on export), so
            // keep this machine's live keys/passwords and take only the
            // non-secret preferences from the archive. Validate before applying
            // (a hand-edited archive must not persist an invalid settings file),
            // then persist through the store so server-owned fields (EULA,
            // onboarding, counter, camera profiles) are preserved. `set` does not
            // emit, so the running UI re-reads settings via `settingsGet` once the
            // restore returns (see BackupDialog) — that is what makes it live.
            let restored_value: Value = serde_json::from_slice(&bytes)
                .map_err(|err| format!("bad settings in backup: {err}"))?;
            let store = app.state::<SettingsStore>();
            let current_value = serde_json::to_value(store.get())
                .map_err(|err| format!("could not read current settings: {err}"))?;
            let settings: Settings =
                serde_json::from_value(graft_secrets(restored_value, &current_value))
                    .map_err(|err| format!("bad settings in backup: {err}"))?;
            settings.validate()?;
            store
                .set(settings)
                .map_err(|err| format!("could not apply restored settings: {err}"))?;
            restored_settings = true;
            continue;
        }

        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|err| format!("could not create {}: {err}", parent.display()))?;
        }
        crate::settings::write_atomic(&target, &String::from_utf8_lossy(&bytes))
            .map_err(|err| err.to_string())?;
        if name.starts_with("collections/") || name == "scene-collection.json" {
            collections += 1;
            restored_collection_paths.push(target.clone());
        } else if name.starts_with("profiles/") {
            profiles += 1;
        }
    }

    // If the *active* collection's file was just overwritten, the in-memory
    // studio still holds the old scenes and its ~5s autosave (and the exit-time
    // save) would clobber the restore straight back to disk. Reload the active
    // collection from disk so the restored scenes are live immediately and can't
    // be overwritten; other collections + profiles still load on the next launch.
    let studio = app.state::<crate::studio::StudioState>();
    let active_reloaded = studio
        .active_path()
        .is_some_and(|active| restored_collection_paths.contains(&active));
    if active_reloaded {
        studio.reload_active(&app);
    }

    println!(
        "backup: restored settings={restored_settings} collections={collections} profiles={profiles} active_reloaded={active_reloaded}"
    );
    Ok(RestoreReport {
        settings: restored_settings,
        collections,
        profiles,
        // The active collection (if it was restored) is now live; only *other*
        // collections and any profiles still need a restart to take effect.
        restart_recommended: profiles > 0 || collections > usize::from(active_reloaded),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn sample_secrets() -> serde_json::Value {
        serde_json::json!({
            "stream": { "targets": [ { "streamKey": "live_SECRET", "ingestUrl": "rtmp://x/app/live_URLKEY", "bitrateKbps": 6000 } ] },
            "remote": { "turnCredential": "turnpass", "turnUsername": "turnuser77" },
            "remoteControl": { "password": "deckpass" },
            "webPanel": { "password": "panelpass" },
            "browserDocks": [ { "name": "Chat", "url": "https://chat.example/?token=DOCKTOKEN" } ],
            "language": "en"
        })
    }

    #[test]
    fn strips_every_secret() {
        let stripped = strip_secrets(sample_secrets());
        let text = serde_json::to_string(&stripped).unwrap();
        for secret in [
            "live_SECRET",
            "URLKEY",
            "turnpass",
            "turnuser77",
            "deckpass",
            "panelpass",
            "DOCKTOKEN",
        ] {
            assert!(!text.contains(secret), "leaked {secret}");
        }
        // non-secrets survive
        assert!(text.contains("6000"));
        assert!(text.contains("\"en\""));
    }

    #[test]
    fn graft_keeps_this_machines_secrets_on_restore() {
        // A restored archive has blank secrets but the user's chosen non-secrets
        // (a different language); grafting must re-fill every secret from the live
        // settings while leaving the restored preferences untouched.
        let live = sample_secrets();
        let restored = {
            let mut r = strip_secrets(sample_secrets());
            r["language"] = serde_json::json!("fr");
            r
        };
        let merged = graft_secrets(restored, &live);
        let text = serde_json::to_string(&merged).unwrap();
        for secret in [
            "live_SECRET",
            "URLKEY",
            "turnpass",
            "turnuser77",
            "deckpass",
            "panelpass",
            "DOCKTOKEN",
        ] {
            assert!(text.contains(secret), "dropped live secret {secret}");
        }
        // the restored (non-secret) preference wins
        assert!(text.contains("\"fr\""));
    }

    #[test]
    fn safe_target_blocks_traversal_and_honors_selection() {
        let base = Path::new("C:/cfg");
        let all = RestoreSelection {
            settings: true,
            collections: true,
            profiles: true,
        };
        assert!(safe_target(base, "../evil.json", &all).is_none());
        assert!(safe_target(base, "collections/../../evil.json", &all).is_none());
        // Windows drive-relative + NTFS alternate-data-stream names are rejected.
        assert!(safe_target(base, "collections/C:evil.json", &all).is_none());
        assert!(safe_target(base, "collections/Show:evil.json", &all).is_none());
        assert!(safe_target(base, "collections/Show.json", &all).is_some());
        assert!(safe_target(base, "settings.json", &all).is_some());
        // selection gates categories
        let only_profiles = RestoreSelection {
            settings: false,
            collections: false,
            profiles: true,
        };
        assert!(safe_target(base, "settings.json", &only_profiles).is_none());
        assert!(safe_target(base, "collections/Show.json", &only_profiles).is_none());
        assert!(safe_target(base, "profiles/Podcast.json", &only_profiles).is_some());
    }

    #[test]
    fn backup_roundtrips_a_collection() {
        // A tiny config dir with one named collection.
        let dir = std::env::temp_dir().join(format!("fcap-backup-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("collections")).unwrap();
        std::fs::write(dir.join("settings.json"), r#"{"language":"fr"}"#).unwrap();
        std::fs::write(dir.join("collections/Show.json"), r#"{"scenes":[]}"#).unwrap();

        let mut buf = Cursor::new(Vec::new());
        let report = write_backup(&mut buf, &dir, "t").unwrap();
        assert!(report.settings);
        assert_eq!(report.collections, 1);

        buf.set_position(0);
        let mut archive = zip::ZipArchive::new(buf).unwrap();
        assert!(archive.by_name("settings.json").is_ok());
        assert!(archive.by_name("collections/Show.json").is_ok());
        assert!(archive.by_name(MANIFEST_ENTRY).is_ok());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
