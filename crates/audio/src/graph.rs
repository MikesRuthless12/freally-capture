//! The mixing core — pure and device-free, so every routing rule is unit
//! tested without hardware.
//!
//! Per block (10 ms), each source runs its strip: **sync-offset delay →
//! filter chain (with the previous block's envelopes as the ducker
//! sidechain) → push-to-talk / push-to-mute / mute / fader gain (click-free
//! smoothed)** — then routes into the up-to-6 **track buses**, the **master**
//! (program) mix, and the **monitor** mix per its monitor mode. Meters read
//! what actually mixes (post-gain: a muted strip meters flat); the LUFS meter
//! reads the master.

use std::collections::HashMap;

use fcap_scene::{AudioSettings, MonitorMode, SourceId, MIN_VOLUME_DB, TRACK_COUNT};

use fcap_scene::AudioFilter;

use crate::delay::DelayLine;
use crate::dsp::{db_to_lin, one_pole_coef};
use crate::filters::{build_chain, enabled_filters, reconcile_chain, FilterCtx, FilterProc};
use crate::lufs::LufsMeter;
use crate::meter::{LevelAccumulator, Levels};
use crate::{BLOCK_FRAMES, SAMPLE_RATE};

const BLOCK_SAMPLES: usize = BLOCK_FRAMES * 2;
/// Per-block envelope fall (~35 ms) for the ducker sidechain.
const ENVELOPE_FALL: f32 = 0.6;

/// The fader curve: the bottom of the fader is silence, everything else dB.
fn fader_gain(volume_db: f32) -> f32 {
    if volume_db <= MIN_VOLUME_DB + 0.01 {
        0.0
    } else {
        db_to_lin(volume_db)
    }
}

/// The stereo balance law (CAP-M19): center = unity on both channels (an
/// untouched pan changes nothing); panning attenuates the OPPOSITE side
/// only, down to silence at the extremes.
fn balance_gains(pan: f32) -> (f32, f32) {
    let pan = if pan.is_finite() {
        pan.clamp(-1.0, 1.0)
    } else {
        0.0
    };
    if pan > 0.0 {
        (1.0 - pan, 1.0)
    } else {
        (1.0, 1.0 + pan)
    }
}

/// One source's per-block control state (model + hotkey runtime).
#[derive(Debug, Clone)]
pub struct StripControl {
    pub settings: AudioSettings,
    /// Push-to-talk key currently held (only meaningful when configured).
    pub ptt_held: bool,
    /// Push-to-mute key currently held (only meaningful when configured).
    pub ptm_held: bool,
}

impl StripControl {
    pub fn new(settings: AudioSettings) -> Self {
        Self {
            settings,
            ptt_held: false,
            ptm_held: false,
        }
    }

    /// Whether the strip is audible this block (mute + PTT/PTM resolved).
    pub fn effectively_muted(&self) -> bool {
        self.settings.muted
            || (self.settings.push_to_talk.is_some() && !self.ptt_held)
            || (self.settings.push_to_mute.is_some() && self.ptm_held)
    }
}

/// Everything one running source owns.
struct Strip {
    delay: DelayLine,
    chain: Vec<Box<dyn FilterProc>>,
    /// The settings the delay/chain were built from (rebuild detector).
    built_from: AudioSettings,
    /// The enabled-filter snapshot `chain` was built from — the key that lets
    /// a pure parameter edit update filters in place instead of rebuilding.
    chain_filters: Vec<AudioFilter>,
    meter: LevelAccumulator,
    envelope: f32,
    gain: f32,
    scratch: Vec<f32>,
}

impl Strip {
    fn new(settings: &AudioSettings) -> Self {
        Self {
            delay: DelayLine::new(sync_frames(settings.sync_offset_ms)),
            chain: build_chain(&settings.filters, SAMPLE_RATE as f32),
            built_from: settings.clone(),
            chain_filters: enabled_filters(&settings.filters),
            meter: LevelAccumulator::default(),
            envelope: 0.0,
            gain: 0.0,
            scratch: vec![0.0; BLOCK_SAMPLES],
        }
    }

