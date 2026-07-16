//! The audio engine thread: owns the cpal streams (they are `!Send`), pulls
//! every source's ring into the [`MixerCore`] on a 10 ms clock, feeds the
//! monitor output, and publishes a levels/status snapshot the app emits to
//! the UI. Commands arrive over a channel from any thread.

use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::Arc;
use std::time::{Duration, Instant};

use fcap_scene::{AudioOutputRoute, AudioSettings, MonitorMode, OutputBus, SourceId};
use parking_lot::Mutex;

use crate::capture::{open_capture, CaptureRing, CaptureStream};
use crate::graph::{MixerCore, StripControl, TransitionDuck};
use crate::meter::Levels;
use crate::monitor::{open_monitor, MonitorStream};
use crate::spectrum::SpectrumAnalyzer;
use crate::{AudioError, InputSpec, BLOCK_FRAMES};

const TICK: Duration = Duration::from_millis(10);
/// Backoff between output-device (re)open attempts after a failure/break —
/// shared by the monitor and the CAP-N30 program-bus routes.
const MONITOR_RETRY: Duration = Duration::from_secs(2);
const BLOCK_SAMPLES: usize = BLOCK_FRAMES * 2;
/// The calibration tap's sample cap (~40 s of 10 ms blocks) — a workbench
/// run is ~15 s; the cap only guards a dialog left armed.
const CALIBRATION_MAX_SAMPLES: usize = 4_096;
/// Wait for this much buffered audio before a source starts mixing —
/// absorbs callback jitter without audible underruns.
const PREBUFFER_SAMPLES: usize = BLOCK_SAMPLES * 3; // 30 ms
/// Past this the ring is trimmed back (device clock running fast).
const MAX_BUFFER_SAMPLES: usize = BLOCK_SAMPLES * 12; // 120 ms

/// Linear gain for an output-route trim (CAP-N30), with the fader floor =
/// silence: a trim parked at the bottom mutes the route rather than passing
/// −60 dB through.
fn output_gain(gain_db: f32) -> f32 {
    if !gain_db.is_finite() || gain_db <= fcap_scene::MIN_VOLUME_DB + 0.01 {
        0.0
    } else {
        crate::dsp::db_to_lin(gain_db)
    }
}

/// One audio source the engine should run.
#[derive(Debug, Clone)]
pub struct SourceConfig {
    pub id: SourceId,
    pub input: InputSpec,
    pub settings: AudioSettings,
    /// Bump to force a reopen with unchanged settings (the user's retry).
    pub nonce: u64,
}

/// Honest per-source runtime state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceState {
    Waiting,
    Live,
    Error,
}

impl SourceState {
    pub fn as_str(self) -> &'static str {
        match self {
            SourceState::Waiting => "waiting",
            SourceState::Live => "live",
            SourceState::Error => "error",
        }
    }
}

/// One source's slice of the published snapshot.
#[derive(Debug, Clone)]
pub struct SourceSnapshot {
    pub state: SourceState,
    pub error_code: Option<&'static str>,
    pub error_message: Option<String>,
    pub levels: Levels,
    /// True when the strip mixes silence right now (mute or PTT/PTM gate) —
    /// the UI dims the meter and shows the gate state.
    pub gated: bool,
}

/// One CAP-N30 program-bus output route that could not open its device — the
/// route is configured but currently silent (the UI flags it in the matrix).
#[derive(Debug, Clone)]
pub struct OutputRouteError {
    pub bus: OutputBus,
    pub message: String,
}

/// The CAP-N35 live spectrum of the armed source: log-spaced magnitude bins
/// (dBFS) plus the source it belongs to, so a stale editor ignores it.
#[derive(Debug, Clone)]
pub struct SpectrumSnapshot {
    pub source: SourceId,
    pub magnitudes: Vec<f32>,
}

/// One filter's live meter (the armed strip's plugin editor): linear in/out
/// peaks; the UI derives gain reduction as 20·log10(out/in).
#[derive(Debug, Clone, Copy)]
pub struct FilterMeter {
    pub id: fcap_scene::AudioFilterId,
    pub in_peak: f32,
    pub out_peak: f32,
}

/// The armed strip's per-filter meters + the source they belong to.
#[derive(Debug, Clone)]
pub struct FilterMetersSnapshot {
    pub source: SourceId,
    pub meters: Vec<FilterMeter>,
}

