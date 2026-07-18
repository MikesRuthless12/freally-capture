//! CAP-N62: named version **snapshots** of a scene collection.
//!
//! One-click, intentionally-kept checkpoints of the active collection — "pre-
//! rework", "Show 12 layout" — with a browsable timeline, a diff view (reuses
//! the CAP-N61 [`crate::merge`] compare against a `Snapshot` target), and
//! restore. This is long-lived insurance that sits *beside* autosave and the
//! CAP-M01 undo stack: autosave keeps only the latest, undo is a within-session
//! stack; snapshots keep history on purpose. Strictly local.
//!
//! On-disk: `<config_dir>/snapshots/<collection>/<id>.json`, each file a
//! [`Snapshot`] (metadata + the full collection). The id is a sortable
//! timestamp we generate; it is validated on the way back in so a crafted id
//! can never escape the snapshots directory.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Runtime, State};

use fcap_scene::Collection;

use crate::profiles::WorkspaceState;
use crate::studio::StudioState;

/// Keep at most this many snapshots per collection; creating past the cap prunes
/// the oldest. Disk use is disclosed to the UI so the cap is honest.
const MAX_SNAPSHOTS: usize = 50;

/// The on-disk snapshot file.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Snapshot {
    id: String,
    name: String,
    created: String,
    collection: Collection,
}

/// One row in the snapshot timeline (no collection payload).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotMeta {
    pub id: String,
    pub name: String,
    pub created: String,
    pub scenes: usize,
    pub sources: usize,
    pub bytes: u64,
}

/// The `snapshot_list` payload: the timeline plus disclosed disk usage.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotList {
    /// Newest first.
    pub snapshots: Vec<SnapshotMeta>,
    pub total_bytes: u64,
    pub cap: usize,
}

/// A snapshot id is our own timestamp string: digits and dashes only. Reject
/// anything else so `id` can never contain a path separator or `..`.
fn valid_id(id: &str) -> bool {
    !id.is_empty() && id.len() <= 40 && id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
}

/// `<config_dir>/snapshots/<collection>`. The collection name is already
/// whitelisted by the profiles layer (1–40 chars, `[alnum space - _]`).
fn snapshot_dir(base: &Path, collection: &str) -> PathBuf {
    base.join("snapshots").join(collection)
}

/// Read every snapshot's metadata for a collection, newest first.
fn read_metas(dir: &Path) -> Vec<SnapshotMeta> {
    let mut metas: Vec<SnapshotMeta> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                return None;
            }
            let bytes = std::fs::metadata(&path).ok()?.len();
            let text = std::fs::read_to_string(&path).ok()?;
            let snapshot: Snapshot = serde_json::from_str(&text).ok()?;
            Some(SnapshotMeta {
                id: snapshot.id,
                name: snapshot.name,
                created: snapshot.created,
                scenes: snapshot.collection.scenes.len(),
                sources: snapshot.collection.sources.len(),
                bytes,
            })
        })
        .collect();
    // ids are sortable timestamps → reverse-lexicographic = newest first.
    metas.sort_by(|a, b| b.id.cmp(&a.id));
    metas
}

fn list_for(base: &Path, collection: &str) -> SnapshotList {
    let dir = snapshot_dir(base, collection);
    let snapshots = read_metas(&dir);
    let total_bytes = snapshots.iter().map(|m| m.bytes).sum();
    SnapshotList {
        snapshots,
        total_bytes,
        cap: MAX_SNAPSHOTS,
    }
}

/// Load a snapshot's collection (sanitized). Shared with [`crate::merge`] so a
/// snapshot can be a diff/merge target.
pub(crate) fn load_snapshot_collection(
    base: &Path,
    collection: &str,
    id: &str,
) -> Result<Collection, String> {
    if !valid_id(id) {
        return Err("that snapshot id is not valid".to_string());
    }
    let path = snapshot_dir(base, collection).join(format!("{id}.json"));
    let text =
        std::fs::read_to_string(&path).map_err(|err| format!("could not read snapshot: {err}"))?;
    let snapshot: Snapshot =
        serde_json::from_str(&text).map_err(|err| format!("bad snapshot file: {err}"))?;
    let mut collection = snapshot.collection;
    collection.sanitize();
    Ok(collection)
}