    /// Reconcile the strip against edited settings. A pure filter-parameter
    /// change (same filters, same order) updates coefficients **in place** so
    /// a live slider-drag stays click-free (envelopes and the denoiser's
    /// learned floor carry); a structural change rebuilds the chain.
    fn reconcile(&mut self, settings: &AudioSettings) {
        if self.built_from.filters != settings.filters {
            let chain = std::mem::take(&mut self.chain);
            self.chain = reconcile_chain(
                chain,
                &self.chain_filters,
                &settings.filters,
                SAMPLE_RATE as f32,
            );
            self.chain_filters = enabled_filters(&settings.filters);
        }
        if self.built_from.sync_offset_ms != settings.sync_offset_ms {
            self.delay = DelayLine::new(sync_frames(settings.sync_offset_ms));
        }
        self.built_from = settings.clone();
    }
}

fn sync_frames(ms: u32) -> usize {
    (ms as usize * SAMPLE_RATE as usize) / 1_000
}

/// A program-audio duck driven by a transition's own audio envelope
/// (CAP-N29). While a stinger plays, its decoded audio's peak envelope opens
/// the duck and the program mix — the master **and every track bus** (what
/// records and streams) — dips by up to `depth_db`. Off unless armed; the
/// program mix is bit-identical when no duck is set.
#[derive(Debug, Clone, Copy)]
pub struct TransitionDuck {
    /// Full attenuation applied while the trigger sits above threshold, dB.
    depth_db: f32,
    /// Per-sample one-pole coefficients for the dive / recover ramps.
    attack_coef: f32,
    release_coef: f32,
    /// Linear trigger level above which the duck engages.
    threshold_lin: f32,
}

impl TransitionDuck {
    /// `depth_db` how far to duck; `threshold_db` the trigger gate; the ramps
    /// in ms. Same ballistics shape as the per-strip [`crate::filters`] ducker.
    pub fn new(depth_db: f32, attack_ms: f32, release_ms: f32, threshold_db: f32) -> Self {
        Self {
            depth_db: depth_db.max(0.0),
            attack_coef: one_pole_coef(attack_ms.max(1.0), SAMPLE_RATE as f32),
            release_coef: one_pole_coef(release_ms.max(1.0), SAMPLE_RATE as f32),
            threshold_lin: db_to_lin(threshold_db),
        }
    }
}

/// Multiply a stereo bus by a per-frame gain (one gain value per L/R pair).
fn apply_frame_gain(bus: &mut [f32], gain: &[f32]) {
    for (frame, g) in bus.chunks_exact_mut(2).zip(gain) {
        frame[0] *= *g;
        frame[1] *= *g;
    }
}

/// The block outputs, borrowed from the core after [`MixerCore::process`].
pub struct MixerCore {
    strips: HashMap<SourceId, Strip>,
    envelopes: HashMap<SourceId, f32>,
    tracks: Vec<Vec<f32>>,
    master: Vec<f32>,
    monitor: Vec<f32>,
    lufs: LufsMeter,
    master_meter: LevelAccumulator,
    gain_coef: f32,
    /// Transition duck (CAP-N29): `None` when idle. `duck_releasing` ramps the
    /// reduction back to unity after the transition ends, then clears it.
    duck: Option<TransitionDuck>,
    duck_releasing: bool,
    /// The trigger (stinger) peak envelope, fed each block while armed.
    duck_env: f32,
    /// Current smoothed gain reduction, dB (≥ 0).
    duck_reduction_db: f32,
    /// Reused per-frame duck gain, so the same curve hits master + every track.
    duck_gain: Vec<f32>,
}

impl Default for MixerCore {
    fn default() -> Self {
        Self::new()
    }
}

impl MixerCore {
    pub fn new() -> Self {
        Self {
            strips: HashMap::new(),
            envelopes: HashMap::new(),
            tracks: vec![vec![0.0; BLOCK_SAMPLES]; TRACK_COUNT],
            master: vec![0.0; BLOCK_SAMPLES],
            monitor: vec![0.0; BLOCK_SAMPLES],
            lufs: LufsMeter::new(SAMPLE_RATE),
            master_meter: LevelAccumulator::default(),
            // ~8 ms fade on mute/PTT/fader moves — click-free, still snappy.
            gain_coef: one_pole_coef(8.0, SAMPLE_RATE as f32),
            duck: None,
            duck_releasing: false,
            duck_env: 0.0,
            duck_reduction_db: 0.0,
            duck_gain: vec![1.0; BLOCK_FRAMES],
        }
    }