/// What the app polls and pushes to the UI (~20 Hz).
#[derive(Debug, Clone, Default)]
pub struct EngineSnapshot {
    pub sources: HashMap<SourceId, SourceSnapshot>,
    pub master: Levels,
    pub lufs_momentary: Option<f32>,
    pub lufs_short_term: Option<f32>,
    /// The monitor device failed (monitoring is requested but silent).
    pub monitor_error: Option<String>,
    /// CAP-N30 program-bus routes that failed to open their device.
    pub output_errors: Vec<OutputRouteError>,
    /// CAP-N35 live spectrum of the armed source, when a parametric-EQ editor
    /// is open (`None` when nothing is armed — costs nothing otherwise).
    pub spectrum: Option<SpectrumSnapshot>,
    /// Per-filter live meters for the strip whose filter editor is open.
    pub filter_meters: Option<FilterMetersSnapshot>,
    /// Total capture samples dropped across sources (ring overflows).
    pub dropped: u64,
}

/// The recording tap (Phase 4): called on the engine thread after **every**
/// mixed block with the enabled track buses, in ascending track order. It
/// runs inside the 10 ms loop — implementations must only hand the data off
/// (the recorder does its own buffering, pause gating, and position
/// accounting). Captured audio still goes nowhere but the mixer, the
/// monitor, and the recording file.
pub struct RecordTap {
    /// Bitmask of the tracks to deliver (bit 0 = track 1).
    pub tracks: u8,
    /// `(track_index, block)` pairs for the enabled tracks, every 10 ms.
    #[allow(clippy::type_complexity)]
    pub sink: Box<dyn FnMut(&[(usize, &[f32])]) + Send>,
}

enum Cmd {
    Sources(Vec<SourceConfig>),
    MonitorDevice(String),
    /// The CAP-N30 program-bus output routes (master / track buses → physical
    /// output devices). Replaces the whole set; empty = today's behavior
    /// (only the monitor bus reaches a device).
    AudioOutputs(Vec<AudioOutputRoute>),
    /// Arm (`Some`) or disarm (`None`) the CAP-N35 spectrum tap on one source —
    /// the strip whose parametric-EQ editor is open.
    Spectrum(Option<SourceId>),
    /// Arm (`Some`) or disarm (`None`) per-filter metering on one source — the
    /// strip whose filter editor is open (the plugin meters).
    MeterTarget(Option<SourceId>),
    /// Arm (`Some((target_lufs, ceiling_db))`) or disarm (`None`) the CAP-N34
    /// loudness rider over the program.
    Loudness(Option<(f32, f32)>),
    /// The CAP-N37 soundboard auto-duck trigger set (currently-playing auto-duck
    /// pad source ids). Empty = idle.
    SoundboardDuck(Vec<SourceId>),
    Keys {
        id: SourceId,
        ptt_held: bool,
        ptm_held: bool,
    },
    RecordTap(Option<RecordTap>),
    /// The live stream's tap — independent of the recording's, so a stream
    /// and a recording can run (and stop) without touching each other.
    StreamTap(Option<RecordTap>),
    /// The replay buffer's tap (Phase 6) — the third independent twin, so
    /// the rolling buffer never contends with recording or streaming.
    ReplayTap(Option<RecordTap>),
    /// Arm (or clear) the A/V sync calibration tap (CAP-M20): record one
    /// source's raw pre-gain block peaks, timestamped against the shared
    /// arm instant so the video probe's clock matches.
    Calibrate(Option<(SourceId, Instant)>),
    /// Arm (or release) the transition audio duck (CAP-N29): while a stinger
    /// plays, its decoded audio (drained from the named media-hub ring) ducks
    /// the program mix. `None` releases it.
    TransitionDuck(Option<TransitionDuckSpec>),
}

/// What arms the transition duck (CAP-N29): the media-hub id whose decoded
/// audio drives the duck (the stinger's), plus the duck shape.
#[derive(Debug, Clone)]
pub struct TransitionDuckSpec {
    /// The `media_hub` ring id the stinger's audio is decoded into.
    pub hub_id: String,
    pub depth_db: f32,
    pub attack_ms: f32,
    pub release_ms: f32,
    pub threshold_db: f32,
}