/// Write one snapshot (metadata + a clone of `graph`) and prune the oldest
/// beyond [`MAX_SNAPSHOTS`]. Shared by create, the before-restore safety net, and
/// the before-merge safety net (CAP-N61).
pub(crate) fn write_snapshot(
    base: &Path,
    collection: &str,
    name: &str,
    graph: &Collection,
) -> Result<(), String> {
    let dir = snapshot_dir(base, collection);
    std::fs::create_dir_all(&dir)
        .map_err(|err| format!("could not create {}: {err}", dir.display()))?;
    let now = chrono::Local::now();
    // The id is a millisecond timestamp; two snapshots in the same millisecond
    // (e.g. an auto "before restore/merge" plus a rapid manual one) would collide
    // and the second write would silently overwrite the first. Suffix on collision
    // so every intended checkpoint keeps its own file.
    let stamp = now.format("%Y%m%d-%H%M%S-%3f").to_string();
    let mut id = stamp.clone();
    let mut dup = 1;
    while dir.join(format!("{id}.json")).exists() {
        id = format!("{stamp}-{dup}");
        dup += 1;
    }
    let snapshot = Snapshot {
        id: id.clone(),
        name: name.to_string(),
        created: now.format("%Y-%m-%d %H:%M:%S").to_string(),
        collection: graph.clone(),
    };
    let json = serde_json::to_string_pretty(&snapshot).map_err(|err| err.to_string())?;
    crate::settings::write_atomic(&dir.join(format!("{id}.json")), &json)
        .map_err(|err| err.to_string())?;
    for stale in read_metas(&dir).iter().skip(MAX_SNAPSHOTS) {
        let _ = std::fs::remove_file(dir.join(format!("{}.json", stale.id)));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

/// Snapshot the active collection under `name` and return the refreshed list.
#[tauri::command]
pub fn snapshot_create<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, WorkspaceState>,
    name: String,
) -> Result<SnapshotList, String> {
    let name = name.trim();
    if name.is_empty() || name.chars().count() > 60 {
        return Err("a snapshot name needs 1–60 characters".to_string());
    }
    let base = state.base()?.to_path_buf();
    let collection_name = state.collection_name();
    let graph = app
        .state::<StudioState>()
        .with_collection(Collection::clone);
    write_snapshot(&base, &collection_name, name, &graph)?;
    Ok(list_for(&base, &collection_name))
}

/// The snapshot timeline for the active collection.
#[tauri::command]
pub fn snapshot_list(state: State<'_, WorkspaceState>) -> Result<SnapshotList, String> {
    let base = state.base()?.to_path_buf();
    Ok(list_for(&base, &state.collection_name()))
}

/// Restore a snapshot: it becomes the active collection (the current one is
/// snapshotted first as `auto: before restore` so a restore is never a one-way
/// door).
#[tauri::command]
pub fn snapshot_restore<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, WorkspaceState>,
    id: String,
) -> Result<SnapshotList, String> {
    let base = state.base()?.to_path_buf();
    let collection_name = state.collection_name();
    let restored = load_snapshot_collection(&base, &collection_name, &id)?;

    // Safety net: checkpoint the current state before overwriting it.
    let current = app
        .state::<StudioState>()
        .with_collection(Collection::clone);
    let _ = write_snapshot(&base, &collection_name, "before restore", &current);

    app.state::<StudioState>().mutate(&app, |collection| {
        *collection = restored;
        Ok(())
    })?;
    Ok(list_for(&base, &collection_name))
}

/// Delete a snapshot.
#[tauri::command]
pub fn snapshot_delete(
    state: State<'_, WorkspaceState>,
    id: String,
) -> Result<SnapshotList, String> {
    if !valid_id(&id) {
        return Err("that snapshot id is not valid".to_string());
    }
    let base = state.base()?.to_path_buf();
    let collection_name = state.collection_name();
    let path = snapshot_dir(&base, &collection_name).join(format!("{id}.json"));
    std::fs::remove_file(&path).map_err(|err| format!("could not delete snapshot: {err}"))?;
    Ok(list_for(&base, &collection_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_traversal_ids() {
        assert!(valid_id("20260718-113045-123"));
        assert!(!valid_id("../secret"));
        assert!(!valid_id("a/b"));
        assert!(!valid_id("a\\b"));
        assert!(!valid_id(".."));
        assert!(!valid_id(""));
    }
}
