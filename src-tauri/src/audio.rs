//! The app side of the audio engine: a reconcile thread that keeps the
//! engine's source set in sync with the studio model (and the monitor device
//! with the settings), registers push-to-talk / push-to-mute global
//! shortcuts, and emits the `audio` levels/status event (~20 Hz).
//!
//! Captured audio stays on this machine: it flows to the mixer, the monitor
//! device the user chose, and (from Phase 4) the recording tracks — nothing
//! else. Global hotkeys may be unavailable on Wayland; registration failures
//! are logged, never fatal.

use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, Runtime};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

use fcap_audio::{AudioEngine, EngineSnapshot, InputSpec, SourceConfig};
use fcap_scene::{SourceId, SourceSettings};

use crate::settings::SettingsStore;
use crate::studio::{AudioSourceSpec, StudioState};

/// How often the thread reconciles + emits levels (20 Hz).
const AUDIO_TICK: Duration = Duration::from_millis(50);

/// Tauri-managed handle to the engine.
pub struct AudioRuntime {
    pub engine: AudioEngine,
}

impl AudioRuntime {
    pub fn new() -> Self {
        Self {
            engine: AudioEngine::spawn(),
        }
    }

    /// Peak dBFS per mixer strip, keyed by the SOURCE NAME (CAP-N01's
    /// audio-level trigger — a rule names the strip the user sees).
    /// Silent/absent strips report a floor, never `-inf`.
    pub fn peaks_by_name(
        &self,
        names: &std::collections::HashMap<fcap_scene::SourceId, String>,
    ) -> std::collections::HashMap<String, f32> {
        let snapshot = self.engine.snapshot();
        snapshot
            .sources
            .into_iter()
            .filter_map(|(id, source)| {
                let name = names.get(&id)?.clone();
                let peak = source.levels.peak[0].max(source.levels.peak[1]);
                let db = if peak > 1e-6 {
                    20.0 * peak.log10()
                } else {
                    -120.0
                };
                Some((name, db))
            })
            .collect()
    }
}

/// What a hotkey does to its source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HotkeyRole {
    Talk,
    Mute,
}

/// The live shortcut → source bindings + per-source held state, shared
/// between the plugin's handler and the reconcile thread.
#[derive(Default)]
pub struct HotkeyRegistry {
    bindings: Mutex<HashMap<Shortcut, Vec<(SourceId, HotkeyRole)>>>,
    held: Mutex<HashMap<SourceId, (bool, bool)>>,
    /// The shortcuts the OS actually accepted. A push-to-talk whose hotkey is
    /// **not** here can never be un-gated (the OS won't deliver the key — e.g.
    /// on Wayland), so the source is run **fail-open** (audible) rather than
    /// silent-forever; the engine only gates on hotkeys in this set.
    registered: Mutex<HashSet<Shortcut>>,
}

impl HotkeyRegistry {
    /// A snapshot of the currently OS-registered shortcuts.
    pub fn registered(&self) -> HashSet<Shortcut> {
        lock(&self.registered).clone()
    }
}

fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// The global-shortcut plugin handler: route a press/release to the engine.
pub fn on_hotkey<R: Runtime>(app: &AppHandle<R>, shortcut: &Shortcut, state: ShortcutState) {
    let registry = app.state::<HotkeyRegistry>();
    let targets = match lock(&registry.bindings).get(shortcut) {
        Some(targets) => targets.clone(),
        None => return,
    };
    let engine = app.state::<AudioRuntime>().engine.clone();
    let pressed = state == ShortcutState::Pressed;
    let mut held = lock(&registry.held);
    for (source, role) in targets {
        let entry = held.entry(source).or_insert((false, false));
        match role {
            HotkeyRole::Talk => entry.0 = pressed,
            HotkeyRole::Mute => entry.1 = pressed,
        }
        engine.set_key_state(source, entry.0, entry.1);
    }
}