/// Cloneable handle to the engine thread.
#[derive(Clone)]
pub struct AudioEngine {
    tx: mpsc::Sender<Cmd>,
    snapshot: Arc<Mutex<EngineSnapshot>>,
    /// (ms since arm, block peak 0..1) — written only by the engine thread.
    calibration: Arc<Mutex<Vec<(f64, f32)>>>,
}

impl AudioEngine {
    /// Spawn the engine thread. It runs until every handle is dropped.
    pub fn spawn() -> Self {
        let (tx, rx) = mpsc::channel();
        let snapshot = Arc::new(Mutex::new(EngineSnapshot::default()));
        let shared = Arc::clone(&snapshot);
        let calibration = Arc::new(Mutex::new(Vec::new()));
        let calibration_shared = Arc::clone(&calibration);
        std::thread::Builder::new()
            .name("fcap-audio".into())
            .spawn(move || run(rx, shared, calibration_shared))
            .expect("audio engine thread spawns");
        Self {
            tx,
            snapshot,
            calibration,
        }
    }

    /// Replace the desired source set (the app's reconcile).
    pub fn set_sources(&self, configs: Vec<SourceConfig>) {
        let _ = self.tx.send(Cmd::Sources(configs));
    }

    /// Change the monitor output device ("" = the OS default).
    pub fn set_monitor_device(&self, device_id: String) {
        let _ = self.tx.send(Cmd::MonitorDevice(device_id));
    }

    /// Replace the CAP-N30 program-bus output routes (master / track buses →
    /// physical output devices). An empty set routes nothing but the monitor.
    pub fn set_audio_outputs(&self, routes: Vec<AudioOutputRoute>) {
        let _ = self.tx.send(Cmd::AudioOutputs(routes));
    }

    /// Arm (or clear with `None`) the CAP-N35 spectrum tap on one source.
    pub fn set_spectrum_target(&self, source: Option<SourceId>) {
        let _ = self.tx.send(Cmd::Spectrum(source));
    }

    /// Arm (or clear with `None`) per-filter metering on one source.
    pub fn set_meter_target(&self, source: Option<SourceId>) {
        let _ = self.tx.send(Cmd::MeterTarget(source));
    }

    /// Arm (or clear with `None`) the CAP-N34 loudness rider over the program.
    pub fn set_loudness(&self, spec: Option<(f32, f32)>) {
        let _ = self.tx.send(Cmd::Loudness(spec));
    }

    /// Set the CAP-N37 soundboard auto-duck trigger set (empty = idle).
    pub fn set_soundboard_duck(&self, triggers: Vec<SourceId>) {
        let _ = self.tx.send(Cmd::SoundboardDuck(triggers));
    }

    /// Update a source's push-to-talk / push-to-mute key state.
    pub fn set_key_state(&self, id: SourceId, ptt_held: bool, ptm_held: bool) {
        let _ = self.tx.send(Cmd::Keys {
            id,
            ptt_held,
            ptm_held,
        });
    }

    /// Install (or clear) the recording tap. The tap sees every mixed block
    /// from the moment it lands on the engine thread.
    pub fn set_record_tap(&self, tap: Option<RecordTap>) {
        let _ = self.tx.send(Cmd::RecordTap(tap));
    }

    /// Install (or clear) the stream tap — the recording tap's independent
    /// twin, so streaming never contends with the local copy.
    pub fn set_stream_tap(&self, tap: Option<RecordTap>) {
        let _ = self.tx.send(Cmd::StreamTap(tap));
    }

    /// Install (or clear) the replay-buffer tap — the third independent
    /// twin, so the rolling buffer never contends with the other two.
    pub fn set_replay_tap(&self, tap: Option<RecordTap>) {
        let _ = self.tx.send(Cmd::ReplayTap(tap));
    }

    /// The latest levels/status snapshot.
    pub fn snapshot(&self) -> EngineSnapshot {
        self.snapshot.lock().clone()
    }

    /// Arm (or clear) the calibration tap on one source (CAP-M20). Pass the
    /// same `Instant` the video probe was armed with — both series share it
    /// as their zero. The buffer is cleared by the engine thread when the
    /// command lands, so stale samples can never leak into a fresh run.
    pub fn calibrate(&self, target: Option<(SourceId, Instant)>) {
        let _ = self.tx.send(Cmd::Calibrate(target));
    }

    /// The calibration series recorded so far: (ms since arm, block peak).
    pub fn calibration_series(&self) -> Vec<(f64, f32)> {
        self.calibration.lock().clone()
    }

