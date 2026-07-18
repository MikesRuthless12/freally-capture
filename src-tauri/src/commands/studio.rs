//! The studio command surface: every scene/source/item/filter mutation the
//! UI (and later the remote API / scripting) drives.
//!
//! Each command validates against the model (`fcap-scene` returns an error
//! and mutates nothing on a bad target), bumps the revision, and pushes the
//! fresh model on the `studio` event — the UI never has to guess.
//! (JS sends camelCase argument names; Tauri maps them onto these
//! snake_case parameters, same as the Phase 1 commands.)

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Runtime, State};

use fcap_scene::{
    BackdropSplit, BlendMode, Corner, DskId, FileRef, FileRefKind, Filter, FilterId, FilterKind,
    GuideLine, ItemId, NormRect, ScaleMode, SceneError, SceneId, Source, SourceId, SourceSettings,
    Transform, TransitionKind,
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

/// Drive one timer source's run state (CAP-M15): "start" | "pause" |
/// "toggle" | "reset". Runtime-only — no model change, no undo; the render
/// loop repaints the face next tick.
#[tauri::command]
pub fn studio_timer_control(
    state: State<'_, StudioState>,
    source_id: SourceId,
    action: String,
) -> Result<(), String> {
    let cmd = crate::studio::TimerCmd::parse(&action)
        .ok_or_else(|| format!("unknown timer action: {action}"))?;
    state.timer_control(source_id, cmd);
    Ok(())
}

/// Drive one split-timer source's run (CAP-N18): "split" | "undo" | "skip"
/// | "reset". Runtime-only, like the timer control above; the session
/// thread repaints the face on the next state change.
#[tauri::command]
pub fn studio_split_control(source_id: SourceId, action: String) -> Result<(), String> {
    let action = match action.as_str() {
        "split" => fcap_sources::splits::SplitAction::Split,
        "undo" => fcap_sources::splits::SplitAction::Undo,
        "skip" => fcap_sources::splits::SplitAction::Skip,
        "reset" => fcap_sources::splits::SplitAction::Reset,
        other => return Err(format!("unknown split action: {other}")),
    };
    if !fcap_sources::splits::control(&source_id.0.to_string(), action) {
        return Err("that split timer is not running".into());
    }
    Ok(())
}

/// Jump one playlist (CAP-N17): "next" | "previous". Runtime-only.
#[tauri::command]
pub fn studio_playlist_control(source_id: SourceId, action: String) -> Result<(), String> {
    let action = match action.as_str() {
        "next" => fcap_sources::playlist::PlaylistAction::Next,
        "previous" => fcap_sources::playlist::PlaylistAction::Previous,
        other => return Err(format!("unknown playlist action: {other}")),
    };
    if !fcap_sources::playlist::control(&source_id.0.to_string(), action) {
        return Err("that playlist is not running".into());
    }
    Ok(())
}

/// Fire a title's animate-in/out (CAP-N16): "in" | "out". Runtime-only,
/// like the timer control above — the session thread animates from the
/// next tick.
#[tauri::command]
pub fn studio_title_fire(source_id: SourceId, action: String) -> Result<(), String> {
    let action = match action.as_str() {
        "in" => fcap_sources::title::TitleAction::FireIn,
        "out" => fcap_sources::title::TitleAction::FireOut,
        other => return Err(format!("unknown title action: {other}")),
    };
    if !fcap_sources::title::control(&source_id.0.to_string(), action) {
        return Err("that title is not running".into());
    }
    Ok(())
}

/// Push live text into one title layer (CAP-N16) WITHOUT restarting the
/// session — the scoreboard operator's edit. Runtime-only: Apply rebuilds
/// the session from the model and clears these overrides.
#[tauri::command]
pub fn studio_title_set_text(
    source_id: SourceId,
    layer: usize,
    value: String,
) -> Result<(), String> {
    if !fcap_sources::title::control(
        &source_id.0.to_string(),
        fcap_sources::title::TitleAction::SetLayerText(layer, value),
    ) {
        return Err("that title is not running".into());
    }
    Ok(())
}

/// Jump one playlist to a cue: `seconds` into ORIGINAL item `item`'s file.
#[tauri::command]
pub fn studio_playlist_cue(source_id: SourceId, item: usize, seconds: f32) -> Result<(), String> {
    if !seconds.is_finite() || seconds < 0.0 {
        return Err("cue seconds must be a non-negative number".into());
    }
    if !fcap_sources::playlist::cue(&source_id.0.to_string(), item, seconds) {
        return Err("that playlist is not running".into());
    }
    Ok(())
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

// -- downstream keyers (CAP-N24) --------------------------------------------

/// Add a downstream-keyer layer over the program for `source_id`.
#[tauri::command]
pub fn studio_downstream_add(
    app: AppHandle,
    state: State<'_, StudioState>,
    source_id: SourceId,
) -> Result<DskId, String> {
    state.mutate_tracked(&app, "addDownstream", None, |collection| {
        collection
            .add_downstream(source_id)
            .ok_or(SceneError::SourceNotFound)
    })
}

/// Remove a downstream-keyer layer.
#[tauri::command]
pub fn studio_downstream_remove(
    app: AppHandle,
    state: State<'_, StudioState>,
    id: DskId,
) -> Result<(), String> {
    state.mutate_tracked(&app, "removeDownstream", None, |collection| {
        collection.remove_downstream(id);
        Ok(())
    })
}

/// Move a keyer one step in draw order (`up` = on top).
#[tauri::command]
pub fn studio_downstream_move(
    app: AppHandle,
    state: State<'_, StudioState>,
    id: DskId,
    up: bool,
) -> Result<(), String> {
    state.mutate_tracked(&app, "moveDownstream", None, |collection| {
        collection.move_downstream(id, up);
        Ok(())
    })
}

/// Toggle a keyer on/off (off = not composited, but kept in the list).
#[tauri::command]
pub fn studio_downstream_set_enabled(
    app: AppHandle,
    state: State<'_, StudioState>,
    id: DskId,
    enabled: bool,
) -> Result<(), String> {
    state.mutate_tracked(&app, "downstreamEnabled", None, |collection| {
        if let Some(dsk) = collection.downstream_mut(id) {
            dsk.enabled = enabled;
        }
        Ok(())
    })
}

/// Set a keyer's opacity (0..=1).
#[tauri::command]
pub fn studio_downstream_set_opacity(
    app: AppHandle,
    state: State<'_, StudioState>,
    id: DskId,
    opacity: f32,
) -> Result<(), String> {
    state.mutate_tracked(
        &app,
        "downstreamOpacity",
        Some(coalesce_key("dskOpacity", id.0)),
        |collection| {
            if let Some(dsk) = collection.downstream_mut(id) {
                dsk.opacity = opacity.clamp(0.0, 1.0);
            }
            Ok(())
        },
    )
}

/// Set a keyer's transform (position / size / 3D tilt).
#[tauri::command]
pub fn studio_downstream_set_transform(
    app: AppHandle,
    state: State<'_, StudioState>,
    id: DskId,
    mut transform: Transform,
) -> Result<(), String> {
    // Defense-in-depth: keep a keyer's geometry sane even if a value bypasses
    // the (clamping) UI — a 0/negative/NaN scale would collapse or mirror it.
    let sane = |v: f32, fallback: f32| if v.is_finite() { v } else { fallback };
    transform.x = sane(transform.x, 0.0);
    transform.y = sane(transform.y, 0.0);
    transform.scale_x = sane(transform.scale_x, 1.0).clamp(0.01, 100.0);
    transform.scale_y = sane(transform.scale_y, 1.0).clamp(0.01, 100.0);
    transform.rotation = sane(transform.rotation, 0.0);
    transform.rotation_x = sane(transform.rotation_x, 0.0);
    transform.rotation_y = sane(transform.rotation_y, 0.0);
    transform.perspective = sane(transform.perspective, 0.0).clamp(0.0, 1.0);
    state.mutate_tracked(
        &app,
        "downstreamTransform",
        Some(coalesce_key("dskTransform", id.0)),
        |collection| {
            if let Some(dsk) = collection.downstream_mut(id) {
                dsk.transform = transform;
            }
            Ok(())
        },
    )
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

/// Per-output visibility (CAP-N53): compose the item into the live outputs
/// (`on_stream`) / the local-disk outputs (`on_record`). Master visibility
/// (the eye toggle) is a separate flag and stays untouched.
#[tauri::command]
pub fn studio_set_item_output_visible(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    item_id: ItemId,
    on_stream: bool,
    on_record: bool,
) -> Result<(), String> {
    state.mutate_tracked(&app, "toggleOutputVisibility", None, |collection| {
        collection.set_item_output_visible(scene_id, item_id, on_stream, on_record)
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

/// Pixel-perfect scaling (CAP-N70): smooth / nearest / integer-snapped
/// nearest / sharp-bilinear, per item.
#[tauri::command]
pub fn studio_set_item_scaling(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    item_id: ItemId,
    scaling: ScaleMode,
) -> Result<(), String> {
    state.mutate_tracked(&app, "setScaling", None, |collection| {
        collection.set_item_scaling(scene_id, item_id, scaling)
    })
}

/// Show/hide fade-in duration for an item (CAP-N21); 0 = appear instantly.
#[tauri::command]
pub fn studio_set_item_reveal(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    item_id: ItemId,
    reveal_ms: u32,
) -> Result<(), String> {
    state.mutate_tracked(&app, "setReveal", None, |collection| {
        collection.set_item_reveal(scene_id, item_id, reveal_ms)
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

/// One participant of an auto-grid (CAP-N59): the guest's scene item + name.
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridParticipant {
    pub item_id: ItemId,
    pub name: String,
}

/// Reflow the given participants into an automatic 1–9 grid (CAP-N59). The
/// non-overlapping geometry is engine-side (`grid_seats`); each participant's
/// name is exposed as the `{{guestN}}` variable, so a title source referencing
/// it becomes an auto-updating nameplate (CAP-N16 wiring).
#[tauri::command]
pub fn studio_auto_grid(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    participants: Vec<GridParticipant>,
) -> Result<(), String> {
    // Nameplates: publish each seat's name as {{guestN}} (CAP-N02 variable
    // store → the studio loop feeds it to every title each revision). Blank the
    // tail slots so a shrinking grid (5 guests → 2) never leaves a departed
    // name ghosting in a title bound to {{guest3}}.
    let automation = app.state::<crate::automation::AutomationState>();
    for (index, participant) in participants.iter().enumerate() {
        automation.set_variable(&format!("guest{}", index + 1), &participant.name);
    }
    for index in participants.len()..fcap_scene::scene::MAX_GRID {
        automation.set_variable(&format!("guest{}", index + 1), "");
    }
    let seats = fcap_scene::scene::grid_seats(participants.len());
    let assignments: Vec<(ItemId, NormRect)> = participants
        .iter()
        .zip(seats)
        .map(|(participant, seat)| (participant.item_id, seat))
        .collect();
    state.mutate(&app, |collection| {
        collection.apply_grid(scene_id, &assignments)
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
    let mut transition = settings.get().transition;
    // CAP-N21: a per-scene-pair rule overrides the default kind + duration for
    // this pair (the stinger/luma file still comes from the global settings).
    if let Some((kind, duration_ms)) = state.transition_override() {
        // A rule that needs a file the global settings don't provide (a Stinger
        // with no video, an Image wipe with no luma) would hard-fail the take —
        // fall back to the default transition rather than not switching scenes.
        let file_ready = match kind {
            TransitionKind::Stinger => !transition.stinger_path.trim().is_empty(),
            TransitionKind::LumaImage => !transition.luma_image.trim().is_empty(),
            _ => true,
        };
        if file_ready {
            transition.kind = kind;
            transition.duration_ms = duration_ms;
        }
    }
    transition.validate()?;
    state.begin_transition(&app, &transition)
}

/// Set (or replace) the per-scene-pair transition rule (CAP-N21) for `from`→`to`.
#[tauri::command]
pub fn studio_transition_override_set(
    app: AppHandle,
    state: State<'_, StudioState>,
    from: SceneId,
    to: SceneId,
    kind: TransitionKind,
    duration_ms: u32,
) -> Result<(), String> {
    state.mutate_tracked(&app, "transitionRule", None, |collection| {
        collection.set_transition_override(from, to, kind, duration_ms)
    })
}

/// Remove the per-scene-pair transition rule (CAP-N21) for `from`→`to`.
#[tauri::command]
pub fn studio_transition_override_remove(
    app: AppHandle,
    state: State<'_, StudioState>,
    from: SceneId,
    to: SceneId,
) -> Result<(), String> {
    state.mutate_tracked(&app, "removeTransitionRule", None, |collection| {
        collection.remove_transition_override(from, to);
        Ok(())
    })
}

/// Export a bezier mask's path (CAP-N28) as a grayscale luma-wipe PNG at `path`,
/// usable as an Image Wipe transition pattern.
#[tauri::command]
pub fn bezier_export_wipe(
    state: State<'_, StudioState>,
    scene_id: SceneId,
    item_id: ItemId,
    filter_id: FilterId,
    path: String,
) -> Result<(), String> {
    let (points, feather, invert) = state.bezier_mask_params(scene_id, item_id, filter_id)?;
    let rgba = crate::bezier_mask::wipe_rgba(&points, feather, invert)
        .ok_or("the mask needs at least three points to export")?;
    let size = crate::bezier_mask::MASK_SIZE;
    let buffer: ::image::RgbaImage =
        ::image::ImageBuffer::from_raw(size, size, rgba).ok_or("could not build the wipe image")?;
    buffer.save(&path).map_err(|err| err.to_string())
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

/// Jump a Media source's playback to `seconds` (the transport scrubber).
/// Applied at the next frame boundary; seeking while paused still shows the
/// sought frame.
#[tauri::command]
pub fn studio_media_seek(source_id: SourceId, seconds: f32) {
    fcap_sources::media::media_seek(&source_id.0.to_string(), seconds);
}

/// A Media source's transport state, polled by the scrubber UI.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaTransport {
    pub position: f32,
    /// `0` while unknown (a `.frec` learns its total at first end-of-file).
    pub duration: f32,
}

#[tauri::command]
pub fn studio_media_transport(source_id: SourceId) -> MediaTransport {
    let (position, duration) = fcap_sources::media::media_transport(&source_id.0.to_string());
    MediaTransport { position, duration }
}

// -- passthrough monitor (CAP-N69) ------------------------------------------------

/// The passthrough monitor's measured end-to-end latency for a source, in
/// milliseconds — the capture timestamp → published-frame delta (an EMA).
/// `null` until a passthrough monitor has been open long enough to measure.
/// Honest scope: this is the in-app pipeline latency; the display's own
/// buffering is not visible to us, which is why the DoD wants a camera-clap
/// validation on real hardware.
#[tauri::command]
pub fn studio_passthrough_latency(
    state: State<'_, StudioState>,
    source_id: SourceId,
) -> Option<f32> {
    state.passthrough_latency(source_id)
}

// -- window↔app-audio auto-link (CAP-N73) ----------------------------------------

/// Add a Window Capture together with its app's audio as one linked pair
/// (one undo entry). The audio follows the window: hidden mutes it, removal
/// removes it, and the engine re-resolves its pid across app restarts.
/// Windows-only (process resolution); the picker only offers it there.
#[tauri::command]
pub fn studio_add_linked_window(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    capture_id: String,
    label: String,
) -> Result<AddedItem, String> {
    let (pid, exe) = fcap_capture::window_process(&capture_id)
        .ok_or_else(|| "could not resolve the window's process".to_string())?;
    state.mutate_tracked(&app, "addLinkedWindow", None, |collection| {
        let window = Source::new(label.clone(), SourceSettings::Window { capture_id, label });
        let (window_id, item_id) = collection.add_item_with_new_source(scene_id, window)?;
        let audio = Source::new(
            format!("{exe} audio"),
            SourceSettings::AppAudio {
                pid,
                exe,
                linked_window: Some(window_id),
            },
        );
        collection.add_item_with_new_source(scene_id, audio)?;
        Ok(AddedItem {
            source_id: window_id,
            item_id,
        })
    })
}

// -- auto black-bar crop (CAP-N72) ---------------------------------------------

/// One-shot: scan the item's next frame for letterbox/pillarbox bars and
/// apply the detected crop as an undoable edit.
#[tauri::command]
pub fn studio_autocrop(
    state: State<'_, StudioState>,
    scene_id: SceneId,
    item_id: ItemId,
) -> Result<(), String> {
    state.autocrop_request(scene_id, item_id)
}

/// Arm/disarm re-detection when the source's resolution changes (follow
/// mode; off by default).
#[tauri::command]
pub fn studio_autocrop_follow(
    state: State<'_, StudioState>,
    scene_id: SceneId,
    item_id: ItemId,
    follow: bool,
) -> Result<(), String> {
    state.autocrop_set_follow(scene_id, item_id, follow)
}

/// Whether follow-mode auto-crop is armed for an item (the UI checkbox).
#[tauri::command]
pub fn studio_autocrop_get(state: State<'_, StudioState>, item_id: ItemId) -> bool {
    state.autocrop_get(item_id)
}

// -- punch-in zoom (CAP-N71) --------------------------------------------------

/// A punch-in lens's target state, for the UI.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ZoomLensState {
    pub zoom: f32,
    pub follow: bool,
}

/// Set a lens's target zoom (absolute, clamped `1..=8`) about an optional
/// normalized content anchor. Runtime-only: drawn every tick, spring-smoothed,
/// never written to the item's transform or the undo history.
#[tauri::command]
pub fn studio_zoom_set(
    state: State<'_, StudioState>,
    item_id: ItemId,
    zoom: f32,
    anchor_x: Option<f32>,
    anchor_y: Option<f32>,
) {
    state.zoom_set(item_id, zoom, anchor_x.zip(anchor_y));
}

/// Multiply a lens's target zoom (the canvas wheel) about an anchor.
#[tauri::command]
pub fn studio_zoom_scroll(
    state: State<'_, StudioState>,
    item_id: ItemId,
    factor: f32,
    anchor_x: f32,
    anchor_y: f32,
) {
    state.zoom_scroll(item_id, factor, (anchor_x, anchor_y));
}

/// Toggle follow-the-cursor panning while zoomed (Windows maps the cursor
/// into display/window captures; elsewhere the anchor stays manual).
#[tauri::command]
pub fn studio_zoom_follow(state: State<'_, StudioState>, item_id: ItemId, follow: bool) {
    state.zoom_follow(item_id, follow);
}

/// A lens's current target (the UI reads this after a reload).
#[tauri::command]
pub fn studio_zoom_get(state: State<'_, StudioState>, item_id: ItemId) -> ZoomLensState {
    let (zoom, follow) = state.zoom_get(item_id);
    ZoomLensState { zoom, follow }
}

// -- scene backdrop (wallpaper) ---------------------------------------------------

/// Image extensions a backdrop decodes once (the Image source path).
const BACKDROP_STILL_EXTS: [&str; 7] = ["png", "jpg", "jpeg", "bmp", "webp", "tif", "tiff"];
/// Extensions a backdrop plays on a loop (the Media source path — GIFs
/// animate through the owned decoder, `.frec` through the owned codec, the
/// wire formats through the labeled ffmpeg component).
const BACKDROP_MEDIA_EXTS: [&str; 6] = ["gif", "mp4", "mkv", "webm", "mov", "frec"];

/// Set (or clear, with `None`) a scene's backdrop wallpaper: any image, an
/// animated GIF, or a looping video — a pinned bottom layer the capture
/// always sits on top of. A video backdrop's audio strip starts muted so the
/// wallpaper never interferes with the program sound (unmute it in the mixer
/// to keep it).
#[tauri::command]
pub fn studio_set_scene_backdrop(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    path: Option<String>,
) -> Result<(), String> {
    let source = match path
        .map(|path| path.trim().to_string())
        .filter(|path| !path.is_empty())
    {
        None => None,
        Some(path) => {
            let ext = std::path::Path::new(&path)
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.to_ascii_lowercase())
                .unwrap_or_default();
            if BACKDROP_STILL_EXTS.contains(&ext.as_str()) {
                // Decode now so a bad pick errors in the dialog, not on air.
                fcap_sources::image::load_image_rgba(std::path::Path::new(&path))
                    .map_err(|err| err.to_string())?;
                Some(Source::new("Backdrop", SourceSettings::Image { path }))
            } else if BACKDROP_MEDIA_EXTS.contains(&ext.as_str()) {
                if !std::path::Path::new(&path).is_file() {
                    return Err(format!("media file not found: {path}"));
                }
                let mut source = Source::new(
                    "Backdrop",
                    SourceSettings::Media {
                        path,
                        looping: true,
                        hw_decode: true,
                        start_with_recording: false,
                        reverse: false,
                    },
                );
                if let Some(audio) = source.audio.as_mut() {
                    audio.muted = true;
                }
                Some(source)
            } else {
                return Err(format!(
                    "a backdrop can be an image, a GIF, or a video — not .{ext}"
                ));
            }
        }
    };
    state
        .mutate_tracked(&app, "setSceneBackdrop", None, |collection| {
            collection.set_scene_backdrop(scene_id, source)
        })
        .map(|_| ())
}

/// Move a scene's backdrop between the whole canvas and a half split (the
/// capture is seated into the other half — "video left, capture right" in
/// one click).
#[tauri::command]
pub fn studio_set_backdrop_split(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    split: BackdropSplit,
) -> Result<(), String> {
    state.mutate_tracked(&app, "setBackdropSplit", None, |collection| {
        collection.set_backdrop_split(scene_id, split)
    })
}

/// Toggle a video backdrop's "start playback with recording" hold: on, the
/// video shows its first frame and starts from the top the moment recording
/// begins (each take included); off, it just loops.
#[tauri::command]
pub fn studio_set_backdrop_sync(
    app: AppHandle,
    state: State<'_, StudioState>,
    scene_id: SceneId,
    start_with_recording: bool,
) -> Result<(), String> {
    state.mutate_tracked(&app, "setBackdropSync", None, |collection| {
        collection.set_backdrop_sync(scene_id, start_with_recording)
    })
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
///
/// Also the gate for CAP-M16's bound-text poll (`studio.rs` §5c) — an untrusted
/// collection must never make the render loop reach out to an attacker's host.
pub(crate) fn is_remote(path: &str) -> bool {
    path.contains("://") || path.starts_with("\\\\") || path.starts_with("//")
}

/// A best-effort LAN address for the CAP-N11 ingest URL/QR — the web
/// panel's probe, called rather than copied.
pub(crate) fn lan_ip() -> String {
    crate::webpanel::local_ip().unwrap_or_else(|| "127.0.0.1".to_owned())
}

/// CAP-N11: the address the ingest pickers show behind the URL + QR.
#[tauri::command]
pub fn local_lan_ip() -> String {
    lan_ip()
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