/// Keep the registered global shortcuts in sync with the model. Registration
/// runs on the main thread (macOS requires it); failures are logged honestly
/// (Wayland has no global hotkeys) and never break the mix.
fn reconcile_hotkeys<R: Runtime>(app: &AppHandle<R>, specs: &[AudioSourceSpec]) {
    let mut desired: HashMap<Shortcut, Vec<(SourceId, HotkeyRole)>> = HashMap::new();
    for spec in specs {
        for (key, role) in [
            (&spec.audio.push_to_talk, HotkeyRole::Talk),
            (&spec.audio.push_to_mute, HotkeyRole::Mute),
        ] {
            let Some(text) = key else { continue };
            match text.parse::<Shortcut>() {
                Ok(shortcut) => desired.entry(shortcut).or_default().push((spec.id, role)),
                Err(err) => eprintln!("audio: unusable hotkey {text:?}: {err}"),
            }
        }
    }

    let registry = app.state::<HotkeyRegistry>();
    // Diff against what is actually OS-registered (not merely desired) so a
    // shortcut that failed to register last time is retried.
    let registered_now = lock(&registry.registered).clone();
    let to_register: Vec<Shortcut> = desired
        .keys()
        .filter(|shortcut| !registered_now.contains(shortcut))
        .copied()
        .collect();
    let to_unregister: Vec<Shortcut> = registered_now
        .iter()
        .filter(|shortcut| !desired.contains_key(shortcut))
        .copied()
        .collect();
    *lock(&registry.bindings) = desired;
    // Held state only survives for sources that still exist.
    let live: Vec<SourceId> = specs.iter().map(|spec| spec.id).collect();
    lock(&registry.held).retain(|id, _| live.contains(id));

    if to_register.is_empty() && to_unregister.is_empty() {
        return;
    }
    // Register on the main thread (macOS requires it) and **wait** for the
    // result: the caller needs the updated `registered` set before it decides
    // which push-to-talk gates are actually live. Failures are logged, never
    // fatal, and leave the shortcut out of `registered` (→ fail-open).
    let (tx, rx) = std::sync::mpsc::channel();
    let handle = app.clone();
    let dispatched = app.run_on_main_thread(move || {
        let shortcuts = handle.global_shortcut();
        let registry = handle.state::<HotkeyRegistry>();
        let mut registered = lock(&registry.registered);
        for shortcut in to_unregister {
            if let Err(err) = shortcuts.unregister(shortcut) {
                eprintln!("audio: could not unregister a hotkey: {err}");
            }
            registered.remove(&shortcut);
        }
        for shortcut in to_register {
            match shortcuts.register(shortcut) {
                Ok(()) => {
                    registered.insert(shortcut);
                }
                Err(err) => eprintln!(
                    "audio: could not register a hotkey ({err}) — global hotkeys \
                     may be unavailable here (e.g. Wayland); the source runs \
                     audible (push-to-talk cannot gate it)"
                ),
            }
        }
        let _ = tx.send(());
    });
    match dispatched {
        // Bounded wait — never hang the bridge if the main loop is busy.
        Ok(()) => {
            let _ = rx.recv_timeout(Duration::from_secs(2));
        }
        Err(err) => {
            eprintln!("audio: hotkey registration could not reach the main thread: {err}")
        }
    }
}

// ---------------------------------------------------------------------------
// The `audio` event payload (mirrored in ui/src/api/types.ts)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelsDto {
    /// Linear peak per channel [L, R] since the last event.
    pub peak: [f32; 2],
    /// Linear RMS per channel [L, R] since the last event.
    pub rms: [f32; 2],
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioSourceDto {
    /// "waiting" | "live" | "error"
    pub state: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    pub peak: [f32; 2],
    pub rms: [f32; 2],
    /// The strip mixes silence right now (mute or a PTT/PTM gate).
    pub gated: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LufsDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub momentary: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_term: Option<f32>,
}

/// One CAP-N30 output route that could not open its device (the UI flags the
/// bus in the routing matrix). `bus` is the stable key: "master" | "track1"…
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputRouteErrorDto {
    pub bus: String,
    pub message: String,
}

