//! # fcap-scene
//!
//! The **owned** scene / source / filter data model: scenes contain ordered
//! source items with z-order; sources live in a shared pool and are
//! referenced across scenes (rename one, every scene updates); each item
//! carries transform, blend, visibility, lock, and an ordered filter chain.
//!
//! [`Collection`] is the root aggregate **and** the on-disk scene-collection
//! project format (serde JSON) **and** the mutation surface the app, the
//! remote API, and scripting drive. Every mutation validates its targets and
//! keeps the collection's invariants:
//!
//! - there is always at least one scene, and `active_scene` always points at
//!   an existing scene;
//! - every item's `source` points at a source in the pool;
//! - a source with no referencing items anywhere is dropped from the pool.
//!
//! (Nested scene-as-a-source arrives with the Studio-Mode depth work.)

#![forbid(unsafe_code)]

pub mod filter;
pub mod scene;
pub mod source;

pub use filter::{Filter, FilterId, FilterKind, MaskMode};
pub use scene::{BlendMode, Crop, ItemId, Scene, SceneId, SceneItem, Transform};
pub use source::{Rgba, Source, SourceId, SourceSettings, TextAlign, VideoDeviceFormat};

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// This crate's version (inherited from the workspace).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The current on-disk format version (bumped only on breaking shape changes;
/// additive fields ride on serde defaults instead).
pub const FORMAT_VERSION: u32 = 1;

/// Why a mutation was refused. Every variant names the missing target — the
/// collection is never left half-mutated on error.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SceneError {
    #[error("scene not found")]
    SceneNotFound,
    #[error("source not found")]
    SourceNotFound,
    #[error("scene item not found")]
    ItemNotFound,
    #[error("filter not found")]
    FilterNotFound,
    #[error("a collection needs at least one scene")]
    LastScene,
}

fn default_format_version() -> u32 {
    FORMAT_VERSION
}

fn default_canvas_width() -> u32 {
    1920
}

fn default_canvas_height() -> u32 {
    1080
}

/// The largest canvas edge the model accepts — [`Collection::sanitize`]
/// clamps to this so a hand-edited file can never ask the GPU for a
/// 100 000 px program texture (the compositor additionally clamps to the
/// real adapter limit, which may be smaller).
pub const MAX_CANVAS_DIMENSION: u32 = 16_384;

/// The root of the model: canvas size, the shared source pool, the scenes,
/// and which scene is live. This is the project format on disk.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    #[serde(default = "default_format_version")]
    pub format_version: u32,
    #[serde(default = "default_canvas_width")]
    pub canvas_width: u32,
    #[serde(default = "default_canvas_height")]
    pub canvas_height: u32,
    /// The shared pool scene items reference by [`SourceId`].
    #[serde(default)]
    pub sources: Vec<Source>,
    #[serde(default)]
    pub scenes: Vec<Scene>,
    /// Always a valid scene id after any constructor or mutation.
    pub active_scene: SceneId,
}

impl Default for Collection {
    fn default() -> Self {
        Self::new()
    }
}

impl Collection {
    /// A fresh collection: one empty scene named "Scene", 1080p canvas.
    pub fn new() -> Self {
        let scene = Scene::new("Scene");
        let active = scene.id;
        Self {
            format_version: FORMAT_VERSION,
            canvas_width: default_canvas_width(),
            canvas_height: default_canvas_height(),
            sources: Vec::new(),
            scenes: vec![scene],
            active_scene: active,
        }
    }

    /// Restore the invariants on a freshly deserialized collection: at least
    /// one scene, a valid `active_scene`, no dangling item→source references,
    /// no unreferenced sources. A hand-edited or truncated file loads into
    /// something the engine can always run.
    pub fn sanitize(&mut self) {
        if self.canvas_width == 0 {
            self.canvas_width = default_canvas_width();
        }
        if self.canvas_height == 0 {
            self.canvas_height = default_canvas_height();
        }
        self.canvas_width = self.canvas_width.min(MAX_CANVAS_DIMENSION);
        self.canvas_height = self.canvas_height.min(MAX_CANVAS_DIMENSION);
        if self.scenes.is_empty() {
            self.scenes.push(Scene::new("Scene"));
        }
        if !self
            .scenes
            .iter()
            .any(|scene| scene.id == self.active_scene)
        {
            self.active_scene = self.scenes[0].id;
        }
        // Drop items pointing at sources that don't exist…
        let source_ids: Vec<SourceId> = self.sources.iter().map(|source| source.id).collect();
        for scene in &mut self.scenes {
            scene.items.retain(|item| source_ids.contains(&item.source));
        }
        // …then sources nothing references.
        self.gc_sources();
    }

