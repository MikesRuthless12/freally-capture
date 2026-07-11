//! Multi-step undo/redo for the scene collection (CAP-M01).
//!
//! [`History`] is a **snapshot** stack: it keeps whole-[`Collection`] clones
//! taken *before* each mutation. Because [`Collection`] is `Clone + PartialEq`
//! and every edit is a method on it, one snapshot mechanism covers every
//! mutation — add/remove/reorder, transform, crop, filter edits, visibility,
//! audio — with no per-operation inverse to author or keep in sync. Snapshots
//! are cheap: sources hold *paths*, not pixels, so a snapshot is kilobytes.
//!
//! Two behaviours make the stack usable rather than merely correct:
//!
//! - **No-op suppression.** [`History::edit`] compares the collection before
//!   and after and records nothing when a mutation changed nothing (a rejected
//!   edit, or setting a value to what it already was) — so Ctrl+Z never lands
//!   on a step that does nothing visible.
//! - **Gesture coalescing.** Dragging a handle or riding a fader fires many
//!   mutations a second; checkpoints sharing a `coalesce_key` within
//!   [`COALESCE_WINDOW_MS`] fold into a single undo step, so one drag is one
//!   Ctrl+Z. Discrete edits pass `None` and always stand alone.
//!
//! The clock is injected (`now_ms`) so the whole module is deterministic under
//! test; the app passes real monotonic milliseconds.

use std::collections::VecDeque;

use serde::Serialize;

use crate::Collection;

/// Default cap on undo depth. Snapshots are small, so this is generous while
/// still bounding memory for a pathological session.
pub const DEFAULT_CAPACITY: usize = 100;

/// Checkpoints sharing a coalesce key that arrive within this many milliseconds
/// of the previous one collapse into a single undo step (one drag = one undo).
pub const COALESCE_WINDOW_MS: u64 = 700;

/// One saved pre-mutation state plus the label of the edit it precedes.
#[derive(Debug, Clone)]
struct Entry {
    /// The collection as it was *before* the edit this entry guards.
    collection: Collection,
    /// Human label of the edit (what an undo of this entry would reverse).
    label: String,
    /// Set while a continuous gesture is still folding into this entry.
    coalesce: Option<Coalesce>,
}

#[derive(Debug, Clone, Copy)]
struct Coalesce {
    key: u64,
    at_ms: u64,
}

/// The undo/redo stack for a single scene collection.
///
/// `past` holds pre-edit snapshots oldest→newest; its back is the next undo.
/// `future` holds snapshots produced by undoing, its top the next redo. A fresh
/// edit clears `future` (history forks — the redone branch is abandoned).
#[derive(Debug, Clone)]
pub struct History {
    past: VecDeque<Entry>,
    future: Vec<Entry>,
    capacity: usize,
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}