    /// Mix one block. `inputs` carries each source's interleaved stereo block
    /// (missing/short input mixes as silence); `controls` is the authoritative
    /// source list — strips for sources not in it are dropped.
    pub fn process(
        &mut self,
        inputs: &HashMap<SourceId, Vec<f32>>,
        controls: &HashMap<SourceId, StripControl>,
    ) {
        // Reconcile the strip set with the control set.
        self.strips.retain(|id, _| controls.contains_key(id));
        self.envelopes.retain(|id, _| controls.contains_key(id));
        for (id, control) in controls {
            match self.strips.get_mut(id) {
                Some(strip) => strip.reconcile(&control.settings),
                None => {
                    self.strips.insert(*id, Strip::new(&control.settings));
                }
            }
        }

        for track in &mut self.tracks {
            track.fill(0.0);
        }
        self.master.fill(0.0);
        self.monitor.fill(0.0);

        let mut next_envelopes: HashMap<SourceId, f32> = HashMap::with_capacity(controls.len());

        // CAP-M19 PFL: while ANY strip is soloed, the monitor bus carries
        // ONLY soloed strips — the program/track mix never changes.
        let any_solo = controls.values().any(|control| control.settings.solo);

        for (id, control) in controls {
            let strip = self.strips.get_mut(id).expect("reconciled above");

            // 1. The source block (silence when the capture has nothing).
            let input = inputs.get(id);
            match input {
                Some(block) if block.len() == BLOCK_SAMPLES => {
                    strip.scratch.copy_from_slice(block);
                }
                Some(block) => {
                    strip.scratch.fill(0.0);
                    let len = block.len().min(BLOCK_SAMPLES);
                    strip.scratch[..len].copy_from_slice(&block[..len]);
                }
                None => strip.scratch.fill(0.0),
            }

            // 2. Sync offset, then the filter chain (prev-block sidechain).
            strip.delay.process(&mut strip.scratch);
            let ctx = FilterCtx {
                envelopes: &self.envelopes,
            };
            for filter in &mut strip.chain {
                filter.process(&mut strip.scratch, &ctx);
            }

            // 3. Mono downmix + stereo balance (CAP-M19; center = unity, so
            //    an untouched pan leaves the mix bit-identical).
            let (left_bal, right_bal) = balance_gains(control.settings.pan);
            let mono = control.settings.mono;
            for frame in strip.scratch.chunks_exact_mut(2) {
                if mono {
                    let mixed = (frame[0] + frame[1]) * 0.5;
                    frame[0] = mixed;
                    frame[1] = mixed;
                }
                frame[0] *= left_bal;
                frame[1] *= right_bal;
            }

            // 4. PFL (CAP-M19) taps HERE — pre-fader and pre-mute, which is
            //    the whole point of pre-fade listen: cueing a muted or
            //    pulled-down strip must still be audible on the monitor.
            //    (Tapping after step 5 would monitor silence.)
            if any_solo && control.settings.solo {
                add_into(&mut self.monitor, &strip.scratch);
            }

            // 5. Mute/PTT/fader, smoothed per sample against clicks.
            let target = if control.effectively_muted() {
                0.0
            } else {
                fader_gain(control.settings.volume_db)
            };
            let mut peak = 0.0f32;
            for frame in strip.scratch.chunks_exact_mut(2) {
                strip.gain += (target - strip.gain) * (1.0 - self.gain_coef);
                frame[0] *= strip.gain;
                frame[1] *= strip.gain;
                peak = peak.max(frame[0].abs()).max(frame[1].abs());
            }

            // 6. What actually mixes is what meters + drives the sidechain.
            strip.meter.push_block(&strip.scratch);
            strip.envelope = peak.max(strip.envelope * ENVELOPE_FALL);
            next_envelopes.insert(*id, strip.envelope);

            // 7. Routing. While ANY strip solos, the monitor carries only the
            //    soloed strips (tapped pre-fader above); everything else
            //    leaves the monitor. The program/track mix never changes.
            let monitor = control.settings.monitor;
            if !any_solo && monitor != MonitorMode::Off {
                add_into(&mut self.monitor, &strip.scratch);
            }
            if monitor != MonitorMode::MonitorOnly {
                add_into(&mut self.master, &strip.scratch);
                for (index, track) in self.tracks.iter_mut().enumerate() {
                    if control.settings.track_enabled(index) {
                        add_into(track, &strip.scratch);
                    }
                }
            }
        }

        // Transition duck (CAP-N29): after the buses are summed, dip the
        // program (master + every track) by the stinger-driven reduction, so
        // metering, recording, and streaming all see the same duck.
        self.apply_transition_duck();

        self.envelopes = next_envelopes;
        self.lufs.push(&self.master);
        self.master_meter.push_block(&self.master);
    }

