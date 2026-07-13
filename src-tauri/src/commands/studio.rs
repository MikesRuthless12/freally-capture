//! The studio command surface: every scene/source/item/filter mutation the
//! UI (and later the remote API / scripting) drives.
//!
//! Each command validates against the model (`fcap-scene` returns an error
//! and mutates nothing on a bad target), bumps the revision, and pushes the
//! fresh model on the `studio` event — the UI never has to guess.
//! (JS sends camelCase argument names; Tauri maps them onto these
//! snake_case parameters, same as the Phase 1 commands.)

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Runtime, State};

use fcap_scene::{
    BlendMode, Corner, FileRef, FileRefKind, Filter, FilterId, FilterKind, GuideLine, ItemId,
    NormRect, SceneId, Source, SourceId, SourceSettings, Transform,
};

use crate::studio::{coalesce_key, StillTarget, StudioDto, StudioState, WorkbenchMode};

/// The whole current model (initial load / reconnect).
#[tauri::command]
pub fn studio_get(state: State<'_, StudioState>) -> StudioDto {
    state.snapshot()
}

// -- undo / redo (CAP-M01) --------------------------------------------------

/// Undo the newest scene edit. Returns the reversed edit's label (a stable key
/// the UI localizes), or `null` when there was nothing to undo. The restored
/// model arrives on the `studio` event like any mutation.
#[tauri::command]
pub fn studio_undo(app: AppHandle, state: State<'_, StudioState>) -> Option<String> {
    state.undo(&app)
}

/// Redo the most recently undone scene edit. Mirror of [`studio_undo`].
#[tauri::command]
pub fn studio_redo(app: AppHandle, state: State<'_, StudioState>) -> Option<String> {
    state.redo(&app)
}

// -- scenes -----------------------------------------------------------------

#[tauri::command]
pub fn studio_add_scene(
    app: AppHandle,
    state: State<'_, StudioState>,
    name: String,
) -> Result<SceneId, String> {
    state.mutate_tracked(&app, "addScene", None, |collection| {
        Ok(collection.add_scene(&name))
    })
}

#[tauri::command]
pub fn studio_rename_scene(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    name: String,
) -> Result<(), String> {
    state.mutate_tracked(&app, "renameScene", None, |collection| {
        collection.rename_scene(scene_id, &name)
    })
}

#[tauri::command]
pub fn studio_remove_scene(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
) -> Result<(), String> {
    state.mutate_tracked(&app, "removeScene", None, |collection| {
        collection.remove_scene(scene_id)
    })
}

#[tauri::command]
pub fn studio_select_scene(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
) -> Result<(), String> {
    state.mutate(&app, |collection| collection.set_active_scene(scene_id))
}

#[tauri::command]
pub fn studio_reorder_scene(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    to_index: usize,
) -> Result<(), String> {
    state.mutate_tracked(&app, "reorderScene", None, |collection| {
        collection.reorder_scene(scene_id, to_index)
    })
}

// -- items ------------------------------------------------------------------

/// What `studio_add_item` created.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddedItem {
    pub source_id: SourceId,
    pub item_id: ItemId,
}

/// Add a brand-new source (name optional — defaults per kind) on top of a scene.
#[tauri::command]
pub fn studio_add_item(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    name: Option<String>,
    settings: SourceSettings,
) -> Result<AddedItem, String> {
    state.mutate_tracked(&app, "addSource", None, |collection| {
        let source = Source::new(name.unwrap_or_default(), settings);
        collection
            .add_item_with_new_source(scene_id, source)
            .map(|(source_id, item_id)| AddedItem { source_id, item_id })
    })
}

/// Place an existing pool source on top of a scene (source sharing).
#[tauri::command]
pub fn studio_add_existing_source(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    source_id: SourceId,
) -> Result<ItemId, String> {
    state.mutate_tracked(&app, "addSource", None, |collection| {
        collection.add_item_with_existing_source(scene_id, source_id)
    })
}

#[tauri::command]
pub fn studio_remove_item(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    item_id: ItemId,
) -> Result<(), String> {
    state.mutate_tracked(&app, "removeSource", None, |collection| {
        collection.remove_item(scene_id, item_id)
    })
}

#[tauri::command]
pub fn studio_reorder_item(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    item_id: ItemId,
    to_index: usize,
) -> Result<(), String> {
    state.mutate_tracked(&app, "reorderSource", None, |collection| {
        collection.reorder_item(scene_id, item_id, to_index)
    })
}

