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
pub mod history;
pub mod obs_import;
pub mod scene;
pub mod source;

pub use audio::{
    AudioFilter, AudioFilterId, AudioFilterKind, AudioSettings, MonitorMode, MAX_SYNC_OFFSET_MS,
    MAX_VOLUME_DB, MIN_VOLUME_DB, TRACK_COUNT,
};
pub use filter::{Filter, FilterId, FilterKind, MaskMode};
pub use history::{History, HistoryState};
pub use obs_import::{
    import_obs, ImportNote, ImportReport, ImportedSource, ObsImport, ObsImportError, SkipReason,
    SkippedSource,
};
pub use scene::{
    BackdropSplit, BlendMode, Corner, Crop, FocusRestore, FocusState, GroupId, GuideLine,
    GuideOrientation, ItemId, NormRect, ScaleMode, Scene, SceneAudioOverride, SceneId, SceneItem,
    SourceGroup, Transform, TransitionKind,
};
pub use source::{
    CountdownEnd, DeinterlaceMode, FieldOrder, FileBinding, IngestProtocol, InputLayout,
    PlaylistEntry, ReplaySpeed, Rgba, Source, SourceId, SourceSettings, SplitComparison, TextAlign,
    TimerMode, TitleAnimation, TitleLayer, VideoDeviceFormat, VisStyle, VisTargetKind,
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// This crate's version (inherited from the workspace).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The current on-disk format version (bumped only on breaking shape changes;
/// additive fields ride on serde defaults instead).
pub const FORMAT_VERSION: u32 = 1;

/// The most custom alignment guides one scene keeps (CAP-M04 follow-on). Bounds
/// an untrusted `set_guides`, and stays under the native overlay's 16-guide
/// budget with headroom for the live snap guides pushed alongside it — so every
/// custom guide renders on the native GPU path, not just the SVG fallback.
pub const MAX_GUIDES_PER_SCENE: usize = 12;

/// Two seats match approximately: the webview sends f64 JSON that rounds to
/// f32 a ULP differently than Rust-side arithmetic, so exact equality would
/// miss an occupied seat.
fn same_seat(a: NormRect, b: NormRect) -> bool {
    const EPS: f32 = 1e-4;
    (a.x - b.x).abs() < EPS
        && (a.y - b.y).abs() < EPS
        && (a.w - b.w).abs() < EPS
        && (a.h - b.h).abs() < EPS
}

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
    #[error("slot outside the canvas (normalized 0..=1)")]
    InvalidSlot,
    #[error("a nested scene cannot contain itself (directly or through other scenes)")]
    SceneCycle,
    #[error("group not found")]
    GroupNotFound,
    #[error("an item can belong to only one group")]
    AlreadyGrouped,
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
    /// CAP-N73 (runtime-only): linked app-audio strips this collection
    /// auto-muted because their window was hidden. Showing the window unmutes
    /// only these — never a strip the operator muted by hand. Never persisted.
    #[serde(default, skip)]
    pub hidden_muted: std::collections::HashSet<SourceId>,
    /// The optional second output canvas (Phase 6, TASK-604): e.g. a
    /// vertical 9:16 feed composed from any scene in the collection,
    /// recordable/streamable independently of the program canvas.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vertical: Option<VerticalCanvas>,
    /// Downstream keyer layers (CAP-N24): overlays composited on the PROGRAM
    /// output, above every scene, surviving scene cuts (a station logo, a LIVE
    /// badge, a persistent lower-third). Drawn bottom-to-top in list order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub downstream: Vec<DownstreamKeyer>,
}

/// One downstream-keyer layer (CAP-N24): a source composited over the program
/// at its own transform and opacity, unaffected by scene switches.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownstreamKeyer {
    pub id: DskId,
    /// The overlay's source (from the shared pool).
    pub source: SourceId,
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Layer opacity, 0..=1.
    #[serde(default = "default_one")]
    pub opacity: f32,
    #[serde(default)]
    pub transform: Transform,
}

/// Stable id for a [`DownstreamKeyer`], so reorder/remove/edit target one layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DskId(pub uuid::Uuid);

impl DskId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl Default for DskId {
    fn default() -> Self {
        Self::new()
    }
}

fn default_true() -> bool {
    true
}

fn default_one() -> f32 {
    1.0
}

/// The second canvas: its own size + the scene it composes. Item transforms
/// are canvas-pixel-based, so an item sits at the same pixel spot on either
/// canvas — arrange the chosen scene for these dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerticalCanvas {
    pub width: u32,
    pub height: u32,
    /// The scene this canvas composes (independent of the program scene).
    pub scene: SceneId,
}

/// What kind of thing references a file path (for the missing-file doctor,
/// CAP-M03). Purely informational — relinking is by path, not by kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FileRefKind {
    Image,
    Media,
    Slideshow,
    Font,
    Lut,
    Mask,
}

