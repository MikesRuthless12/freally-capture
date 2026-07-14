//! CAP-N15: the visualizer tap registry.
//!
//! Same weak-map shape as [`crate::media_hub`], pointing the other way: a
//! *renderer* subscribes by holding the ring for its target, and the engine
//! publishes each 10 ms block only for targets that are alive right now. A
//! removed visualizer unsubscribes by dropping its `Arc` — nothing to leak,
//! nothing to forget. With no subscribers the engine pays one map lock per
//! tick and copies nothing.

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, OnceLock, Weak};
use std::time::{Duration, Instant};

/// Interleaved-stereo rolling window a renderer reads (~340 ms at 48 kHz) —
/// enough for a 2048-point FFT, a scope sweep, and VU ballistics.
pub const VIS_WINDOW_SAMPLES: usize = 32_768;

/// What a visualizer listens to. Strips are tapped **post-fader** — the
/// signal that actually mixes, so a muted strip visualizes flat, honestly.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VisTarget {
    /// The program (master) mix.
    Master,
    /// A track bus, 0-based.
    Track(usize),
    /// A mixer strip, keyed by the source id's string form (the same
    /// vocabulary the media hub uses).
    Source(String),
}

struct Inner {
    samples: VecDeque<f32>,
    last_push: Option<Instant>,
    /// Samples ever pushed — a monotonic cursor space for [`VisRing::since`]
    /// (CAP-N12: the Freally Link sender reads *contiguous* audio; the
    /// window-shaped `latest` would duplicate or drop samples between polls).
    total: u64,
}

/// The rolling sample window one subscription reads.
pub struct VisRing {
    inner: Mutex<Inner>,
}

