//! The audio engine thread: owns the cpal streams (they are `!Send`), pulls
//! every source's ring into the [`MixerCore`] on a 10 ms clock, feeds the
//! monitor output, and publishes a levels/status snapshot the app emits to
//! the UI. Commands arrive over a channel from any thread.

use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::Arc;
use std::time::{Duration, Instant};

use fcap_scene::{AudioSettings, MonitorMode, SourceId};
use parking_lot::Mutex;

use crate::capture::{open_capture, CaptureStream};
use crate::graph::{MixerCore, StripControl};
use crate::meter::Levels;
use crate::monitor::{open_monitor, MonitorStream};
use crate::{AudioError, InputSpec, BLOCK_FRAMES};

const TICK: Duration = Duration::from_millis(10);
/// Backoff between monitor-device (re)open attempts after a failure/break.
const MONITOR_RETRY: Duration = Duration::from_secs(2);
const BLOCK_SAMPLES: usize = BLOCK_FRAMES * 2;
/// Wait for this much buffered audio before a source starts mixing —
/// absorbs callback jitter without audible underruns.
const PREBUFFER_SAMPLES: usize = BLOCK_SAMPLES * 3; // 30 ms
/// Past this the ring is trimmed back (device clock running fast).
const MAX_BUFFER_SAMPLES: usize = BLOCK_SAMPLES * 12; // 120 ms

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

/// What the app polls and pushes to the UI (~20 Hz).
#[derive(Debug, Clone, Default)]
pub struct EngineSnapshot {
    pub sources: HashMap<SourceId, SourceSnapshot>,
    pub master: Levels,
    pub lufs_momentary: Option<f32>,
    pub lufs_short_term: Option<f32>,
    /// The monitor device failed (monitoring is requested but silent).
    pub monitor_error: Option<String>,
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
    Keys {
        id: SourceId,
        ptt_held: bool,
        ptm_held: bool,
    },
    RecordTap(Option<RecordTap>),
    /// The live stream's tap — independent of the recording's, so a stream
    /// and a recording can run (and stop) without touching each other.
    StreamTap(Option<RecordTap>),
}

/// Cloneable handle to the engine thread.
#[derive(Clone)]
pub struct AudioEngine {
    tx: mpsc::Sender<Cmd>,
    snapshot: Arc<Mutex<EngineSnapshot>>,
}

impl AudioEngine {
    /// Spawn the engine thread. It runs until every handle is dropped.
    pub fn spawn() -> Self {
        let (tx, rx) = mpsc::channel();
        let snapshot = Arc::new(Mutex::new(EngineSnapshot::default()));
        let shared = Arc::clone(&snapshot);
        std::thread::Builder::new()
            .name("fcap-audio".into())
            .spawn(move || run(rx, shared))
            .expect("audio engine thread spawns");
        Self { tx, snapshot }
    }

    /// Replace the desired source set (the app's reconcile).
    pub fn set_sources(&self, configs: Vec<SourceConfig>) {
        let _ = self.tx.send(Cmd::Sources(configs));
    }

    /// Change the monitor output device ("" = the OS default).
    pub fn set_monitor_device(&self, device_id: String) {
        let _ = self.tx.send(Cmd::MonitorDevice(device_id));
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

    /// The latest levels/status snapshot.
    pub fn snapshot(&self) -> EngineSnapshot {
        self.snapshot.lock().clone()
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

fn run(rx: mpsc::Receiver<Cmd>, shared: Arc<Mutex<EngineSnapshot>>) {
    let mut core = MixerCore::new();
    let mut sources: HashMap<SourceId, SourceRuntime> = HashMap::new();
    let mut record_tap: Option<RecordTap> = None;
    let mut stream_tap: Option<RecordTap> = None;
    let mut monitor: Option<MonitorStream> = None;
    let mut monitor_device = String::new();
    let mut monitor_error: Option<String> = None;
    // When to next attempt a monitor (re)open — a broken/failed device is
    // retried on this backoff instead of never (the reopen must not spin every
    // 10 ms tick).
    let mut next_monitor_retry = Instant::now();
    // The final dropped-sample counts of streams that have been retired
    // (reopened / removed / broken). Added to the live rings' running counts so
    // the published total is **monotonic** — a device that recovers with a
    // fresh ring never makes the count jump backwards.
    let mut retired_dropped = 0u64;
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

        // -- mix one block -----------------------------------------------------
        core.process(&inputs, &controls);

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

        // -- monitor output ----------------------------------------------------
        let monitoring = controls
            .values()
            .any(|control| control.settings.monitor != MonitorMode::Off);
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
        *shared.lock() = EngineSnapshot {
            sources: snapshot_sources,
            master: core.take_master_levels(),
            lufs_momentary,
            lufs_short_term,
            monitor_error: monitor_error.clone(),
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
