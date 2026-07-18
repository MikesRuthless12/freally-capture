//! CAP-N61: scene-collection **diff & merge**.
//!
//! Compare two collections — typically a collection and its backup, a
//! [`crate::Collection`] and one imported from a `.fcappack`, or a named
//! snapshot (CAP-N62) — and report what changed at the scene, source and item
//! level, then let the operator **cherry-pick** which changes to merge in. All
//! local, no server: two people can trade `.fcappack`s and reconcile by hand.
//!
//! Matching is by **stable id** ([`crate::SceneId`] / [`crate::SourceId`] /
//! [`crate::ItemId`]), so a collection and a copy of it line up cleanly and only
//! genuine edits surface. Two *unrelated* collections share no ids, so every
//! scene/source reads as added-or-removed — honest, if not useful; the feature
//! is built for collections that share lineage.

use serde::{Deserialize, Serialize};

use crate::{Collection, ItemId, Scene, SceneId, SceneItem, SourceId};

/// Whether an entity is new in `other`, gone from `other`, or edited.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChangeKind {
    Added,
    Removed,
    Modified,
}

/// One changed item within a modified scene.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemChange {
    pub item: ItemId,
    /// The item's source name (for a human-readable row).
    pub source_name: String,
    pub kind: ChangeKind,
    /// For `Modified`: which facets differ — any of `transform`, `visibility`,
    /// `outputs`, `filters`, `lock`, `blend`, `scaling`, `backdrop`, `source`.
    pub aspects: Vec<String>,
}

/// One changed scene.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneChange {
    pub scene: SceneId,
    pub name: String,
    pub kind: ChangeKind,
    /// For `Modified`: the previous name, when the scene was renamed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renamed_from: Option<String>,
    /// For `Modified`: the shared items were re-stacked (z-order changed).
    #[serde(default)]
    pub reordered: bool,
    /// For `Modified`: per-item changes (empty for Added/Removed).
    #[serde(default)]
    pub items: Vec<ItemChange>,
}

/// One changed source (the shared pool).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceChange {
    pub source: SourceId,
    pub name: String,
    pub kind: ChangeKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renamed_from: Option<String>,
    /// For `Modified`: any of `name`, `settings`, `audio`.
    pub aspects: Vec<String>,
}

/// The full comparison of `other` against `base`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionDiff {
    pub sources: Vec<SourceChange>,
    pub scenes: Vec<SceneChange>,
    /// `sources.len() + scenes.len()` — the top-level change count.
    pub total: usize,
}

impl CollectionDiff {
    pub fn is_empty(&self) -> bool {
        self.sources.is_empty() && self.scenes.is_empty()
    }
}

fn source_name(collection: &Collection, id: SourceId) -> String {
    collection
        .source(id)
        .map(|source| source.name.clone())
        .unwrap_or_default()
}

/// Which facets of an item changed between `a` (base) and `b` (other).
fn item_aspects(a: &SceneItem, b: &SceneItem) -> Vec<String> {
    let mut aspects = Vec::new();
    if a.source != b.source {
        aspects.push("source".to_string());
    }
    if a.transform != b.transform {
        aspects.push("transform".to_string());
    }
    if a.visible != b.visible {
        aspects.push("visibility".to_string());
    }
    if a.on_stream != b.on_stream || a.on_record != b.on_record {
        aspects.push("outputs".to_string());
    }
    if a.filters != b.filters {
        aspects.push("filters".to_string());
    }
    if a.locked != b.locked {
        aspects.push("lock".to_string());
    }
    if a.blend != b.blend {
        aspects.push("blend".to_string());
    }
    if a.scaling != b.scaling {
        aspects.push("scaling".to_string());
    }
    if a.backdrop != b.backdrop {
        aspects.push("backdrop".to_string());
    }
    if a.reveal_ms != b.reveal_ms {
        aspects.push("reveal".to_string());
    }
    aspects
}

fn diff_items(
    base_scene: &Scene,
    other_scene: &Scene,
    base: &Collection,
    other: &Collection,
) -> Vec<ItemChange> {
    let mut changes = Vec::new();
    for item in &other_scene.items {
        match base_scene.items.iter().find(|i| i.id == item.id) {
            None => changes.push(ItemChange {
                item: item.id,
                source_name: source_name(other, item.source),
                kind: ChangeKind::Added,
                aspects: Vec::new(),
            }),
            Some(base_item) if base_item != item => changes.push(ItemChange {
                item: item.id,
                source_name: source_name(other, item.source),
                kind: ChangeKind::Modified,
                aspects: item_aspects(base_item, item),
            }),
            Some(_) => {}
        }
    }
    for item in &base_scene.items {
        if !other_scene.items.iter().any(|i| i.id == item.id) {
            changes.push(ItemChange {
                item: item.id,
                source_name: source_name(base, item.source),
                kind: ChangeKind::Removed,
                aspects: Vec::new(),
            });
        }
    }
    changes
}