#[tauri::command]
pub fn studio_set_item_transform(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    item_id: ItemId,
    transform: Transform,
) -> Result<(), String> {
    state.mutate_tracked(
        &app,
        "transformSource",
        Some(coalesce_key("transform", item_id)),
        |collection| collection.set_item_transform(scene_id, item_id, transform),
    )
}

#[tauri::command]
pub fn studio_set_item_visible(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    item_id: ItemId,
    visible: bool,
) -> Result<(), String> {
    state.mutate_tracked(&app, "toggleVisibility", None, |collection| {
        collection.set_item_visible(scene_id, item_id, visible)
    })
}

#[tauri::command]
pub fn studio_set_item_locked(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    item_id: ItemId,
    locked: bool,
) -> Result<(), String> {
    state.mutate_tracked(&app, "toggleLock", None, |collection| {
        collection.set_item_locked(scene_id, item_id, locked)
    })
}

#[tauri::command]
pub fn studio_set_item_blend(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    item_id: ItemId,
    blend: BlendMode,
) -> Result<(), String> {
    state.mutate_tracked(&app, "setBlendMode", None, |collection| {
        collection.set_item_blend(scene_id, item_id, blend)
    })
}

/// One corner assignment in a layout request (JS sends `{ itemId, corner }`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CornerSlot {
    pub item_id: ItemId,
    pub corner: Corner,
}

/// Arrange the scene as a centered screen with up to four corner cameras — the
/// screen-plus-corners layout. `center` (if any) becomes the backdrop at the
/// bottom of the z-order; each corner item fits into its slot on top.
#[tauri::command]
pub fn studio_apply_layout(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    center: Option<ItemId>,
    corners: Vec<CornerSlot>,
) -> Result<(), String> {
    state.mutate_tracked(&app, "applyLayout", None, |collection| {
        let corners: Vec<(ItemId, Corner)> = corners
            .iter()
            .map(|slot| (slot.item_id, slot.corner))
            .collect();
        collection.apply_layout(scene_id, center, &corners)
    })
}

/// Seat one item into a normalized canvas slot — the one-click position
/// presets (top/middle/bottom × left/right) for a remote guest or any item.
/// The model validates the slot (untrusted webview input).
#[tauri::command]
pub fn studio_set_item_slot(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    item_id: ItemId,
    slot: NormRect,
) -> Result<(), String> {
    state.mutate_tracked(&app, "moveToSeat", None, |collection| {
        collection.set_item_slot(scene_id, item_id, slot)
    })
}

/// Center-view routing: `Some(item)` promotes that capture into the center
/// seat (the displaced center swaps onto the mover's old seat; overlapping
/// cams bump to the rail; one screen view at a time). `None` retires the
/// center onto the rail.
#[tauri::command]
pub fn studio_set_center_view(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    item_id: Option<ItemId>,
) -> Result<(), String> {
    state.mutate(&app, |collection| {
        collection.set_center_view(scene_id, item_id)
    })
}

/// Group items so they move / show / hide together (Phase 6, TASK-605).
#[tauri::command]
pub fn studio_create_group(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    name: String,
    item_ids: Vec<ItemId>,
) -> Result<fcap_scene::GroupId, String> {
    state.mutate_tracked(&app, "groupSources", None, |collection| {
        collection.create_group(scene_id, &name, &item_ids)
    })
}

/// Dissolve a group — its items stay exactly where they are.
#[tauri::command]
pub fn studio_ungroup(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    group_id: fcap_scene::GroupId,
) -> Result<(), String> {
    state.mutate_tracked(&app, "ungroupSources", None, |collection| {
        collection.ungroup(scene_id, group_id)
    })
}

/// A group's eye toggle — hides/shows every member together.
#[tauri::command]
pub fn studio_set_group_visible(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    group_id: fcap_scene::GroupId,
    visible: bool,
) -> Result<(), String> {
    state.mutate_tracked(&app, "toggleGroupVisibility", None, |collection| {
        collection.set_group_visible(scene_id, group_id, visible)
    })
}

/// A source's per-scene mixer override (Phase 6, TASK-605): while the scene
/// is the program, the override replaces the global fader/mute. `null`
/// clears it (back to the global mix).
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneAudioOverrideArg {
    pub volume_db: f32,
    pub muted: bool,
}