/// The CAP-N35 live spectrum of the armed source: dBFS magnitude bins + the
/// source id (a stale editor ignores a spectrum that isn't its own).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpectrumDto {
    pub source: String,
    pub magnitudes: Vec<f32>,
}

/// One filter's live meter (linear in/out peaks) for the armed strip's editor.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterMeterDto {
    pub id: String,
    pub in_peak: f32,
    pub out_peak: f32,
}

/// The armed strip's per-filter meters + the source they belong to.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterMetersDto {
    pub source: String,
    pub meters: Vec<FilterMeterDto>,
}

/// The `audio` event: per-source levels/status + the program mix health.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioLevelsDto {
    /// Keyed by source id (UUID string).
    pub sources: HashMap<String, AudioSourceDto>,
    pub master: LevelsDto,
    pub lufs: LufsDto,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_error: Option<String>,
    /// CAP-N30 program-bus routes that could not open their device.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub output_errors: Vec<OutputRouteErrorDto>,
    /// CAP-N35 live spectrum of the armed source (a parametric-EQ editor open).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spectrum: Option<SpectrumDto>,
    /// Per-filter live meters for the strip whose filter editor is open.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_meters: Option<FilterMetersDto>,
    /// Capture samples dropped across sources (ring overflows).
    pub dropped: u64,
    /// CAP-N47: the LTC reader's latest decode (`HH:MM:SS:FF`), when locked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ltc: Option<String>,
}

fn snapshot_dto(snapshot: EngineSnapshot) -> AudioLevelsDto {
    AudioLevelsDto {
        sources: snapshot
            .sources
            .into_iter()
            .map(|(id, source)| {
                (
                    id.0.to_string(),
                    AudioSourceDto {
                        state: source.state.as_str(),
                        error_code: source.error_code,
                        error_message: source.error_message,
                        peak: source.levels.peak,
                        rms: source.levels.rms,
                        gated: source.gated,
                    },
                )
            })
            .collect(),
        master: LevelsDto {
            peak: snapshot.master.peak,
            rms: snapshot.master.rms,
        },
        lufs: LufsDto {
            momentary: snapshot.lufs_momentary,
            short_term: snapshot.lufs_short_term,
        },
        monitor_error: snapshot.monitor_error,
        output_errors: snapshot
            .output_errors
            .into_iter()
            .map(|err| OutputRouteErrorDto {
                bus: err.bus.key(),
                message: err.message,
            })
            .collect(),
        spectrum: snapshot.spectrum.map(|spectrum| SpectrumDto {
            source: spectrum.source.0.to_string(),
            magnitudes: spectrum.magnitudes,
        }),
        filter_meters: snapshot.filter_meters.map(|fm| FilterMetersDto {
            source: fm.source.0.to_string(),
            meters: fm
                .meters
                .into_iter()
                .map(|meter| FilterMeterDto {
                    id: meter.id.0.to_string(),
                    in_peak: meter.in_peak,
                    out_peak: meter.out_peak,
                })
                .collect(),
        }),
        dropped: snapshot.dropped,
        ltc: snapshot.ltc,
    }
}