/// The shared items appear in a different relative z-order (ignoring the ones
/// that were only added or only removed).
fn is_reordered(base_scene: &Scene, other_scene: &Scene) -> bool {
    let common_in_base: Vec<ItemId> = base_scene
        .items
        .iter()
        .map(|i| i.id)
        .filter(|id| other_scene.items.iter().any(|i| i.id == *id))
        .collect();
    let common_in_other: Vec<ItemId> = other_scene
        .items
        .iter()
        .map(|i| i.id)
        .filter(|id| base_scene.items.iter().any(|i| i.id == *id))
        .collect();
    common_in_base != common_in_other
}

/// Diff `other` against `base`: what would change if `base` became `other`.
pub fn diff_collections(base: &Collection, other: &Collection) -> CollectionDiff {
    let mut diff = CollectionDiff::default();

    // Sources (the shared pool) — scenes reference these, so report them first.
    for source in &other.sources {
        match base.source(source.id) {
            None => diff.sources.push(SourceChange {
                source: source.id,
                name: source.name.clone(),
                kind: ChangeKind::Added,
                renamed_from: None,
                aspects: Vec::new(),
            }),
            Some(base_source) if base_source != source => {
                let renamed_from =
                    (base_source.name != source.name).then(|| base_source.name.clone());
                let mut aspects = Vec::new();
                if renamed_from.is_some() {
                    aspects.push("name".to_string());
                }
                if base_source.settings != source.settings {
                    aspects.push("settings".to_string());
                }
                if base_source.audio != source.audio {
                    aspects.push("audio".to_string());
                }
                diff.sources.push(SourceChange {
                    source: source.id,
                    name: source.name.clone(),
                    kind: ChangeKind::Modified,
                    renamed_from,
                    aspects,
                });
            }
            Some(_) => {}
        }
    }
    for source in &base.sources {
        if other.source(source.id).is_none() {
            diff.sources.push(SourceChange {
                source: source.id,
                name: source.name.clone(),
                kind: ChangeKind::Removed,
                renamed_from: None,
                aspects: Vec::new(),
            });
        }
    }

    // Scenes.
    for scene in &other.scenes {
        match base.scenes.iter().find(|s| s.id == scene.id) {
            None => diff.scenes.push(SceneChange {
                scene: scene.id,
                name: scene.name.clone(),
                kind: ChangeKind::Added,
                renamed_from: None,
                reordered: false,
                items: Vec::new(),
            }),
            Some(base_scene) if base_scene != scene => {
                let renamed_from = (base_scene.name != scene.name).then(|| base_scene.name.clone());
                diff.scenes.push(SceneChange {
                    scene: scene.id,
                    name: scene.name.clone(),
                    kind: ChangeKind::Modified,
                    renamed_from,
                    reordered: is_reordered(base_scene, scene),
                    items: diff_items(base_scene, scene, base, other),
                });
            }
            Some(_) => {}
        }
    }
    for scene in &base.scenes {
        if !other.scenes.iter().any(|s| s.id == scene.id) {
            diff.scenes.push(SceneChange {
                scene: scene.id,
                name: scene.name.clone(),
                kind: ChangeKind::Removed,
                renamed_from: None,
                reordered: false,
                items: Vec::new(),
            });
        }
    }

    diff.total = diff.sources.len() + diff.scenes.len();
    diff
}