#[tauri::command]
pub fn studio_set_scene_audio_override(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    source_id: fcap_scene::SourceId,
    over: Option<SceneAudioOverrideArg>,
) -> Result<(), String> {
    state.mutate_tracked(
        &app,
        "setSceneAudio",
        Some(coalesce_key("sceneAudio", source_id)),
        |collection| {
            collection.set_scene_audio_override(
                scene_id,
                source_id,
                over.as_ref().map(|entry| (entry.volume_db, entry.muted)),
            )
        },
    )
}

/// Configure (or clear, with `null`) the second output canvas — e.g. a
/// vertical 9:16 feed composed from any scene (Phase 6, TASK-604).
#[tauri::command]
pub fn studio_set_vertical(
    app: AppHandle,
    state: State<'_, StudioState>,
    vertical: Option<fcap_scene::VerticalCanvas>,
) -> Result<(), String> {
    state.mutate_tracked(&app, "setVerticalCanvas", None, |collection| {
        collection.set_vertical(vertical)
    })
}

/// Studio Mode (Phase 5): on = a preview pane opens on the program scene;
/// off = back to single-pane editing.
#[tauri::command]
pub fn studio_set_studio_mode(
    app: AppHandle,
    state: State<'_, StudioState>,
    on: bool,
) -> Result<(), String> {
    state.set_studio_mode(&app, on);
    Ok(())
}

/// Point the Studio-Mode preview pane at a scene.
#[tauri::command]
pub fn studio_set_preview_scene(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
) -> Result<(), String> {
    state.set_preview_scene(&app, scene_id)
}

/// Commit Preview → Program through the configured transition — the audience
/// sees the blend; the panes swap (OBS semantics).
#[tauri::command]
pub fn studio_transition(
    app: AppHandle,
    state: State<'_, StudioState>,
    settings: State<'_, crate::settings::SettingsStore>,
) -> Result<(), String> {
    let transition = settings.get().transition;
    transition.validate()?;
    state.begin_transition(&app, &transition)
}

/// Highlight Speaker (Focus/Spotlight): `Some(item)` promotes that item to
/// fill the canvas (hiding the other video items); `None` restores the exact
/// pre-focus layout.
#[tauri::command]
pub fn studio_set_focus(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    item_id: Option<ItemId>,
) -> Result<(), String> {
    state.mutate(&app, |collection| match item_id {
        Some(item) => collection.set_focus(scene_id, item),
        None => collection.clear_focus(scene_id),
    })
}

// -- sources ------------------------------------------------------------------

#[tauri::command]
pub fn studio_rename_source(
    app: AppHandle,
    state: State<'_, StudioState>,
    source_id: SourceId,
    name: String,
) -> Result<(), String> {
    state.mutate_tracked(&app, "renameSource", None, |collection| {
        collection.rename_source(source_id, &name)
    })
}

/// Replace a source's settings (the Properties dialog). The engine restarts
/// that source on its next tick.
#[tauri::command]
pub fn studio_update_source_settings(
    app: AppHandle,
    state: State<'_, StudioState>,
    source_id: SourceId,
    settings: SourceSettings,
) -> Result<(), String> {
    state.mutate_tracked(
        &app,
        "editSourceProperties",
        Some(coalesce_key("sourceProps", source_id)),
        |collection| collection.update_source_settings(source_id, settings),
    )
}

/// Restart an errored source with unchanged settings (replugged camera,
/// permission granted after a denial, reopened window).
#[tauri::command]
pub fn studio_retry_source(
    state: State<'_, StudioState>,
    source_id: SourceId,
) -> Result<(), String> {
    state.retry_source(source_id)
}

/// Engage or restore the panic slate (CAP-M22). The hotkey and the palette
/// engage; only the UI's deliberate two-step confirm restores.
#[tauri::command]
pub fn studio_panic_set<R: Runtime>(app: AppHandle<R>, state: State<'_, StudioState>, on: bool) {
    state.set_panic(&app, on);
}

/// Pause or resume a **Media source** (an embedded video) mid-broadcast — it
/// holds the last frame + goes silent while paused, so a streamer can talk
/// over a video, resume it, and remove it at any time, all live on the stream
/// and the recording. A no-op for non-media sources.
#[tauri::command]
pub fn studio_media_set_paused(source_id: SourceId, paused: bool) {
    fcap_sources::media::set_media_paused(&source_id.0.to_string(), paused);
}