/// The engine's source set from the model's audio specs. A push-to-talk /
/// push-to-mute hotkey the OS did **not** register is dropped from the config
/// the engine sees, so the mixer never gates on a key that can never arrive —
/// the source stays audible (fail-open) instead of silent-forever.
fn specs_to_configs(
    specs: &[AudioSourceSpec],
    registered: &HashSet<Shortcut>,
) -> Vec<SourceConfig> {
    let live_key = |key: &Option<String>| -> Option<String> {
        let text = key.as_ref()?;
        match text.parse::<Shortcut>() {
            Ok(shortcut) if registered.contains(&shortcut) => Some(text.clone()),
            _ => None, // unparsable or unregistered → don't gate on it
        }
    };
    specs
        .iter()
        .filter_map(|spec| {
            let input = match &spec.settings {
                SourceSettings::AudioInput { device_id } => InputSpec::Input {
                    device_id: device_id.clone(),
                },
                SourceSettings::AudioOutput { device_id } => InputSpec::Loopback {
                    device_id: device_id.clone(),
                },
                // Hub-ring sources — someone else's thread feeds the ring
                // keyed by the source id and the engine drains it like any
                // capture: Media's decode thread, the Playlist concat decode
                // (CAP-N17), full-speed Replay rolls (CAP-N10; slow-mo is
                // silent by design), the Remote-guest webview's WebRTC push,
                // the LAN-ingest demux (CAP-N11), the Freally Link receiver
                // (CAP-N12), and the WASAPI process-loopback capture
                // (per-app audio; see `reconcile_app_audio`).
                SourceSettings::Media { .. }
                | SourceSettings::Playlist { .. }
                | SourceSettings::ReplayPlayback { .. }
                | SourceSettings::RemoteGuest { .. }
                | SourceSettings::LanIngest { .. }
                | SourceSettings::FreallyLink { .. }
                | SourceSettings::AppAudio { .. } => InputSpec::Media {
                    id: spec.id.0.to_string(),
                },
                // Test signals (CAP-M21): the tone generator runs on this
                // thread (see `reconcile_test_tones`); the flash+beep session
                // thread feeds the ring itself (like Media's decode thread).
                SourceSettings::TestTone {} | SourceSettings::TestFlashBeep { .. } => {
                    InputSpec::Media {
                        id: spec.id.0.to_string(),
                    }
                }
                _ => return None,
            };
            let mut settings = spec.audio.clone();
            settings.push_to_talk = live_key(&settings.push_to_talk);
            settings.push_to_mute = live_key(&settings.push_to_mute);
            Some(SourceConfig {
                id: spec.id,
                input,
                settings,
                nonce: spec.nonce,
            })
        })
        .collect()
}

/// Build the engine source configs for the CAP-N37 pads sounding right now. A
/// pad's id is both its media-hub ring key and its transient source id, so the
/// engine drains exactly the ring the soundboard player feeds.
fn soundboard_configs(active: &[crate::soundboard::ActivePad]) -> Vec<SourceConfig> {
    active
        .iter()
        .filter_map(|pad| {
            let id = SourceId::parse(&pad.id)?;
            Some(SourceConfig {
                id,
                input: InputSpec::Media { id: pad.id.clone() },
                settings: fcap_scene::AudioSettings {
                    volume_db: pad.gain_db,
                    tracks: pad.tracks,
                    ..Default::default()
                },
                nonce: 0,
            })
        })
        .collect()
}

/// Start/stop the per-app (WASAPI process-loopback) captures to match the
/// current `AppAudio` sources. Each running capture pushes stereo f32 @ 48 kHz
/// into the media hub ring keyed by its source id — exactly the ring the engine
/// drains for that source's `InputSpec::Media`. Off-Windows (or on an
/// unsupported build) `start_capture` returns an honest error and no capture
/// runs. Captures live on the audio-bridge thread and are never moved off it.
fn reconcile_app_audio(
    specs: &[AudioSourceSpec],
    running: &mut HashMap<String, (u32, fcap_appaudio::AppCapture)>,
) {
    let mut wanted: HashMap<String, u32> = HashMap::new();
    for spec in specs {
        if let SourceSettings::AppAudio { pid, .. } = &spec.settings {
            wanted.insert(spec.id.0.to_string(), *pid);
        }
    }
    // Drop captures no longer wanted, or whose target pid changed (dropping a
    // ProcessCapture stops the WASAPI stream + joins its thread).
    running.retain(|id, entry| wanted.get(id) == Some(&entry.0));
    // Start captures for newly-present app-audio sources.
    for (id, pid) in wanted {
        if running.contains_key(&id) {
            continue;
        }
        let ring = fcap_audio::media_hub::ring(&id);
        match fcap_appaudio::start_capture(pid, move |samples, _rate, _channels| {
            ring.push(samples);
        }) {
            Ok(capture) => {
                running.insert(id, (pid, capture));
            }
            Err(err) => {
                // Honest, non-fatal: the source stays in the model (and shows
                // its guidance in the UI); it simply produces no audio here.
                eprintln!("fcap: per-app audio capture for pid {pid} unavailable: {err}");
            }
        }
    }
}

