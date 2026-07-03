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
    /// Capture samples dropped across sources (ring overflows).
    pub dropped: u64,
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
        dropped: snapshot.dropped,
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
                // Media audio: the decode thread feeds the hub ring keyed
                // by the source id; the engine drains it like any capture.
                SourceSettings::Media { .. } => InputSpec::Media {
                    id: spec.id.0.to_string(),
                },
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

/// Spawn the reconcile + levels thread. It winds down once the app is gone
/// (emit failures), like the other emitter threads.
pub fn spawn_audio_thread<R: Runtime>(app: AppHandle<R>) {
    thread::Builder::new()
        .name("fcap-audio-bridge".into())
        .spawn(move || {
            let engine = app.state::<AudioRuntime>().engine.clone();
            let mut seen_revision = 0u64;
            let mut seen_monitor: Option<String> = None;
            let mut had_sources = false;
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
                    engine.set_sources(specs_to_configs(&specs, &registered));
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

                // 3. Levels out (skip the idle no-sources steady state, but
                //    always send the transition back to empty).
                let snapshot = engine.snapshot();
                let has_sources = !snapshot.sources.is_empty();
                if (has_sources || had_sources)
                    && app.emit("audio", &snapshot_dto(snapshot)).is_err()
                {
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
}