/// Whether a Media source is currently paused (the UI reads this to label the
/// pause/resume button after a reload).
#[tauri::command]
pub fn studio_media_paused(source_id: SourceId) -> bool {
    fcap_sources::media::is_media_paused(&source_id.0.to_string())
}

// -- filters --------------------------------------------------------------------

#[tauri::command]
pub fn studio_add_filter(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    item_id: ItemId,
    kind: FilterKind,
) -> Result<FilterId, String> {
    state.mutate_tracked(&app, "addFilter", None, |collection| {
        collection.add_filter(scene_id, item_id, kind)
    })
}

#[tauri::command]
pub fn studio_remove_filter(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    item_id: ItemId,
    filter_id: FilterId,
) -> Result<(), String> {
    state.mutate_tracked(&app, "removeFilter", None, |collection| {
        collection.remove_filter(scene_id, item_id, filter_id)
    })
}

#[tauri::command]
pub fn studio_reorder_filter(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    item_id: ItemId,
    filter_id: FilterId,
    to_index: usize,
) -> Result<(), String> {
    state.mutate_tracked(&app, "reorderFilter", None, |collection| {
        collection.reorder_filter(scene_id, item_id, filter_id, to_index)
    })
}

#[tauri::command]
pub fn studio_update_filter(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    item_id: ItemId,
    filter_id: FilterId,
    kind: FilterKind,
) -> Result<(), String> {
    state.mutate_tracked(
        &app,
        "editFilter",
        Some(coalesce_key("filterParams", filter_id)),
        |collection| collection.update_filter(scene_id, item_id, filter_id, kind),
    )
}

#[tauri::command]
pub fn studio_set_filter_enabled(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    item_id: ItemId,
    filter_id: FilterId,
    enabled: bool,
) -> Result<(), String> {
    state.mutate_tracked(&app, "toggleFilter", None, |collection| {
        collection.set_filter_enabled(scene_id, item_id, filter_id, enabled)
    })
}

/// Paste a copied filter chain onto an item (CAP-M05). Each copied filter is
/// appended on top with a fresh id, keeping its kind + enabled state; the whole
/// paste is one undo step. Returns how many filters were added.
#[tauri::command]
pub fn studio_paste_filters(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    item_id: ItemId,
    filters: Vec<Filter>,
) -> Result<usize, String> {
    state.mutate_tracked(&app, "pasteFilters", None, |collection| {
        collection.paste_filters(scene_id, item_id, filters)
    })
}

// -- keying workbench (CAP-M26) ------------------------------------------------

/// Open/update the keying workbench: render `item_id` in `mode` (with the
/// `split` divider position for Split mode). Preview-only — nothing persists;
/// the render thread publishes to the `workbench-preview` slot.
#[tauri::command]
pub fn studio_workbench_set(
    state: State<'_, StudioState>,
    item_id: ItemId,
    mode: WorkbenchMode,
    split: f32,
) {
    state.set_workbench(item_id, mode, split);
}

/// Close the keying workbench (clears its preview slot).
#[tauri::command]
pub fn studio_workbench_close(state: State<'_, StudioState>) {
    state.close_workbench();
}

// -- multiview monitor (CAP-M06) ----------------------------------------------

/// Open/close the multiview monitor: while `on`, the render loop keeps every
/// scene's sources live and publishes per-scene thumbnails to `/multiview/<id>`.
#[tauri::command]
pub fn studio_multiview_set(state: State<'_, StudioState>, on: bool) {
    state.set_multiview(on);
}

/// Grab a still frame (CAP-M08): a lossless PNG of the program or a single
/// source, saved into the recordings folder. The render loop saves it on its
/// next tick and emits `still-saved` (the path) or `still-error`.
#[tauri::command]
pub fn studio_capture_still(app: AppHandle, target: StillTarget) {
    crate::studio::capture_still(&app, target);
}

// -- missing-file doctor (CAP-M03) --------------------------------------------

/// One broken file reference, grouped by path — the same missing file used in
/// several places is listed once with a `uses` count. Relinking is by path, so
/// fixing it here repairs every use.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MissingFile {
    pub path: String,
    /// The kind of the first reference to this path (for an icon/label).
    pub kind: FileRefKind,
    /// The first source that uses it (context for the user).
    pub source_name: String,
    /// How many references across the collection share this path.
    pub uses: usize,
}