    // -- scenes ------------------------------------------------------------

    pub fn scene(&self, id: SceneId) -> Option<&Scene> {
        self.scenes.iter().find(|scene| scene.id == id)
    }

    pub fn scene_mut(&mut self, id: SceneId) -> Option<&mut Scene> {
        self.scenes.iter_mut().find(|scene| scene.id == id)
    }

    /// The scene currently on program.
    pub fn active_scene(&self) -> &Scene {
        self.scene(self.active_scene)
            .expect("invariant: active_scene always exists")
    }

    /// Add a scene (auto-deduping the name: "Scene" → "Scene 2") and return
    /// its id. Does not change the active scene.
    pub fn add_scene(&mut self, name: &str) -> SceneId {
        let name = self.dedupe_scene_name(name);
        let scene = Scene::new(name);
        let id = scene.id;
        self.scenes.push(scene);
        id
    }

    pub fn rename_scene(&mut self, id: SceneId, name: &str) -> Result<(), SceneError> {
        let name = if name.trim().is_empty() {
            "Scene".to_string()
        } else if self
            .scenes
            .iter()
            .any(|scene| scene.id != id && scene.name == name)
        {
            self.dedupe_scene_name(name)
        } else {
            name.to_string()
        };
        let scene = self.scene_mut(id).ok_or(SceneError::SceneNotFound)?;
        scene.name = name;
        Ok(())
    }

    /// Remove a scene. Refuses to remove the last one; if the removed scene
    /// was active, the nearest remaining scene becomes active. Sources used
    /// only by this scene are dropped from the pool.
    pub fn remove_scene(&mut self, id: SceneId) -> Result<(), SceneError> {
        let index = self
            .scenes
            .iter()
            .position(|scene| scene.id == id)
            .ok_or(SceneError::SceneNotFound)?;
        if self.scenes.len() == 1 {
            return Err(SceneError::LastScene);
        }
        self.scenes.remove(index);
        if self.active_scene == id {
            self.active_scene = self.scenes[index.min(self.scenes.len() - 1)].id;
        }
        self.gc_sources();
        Ok(())
    }

    /// Move a scene to `to_index` in the rail (clamped).
    pub fn reorder_scene(&mut self, id: SceneId, to_index: usize) -> Result<(), SceneError> {
        let from = self
            .scenes
            .iter()
            .position(|scene| scene.id == id)
            .ok_or(SceneError::SceneNotFound)?;
        let to = to_index.min(self.scenes.len() - 1);
        let scene = self.scenes.remove(from);
        self.scenes.insert(to, scene);
        Ok(())
    }

    pub fn set_active_scene(&mut self, id: SceneId) -> Result<(), SceneError> {
        if self.scene(id).is_none() {
            return Err(SceneError::SceneNotFound);
        }
        self.active_scene = id;
        Ok(())
    }

    fn dedupe_scene_name(&self, base: &str) -> String {
        let base = if base.trim().is_empty() {
            "Scene"
        } else {
            base
        };
        if !self.scenes.iter().any(|scene| scene.name == base) {
            return base.to_string();
        }
        let mut counter = 2usize;
        loop {
            let candidate = format!("{base} {counter}");
            if !self.scenes.iter().any(|scene| scene.name == candidate) {
                return candidate;
            }
            counter += 1;
        }
    }

    // -- sources -----------------------------------------------------------

    pub fn source(&self, id: SourceId) -> Option<&Source> {
        self.sources.iter().find(|source| source.id == id)
    }

    pub fn source_mut(&mut self, id: SourceId) -> Option<&mut Source> {
        self.sources.iter_mut().find(|source| source.id == id)
    }

    /// Whether any scene shows this source.
    pub fn is_source_referenced(&self, id: SourceId) -> bool {
        self.scenes
            .iter()
            .any(|scene| scene.items.iter().any(|item| item.source == id))
    }

