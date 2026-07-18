//! CAP-N61: scene-collection **diff & merge** commands.
//!
//! Compare the live (active) collection against another one — a named
//! collection on disk, or the collection inside a `.fcappack` (CAP-N60) — and
//! return a [`fcap_scene::CollectionDiff`]. The operator cherry-picks which
//! scenes/sources to take, and [`collection_merge`] applies just those onto the
//! active collection. Everything is local; nothing is sent anywhere.

use std::io::BufReader;
use std::path::Path;

use serde::Deserialize;
use tauri::{AppHandle, Manager, Runtime, State};

use fcap_scene::{Collection, CollectionDiff, SceneId, SourceId};

use crate::profiles::WorkspaceState;
use crate::studio::StudioState;

/// What to compare the active collection against.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DiffTarget {
    /// A named scene collection on disk (`"Default"` = the live file).
    Collection { name: String },
    /// A `.fcappack` file — compared against the collection it carries.
    Pack { path: String },
    /// A CAP-N62 named snapshot of the *active* collection.
    Snapshot { id: String },
}

/// Load the "other" side of the comparison. A named collection is read from its
/// file (missing/corrupt → an empty collection, exactly as a switch would see
/// it); a pack via the shared [`crate::pack::read_pack`]; a snapshot via
/// [`crate::snapshot`] (keyed to the active collection).
fn load_other(
    base_dir: &Path,
    active_collection: &str,
    target: &DiffTarget,
) -> Result<Collection, String> {
    match target {
        DiffTarget::Collection { name } => {
            let path = crate::studio::collection_file(base_dir, name);
            Ok(crate::studio::read_collection(&path))
        }
        DiffTarget::Pack { path } => {
            let file = std::fs::File::open(path)
                .map_err(|err| format!("could not open {path:?}: {err}"))?;
            let mut collection = crate::pack::read_pack(BufReader::new(file))?.collection;
            // A pack's collection is parsed but not sanitized on read; do it here
            // so the diff compares two valid collections.
            collection.sanitize();
            Ok(collection)
        }
        DiffTarget::Snapshot { id } => {
            crate::snapshot::load_snapshot_collection(base_dir, active_collection, id)
        }
    }
}

/// Diff the active collection against `target` (what would change if the active
/// collection became `target`).
#[tauri::command]
pub fn collection_diff<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, WorkspaceState>,
    target: DiffTarget,
) -> Result<CollectionDiff, String> {
    let base = app
        .state::<StudioState>()
        .with_collection(Collection::clone);
    let base_dir = state.base()?.to_path_buf();
    let other = load_other(&base_dir, &state.collection_name(), &target)?;
    Ok(fcap_scene::diff_collections(&base, &other))
}

/// Apply the selected scene/source changes from `target` onto the active
/// collection, then return the diff that remains (empty if everything was
/// taken). Selecting a scene also pulls in the sources it references.
#[tauri::command]
pub fn collection_merge<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, WorkspaceState>,
    target: DiffTarget,
    scenes: Vec<SceneId>,
    sources: Vec<SourceId>,
) -> Result<CollectionDiff, String> {
    let base_dir = state.base()?.to_path_buf();
    let collection_name = state.collection_name();
    let other = load_other(&base_dir, &collection_name, &target)?;
    let studio = app.state::<StudioState>();
    // Safety net (mirrors snapshot_restore): a merge overwrites the whole active
    // collection through `mutate`, which is off the Ctrl+Z stack, so checkpoint
    // the current state first as `before merge` — an unwanted merge is never a
    // one-way door.
    let before = studio.with_collection(Collection::clone);
    let _ = crate::snapshot::write_snapshot(&base_dir, &collection_name, "before merge", &before);
    studio.mutate(&app, |collection| {
        *collection = fcap_scene::merge_selected(collection, &other, &scenes, &sources);
        Ok(())
    })?;
    let base = studio.with_collection(Collection::clone);
    Ok(fcap_scene::diff_collections(&base, &other))
}