/// Start/stop the 1 kHz lineup-tone generators (CAP-M21) to match the current
/// `TestTone` sources. Each generator pushes stereo f32 @ 48 kHz into the
/// media hub ring keyed by its source id — the ring the engine drains for the
/// source's `InputSpec::Media`. Same lifecycle shape as `reconcile_app_audio`;
/// dropping a task stops + joins its thread.
fn reconcile_test_tones(
    specs: &[AudioSourceSpec],
    running: &mut HashMap<String, fcap_sources::testsignal::ToneTask>,
) {
    let wanted: HashSet<String> = specs
        .iter()
        .filter(|spec| matches!(spec.settings, SourceSettings::TestTone {}))
        .map(|spec| spec.id.0.to_string())
        .collect();
    running.retain(|id, _| wanted.contains(id));
    for id in wanted {
        running
            .entry(id)
            .or_insert_with_key(|id| fcap_sources::testsignal::start_tone(id));
    }
}

/// Spawn the reconcile + levels thread. It winds down once the app is gone
/// (emit failures), like the other emitter threads.
pub fn spawn_audio_thread<R: Runtime>(app: AppHandle<R>) {
    thread::Builder::new()
        .name("fcap-audio-bridge".into())
        .spawn(move || {
            let engine = app.state::<AudioRuntime>().engine.clone();
            let mut seen_revision = 0u64;
            let mut seen_monitor: Option<String> = None;
            // CAP-N30 output routes last sent to the engine (change-detected).
            let mut seen_outputs: Vec<fcap_scene::AudioOutputRoute> = Vec::new();
            // CAP-N34 loudness spec last sent (change-detected).
            let mut seen_loudness: Option<crate::settings::LoudnessSettings> = None;
            // CAP-N47 LTC spec last sent (change-detected).
            let mut seen_ltc: Option<crate::settings::LtcSettings> = None;
            // CAP-N37: the scene's engine configs (cached so soundboard pads can
            // be appended without a scene-revision change), the last-seen active
            // pads, and a flag to re-send the combined set when either changes.
            let mut scene_configs: Vec<SourceConfig> = Vec::new();
            let mut seen_soundboard: Vec<crate::soundboard::ActivePad> = Vec::new();
            let mut sources_dirty = false;
            let mut had_sources = false;
            // Running per-app captures, keyed by source id (owned by this thread).
            let mut app_captures: HashMap<String, (u32, fcap_appaudio::AppCapture)> =
                HashMap::new();
            // Running 1 kHz tone generators (CAP-M21), keyed by source id.
            let mut tone_tasks: HashMap<String, fcap_sources::testsignal::ToneTask> =
                HashMap::new();
            // Silence/clipping watch over the master mix (CAP-M10).
            let mut audio_watch = crate::alarms::AudioWatch::default();
            loop {
                // 1. Model → engine (sources + hotkeys), only on change. The
                //    revision is a cheap read; the (cloning) spec fetch happens
                //    only when it actually changed.
                let revision = app.state::<StudioState>().audio_revision();
                if revision != seen_revision {
                    seen_revision = revision;
                    let specs = app.state::<StudioState>().audio_specs().1;
                    // Register hotkeys first so the config reflects which
                    // push-to-talk gates are actually live (fail-open).
                    reconcile_hotkeys(&app, &specs);
                    let registered = app.state::<HotkeyRegistry>().registered();
                    scene_configs = specs_to_configs(&specs, &registered);
                    reconcile_app_audio(&specs, &mut app_captures);
                    reconcile_test_tones(&specs, &mut tone_tasks);
                    sources_dirty = true;
                }

                // 1b. CAP-N37 soundboard: transient pad sources + auto-duck. A
                //     playing pad is drained like Media; the active set changes
                //     as pads start/end, so re-send the combined list on change.
                let active = app.state::<crate::soundboard::SoundboardState>().active();
                if active != seen_soundboard {
                    seen_soundboard = active;
                    sources_dirty = true;
                }
                if sources_dirty {
                    let mut configs = scene_configs.clone();
                    configs.extend(soundboard_configs(&seen_soundboard));
                    engine.set_sources(configs);
                    let duck: Vec<SourceId> = seen_soundboard
                        .iter()
                        .filter(|pad| pad.auto_duck)
                        .filter_map(|pad| SourceId::parse(&pad.id))
                        .collect();
                    engine.set_soundboard_duck(duck);
                    sources_dirty = false;
                }

                // 2. Settings → monitor device, only on change (read just the
                //    one field, not a full Settings clone, every tick).
                let monitor = app
                    .state::<SettingsStore>()
                    .monitor_device()
                    .unwrap_or_default();
                if seen_monitor.as_deref() != Some(monitor.as_str()) {
                    engine.set_monitor_device(monitor.clone());
                    seen_monitor = Some(monitor);
                }

                // 2b. Settings → CAP-N30 program-bus output routes, on change.
                let outputs = app.state::<SettingsStore>().audio_outputs();
                if outputs != seen_outputs {
                    engine.set_audio_outputs(outputs.clone());
                    seen_outputs = outputs;
                }

                // 2c. Settings → CAP-N34 loudness rider, on change.
                let loudness = app.state::<SettingsStore>().loudness();
                if seen_loudness != Some(loudness) {
                    engine.set_loudness(
                        loudness
                            .enabled
                            .then_some((loudness.target_lufs, loudness.ceiling_db)),
                    );
                    seen_loudness = Some(loudness);
                }

                // 2d. Settings → CAP-N47 LTC generator + reader, on change.
                //     The generator jam-syncs to time of day at arm time —
                //     the free-run broadcast practice.
                let ltc = app.state::<SettingsStore>().ltc();
                if seen_ltc.as_ref() != Some(&ltc) {
                    engine.set_ltc(ltc.enabled.then(|| {
                        use chrono::Timelike;
                        let now = chrono::Local::now();
                        let frames = (f64::from(now.nanosecond()) / 1e9 * f64::from(ltc.fps)) as u8;
                        fcap_audio::LtcSpec {
                            track: usize::from(ltc.track),
                            fps: ltc.fps,
                            start: fcap_audio::ltc::LtcTime {
                                hours: now.hour() as u8,
                                minutes: now.minute() as u8,
                                seconds: now.second() as u8,
                                frames: frames.min(ltc.fps as u8 - 1),
                            },
                        }
                    }));
                    engine.set_ltc_read(
                        (!ltc.read_source.is_empty())
                            .then(|| SourceId::parse(&ltc.read_source))
                            .flatten(),
                    );
                    seen_ltc = Some(ltc);
                }

                // 3. Levels out (skip the idle no-sources steady state, but
                //    always send the transition back to empty).
                let snapshot = engine.snapshot();
                let has_sources = !snapshot.sources.is_empty();
                let dto = snapshot_dto(snapshot);
                // CAP-M10: silence/clipping over the master mix, only while
                // the mix actually goes out (live or recording). A panic
                // (CAP-M22) mutes deliberately — no alarm for that.
                let engaged = (app.state::<crate::stream::StreamBridgeState>().is_live()
                    || app.state::<crate::recording::RecordingState>().is_active()
                    || app.state::<crate::audiorec::AudioRecState>().is_active())
                    && !app.state::<StudioState>().is_panicked();
                let peak = dto.master.peak[0].max(dto.master.peak[1]);
                for (kind, active) in audio_watch.evaluate(peak, engaged, std::time::Instant::now())
                {
                    crate::alarms::emit_alarm(&app, kind, active, None);
                }
                if (has_sources || had_sources) && app.emit("audio", &dto).is_err() {
                    return; // the app is gone — wind down
                }
                had_sources = has_sources;

                thread::sleep(AUDIO_TICK);
            }
        })
        .expect("audio bridge thread spawns");
}

