//! The studio command surface: every scene/source/item/filter mutation the
//! UI (and later the remote API / scripting) drives.
//!
//! Each command validates against the model (`fcap-scene` returns an error
//! and mutates nothing on a bad target), bumps the revision, and pushes the
//! fresh model on the `studio` event — the UI never has to guess.
//! (JS sends camelCase argument names; Tauri maps them onto these
//! snake_case parameters, same as the Phase 1 commands.)

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, State};

use fcap_scene::{
    BlendMode, Corner, FilterId, FilterKind, ItemId, SceneId, Source, SourceId, SourceSettings,
    Transform,
};

use crate::studio::{StudioDto, StudioState};

/// The whole current model (initial load / reconnect).
#[tauri::command]
pub fn studio_get(state: State<'_, StudioState>) -> StudioDto {
    state.snapshot()
}

// -- scenes -----------------------------------------------------------------

#[tauri::command]
pub fn studio_add_scene(
    app: AppHandle,
    state: State<'_, StudioState>,
    name: String,
) -> Result<SceneId, String> {
    state.mutate(&app, |collection| Ok(collection.add_scene(&name)))
}

#[tauri::command]
pub fn studio_rename_scene(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    name: String,
) -> Result<(), String> {
    state.mutate(&app, |collection| collection.rename_scene(scene_id, &name))
}

#[tauri::command]
pub fn studio_remove_scene(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
) -> Result<(), String> {
    state.mutate(&app, |collection| collection.remove_scene(scene_id))
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
    state.mutate(&app, |collection| {
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
    state.mutate(&app, |collection| {
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
    state.mutate(&app, |collection| {
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
    state.mutate(&app, |collection| collection.remove_item(scene_id, item_id))
}

#[tauri::command]
pub fn studio_reorder_item(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    item_id: ItemId,
    to_index: usize,
) -> Result<(), String> {
    state.mutate(&app, |collection| {
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
    state.mutate(&app, |collection| {
        collection.set_item_transform(scene_id, item_id, transform)
    })
}

#[tauri::command]
pub fn studio_set_item_visible(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    item_id: ItemId,
    visible: bool,
) -> Result<(), String> {
    state.mutate(&app, |collection| {
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
    state.mutate(&app, |collection| {
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
    state.mutate(&app, |collection| {
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
    state.mutate(&app, |collection| {
        let corners: Vec<(ItemId, Corner)> = corners
            .iter()
            .map(|slot| (slot.item_id, slot.corner))
            .collect();
        collection.apply_layout(scene_id, center, &corners)
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
    state.mutate(&app, |collection| {
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
    state.mutate(&app, |collection| {
        collection.update_source_settings(source_id, settings)
    })
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

// -- filters --------------------------------------------------------------------

#[tauri::command]
pub fn studio_add_filter(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    item_id: ItemId,
    kind: FilterKind,
) -> Result<FilterId, String> {
    state.mutate(&app, |collection| {
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
    state.mutate(&app, |collection| {
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
    state.mutate(&app, |collection| {
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
    state.mutate(&app, |collection| {
        collection.update_filter(scene_id, item_id, filter_id, kind)
    })
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
    state.mutate(&app, |collection| {
        collection.set_filter_enabled(scene_id, item_id, filter_id, enabled)
    })
}