    /// Drop pool sources no item references (called by the removal paths).
    fn gc_sources(&mut self) {
        let referenced: Vec<SourceId> = self
            .scenes
            .iter()
            .flat_map(|scene| scene.items.iter().map(|item| item.source))
            .collect();
        self.sources
            .retain(|source| referenced.contains(&source.id));
    }

    pub fn rename_source(&mut self, id: SourceId, name: &str) -> Result<(), SceneError> {
        let source = self.source_mut(id).ok_or(SceneError::SourceNotFound)?;
        if !name.trim().is_empty() {
            source.name = name.to_string();
        }
        Ok(())
    }

    /// Replace a source's settings (the Properties dialog). The kind may
    /// change too — the engine treats that as a restart of that source.
    pub fn update_source_settings(
        &mut self,
        id: SourceId,
        settings: SourceSettings,
    ) -> Result<(), SceneError> {
        let source = self.source_mut(id).ok_or(SceneError::SourceNotFound)?;
        source.settings = settings;
        Ok(())
    }

    // -- items -------------------------------------------------------------

    /// Add a brand-new source to the pool and place it on top of `scene_id`.
    pub fn add_item_with_new_source(
        &mut self,
        scene_id: SceneId,
        source: Source,
    ) -> Result<(SourceId, ItemId), SceneError> {
        if self.scene(scene_id).is_none() {
            return Err(SceneError::SceneNotFound);
        }
        let source_id = source.id;
        self.sources.push(source);
        let item_id = self
            .add_item_with_existing_source(scene_id, source_id)
            .expect("scene and source were just verified");
        Ok((source_id, item_id))
    }

    /// Place an existing pool source on top of `scene_id` (source sharing).
    pub fn add_item_with_existing_source(
        &mut self,
        scene_id: SceneId,
        source_id: SourceId,
    ) -> Result<ItemId, SceneError> {
        if self.source(source_id).is_none() {
            return Err(SceneError::SourceNotFound);
        }
        let scene = self.scene_mut(scene_id).ok_or(SceneError::SceneNotFound)?;
        let item = SceneItem::new(source_id);
        let id = item.id;
        scene.items.push(item);
        Ok(id)
    }

    /// Remove an item; its source leaves the pool too if nothing else shows it.
    pub fn remove_item(&mut self, scene_id: SceneId, item_id: ItemId) -> Result<(), SceneError> {
        let scene = self.scene_mut(scene_id).ok_or(SceneError::SceneNotFound)?;
        let index = scene
            .items
            .iter()
            .position(|item| item.id == item_id)
            .ok_or(SceneError::ItemNotFound)?;
        scene.items.remove(index);
        self.gc_sources();
        Ok(())
    }

    /// Move an item to `to_index` in the z-order (clamped; 0 = bottom).
    pub fn reorder_item(
        &mut self,
        scene_id: SceneId,
        item_id: ItemId,
        to_index: usize,
    ) -> Result<(), SceneError> {
        let scene = self.scene_mut(scene_id).ok_or(SceneError::SceneNotFound)?;
        let from = scene
            .items
            .iter()
            .position(|item| item.id == item_id)
            .ok_or(SceneError::ItemNotFound)?;
        let to = to_index.min(scene.items.len() - 1);
        let item = scene.items.remove(from);
        scene.items.insert(to, item);
        Ok(())
    }

    fn item_mut(
        &mut self,
        scene_id: SceneId,
        item_id: ItemId,
    ) -> Result<&mut SceneItem, SceneError> {
        let scene = self.scene_mut(scene_id).ok_or(SceneError::SceneNotFound)?;
        scene.item_mut(item_id).ok_or(SceneError::ItemNotFound)
    }

    pub fn set_item_transform(
        &mut self,
        scene_id: SceneId,
        item_id: ItemId,
        transform: Transform,
    ) -> Result<(), SceneError> {
        let item = self.item_mut(scene_id, item_id)?;
        item.transform = transform;
        // A user-driven transform supersedes the first-frame auto-fit.
        item.pending_fit = false;
        Ok(())
    }

    pub fn set_item_visible(
        &mut self,
        scene_id: SceneId,
        item_id: ItemId,
        visible: bool,
    ) -> Result<(), SceneError> {
        self.item_mut(scene_id, item_id)?.visible = visible;
        Ok(())
    }