#[cfg(test)]
mod tests {
    use fcap_scene::{AudioSettings, SourceId, SourceSettings};

    use super::*;

    fn mic_with_ptt(key: &str) -> AudioSourceSpec {
        AudioSourceSpec {
            id: SourceId::new(),
            settings: SourceSettings::AudioInput {
                device_id: String::new(),
            },
            audio: AudioSettings {
                push_to_talk: Some(key.to_string()),
                ..AudioSettings::default()
            },
            nonce: 0,
        }
    }

    #[test]
    fn unregistered_ptt_is_dropped_so_the_source_stays_audible() {
        // No hotkeys registered (e.g. Wayland, or the accelerator was grabbed):
        // the PTT gate must NOT reach the engine, or the mic is silent forever.
        let spec = mic_with_ptt("F13");
        let configs = specs_to_configs(std::slice::from_ref(&spec), &HashSet::new());
        assert_eq!(configs.len(), 1);
        assert_eq!(
            configs[0].settings.push_to_talk, None,
            "an unregistered PTT must fail open (audible)"
        );
    }

    #[test]
    fn registered_ptt_is_kept() {
        let spec = mic_with_ptt("F13");
        let mut registered = HashSet::new();
        registered.insert("F13".parse::<Shortcut>().expect("valid accelerator"));
        let configs = specs_to_configs(std::slice::from_ref(&spec), &registered);
        assert_eq!(
            configs[0].settings.push_to_talk.as_deref(),
            Some("F13"),
            "a registered PTT gates as configured"
        );
    }

