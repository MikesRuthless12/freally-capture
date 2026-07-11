//! Profiles + scene collections (TASK-506): switchable named snapshots.
//!
//! A **profile** is a named copy of the settings (`profiles/<name>.json`);
//! a **scene collection** is a named scene file (`collections/<name>.json`).
//! `workspace.json` remembers which of each is active. Switching saves the
//! current one first — nothing is ever silently lost. The live files
//! (`settings.json`, the active collection's file) stay the source of truth;
//! the studio's autosave keeps writing to whichever collection is active.
//!
//! Names arrive from the webview — UNTRUSTED. They are whitelisted to a safe
//! character set and bounded, so a name can never traverse paths.

use std::path::PathBuf;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Runtime, State};

use crate::settings::{write_atomic, Settings, SettingsStore};
use crate::studio::StudioState;

pub const DEFAULT_NAME: &str = "Default";

/// One open projector/aux window, remembered so it reopens next launch
/// (CAP-M07 extension). The `label` encodes the target
/// (`projector-program|preview`, `projector-scene:<id>`, `projector-source:<id>`,
/// `multiview`); scene/source ids are validated against the loaded collection
/// on reopen (stale ones are skipped).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectorState {
    pub label: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<usize>,
    #[serde(default)]
    pub fullscreen: bool,
}

/// Which profile + collection are active (persisted as `workspace.json`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Workspace {
    pub profile: String,
    pub collection: String,
    /// Open projector windows, reopened on launch (CAP-M07 extension).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub projectors: Vec<ProjectorState>,
}

impl Default for Workspace {
    fn default() -> Self {
        Workspace {
            profile: DEFAULT_NAME.to_string(),
            collection: DEFAULT_NAME.to_string(),
            projectors: Vec::new(),
        }
    }
}

/// Managed state: the active names + where the workspace lives.
pub struct WorkspaceState {
    dir: Option<PathBuf>,
    current: Mutex<Workspace>,
}

impl WorkspaceState {
    pub fn load_default() -> Self {
        let dir = directories::ProjectDirs::from("com", "Freally", "Freally Capture")
            .map(|dirs| dirs.config_dir().to_path_buf());
        let current = dir
            .as_ref()
            .and_then(|dir| std::fs::read_to_string(dir.join("workspace.json")).ok())
            .and_then(|text| serde_json::from_str::<Workspace>(&text).ok())
            .unwrap_or_default();
        WorkspaceState {
            dir,
            current: Mutex::new(current),
        }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Workspace> {
        self.current
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn save(&self) {
        let Some(dir) = &self.dir else { return };
        let json = serde_json::to_string_pretty(&*self.lock()).expect("workspace serializes");
        if let Err(err) = write_atomic(&dir.join("workspace.json"), &json) {
            eprintln!("workspace: could not save the active names: {err}");
        }
    }

    fn subdir(&self, kind: &str) -> Option<PathBuf> {
        let dir = self.dir.as_ref()?.join(kind);
        if let Err(err) = std::fs::create_dir_all(&dir) {
            eprintln!("workspace: could not create {}: {err}", dir.display());
            return None;
        }
        Some(dir)
    }

    fn base(&self) -> Result<&std::path::Path, String> {
        self.dir
            .as_deref()
            .ok_or_else(|| "no config directory".to_string())
    }

    /// Replace the remembered open-projector list and persist it (CAP-M07
    /// extension). Called at exit with the windows still open, so they reopen.
    pub fn set_projectors(&self, projectors: Vec<ProjectorState>) {
        self.lock().projectors = projectors;
        self.save();
    }

    /// The remembered open-projector list (for reopen on launch).
    pub fn projectors(&self) -> Vec<ProjectorState> {
        self.lock().projectors.clone()
    }
}

/// A named profile's on-disk file: the live `settings.json` for `"Default"`,
/// else `profiles/<name>.json`. Mirrors the collection mapping so `"Default"`
/// is always the live file and can never be "missing".
fn profile_file(config_dir: &std::path::Path, name: &str) -> PathBuf {
    if name == DEFAULT_NAME {
        config_dir.join("settings.json")
    } else {
        config_dir.join("profiles").join(format!("{name}.json"))
    }
}

fn profile_exists(config_dir: &std::path::Path, name: &str) -> bool {
    name == DEFAULT_NAME || profile_file(config_dir, name).is_file()
}

fn collection_exists(config_dir: &std::path::Path, name: &str) -> bool {
    name == DEFAULT_NAME || crate::studio::collection_file(config_dir, name).is_file()
}

/// Whitelist a user-supplied name: 1–40 chars of letters/digits/space/-/_.
/// Anything else is rejected — a name is a filename, never a path.
fn sanitize_name(name: &str) -> Result<String, String> {
    let name = name.trim();
    if name.is_empty() || name.len() > 40 {
        return Err("a name needs 1–40 characters".to_string());
    }
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == ' ' || c == '-' || c == '_')
    {
        return Err("names may use letters, digits, spaces, - and _".to_string());
    }
    Ok(name.to_string())
}

/// The names on disk for one kind (`profiles` / `collections`).
fn list_names(state: &WorkspaceState, kind: &str) -> Vec<String> {
    let Some(dir) = state.subdir(kind) else {
        return Vec::new();
    };
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            (path.extension().and_then(|ext| ext.to_str()) == Some("json"))
                .then(|| path.file_stem()?.to_str().map(str::to_owned))
                .flatten()
        })
        .collect();
    names.sort();
    names
}

