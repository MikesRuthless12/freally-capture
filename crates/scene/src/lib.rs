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

pub mod audio;
pub mod filter;
pub mod scene;
pub mod source;

pub use audio::{
    AudioFilter, AudioFilterId, AudioFilterKind, AudioSettings, MonitorMode, MAX_SYNC_OFFSET_MS,
    MAX_VOLUME_DB, MIN_VOLUME_DB, TRACK_COUNT,
};
pub use filter::{Filter, FilterId, FilterKind, MaskMode};
pub use scene::{BlendMode, Corner, Crop, ItemId, NormRect, Scene, SceneId, SceneItem, Transform};
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
    #[error("this source has no audio")]
    SourceNotAudio,
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
        // Audio state exists exactly on audio-capable sources, inside range.
        for source in &mut self.sources {
            if source.settings.has_audio() {
                source
                    .audio
                    .get_or_insert_with(AudioSettings::default)
                    .clamp();
            } else {
                source.audio = None;
            }
        }
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
        // A user-driven transform supersedes any pending first-frame placement.
        item.pending_fit = false;
        item.pending_slot = None;
        Ok(())
    }

    /// Arrange a scene as a centered screen with up to four corner cameras —
    /// the screen-plus-corners layout (host + up to three guests). `center`
    /// fills the canvas as the backdrop (bottom of the z-order); each
    /// `(item, corner)` fits into its corner on top. Placement resolves on
    /// each source's next sized frame — the same first-frame mechanism as a
    /// freshly added item — so any camera resolution lands correctly. Every
    /// referenced item must already live in the scene; a bad id is a no-op.
    pub fn apply_layout(
        &mut self,
        scene_id: SceneId,
        center: Option<ItemId>,
        corners: &[(ItemId, Corner)],
    ) -> Result<(), SceneError> {
        // Validate every id first so a bad reference changes nothing.
        let scene = self.scene(scene_id).ok_or(SceneError::SceneNotFound)?;
        let present = |id: ItemId| scene.items.iter().any(|item| item.id == id);
        let unknown =
            center.is_some_and(|c| !present(c)) || corners.iter().any(|(id, _)| !present(*id));
        if unknown {
            return Err(SceneError::ItemNotFound);
        }

        // Stage placement intents (resolved on each source's next sized frame).
        if let Some(center) = center {
            let item = self.item_mut(scene_id, center)?;
            item.pending_fit = true;
            item.pending_slot = None; // whole-canvas fit (never upscales)
        }
        for (id, corner) in corners {
            let item = self.item_mut(scene_id, *id)?;
            item.pending_fit = true;
            item.pending_slot = Some(corner.slot());
        }

        // Z-order: screen at the bottom, corners above it in order; any other
        // items (overlays) stay on top.
        let mut index = 0;
        if let Some(center) = center {
            self.reorder_item(scene_id, center, index)?;
            index += 1;
        }
        for (id, _) in corners {
            self.reorder_item(scene_id, *id, index)?;
            index += 1;
        }
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

    // -- audio (per-source mixer state) --------------------------------------

    /// The mutable audio strip of an audio-capable source.
    fn audio_mut(&mut self, id: SourceId) -> Result<&mut AudioSettings, SceneError> {
        let source = self.source_mut(id).ok_or(SceneError::SourceNotFound)?;
        if !source.settings.has_audio() {
            return Err(SceneError::SourceNotAudio);
        }
        Ok(source.audio.get_or_insert_with(AudioSettings::default))
    }

    /// Set the fader, clamped to [`MIN_VOLUME_DB`]..=[`MAX_VOLUME_DB`]
    /// (non-finite input resets to unity).
    pub fn set_audio_volume(&mut self, id: SourceId, volume_db: f32) -> Result<(), SceneError> {
        let audio = self.audio_mut(id)?;
        audio.volume_db = if volume_db.is_finite() {
            volume_db.clamp(MIN_VOLUME_DB, MAX_VOLUME_DB)
        } else {
            0.0
        };
        Ok(())
    }

    pub fn set_audio_muted(&mut self, id: SourceId, muted: bool) -> Result<(), SceneError> {
        self.audio_mut(id)?.muted = muted;
        Ok(())
    }

    pub fn set_audio_monitor(
        &mut self,
        id: SourceId,
        monitor: MonitorMode,
    ) -> Result<(), SceneError> {
        self.audio_mut(id)?.monitor = monitor;
        Ok(())
    }

    /// Set the track-assignment bitmask (bits past track 6 are dropped).
    pub fn set_audio_tracks(&mut self, id: SourceId, tracks: u8) -> Result<(), SceneError> {
        self.audio_mut(id)?.tracks = tracks & 0b0011_1111;
        Ok(())
    }

    /// Set the A/V sync offset, clamped to 0..=[`MAX_SYNC_OFFSET_MS`].
    pub fn set_audio_sync_offset(&mut self, id: SourceId, ms: u32) -> Result<(), SceneError> {
        self.audio_mut(id)?.sync_offset_ms = ms.min(MAX_SYNC_OFFSET_MS);
        Ok(())
    }

    /// Set the push-to-talk / push-to-mute hotkeys (blank strings clear).
    pub fn set_audio_hotkeys(
        &mut self,
        id: SourceId,
        push_to_talk: Option<String>,
        push_to_mute: Option<String>,
    ) -> Result<(), SceneError> {
        let audio = self.audio_mut(id)?;
        audio.push_to_talk = push_to_talk.filter(|key| !key.trim().is_empty());
        audio.push_to_mute = push_to_mute.filter(|key| !key.trim().is_empty());
        Ok(())
    }

    /// Append an audio filter to a source's chain; returns the new filter's id.
    pub fn add_audio_filter(
        &mut self,
        id: SourceId,
        kind: AudioFilterKind,
    ) -> Result<AudioFilterId, SceneError> {
        let audio = self.audio_mut(id)?;
        let filter = AudioFilter::new(kind);
        let filter_id = filter.id;
        audio.filters.push(filter);
        Ok(filter_id)
    }

    pub fn remove_audio_filter(
        &mut self,
        id: SourceId,
        filter_id: AudioFilterId,
    ) -> Result<(), SceneError> {
        let audio = self.audio_mut(id)?;
        let index = audio
            .filters
            .iter()
            .position(|filter| filter.id == filter_id)
            .ok_or(SceneError::FilterNotFound)?;
        audio.filters.remove(index);
        Ok(())
    }

    /// Move an audio filter to `to_index` in the chain (clamped).
    pub fn reorder_audio_filter(
        &mut self,
        id: SourceId,
        filter_id: AudioFilterId,
        to_index: usize,
    ) -> Result<(), SceneError> {
        let audio = self.audio_mut(id)?;
        let from = audio
            .filters
            .iter()
            .position(|filter| filter.id == filter_id)
            .ok_or(SceneError::FilterNotFound)?;
        let to = to_index.min(audio.filters.len() - 1);
        let filter = audio.filters.remove(from);
        audio.filters.insert(to, filter);
        Ok(())
    }

    /// Replace an audio filter's parameters (its id and position stay).
    pub fn update_audio_filter(
        &mut self,
        id: SourceId,
        filter_id: AudioFilterId,
        kind: AudioFilterKind,
    ) -> Result<(), SceneError> {
        let audio = self.audio_mut(id)?;
        let filter = audio
            .filters
            .iter_mut()
            .find(|filter| filter.id == filter_id)
            .ok_or(SceneError::FilterNotFound)?;
        filter.kind = kind;
        Ok(())
    }

    pub fn set_audio_filter_enabled(
        &mut self,
        id: SourceId,
        filter_id: AudioFilterId,
        enabled: bool,
    ) -> Result<(), SceneError> {
        let audio = self.audio_mut(id)?;
        let filter = audio
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

        // Audio sources with a non-default strip + every audio filter kind.
        let (mic, _) = collection
            .add_item_with_new_source(
                scene_a,
                Source::new(
                    "Mic",
                    SourceSettings::AudioInput {
                        device_id: "USB Microphone".into(),
                    },
                ),
            )
            .expect("add mic");
        let (desktop, _) = collection
            .add_item_with_new_source(
                scene_a,
                Source::new(
                    "Desktop",
                    SourceSettings::AudioOutput {
                        device_id: String::new(),
                    },
                ),
            )
            .expect("add desktop audio");
        for kind in [
            AudioFilterKind::Denoise { strength: 0.7 },
            AudioFilterKind::NoiseGate {
                open_threshold_db: -24.0,
                close_threshold_db: -30.0,
                attack_ms: 20.0,
                hold_ms: 150.0,
                release_ms: 100.0,
            },
            AudioFilterKind::Compressor {
                ratio: 3.0,
                threshold_db: -16.0,
                attack_ms: 5.0,
                release_ms: 80.0,
                output_gain_db: 2.0,
            },
            AudioFilterKind::Limiter {
                threshold_db: -2.0,
                release_ms: 50.0,
            },
            AudioFilterKind::Eq {
                low_db: 2.0,
                mid_db: -1.5,
                high_db: 3.0,
            },
            AudioFilterKind::Gain { db: -3.0 },
        ] {
            collection.add_audio_filter(mic, kind).expect("add filter");
        }
        collection
            .add_audio_filter(
                desktop,
                AudioFilterKind::Ducker {
                    trigger: Some(mic),
                    threshold_db: -28.0,
                    amount_db: 10.0,
                    attack_ms: 40.0,
                    release_ms: 250.0,
                },
            )
            .expect("add ducker");
        collection.set_audio_volume(mic, -6.5).expect("volume");
        collection
            .set_audio_monitor(mic, MonitorMode::MonitorAndOutput)
            .expect("monitor");
        collection.set_audio_tracks(mic, 0b000011).expect("tracks");
        collection.set_audio_sync_offset(mic, 120).expect("sync");
        collection
            .set_audio_hotkeys(mic, Some("Ctrl+Shift+T".into()), None)
            .expect("hotkeys");
        collection.set_audio_muted(desktop, true).expect("mute");

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
    fn audio_wire_format_uses_camel_case_fields() {
        let source = Source::new(
            "Mic",
            SourceSettings::AudioInput {
                device_id: "USB Microphone".into(),
            },
        );
        let json = serde_json::to_string(&source).expect("serialize");
        assert!(json.contains("\"kind\":\"audioInput\""), "got: {json}");
        assert!(
            json.contains("\"deviceId\":\"USB Microphone\""),
            "got: {json}"
        );
        for key in [
            "\"volumeDb\"",
            "\"syncOffsetMs\"",
            "\"tracks\"",
            "\"monitor\"",
        ] {
            assert!(json.contains(key), "missing {key} in {json}");
        }
        assert!(
            !json.contains("pushToTalk"),
            "unset hotkeys stay off the wire: {json}"
        );

        let parsed: SourceSettings =
            serde_json::from_str(r#"{"kind":"audioOutput","deviceId":"Speakers"}"#).expect("parse");
        assert_eq!(
            parsed,
            SourceSettings::AudioOutput {
                device_id: "Speakers".into(),
            }
        );

        let gate: AudioFilterKind = serde_json::from_str(
            r#"{"type":"noiseGate","openThresholdDb":-20.0,"closeThresholdDb":-26.0}"#,
        )
        .expect("parse gate");
        match gate {
            AudioFilterKind::NoiseGate {
                open_threshold_db,
                close_threshold_db,
                ..
            } => {
                assert_eq!(open_threshold_db, -20.0);
                assert_eq!(close_threshold_db, -26.0);
            }
            other => panic!("wrong kind: {other:?}"),
        }
        let ducker_json = serde_json::to_string(&AudioFilterKind::Ducker {
            trigger: None,
            threshold_db: -30.0,
            amount_db: 12.0,
            attack_ms: 50.0,
            release_ms: 300.0,
        })
        .expect("serialize ducker");
        for key in [
            "\"thresholdDb\"",
            "\"amountDb\"",
            "\"attackMs\"",
            "\"releaseMs\"",
        ] {
            assert!(ducker_json.contains(key), "missing {key} in {ducker_json}");
        }
        let monitor_json =
            serde_json::to_string(&MonitorMode::MonitorAndOutput).expect("serialize monitor");
        assert_eq!(monitor_json, "\"monitorAndOutput\"");
    }

    #[test]
    fn audio_mutations_clamp_and_validate() {
        let mut collection = Collection::new();
        let scene = collection.active_scene;
        let (mic, _) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "Mic",
                    SourceSettings::AudioInput {
                        device_id: String::new(),
                    },
                ),
            )
            .expect("add mic");
        let (block, _) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "Block",
                    SourceSettings::Color {
                        color: Rgba::WHITE,
                        width: 8,
                        height: 8,
                    },
                ),
            )
            .expect("add color");

        collection.set_audio_volume(mic, -200.0).expect("volume");
        assert_eq!(
            collection
                .source(mic)
                .unwrap()
                .audio
                .as_ref()
                .unwrap()
                .volume_db,
            MIN_VOLUME_DB
        );
        collection
            .set_audio_volume(mic, f32::INFINITY)
            .expect("volume");
        assert_eq!(
            collection
                .source(mic)
                .unwrap()
                .audio
                .as_ref()
                .unwrap()
                .volume_db,
            0.0
        );
        collection.set_audio_tracks(mic, 0xFF).expect("tracks");
        assert_eq!(
            collection
                .source(mic)
                .unwrap()
                .audio
                .as_ref()
                .unwrap()
                .tracks,
            0b0011_1111
        );
        collection.set_audio_sync_offset(mic, 5_000).expect("sync");
        assert_eq!(
            collection
                .source(mic)
                .unwrap()
                .audio
                .as_ref()
                .unwrap()
                .sync_offset_ms,
            MAX_SYNC_OFFSET_MS
        );
        collection
            .set_audio_hotkeys(mic, Some("  ".into()), Some("F13".into()))
            .expect("hotkeys");
        let audio = collection.source(mic).unwrap().audio.as_ref().unwrap();
        assert_eq!(audio.push_to_talk, None, "blank hotkeys clear");
        assert_eq!(audio.push_to_mute.as_deref(), Some("F13"));

        // A video-only source refuses audio mutations without mutating.
        assert_eq!(
            collection.set_audio_muted(block, true),
            Err(SceneError::SourceNotAudio)
        );
        assert!(collection.source(block).unwrap().audio.is_none());
    }

    #[test]
    fn audio_filter_lifecycle_add_update_reorder_remove() {
        let mut collection = Collection::new();
        let scene = collection.active_scene;
        let (mic, _) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "Mic",
                    SourceSettings::AudioInput {
                        device_id: String::new(),
                    },
                ),
            )
            .expect("add mic");

        let denoise = collection
            .add_audio_filter(mic, AudioFilterKind::Denoise { strength: 0.5 })
            .expect("add denoise");
        let gain = collection
            .add_audio_filter(mic, AudioFilterKind::Gain { db: 3.0 })
            .expect("add gain");

        collection
            .update_audio_filter(mic, gain, AudioFilterKind::Gain { db: -6.0 })
            .expect("update");
        collection
            .set_audio_filter_enabled(mic, denoise, false)
            .expect("disable");
        collection
            .reorder_audio_filter(mic, gain, 0)
            .expect("reorder");

        let filters = &collection
            .source(mic)
            .unwrap()
            .audio
            .as_ref()
            .unwrap()
            .filters;
        assert_eq!(filters.len(), 2);
        assert_eq!(filters[0].id, gain);
        assert_eq!(filters[0].kind, AudioFilterKind::Gain { db: -6.0 });
        assert!(!filters[1].enabled);

        collection
            .remove_audio_filter(mic, denoise)
            .expect("remove");
        assert_eq!(
            collection
                .source(mic)
                .unwrap()
                .audio
                .as_ref()
                .unwrap()
                .filters
                .len(),
            1
        );
        assert_eq!(
            collection.remove_audio_filter(mic, denoise),
            Err(SceneError::FilterNotFound)
        );
    }

    #[test]
    fn sanitize_repairs_audio_state() {
        let mut collection = Collection::new();
        let scene = collection.active_scene;
        let (mic, _) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "Mic",
                    SourceSettings::AudioInput {
                        device_id: String::new(),
                    },
                ),
            )
            .expect("add mic");
        let (block, _) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "Block",
                    SourceSettings::Color {
                        color: Rgba::WHITE,
                        width: 8,
                        height: 8,
                    },
                ),
            )
            .expect("add color");

        // Simulate hand-editing damage: audio stripped from the mic, junk
        // audio added to a video source, out-of-range values.
        collection.source_mut(mic).unwrap().audio = None;
        collection.source_mut(block).unwrap().audio = Some(AudioSettings {
            volume_db: 99.0,
            ..AudioSettings::default()
        });
        collection.sanitize();

        assert_eq!(
            collection.source(mic).unwrap().audio,
            Some(AudioSettings::default()),
            "audio kinds regain their strip"
        );
        assert!(
            collection.source(block).unwrap().audio.is_none(),
            "video kinds shed junk audio state"
        );
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
    fn apply_layout_seats_the_screen_and_corners() {
        let mut collection = Collection::new();
        let scene = collection.active_scene;
        let (_, screen) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "Screen",
                    SourceSettings::Display {
                        capture_id: String::new(),
                        label: String::new(),
                    },
                ),
            )
            .expect("add screen");
        let (_, cam1) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "Cam 1",
                    SourceSettings::VideoDevice {
                        device_id: String::new(),
                        format: None,
                    },
                ),
            )
            .expect("add cam1");
        let (_, cam2) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "Cam 2",
                    SourceSettings::VideoDevice {
                        device_id: String::new(),
                        format: None,
                    },
                ),
            )
            .expect("add cam2");

        collection
            .apply_layout(
                scene,
                Some(screen),
                &[(cam2, Corner::TopRight), (cam1, Corner::TopLeft)],
            )
            .expect("apply layout");

        let scene_ref = collection.scene(scene).expect("scene");
        // Screen sits at the bottom; corners follow in the given order.
        assert_eq!(scene_ref.items[0].id, screen);
        assert_eq!(scene_ref.items[1].id, cam2);
        assert_eq!(scene_ref.items[2].id, cam1);
        // The screen fills the canvas (no slot); each camera takes its corner.
        let screen_item = scene_ref.item(screen).expect("screen item");
        assert!(screen_item.pending_fit);
        assert_eq!(screen_item.pending_slot, None);
        assert_eq!(
            scene_ref.item(cam2).expect("cam2").pending_slot,
            Some(Corner::TopRight.slot())
        );
        assert_eq!(
            scene_ref.item(cam1).expect("cam1").pending_slot,
            Some(Corner::TopLeft.slot())
        );
    }

    #[test]
    fn apply_layout_rejects_an_unknown_item_without_mutating() {
        let mut collection = Collection::new();
        let scene = collection.active_scene;
        let (_, screen) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "Screen",
                    SourceSettings::Display {
                        capture_id: String::new(),
                        label: String::new(),
                    },
                ),
            )
            .expect("add screen");
        let before = collection.scene(scene).expect("scene").items.clone();

        let result =
            collection.apply_layout(scene, Some(screen), &[(ItemId::new(), Corner::TopLeft)]);

        assert!(matches!(result, Err(SceneError::ItemNotFound)));
        assert_eq!(
            collection.scene(scene).expect("scene").items,
            before,
            "a bad id leaves the scene untouched"
        );
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
