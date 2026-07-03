//! The media-audio hub: rings that **non-device producers** feed — the
//! Media source's decode thread pushes its decoded 48 kHz stereo blocks
//! here, and the engine drains them like any capture.
//!
//! One ring per source id, created on first ask and **never replaced**:
//! the engine's stream and the (restartable) producer always share the
//! same ring, so a Properties edit or retry never strands either side on
//! a stale buffer. A restarting producer calls [`CaptureRing::clear`]
//! first; the engine garbage-collects entries whose source left the mix.

use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

use parking_lot::Mutex;

use crate::capture::CaptureRing;

fn hub() -> &'static Mutex<HashMap<String, Arc<CaptureRing>>> {
    static HUB: OnceLock<Mutex<HashMap<String, Arc<CaptureRing>>>> = OnceLock::new();
    HUB.get_or_init(|| Mutex::new(HashMap::new()))
}

/// The ring for a media source id (created on first ask).
pub fn ring(id: &str) -> Arc<CaptureRing> {
    Arc::clone(
        hub()
            .lock()
            .entry(id.to_string())
            .or_insert_with(CaptureRing::new),
    )
}

/// Drop every ring whose id is not in `live` (the engine's reconcile calls
/// this with the current media-source set).
pub fn retain(live: &[String]) {
    hub().lock().retain(|id, _| live.contains(id));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_ring_per_id_and_retain_gcs() {
        let a1 = ring("hub-test-a");
        let a2 = ring("hub-test-a");
        assert!(Arc::ptr_eq(&a1, &a2), "same id → same ring");

        a1.push(&[0.5; 96]);
        assert_eq!(a2.len(), 96, "both handles see the pushed audio");
        a1.clear();
        assert!(a2.is_empty());
        assert_eq!(a1.dropped(), 0, "clear is not a drop");

        let _b = ring("hub-test-b");
        retain(&["hub-test-b".to_string()]);
        let a3 = ring("hub-test-a");
        assert!(!Arc::ptr_eq(&a1, &a3), "retained-out ids start fresh");
        retain(&[]);
    }
}
