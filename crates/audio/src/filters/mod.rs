//! The per-source audio filter chain: owned classic DSP built straight from
//! the [`fcap_scene::AudioFilterKind`] model — **no ML anywhere**, per the
//! charter. Parameters are clamped defensively here so a hand-edited scene
//! file can never build a runaway processor.

pub mod denoise;
pub mod dynamics;
pub mod eq;
pub mod voice;

use std::collections::HashMap;
use std::sync::OnceLock;

use fcap_scene::{AudioFilter, AudioFilterKind, SourceId};

use crate::dsp::db_to_lin;

/// What a filter can see besides its own block: the previous block's
/// post-chain, pre-fader peak envelope of every live source (linear) — the
/// sidechain the Ducker reads. One block (~10 ms) of sidechain latency, well
/// inside typical duck attack times.
pub struct FilterCtx<'a> {
    pub envelopes: &'a HashMap<SourceId, f32>,
}

impl FilterCtx<'_> {
    /// A context with no live sources (tests, chains without sidechains).
    pub fn empty() -> FilterCtx<'static> {
        static EMPTY: OnceLock<HashMap<SourceId, f32>> = OnceLock::new();
        FilterCtx {
            envelopes: EMPTY.get_or_init(HashMap::new),
        }
    }
}

/// One running audio processor. Blocks are interleaved stereo f32 at the
/// engine rate; every filter must write exactly in place.
pub trait FilterProc: Send {
    fn process(&mut self, block: &mut [f32], ctx: &FilterCtx);

    /// Update this processor's parameters **in place** from a new `kind` of
    /// the same filter type, **preserving running DSP state** (biquad memory,
    /// envelopes, the denoiser's learned noise floor). Returns `false` if
    /// `kind` is a different filter type (the caller then rebuilds).
    ///
    /// This is what keeps a live slider-drag click-free: without it, every
    /// intermediate value would rebuild the whole chain and reset every
    /// filter's state.
    fn update(&mut self, kind: &AudioFilterKind) -> bool;
}

/// Plain gain.
struct Gain {
    gain: f32,
}

impl FilterProc for Gain {
    fn process(&mut self, block: &mut [f32], _ctx: &FilterCtx) {
        for sample in block {
            *sample *= self.gain;
        }
    }

    fn update(&mut self, kind: &AudioFilterKind) -> bool {
        if let AudioFilterKind::Gain { db } = kind {
            self.gain = db_to_lin(db.clamp(-30.0, 30.0));
            true
        } else {
            false
        }
    }
}

/// Build one processor from its model parameters (clamped to the documented
/// ranges).
pub fn build_filter(kind: &AudioFilterKind, sample_rate: f32) -> Box<dyn FilterProc> {
    match kind {
        AudioFilterKind::Gain { db } => Box::new(Gain {
            gain: db_to_lin(db.clamp(-30.0, 30.0)),
        }),
        AudioFilterKind::NoiseGate {
            open_threshold_db,
            close_threshold_db,
            attack_ms,
            hold_ms,
            release_ms,
        } => {
            let open = open_threshold_db.clamp(-96.0, 0.0);
            // The close threshold may never sit above the open threshold.
            let close = close_threshold_db.clamp(-96.0, 0.0).min(open);
            Box::new(dynamics::NoiseGate::new(
                sample_rate,
                open,
                close,
                attack_ms.clamp(1.0, 500.0),
                hold_ms.clamp(0.0, 3_000.0),
                release_ms.clamp(1.0, 3_000.0),
            ))
        }
        AudioFilterKind::Compressor {
            ratio,
            threshold_db,
            attack_ms,
            release_ms,
            output_gain_db,
        } => Box::new(dynamics::Compressor::new(
            sample_rate,
            ratio.clamp(1.0, 32.0),
            threshold_db.clamp(-60.0, 0.0),
            attack_ms.clamp(0.1, 500.0),
            release_ms.clamp(1.0, 3_000.0),
            output_gain_db.clamp(-30.0, 30.0),
        )),
        AudioFilterKind::Limiter {
            threshold_db,
            release_ms,
        } => Box::new(dynamics::Limiter::new(
            sample_rate,
            threshold_db.clamp(-30.0, 0.0),
            release_ms.clamp(1.0, 1_000.0),
        )),
        AudioFilterKind::Eq {
            low_db,
            mid_db,
            high_db,
        } => Box::new(eq::ToneEq::new(
            sample_rate,
            low_db.clamp(-20.0, 20.0),
            mid_db.clamp(-20.0, 20.0),
            high_db.clamp(-20.0, 20.0),
        )),
        AudioFilterKind::Denoise { strength } => {
            Box::new(denoise::Denoiser::new(strength.clamp(0.0, 1.0)))
        }
        AudioFilterKind::Ducker {
            trigger,
            threshold_db,
            amount_db,
            attack_ms,
            release_ms,
        } => Box::new(dynamics::Ducker::new(
            sample_rate,
            *trigger,
            threshold_db.clamp(-96.0, 0.0),
            amount_db.clamp(0.0, 60.0),
            attack_ms.clamp(1.0, 1_000.0),
            release_ms.clamp(1.0, 5_000.0),
        )),
        AudioFilterKind::ParametricEq { bands } => {
            Box::new(eq::ParametricEq::new(sample_rate, bands))
        }
        AudioFilterKind::DeEsser {
            freq_hz,
            threshold_db,
            amount_db,
        } => Box::new(voice::DeEsser::new(
            sample_rate,
            freq_hz.clamp(3_000.0, 12_000.0),
            threshold_db.clamp(-96.0, 0.0),
            amount_db.clamp(0.0, 24.0),
        )),
        AudioFilterKind::RumbleGuard { freq_hz } => Box::new(voice::RumbleGuard::new(
            sample_rate,
            freq_hz.clamp(20.0, 300.0),
        )),
    }
}

