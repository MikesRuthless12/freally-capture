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

/// Which profile + collection are active (persisted as `workspace.json`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Workspace {
    pub profile: String,
    pub collection: String,
}

impl Default for Workspace {
    fn default() -> Self {
        Workspace {
            profile: DEFAULT_NAME.to_string(),
            collection: DEFAULT_NAME.to_string(),
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

#[cfg(test)]
mod tests {
    use super::*;

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