    pub fn set_item_locked(
        &mut self,
        scene_id: SceneId,
        item_id: ItemId,
        locked: bool,
    ) -> Result<(), SceneError> {
        self.item_mut(scene_id, item_id)?.locked = locked;
        Ok(())
    }

    pub fn set_item_blend(
        &mut self,
        scene_id: SceneId,
        item_id: ItemId,
        blend: BlendMode,
    ) -> Result<(), SceneError> {
        self.item_mut(scene_id, item_id)?.blend = blend;
        Ok(())
    }

    // -- filters -----------------------------------------------------------

    /// Append a filter to an item's chain; returns the new filter's id.
    pub fn add_filter(
        &mut self,
        scene_id: SceneId,
        item_id: ItemId,
        kind: FilterKind,
    ) -> Result<FilterId, SceneError> {
        let item = self.item_mut(scene_id, item_id)?;
        let filter = Filter::new(kind);
        let id = filter.id;
        item.filters.push(filter);
        Ok(id)
    }

    pub fn remove_filter(
        &mut self,
        scene_id: SceneId,
        item_id: ItemId,
        filter_id: FilterId,
    ) -> Result<(), SceneError> {
        let item = self.item_mut(scene_id, item_id)?;
        let index = item
            .filters
            .iter()
            .position(|filter| filter.id == filter_id)
            .ok_or(SceneError::FilterNotFound)?;
        item.filters.remove(index);
        Ok(())
    }

    /// Move a filter to `to_index` in the chain (clamped).
    pub fn reorder_filter(
        &mut self,
        scene_id: SceneId,
        item_id: ItemId,
        filter_id: FilterId,
        to_index: usize,
    ) -> Result<(), SceneError> {
        let item = self.item_mut(scene_id, item_id)?;
        let from = item
            .filters
            .iter()
            .position(|filter| filter.id == filter_id)
            .ok_or(SceneError::FilterNotFound)?;
        let to = to_index.min(item.filters.len() - 1);
        let filter = item.filters.remove(from);
        item.filters.insert(to, filter);
        Ok(())
    }

    /// Replace a filter's parameters (its id and position stay).
    pub fn update_filter(
        &mut self,
        scene_id: SceneId,
        item_id: ItemId,
        filter_id: FilterId,
        kind: FilterKind,
    ) -> Result<(), SceneError> {
        let item = self.item_mut(scene_id, item_id)?;
        let filter = item
            .filters
            .iter_mut()
            .find(|filter| filter.id == filter_id)
            .ok_or(SceneError::FilterNotFound)?;
        filter.kind = kind;
        Ok(())
    }