/// The `profiles_list` / `collections_list` payload.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedList {
    pub active: String,
    pub names: Vec<String>,
}

fn with_default(active: &str, mut names: Vec<String>) -> NamedList {
    if !names.iter().any(|name| name == active) {
        names.push(active.to_string());
        names.sort();
    }
    NamedList {
        active: active.to_string(),
        names,
    }
}

// -- profiles -----------------------------------------------------------------

#[tauri::command]
pub fn profiles_list(state: State<'_, WorkspaceState>) -> NamedList {
    with_default(
        &state.lock().profile.clone(),
        list_names(&state, "profiles"),
    )
}

/// Save the current settings under a NEW `name` and make it active. Refuses
/// to clobber an existing profile.
#[tauri::command]
pub fn profile_create<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, WorkspaceState>,
    name: String,
) -> Result<NamedList, String> {
    let name = sanitize_name(&name)?;
    let base = state.base()?.to_path_buf();
    if profile_exists(&base, &name) {
        return Err(format!("a profile named {name:?} already exists"));
    }
    state.subdir("profiles"); // ensure the dir exists
    let settings = app.state::<SettingsStore>().get();
    let json = serde_json::to_string_pretty(&settings).map_err(|err| err.to_string())?;
    write_atomic(&profile_file(&base, &name), &json).map_err(|err| err.to_string())?;
    state.lock().profile = name;
    state.save();
    Ok(profiles_list(state))
}

/// Switch profiles: snapshot the active one, load the target, apply it.
#[tauri::command]
pub fn profile_switch<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, WorkspaceState>,
    name: String,
) -> Result<Settings, String> {
    let name = sanitize_name(&name)?;
    let base = state.base()?.to_path_buf();
    let store = app.state::<SettingsStore>();
    let current_name = state.lock().profile.clone();
    if name == current_name {
        return Ok(store.get());
    }
    if !profile_exists(&base, &name) {
        return Err(format!("profile {name:?} was not found"));
    }
    state.subdir("profiles"); // ensure the dir exists for the save-outgoing

    // Save the outgoing profile first — switching never loses edits.
    let current = store.get();
    let json = serde_json::to_string_pretty(&current).map_err(|err| err.to_string())?;
    write_atomic(&profile_file(&base, &current_name), &json).map_err(|err| err.to_string())?;

    let text = std::fs::read_to_string(profile_file(&base, &name))
        .map_err(|_| format!("profile {name:?} was not found"))?;
    let loaded: Settings = serde_json::from_str(&text)
        .map_err(|err| format!("profile {name:?} is not valid: {err}"))?;
    loaded.validate()?;
    store.set(loaded.clone()).map_err(|err| err.to_string())?;
    state.lock().profile = name;
    state.save();
    Ok(loaded)
}