/// Merge selected scenes/sources from `other` into a copy of `base` and return
/// it. A selected id present in `other` is added/replaced; one absent from
/// `other` (i.e. a Removed change the user accepted) is deleted from the result.
/// Selecting a scene also pulls in every source it references, so the cherry-
/// picked scene lands playable. The result is [`Collection::sanitize`]d.
pub fn merge_selected(
    base: &Collection,
    other: &Collection,
    scenes: &[SceneId],
    sources: &[SourceId],
) -> Collection {
    let mut result = base.clone();

    // Sources first (a scene we add below may reference one of these).
    for &id in sources {
        match other.source(id) {
            Some(source) => match result.sources.iter_mut().find(|s| s.id == id) {
                Some(existing) => *existing = source.clone(),
                None => result.sources.push(source.clone()),
            },
            None => result.sources.retain(|s| s.id != id),
        }
    }

    for &id in scenes {
        match other.scenes.iter().find(|s| s.id == id) {
            Some(scene) => {
                // Ensure the scene's sources exist so its items don't get
                // sanitized away as dangling.
                for item in &scene.items {
                    if result.source(item.source).is_none() {
                        if let Some(source) = other.source(item.source) {
                            result.sources.push(source.clone());
                        }
                    }
                }
                match result.scenes.iter_mut().find(|s| s.id == id) {
                    Some(existing) => *existing = scene.clone(),
                    None => result.scenes.push(scene.clone()),
                }
            }
            None => result.scenes.retain(|s| s.id != id),
        }
    }

    result.sanitize();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Rgba, Source, SourceSettings, Transform};

    fn color(name: &str) -> Source {
        Source::new(
            name,
            SourceSettings::Color {
                color: Rgba::new(1, 2, 3, 255),
                width: 100,
                height: 100,
            },
        )
    }

    #[test]
    fn identical_collections_have_no_diff() {
        let base = Collection::new();
        let other = base.clone();
        let diff = diff_collections(&base, &other);
        assert!(diff.is_empty());
        assert_eq!(diff.total, 0);
    }

    #[test]
    fn detects_added_scene_and_removed_source() {
        let mut base = Collection::new();
        let scene = base.active_scene().id;
        // base has a color source + item; other removes the source and adds a scene.
        let (source_id, _item) = base.add_item_with_new_source(scene, color("Box")).unwrap();

        let mut other = base.clone();
        // remove the source's item then the source
        other.scenes[0].items.clear();
        other.sources.retain(|s| s.id != source_id);
        // add a brand-new scene
        other.scenes.push(Scene::new("Intro"));

        let diff = diff_collections(&base, &other);
        assert_eq!(
            diff.sources
                .iter()
                .filter(|c| c.kind == ChangeKind::Removed)
                .count(),
            1
        );
        assert_eq!(
            diff.scenes
                .iter()
                .filter(|c| c.kind == ChangeKind::Added)
                .count(),
            1
        );
    }

    #[test]
    fn detects_item_transform_change_and_scene_rename() {
        let mut base = Collection::new();
        let scene = base.active_scene().id;
        let (_src, item) = base.add_item_with_new_source(scene, color("Box")).unwrap();

        let mut other = base.clone();
        other.scenes[0].name = "Renamed".to_string();
        let target = other.scenes[0]
            .items
            .iter_mut()
            .find(|i| i.id == item)
            .unwrap();
        target.transform = Transform {
            x: 500.0,
            ..target.transform
        };

        let diff = diff_collections(&base, &other);
        assert_eq!(diff.scenes.len(), 1);
        let scene_change = &diff.scenes[0];
        assert_eq!(scene_change.kind, ChangeKind::Modified);
        assert_eq!(scene_change.renamed_from.as_deref(), Some("Scene"));
        assert_eq!(scene_change.items.len(), 1);
        assert_eq!(scene_change.items[0].kind, ChangeKind::Modified);
        assert!(scene_change.items[0]
            .aspects
            .contains(&"transform".to_string()));
    }

    #[test]
    fn detects_pure_reorder() {
        let mut base = Collection::new();
        let scene = base.active_scene().id;
        base.add_item_with_new_source(scene, color("A")).unwrap();
        base.add_item_with_new_source(scene, color("B")).unwrap();

        let mut other = base.clone();
        other.scenes[0].items.reverse();

        let diff = diff_collections(&base, &other);
        assert_eq!(diff.scenes.len(), 1);
        assert!(diff.scenes[0].reordered);
        // A pure reorder changes no item fields, so no per-item Modified rows.
        assert!(diff.scenes[0].items.is_empty());
    }

    #[test]
    fn merge_pulls_in_a_scene_and_its_source() {
        let base = Collection::new();

        // other = base + a new scene carrying a new source.
        let mut other = base.clone();
        let mut intro = Scene::new("Intro");
        let src = color("Logo");
        let src_id = src.id;
        let item = SceneItem::new(src_id);
        intro.items.push(item);
        other.sources.push(src);
        let intro_id = intro.id;
        other.scenes.push(intro);

        // Merge only the scene; its source must ride along.
        let merged = merge_selected(&base, &other, &[intro_id], &[]);
        assert!(merged.scenes.iter().any(|s| s.id == intro_id));
        assert!(
            merged.source(src_id).is_some(),
            "the scene's source must be pulled in so its item survives sanitize"
        );
        assert!(merged
            .scenes
            .iter()
            .find(|s| s.id == intro_id)
            .unwrap()
            .items
            .iter()
            .any(|i| i.source == src_id));
    }
}