    pub fn set_filter_enabled(
        &mut self,
        scene_id: SceneId,
        item_id: ItemId,
        filter_id: FilterId,
        enabled: bool,
    ) -> Result<(), SceneError> {
        let item = self.item_mut(scene_id, item_id)?;
        let filter = item
            .filters
            .iter_mut()
            .find(|filter| filter.id == filter_id)
            .ok_or(SceneError::FilterNotFound)?;
        filter.enabled = enabled;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_a_semver_triple() {
        assert_eq!(
            VERSION.split('.').count(),
            3,
            "workspace version should be MAJOR.MINOR.PATCH"
        );
    }

    /// A collection exercising every source kind, filter kind, and
    /// non-default item state — the round-trip workhorse.
    fn full_collection() -> Collection {
        let mut collection = Collection::new();
        let scene_a = collection.active_scene;
        let scene_b = collection.add_scene("Gameplay");

        let (webcam, item_a) = collection
            .add_item_with_new_source(
                scene_a,
                Source::new(
                    "Face cam",
                    SourceSettings::VideoDevice {
                        device_id: "cam-0".into(),
                        format: Some(VideoDeviceFormat {
                            width: 1280,
                            height: 720,
                            fps: 30,
                            fourcc: "MJPG".into(),
                        }),
                    },
                ),
            )
            .expect("add webcam");
        // The same webcam shared into the second scene.
        collection
            .add_item_with_existing_source(scene_b, webcam)
            .expect("share webcam");

        for settings in [
            SourceSettings::Display {
                capture_id: "dxgi:0".into(),
                label: "Display 1 — 2560×1440".into(),
            },
            SourceSettings::Window {
                capture_id: "wgc:1234".into(),
                label: "App — Window".into(),
            },
            SourceSettings::Portal {},
            SourceSettings::Image {
                path: "C:/art/overlay.png".into(),
            },
            SourceSettings::Color {
                color: Rgba::new(10, 20, 30, 255),
                width: 640,
                height: 360,
            },
            SourceSettings::Text {
                text: "مرحبا Freally".into(),
                font_family: Some("Segoe UI".into()),
                font_file: None,
                size_px: 48.0,
                color: Rgba::WHITE,
                align: TextAlign::Center,
                line_spacing: 1.2,
                force_rtl: false,
                wrap_width: Some(800),
            },
        ] {
            let source = Source::new("", settings);
            collection
                .add_item_with_new_source(scene_a, source)
                .expect("add source");
        }

        // Every filter kind on the webcam item, plus non-default item state.
        for kind in [
            FilterKind::ChromaKey {
                key: Rgba::new(0, 255, 0, 255),
                similarity: 0.35,
                smoothness: 0.1,
                spill: 0.2,
            },
            FilterKind::ColorCorrection {
                gamma: 0.2,
                brightness: -0.05,
                contrast: 0.1,
                saturation: 1.3,
                hue_shift: 15.0,
                opacity: 0.9,
            },
            FilterKind::Lut {
                path: "C:/luts/warm.cube".into(),
                amount: 0.8,
            },
            FilterKind::Blur { radius: 12.0 },
            FilterKind::Mask {
                path: "C:/masks/rounded.png".into(),
                mode: MaskMode::Luma,
                invert: true,
            },
            FilterKind::Sharpen { amount: 0.5 },
            FilterKind::Scroll {
                speed_x: 120.0,
                speed_y: 0.0,
            },
            FilterKind::Crop {
                left: 8,
                top: 8,
                right: 8,
                bottom: 8,
            },
        ] {
            collection
                .add_filter(scene_a, item_a, kind)
                .expect("add filter");
        }

        collection
            .set_item_transform(
                scene_a,
                item_a,
                Transform {
                    x: 1600.0,
                    y: 900.0,
                    scale_x: 0.25,
                    scale_y: 0.25,
                    rotation: -8.5,
                    crop: Crop {
                        left: 10,
                        top: 0,
                        right: 10,
                        bottom: 4,
                    },
                },
            )
            .expect("set transform");
        collection
            .set_item_blend(scene_a, item_a, BlendMode::Screen)
            .expect("set blend");
        collection
            .set_item_locked(scene_a, item_a, true)
            .expect("lock");

        collection
    }

    #[test]
    fn round_trips_through_json() {
        let original = full_collection();
        let json = serde_json::to_string_pretty(&original).expect("serialize");
        let restored: Collection = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(original, restored, "the model must round-trip losslessly");
    }

    #[test]
    fn a_shared_source_updates_in_every_scene() {
        let mut collection = full_collection();
        let scene_a = collection.scenes[0].id;
        let scene_b = collection.scenes[1].id;
        let shared = collection.scenes[1].items[0].source;

        collection
            .rename_source(shared, "Sony a6400")
            .expect("rename");

        let via_a = collection.scene(scene_a).unwrap().items[0].source;
        let via_b = collection.scene(scene_b).unwrap().items[0].source;
        assert_eq!(via_a, via_b, "both scenes reference the same source id");
        assert_eq!(collection.source(via_a).unwrap().name, "Sony a6400");
    }

    #[test]
    fn removing_the_last_referencing_item_drops_the_source() {
        let mut collection = Collection::new();
        let scene = collection.active_scene;
        let (source, item) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "Block",
                    SourceSettings::Color {
                        color: Rgba::WHITE,
                        width: 64,
                        height: 64,
                    },
                ),
            )
            .expect("add");
        assert!(collection.is_source_referenced(source));

