//! The media-audio hub: rings that **non-device producers** feed — the
//! Media source's decode thread pushes its decoded 48 kHz stereo blocks
//! here, and the engine drains them like any capture.
//!
//! One ring per source id, shared by the (restartable) decode thread and the
//! engine's audio strip. The map holds a **weak** reference, so a ring lives
//! exactly as long as *either* side still holds it and is freed only when
//! both are gone. That is what keeps hide→show correct: hiding a media source
//! drops it from the audio mix (the engine releases its `Arc`), but the
//! decoder keeps running and holding the ring, so showing it again rendezvous
//! on the **same** ring the decoder is still feeding — never a fresh empty one.
//! Dead entries are swept lazily on the next lookup, so the map can't grow
//! without bound.

use std::collections::HashMap;
use std::sync::{Arc, OnceLock, Weak};

use parking_lot::Mutex;

use crate::capture::CaptureRing;

fn hub() -> &'static Mutex<HashMap<String, Weak<CaptureRing>>> {
    static HUB: OnceLock<Mutex<HashMap<String, Weak<CaptureRing>>>> = OnceLock::new();
    HUB.get_or_init(|| Mutex::new(HashMap::new()))
}

/// The ring for a media source id — the live one if either side still holds
/// it, otherwise a fresh ring. Both the decoder and the engine call this to
/// rendezvous on the same buffer.
pub fn ring(id: &str) -> Arc<CaptureRing> {
    let mut map = hub().lock();
    // Sweep entries whose ring both sides have dropped (cheap; lookups are
    // per-reconcile, not per-block).
    map.retain(|_, weak| weak.strong_count() > 0);
    if let Some(existing) = map.get(id).and_then(Weak::upgrade) {
        return existing;
    }
    let ring = CaptureRing::new();
    map.insert(id.to_string(), Arc::downgrade(&ring));
    ring
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_ring_per_id_while_referenced_then_freed() {
        let a1 = ring("hub-test-a");
        let a2 = ring("hub-test-a");
        assert!(Arc::ptr_eq(&a1, &a2), "same id → same ring while alive");

        a1.push(&[0.5; 96]);
        assert_eq!(a2.len(), 96, "both handles see the pushed audio");
        a1.clear();
        assert!(a2.is_empty());
        assert_eq!(a1.dropped(), 0, "clear is not a drop");

        // While one side still holds the ring (the decoder), a re-lookup (the
        // engine re-adding the source after a hide) returns the SAME ring —
        // the strand bug: it must not mint a fresh empty one.
        let held = a1.clone();
        drop(a1);
        drop(a2);
        let again = ring("hub-test-a");
        assert!(Arc::ptr_eq(&held, &again), "a still-held ring is reused");

        // Once every side drops it, the next lookup is a fresh ring.
        drop(held);
        drop(again);
        let fresh = ring("hub-test-a");
        assert_eq!(fresh.len(), 0);
        drop(fresh);
    }
}