    /// Arm/refresh (`Some`) or release (`None`) the transition duck (CAP-N29).
    /// Release keeps the last config so the reduction ramps out on its release
    /// curve, then clears itself once back at unity.
    pub fn set_transition_duck(&mut self, duck: Option<TransitionDuck>) {
        match duck {
            Some(cfg) => {
                self.duck = Some(cfg);
                self.duck_releasing = false;
            }
            None => self.duck_releasing = true,
        }
    }

    /// Feed the trigger's samples (the stinger's decoded audio) each block so
    /// its peak envelope can open the duck. No-op effect until the duck arms.
    pub fn feed_duck_trigger(&mut self, samples: &[f32]) {
        let peak = samples.iter().fold(0.0f32, |acc, s| acc.max(s.abs()));
        self.duck_env = peak.max(self.duck_env * ENVELOPE_FALL);
    }

    /// Advance the duck one block and multiply it into the program buses.
    fn apply_transition_duck(&mut self) {
        let Some(duck) = self.duck else {
            return;
        };
        // Trigger over threshold opens the duck — unless we're releasing.
        let target_db = if !self.duck_releasing && self.duck_env >= duck.threshold_lin {
            duck.depth_db
        } else {
            0.0
        };
        // Build one per-frame gain curve, advancing the ballistics once, so the
        // identical curve applies to master and every track (no double-ramp).
        for g in self.duck_gain.iter_mut() {
            let coef = if target_db > self.duck_reduction_db {
                duck.attack_coef
            } else {
                duck.release_coef
            };
            self.duck_reduction_db += (target_db - self.duck_reduction_db) * (1.0 - coef);
            *g = db_to_lin(-self.duck_reduction_db);
        }
        apply_frame_gain(&mut self.master, &self.duck_gain);
        for track in &mut self.tracks {
            apply_frame_gain(track, &self.duck_gain);
        }
        // Fully released → stop touching the mix (the common idle path costs
        // nothing once cleared).
        if self.duck_releasing && self.duck_reduction_db < 5e-4 {
            self.duck = None;
            self.duck_reduction_db = 0.0;
            self.duck_env = 0.0;
            self.duck_releasing = false;
        }
    }

    /// One track bus's last block (0-based; recording consumes these in P4).
    pub fn track(&self, index: usize) -> &[f32] {
        &self.tracks[index]
    }

    /// The program (master) mix's last block.
    pub fn master(&self) -> &[f32] {
        &self.master
    }

    /// The monitor mix's last block.
    pub fn monitor(&self) -> &[f32] {
        &self.monitor
    }

    /// One strip's post-fader block from the last [`MixerCore::process`] —
    /// the CAP-N15 visualizer tap. Post-fader is "what actually mixes": a
    /// muted or pulled-down strip reads flat, exactly like it sounds.
    /// Keyed by the source id's string form (the vis registry's vocabulary).
    pub fn strip_block(&self, key: &str) -> Option<&[f32]> {
        // This runs inside the 10 ms mix loop: parse the key on the stack
        // and hit the map — a per-strip `to_string()` scan allocated per
        // block per subscription, which is real-time-audio poison.
        let id = SourceId(key.parse().ok()?);
        self.strips.get(&id).map(|strip| strip.scratch.as_slice())
    }

    /// A source's accumulated levels since last asked (resets).
    pub fn take_source_levels(&mut self, id: SourceId) -> Option<Levels> {
        self.strips.get_mut(&id).map(|strip| strip.meter.take())
    }

    /// The master mix's accumulated levels since last asked (resets).
    pub fn take_master_levels(&mut self) -> Levels {
        self.master_meter.take()
    }

    /// Momentary + short-term LUFS of the master mix.
    pub fn lufs(&self) -> (Option<f32>, Option<f32>) {
        (self.lufs.momentary(), self.lufs.short_term())
    }
}

fn add_into(bus: &mut [f32], block: &[f32]) {
    for (out, sample) in bus.iter_mut().zip(block) {
        *out += sample;
    }
}