    /// Arm (or release with `None`) the transition audio duck (CAP-N29).
    pub fn set_transition_duck(&self, duck: Option<TransitionDuckSpec>) {
        let _ = self.tx.send(Cmd::TransitionDuck(duck));
    }
}

/// Everything the engine tracks per source.
struct SourceRuntime {
    config: SourceConfig,
    stream: Option<CaptureStream>,
    error: Option<AudioError>,
    /// Prebuffer passed — the ring is feeding blocks.
    started: bool,
    ptt_held: bool,
    ptm_held: bool,
}

/// One CAP-N30 program-bus route the engine keeps a device open for. Mirrors
/// the monitor's open/retry/broken bookkeeping so a route survives an
/// unplug/replug the same way monitoring does.
struct OutputRuntime {
    route: AudioOutputRoute,
    stream: Option<MonitorStream>,
    /// Cached linear trim (`output_gain(route.gain_db)`).
    gain: f32,
    error: Option<String>,
    /// When to next attempt an open after a failure/break.
    next_retry: Instant,
    /// Reused scaled block, so applying the trim allocates nothing per tick.
    scratch: Vec<f32>,
}

impl OutputRuntime {
    fn new(route: AudioOutputRoute) -> Self {
        Self {
            gain: output_gain(route.gain_db),
            route,
            stream: None,
            error: None,
            next_retry: Instant::now(),
            scratch: Vec::with_capacity(BLOCK_SAMPLES),
        }
    }
}

impl SourceRuntime {
    /// This runtime's ring's cumulative dropped-sample count (0 if no stream).
    fn dropped(&self) -> u64 {
        self.stream
            .as_ref()
            .map(|stream| stream.ring().dropped())
            .unwrap_or(0)
    }

    fn open(config: SourceConfig) -> Self {
        let (stream, error) = match open_capture(&config.input) {
            Ok(stream) => (Some(stream), None),
            Err(err) => (None, Some(err)),
        };
        Self {
            config,
            stream,
            error,
            started: false,
            ptt_held: false,
            ptm_held: false,
        }
    }
}