impl History {
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_CAPACITY)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            past: VecDeque::new(),
            future: Vec::new(),
            capacity: capacity.max(1),
        }
    }

    /// Forget all history (call when a *different* collection is loaded — a new
    /// project, an OBS import, a snapshot restore — so undo can't cross-load).
    pub fn clear(&mut self) {
        self.past.clear();
        self.future.clear();
    }

    pub fn can_undo(&self) -> bool {
        !self.past.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.future.is_empty()
    }

    /// Label of the edit the next Ctrl+Z would reverse.
    pub fn undo_label(&self) -> Option<&str> {
        self.past.back().map(|entry| entry.label.as_str())
    }

    /// Label of the edit the next Ctrl+Shift+Z would replay.
    pub fn redo_label(&self) -> Option<&str> {
        self.future.last().map(|entry| entry.label.as_str())
    }

    /// Run `mutate` against `collection`, recording an undo checkpoint **iff it
    /// changed the collection**. Errors from `mutate` leave the history
    /// untouched (the `?`/early-return happens before anything is recorded).
    /// Continuous gestures that pass the same `Some(coalesce_key)` within
    /// [`COALESCE_WINDOW_MS`] fold into one step; discrete edits pass `None`.
    ///
    /// This is the single choke point every mutating command funnels through.
    pub fn edit<T, E>(
        &mut self,
        collection: &mut Collection,
        label: impl Into<String>,
        coalesce_key: Option<u64>,
        now_ms: u64,
        mutate: impl FnOnce(&mut Collection) -> Result<T, E>,
    ) -> Result<T, E> {
        // Inside an open gesture the pre-state is already saved — skip the clone.
        let folding = coalesce_key.is_some_and(|key| self.is_open_gesture(key, now_ms));
        let before = if folding {
            None
        } else {
            Some(collection.clone())
        };

        let out = mutate(collection)?;

        match before {
            // A first (or discrete) edit: record only a real change.
            Some(before) if *collection != before => {
                self.push_entry(before, label.into(), coalesce_key, now_ms);
            }
            Some(_) => { /* no visible change — record nothing */ }
            // A later frame of an open gesture: extend the window, keep the
            // single saved pre-gesture state.
            None => {
                if let Some(key) = coalesce_key {
                    self.touch_gesture(key, now_ms);
                }
            }
        }
        Ok(out)
    }

    /// Undo the newest recorded edit, mutating `collection` back to its
    /// pre-edit state and returning that edit's label. `None` when there is
    /// nothing to undo (the collection is left untouched).
    pub fn undo(&mut self, collection: &mut Collection) -> Option<String> {
        let entry = self.past.pop_back()?;
        let restored = entry.collection;
        let post = std::mem::replace(collection, restored);
        self.future.push(Entry {
            collection: post,
            label: entry.label.clone(),
            coalesce: None,
        });
        Some(entry.label)
    }

    /// Redo the most recently undone edit, returning its label. `None` when
    /// there is nothing to redo.
    pub fn redo(&mut self, collection: &mut Collection) -> Option<String> {
        let entry = self.future.pop()?;
        let restored = entry.collection;
        let pre = std::mem::replace(collection, restored);
        self.past.push_back(Entry {
            collection: pre,
            label: entry.label.clone(),
            coalesce: None,
        });
        Some(entry.label)
    }

    /// A serializable view for the UI: the next-action labels plus the full
    /// stacks for a viewable history list (undo oldest→newest, redo top last).
    pub fn state(&self) -> HistoryState {
        HistoryState {
            can_undo: self.can_undo(),
            can_redo: self.can_redo(),
            undo_label: self.undo_label().map(str::to_owned),
            redo_label: self.redo_label().map(str::to_owned),
            undo: self.past.iter().map(|entry| entry.label.clone()).collect(),
            redo: self
                .future
                .iter()
                .map(|entry| entry.label.clone())
                .collect(),
        }
    }

    // -- internals ---------------------------------------------------------

    fn is_open_gesture(&self, key: u64, now_ms: u64) -> bool {
        self.past.back().is_some_and(|entry| {
            entry.coalesce.is_some_and(|c| {
                c.key == key && now_ms.saturating_sub(c.at_ms) <= COALESCE_WINDOW_MS
            })
        })
    }

    fn touch_gesture(&mut self, key: u64, now_ms: u64) {
        if let Some(entry) = self.past.back_mut() {
            entry.coalesce = Some(Coalesce { key, at_ms: now_ms });
        }
    }

    fn push_entry(
        &mut self,
        before: Collection,
        label: String,
        coalesce_key: Option<u64>,
        now_ms: u64,
    ) {
        self.past.push_back(Entry {
            collection: before,
            label,
            coalesce: coalesce_key.map(|key| Coalesce { key, at_ms: now_ms }),
        });
        while self.past.len() > self.capacity {
            self.past.pop_front();
        }
        self.future.clear();
    }
}