/// Build a source's running chain: enabled filters, model order.
pub fn build_chain(filters: &[AudioFilter], sample_rate: f32) -> Vec<Box<dyn FilterProc>> {
    filters
        .iter()
        .filter(|filter| filter.enabled)
        .map(|filter| build_filter(&filter.kind, sample_rate))
        .collect()
}

/// Reconcile a running chain against edited filters, **preserving DSP state**
/// when the structure (the enabled filters' identities + order) is unchanged
/// and only parameters differ — so a live slider-drag updates coefficients in
/// place instead of rebuilding (which would reset envelopes and re-seed the
/// denoiser, clicking on every tick). Falls back to a full rebuild on any
/// structural change (add / remove / reorder / enable-toggle).
///
/// `prev_enabled` is the enabled-filter snapshot `chain` was built from.
pub fn reconcile_chain(
    mut chain: Vec<Box<dyn FilterProc>>,
    prev_enabled: &[AudioFilter],
    filters: &[AudioFilter],
    sample_rate: f32,
) -> Vec<Box<dyn FilterProc>> {
    let new_enabled: Vec<&AudioFilter> = filters.iter().filter(|filter| filter.enabled).collect();
    let same_structure = chain.len() == new_enabled.len()
        && prev_enabled.len() == new_enabled.len()
        && prev_enabled
            .iter()
            .zip(&new_enabled)
            .all(|(a, b)| a.id == b.id);
    if same_structure
        && chain
            .iter_mut()
            .zip(&new_enabled)
            .all(|(proc, filter)| proc.update(&filter.kind))
    {
        return chain;
    }
    build_chain(filters, sample_rate)
}

/// The enabled-filter snapshot a chain was built from (the reconcile key).
pub fn enabled_filters(filters: &[AudioFilter]) -> Vec<AudioFilter> {
    filters
        .iter()
        .filter(|filter| filter.enabled)
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gain_multiplies_exactly() {
        let mut chain = build_chain(
            &[AudioFilter::new(AudioFilterKind::Gain { db: -6.0 })],
            48_000.0,
        );
        let mut block = vec![1.0f32; 8];
        let ctx = FilterCtx::empty();
        for filter in &mut chain {
            filter.process(&mut block, &ctx);
        }
        let expected = db_to_lin(-6.0);
        for sample in block {
            assert!((sample - expected).abs() < 1e-6);
        }
    }

    #[test]
    fn disabled_filters_stay_out_of_the_chain() {
        let mut muted = AudioFilter::new(AudioFilterKind::Gain { db: -20.0 });
        muted.enabled = false;
        let chain = build_chain(&[muted], 48_000.0);
        assert!(chain.is_empty());
    }

    #[test]
    fn out_of_range_parameters_are_clamped() {
        // A 10^6 dB gain from a hand-edited file must not build.
        let mut chain = build_chain(
            &[AudioFilter::new(AudioFilterKind::Gain { db: 1e6 })],
            48_000.0,
        );
        let mut block = vec![1.0f32; 2];
        let ctx = FilterCtx::empty();
        for filter in &mut chain {
            filter.process(&mut block, &ctx);
        }
        assert!(block[0] <= db_to_lin(30.0) + 1e-3);
    }

    fn apply(chain: &mut [Box<dyn FilterProc>]) -> f32 {
        let mut block = vec![1.0f32; 2];
        let ctx = FilterCtx::empty();
        for filter in chain.iter_mut() {
            filter.process(&mut block, &ctx);
        }
        block[0]
    }

    #[test]
    fn reconcile_updates_params_in_place_when_structure_matches() {
        // Same filter id + type, edited parameter → the new value applies (an
        // in-place update, not a rebuild that would reset DSP state).
        let mut filter = AudioFilter::new(AudioFilterKind::Gain { db: 0.0 });
        let prev = vec![filter.clone()];
        let chain = build_chain(&prev, 48_000.0);

        filter.kind = AudioFilterKind::Gain { db: -6.0 };
        let mut chain = reconcile_chain(chain, &prev, &[filter.clone()], 48_000.0);
        assert!((apply(&mut chain) - db_to_lin(-6.0)).abs() < 1e-6);
    }

    #[test]
    fn reconcile_rebuilds_on_a_structural_change() {
        // A different filter id (add/remove/reorder) forces a fresh chain.
        let a = AudioFilter::new(AudioFilterKind::Gain { db: 0.0 });
        let prev = vec![a.clone()];
        let chain = build_chain(&prev, 48_000.0);

        let b = AudioFilter::new(AudioFilterKind::Gain { db: -12.0 });
        assert_ne!(a.id, b.id);
        let mut chain = reconcile_chain(chain, &prev, &[b], 48_000.0);
        assert_eq!(chain.len(), 1);
        assert!((apply(&mut chain) - db_to_lin(-12.0)).abs() < 1e-6);
    }

    #[test]
    fn update_rejects_a_mismatched_kind() {
        let mut chain = build_chain(
            &[AudioFilter::new(AudioFilterKind::Gain { db: 0.0 })],
            48_000.0,
        );
        // A Gain processor cannot become an EQ in place.
        assert!(!chain[0].update(&AudioFilterKind::Eq {
            low_db: 3.0,
            mid_db: 0.0,
            high_db: 0.0,
        }));
    }
}