// -- scene collections ---------------------------------------------------------

#[tauri::command]
pub fn collections_list(state: State<'_, WorkspaceState>) -> NamedList {
    with_default(
        &state.lock().collection.clone(),
        list_names(&state, "collections"),
    )
}

/// Duplicate the current scenes under a NEW `name` and switch to that copy.
/// Refuses to clobber an existing collection.
#[tauri::command]
pub fn collection_create<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, WorkspaceState>,
    name: String,
) -> Result<NamedList, String> {
    let name = sanitize_name(&name)?;
    let base = state.base()?.to_path_buf();
    if collection_exists(&base, &name) {
        return Err(format!("a scene collection named {name:?} already exists"));
    }
    state.subdir("collections"); // ensure the dir exists
    let current = state.lock().collection.clone();
    let save_as = crate::studio::collection_file(&base, &current);
    let load = crate::studio::collection_file(&base, &name);
    app.state::<StudioState>()
        .switch_collection_file(&app, save_as, load, true)?;
    state.lock().collection = name;
    state.save();
    Ok(collections_list(state))
}

/// Switch scene collections: the active one saves, the target loads.
#[tauri::command]
pub fn collection_switch<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, WorkspaceState>,
    name: String,
) -> Result<NamedList, String> {
    let name = sanitize_name(&name)?;
    let base = state.base()?.to_path_buf();
    let current = state.lock().collection.clone();
    if name == current {
        return Ok(collections_list(state));
    }
    // "Default" always loads (its file may not exist yet → an empty
    // collection); a named collection must exist on disk.
    if !collection_exists(&base, &name) {
        return Err(format!("scene collection {name:?} was not found"));
    }
    state.subdir("collections"); // ensure the dir exists for the save-outgoing
    let save_as = crate::studio::collection_file(&base, &current);
    let load = crate::studio::collection_file(&base, &name);
    app.state::<StudioState>()
        .switch_collection_file(&app, save_as, load, false)?;
    state.lock().collection = name;
    state.save();
    Ok(collections_list(state))
}

/// Turn an OBS collection name into a safe, unique collection name: keep only
/// whitelisted characters, cap the stem so a ` N` suffix still fits, fall back
/// to `"Imported"`, then bump a counter until it does not clash.
fn make_import_name(base: &std::path::Path, raw: &str) -> String {
    let cleaned: String = raw
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == ' ' || c == '-' || c == '_' {
                c
            } else {
                ' '
            }
        })
        .collect();
    // Truncate by CHARS, never bytes — `String::truncate` panics on a multi-byte
    // boundary, and the whitelist keeps non-ASCII alphanumerics (accents, CJK, …).
    let stem: String = cleaned
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .take(36)
        .collect();
    let stem = stem.trim();
    // Empty, or a Windows reserved device name (CON/NUL/COM1…/LPT1…), falls back
    // to a safe default — the atomic write to such a name would fail.
    let reserved = {
        let upper = stem.to_ascii_uppercase();
        matches!(upper.as_str(), "CON" | "PRN" | "AUX" | "NUL")
            || ((upper.starts_with("COM") || upper.starts_with("LPT"))
                && upper.len() == 4
                && upper.as_bytes()[3].is_ascii_digit())
    };
    let stem = if stem.is_empty() || reserved {
        "Imported"
    } else {
        stem
    };

    if stem != DEFAULT_NAME && !collection_exists(base, stem) {
        return stem.to_string();
    }
    for n in 2..1000 {
        let candidate = format!("{stem} {n}");
        if !collection_exists(base, &candidate) {
            return candidate;
        }
    }
    stem.to_string()
}