#[cfg(test)]
mod tests {
    use fcap_scene::AudioFilterKind;

    use super::*;

    fn tone_block(amp: f32, phase0: usize) -> Vec<f32> {
        (0..BLOCK_FRAMES)
            .flat_map(|i| {
                let t = (phase0 + i) as f32 / SAMPLE_RATE as f32;
                let s = amp * (2.0 * std::f32::consts::PI * 440.0 * t).sin();
                [s, s]
            })
            .collect()
    }

    fn peak(block: &[f32]) -> f32 {
        block.iter().fold(0.0f32, |acc, s| acc.max(s.abs()))
    }

    /// Run `blocks` blocks of a steady tone through one source and return
    /// the core (gain smoothing settled).
    fn run_one(
        core: &mut MixerCore,
        id: SourceId,
        control: &StripControl,
        amp: f32,
        blocks: usize,
    ) {
        let mut controls = HashMap::new();
        controls.insert(id, control.clone());
        for block in 0..blocks {
            let mut inputs = HashMap::new();
            inputs.insert(id, tone_block(amp, block * BLOCK_FRAMES));
            core.process(&inputs, &controls);
        }
    }

    #[test]
    fn default_strip_reaches_master_and_track_one() {
        let mut core = MixerCore::new();
        let id = SourceId::new();
        let control = StripControl::new(AudioSettings::default());
        run_one(&mut core, id, &control, 0.5, 20);

        assert!((peak(core.master()) - 0.5).abs() < 0.02);
        assert!((peak(core.track(0)) - 0.5).abs() < 0.02);
        assert_eq!(peak(core.track(1)), 0.0, "track 2 not assigned");
        assert_eq!(peak(core.monitor()), 0.0, "monitor off by default");
        let levels = core.take_source_levels(id).expect("strip exists");
        assert!(levels.peak[0] > 0.4);
    }

    #[test]
    fn track_assignment_routes_to_every_selected_bus() {
        let mut core = MixerCore::new();
        let id = SourceId::new();
        let control = StripControl::new(AudioSettings {
            tracks: 0b10_0110, // tracks 2, 3, 6
            ..AudioSettings::default()
        });
        run_one(&mut core, id, &control, 0.5, 20);
        assert_eq!(peak(core.track(0)), 0.0);
        assert!(peak(core.track(1)) > 0.4);
        assert!(peak(core.track(2)) > 0.4);
        assert!(peak(core.track(5)) > 0.4);
    }

    #[test]
    fn fader_applies_in_decibels() {
        let mut core = MixerCore::new();
        let id = SourceId::new();
        let control = StripControl::new(AudioSettings {
            volume_db: -6.0,
            ..AudioSettings::default()
        });
        run_one(&mut core, id, &control, 0.5, 30);
        let expected = 0.5 * db_to_lin(-6.0);
        assert!(
            (peak(core.master()) - expected).abs() < 0.02,
            "expected ~{expected}, got {}",
            peak(core.master())
        );
    }

    fn channel_peak(block: &[f32], channel: usize) -> f32 {
        block
            .iter()
            .skip(channel)
            .step_by(2)
            .fold(0.0f32, |acc, s| acc.max(s.abs()))
    }

    #[test]
    fn balance_attenuates_only_the_opposite_side() {
        // Hard right: the left channel empties, the right keeps unity —
        // and a centered pan is bit-identical to no pan at all.
        let mut core = MixerCore::new();
        let id = SourceId::new();
        let control = StripControl::new(AudioSettings {
            pan: 1.0,
            ..AudioSettings::default()
        });
        run_one(&mut core, id, &control, 0.5, 20);
        assert!(channel_peak(core.master(), 0) < 0.01, "left emptied");
        assert!(
            (channel_peak(core.master(), 1) - 0.5).abs() < 0.02,
            "right at unity"
        );
        assert_eq!(balance_gains(0.0), (1.0, 1.0), "center = unity");
        assert_eq!(balance_gains(-1.0), (1.0, 0.0), "hard left");
    }

