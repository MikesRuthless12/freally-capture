//! A String-keyed registry of `Weak` session handles — the shape every
//! controllable source needs (roll slots, playlist transports, split
//! machines, title sessions): insert on session start, entries expire with
//! the session's `Arc`, control paths upgrade and fan out. Extracted when
//! the fourth copy appeared (CAP-N Phase 2).

use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard, OnceLock, Weak};

pub(crate) struct WeakRegistry<T>(OnceLock<Mutex<HashMap<String, Weak<T>>>>);

impl<T> WeakRegistry<T> {
    pub(crate) const fn new() -> Self {
        Self(OnceLock::new())
    }

    fn map(&self) -> MutexGuard<'_, HashMap<String, Weak<T>>> {
        self.0
            .get_or_init(|| Mutex::new(HashMap::new()))
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    pub(crate) fn insert(&self, id: &str, session: &Arc<T>) {
        self.map().insert(id.to_owned(), Arc::downgrade(session));
    }

    pub(crate) fn get(&self, id: &str) -> Option<Arc<T>> {
        self.map().get(id).and_then(Weak::upgrade)
    }

    /// Every live session, dead entries dropped. The lock is released
    /// before the caller acts on the upgrades — never control a session
    /// while holding the registry.
    pub(crate) fn live(&self) -> Vec<Arc<T>> {
        let mut map = self.map();
        map.retain(|_, weak| weak.strong_count() > 0);
        map.values().filter_map(Weak::upgrade).collect()
    }

    /// The ids of every live session (hotkey fan-outs name their targets).
    pub(crate) fn live_ids(&self) -> Vec<String> {
        let mut map = self.map();
        map.retain(|_, weak| weak.strong_count() > 0);
        map.keys().cloned().collect()
    }
}