/// Import an OBS scene collection (`scenes.json`) as a new native collection and
/// switch to it. Returns the honest per-source [`fcap_scene::ImportReport`]; the
/// current collection is saved first (never silently lost), exactly like a
/// switch. `path` is a user-picked file — read, never written.
#[tauri::command]
pub fn collection_import_obs<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, WorkspaceState>,
    path: String,
) -> Result<fcap_scene::ImportReport, String> {
    // Bound the read: a scene collection is KB–MB; a multi-GB file is hostile
    // (it balloons several× as a serde_json::Value).
    const MAX_OBS_BYTES: u64 = 64 * 1024 * 1024;
    let size = std::fs::metadata(&path)
        .map_err(|err| format!("could not read {path:?}: {err}"))?
        .len();
    if size > MAX_OBS_BYTES {
        return Err(format!(
            "that file is too large to import ({} MB) — an OBS scene collection is normally a few KB",
            size / (1024 * 1024)
        ));
    }
    let text =
        std::fs::read_to_string(&path).map_err(|err| format!("could not read {path:?}: {err}"))?;
    let imported = fcap_scene::import_obs(&text).map_err(|err| err.to_string())?;

    let base = state.base()?.to_path_buf();
    state.subdir("collections"); // ensure the dir exists
    let name = make_import_name(&base, &imported.report.name);

    // Write the mapped collection to its own file, then switch to it (which
    // saves the outgoing collection to its own file first).
    let load = crate::studio::collection_file(&base, &name);
    let json = serde_json::to_string_pretty(&imported.collection).map_err(|err| err.to_string())?;
    write_atomic(&load, &json).map_err(|err| err.to_string())?;

    let current = state.lock().collection.clone();
    let save_as = crate::studio::collection_file(&base, &current);
    app.state::<StudioState>()
        .switch_collection_file(&app, save_as, load, false)?;
    state.lock().collection = name;
    state.save();

    Ok(imported.report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn import_names_are_cleaned_and_bounded() {
        let dir = std::path::Path::new("/nonexistent-cfg");
        assert_eq!(make_import_name(dir, "My Stream"), "My Stream");
        // Illegal characters become spaces and collapse.
        assert_eq!(make_import_name(dir, "../my:stream?"), "my stream");
        // Empty / all-illegal falls back.
        assert_eq!(make_import_name(dir, "***"), "Imported");
        assert_eq!(make_import_name(dir, ""), "Imported");
        // The stem is bounded so a counter suffix still fits in 40 chars.
        assert!(make_import_name(dir, &"x".repeat(80)).len() <= 40);
        // A multi-byte name that would straddle the truncation boundary must
        // truncate by chars, not bytes — never panic (security review).
        assert!(!make_import_name(dir, &"é".repeat(40)).is_empty());
        // Windows reserved device names fall back instead of naming a device.
        assert_eq!(make_import_name(dir, "CON"), "Imported");
        assert_eq!(make_import_name(dir, "com1"), "Imported");
    }

    #[test]
    fn names_are_whitelisted() {
        assert_eq!(sanitize_name(" My Show 2 ").unwrap(), "My Show 2");
        for bad in [
            "",
            "  ",
            "../escape",
            "a/b",
            "a\\b",
            "x".repeat(41).as_str(),
            "dot.dot",
        ] {
            assert!(sanitize_name(bad).is_err(), "should reject {bad:?}");
        }
    }

    #[test]
    fn default_maps_to_the_live_files_named_maps_to_the_subdir() {
        let dir = std::path::Path::new("/cfg");
        // "Default" is always the live file — it can never be "missing".
        assert_eq!(profile_file(dir, DEFAULT_NAME), dir.join("settings.json"));
        assert_eq!(
            crate::studio::collection_file(dir, DEFAULT_NAME),
            dir.join("scene-collection.json")
        );
        assert!(profile_exists(dir, DEFAULT_NAME));
        assert!(collection_exists(dir, DEFAULT_NAME));
        // A named snapshot lives under its own subdir.
        assert_eq!(
            profile_file(dir, "Show"),
            dir.join("profiles").join("Show.json")
        );
        assert_eq!(
            crate::studio::collection_file(dir, "Show"),
            dir.join("collections").join("Show.json")
        );
    }
}