    #[test]
    fn mono_downmix_equalizes_the_channels() {
        // A hard-left-only signal (L=0.8, R=0) lands equally on both sides.
        let mut core = MixerCore::new();
        let id = SourceId::new();
        let control = StripControl::new(AudioSettings {
            mono: true,
            ..AudioSettings::default()
        });
        let mut controls = HashMap::new();
        controls.insert(id, control);
        let block: Vec<f32> = (0..BLOCK_FRAMES).flat_map(|_| [0.8f32, 0.0]).collect();
        for _ in 0..20 {
            let mut inputs = HashMap::new();
            inputs.insert(id, block.clone());
            core.process(&inputs, &controls);
        }
        assert!((channel_peak(core.master(), 0) - 0.4).abs() < 0.02);
        assert!((channel_peak(core.master(), 1) - 0.4).abs() < 0.02);
    }

    #[test]
    fn solo_is_pfl_monitor_only() {
        // Strip A solos with monitor OFF; strip B monitors normally. While
        // the solo holds: the monitor carries ONLY A (PFL reaches past A's
        // Off mode, B leaves), and the program mix carries BOTH unchanged.
        let mut core = MixerCore::new();
        let id_a = SourceId::new();
        let id_b = SourceId::new();
        let mut controls = HashMap::new();
        controls.insert(
            id_a,
            StripControl::new(AudioSettings {
                solo: true,
                monitor: MonitorMode::Off,
                ..AudioSettings::default()
            }),
        );
        controls.insert(
            id_b,
            StripControl::new(AudioSettings {
                monitor: MonitorMode::MonitorAndOutput,
                ..AudioSettings::default()
            }),
        );
        for block in 0..20 {
            let mut inputs = HashMap::new();
            inputs.insert(id_a, tone_block(0.3, block * BLOCK_FRAMES));
            inputs.insert(id_b, tone_block(0.5, block * BLOCK_FRAMES));
            core.process(&inputs, &controls);
        }
        assert!(
            (peak(core.monitor()) - 0.3).abs() < 0.02,
            "monitor = the soloed strip alone, got {}",
            peak(core.monitor())
        );
        assert!(
            (peak(core.master()) - 0.8).abs() < 0.03,
            "program unchanged (both strips), got {}",
            peak(core.master())
        );
    }

    #[test]
    fn solo_is_pre_fade_so_a_muted_strip_still_cues() {
        // The classic reason to cue a strip is that it is NOT in the program
        // mix. PFL must tap before the mute/fader — a post-fader tap would
        // monitor silence and the operator would think the mic was dead.
        let mut core = MixerCore::new();
        let id = SourceId::new();
        let control = StripControl::new(AudioSettings {
            solo: true,
            muted: true,
            volume_db: MIN_VOLUME_DB, // fader all the way down, too
            monitor: MonitorMode::Off,
            ..AudioSettings::default()
        });
        run_one(&mut core, id, &control, 0.5, 20);
        assert!(
            (peak(core.monitor()) - 0.5).abs() < 0.02,
            "a soloed strip cues at full pre-fader level, got {}",
            peak(core.monitor())
        );
        assert_eq!(
            peak(core.master()),
            0.0,
            "and it stays out of the program mix (muted)"
        );
    }

    #[test]
    fn the_fader_floor_is_silence() {
        let mut core = MixerCore::new();
        let id = SourceId::new();
        let control = StripControl::new(AudioSettings {
            volume_db: MIN_VOLUME_DB,
            ..AudioSettings::default()
        });
        run_one(&mut core, id, &control, 0.5, 30);
        assert!(peak(core.master()) < 1e-4);
    }

    #[test]
    fn mute_silences_after_the_fade() {
        let mut core = MixerCore::new();
        let id = SourceId::new();
        let control = StripControl::new(AudioSettings {
            muted: true,
            ..AudioSettings::default()
        });
        run_one(&mut core, id, &control, 0.5, 30);
        assert!(peak(core.master()) < 1e-4, "muted strips mix silence");
        let levels = core.take_source_levels(id).expect("strip");
        // The meter reads what mixes — flat while muted (the UI dims it).
        assert!(levels.peak[0] < 1e-3);
    }

    #[test]
    fn push_to_talk_gates_until_held() {
        let mut core = MixerCore::new();
        let id = SourceId::new();
        let mut control = StripControl::new(AudioSettings {
            push_to_talk: Some("F13".into()),
            ..AudioSettings::default()
        });
        run_one(&mut core, id, &control, 0.5, 30);
        assert!(peak(core.master()) < 1e-4, "silent until the key is held");

        control.ptt_held = true;
        run_one(&mut core, id, &control, 0.5, 30);
        assert!(
            (peak(core.master()) - 0.5).abs() < 0.02,
            "held PTT passes audio"
        );
    }