/// UI-facing snapshot of the stack (emitted after every edit/undo/redo).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryState {
    pub can_undo: bool,
    pub can_redo: bool,
    /// Label the next undo would reverse (menu item / tooltip).
    pub undo_label: Option<String>,
    /// Label the next redo would replay.
    pub redo_label: Option<String>,
    /// Every undoable edit, oldest → newest (the newest is the next undo).
    pub undo: Vec<String>,
    /// Every undone edit still redoable; the last is the next redo.
    pub redo: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Source, SourceId, SourceSettings};

    /// Rename the (only) source to `name` through the history, as a discrete
    /// edit, at time `now_ms`.
    fn rename(
        history: &mut History,
        collection: &mut Collection,
        id: SourceId,
        name: &str,
        now_ms: u64,
    ) {
        history
            .edit(collection, format!("Rename to {name}"), None, now_ms, |c| {
                c.rename_source(id, name)
            })
            .expect("rename");
    }

    fn collection_with_source() -> (Collection, SourceId) {
        let mut collection = Collection::new();
        let scene = collection.active_scene;
        let (source, _item) = collection
            .add_item_with_new_source(
                scene,
                Source::new(
                    "Original",
                    SourceSettings::Color {
                        color: crate::Rgba::WHITE,
                        width: 64,
                        height: 64,
                    },
                ),
            )
            .expect("add source");
        (collection, source)
    }

    #[test]
    fn undo_and_redo_walk_the_stack() {
        let (mut collection, source) = collection_with_source();
        let mut history = History::new();

        rename(&mut history, &mut collection, source, "A", 0);
        rename(&mut history, &mut collection, source, "B", 1000);
        assert_eq!(collection.source(source).unwrap().name, "B");

        let label = history.undo(&mut collection).expect("undo B");
        assert_eq!(label, "Rename to B");
        assert_eq!(collection.source(source).unwrap().name, "A");

        history.undo(&mut collection).expect("undo A");
        assert_eq!(collection.source(source).unwrap().name, "Original");
        assert!(!history.can_undo());

        history.redo(&mut collection).expect("redo A");
        assert_eq!(collection.source(source).unwrap().name, "A");
        history.redo(&mut collection).expect("redo B");
        assert_eq!(collection.source(source).unwrap().name, "B");
        assert!(!history.can_redo());
    }

    #[test]
    fn a_new_edit_forks_history_and_clears_redo() {
        let (mut collection, source) = collection_with_source();
        let mut history = History::new();

        rename(&mut history, &mut collection, source, "A", 0);
        rename(&mut history, &mut collection, source, "B", 1000);
        history.undo(&mut collection).expect("undo B"); // now "A", redo has "B"
        assert!(history.can_redo());

        rename(&mut history, &mut collection, source, "C", 2000);
        assert!(!history.can_redo(), "a fresh edit abandons the redo branch");
        assert_eq!(collection.source(source).unwrap().name, "C");
        // Undo returns to "A" (the state before C), not "B".
        history.undo(&mut collection).expect("undo C");
        assert_eq!(collection.source(source).unwrap().name, "A");
    }

    #[test]
    fn a_no_op_edit_records_nothing() {
        let (mut collection, source) = collection_with_source();
        let mut history = History::new();

        // Renaming to the same name leaves the collection unchanged.
        rename(&mut history, &mut collection, source, "Original", 0);
        assert!(
            !history.can_undo(),
            "an edit that changed nothing is not recorded"
        );
    }

    #[test]
    fn a_failed_edit_leaves_history_untouched() {
        let (mut collection, _source) = collection_with_source();
        let mut history = History::new();
        let missing = SourceId::new();

        let result = history.edit(&mut collection, "Rename missing", None, 0, |c| {
            c.rename_source(missing, "nope")
        });
        assert!(result.is_err());
        assert!(
            !history.can_undo(),
            "a rejected mutation records no checkpoint"
        );
    }

    #[test]
    fn a_gesture_coalesces_into_one_step() {
        let (mut collection, _source) = collection_with_source();
        let scene = collection.active_scene;
        let item = collection.scene(scene).unwrap().items[0].id;
        let mut history = History::new();
        let key = 42;

        // Twelve drag frames, each 30 ms apart — one continuous gesture.
        for frame in 0..12u64 {
            let x = 100.0 + frame as f32 * 5.0;
            let transform = crate::Transform {
                x,
                ..Default::default()
            };
            history
                .edit(&mut collection, "Move source", Some(key), frame * 30, |c| {
                    c.set_item_transform(scene, item, transform)
                })
                .expect("move");
        }

        // One undo returns to the pre-gesture transform, not frame 11.
        assert_eq!(history.state().undo.len(), 1, "the whole drag is one step");
        history.undo(&mut collection).expect("undo drag");
        let x = collection
            .scene(scene)
            .unwrap()
            .item(item)
            .unwrap()
            .transform
            .x;
        assert_eq!(x, 0.0, "undo returns to before the gesture began");
    }

    #[test]
    fn gestures_separated_by_a_pause_are_distinct_steps() {
        let (mut collection, source) = collection_with_source();
        let scene = collection.active_scene;
        let item = collection.scene(scene).unwrap().items[0].id;
        let mut history = History::new();
        let key = 7;

        let mv = |history: &mut History, collection: &mut Collection, x: f32, now: u64| {
            let transform = crate::Transform {
                x,
                ..Default::default()
            };
            history
                .edit(collection, "Move source", Some(key), now, |c| {
                    c.set_item_transform(scene, item, transform)
                })
                .expect("move");
        };

        mv(&mut history, &mut collection, 50.0, 0);
        mv(&mut history, &mut collection, 60.0, 100); // same gesture
                                                      // A gap longer than the window opens a new gesture.
        mv(
            &mut history,
            &mut collection,
            200.0,
            100 + COALESCE_WINDOW_MS + 1,
        );
        let _ = source;

        assert_eq!(history.state().undo.len(), 2, "a pause splits the gesture");
    }

    #[test]
    fn capacity_bounds_the_stack_and_drops_oldest() {
        let (mut collection, source) = collection_with_source();
        let mut history = History::with_capacity(3);
        for i in 0..10 {
            rename(
                &mut history,
                &mut collection,
                source,
                &format!("N{i}"),
                i as u64 * 1000,
            );
        }
        assert_eq!(history.state().undo.len(), 3, "capped at capacity");
        // Only the last three edits remain undoable.
        assert_eq!(history.undo_label(), Some("Rename to N9"));
    }

    #[test]
    fn clear_forgets_everything() {
        let (mut collection, source) = collection_with_source();
        let mut history = History::new();
        rename(&mut history, &mut collection, source, "A", 0);
        history.clear();
        assert!(!history.can_undo() && !history.can_redo());
    }
}