    #[test]
    fn an_unparsable_hotkey_never_gates() {
        let spec = mic_with_ptt("not a real accelerator!!");
        let configs = specs_to_configs(std::slice::from_ref(&spec), &HashSet::new());
        assert_eq!(configs[0].settings.push_to_talk, None);
    }

    #[test]
    fn test_signals_map_to_their_media_rings() {
        // CAP-M21: tone + flash+beep drain the hub ring keyed by the source
        // id — exactly like Media — so their generators reach the mixer.
        for settings in [
            SourceSettings::TestTone {},
            SourceSettings::TestFlashBeep {
                width: 8,
                height: 8,
            },
        ] {
            let spec = AudioSourceSpec {
                id: SourceId::new(),
                settings,
                audio: AudioSettings::default(),
                nonce: 0,
            };
            let configs = specs_to_configs(std::slice::from_ref(&spec), &HashSet::new());
            assert_eq!(
                configs[0].input,
                InputSpec::Media {
                    id: spec.id.0.to_string()
                }
            );
        }
    }

    #[test]
    fn lan_ingest_maps_to_its_media_ring() {
        // CAP-N11: the listener's demux thread feeds the hub ring keyed by
        // the source id — exactly like Media — so the sender's audio gets
        // its own strip.
        let spec = AudioSourceSpec {
            id: SourceId::new(),
            settings: SourceSettings::LanIngest {
                protocol: fcap_scene::IngestProtocol::Srt,
                port: 9710,
                passphrase: String::new(),
            },
            audio: AudioSettings::default(),
            nonce: 0,
        };
        let configs = specs_to_configs(std::slice::from_ref(&spec), &HashSet::new());
        assert_eq!(
            configs[0].input,
            InputSpec::Media {
                id: spec.id.0.to_string()
            }
        );
    }

    #[test]
    fn tone_generators_track_the_spec_set() {
        let spec = AudioSourceSpec {
            id: SourceId::new(),
            settings: SourceSettings::TestTone {},
            audio: AudioSettings::default(),
            nonce: 0,
        };
        let mut running = HashMap::new();
        reconcile_test_tones(std::slice::from_ref(&spec), &mut running);
        assert_eq!(running.len(), 1, "a TestTone source starts its generator");
        let ring = fcap_audio::media_hub::ring(&spec.id.0.to_string());
        let filled = (0..50).any(|_| {
            std::thread::sleep(std::time::Duration::from_millis(10));
            !ring.is_empty()
        });
        assert!(filled, "the generator feeds the source's ring");
        reconcile_test_tones(&[], &mut running);
        assert!(
            running.is_empty(),
            "removing the source stops the generator"
        );
    }
}