    #[test]
    fn push_to_mute_silences_while_held() {
        let mut core = MixerCore::new();
        let id = SourceId::new();
        let mut control = StripControl::new(AudioSettings {
            push_to_mute: Some("F14".into()),
            ..AudioSettings::default()
        });
        control.ptm_held = true;
        run_one(&mut core, id, &control, 0.5, 30);
        assert!(peak(core.master()) < 1e-4);
    }

    #[test]
    fn monitor_modes_route_correctly() {
        // MonitorOnly: hears it, master/tracks don't.
        let mut core = MixerCore::new();
        let id = SourceId::new();
        let control = StripControl::new(AudioSettings {
            monitor: MonitorMode::MonitorOnly,
            ..AudioSettings::default()
        });
        run_one(&mut core, id, &control, 0.5, 20);
        assert!(peak(core.monitor()) > 0.4);
        assert_eq!(peak(core.master()), 0.0);
        assert_eq!(peak(core.track(0)), 0.0);

        // MonitorAndOutput: both.
        let control = StripControl::new(AudioSettings {
            monitor: MonitorMode::MonitorAndOutput,
            ..AudioSettings::default()
        });
        run_one(&mut core, id, &control, 0.5, 20);
        assert!(peak(core.monitor()) > 0.4);
        assert!(peak(core.master()) > 0.4);
    }

    #[test]
    fn sync_offset_delays_the_source() {
        let mut core = MixerCore::new();
        let id = SourceId::new();
        let control = StripControl::new(AudioSettings {
            sync_offset_ms: 20, // exactly two blocks
            ..AudioSettings::default()
        });
        let mut controls = HashMap::new();
        controls.insert(id, control.clone());

        // Settle the gain smoothing on silence first.
        for _ in 0..20 {
            core.process(&HashMap::new(), &controls);
        }
        // An impulse block, then silence blocks.
        let mut impulse = vec![0.0f32; BLOCK_SAMPLES];
        impulse[0] = 1.0;
        let mut inputs = HashMap::new();
        inputs.insert(id, impulse);
        core.process(&inputs, &controls);
        assert!(
            peak(core.master()) < 1e-4,
            "block 0: still inside the delay"
        );
        core.process(&HashMap::new(), &controls);
        assert!(
            peak(core.master()) < 1e-4,
            "block 1: still inside the delay"
        );
        core.process(&HashMap::new(), &controls);
        assert!(
            peak(core.master()) > 0.9,
            "block 2: the impulse arrives 20 ms late"
        );
    }

    #[test]
    fn ducker_dips_the_music_under_the_mic() {
        let mut core = MixerCore::new();
        let mic = SourceId::new();
        let music = SourceId::new();

        let mic_control = StripControl::new(AudioSettings::default());
        let mut music_settings = AudioSettings::default();
        music_settings
            .filters
            .push(fcap_scene::AudioFilter::new(AudioFilterKind::Ducker {
                trigger: Some(mic),
                threshold_db: -30.0,
                amount_db: 12.0,
                attack_ms: 10.0,
                release_ms: 50.0,
            }));
        let music_control = StripControl::new(music_settings);

        let mut controls = HashMap::new();
        controls.insert(mic, mic_control);
        controls.insert(music, music_control.clone());

        // Music alone at 0.4: passes ~full.
        for block in 0..30 {
            let mut inputs = HashMap::new();
            inputs.insert(music, tone_block(0.4, block * BLOCK_FRAMES));
            core.process(&inputs, &controls);
        }
        // Only the music routes to track 1 in this setup check.
        let alone = peak(core.master());
        assert!(alone > 0.35, "music alone ~0.4, got {alone}");

        // Mic speaks at -10 dB: the music (isolate via track 2) dips ~12 dB.
        let mut controls2 = controls.clone();
        controls2.get_mut(&music).unwrap().settings.tracks = 0b10; // track 2
        for block in 0..60 {
            let mut inputs = HashMap::new();
            inputs.insert(mic, tone_block(db_to_lin(-10.0), block * BLOCK_FRAMES));
            inputs.insert(music, tone_block(0.4, block * BLOCK_FRAMES));
            core.process(&inputs, &controls2);
        }
        let ducked = peak(core.track(1));
        let expected = 0.4 * db_to_lin(-12.0);
        assert!(
            (ducked - expected).abs() < 0.03,
            "expected the music near {expected}, got {ducked}"
        );

        // Mic stops: the music recovers.
        for block in 0..60 {
            let mut inputs = HashMap::new();
            inputs.insert(music, tone_block(0.4, block * BLOCK_FRAMES));
            core.process(&inputs, &controls2);
        }
        assert!(
            peak(core.track(1)) > 0.35,
            "music recovers once the mic is quiet"
        );
    }