/// A path we won't stat locally — a stream/URL, or a UNC network path. Statting
/// a UNC path on Windows forces an SMB connection (and an NTLM handshake) to the
/// host, so an imported collection referencing `\\attacker\share\x.png` must
/// never be probed. Such paths are treated as "not missing" (never reported).
fn is_remote(path: &str) -> bool {
    path.contains("://") || path.starts_with("\\\\") || path.starts_with("//")
}

/// Collapse the collection's file references down to the ones that don't
/// resolve on disk, grouped by path.
fn missing_from(refs: Vec<FileRef>) -> Vec<MissingFile> {
    let mut out: Vec<MissingFile> = Vec::new();
    for r in refs {
        if is_remote(&r.path) || std::path::Path::new(&r.path).exists() {
            continue;
        }
        if let Some(existing) = out.iter_mut().find(|m| m.path == r.path) {
            existing.uses += 1;
        } else {
            out.push(MissingFile {
                path: r.path,
                kind: r.kind,
                source_name: r.source_name,
                uses: 1,
            });
        }
    }
    out
}

/// The doctor's scan: every referenced file that is missing from disk. An empty
/// list means every image/media/font/LUT/mask path resolves.
#[tauri::command]
pub fn collection_missing_files(state: State<'_, StudioState>) -> Vec<MissingFile> {
    missing_from(state.with_collection(|c| c.file_refs()))
}

/// Repoint one broken path to a new one everywhere it appears; returns how many
/// references changed. Undoable (one step).
#[tauri::command]
pub fn collection_relink(
    app: AppHandle,
    state: State<'_, StudioState>,
    old_path: String,
    new_path: String,
) -> Result<usize, String> {
    state.mutate_tracked(&app, "relinkFiles", None, |collection| {
        Ok(collection.relink_file(&old_path, &new_path))
    })
}

/// Bulk relink: for each still-missing file, look for a file of the same name
/// in `folder`; repoint the ones found. Returns how many references changed.
/// Undoable as a single step.
#[tauri::command]
pub fn collection_relink_folder(
    app: AppHandle,
    state: State<'_, StudioState>,
    folder: String,
) -> Result<usize, String> {
    let plan: Vec<(String, String)> = missing_from(state.with_collection(|c| c.file_refs()))
        .into_iter()
        .filter_map(|missing| {
            let name = std::path::Path::new(&missing.path).file_name()?;
            let candidate = std::path::Path::new(&folder).join(name);
            candidate
                .is_file()
                .then(|| (missing.path, candidate.to_string_lossy().into_owned()))
        })
        .collect();
    if plan.is_empty() {
        return Ok(0);
    }
    state.mutate_tracked(&app, "relinkFiles", None, |collection| {
        Ok(plan
            .iter()
            .map(|(old, new)| collection.relink_file(old, new))
            .sum())
    })
}

// -- multi-item arrange + custom guides (CAP-M04 follow-on) --------------------

/// One item's new transform in a batch arrange (align / distribute / group move).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformChange {
    pub item: ItemId,
    pub transform: Transform,
}

/// Apply several item transforms as a **single** undo step — align-to-each-other,
/// distribute, and group drags. `coalesce` folds a streaming group drag into one
/// step; discrete arranges pass `false`.
#[tauri::command]
pub fn studio_set_item_transforms(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    changes: Vec<TransformChange>,
    coalesce: bool,
) -> Result<(), String> {
    let key = coalesce.then(|| coalesce_key("arrange", scene_id));
    state.mutate_tracked(&app, "arrangeItems", key, move |collection| {
        // Validate every target first, so a partial batch can never leave the
        // collection half-moved (an early Err records nothing on the undo stack).
        for change in &changes {
            collection
                .scene(scene_id)
                .and_then(|scene| scene.item(change.item))
                .ok_or(fcap_scene::SceneError::ItemNotFound)?;
        }
        for change in &changes {
            collection.set_item_transform(scene_id, change.item, change.transform)?;
        }
        Ok(())
    })
}

/// Replace a scene's custom alignment guides (CAP-M04 follow-on). The UI manages
/// add/move/delete locally and commits the whole list on drop — one undo step.
#[tauri::command]
pub fn studio_set_guides(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    guides: Vec<GuideLine>,
) -> Result<(), String> {
    state.mutate_tracked(&app, "editGuides", None, move |collection| {
        collection.set_guides(scene_id, guides)
    })
}