impl VisRing {
    fn new() -> Self {
        Self {
            inner: Mutex::new(Inner {
                samples: VecDeque::with_capacity(VIS_WINDOW_SAMPLES),
                last_push: None,
                total: 0,
            }),
        }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Inner> {
        self.inner
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// Engine side: append one interleaved-stereo block, trimming the oldest
    /// samples past the window.
    pub fn push_block(&self, block: &[f32]) {
        let mut inner = self.lock();
        inner.samples.extend(block.iter().copied());
        let len = inner.samples.len();
        if len > VIS_WINDOW_SAMPLES {
            inner.samples.drain(..len - VIS_WINDOW_SAMPLES);
        }
        inner.last_push = Some(Instant::now());
        inner.total += block.len() as u64;
    }

    /// A cursor at "now" — start here to read only audio pushed from now on.
    pub fn cursor(&self) -> u64 {
        self.lock().total
    }

    /// Consumer side with continuity (CAP-N12): every sample pushed after
    /// `cursor` that is still inside the window, plus the advanced cursor.
    /// A consumer slower than the window loses the overwritten samples
    /// (they are simply not returned) but never receives a duplicate.
    pub fn since(&self, cursor: u64) -> (u64, Vec<f32>) {
        let inner = self.lock();
        let total = inner.total;
        // A cursor from a previous ring generation (or clock weirdness)
        // resets to "now" instead of replaying the whole window.
        if cursor >= total {
            return (total, Vec::new());
        }
        let missed = total - cursor;
        let take = (missed as usize).min(inner.samples.len());
        let skip = inner.samples.len() - take;
        (total, inner.samples.iter().skip(skip).copied().collect())
    }

    /// Renderer side: copy the newest `out.len()` samples into `out`,
    /// zero-padding the front while the ring is younger than the request.
    pub fn latest(&self, out: &mut [f32]) {
        let inner = self.lock();
        let take = out.len().min(inner.samples.len());
        let pad = out.len() - take;
        out[..pad].fill(0.0);
        for (dst, src) in out[pad..]
            .iter_mut()
            .zip(inner.samples.iter().skip(inner.samples.len() - take))
        {
            *dst = *src;
        }
    }

    /// Time since the engine last fed this ring — `None` before the first
    /// block. A renderer treats a stale ring as silence, so a visualizer
    /// whose target vanished decays to the floor instead of freezing.
    pub fn age(&self) -> Option<Duration> {
        self.lock().last_push.map(|at| at.elapsed())
    }
}

fn hub() -> &'static Mutex<HashMap<VisTarget, Weak<VisRing>>> {
    static HUB: OnceLock<Mutex<HashMap<VisTarget, Weak<VisRing>>>> = OnceLock::new();
    HUB.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Renderer side: subscribe to a target — creates the ring or revives the
/// existing live one (two visualizers on one target share a ring).
pub fn ring(target: &VisTarget) -> Arc<VisRing> {
    let mut hub = hub()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(live) = hub.get(target).and_then(Weak::upgrade) {
        return live;
    }
    let fresh = Arc::new(VisRing::new());
    hub.insert(target.clone(), Arc::downgrade(&fresh));
    fresh
}

/// Engine side: every subscription alive right now (dead entries pruned).
/// Empty — the overwhelmingly common case — costs one lock and no copies.
pub fn live_targets() -> Vec<(VisTarget, Arc<VisRing>)> {
    let mut hub = hub()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    hub.retain(|_, weak| weak.strong_count() > 0);
    hub.iter()
        .filter_map(|(target, weak)| weak.upgrade().map(|ring| (target.clone(), ring)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn latest_zero_pads_a_young_ring() {
        let ring = VisRing::new();
        ring.push_block(&[1.0, 2.0, 3.0, 4.0]);
        let mut out = [9.0f32; 6];
        ring.latest(&mut out);
        assert_eq!(out, [0.0, 0.0, 1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn the_window_trims_the_oldest_samples() {
        let ring = VisRing::new();
        let block = vec![1.0f32; VIS_WINDOW_SAMPLES];
        ring.push_block(&block);
        ring.push_block(&[2.0, 2.0]);
        let mut out = [0.0f32; 2];
        ring.latest(&mut out);
        assert_eq!(out, [2.0, 2.0]);
        assert_eq!(
            ring.lock().samples.len(),
            VIS_WINDOW_SAMPLES,
            "capacity held"
        );
    }

    #[test]
    fn since_is_contiguous_and_never_duplicates() {
        let ring = VisRing::new();
        let mut cursor = ring.cursor();
        assert_eq!(cursor, 0);

        ring.push_block(&[1.0, 2.0]);
        let (next, got) = ring.since(cursor);
        assert_eq!(got, vec![1.0, 2.0]);
        cursor = next;

        // Nothing new → nothing returned, cursor unchanged.
        let (next, got) = ring.since(cursor);
        assert!(got.is_empty());
        assert_eq!(next, cursor);

        ring.push_block(&[3.0, 4.0]);
        ring.push_block(&[5.0, 6.0]);
        let (next, got) = ring.since(cursor);
        assert_eq!(got, vec![3.0, 4.0, 5.0, 6.0], "blocks concatenate");
        cursor = next;

        // A consumer slower than the window loses the overwritten samples
        // but resumes contiguously at the window's oldest survivor.
        ring.push_block(&vec![7.0f32; VIS_WINDOW_SAMPLES + 2]);
        let (_, got) = ring.since(cursor);
        assert_eq!(got.len(), VIS_WINDOW_SAMPLES);
        assert!(got.iter().all(|s| *s == 7.0));

        // A cursor past the total (stale generation) resets to "now".
        let (reset, got) = ring.since(u64::MAX);
        assert!(got.is_empty());
        assert_eq!(reset, ring.cursor());
    }

    #[test]
    fn age_reports_feeding_and_silence() {
        let ring = VisRing::new();
        assert!(ring.age().is_none(), "never fed");
        ring.push_block(&[0.0, 0.0]);
        assert!(ring.age().expect("fed") < Duration::from_secs(1));
    }

    #[test]
    fn a_dropped_subscription_unsubscribes() {
        let target = VisTarget::Source("vis-test-lifecycle".into());
        let live = ring(&target);
        assert!(live_targets().iter().any(|(t, _)| *t == target));
        drop(live);
        assert!(!live_targets().iter().any(|(t, _)| *t == target));
    }

    #[test]
    fn two_subscribers_share_one_ring() {
        let target = VisTarget::Source("vis-test-shared".into());
        let a = ring(&target);
        let b = ring(&target);
        a.push_block(&[5.0, 5.0]);
        let mut out = [0.0f32; 2];
        b.latest(&mut out);
        assert_eq!(out, [5.0, 5.0]);
        drop((a, b));
    }
}