    /// CAP-N29: while a stinger plays, its (loud) audio ducks the program —
    /// master AND the track buses (what records/streams) — and it recovers
    /// once the stinger ends.
    #[test]
    fn transition_duck_dips_the_program_under_the_stinger() {
        let mut core = MixerCore::new();
        let music = SourceId::new();
        let mut controls = HashMap::new();
        controls.insert(music, StripControl::new(AudioSettings::default()));

        // Baseline: music alone at 0.4 reaches the program near full.
        for block in 0..30 {
            let mut inputs = HashMap::new();
            inputs.insert(music, tone_block(0.4, block * BLOCK_FRAMES));
            core.process(&inputs, &controls);
        }
        assert!(peak(core.master()) > 0.35, "music alone ~0.4");

        // Arm the duck, feed a loud stinger envelope: program dips ~12 dB on
        // both the master and the track bus.
        core.set_transition_duck(Some(TransitionDuck::new(12.0, 10.0, 50.0, -30.0)));
        for block in 0..60 {
            core.feed_duck_trigger(&[0.8f32; BLOCK_SAMPLES]);
            let mut inputs = HashMap::new();
            inputs.insert(music, tone_block(0.4, block * BLOCK_FRAMES));
            core.process(&inputs, &controls);
        }
        let expected = 0.4 * db_to_lin(-12.0);
        let master = peak(core.master());
        let track = peak(core.track(0));
        assert!(
            (master - expected).abs() < 0.03,
            "master ducked near {expected}, got {master}"
        );
        assert!(
            (track - expected).abs() < 0.03,
            "track ducked near {expected}, got {track}"
        );

        // Release (the stinger ended): the program recovers.
        core.set_transition_duck(None);
        for block in 0..60 {
            let mut inputs = HashMap::new();
            inputs.insert(music, tone_block(0.4, block * BLOCK_FRAMES));
            core.process(&inputs, &controls);
        }
        assert!(
            peak(core.master()) > 0.35,
            "the program recovers after the stinger"
        );
    }

    /// A silent stinger never ducks — the duck is driven by the trigger's own
    /// envelope, so no envelope means the program mix is untouched.
    #[test]
    fn transition_duck_is_inert_without_a_trigger() {
        let mut core = MixerCore::new();
        let music = SourceId::new();
        let mut controls = HashMap::new();
        controls.insert(music, StripControl::new(AudioSettings::default()));
        core.set_transition_duck(Some(TransitionDuck::new(12.0, 10.0, 50.0, -30.0)));
        for block in 0..40 {
            core.feed_duck_trigger(&[0.0f32; BLOCK_SAMPLES]);
            let mut inputs = HashMap::new();
            inputs.insert(music, tone_block(0.4, block * BLOCK_FRAMES));
            core.process(&inputs, &controls);
        }
        assert!(
            peak(core.master()) > 0.35,
            "a silent stinger does not duck the program"
        );
    }

    #[test]
    fn sources_leaving_the_scene_drop_their_strips() {
        let mut core = MixerCore::new();
        let id = SourceId::new();
        let control = StripControl::new(AudioSettings::default());
        run_one(&mut core, id, &control, 0.5, 5);
        assert!(core.take_source_levels(id).is_some());

        core.process(&HashMap::new(), &HashMap::new());
        assert!(core.take_source_levels(id).is_none(), "strip dropped");
    }

    #[test]
    fn lufs_reads_the_program_mix() {
        let mut core = MixerCore::new();
        let id = SourceId::new();
        let control = StripControl::new(AudioSettings::default());
        // 1 s of a loud tone: momentary must be present and sane.
        run_one(&mut core, id, &control, 0.5, 100);
        let (momentary, _short) = core.lufs();
        let momentary = momentary.expect("above the gate");
        assert!(
            (-14.0..=-4.0).contains(&momentary),
            "a 0.5-amp stereo tone lands around -6..-9 LUFS, got {momentary}"
        );
    }
}