fn run(
    rx: mpsc::Receiver<Cmd>,
    shared: Arc<Mutex<EngineSnapshot>>,
    calibration: Arc<Mutex<Vec<(f64, f32)>>>,
) {
    let mut core = MixerCore::new();
    let mut sources: HashMap<SourceId, SourceRuntime> = HashMap::new();
    let mut record_tap: Option<RecordTap> = None;
    let mut stream_tap: Option<RecordTap> = None;
    let mut replay_tap: Option<RecordTap> = None;
    let mut calibration_target: Option<(SourceId, Instant)> = None;
    let mut monitor: Option<MonitorStream> = None;
    let mut monitor_device = String::new();
    let mut monitor_error: Option<String> = None;
    // CAP-N30 program-bus output routes (master / track buses → physical
    // devices). Empty by default — nothing but the monitor reaches a device.
    let mut outputs: Vec<OutputRuntime> = Vec::new();
    // CAP-N35 spectrum tap: the armed source (a strip's EQ editor is open) and
    // its rolling analyzer. `None` = nothing armed, so this costs nothing.
    let mut spectrum_target: Option<SourceId> = None;
    // The armed source's strip key, cached on arm so the per-block tap avoids a
    // `to_string()` allocation every 10 ms tick.
    let mut spectrum_key: Option<String> = None;
    let mut spectrum = SpectrumAnalyzer::new();
    // The strip whose filter editor is open (per-filter plugin meters).
    let mut meter_target: Option<SourceId> = None;
    // CAP-N34 loudness rider config currently armed (rebuild only on change).
    let mut loudness_spec: Option<(f32, f32)> = None;
    // CAP-N39 mix-minus return rings, HELD so the per-block publish never
    // re-locks the media hub or reallocates a swept ring (the hub is a
    // per-reconcile lookup, not per-block). Rebuilt only when the guest set
    // changes; `mixminus_present` is a reused scratch to prune departed guests.
    let mut mixminus_rings: HashMap<SourceId, Arc<CaptureRing>> = HashMap::new();
    let mut mixminus_present: Vec<SourceId> = Vec::new();
    // When to next attempt a monitor (re)open — a broken/failed device is
    // retried on this backoff instead of never (the reopen must not spin every
    // 10 ms tick).
    let mut next_monitor_retry = Instant::now();
    // The final dropped-sample counts of streams that have been retired
    // (reopened / removed / broken). Added to the live rings' running counts so
    // the published total is **monotonic** — a device that recovers with a
    // fresh ring never makes the count jump backwards.
    let mut retired_dropped = 0u64;
    // The media-hub id whose audio drives the transition duck (CAP-N29), while
    // armed. `None` between transitions — the duck is released and idle.
    let mut duck_hub: Option<String> = None;
    // Reused each block to drain the stinger's audio (peak only) without a
    // per-block allocation on this real-time thread.
    let mut duck_trigger: Vec<f32> = Vec::new();
    let mut next_tick = Instant::now() + TICK;

    loop {
        // -- commands ---------------------------------------------------------
        loop {
            match rx.try_recv() {
                Ok(Cmd::Sources(configs)) => {
                    // Media-hub rings self-GC (weak-referenced) — a ring lives
                    // as long as its decoder or this engine holds it, so a
                    // hidden-then-shown media source keeps its audio. No
                    // config-set-driven drop here (that stranded the decoder).
                    let mut next: HashMap<SourceId, SourceRuntime> =
                        HashMap::with_capacity(configs.len());
                    for config in configs {
                        match sources.remove(&config.id) {
                            // Same input + nonce: keep the running stream,
                            // refresh the settings.
                            Some(mut runtime)
                                if runtime.config.input == config.input
                                    && runtime.config.nonce == config.nonce =>
                            {
                                runtime.config = config;
                                next.insert(runtime.config.id, runtime);
                            }
                            // A reopen (input/nonce changed): preserve the old
                            // ring's final drop count before it is dropped.
                            other => {
                                if let Some(old) = other {
                                    retired_dropped += old.dropped();
                                }
                                next.insert(config.id, SourceRuntime::open(config));
                            }
                        }
                    }
                    // Whatever is left in `sources` is being removed — retire
                    // its drop count too so the total never regresses.
                    for (_, old) in sources.drain() {
                        retired_dropped += old.dropped();
                    }
                    sources = next; // dropped runtimes stop their streams
                }
                Ok(Cmd::MonitorDevice(device_id)) => {
                    if device_id != monitor_device {
                        monitor_device = device_id;
                        monitor = None;
                        monitor_error = None; // reopen on the new device
                        next_monitor_retry = Instant::now();
                    }
                }
                Ok(Cmd::AudioOutputs(routes)) => {
                    // Reconcile in place: a route whose device is unchanged
                    // keeps its open stream (a trim edit never reopens); a
                    // changed device reopens; a dropped route closes.
                    let mut next: Vec<OutputRuntime> = Vec::with_capacity(routes.len());
                    for route in routes {
                        match outputs.iter().position(|out| out.route.bus == route.bus) {
                            Some(pos) => {
                                let mut existing = outputs.swap_remove(pos);
                                if existing.route.device_id != route.device_id {
                                    existing.stream = None; // device changed → reopen
                                    existing.error = None;
                                    existing.next_retry = Instant::now();
                                }
                                existing.gain = output_gain(route.gain_db);
                                existing.route = route;
                                next.push(existing);
                            }
                            None => next.push(OutputRuntime::new(route)),
                        }
                    }
                    outputs = next; // dropped runtimes close their streams
                }
                Ok(Cmd::Spectrum(target)) => {
                    if target != spectrum_target {
                        spectrum_key = target.as_ref().map(|id| id.0.to_string());
                        spectrum_target = target;
                        spectrum.clear(); // fresh history on a re-arm
                    }
                }
                Ok(Cmd::MeterTarget(target)) => {
                    if target != meter_target {
                        meter_target = target;
                        core.set_meter_target(target);
                    }
                }
                Ok(Cmd::Loudness(spec)) => {
                    if spec != loudness_spec {
                        loudness_spec = spec;
                        core.set_loudness(spec); // keeps the rider on an unchanged re-arm
                    }
                }
                Ok(Cmd::SoundboardDuck(triggers)) => core.set_soundboard_duck(triggers),
                Ok(Cmd::Keys {
                    id,
                    ptt_held,
                    ptm_held,
                }) => {
                    if let Some(runtime) = sources.get_mut(&id) {
                        runtime.ptt_held = ptt_held;
                        runtime.ptm_held = ptm_held;
                    }
                }
                Ok(Cmd::RecordTap(tap)) => record_tap = tap,
                Ok(Cmd::StreamTap(tap)) => stream_tap = tap,
                Ok(Cmd::ReplayTap(tap)) => replay_tap = tap,
                Ok(Cmd::Calibrate(target)) => {
                    calibration_target = target;
                    // Cleared here (engine thread), never handle-side, so a
                    // late block from the previous target can't leak in.
                    calibration.lock().clear();
                }
                Ok(Cmd::TransitionDuck(spec)) => match spec {
                    Some(spec) => {
                        core.set_transition_duck(Some(TransitionDuck::new(
                            spec.depth_db,
                            spec.attack_ms,
                            spec.release_ms,
                            spec.threshold_db,
                        )));
                        duck_hub = Some(spec.hub_id);
                    }
                    None => {
                        core.set_transition_duck(None); // ramps out, then clears
                        duck_hub = None;
                    }
                },
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => return, // app gone
            }
        }

        // -- pull every ring, build the control set ---------------------------
        let mut inputs: HashMap<SourceId, Vec<f32>> = HashMap::with_capacity(sources.len());
        let mut controls: HashMap<SourceId, StripControl> = HashMap::with_capacity(sources.len());
        let mut dropped_total = 0u64;
        for (id, runtime) in sources.iter_mut() {
            controls.insert(
                *id,
                StripControl {
                    settings: runtime.config.settings.clone(),
                    ptt_held: runtime.ptt_held,
                    ptm_held: runtime.ptm_held,
                },
            );
            let Some(stream) = &runtime.stream else {
                continue;
            };
            let ring = stream.ring();
            if ring.is_broken() {
                // Retire this ring's final count (the live-ring sum below won't
                // see it again once the stream is dropped).
                retired_dropped += ring.dropped();
                runtime.error = Some(AudioError::Backend(
                    "the device stream ended — retry or re-pick the device".into(),
                ));
                runtime.stream = None;
                runtime.started = false;
                continue;
            }
            dropped_total += ring.dropped();
            if !runtime.started && ring.len() >= PREBUFFER_SAMPLES {
                runtime.started = true;
            }
            if runtime.started {
                if ring.len() > MAX_BUFFER_SAMPLES {
                    ring.trim_to(PREBUFFER_SAMPLES); // device clock ran fast
                }
                let mut block = Vec::with_capacity(BLOCK_SAMPLES);
                let got = ring.pop_into(&mut block, BLOCK_SAMPLES);
                if got < BLOCK_SAMPLES {
                    block.resize(BLOCK_SAMPLES, 0.0); // underrun → silence
                }
                inputs.insert(*id, block);
            }
        }

        // -- the calibration tap (CAP-M20): the armed source's RAW block
        //    peak — pre-gain, pre-sync-delay — so the measurement doesn't
        //    depend on the fader or the offset being tuned -------------------
        if let Some((id, armed_at)) = &calibration_target {
            if let Some(block) = inputs.get(id) {
                let peak = block.iter().fold(0.0f32, |acc, s| acc.max(s.abs()));
                let mut series = calibration.lock();
                if series.len() < CALIBRATION_MAX_SAMPLES {
                    series.push((armed_at.elapsed().as_secs_f64() * 1_000.0, peak));
                }
            }
        }

        // -- the transition duck's trigger (CAP-N29): drain the stinger's
        //    decoded audio and feed its peak envelope to the duck. The samples
        //    are used for their loudness only — never mixed into the program —
        //    and the ring is emptied each tick so unplayed audio can't lag ----
        if let Some(hub) = &duck_hub {
            let ring = crate::media_hub::ring(hub);
            duck_trigger.clear();
            ring.pop_into(&mut duck_trigger, ring.len());
            core.feed_duck_trigger(&duck_trigger);
        }

        // -- mix one block -----------------------------------------------------
        core.process(&inputs, &controls);

        // -- the visualizer taps (CAP-N15): push this block for every live
        //    subscription — post-fader strips, track buses, or the master.
        //    No subscribers (the common case) costs one map lock, no copies.
        //    Iterated in place under the registry lock — a collected Vec of
        //    cloned targets allocated per block here, which is real-time-
        //    audio poison; holding the lock across these short `push_block`
        //    copies is the cheaper trade.
        crate::vis::for_each_live(|target, ring| match target {
            crate::vis::VisTarget::Master => ring.push_block(core.master()),
            crate::vis::VisTarget::Track(index) => {
                if *index < fcap_scene::TRACK_COUNT {
                    ring.push_block(core.track(*index));
                }
            }
            crate::vis::VisTarget::Source(key) => {
                if let Some(block) = core.strip_block(key) {
                    ring.push_block(block);
                }
            }
        });

        // -- CAP-N35 spectrum tap: feed the armed source's post-chain block
        //    (post-EQ — the analyzer shows what the strip actually outputs) ----
        if let Some(key) = &spectrum_key {
            if let Some(block) = core.strip_block(key) {
                spectrum.push(block);
            }
        }

        // -- CAP-N39 mix-minus: publish each guest's N−1 return into a hub ring
        //    (`mixminus:<id>`) the remote transport drains as their echo-free
        //    return. The ring Arc is HELD across ticks, so the common case is a
        //    map hit + push — no hub lock, no `format!`, and (critically) no
        //    realloc of a swept ring when the guest's consumer is offline.
        mixminus_present.clear();
        core.for_each_mix_minus(|id, block| {
            let ring = mixminus_rings
                .entry(id)
                .or_insert_with(|| crate::media_hub::ring(&format!("mixminus:{}", id.0)));
            ring.push(block);
            mixminus_present.push(id);
        });
        // A guest left → drop our Arc so its ring frees once the consumer does.
        if mixminus_rings.len() != mixminus_present.len() {
            mixminus_rings.retain(|id, _| mixminus_present.contains(id));
        }

        // -- the recording tap (P4): every block, whether sources exist or
        //    not — a video-only scene records silent tracks, correctly ------
        if let Some(tap) = &mut record_tap {
            let blocks: Vec<(usize, &[f32])> = (0..fcap_scene::TRACK_COUNT)
                .filter(|index| tap.tracks & (1 << index) != 0)
                .map(|index| (index, core.track(index)))
                .collect();
            (tap.sink)(&blocks);
        }
        if let Some(tap) = &mut stream_tap {
            let blocks: Vec<(usize, &[f32])> = (0..fcap_scene::TRACK_COUNT)
                .filter(|index| tap.tracks & (1 << index) != 0)
                .map(|index| (index, core.track(index)))
                .collect();
            (tap.sink)(&blocks);
        }
        if let Some(tap) = &mut replay_tap {
            let blocks: Vec<(usize, &[f32])> = (0..fcap_scene::TRACK_COUNT)
                .filter(|index| tap.tracks & (1 << index) != 0)
                .map(|index| (index, core.track(index)))
                .collect();
            (tap.sink)(&blocks);
        }

        // -- monitor output ----------------------------------------------------
        // A soloed strip monitors even with its monitor mode Off (PFL,
        // CAP-M19) — the device must open for it too.
        let monitoring = controls
            .values()
            .any(|control| control.settings.monitor != MonitorMode::Off || control.settings.solo);
        if monitoring {
            // Open (or reopen after a break) on a backoff — a device that
            // failed or died is retried, not abandoned until the user toggles
            // monitoring off.
            if monitor.is_none() && Instant::now() >= next_monitor_retry {
                match open_monitor(&monitor_device) {
                    Ok(stream) => {
                        monitor = Some(stream);
                        monitor_error = None;
                    }
                    Err(err) => {
                        monitor_error = Some(err.to_string());
                        next_monitor_retry = Instant::now() + MONITOR_RETRY;
                    }
                }
            }
            if let Some(stream) = &mut monitor {
                if stream.is_broken() {
                    monitor = None;
                    monitor_error = Some("the monitor device stream ended".into());
                    next_monitor_retry = Instant::now() + MONITOR_RETRY;
                } else {
                    stream.push(core.monitor());
                }
            }
        } else {
            // Nothing monitors: release the device; a failed device gets a
            // fresh chance the moment monitoring turns on again.
            monitor = None;
            monitor_error = None;
            next_monitor_retry = Instant::now();
        }

        // -- CAP-N30 program-bus output routes ---------------------------------
        // Each configured route holds its output device open (a hardware-
        // recorder feed / other-room speakers want to stay fed), pushing its
        // bus's block — trimmed — every tick. A device that fails or dies is
        // retried on the shared backoff, never abandoned. The buses read here
        // are post-duck, post-mix — exactly what records and streams.
        let now = Instant::now();
        for out in outputs.iter_mut() {
            if out.stream.is_none() && now >= out.next_retry {
                match open_monitor(&out.route.device_id) {
                    Ok(stream) => {
                        out.stream = Some(stream);
                        out.error = None;
                    }
                    Err(err) => {
                        out.error = Some(err.to_string());
                        out.next_retry = now + MONITOR_RETRY;
                    }
                }
            }
            let Some(stream) = &mut out.stream else {
                continue;
            };
            if stream.is_broken() {
                out.stream = None;
                out.error = Some("the output device stream ended".into());
                out.next_retry = now + MONITOR_RETRY;
                continue;
            }
            let bus = match out.route.bus {
                OutputBus::Master => core.master(),
                OutputBus::Track { index } => {
                    let index = index as usize;
                    if index < fcap_scene::TRACK_COUNT {
                        core.track(index)
                    } else {
                        continue; // clamp keeps this unreachable; stay safe
                    }
                }
            };
            if (out.gain - 1.0).abs() < 1e-4 {
                stream.push(bus);
            } else {
                out.scratch.clear();
                out.scratch.extend_from_slice(bus);
                for sample in out.scratch.iter_mut() {
                    *sample *= out.gain;
                }
                stream.push(&out.scratch);
            }
        }

        // -- publish the snapshot ----------------------------------------------
        let mut snapshot_sources = HashMap::with_capacity(sources.len());
        for (id, runtime) in sources.iter() {
            let levels = core.take_source_levels(*id).unwrap_or_default();
            let (state, error_code, error_message) = match &runtime.error {
                Some(err) => (SourceState::Error, Some(err.code()), Some(err.to_string())),
                None if runtime.started => (SourceState::Live, None, None),
                None => (SourceState::Waiting, None, None),
            };
            let gated = controls
                .get(id)
                .is_some_and(|control| control.effectively_muted());
            snapshot_sources.insert(
                *id,
                SourceSnapshot {
                    state,
                    error_code,
                    error_message,
                    levels,
                    gated,
                },
            );
        }
        let (lufs_momentary, lufs_short_term) = core.lufs();
        let output_errors = outputs
            .iter()
            .filter_map(|out| {
                out.error.clone().map(|message| OutputRouteError {
                    bus: out.route.bus,
                    message,
                })
            })
            .collect();
        let spectrum_out = spectrum_target.map(|source| SpectrumSnapshot {
            source,
            magnitudes: spectrum.magnitudes(),
        });
        let filter_meters_out = meter_target.map(|source| FilterMetersSnapshot {
            source,
            meters: core
                .filter_meters()
                .iter()
                .map(|&(id, in_peak, out_peak)| FilterMeter {
                    id,
                    in_peak,
                    out_peak,
                })
                .collect(),
        });
        *shared.lock() = EngineSnapshot {
            sources: snapshot_sources,
            master: core.take_master_levels(),
            lufs_momentary,
            lufs_short_term,
            monitor_error: monitor_error.clone(),
            output_errors,
            spectrum: spectrum_out,
            filter_meters: filter_meters_out,
            dropped: retired_dropped + dropped_total,
        };

        // -- pace to the 10 ms block clock --------------------------------------
        let now = Instant::now();
        if next_tick > now {
            std::thread::sleep(next_tick - now);
        }
        next_tick += TICK;
        // After a long stall (debugger, suspend) resync instead of sprinting.
        if next_tick + Duration::from_millis(100) < Instant::now() {
            next_tick = Instant::now() + TICK;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::output_gain;
    use fcap_scene::{MAX_VOLUME_DB, MIN_VOLUME_DB};

    #[test]
    fn output_trim_floor_is_silence_and_unity_passes() {
        assert_eq!(output_gain(MIN_VOLUME_DB), 0.0, "the trim floor mutes");
        assert_eq!(output_gain(f32::NAN), 0.0, "a non-finite trim mutes");
        assert!((output_gain(0.0) - 1.0).abs() < 1e-6, "0 dB = unity");
        let half = output_gain(-6.0);
        assert!((half - 0.501).abs() < 0.01, "-6 dB ~ 0.5, got {half}");
        assert!(output_gain(MAX_VOLUME_DB) > 1.0, "positive trim boosts");
    }
}