/// One file path the collection references, tagged with what points at it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileRef {
    pub path: String,
    pub kind: FileRefKind,
    /// The source that carries this path (for a filter path, the item's source).
    pub source: SourceId,
    pub source_name: String,
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
            hidden_muted: std::collections::HashSet::new(),
            vertical: None,
            downstream: Vec::new(),
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
        // The vertical canvas must point at a real scene with sane bounds.
        if let Some(vertical) = &mut self.vertical {
            vertical.width = vertical.width.clamp(1, MAX_CANVAS_DIMENSION);
            vertical.height = vertical.height.clamp(1, MAX_CANVAS_DIMENSION);
            let scene = vertical.scene;
            if !self.scenes.iter().any(|s| s.id == scene) {
                self.vertical = None;
            }
        }
        // Nested-scene sources with a missing target render nothing forever —
        // drop them (their items go with the dangling-source pass below).
        let scene_ids: Vec<SceneId> = self.scenes.iter().map(|scene| scene.id).collect();
        self.sources.retain(|source| {
            !matches!(&source.settings, SourceSettings::NestedScene { scene } if !scene_ids.contains(scene))
        });
        // A hand-edited file could hold a nested-scene cycle the engine must
        // never walk: drop any nested source whose target reaches back to a
        // scene showing it.
        let cyclic: Vec<SourceId> = self
            .sources
            .iter()
            .filter_map(|source| match &source.settings {
                SourceSettings::NestedScene { scene: target } => {
                    let cycles = self.scenes.iter().any(|holder| {
                        holder.items.iter().any(|item| item.source == source.id)
                            && (*target == holder.id || self.scene_reaches(*target, holder.id))
                    });
                    cycles.then_some(source.id)
                }
                _ => None,
            })
            .collect();
        self.sources.retain(|source| !cyclic.contains(&source.id));
        // Drop items pointing at sources that don't exist…
        let source_ids: Vec<SourceId> = self.sources.iter().map(|source| source.id).collect();
        for scene in &mut self.scenes {
            scene.items.retain(|item| source_ids.contains(&item.source));
            // Groups only ever reference live items; empty groups go.
            scene.prune_groups();
        }
        // Per-scene mixer overrides stay bounded and only on audio sources.
        let audio_ids: Vec<SourceId> = self
            .sources
            .iter()
            .filter(|source| source.settings.has_audio())
            .map(|source| source.id)
            .collect();
        for scene in &mut self.scenes {
            scene
                .audio_overrides
                .retain(|entry| audio_ids.contains(&entry.source));
            for entry in &mut scene.audio_overrides {
                entry.volume_db = entry.volume_db.clamp(MIN_VOLUME_DB, MAX_VOLUME_DB);
            }
        }
        // Downstream keyers (CAP-N24): drop layers whose source vanished and
        // clamp opacity; a live layer keeps its source alive through the gc.
        let live_sources: std::collections::HashSet<SourceId> =
            self.sources.iter().map(|source| source.id).collect();
        self.downstream
            .retain(|dsk| live_sources.contains(&dsk.source));
        for dsk in &mut self.downstream {
            dsk.opacity = dsk.opacity.clamp(0.0, 1.0);
        }
        // …then sources nothing references.
        self.gc_sources();
        // A focus pointing at a vanished item could never be toggled off from
        // the UI — restore its snapshot and drop it.
        let dangling_focus: Vec<SceneId> = self
            .scenes
            .iter()
            .filter(|scene| {
                scene
                    .focus
                    .as_ref()
                    .is_some_and(|focus| !scene.items.iter().any(|item| item.id == focus.item))
            })
            .map(|scene| scene.id)
            .collect();
        for scene_id in dangling_focus {
            let _ = self.clear_focus(scene_id);
        }
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
        if self.vertical.is_some_and(|vertical| vertical.scene == id) {
            self.vertical = None; // its scene is gone — the canvas goes dark honestly
        }
        // Nested-scene sources pointing at the removed scene die with it.
        let dead: Vec<SourceId> = self
            .sources
            .iter()
            .filter(|source| {
                matches!(&source.settings, SourceSettings::NestedScene { scene } if *scene == id)
            })
            .map(|source| source.id)
            .collect();
        if !dead.is_empty() {
            for scene in &mut self.scenes {
                scene.items.retain(|item| !dead.contains(&item.source));
                scene.prune_groups();
            }
        }
        self.gc_sources();
        Ok(())
    }

    /// Configure (or clear, with `None`) the second output canvas
    /// (Phase 6, TASK-604). The scene must exist; dimensions clamp.
    pub fn set_vertical(&mut self, vertical: Option<VerticalCanvas>) -> Result<(), SceneError> {
        match vertical {
            None => {
                self.vertical = None;
                Ok(())
            }
            Some(mut config) => {
                if !self.scenes.iter().any(|scene| scene.id == config.scene) {
                    return Err(SceneError::SceneNotFound);
                }
                config.width = config.width.clamp(1, MAX_CANVAS_DIMENSION);
                config.height = config.height.clamp(1, MAX_CANVAS_DIMENSION);
                self.vertical = Some(config);
                Ok(())
            }
        }
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
            || self.downstream.iter().any(|dsk| dsk.source == id)
    }

    // -- downstream keyers (CAP-N24) ---------------------------------------

    /// Add a downstream-keyer layer for `source` (full-canvas, opaque, enabled),
    /// on top of the existing layers. Returns its id, or `None` if the source
    /// does not exist.
    pub fn add_downstream(&mut self, source: SourceId) -> Option<DskId> {
        self.source(source)?; // the source must exist in the pool
        let id = DskId::new();
        // Centered at native size — the operator then positions/sizes it (a
        // logo bug, say, gets dragged to a corner and scaled down).
        self.downstream.push(DownstreamKeyer {
            id,
            source,
            enabled: true,
            opacity: 1.0,
            transform: Transform {
                x: self.canvas_width as f32 * 0.5,
                y: self.canvas_height as f32 * 0.5,
                ..Transform::default()
            },
        });
        Some(id)
    }

    /// Remove a keyer layer. The now-unreferenced source is GC'd by [`sanitize`].
    pub fn remove_downstream(&mut self, id: DskId) -> bool {
        let before = self.downstream.len();
        self.downstream.retain(|dsk| dsk.id != id);
        self.downstream.len() != before
    }

    /// Mutable access to one keyer layer, for the targeted setters/commands.
    pub fn downstream_mut(&mut self, id: DskId) -> Option<&mut DownstreamKeyer> {
        self.downstream.iter_mut().find(|dsk| dsk.id == id)
    }

    /// Move a keyer one step in the draw order (`up` = later/on top). No-op at
    /// the end. Returns whether anything moved.
    pub fn move_downstream(&mut self, id: DskId, up: bool) -> bool {
        let Some(index) = self.downstream.iter().position(|dsk| dsk.id == id) else {
            return false;
        };
        // List order is bottom-to-top, so "up" (on top) means a higher index.
        let target = if up { index + 1 } else { index.wrapping_sub(1) };
        if target >= self.downstream.len() {
            return false;
        }
        self.downstream.swap(index, target);
        true
    }

    /// Every file path the collection references — source media/images/fonts +
    /// slideshow entries + each item's LUT/mask filters — tagged with what
    /// points at it (CAP-M03). Paths are returned verbatim (URLs included); a
    /// caller checking existence should skip non-local paths. Empty paths are
    /// omitted (an unset path is not a *missing* file).
    pub fn file_refs(&self) -> Vec<FileRef> {
        let mut refs = Vec::new();
        let mut push = |path: &str, kind: FileRefKind, source: SourceId, name: &str| {
            if !path.is_empty() {
                refs.push(FileRef {
                    path: path.to_string(),
                    kind,
                    source,
                    source_name: name.to_string(),
                });
            }
        };
        for source in &self.sources {
            match &source.settings {
                SourceSettings::Image { path } => {
                    push(path, FileRefKind::Image, source.id, &source.name)
                }
                SourceSettings::Media { path, .. } => {
                    push(path, FileRefKind::Media, source.id, &source.name)
                }
                SourceSettings::Slideshow { paths, .. } => {
                    for path in paths {
                        push(path, FileRefKind::Slideshow, source.id, &source.name);
                    }
                }
                SourceSettings::Text {
                    font_file: Some(path),
                    ..
                } => push(path, FileRefKind::Font, source.id, &source.name),
                _ => {}
            }
        }
        for scene in &self.scenes {
            for item in &scene.items {
                let name = self
                    .source(item.source)
                    .map(|s| s.name.as_str())
                    .unwrap_or("");
                for filter in &item.filters {
                    match &filter.kind {
                        FilterKind::Lut { path, .. } => {
                            push(path, FileRefKind::Lut, item.source, name)
                        }
                        FilterKind::Mask { path, .. } => {
                            push(path, FileRefKind::Mask, item.source, name)
                        }
                        _ => {}
                    }
                }
            }
        }
        refs
    }

    /// Replace every exact occurrence of `old` with `new` across all source
    /// settings and item filters (CAP-M03 relink). Returns how many references
    /// changed — fix one broken path, and every place that used it is repaired.
    pub fn relink_file(&mut self, old: &str, new: &str) -> usize {
        if old.is_empty() || old == new {
            return 0;
        }
        let mut changed = 0;
        let mut swap = |path: &mut String| {
            if path == old {
                *path = new.to_string();
                changed += 1;
            }
        };
        for source in &mut self.sources {
            match &mut source.settings {
                SourceSettings::Image { path } | SourceSettings::Media { path, .. } => swap(path),
                SourceSettings::Slideshow { paths, .. } => paths.iter_mut().for_each(&mut swap),
                SourceSettings::Text {
                    font_file: Some(path),
                    ..
                } => swap(path),
                _ => {}
            }
        }
        for scene in &mut self.scenes {
            for item in &mut scene.items {
                for filter in &mut item.filters {
                    match &mut filter.kind {
                        FilterKind::Lut { path, .. } | FilterKind::Mask { path, .. } => swap(path),
                        _ => {}
                    }
                }
            }
        }
        changed
    }

    /// Replace a scene's custom alignment guides (CAP-M04 follow-on). The list
    /// is bounded so an untrusted webview can't grow it without limit; off-canvas
    /// positions are harmless (they just draw off-screen).
    pub fn set_guides(
        &mut self,
        scene_id: SceneId,
        mut guides: Vec<GuideLine>,
    ) -> Result<(), SceneError> {
        let scene = self.scene_mut(scene_id).ok_or(SceneError::SceneNotFound)?;
        guides.truncate(MAX_GUIDES_PER_SCENE);
        scene.guides = guides;
        Ok(())
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
        // Repointing a nested-scene source must not create a cycle through
        // any scene that shows it.
        if let SourceSettings::NestedScene { scene: target } = &settings {
            let target = *target;
            if self.scene(target).is_none() {
                return Err(SceneError::SceneNotFound);
            }
            let holders: Vec<SceneId> = self
                .scenes
                .iter()
                .filter(|scene| scene.items.iter().any(|item| item.source == id))
                .map(|scene| scene.id)
                .collect();
            for holder in holders {
                if holder == target || self.scene_reaches(target, holder) {
                    return Err(SceneError::SceneCycle);
                }
            }
        }
        let source = self.source_mut(id).ok_or(SceneError::SourceNotFound)?;
        source.settings = settings;
        Ok(())
    }

    /// Whether composing `from` (walking its nested-scene sources
    /// transitively) eventually reaches `needle`. Cycle-safe on its own —
    /// visited scenes are walked once.
    pub fn scene_reaches(&self, from: SceneId, needle: SceneId) -> bool {
        let mut stack = vec![from];
        let mut visited: Vec<SceneId> = Vec::new();
        while let Some(current) = stack.pop() {
            if current == needle {
                return true;
            }
            if visited.contains(&current) {
                continue;
            }
            visited.push(current);
            let Some(scene) = self.scene(current) else {
                continue;
            };
            for item in &scene.items {
                if let Some(source) = self.source(item.source) {
                    if let SourceSettings::NestedScene { scene: target } = &source.settings {
                        stack.push(*target);
                    }
                }
            }
        }
        false
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
        match self.add_item_with_existing_source(scene_id, source_id) {
            Ok(item_id) => Ok((source_id, item_id)),
            Err(err) => {
                // Roll the pool push back (a rejected nested-scene cycle).
                self.sources.retain(|source| source.id != source_id);
                Err(err)
            }
        }
    }

    /// Place an existing pool source on top of `scene_id` (source sharing).
    ///
    /// A newly added *video* item seats into the first free corner instead of
    /// filling the canvas dead-center on top of what's already there — so
    /// adding sources never dumps them overlapping (Mike's rule). The very
    /// first video item still fills the canvas; audio-only items render
    /// nothing and are never seated.
    pub fn add_item_with_existing_source(
        &mut self,
        scene_id: SceneId,
        source_id: SourceId,
    ) -> Result<ItemId, SceneError> {
        let Some(source) = self.source(source_id) else {
            return Err(SceneError::SourceNotFound);
        };
        // A nested-scene source must never make a scene contain itself.
        if let SourceSettings::NestedScene { scene: target } = &source.settings {
            let target = *target;
            if self.scene(target).is_none() {
                return Err(SceneError::SceneNotFound);
            }
            if target == scene_id || self.scene_reaches(target, scene_id) {
                return Err(SceneError::SceneCycle);
            }
        }
        let seat = if source.settings.is_audio_only() {
            None
        } else if matches!(source.settings, SourceSettings::ChatOverlay { .. }) {
            // The classic spot for a chat feed (the spec's default) — still
            // freely draggable afterwards.
            Some(scene::Corner::TopRight.slot())
        } else {
            self.free_corner_for_new_item(scene_id)
        };
        let scene = self.scene_mut(scene_id).ok_or(SceneError::SceneNotFound)?;
        let mut item = SceneItem::new(source_id);
        if let Some(slot) = seat {
            item.pending_slot = Some(slot); // pending_fit is already set
        }
        let id = item.id;
        scene.items.push(item);
        Ok(id)
    }

    /// The first preset seat not already taken by a visible video item — where
    /// a newly added source lands so it doesn't overlap. `None` when the scene
    /// has no other video item yet (the first item fills the canvas) or every
    /// seat is taken (fall back to a centered fit).
    fn free_corner_for_new_item(&self, scene_id: SceneId) -> Option<NormRect> {
        let scene = self.scene(scene_id)?;
        // The backdrop wallpaper doesn't count as "another video item": the
        // first capture added over a wallpaper still fills the whole canvas.
        let is_video = |item: &SceneItem| {
            item.backdrop.is_none()
                && self
                    .source(item.source)
                    .is_some_and(|source| !source.settings.is_audio_only())
        };
        let existing: Vec<&SceneItem> = scene.items.iter().filter(|item| is_video(item)).collect();
        if existing.is_empty() {
            return None;
        }
        let occupied: Vec<NormRect> = existing
            .iter()
            .filter_map(|item| item.pending_slot)
            .collect();
        scene::preset_seats()
            .into_iter()
            .find(|seat| !occupied.iter().any(|taken| same_seat(*taken, *seat)))
    }

    /// Remove an item; its source leaves the pool too if nothing else shows it.
    pub fn remove_item(&mut self, scene_id: SceneId, item_id: ItemId) -> Result<(), SceneError> {
        // Removing the focused item first restores the pre-focus layout —
        // otherwise the scene would be left with everything hidden and no
        // focus toggle to undo it.
        let scene = self.scene(scene_id).ok_or(SceneError::SceneNotFound)?;
        if scene
            .focus
            .as_ref()
            .is_some_and(|focus| focus.item == item_id)
        {
            self.clear_focus(scene_id)?;
        }
        let scene = self.scene_mut(scene_id).ok_or(SceneError::SceneNotFound)?;
        let index = scene
            .items
            .iter()
            .position(|item| item.id == item_id)
            .ok_or(SceneError::ItemNotFound)?;
        let removed_source = scene.items[index].source;
        scene.items.remove(index);
        // CAP-N73: audio linked to the removed window leaves with it —
        // "removed together" is the pairing's whole point.
        let linked: Vec<SourceId> = self
            .sources
            .iter()
            .filter(|source| {
                matches!(
                    &source.settings,
                    SourceSettings::AppAudio { linked_window: Some(target), .. }
                        if *target == removed_source
                )
            })
            .map(|source| source.id)
            .collect();
        if !linked.is_empty() {
            let scene = self.scene_mut(scene_id).ok_or(SceneError::SceneNotFound)?;
            scene.items.retain(|item| !linked.contains(&item.source));
        }
        let scene = self.scene_mut(scene_id).ok_or(SceneError::SceneNotFound)?;
        scene.prune_groups();
        self.gc_sources();
        Ok(())
    }

    /// Move an item to `to_index` in the z-order (clamped; 0 = bottom).
    /// The backdrop wallpaper is pinned: moving it is a no-op, and no other
    /// item can move below it — the capture always renders above it.
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
        if scene.items[from].backdrop.is_some() {
            return Ok(());
        }
        let floor = usize::from(
            scene
                .items
                .first()
                .is_some_and(|item| item.backdrop.is_some()),
        );
        let to = to_index.clamp(floor, scene.items.len() - 1);
        let item = scene.items.remove(from);
        scene.items.insert(to, item);
        Ok(())
    }

    /// Set (or clear, with `None`) a scene's backdrop wallpaper: a pinned
    /// item at the very bottom of the z-order. At most one per scene —
    /// setting replaces the previous backdrop (whose source leaves the pool
    /// when nothing else shows it). The new item lands locked, in
    /// [`BackdropSplit::Full`] (whole-canvas) mode, with the compositor
    /// owning its placement (no first-frame fit).
    pub fn set_scene_backdrop(
        &mut self,
        scene_id: SceneId,
        source: Option<Source>,
    ) -> Result<Option<(SourceId, ItemId)>, SceneError> {
        let scene = self.scene(scene_id).ok_or(SceneError::SceneNotFound)?;
        let old: Vec<ItemId> = scene
            .items
            .iter()
            .filter(|item| item.backdrop.is_some())
            .map(|item| item.id)
            .collect();
        for id in old {
            self.remove_item(scene_id, id)?;
        }
        let Some(source) = source else {
            return Ok(None);
        };
        let (source_id, item_id) = self.add_item_with_new_source(scene_id, source)?;
        let scene = self.scene_mut(scene_id).expect("checked above");
        let from = scene
            .items
            .iter()
            .position(|item| item.id == item_id)
            .expect("just added");
        let mut item = scene.items.remove(from);
        item.backdrop = Some(BackdropSplit::Full);
        item.locked = true;
        // The compositor lays the backdrop out every frame; the transform is
        // only its zoom/pan, so no first-frame fit may overwrite it.
        item.pending_fit = false;
        item.pending_slot = None;
        scene.items.insert(0, item);
        Ok(Some((source_id, item_id)))
    }

    /// Change where a scene's backdrop sits — the whole canvas or one half —
    /// and seat the top-most visible non-backdrop video item into the other
    /// half (back to a whole-canvas fit when returning to Full), so "video
    /// left, capture right" is one call. Errors when the scene has no
    /// backdrop.
    pub fn set_backdrop_split(
        &mut self,
        scene_id: SceneId,
        split: BackdropSplit,
    ) -> Result<(), SceneError> {
        let scene = self.scene(scene_id).ok_or(SceneError::SceneNotFound)?;
        if !scene.items.iter().any(|item| item.backdrop.is_some()) {
            return Err(SceneError::ItemNotFound);
        }
        // The item that takes the other half: the top-most visible screen
        // capture if there is one (the usual pairing), else the top-most
        // visible video item — never a text overlay when a capture exists.
        let candidates: Vec<(ItemId, bool)> = scene
            .items
            .iter()
            .rev()
            .filter(|item| item.backdrop.is_none() && item.visible)
            .filter_map(|item| {
                self.source(item.source).and_then(|source| {
                    (!source.settings.is_audio_only())
                        .then(|| (item.id, source.settings.is_screen_view()))
                })
            })
            .collect();
        let partner = candidates
            .iter()
            .find(|(_, is_screen)| *is_screen)
            .or_else(|| candidates.first())
            .map(|(id, _)| *id);
        let scene = self.scene_mut(scene_id).expect("checked above");
        for item in &mut scene.items {
            if item.backdrop.is_some() {
                item.backdrop = Some(split);
                // Region changed: yesterday's zoom/pan is meaningless there.
                item.transform = Transform::default();
            }
        }
        if let Some(partner) = partner {
            let slot = split.opposite();
            let item = scene.item_mut(partner).expect("listed above");
            item.pending_fit = true;
            item.pending_slot = slot; // None = whole-canvas fit (Full)
        }
        Ok(())
    }

    /// Toggle a backdrop video's "start playback with recording" hold (its
    /// media restarts and holds on the first frame until recording begins).
    /// Errors when the scene has no backdrop or it is a still image.
    pub fn set_backdrop_sync(
        &mut self,
        scene_id: SceneId,
        start_with_recording: bool,
    ) -> Result<(), SceneError> {
        let scene = self.scene(scene_id).ok_or(SceneError::SceneNotFound)?;
        let source_id = scene
            .items
            .iter()
            .find(|item| item.backdrop.is_some())
            .map(|item| item.source)
            .ok_or(SceneError::ItemNotFound)?;
        let source = self
            .sources
            .iter_mut()
            .find(|source| source.id == source_id)
            .ok_or(SceneError::SourceNotFound)?;
        match &mut source.settings {
            SourceSettings::Media {
                start_with_recording: flag,
                ..
            } => {
                *flag = start_with_recording;
                Ok(())
            }
            _ => Err(SceneError::SourceNotFound),
        }
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
        let scene = self.scene_mut(scene_id).ok_or(SceneError::SceneNotFound)?;
        let old = scene
            .item(item_id)
            .ok_or(SceneError::ItemNotFound)?
            .transform;
        {
            let item = scene.item_mut(item_id).expect("checked above");
            item.transform = transform;
            // A user-driven transform supersedes any pending first-frame placement.
            item.pending_fit = false;
            item.pending_slot = None;
        }
        // Grouped items move as one: a translation carries the group along
        // (scale / rotate / crop stay per-item).
        let (dx, dy) = (transform.x - old.x, transform.y - old.y);
        if dx != 0.0 || dy != 0.0 {
            let mates: Vec<ItemId> = scene
                .group_of(item_id)
                .map(|group| group.items.clone())
                .unwrap_or_default();
            for mate in mates {
                if mate == item_id {
                    continue;
                }
                if let Some(item) = scene.item_mut(mate) {
                    item.transform.x += dx;
                    item.transform.y += dy;
                    item.pending_fit = false;
                    item.pending_slot = None;
                }
            }
        }
        Ok(())
    }

    // -- source groups (Phase 6) --------------------------------------------

    /// Group `items` (all in `scene_id`, none already grouped) so they move
    /// and show/hide together.
    pub fn create_group(
        &mut self,
        scene_id: SceneId,
        name: &str,
        items: &[ItemId],
    ) -> Result<GroupId, SceneError> {
        let scene = self.scene_mut(scene_id).ok_or(SceneError::SceneNotFound)?;
        if items.is_empty() {
            return Err(SceneError::ItemNotFound);
        }
        for id in items {
            if scene.item(*id).is_none() {
                return Err(SceneError::ItemNotFound);
            }
            if scene.group_of(*id).is_some() {
                return Err(SceneError::AlreadyGrouped);
            }
        }
        let name = name.trim();
        let group = scene::SourceGroup {
            id: GroupId::new(),
            name: if name.is_empty() {
                format!("Group {}", scene.groups.len() + 1)
            } else {
                name.to_string()
            },
            items: items.to_vec(),
            visible: true,
        };
        let id = group.id;
        scene.groups.push(group);
        Ok(id)
    }

    /// Dissolve a group — its items stay exactly where they are.
    pub fn ungroup(&mut self, scene_id: SceneId, group: GroupId) -> Result<(), SceneError> {
        let scene = self.scene_mut(scene_id).ok_or(SceneError::SceneNotFound)?;
        let before = scene.groups.len();
        scene.groups.retain(|candidate| candidate.id != group);
        if scene.groups.len() == before {
            return Err(SceneError::GroupNotFound);
        }
        Ok(())
    }

    /// A group's eye toggle — hides/shows every member together (ANDs with
    /// each member's own visibility).
    pub fn set_group_visible(
        &mut self,
        scene_id: SceneId,
        group: GroupId,
        visible: bool,
    ) -> Result<(), SceneError> {
        let scene = self.scene_mut(scene_id).ok_or(SceneError::SceneNotFound)?;
        let group = scene
            .groups
            .iter_mut()
            .find(|candidate| candidate.id == group)
            .ok_or(SceneError::GroupNotFound)?;
        group.visible = visible;
        Ok(())
    }

    // -- per-scene audio (Phase 6) -------------------------------------------

    /// Set (or clear, with `None`) a source's per-scene mixer override:
    /// while `scene_id` is the program, the override replaces the source's
    /// global fader/mute.
    pub fn set_scene_audio_override(
        &mut self,
        scene_id: SceneId,
        source: SourceId,
        override_: Option<(f32, bool)>,
    ) -> Result<(), SceneError> {
        let has_audio = self
            .source(source)
            .ok_or(SceneError::SourceNotFound)?
            .settings
            .has_audio();
        if !has_audio {
            return Err(SceneError::SourceNotAudio);
        }
        let scene = self.scene_mut(scene_id).ok_or(SceneError::SceneNotFound)?;
        scene.audio_overrides.retain(|entry| entry.source != source);
        if let Some((volume_db, muted)) = override_ {
            scene.audio_overrides.push(scene::SceneAudioOverride {
                source,
                volume_db: volume_db.clamp(MIN_VOLUME_DB, MAX_VOLUME_DB),
                muted,
            });
        }
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
        // Validate every id first so a bad reference changes nothing. The
        // backdrop wallpaper is not layoutable (a first-frame fit would
        // overwrite the zoom/pan its transform actually holds).
        let scene = self.scene(scene_id).ok_or(SceneError::SceneNotFound)?;
        let present = |id: ItemId| {
            scene
                .items
                .iter()
                .any(|item| item.id == id && item.backdrop.is_none())
        };
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
        // items (overlays) stay on top. A backdrop wallpaper keeps index 0 —
        // the layout stacks above it, never around it.
        let mut index = usize::from(
            self.scene(scene_id)
                .expect("validated above")
                .items
                .first()
                .is_some_and(|item| item.backdrop.is_some()),
        );
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

    /// Seat one item into a normalized canvas slot — the one-click position
    /// presets (a guest to a corner or a mid-edge seat). Placement resolves
    /// on the source's next sized frame, exactly like a corner of
    /// [`Self::apply_layout`]. The slot is untrusted input (the webview sends
    /// it): it must be finite, non-empty, and lie fully inside the canvas.
    ///
    /// **Seat swap:** two items never share a seat. If the target seat is
    /// occupied, the occupant moves to the mover's old seat — guest 1 onto
    /// guest 2's corner puts guest 2 on guest 1's corner. A mover with no
    /// seat bumps the occupant to the first free [`scene::preset_seats`]
    /// seat instead.
    pub fn set_item_slot(
        &mut self,
        scene_id: SceneId,
        item_id: ItemId,
        slot: NormRect,
    ) -> Result<(), SceneError> {
        let finite =
            slot.x.is_finite() && slot.y.is_finite() && slot.w.is_finite() && slot.h.is_finite();
        let inside = finite
            && slot.w > 0.0
            && slot.h > 0.0
            && slot.x >= 0.0
            && slot.y >= 0.0
            && slot.x + slot.w <= 1.0
            && slot.y + slot.h <= 1.0;
        if !inside {
            return Err(SceneError::InvalidSlot);
        }

        let scene_ref = self.scene(scene_id).ok_or(SceneError::SceneNotFound)?;
        let mover = scene_ref.item(item_id).ok_or(SceneError::ItemNotFound)?;
        // The backdrop is not seatable — the compositor owns its placement.
        if mover.backdrop.is_some() {
            return Ok(());
        }
        let old_seat = mover.pending_slot;

        // "Nothing overlaps the shared view": while another item holds the
        // center seat, a request for a seat that intersects it lands on the
        // first free rail seat instead.
        let center = scene::center_slot();
        let center_taken = scene_ref.items.iter().any(|other| {
            other.id != item_id && other.pending_slot.is_some_and(|s| same_seat(s, center))
        });
        let slot = if center_taken && !same_seat(slot, center) && scene::rects_overlap(slot, center)
        {
            scene::rail_seats()
                .into_iter()
                .find(|candidate| {
                    !scene_ref.items.iter().any(|other| {
                        other.id != item_id
                            && other.pending_slot.is_some_and(|s| same_seat(s, *candidate))
                    })
                })
                .ok_or(SceneError::InvalidSlot)?
        } else {
            slot
        };

        let occupant = scene_ref
            .items
            .iter()
            .find(|other| {
                other.id != item_id && other.pending_slot.is_some_and(|s| same_seat(s, slot))
            })
            .map(|other| other.id);
        // Where the occupant goes: the mover's old seat, else the first free
        // preset seat (a seat always exists — six seats, and the mover only
        // takes one).
        let bumped_to = occupant.and_then(|_| {
            old_seat.or_else(|| {
                scene::preset_seats().into_iter().find(|candidate| {
                    !same_seat(*candidate, slot)
                        && !scene_ref.items.iter().any(|other| {
                            other.id != item_id
                                && other.pending_slot.is_some_and(|s| same_seat(s, *candidate))
                        })
                })
            })
        });

        if let (Some(occupant_id), Some(seat)) = (occupant, bumped_to) {
            let bumped = self.item_mut(scene_id, occupant_id)?;
            bumped.pending_fit = true;
            bumped.pending_slot = Some(seat);
        }
        let item = self.item_mut(scene_id, item_id)?;
        item.pending_fit = true;
        item.pending_slot = Some(slot);
        Ok(())
    }

    /// The engine applies a resolved first-frame placement: set the computed
    /// transform and clear the one-shot flag — **keeping** `pending_slot` as
    /// the item's remembered seat (inert for the engine once `pending_fit`
    /// is false, but seat-swap reads it). A *user-driven* transform goes
    /// through [`Self::set_item_transform`] instead, which vacates the seat.
    pub fn resolve_pending(
        &mut self,
        scene_id: SceneId,
        item_id: ItemId,
        transform: Transform,
    ) -> Result<(), SceneError> {
        let item = self.item_mut(scene_id, item_id)?;
        item.transform = transform;
        item.pending_fit = false;
        Ok(())
    }

    /// Center-view routing (host-controlled; Mike's spec): promote any
    /// capture — a cam, a Display/Window capture, or a remote guest's share —
    /// into the CENTER seat beside the cam rail. The rules:
    ///
    /// - the current center occupant takes the promoted item's old seat (the
    ///   "display capture takes the camera's top-left spot" swap), or the
    ///   first free rail seat when the mover was unseated;
    /// - **nothing overlaps the shared view**: every other seated item whose
    ///   seat intersects the center region is bumped onto the rail;
    /// - **one screen view at a time**: centering a Display/Window hides the
    ///   other visible Display/Window items (cams are untouched);
    /// - `None` retires the center — the occupant moves to a free rail seat.
    pub fn set_center_view(
        &mut self,
        scene_id: SceneId,
        item_id: Option<ItemId>,
    ) -> Result<(), SceneError> {
        let center = scene::center_slot();
        let scene_ref = self.scene(scene_id).ok_or(SceneError::SceneNotFound)?;
        let occupant = scene_ref
            .items
            .iter()
            .find(|item| item.pending_slot.is_some_and(|s| same_seat(s, center)))
            .map(|item| item.id);

        // Rail-seat allocator over the seats already taken in this scene.
        let mut taken: Vec<NormRect> = scene_ref
            .items
            .iter()
            .filter_map(|item| item.pending_slot)
            .collect();
        let take_free_rail = |taken: &mut Vec<NormRect>| -> Option<NormRect> {
            let free = scene::rail_seats()
                .into_iter()
                .find(|candidate| !taken.iter().any(|t| same_seat(*t, *candidate)));
            if let Some(seat) = free {
                taken.push(seat);
            }
            free
        };

        let mut reseat: Vec<(ItemId, NormRect)> = Vec::new();

        let Some(target) = item_id else {
            // Retire the center: its occupant joins the rail.
            if let Some(occ) = occupant {
                if let Some(seat) = take_free_rail(&mut taken) {
                    reseat.push((occ, seat));
                }
            }
            for (id, seat) in reseat {
                let item = self.item_mut(scene_id, id)?;
                item.pending_fit = true;
                item.pending_slot = Some(seat);
            }
            return Ok(());
        };

        let mover = scene_ref.item(target).ok_or(SceneError::ItemNotFound)?;
        // The backdrop wallpaper can't take the center seat (the compositor
        // owns its placement; its transform is only zoom/pan).
        if mover.backdrop.is_some() {
            return Err(SceneError::ItemNotFound);
        }
        let mover_old = mover.pending_slot;

        // The displaced center takes the mover's old seat, else a rail seat.
        if let Some(occ) = occupant.filter(|occ| *occ != target) {
            let next = mover_old
                .filter(|seat| !same_seat(*seat, center))
                .or_else(|| take_free_rail(&mut taken));
            if let Some(seat) = next {
                reseat.push((occ, seat));
            }
        }

        // Nothing overlaps the shared view: bump intersecting seats to the rail.
        let overlapping: Vec<ItemId> = scene_ref
            .items
            .iter()
            .filter(|item| {
                item.id != target
                    && Some(item.id) != occupant
                    && item
                        .pending_slot
                        .is_some_and(|s| scene::rects_overlap(s, center) && !same_seat(s, center))
            })
            .map(|item| item.id)
            .collect();
        for id in overlapping {
            if let Some(seat) = take_free_rail(&mut taken) {
                reseat.push((id, seat));
            }
        }

        // One screen view at a time.
        let target_source = scene_ref
            .item(target)
            .ok_or(SceneError::ItemNotFound)?
            .source;
        let target_is_screen = self
            .sources
            .iter()
            .find(|source| source.id == target_source)
            .is_some_and(|source| source.settings.is_screen_view());
        let hide: Vec<ItemId> = if target_is_screen {
            scene_ref
                .items
                .iter()
                .filter(|item| {
                    item.id != target
                        && item.visible
                        && self
                            .sources
                            .iter()
                            .find(|source| source.id == item.source)
                            .is_some_and(|source| source.settings.is_screen_view())
                })
                .map(|item| item.id)
                .collect()
        } else {
            Vec::new()
        };

        for (id, seat) in reseat {
            let item = self.item_mut(scene_id, id)?;
            item.pending_fit = true;
            item.pending_slot = Some(seat);
        }
        for id in hide {
            self.item_mut(scene_id, id)?.visible = false;
        }
        let item = self.item_mut(scene_id, target)?;
        item.visible = true;
        item.pending_fit = true;
        item.pending_slot = Some(center);
        Ok(())
    }

    /// Highlight Speaker (Focus/Spotlight): promote one item to fill the
    /// whole canvas and hide the other video items, snapshotting every item's
    /// placement + visibility so [`Self::clear_focus`] restores the layout
    /// exactly. Audio-only items are untouched — they render nothing, and
    /// hiding them would silence their audio mid-show. Focusing while already
    /// focused re-targets: the original snapshot is restored first, so
    /// stepping through several speakers always returns to the true
    /// pre-focus layout. The promoted item resolves to a whole-canvas fit on
    /// its next sized frame (the same mechanism as a freshly added item).
    pub fn set_focus(&mut self, scene_id: SceneId, item_id: ItemId) -> Result<(), SceneError> {
        let scene = self.scene(scene_id).ok_or(SceneError::SceneNotFound)?;
        if scene.focus.is_some() {
            self.clear_focus(scene_id)?;
        }
        let audio_only: Vec<SourceId> = self
            .sources
            .iter()
            .filter(|source| source.settings.is_audio_only())
            .map(|source| source.id)
            .collect();

        let scene = self.scene_mut(scene_id).ok_or(SceneError::SceneNotFound)?;
        // The backdrop wallpaper can't be promoted (a first-frame fit would
        // overwrite the zoom/pan its transform actually holds); it does get
        // hidden-and-restored like any other video item.
        if !scene
            .items
            .iter()
            .any(|item| item.id == item_id && item.backdrop.is_none())
        {
            return Err(SceneError::ItemNotFound);
        }
        let prior = scene
            .items
            .iter()
            .map(|item| FocusRestore {
                item: item.id,
                transform: item.transform,
                visible: item.visible,
                pending_slot: item.pending_slot,
            })
            .collect();
        for item in scene.items.iter_mut() {
            if item.id == item_id {
                item.visible = true;
                item.pending_fit = true;
                item.pending_slot = None; // whole-canvas fit
            } else if !audio_only.contains(&item.source) {
                item.visible = false;
            }
        }
        scene.focus = Some(FocusState {
            item: item_id,
            prior,
        });
        Ok(())
    }

    /// Toggle Focus off: restore every snapshotted item's placement +
    /// visibility exactly. Items added while focused keep their current
    /// state; snapshot entries whose item was removed are skipped. A scene
    /// with no focus is a no-op.
    pub fn clear_focus(&mut self, scene_id: SceneId) -> Result<(), SceneError> {
        let scene = self.scene_mut(scene_id).ok_or(SceneError::SceneNotFound)?;
        let Some(focus) = scene.focus.take() else {
            return Ok(());
        };
        // Seats claimed while focused (by items OUTSIDE the snapshot — e.g. a
        // guest who joined mid-spotlight) win: restoring a remembered seat on
        // top of one would double-book it and break the no-overlap rule.
        let snapshot: Vec<ItemId> = focus.prior.iter().map(|saved| saved.item).collect();
        let claimed: Vec<NormRect> = scene
            .items
            .iter()
            .filter(|item| !snapshot.contains(&item.id))
            .filter_map(|item| item.pending_slot)
            .collect();
        for saved in focus.prior {
            if let Some(item) = scene.item_mut(saved.item) {
                item.transform = saved.transform;
                item.visible = saved.visible;
                item.pending_fit = false;
                // The remembered seat comes back unless someone claimed it.
                item.pending_slot = saved
                    .pending_slot
                    .filter(|seat| !claimed.iter().any(|taken| same_seat(*taken, *seat)));
            }
        }
        Ok(())
    }

    pub fn set_item_visible(
        &mut self,
        scene_id: SceneId,
        item_id: ItemId,
        visible: bool,
    ) -> Result<(), SceneError> {
        let source = {
            let item = self.item_mut(scene_id, item_id)?;
            item.visible = visible;
            item.source
        };
        // CAP-N73: hiding a window mutes its linked app audio ("muted
        // together"); showing it unmutes — but only a strip WE auto-muted,
        // never one the operator muted by hand (else a deliberately-silenced
        // copyrighted track would go back on air the next time the window is
        // shown). `hidden_muted` records exactly which strips this link muted.
        let linked: Vec<SourceId> = self
            .sources
            .iter()
            .filter(|entry| {
                matches!(
                    &entry.settings,
                    SourceSettings::AppAudio { linked_window: Some(target), .. }
                        if *target == source
                )
            })
            .map(|entry| entry.id)
            .collect();
        for id in linked {
            if visible {
                // Restore only what the hide muted; a manual mute stays put.
                if self.hidden_muted.remove(&id) {
                    let _ = self.set_audio_muted(id, false);
                }
            } else {
                // Auto-mute only a strip that is currently audible; a strip the
                // operator already muted is left alone (and not tracked, so it
                // will not be auto-unmuted on show).
                let audible = self
                    .sources
                    .iter()
                    .find(|entry| entry.id == id)
                    .and_then(|entry| entry.audio.as_ref())
                    .is_some_and(|audio| !audio.muted);
                if audible {
                    self.hidden_muted.insert(id);
                    let _ = self.set_audio_muted(id, true);
                }
            }
        }
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

    /// Pixel-perfect scaling (CAP-N70): how the item's pixels reach the
    /// canvas (smooth / nearest / integer-snapped nearest / sharp-bilinear).
    pub fn set_item_scaling(
        &mut self,
        scene_id: SceneId,
        item_id: ItemId,
        scaling: ScaleMode,
    ) -> Result<(), SceneError> {
        self.item_mut(scene_id, item_id)?.scaling = scaling;
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

    /// Append copied filters to an item's chain (CAP-M05 paste). Each lands on
    /// top with a **fresh id** (so a filter can be pasted onto the same item it
    /// was copied from) while keeping its kind + enabled state. Returns how many
    /// were added. One mutation — the whole paste is a single undo step.
    pub fn paste_filters(
        &mut self,
        scene_id: SceneId,
        item_id: ItemId,
        filters: Vec<Filter>,
    ) -> Result<usize, SceneError> {
        let item = self.item_mut(scene_id, item_id)?;
        let count = filters.len();
        for filter in filters {
            item.filters.push(Filter {
                id: FilterId::new(),
                enabled: filter.enabled,
                kind: filter.kind,
            });
        }
        Ok(count)
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

    /// Set the stereo balance (CAP-M19), clamped to −1..=1.
    pub fn set_audio_pan(&mut self, id: SourceId, pan: f32) -> Result<(), SceneError> {
        self.audio_mut(id)?.pan = if pan.is_finite() {
            pan.clamp(-1.0, 1.0)
        } else {
            0.0
        };
        Ok(())
    }

    /// Set PFL solo (CAP-M19) — monitor-bus routing only.
    pub fn set_audio_solo(&mut self, id: SourceId, solo: bool) -> Result<(), SceneError> {
        self.audio_mut(id)?.solo = solo;
        Ok(())
    }

    /// Set the mono downmix (CAP-M19).
    pub fn set_audio_mono(&mut self, id: SourceId, mono: bool) -> Result<(), SceneError> {
        self.audio_mut(id)?.mono = mono;
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
                        deinterlace: DeinterlaceMode::MotionAdaptive,
                        field_order: FieldOrder::BottomFirst,
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
                source_file: "C:/data/score.csv".into(),
                binding: FileBinding::CsvCell,
                csv_row: 2,
                csv_column: "score".into(),
                json_pointer: String::new(),
            },
            SourceSettings::SystemStats {
                show_fps: true,
                show_cpu: true,
                show_memory: false,
                show_render_ms: true,
                show_dropped: true,
                show_bitrate: false,
                font_family: None,
                font_file: None,
                size_px: 28.0,
                color: Rgba::WHITE,
            },
            SourceSettings::AudioVisualizer {
                style: VisStyle::Scope,
                target: VisTargetKind::Track,
                track: 3,
                source: None,
                width: 640,
                height: 200,
                bands: 32,
                color: Rgba::new(10, 220, 120, 255),
                peak_hold: false,
                decay: 24.0,
            },
            SourceSettings::SplitTimer {
                path: "C:/runs/any-percent.lss".into(),
                comparison: SplitComparison::BestSegments,
                width: 420,
                height: 380,
                size_px: 18.0,
                color: Rgba::WHITE,
                ahead: Rgba::new(0x22, 0xc5, 0x5e, 255),
                behind: Rgba::new(0xef, 0x44, 0x44, 255),
                gold: Rgba::new(0xfb, 0xbf, 0x24, 255),
            },
            SourceSettings::Playlist {
                items: vec![
                    PlaylistEntry {
                        path: "C:/vt/intro.mp4".into(),
                        in_s: 1.5,
                        out_s: 0.0,
                        cues: vec![10.0, 42.5],
                    },
                    PlaylistEntry {
                        path: "C:/vt/loop.mp4".into(),
                        in_s: 0.0,
                        out_s: 30.0,
                        cues: Vec::new(),
                    },
                ],
                looping: true,
                shuffle: false,
                hold_last: true,
                hw_decode: true,
                now_playing_variable: "nowPlaying".into(),
            },
            SourceSettings::ReplayPlayback {
                seconds: 20,
                speed: ReplaySpeed::Quarter,
                hw_decode: false,
            },
            SourceSettings::LanIngest {
                protocol: IngestProtocol::Rtmp,
                port: 1935,
                passphrase: String::new(),
            },
            SourceSettings::LanIngest {
                protocol: IngestProtocol::Srt,
                port: 9710,
                passphrase: "correct horse battery".into(),
            },
            SourceSettings::InputOverlay {
                layout: InputLayout::Gamepad,
                color: Rgba::WHITE,
                accent: Rgba::new(0x4a, 0x9e, 0xff, 255),
            },
            SourceSettings::Title {
                width: 1920,
                height: 1080,
                layers: vec![
                    TitleLayer::Rect {
                        x: 60,
                        y: 840,
                        width: 800,
                        height: 140,
                        color: Rgba::new(0x4a, 0x9e, 0xff, 230),
                    },
                    TitleLayer::Text {
                        x: 88,
                        y: 856,
                        text: "Player One {{score}}".into(),
                        font_family: None,
                        font_file: None,
                        size_px: 56.0,
                        color: Rgba::WHITE,
                        align: TextAlign::Left,
                        outline_px: 2.0,
                        outline_color: Rgba::new(0, 0, 0, 255),
                        shadow: true,
                        source_file: "C:/data/score.csv".into(),
                        binding: FileBinding::CsvCell,
                        csv_row: 2,
                        csv_column: "score".into(),
                        json_pointer: String::new(),
                    },
                    TitleLayer::Image {
                        x: 1700,
                        y: 60,
                        path: "C:/overlays/badge.png".into(),
                    },
                ],
                animation: TitleAnimation::SlideLeft,
                duration_ms: 350,
            },
            SourceSettings::FreallyLink {
                host: "192.168.1.20".into(),
                port: 9720,
                label: "Gaming PC".into(),
                key: "gaming-pc-key".into(),
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
                    ..Default::default()
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
        let mut original = full_collection();
        // A split media backdrop rides along so the new fields round-trip.
        let scene = original.active_scene;
        original
            .set_scene_backdrop(scene, Some(backdrop_media("wall.mp4")))
            .expect("backdrop");
        original
            .set_backdrop_split(scene, BackdropSplit::Left)
            .expect("split");
        // Pixel-perfect scaling (CAP-N70) rides along too.
        let item = original.scenes[0].items[0].id;
        original
            .set_item_scaling(scene, item, ScaleMode::Integer)
            .expect("scaling");
        let json = serde_json::to_string_pretty(&original).expect("serialize");
        let restored: Collection = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(original, restored, "the model must round-trip losslessly");
    }

    #[test]
    fn file_refs_lists_paths_and_relink_repairs_every_use() {
        let mut collection = Collection::new();
        let scene = collection.active_scene;
        // An image source pointing at a broken path...
        let (_img, item) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "Logo",
                    SourceSettings::Image {
                        path: "old.png".into(),
                    },
                ),
            )
            .expect("add image");
        // ...plus a LUT filter that (say) points at the SAME path.
        collection
            .add_filter(
                scene,
                item,
                FilterKind::Lut {
                    path: "old.png".into(),
                    amount: 1.0,
                },
            )
            .expect("add lut");

        let refs = collection.file_refs();
        assert_eq!(refs.len(), 2, "the image and the LUT both reference a file");
        assert!(refs.iter().any(|r| r.kind == FileRefKind::Image));
        assert!(refs.iter().any(|r| r.kind == FileRefKind::Lut));

        // One relink repairs every use of that path.
        assert_eq!(collection.relink_file("old.png", "new.png"), 2);
        assert!(collection.file_refs().iter().all(|r| r.path == "new.png"));
        // No-ops: unknown path, and old == new.
        assert_eq!(collection.relink_file("absent.png", "x.png"), 0);
        assert_eq!(collection.relink_file("new.png", "new.png"), 0);
    }

    #[test]
    fn set_guides_replaces_and_bounds_the_list() {
        let mut collection = Collection::new();
        let scene = collection.active_scene;
        collection
            .set_guides(
                scene,
                vec![GuideLine {
                    orientation: GuideOrientation::V,
                    position: 960.0,
                }],
            )
            .expect("set");
        assert_eq!(collection.scene(scene).expect("scene").guides.len(), 1);

        // The list is bounded (an untrusted webview can't grow it without limit).
        let many: Vec<GuideLine> = (0..100)
            .map(|i| GuideLine {
                orientation: GuideOrientation::H,
                position: i as f32,
            })
            .collect();
        collection.set_guides(scene, many).expect("set many");
        assert_eq!(
            collection.scene(scene).expect("scene").guides.len(),
            MAX_GUIDES_PER_SCENE
        );

        // An unknown scene is rejected.
        assert!(collection.set_guides(SceneId::new(), Vec::new()).is_err());
    }

    fn backdrop_media(path: &str) -> Source {
        Source::new(
            "Backdrop",
            SourceSettings::Media {
                path: path.into(),
                looping: true,
                hw_decode: true,
                start_with_recording: false,
                reverse: false,
            },
        )
    }

    #[test]
    fn backdrop_sets_replaces_clears_and_stays_pinned() {
        let mut collection = Collection::new();
        let scene = collection.active_scene;
        // A capture already in the scene.
        let (_cap, cap_item) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "Screen",
                    SourceSettings::Display {
                        capture_id: "dxgi:0".into(),
                        label: "Display 1".into(),
                    },
                ),
            )
            .expect("add capture");

        let (wall_src, wall_item) = collection
            .set_scene_backdrop(scene, Some(backdrop_media("wall.mp4")))
            .expect("set")
            .expect("created");
        {
            let scene_ref = collection.scene(scene).expect("scene");
            assert_eq!(
                scene_ref.items[0].id, wall_item,
                "backdrop lands at the bottom"
            );
            assert_eq!(scene_ref.items[0].backdrop, Some(BackdropSplit::Full));
            assert!(scene_ref.items[0].locked);
            assert!(
                !scene_ref.items[0].pending_fit,
                "the compositor owns placement"
            );
        }

        // Pinned: the backdrop won't move, and nothing moves below it.
        collection
            .reorder_item(scene, wall_item, 5)
            .expect("no-op reorder");
        collection
            .reorder_item(scene, cap_item, 0)
            .expect("clamped");
        let scene_ref = collection.scene(scene).expect("scene");
        assert_eq!(scene_ref.items[0].id, wall_item, "still at the bottom");

        // Not seatable, layoutable, focusable, or centerable.
        assert!(collection
            .set_item_slot(scene, wall_item, scene::Corner::TopLeft.slot())
            .is_ok());
        assert_eq!(
            collection
                .scene(scene)
                .expect("scene")
                .item(wall_item)
                .expect("item")
                .pending_slot,
            None,
            "seating the backdrop is a no-op"
        );
        assert!(collection
            .apply_layout(scene, Some(wall_item), &[])
            .is_err());
        assert!(collection.set_focus(scene, wall_item).is_err());

        // Replacing swaps the pool source; clearing drops item + source.
        let (wall2_src, _wall2_item) = collection
            .set_scene_backdrop(scene, Some(backdrop_media("wall2.mp4")))
            .expect("replace")
            .expect("created");
        assert!(collection.source(wall_src).is_none(), "old source gc'd");
        assert!(collection.source(wall2_src).is_some());
        assert!(
            collection
                .set_scene_backdrop(scene, None)
                .expect("clear")
                .is_none(),
            "clear creates nothing"
        );
        assert!(
            collection.source(wall2_src).is_none(),
            "cleared source gc'd"
        );
        assert!(collection
            .scene(scene)
            .expect("scene")
            .items
            .iter()
            .all(|item| item.backdrop.is_none()));

        // An unknown scene is rejected.
        assert!(collection.set_scene_backdrop(SceneId::new(), None).is_err());
    }

    #[test]
    fn first_capture_over_a_wallpaper_still_fills_the_canvas() {
        let mut collection = Collection::new();
        let scene = collection.active_scene;
        collection
            .set_scene_backdrop(scene, Some(backdrop_media("wall.png")))
            .expect("set");
        let (_cap, cap_item) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "Screen",
                    SourceSettings::Display {
                        capture_id: "dxgi:0".into(),
                        label: "Display 1".into(),
                    },
                ),
            )
            .expect("add capture");
        let item = collection
            .scene(scene)
            .expect("scene")
            .item(cap_item)
            .expect("item");
        assert_eq!(
            item.pending_slot, None,
            "the wallpaper is not 'another video item' — the capture fills the canvas"
        );
    }

    #[test]
    fn backdrop_split_reseats_the_capture_into_the_other_half() {
        let mut collection = Collection::new();
        let scene = collection.active_scene;
        // No backdrop yet: split errors.
        assert!(collection
            .set_backdrop_split(scene, BackdropSplit::Left)
            .is_err());

        collection
            .set_scene_backdrop(scene, Some(backdrop_media("wall.mp4")))
            .expect("set");
        let (_cap, cap_item) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "Screen",
                    SourceSettings::Display {
                        capture_id: "dxgi:0".into(),
                        label: "Display 1".into(),
                    },
                ),
            )
            .expect("add capture");

        collection
            .set_backdrop_split(scene, BackdropSplit::Left)
            .expect("split");
        let scene_ref = collection.scene(scene).expect("scene");
        assert_eq!(
            scene_ref.items[0].backdrop,
            Some(BackdropSplit::Left),
            "backdrop takes the left half"
        );
        let cap = scene_ref.item(cap_item).expect("item");
        assert!(cap.pending_fit);
        assert_eq!(
            cap.pending_slot,
            Some(BackdropSplit::Right.region()),
            "the capture is seated into the right half"
        );

        // Back to Full: the capture re-fits to the whole canvas.
        collection
            .set_backdrop_split(scene, BackdropSplit::Full)
            .expect("full");
        let scene_ref = collection.scene(scene).expect("scene");
        let cap = scene_ref.item(cap_item).expect("item");
        assert!(cap.pending_fit);
        assert_eq!(cap.pending_slot, None, "whole-canvas fit");
    }

    #[test]
    fn apply_layout_stacks_above_the_backdrop() {
        let mut collection = Collection::new();
        let scene = collection.active_scene;
        collection
            .set_scene_backdrop(scene, Some(backdrop_media("wall.png")))
            .expect("backdrop");
        let (_screen, screen_item) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "Screen",
                    SourceSettings::Display {
                        capture_id: "dxgi:0".into(),
                        label: "Display 1".into(),
                    },
                ),
            )
            .expect("add screen");
        let (_cam, cam_item) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "Cam",
                    SourceSettings::VideoDevice {
                        device_id: "cam-0".into(),
                        format: None,
                        deinterlace: DeinterlaceMode::Off,
                        field_order: FieldOrder::TopFirst,
                    },
                ),
            )
            .expect("add cam");

        collection
            .apply_layout(scene, Some(screen_item), &[(cam_item, Corner::TopRight)])
            .expect("layout");
        let order: Vec<ItemId> = collection
            .scene(scene)
            .expect("scene")
            .items
            .iter()
            .map(|item| item.id)
            .collect();
        assert!(
            collection.scene(scene).expect("scene").items[0]
                .backdrop
                .is_some(),
            "the backdrop keeps the bottom"
        );
        assert_eq!(
            order[1], screen_item,
            "the centered screen sits directly above the backdrop"
        );
        assert_eq!(order[2], cam_item, "the corner cam sits above the screen");
    }

    #[test]
    fn item_scaling_sets_and_rejects_unknown_targets() {
        let mut collection = Collection::new();
        let scene = collection.active_scene;
        let (_source, item) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "Cam",
                    SourceSettings::Display {
                        capture_id: "dxgi:0".into(),
                        label: "Display 1".into(),
                    },
                ),
            )
            .expect("add");
        assert_eq!(
            collection
                .scene(scene)
                .expect("scene")
                .item(item)
                .expect("item")
                .scaling,
            ScaleMode::Auto,
            "smooth by default"
        );
        collection
            .set_item_scaling(scene, item, ScaleMode::SharpBilinear)
            .expect("set");
        assert_eq!(
            collection
                .scene(scene)
                .expect("scene")
                .item(item)
                .expect("item")
                .scaling,
            ScaleMode::SharpBilinear
        );
        assert!(collection
            .set_item_scaling(scene, ItemId::new(), ScaleMode::Nearest)
            .is_err());
    }

    #[test]
    fn linked_app_audio_follows_its_window() {
        let mut collection = Collection::new();
        let scene = collection.active_scene;
        let (window_id, window_item) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "Game",
                    SourceSettings::Window {
                        capture_id: "window:1234".into(),
                        label: "Game".into(),
                    },
                ),
            )
            .expect("add window");
        let (audio_id, _audio_item) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "game.exe audio",
                    SourceSettings::AppAudio {
                        pid: 4242,
                        exe: "game.exe".into(),
                        linked_window: Some(window_id),
                    },
                ),
            )
            .expect("add audio");

        // Hiding the window mutes the linked strip; showing unmutes.
        collection
            .set_item_visible(scene, window_item, false)
            .expect("hide");
        assert!(
            collection
                .source(audio_id)
                .and_then(|source| source.audio.as_ref())
                .is_some_and(|audio| audio.muted),
            "hidden window = muted app audio"
        );
        collection
            .set_item_visible(scene, window_item, true)
            .expect("show");
        assert!(
            collection
                .source(audio_id)
                .and_then(|source| source.audio.as_ref())
                .is_some_and(|audio| !audio.muted),
            "shown window = unmuted app audio"
        );

        // Removing the window removes the linked audio (and its pool entry).
        collection
            .remove_item(scene, window_item)
            .expect("remove window");
        assert!(collection.source(audio_id).is_none(), "audio leaves too");
        assert!(
            collection.scene(scene).expect("scene").items.is_empty(),
            "no orphaned items"
        );
    }

    #[test]
    fn hiding_a_window_never_clobbers_a_manual_mute() {
        // CAP-N73 regression: a strip the operator muted by hand must NOT be
        // force-unmuted when the linked window is shown again — else a
        // deliberately-silenced (e.g. copyrighted) track goes back on air.
        let mut collection = Collection::new();
        let scene = collection.active_scene;
        let (window_id, window_item) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "Game",
                    SourceSettings::Window {
                        capture_id: "window:1234".into(),
                        label: "Game".into(),
                    },
                ),
            )
            .expect("add window");
        let (audio_id, _) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "game.exe audio",
                    SourceSettings::AppAudio {
                        pid: 4242,
                        exe: "game.exe".into(),
                        linked_window: Some(window_id),
                    },
                ),
            )
            .expect("add audio");
        let is_muted = |c: &Collection| {
            c.source(audio_id)
                .and_then(|source| source.audio.as_ref())
                .is_some_and(|audio| audio.muted)
        };

        // The operator manually mutes the strip while the window is visible.
        collection.set_audio_muted(audio_id, true).expect("mute");
        // Toggle the window hidden then visible again.
        collection
            .set_item_visible(scene, window_item, false)
            .expect("hide");
        collection
            .set_item_visible(scene, window_item, true)
            .expect("show");
        assert!(
            is_muted(&collection),
            "a manual mute survives a hide/show cycle"
        );
    }

    #[test]
    fn backdrop_sync_toggles_only_media_backdrops() {
        let mut collection = Collection::new();
        let scene = collection.active_scene;
        // No backdrop: error.
        assert!(collection.set_backdrop_sync(scene, true).is_err());

        collection
            .set_scene_backdrop(scene, Some(backdrop_media("wall.mp4")))
            .expect("set");
        collection.set_backdrop_sync(scene, true).expect("toggle");
        let flagged = collection.sources.iter().any(|source| {
            matches!(
                source.settings,
                SourceSettings::Media {
                    start_with_recording: true,
                    ..
                }
            )
        });
        assert!(flagged, "the media backdrop carries the flag");

        // A still-image backdrop has no playback to sync.
        collection
            .set_scene_backdrop(
                scene,
                Some(Source::new(
                    "Backdrop",
                    SourceSettings::Image {
                        path: "wall.png".into(),
                    },
                )),
            )
            .expect("image backdrop");
        assert!(collection.set_backdrop_sync(scene, true).is_err());
    }

    #[test]
    fn paste_filters_appends_with_fresh_ids() {
        let mut collection = Collection::new();
        let scene = collection.active_scene;
        let (_source, item) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "Cam",
                    SourceSettings::VideoDevice {
                        device_id: "c".into(),
                        format: None,
                        deinterlace: DeinterlaceMode::Off,
                        field_order: FieldOrder::TopFirst,
                    },
                ),
            )
            .expect("add");
        let original = collection
            .add_filter(scene, item, FilterKind::Sharpen { amount: 0.5 })
            .expect("add filter");

        // Copy the chain, flip the copy's enabled flag, paste it back.
        let mut copied = collection
            .scene(scene)
            .unwrap()
            .item(item)
            .unwrap()
            .filters
            .clone();
        copied[0].enabled = false;
        let added = collection
            .paste_filters(scene, item, copied)
            .expect("paste");
        assert_eq!(added, 1);

        let filters = &collection.scene(scene).unwrap().item(item).unwrap().filters;
        assert_eq!(filters.len(), 2, "the copy is appended on top");
        assert_ne!(filters[1].id, original, "the pasted filter gets a fresh id");
        assert!(!filters[1].enabled, "enabled state is preserved");
        assert_eq!(filters[1].kind, FilterKind::Sharpen { amount: 0.5 });
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
                        deinterlace: DeinterlaceMode::Off,
                        field_order: FieldOrder::TopFirst,
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
                        deinterlace: DeinterlaceMode::Off,
                        field_order: FieldOrder::TopFirst,
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
                deinterlace: DeinterlaceMode::Off,
                field_order: FieldOrder::TopFirst,
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
                deinterlace: DeinterlaceMode::Off,
                field_order: FieldOrder::TopFirst,
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
            source_file: String::new(),
            binding: FileBinding::Whole,
            csv_row: 1,
            csv_column: String::new(),
            json_pointer: String::new(),
        })
        .expect("serialize");
        for key in [
            "\"sizePx\"",
            "\"fontFamily\"",
            "\"lineSpacing\"",
            "\"forceRtl\"",
            "\"wrapWidth\"",
            "\"sourceFile\"",
            "\"jsonPointer\"",
        ] {
            assert!(text_json.contains(key), "missing {key} in {text_json}");
        }

        // CAP-M16: a pre-binding Text JSON (no sourceFile/binding fields)
        // still loads — the binding defaults to "off".
        let legacy: SourceSettings = serde_json::from_str(
            r#"{"kind":"text","text":"old","sizePx":48.0,
                "color":{"r":255,"g":255,"b":255,"a":255},"align":"left",
                "lineSpacing":1.0,"forceRtl":false}"#,
        )
        .expect("legacy text loads");
        assert!(matches!(
            legacy,
            SourceSettings::Text { ref source_file, binding: FileBinding::Whole, .. }
                if source_file.is_empty()
        ));

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
    fn downstream_keyers_add_reorder_and_prune() {
        let mut c = Collection::new();
        let scene = c.active_scene;
        let color = |name: &str| {
            Source::new(
                name,
                SourceSettings::Color {
                    color: Rgba::WHITE,
                    width: 100,
                    height: 100,
                },
            )
        };
        let (src_a, _) = c.add_item_with_new_source(scene, color("Logo")).unwrap();
        let (src_b, _) = c.add_item_with_new_source(scene, color("Bug")).unwrap();

        let a = c.add_downstream(src_a).expect("added");
        let b = c.add_downstream(src_b).expect("added");
        assert_eq!(c.downstream.len(), 2);
        assert!(
            c.is_source_referenced(src_a),
            "a keyer references its source"
        );
        assert!(
            c.add_downstream(SourceId::new()).is_none(),
            "unknown source rejected"
        );

        // List order is bottom-to-top: b is on top. Move a up past b.
        assert_eq!(c.downstream[0].id, a);
        assert!(c.move_downstream(a, true));
        assert_eq!([c.downstream[0].id, c.downstream[1].id], [b, a]);
        assert!(!c.move_downstream(a, true), "already on top");

        // Opacity is clamped by sanitize.
        c.downstream_mut(a).unwrap().opacity = 5.0;
        c.sanitize();
        assert_eq!(c.downstream_mut(a).unwrap().opacity, 1.0);

        // Removing a keyer drops just that layer.
        assert!(c.remove_downstream(b));
        assert_eq!(c.downstream.len(), 1);

        // A keyer whose source vanished out from under it is pruned by sanitize.
        c.downstream.push(DownstreamKeyer {
            id: DskId::new(),
            source: SourceId::new(),
            enabled: true,
            opacity: 1.0,
            transform: Transform::default(),
        });
        let before = c.downstream.len();
        c.sanitize();
        assert_eq!(c.downstream.len(), before - 1, "orphan keyer pruned");
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
    fn focus_promotes_hides_and_restores_exactly() {
        let mut collection = Collection::new();
        let scene = collection.active_scene;
        let color = |name: &str| {
            Source::new(
                name,
                SourceSettings::Color {
                    color: Rgba::new(1, 2, 3, 255),
                    width: 100,
                    height: 50,
                },
            )
        };
        let (_, cam) = collection
            .add_item_with_new_source(scene, color("Cam"))
            .expect("add cam");
        let (_, screen) = collection
            .add_item_with_new_source(scene, color("Screen"))
            .expect("add screen");
        let placed = Transform {
            x: 111.0,
            y: 222.0,
            ..Transform::default()
        };
        collection
            .set_item_transform(scene, cam, placed)
            .expect("place the cam");

        collection.set_focus(scene, cam).expect("focus the cam");
        let s = collection.scene(scene).expect("scene");
        let focused = s.item(cam).expect("cam item");
        assert!(focused.visible && focused.pending_fit && focused.pending_slot.is_none());
        assert!(!s.item(screen).expect("screen item").visible, "others hide");
        assert_eq!(s.focus.as_ref().expect("focus set").item, cam);

        collection.clear_focus(scene).expect("unfocus");
        let s = collection.scene(scene).expect("scene");
        assert!(s.focus.is_none());
        let cam_item = s.item(cam).expect("cam item");
        assert_eq!(cam_item.transform, placed, "transform restored exactly");
        assert!(!cam_item.pending_fit, "no stray refit after restore");
        assert!(
            s.item(screen).expect("screen item").visible,
            "visibility restored"
        );
    }

    #[test]
    fn focus_leaves_audio_only_items_audible() {
        let mut collection = Collection::new();
        let scene = collection.active_scene;
        let (_, mic) = collection
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
        let (_, cam) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "Cam",
                    SourceSettings::Color {
                        color: Rgba::new(1, 2, 3, 255),
                        width: 100,
                        height: 50,
                    },
                ),
            )
            .expect("add cam");

        collection.set_focus(scene, cam).expect("focus the cam");
        let s = collection.scene(scene).expect("scene");
        assert!(
            s.item(mic).expect("mic item").visible,
            "hiding an audio-only item would mute it mid-show"
        );
    }

    #[test]
    fn refocus_retargets_from_the_original_layout() {
        let mut collection = Collection::new();
        let scene = collection.active_scene;
        let color = |name: &str| {
            Source::new(
                name,
                SourceSettings::Color {
                    color: Rgba::new(1, 2, 3, 255),
                    width: 100,
                    height: 50,
                },
            )
        };
        let (_, a) = collection
            .add_item_with_new_source(scene, color("A"))
            .expect("add a");
        let (_, b) = collection
            .add_item_with_new_source(scene, color("B"))
            .expect("add b");

        collection.set_focus(scene, a).expect("focus a");
        collection.set_focus(scene, b).expect("re-focus b");
        let s = collection.scene(scene).expect("scene");
        assert!(s.item(b).expect("b").visible && s.item(b).expect("b").pending_fit);
        assert!(!s.item(a).expect("a").visible, "a returns to non-focused");

        collection.clear_focus(scene).expect("unfocus");
        let s = collection.scene(scene).expect("scene");
        assert!(
            s.item(a).expect("a").visible && s.item(b).expect("b").visible,
            "the ORIGINAL layout returns, not the intermediate focus"
        );
    }

    #[test]
    fn removing_the_focused_item_restores_the_layout() {
        let mut collection = Collection::new();
        let scene = collection.active_scene;
        let color = |name: &str| {
            Source::new(
                name,
                SourceSettings::Color {
                    color: Rgba::new(1, 2, 3, 255),
                    width: 100,
                    height: 50,
                },
            )
        };
        let (_, a) = collection
            .add_item_with_new_source(scene, color("A"))
            .expect("add a");
        let (_, b) = collection
            .add_item_with_new_source(scene, color("B"))
            .expect("add b");

        collection.set_focus(scene, a).expect("focus a");
        collection
            .remove_item(scene, a)
            .expect("remove the focused item");
        let s = collection.scene(scene).expect("scene");
        assert!(s.focus.is_none(), "focus cleared with its item");
        assert!(
            s.item(b).expect("b").visible,
            "the hidden others come back instead of staying stranded"
        );
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
                        deinterlace: DeinterlaceMode::Off,
                        field_order: FieldOrder::TopFirst,
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
                        deinterlace: DeinterlaceMode::Off,
                        field_order: FieldOrder::TopFirst,
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
    fn set_item_slot_seats_one_item() {
        let mut collection = Collection::new();
        let scene = collection.active_scene;
        let (_, cam) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "Guest",
                    SourceSettings::RemoteGuest {
                        label: String::new(),
                    },
                ),
            )
            .expect("add guest");

        let slot = NormRect {
            x: 0.02,
            y: 0.35,
            w: 0.30,
            h: 0.30,
        };
        collection
            .set_item_slot(scene, cam, slot)
            .expect("seat the guest");

        let item = collection
            .scene(scene)
            .expect("scene")
            .item(cam)
            .expect("item");
        assert!(
            item.pending_fit,
            "placement resolves on the next sized frame"
        );
        assert_eq!(item.pending_slot, Some(slot));
    }

    #[test]
    fn set_item_slot_rejects_a_slot_outside_the_canvas() {
        let mut collection = Collection::new();
        let scene = collection.active_scene;
        let (_, cam) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "Guest",
                    SourceSettings::RemoteGuest {
                        label: String::new(),
                    },
                ),
            )
            .expect("add guest");
        let before = collection.scene(scene).expect("scene").items.clone();

        let bad = [
            // Overflows the right edge.
            NormRect {
                x: 0.8,
                y: 0.0,
                w: 0.3,
                h: 0.3,
            },
            // Negative origin.
            NormRect {
                x: -0.1,
                y: 0.0,
                w: 0.3,
                h: 0.3,
            },
            // Empty.
            NormRect {
                x: 0.0,
                y: 0.0,
                w: 0.0,
                h: 0.3,
            },
            // Non-finite.
            NormRect {
                x: f32::NAN,
                y: 0.0,
                w: 0.3,
                h: 0.3,
            },
        ];
        for slot in bad {
            let result = collection.set_item_slot(scene, cam, slot);
            assert!(matches!(result, Err(SceneError::InvalidSlot)), "{slot:?}");
        }
        assert_eq!(
            collection.scene(scene).expect("scene").items,
            before,
            "a rejected slot leaves the scene untouched"
        );
    }

    /// Two remote guests in a fresh collection — the seat-swap fixtures.
    fn two_guests(collection: &mut Collection) -> (SceneId, ItemId, ItemId) {
        let scene = collection.active_scene;
        let (_, guest1) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "Guest 1",
                    SourceSettings::RemoteGuest {
                        label: String::new(),
                    },
                ),
            )
            .expect("add guest 1");
        let (_, guest2) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "Guest 2",
                    SourceSettings::RemoteGuest {
                        label: String::new(),
                    },
                ),
            )
            .expect("add guest 2");
        (scene, guest1, guest2)
    }

    #[test]
    fn set_item_slot_swaps_with_the_occupant() {
        // Mike's example: guest 1 top-left, guest 2 top-right; moving guest 1
        // to the top-right puts guest 2 on the top-left — never stacked.
        let mut collection = Collection::new();
        let (scene, guest1, guest2) = two_guests(&mut collection);
        let top_left = Corner::TopLeft.slot();
        let top_right = Corner::TopRight.slot();
        collection
            .set_item_slot(scene, guest1, top_left)
            .expect("seat 1");
        collection
            .set_item_slot(scene, guest2, top_right)
            .expect("seat 2");

        collection
            .set_item_slot(scene, guest1, top_right)
            .expect("swap");

        let scene_ref = collection.scene(scene).expect("scene");
        let seat_of = |id| {
            scene_ref
                .item(id)
                .expect("item")
                .pending_slot
                .expect("seated")
        };
        assert!(same_seat(seat_of(guest1), top_right));
        assert!(
            same_seat(seat_of(guest2), top_left),
            "the occupant took the vacated seat"
        );
        assert!(
            scene_ref.item(guest2).expect("g2").pending_fit,
            "the bumped guest re-places"
        );
    }

    #[test]
    fn set_item_slot_bumps_the_occupant_to_a_free_seat_when_the_mover_was_unseated() {
        let mut collection = Collection::new();
        let (scene, guest1, guest2) = two_guests(&mut collection);
        let top_right = Corner::TopRight.slot();
        collection
            .set_item_slot(scene, guest2, top_right)
            .expect("seat 2");

        // Guest 1 was never seated — the occupant still has to move away.
        collection
            .set_item_slot(scene, guest1, top_right)
            .expect("take the seat");

        let scene_ref = collection.scene(scene).expect("scene");
        let g1 = scene_ref
            .item(guest1)
            .expect("g1")
            .pending_slot
            .expect("g1 seated");
        let g2 = scene_ref
            .item(guest2)
            .expect("g2")
            .pending_slot
            .expect("g2 reseated");
        assert!(same_seat(g1, top_right));
        assert!(!same_seat(g2, top_right), "no two guests share a seat");
        assert!(
            scene::preset_seats()
                .iter()
                .any(|seat| same_seat(*seat, g2)),
            "the bump lands on a preset seat"
        );
    }

    #[test]
    fn resolve_pending_keeps_the_seat_but_a_user_transform_vacates_it() {
        let mut collection = Collection::new();
        let (scene, guest1, _) = two_guests(&mut collection);
        let seat = Corner::TopLeft.slot();
        collection.set_item_slot(scene, guest1, seat).expect("seat");

        // The engine resolves the placement — the seat is remembered.
        collection
            .resolve_pending(scene, guest1, Transform::default())
            .expect("resolve");
        let item = collection
            .scene(scene)
            .expect("scene")
            .item(guest1)
            .expect("item");
        assert!(!item.pending_fit);
        assert_eq!(item.pending_slot, Some(seat));

        // A user drag vacates the seat.
        collection
            .set_item_transform(scene, guest1, Transform::default())
            .expect("drag");
        let item = collection
            .scene(scene)
            .expect("scene")
            .item(guest1)
            .expect("item");
        assert_eq!(item.pending_slot, None);
    }

    #[test]
    fn focus_restore_keeps_the_seats() {
        let mut collection = Collection::new();
        let (scene, guest1, guest2) = two_guests(&mut collection);
        let seat = Corner::BottomRight.slot();
        collection.set_item_slot(scene, guest2, seat).expect("seat");
        collection
            .resolve_pending(scene, guest2, Transform::default())
            .expect("resolve");

        collection.set_focus(scene, guest1).expect("focus");
        collection.clear_focus(scene).expect("restore");

        let item = collection
            .scene(scene)
            .expect("scene")
            .item(guest2)
            .expect("item");
        assert_eq!(
            item.pending_slot,
            Some(seat),
            "the seat survives a spotlight round-trip"
        );
    }

    #[test]
    fn center_view_swaps_with_the_displaced_center() {
        // Mike's example, under the no-overlap rule: the cam starts top-left;
        // centering the display bumps it to the rail (nothing may overlap the
        // shared view); promoting the cam then trades seats — the display
        // takes the cam's actual seat.
        let mut collection = Collection::new();
        let scene = collection.active_scene;
        let (_, display) = collection
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
            .expect("add display");
        let (_, cam) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "Cam",
                    SourceSettings::VideoDevice {
                        device_id: String::new(),
                        format: None,
                        deinterlace: DeinterlaceMode::Off,
                        field_order: FieldOrder::TopFirst,
                    },
                ),
            )
            .expect("add cam");
        collection
            .set_item_slot(scene, cam, Corner::TopLeft.slot())
            .expect("seat the cam");
        collection
            .set_center_view(scene, Some(display))
            .expect("center the display");
        let cam_seat_before = collection
            .scene(scene)
            .expect("scene")
            .item(cam)
            .expect("cam")
            .pending_slot
            .expect("cam seated");

        collection
            .set_center_view(scene, Some(cam))
            .expect("center the cam");

        let scene_ref = collection.scene(scene).expect("scene");
        let seat = |id| {
            scene_ref
                .item(id)
                .expect("item")
                .pending_slot
                .expect("seated")
        };
        assert!(same_seat(seat(cam), scene::center_slot()));
        assert!(
            same_seat(seat(display), cam_seat_before),
            "the displaced center takes the promoted cam's old seat"
        );
        assert!(
            !scene::rects_overlap(seat(display), scene::center_slot()),
            "the displaced screen never overlaps the new center view"
        );
    }

    #[test]
    fn center_view_bumps_overlapping_seats_to_the_rail() {
        let mut collection = Collection::new();
        let (scene, guest1, guest2) = two_guests(&mut collection);
        // Both guests sit in left/middle seats that intersect the center region.
        collection
            .set_item_slot(scene, guest2, Corner::TopLeft.slot())
            .expect("seat 2");
        let (_, display) = collection
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
            .expect("add display");

        collection
            .set_center_view(scene, Some(display))
            .expect("center the display");

        let scene_ref = collection.scene(scene).expect("scene");
        let center = scene::center_slot();
        for id in [guest1, guest2] {
            let item = scene_ref.item(id).expect("guest");
            if let Some(seat) = item.pending_slot {
                assert!(
                    !scene::rects_overlap(seat, center),
                    "no cam may overlap the shared view: {seat:?}"
                );
            }
        }
    }

    #[test]
    fn center_view_shows_one_screen_at_a_time() {
        let mut collection = Collection::new();
        let scene = collection.active_scene;
        let mk_display = |name: &str| {
            Source::new(
                name,
                SourceSettings::Display {
                    capture_id: String::new(),
                    label: String::new(),
                },
            )
        };
        let (_, display1) = collection
            .add_item_with_new_source(scene, mk_display("Screen 1"))
            .expect("add 1");
        let (_, display2) = collection
            .add_item_with_new_source(scene, mk_display("Screen 2"))
            .expect("add 2");

        collection
            .set_center_view(scene, Some(display1))
            .expect("center screen 1");

        let scene_ref = collection.scene(scene).expect("scene");
        assert!(scene_ref.item(display1).expect("d1").visible);
        assert!(
            !scene_ref.item(display2).expect("d2").visible,
            "only one screen view is visible at a time"
        );
    }

    #[test]
    fn center_view_retires_to_the_rail() {
        let mut collection = Collection::new();
        let (scene, guest1, _) = two_guests(&mut collection);
        collection
            .set_center_view(scene, Some(guest1))
            .expect("center the guest");

        collection
            .set_center_view(scene, None)
            .expect("retire the center");

        let seat = collection
            .scene(scene)
            .expect("scene")
            .item(guest1)
            .expect("guest")
            .pending_slot
            .expect("still seated");
        assert!(
            scene::rail_seats()
                .iter()
                .any(|rail| same_seat(*rail, seat)),
            "the retired center joins the rail"
        );
    }

    #[test]
    fn center_and_rail_geometry_do_not_overlap() {
        let center = scene::center_slot();
        let rail = scene::rail_seats();
        for seat in rail {
            assert!(
                !scene::rects_overlap(center, seat),
                "{seat:?} overlaps the center"
            );
        }
        for (i, a) in rail.iter().enumerate() {
            for b in rail.iter().skip(i + 1) {
                assert!(!scene::rects_overlap(*a, *b), "{a:?} overlaps {b:?}");
            }
        }
    }

    #[test]
    fn focus_restore_never_double_books_a_seat_claimed_while_focused() {
        let mut collection = Collection::new();
        let (scene, guest1, guest2) = two_guests(&mut collection);
        let seat = Corner::TopLeft.slot();
        collection
            .set_item_slot(scene, guest1, seat)
            .expect("seat guest 1");
        collection
            .resolve_pending(scene, guest1, Transform::default())
            .expect("resolve");

        // Spotlight guest 2; while focused, guest 1 looks parked, and the
        // host seats ANOTHER item on the same top-left seat.
        collection.set_focus(scene, guest2).expect("focus");
        let (_, late) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "Late guest",
                    SourceSettings::RemoteGuest {
                        label: String::new(),
                    },
                ),
            )
            .expect("add late guest");
        collection
            .set_item_slot(scene, late, seat)
            .expect("seat the late guest");

        collection.clear_focus(scene).expect("restore");

        let scene_ref = collection.scene(scene).expect("scene");
        let holders = scene_ref
            .items
            .iter()
            .filter(|item| item.pending_slot.is_some_and(|s| same_seat(s, seat)))
            .count();
        assert_eq!(
            holders, 1,
            "exactly one item may hold a seat after a focus round-trip"
        );
    }

    #[test]
    fn set_item_slot_redirects_center_overlapping_seats_to_the_rail() {
        let mut collection = Collection::new();
        let (scene, guest1, _) = two_guests(&mut collection);
        let (_, display) = collection
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
            .expect("add display");
        collection
            .set_center_view(scene, Some(display))
            .expect("center the display");

        // Middle-left intersects the center region — the guest must land on
        // the rail instead of on top of the shared view.
        let middle_left = scene::preset_seats()[2];
        collection
            .set_item_slot(scene, guest1, middle_left)
            .expect("seat while a center view is up");

        let seat = collection
            .scene(scene)
            .expect("scene")
            .item(guest1)
            .expect("guest")
            .pending_slot
            .expect("seated");
        assert!(
            !scene::rects_overlap(seat, scene::center_slot()),
            "no cam may overlap the shared view: {seat:?}"
        );
        assert!(
            scene::rail_seats()
                .iter()
                .any(|rail| same_seat(*rail, seat)),
            "the redirect lands on a rail seat"
        );
    }

    #[test]
    fn added_video_items_seat_into_free_corners_not_on_top() {
        let mut collection = Collection::new();
        let scene = collection.active_scene;
        let mk_cam = |name: &str| {
            Source::new(
                name,
                SourceSettings::VideoDevice {
                    device_id: String::new(),
                    format: None,
                    deinterlace: DeinterlaceMode::Off,
                    field_order: FieldOrder::TopFirst,
                },
            )
        };

        // First video item fills the canvas (no seat).
        let (_, first) = collection
            .add_item_with_new_source(scene, mk_cam("Cam 1"))
            .expect("add cam 1");
        assert_eq!(
            collection
                .scene(scene)
                .expect("scene")
                .item(first)
                .expect("first")
                .pending_slot,
            None,
            "the first item fills the canvas"
        );

        // Second + third get distinct free corners — never dumped on top.
        let (_, second) = collection
            .add_item_with_new_source(scene, mk_cam("Cam 2"))
            .expect("add cam 2");
        let (_, third) = collection
            .add_item_with_new_source(scene, mk_cam("Cam 3"))
            .expect("add cam 3");
        let scene_ref = collection.scene(scene).expect("scene");
        let s2 = scene_ref
            .item(second)
            .expect("second")
            .pending_slot
            .expect("seated");
        let s3 = scene_ref
            .item(third)
            .expect("third")
            .pending_slot
            .expect("seated");
        assert!(
            !same_seat(s2, s3),
            "each added source takes a distinct corner"
        );
        assert!(scene::preset_seats()
            .iter()
            .any(|seat| same_seat(*seat, s2)));
        assert!(scene::preset_seats()
            .iter()
            .any(|seat| same_seat(*seat, s3)));
    }

    #[test]
    fn added_audio_only_items_are_never_seated() {
        let mut collection = Collection::new();
        let scene = collection.active_scene;
        // A camera then a mic: the mic renders nothing, so it gets no seat and
        // does not count toward corner placement.
        collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "Cam",
                    SourceSettings::VideoDevice {
                        device_id: String::new(),
                        format: None,
                        deinterlace: DeinterlaceMode::Off,
                        field_order: FieldOrder::TopFirst,
                    },
                ),
            )
            .expect("add cam");
        let (_, mic) = collection
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
        assert_eq!(
            collection
                .scene(scene)
                .expect("scene")
                .item(mic)
                .expect("mic")
                .pending_slot,
            None,
            "an audio-only source is never seated"
        );
    }

    #[test]
    fn preset_seats_do_not_overlap() {
        let seats = scene::preset_seats();
        for (i, a) in seats.iter().enumerate() {
            for b in seats.iter().skip(i + 1) {
                let apart =
                    a.x + a.w <= b.x || b.x + b.w <= a.x || a.y + a.h <= b.y || b.y + b.h <= a.y;
                assert!(apart, "{a:?} overlaps {b:?}");
            }
        }
    }

    #[test]
    fn reorder_scene_moves_and_never_duplicates() {
        let mut c = Collection::new();
        let a = c.active_scene;
        let b = c.add_scene("B");
        let d = c.add_scene("D");
        assert_eq!(
            c.scenes.iter().map(|s| s.id).collect::<Vec<_>>(),
            vec![a, b, d]
        );

        // Move the 2nd scene (b) up to index 0 — it swaps above a, no copy.
        c.reorder_scene(b, 0).unwrap();
        assert_eq!(
            c.scenes.iter().map(|s| s.id).collect::<Vec<_>>(),
            vec![b, a, d],
            "up = a clean move, not a duplicate"
        );
        assert_eq!(c.scenes.len(), 3, "count unchanged");

        // Move d down past the end — clamps to last, still no dup.
        c.reorder_scene(d, 99).unwrap();
        assert_eq!(c.scenes.last().unwrap().id, d);

        // Every scene id is still unique (never two of the same scene).
        let mut ids: Vec<_> = c.scenes.iter().map(|s| s.id).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), 3, "no scene was duplicated");
    }

    #[test]
    fn set_item_slot_rejects_an_unknown_item() {
        let mut collection = Collection::new();
        let scene = collection.active_scene;
        let slot = NormRect {
            x: 0.02,
            y: 0.02,
            w: 0.30,
            h: 0.30,
        };
        let result = collection.set_item_slot(scene, ItemId::new(), slot);
        assert!(matches!(result, Err(SceneError::ItemNotFound)));
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
                        deinterlace: DeinterlaceMode::Off,
                        field_order: FieldOrder::TopFirst,
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

    #[test]
    fn nested_scene_cycles_are_rejected() {
        let mut collection = Collection::new();
        let scene_a = collection.active_scene;
        let scene_b = collection.add_scene("Inner");

        // A shows B — fine.
        let (nested_b, _) = collection
            .add_item_with_new_source(
                scene_a,
                Source::new("Inner view", SourceSettings::NestedScene { scene: scene_b }),
            )
            .expect("A can nest B");

        // B showing A would loop A→B→A.
        let err = collection
            .add_item_with_new_source(
                scene_b,
                Source::new("Loop", SourceSettings::NestedScene { scene: scene_a }),
            )
            .unwrap_err();
        assert_eq!(err, SceneError::SceneCycle);
        assert!(
            !collection
                .sources
                .iter()
                .any(|source| source.name == "Loop"),
            "the rejected source rolled back out of the pool"
        );

        // A scene can never nest itself.
        let err = collection
            .add_item_with_new_source(
                scene_a,
                Source::new("Selfie", SourceSettings::NestedScene { scene: scene_a }),
            )
            .unwrap_err();
        assert_eq!(err, SceneError::SceneCycle);

        // Repointing the existing nested source at its holder loops too.
        let err = collection
            .update_source_settings(nested_b, SourceSettings::NestedScene { scene: scene_a })
            .unwrap_err();
        assert_eq!(err, SceneError::SceneCycle);

        // Removing B kills the nested source that showed it.
        collection.remove_scene(scene_b).expect("removable");
        assert!(collection.source(nested_b).is_none());
        assert!(collection.scene(scene_a).expect("A lives").items.is_empty());
    }

    #[test]
    fn groups_move_and_hide_together() {
        let mut collection = Collection::new();
        let scene = collection.active_scene;
        let (_, item_a) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "Cam",
                    SourceSettings::Color {
                        color: Rgba::WHITE,
                        width: 100,
                        height: 100,
                    },
                ),
            )
            .expect("adds");
        let (_, item_b) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "Logo",
                    SourceSettings::Color {
                        color: Rgba::WHITE,
                        width: 10,
                        height: 10,
                    },
                ),
            )
            .expect("adds");

        let group = collection
            .create_group(scene, "Overlay", &[item_a, item_b])
            .expect("groups");
        assert_eq!(
            collection
                .create_group(scene, "Again", &[item_a])
                .unwrap_err(),
            SceneError::AlreadyGrouped
        );

        // Moving one member carries the other by the same delta.
        let before_b = collection
            .scene(scene)
            .unwrap()
            .item(item_b)
            .unwrap()
            .transform;
        let mut moved = collection
            .scene(scene)
            .unwrap()
            .item(item_a)
            .unwrap()
            .transform;
        moved.x += 40.0;
        moved.y -= 10.0;
        collection
            .set_item_transform(scene, item_a, moved)
            .expect("moves");
        let after_b = collection
            .scene(scene)
            .unwrap()
            .item(item_b)
            .unwrap()
            .transform;
        assert_eq!(after_b.x, before_b.x + 40.0);
        assert_eq!(after_b.y, before_b.y - 10.0);

        // The group eye hides every member (without touching their own flag).
        collection
            .set_group_visible(scene, group, false)
            .expect("hides");
        let scene_ref = collection.scene(scene).unwrap();
        assert!(scene_ref.group_hides(item_a));
        assert!(scene_ref.group_hides(item_b));
        assert!(
            scene_ref.item(item_a).unwrap().visible,
            "own flag untouched"
        );

        // Removing a member prunes it from the group; the last removal
        // dissolves the group.
        collection.remove_item(scene, item_a).expect("removes");
        assert_eq!(
            collection
                .scene(scene)
                .unwrap()
                .group_of(item_b)
                .map(|g| g.id),
            Some(group)
        );
        collection.remove_item(scene, item_b).expect("removes");
        assert!(collection.scene(scene).unwrap().groups.is_empty());
    }

    #[test]
    fn per_scene_audio_overrides_apply_and_sanitize() {
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
            .expect("adds");
        let (color, _) = collection
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
            .expect("adds");

        assert_eq!(
            collection
                .set_scene_audio_override(scene, color, Some((0.0, false)))
                .unwrap_err(),
            SceneError::SourceNotAudio
        );

        collection
            .set_scene_audio_override(scene, mic, Some((-990.0, true)))
            .expect("sets");
        let entry = collection.scene(scene).unwrap().audio_overrides[0];
        assert_eq!(entry.volume_db, MIN_VOLUME_DB, "clamped");
        assert!(entry.muted);

        // Clearing removes the entry.
        collection
            .set_scene_audio_override(scene, mic, None)
            .expect("clears");
        assert!(collection.scene(scene).unwrap().audio_overrides.is_empty());
    }
}