        collection.remove_item(scene, item).expect("remove");
        assert!(collection.source(source).is_none(), "unreferenced → GC'd");
    }

    #[test]
    fn a_source_shared_elsewhere_survives_item_removal() {
        let mut collection = Collection::new();
        let scene_a = collection.active_scene;
        let scene_b = collection.add_scene("B");
        let (source, item_a) = collection
            .add_item_with_new_source(
                scene_a,
                Source::new(
                    "Cam",
                    SourceSettings::VideoDevice {
                        device_id: "cam-1".into(),
                        format: None,
                    },
                ),
            )
            .expect("add");
        collection
            .add_item_with_existing_source(scene_b, source)
            .expect("share");

        collection.remove_item(scene_a, item_a).expect("remove");
        assert!(
            collection.source(source).is_some(),
            "still referenced by scene B"
        );
    }

    #[test]
    fn the_last_scene_cannot_be_removed() {
        let mut collection = Collection::new();
        let only = collection.active_scene;
        assert_eq!(collection.remove_scene(only), Err(SceneError::LastScene));
        assert_eq!(collection.scenes.len(), 1);
    }

    #[test]
    fn removing_the_active_scene_activates_a_neighbor() {
        let mut collection = Collection::new();
        let first = collection.active_scene;
        let second = collection.add_scene("Second");
        collection.remove_scene(first).expect("remove active");
        assert_eq!(collection.active_scene, second);
    }

    #[test]
    fn scene_names_dedupe() {
        let mut collection = Collection::new();
        collection.add_scene("Scene");
        let third = collection.add_scene("Scene");
        assert_eq!(collection.scene(third).unwrap().name, "Scene 3");
    }

    #[test]
    fn item_reorder_moves_and_clamps() {
        let mut collection = Collection::new();
        let scene = collection.active_scene;
        let mut ids = Vec::new();
        for i in 0..3 {
            let (_, item) = collection
                .add_item_with_new_source(
                    scene,
                    Source::new(
                        format!("c{i}"),
                        SourceSettings::Color {
                            color: Rgba::WHITE,
                            width: 8,
                            height: 8,
                        },
                    ),
                )
                .expect("add");
            ids.push(item);
        }
        // Move the bottom item to the top with an out-of-range index.
        collection
            .reorder_item(scene, ids[0], 99)
            .expect("reorder clamps");
        let order: Vec<ItemId> = collection
            .scene(scene)
            .unwrap()
            .items
            .iter()
            .map(|item| item.id)
            .collect();
        assert_eq!(order, vec![ids[1], ids[2], ids[0]]);
    }

    #[test]
    fn filter_lifecycle_add_update_reorder_remove() {
        let mut collection = Collection::new();
        let scene = collection.active_scene;
        let (_, item) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "Cam",
                    SourceSettings::VideoDevice {
                        device_id: "cam".into(),
                        format: None,
                    },
                ),
            )
            .expect("add");

        let blur = collection
            .add_filter(scene, item, FilterKind::Blur { radius: 4.0 })
            .expect("add blur");
        let sharpen = collection
            .add_filter(scene, item, FilterKind::Sharpen { amount: 0.3 })
            .expect("add sharpen");

        collection
            .update_filter(scene, item, blur, FilterKind::Blur { radius: 9.0 })
            .expect("update");
        collection
            .set_filter_enabled(scene, item, sharpen, false)
            .expect("disable");
        collection
            .reorder_filter(scene, item, sharpen, 0)
            .expect("reorder");

        let filters = &collection.scene(scene).unwrap().item(item).unwrap().filters;
        assert_eq!(filters.len(), 2);
        assert_eq!(filters[0].id, sharpen);
        assert!(!filters[0].enabled);
        assert_eq!(filters[1].kind, FilterKind::Blur { radius: 9.0 });

        collection.remove_filter(scene, item, blur).expect("remove");
        let filters = &collection.scene(scene).unwrap().item(item).unwrap().filters;
        assert_eq!(filters.len(), 1);
    }

    #[test]
    fn wire_format_uses_camel_case_fields() {
        // The UI bridge depends on camelCase FIELD names inside the tagged
        // enums — `rename_all` alone renames only the variant tags, and every
        // field's serde(default) would silently mask the mismatch.
        let source = Source::new(
            "cam",
            SourceSettings::VideoDevice {
                device_id: "cam-7".into(),
                format: None,
            },
        );
        let json = serde_json::to_string(&source).expect("serialize");
        assert!(json.contains("\"deviceId\":\"cam-7\""), "got: {json}");

        let parsed: SourceSettings =
            serde_json::from_str(r#"{"kind":"videoDevice","deviceId":"cam-7"}"#).expect("parse");
        assert_eq!(
            parsed,
            SourceSettings::VideoDevice {
                device_id: "cam-7".into(),
                format: None,
            }
        );

        let display: SourceSettings =
            serde_json::from_str(r#"{"kind":"display","captureId":"dxgi:0","label":"D1"}"#)
                .expect("parse");
        assert_eq!(
            display,
            SourceSettings::Display {
                capture_id: "dxgi:0".into(),
                label: "D1".into(),
            }
        );

        let text_json = serde_json::to_string(&SourceSettings::Text {
            text: "hi".into(),
            font_family: None,
            font_file: None,
            size_px: 48.0,
            color: Rgba::WHITE,
            align: TextAlign::Left,
            line_spacing: 1.0,
            force_rtl: false,
            wrap_width: None,
        })
        .expect("serialize");
        for key in [
            "\"sizePx\"",
            "\"fontFamily\"",
            "\"lineSpacing\"",
            "\"forceRtl\"",
            "\"wrapWidth\"",
        ] {
            assert!(text_json.contains(key), "missing {key} in {text_json}");
        }

        let scroll: FilterKind =
            serde_json::from_str(r#"{"type":"scroll","speedX":120.0,"speedY":-3.0}"#)
                .expect("parse");
        assert_eq!(
            scroll,
            FilterKind::Scroll {
                speed_x: 120.0,
                speed_y: -3.0,
            }
        );
        let cc_json = serde_json::to_string(&FilterKind::ColorCorrection {
            gamma: 0.0,
            brightness: 0.0,
            contrast: 0.0,
            saturation: 1.0,
            hue_shift: 15.0,
            opacity: 1.0,
        })
        .expect("serialize");
        assert!(cc_json.contains("\"hueShift\":15.0"), "got: {cc_json}");
    }

    #[test]
    fn sanitize_clamps_an_oversized_canvas() {
        let mut collection = Collection::new();
        collection.canvas_width = 100_000;
        collection.canvas_height = 90_000;
        collection.sanitize();
        assert_eq!(collection.canvas_width, MAX_CANVAS_DIMENSION);
        assert_eq!(collection.canvas_height, MAX_CANVAS_DIMENSION);
    }

    #[test]
    fn sanitize_repairs_a_broken_file() {
        let mut collection = full_collection();
        // Simulate hand-editing damage: dangling item reference, bad active id,
        // zero canvas.
        collection.sources.remove(0);
        collection.active_scene = SceneId::new();
        collection.canvas_width = 0;
        collection.sanitize();

        assert!(collection.canvas_width > 0);
        assert!(collection
            .scenes
            .iter()
            .any(|scene| scene.id == collection.active_scene));
        for scene in &collection.scenes {
            for item in &scene.items {
                assert!(
                    collection.source(item.source).is_some(),
                    "no dangling source refs after sanitize"
                );
            }
        }
    }

    #[test]
    fn unknown_json_keys_are_tolerated() {
        let json = r#"{
            "formatVersion": 1,
            "canvasWidth": 1920,
            "canvasHeight": 1080,
            "sources": [],
            "scenes": [{
                "id": "6f8fcfa4-3a3e-4b6e-9d5a-111111111111",
                "name": "Scene",
                "items": [],
                "someFutureField": true
            }],
            "activeScene": "6f8fcfa4-3a3e-4b6e-9d5a-111111111111",
            "someFutureTopLevel": {"x": 1}
        }"#;
        let collection: Collection = serde_json::from_str(json).expect("tolerant parse");
        assert_eq!(collection.scenes.len(), 1);
    }

    #[test]
    fn set_transform_clears_pending_fit() {
        let mut collection = Collection::new();
        let scene = collection.active_scene;
        let (_, item) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "Cam",
                    SourceSettings::VideoDevice {
                        device_id: "cam".into(),
                        format: None,
                    },
                ),
            )
            .expect("add");
        assert!(
            collection
                .scene(scene)
                .unwrap()
                .item(item)
                .unwrap()
                .pending_fit
        );
        collection
            .set_item_transform(scene, item, Transform::default())
            .expect("set");
        assert!(
            !collection
                .scene(scene)
                .unwrap()
                .item(item)
                .unwrap()
                .pending_fit
        );
    }
}
