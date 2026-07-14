//! Global action hotkeys (Phase 5): Record, Go Live, and the Studio-Mode
//! Transition, bound to OS-global accelerators. This rides the same
//! `tauri-plugin-global-shortcut` handler the mixer's push-to-talk uses — the
//! one `on_hotkey` callback checks this registry too — but keeps its own
//! desired-set + reconcile so the two never collide.
//!
//! Registration must run on the main thread (macOS) and can honestly fail
//! (Wayland has no global hotkeys); a failure is logged and the action simply
//! stays available from the UI.

use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use std::time::Duration;

use tauri::{AppHandle, Emitter, Manager, Runtime};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

use crate::settings::{HotkeySettings, SettingsStore};

/// What a global hotkey triggers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HotkeyAction {
    /// Run a named macro (CAP-N02) — macros carry their own accelerator, so
    /// this needs no new `HotkeySettings` field.
    RunMacro(String),
    /// The FIRST stroke of a chord (CAP-N05): arms the chord and registers
    /// its followers for a short window.
    ChordLeader(String),
    /// The SECOND stroke of a chord: runs `macro_name` when `leader` is the
    /// armed leader (else it is ignored and the chord disarms).
    ChordFollower {
        leader: String,
        macro_name: String,
    },
    RecordToggle,
    GoLiveToggle,
    Transition,
    SaveReplay,
    AddMarker,
    CaptureStill,
    /// Cut to the privacy slate + hard-mute (CAP-M22). Engage only — the
    /// restore is the deliberate two-step in the UI.
    Panic,
    /// Start/pause EVERY timer source at once (CAP-M15) — a global key
    /// can't name a source; the settings row says so.
    TimerToggle,
    /// Reset every timer source (CAP-M15).
    TimerReset,
    /// Punch-in zoom presets (CAP-N71): the UI resolves which capture the
    /// lens targets (the selected item, else the top-most screen view), so
    /// these just broadcast the requested factor.
    Zoom100,
    Zoom150,
    Zoom200,
}

/// Live accelerator → action bindings + the OS-registered set. Managed state.
#[derive(Default)]
pub struct ActionHotkeys {
    bindings: Mutex<HashMap<Shortcut, HotkeyAction>>,
    registered: Mutex<HashSet<Shortcut>>,
    /// Fingerprint of the last-reconciled settings, so the poll is cheap.
    seen: Mutex<Option<HotkeySettings>>,
    /// Same, for the macros' own accelerators (CAP-N02).
    seen_macros: Mutex<Option<Vec<MacroBinding>>>,
    /// CAP-N05: chord followers, `shortcut -> (leader, macro name)`. These
    /// are registered ONLY while their leader is armed.
    followers: Mutex<HashMap<Shortcut, (String, String)>>,
    /// CAP-N05: the follower shortcuts THIS arming actually registered (a
    /// follower that was already a permanent binding is left alone). Only
    /// these are torn down when the chord window closes — so a plain hotkey
    /// that happens to share a follower's key is never unregistered.
    armed_followers: Mutex<HashSet<Shortcut>>,
    /// CAP-N05: each macro's layer (`None` = fires on every layer).
    macro_layers: Mutex<HashMap<String, Option<u8>>>,
}

/// One macro's binding: `(name, accelerator, layer)` (CAP-N02 / CAP-N05).
pub type MacroBinding = (String, Option<String>, Option<u8>);

fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

impl ActionHotkeys {
    /// A snapshot of the action shortcuts the OS actually accepted — the
    /// CAP-M14 audit's "registered" signal.
    pub fn registered(&self) -> HashSet<Shortcut> {
        lock(&self.registered).clone()
    }
}

/// Route a global-shortcut event to its action (fires on press). Returns
/// whether the shortcut is one WE registered for an action — gating on the
/// OS-**registered** set (not merely the desired bindings) so a key the mixer
/// owns for push-to-talk (our action registration would have failed) still
/// reaches `audio::on_hotkey`, and its Release is never swallowed.
pub fn dispatch<R: Runtime>(app: &AppHandle<R>, shortcut: &Shortcut, state: ShortcutState) -> bool {
    let is_action = lock(&app.state::<ActionHotkeys>().registered).contains(shortcut);
    if !is_action {
        return false; // not ours — let the mixer's PTT/PTM handler see it
    }
    if state != ShortcutState::Pressed {
        return true; // claimed; actions fire on press only
    }
    let Some(action) = lock(&app.state::<ActionHotkeys>().bindings)
        .get(shortcut)
        .cloned()
    else {
        return true;
    };
    // CAP-N05: chords and layers gate the action before it runs.
    match &action {
        HotkeyAction::ChordLeader(leader) => {
            arm_chord(app, leader);
            return true;
        }
        HotkeyAction::ChordFollower { leader, macro_name } => {
            let armed = {
                let state = app.state::<std::sync::Mutex<crate::chords::ChordState>>();
                let mut chords = lock(&state);
                let armed = chords.armed_leader(std::time::Instant::now());
                chords.disarm();
                armed
            };
            release_followers(app);
            if armed.as_deref() == Some(leader.as_str()) {
                let handle = app.clone();
                let name = macro_name.clone();
                std::thread::spawn(move || crate::automation::run_macro_by_name(&handle, &name));
            }
            return true;
        }
        _ => {}
    }
    // A layered macro only fires on its own layer (unlayered keys always do).
    if let HotkeyAction::RunMacro(name) = &action {
        let layer = lock(&app.state::<ActionHotkeys>().macro_layers)
            .get(name)
            .copied()
            .flatten();
        let state = app.state::<std::sync::Mutex<crate::chords::ChordState>>();
        if !lock(&state).fires_on_layer(layer) {
            return true; // claimed, but this layer doesn't own it
        }
    }
    // Off the event-loop thread: record/stream stop do blocking I/O (faststart
    // remux, RTMP flush) that must never freeze the window.
    let handle = app.clone();
    std::thread::spawn(move || run_action(&handle, action));
    true
}

/// CAP-N05: arm a chord — register its followers for the chord window, so a
/// bare `3` is only ever claimed while a chord is actually pending.
fn arm_chord<R: Runtime>(app: &AppHandle<R>, leader: &str) {
    {
        let state = app.state::<std::sync::Mutex<crate::chords::ChordState>>();
        lock(&state).arm(leader, std::time::Instant::now());
    }
    let followers: Vec<Shortcut> = lock(&app.state::<ActionHotkeys>().followers)
        .iter()
        .filter(|(_, (owner, _))| owner == leader)
        .map(|(shortcut, _)| *shortcut)
        .collect();
    if followers.is_empty() {
        return;
    }
    let handle = app.clone();
    let _ = app.run_on_main_thread(move || {
        let shortcuts = handle.global_shortcut();
        let state = handle.state::<ActionHotkeys>();
        let mut bindings = lock(&state.bindings);
        let mut registered = lock(&state.registered);
        let table = lock(&state.followers).clone();
        let mut armed = lock(&state.armed_followers);
        for shortcut in followers {
            // A follower key that is ALREADY registered belongs to a permanent
            // binding (e.g. a plain hotkey on the same key) — leave it, and do
            // not track it for teardown, so releasing the chord never kills it.
            if registered.contains(&shortcut) {
                continue;
            }
            match shortcuts.register(shortcut) {
                Ok(()) => {
                    registered.insert(shortcut);
                    armed.insert(shortcut);
                    if let Some((leader, macro_name)) = table.get(&shortcut) {
                        bindings.insert(
                            shortcut,
                            HotkeyAction::ChordFollower {
                                leader: leader.clone(),
                                macro_name: macro_name.clone(),
                            },
                        );
                    }
                }
                Err(err) => eprintln!("hotkey: could not arm a chord follower: {err}"),
            }
        }
    });
    // The window closes on its own even if the operator never finishes the
    // chord — a half-typed chord must not leave a bare key claimed.
    let handle = app.clone();
    std::thread::spawn(move || {
        std::thread::sleep(crate::chords::CHORD_WINDOW);
        let state = handle.state::<std::sync::Mutex<crate::chords::ChordState>>();
        let expired = { !lock(&state).is_armed(std::time::Instant::now()) };
        if expired {
            release_followers(&handle);
        }
    });
}

/// Unregister the chord followers THIS session armed (the window closed, or
/// one matched). Only the shortcuts `arm_chord` actually registered are torn
/// down — a follower key that was already a permanent binding is untouched.
fn release_followers<R: Runtime>(app: &AppHandle<R>) {
    let handle = app.clone();
    let _ = app.run_on_main_thread(move || {
        let shortcuts = handle.global_shortcut();
        let state = handle.state::<ActionHotkeys>();
        let armed: Vec<Shortcut> = lock(&state.armed_followers).drain().collect();
        let mut registered = lock(&state.registered);
        let mut bindings = lock(&state.bindings);
        for shortcut in armed {
            if registered.remove(&shortcut) {
                let _ = shortcuts.unregister(shortcut);
            }
            bindings.remove(&shortcut);
        }
    });
}

fn run_action<R: Runtime>(app: &AppHandle<R>, action: HotkeyAction) {
    match action {
        HotkeyAction::RecordToggle => {
            let running = app
                .state::<crate::recording::RecordingState>()
                .wants_frames();
            let result = if running {
                crate::recording::stop(app).map(|_| ())
            } else {
                crate::recording::start(app)
            };
            if let Err(err) = result {
                eprintln!("hotkey: record toggle failed: {err}");
            }
        }
        HotkeyAction::GoLiveToggle => {
            let live = app
                .state::<crate::stream::StreamBridgeState>()
                .wants_frames();
            let result = if live {
                crate::stream::stop(app).map(|_| ())
            } else {
                crate::stream::start(app)
            };
            if let Err(err) = result {
                eprintln!("hotkey: go-live toggle failed: {err}");
            }
        }
        HotkeyAction::Transition => {
            let settings = app.state::<SettingsStore>().get().transition;
            if let Err(err) = app
                .state::<crate::studio::StudioState>()
                .begin_transition(app, &settings)
            {
                eprintln!("hotkey: transition failed: {err}");
            }
        }
        HotkeyAction::SaveReplay => {
            if let Err(err) = crate::replay::save(app) {
                eprintln!("hotkey: replay save failed: {err}");
            }
        }
        HotkeyAction::AddMarker => {
            if let Err(err) = crate::recording::add_marker(app) {
                eprintln!("hotkey: marker failed: {err}");
            }
        }
        HotkeyAction::CaptureStill => {
            crate::studio::capture_still(app, crate::studio::StillTarget::Program);
        }
        HotkeyAction::Panic => {
            app.state::<crate::studio::StudioState>()
                .set_panic(app, true);
        }
        HotkeyAction::TimerToggle => {
            app.state::<crate::studio::StudioState>()
                .timer_control_all(crate::studio::TimerCmd::Toggle);
        }
        HotkeyAction::TimerReset => {
            app.state::<crate::studio::StudioState>()
                .timer_control_all(crate::studio::TimerCmd::Reset);
        }
        HotkeyAction::Zoom100 | HotkeyAction::Zoom150 | HotkeyAction::Zoom200 => {
            let factor = match action {
                HotkeyAction::Zoom150 => 1.5f32,
                HotkeyAction::Zoom200 => 2.0,
                _ => 1.0,
            };
            if let Err(err) = app.emit("zoom-preset", factor) {
                eprintln!("hotkey: zoom preset event failed: {err}");
            }
        }
        HotkeyAction::RunMacro(name) => {
            crate::automation::run_macro_by_name(app, &name);
        }
        // Chord strokes are fully handled in `dispatch` (they never reach the
        // worker thread) — listed here so the match stays exhaustive.
        HotkeyAction::ChordLeader(_) | HotkeyAction::ChordFollower { .. } => {}
    }
}

/// Reconcile the OS-registered action hotkeys against the settings. Cheap
/// no-op when the settings haven't changed since last call.
fn reconcile<R: Runtime>(app: &AppHandle<R>, settings: &HotkeySettings, macros: &[MacroBinding]) {
    let state = app.state::<ActionHotkeys>();
    if lock(&state.seen).as_ref() == Some(settings)
        && lock(&state.seen_macros).as_deref() == Some(macros)
    {
        return;
    }
    *lock(&state.seen) = Some(settings.clone());
    *lock(&state.seen_macros) = Some(macros.to_vec());
    let macros = macros.to_vec();

    let mut desired: HashMap<Shortcut, HotkeyAction> = HashMap::new();
    for (key, action) in [
        (&settings.record, HotkeyAction::RecordToggle),
        (&settings.go_live, HotkeyAction::GoLiveToggle),
        (&settings.transition, HotkeyAction::Transition),
        (&settings.save_replay, HotkeyAction::SaveReplay),
        (&settings.add_marker, HotkeyAction::AddMarker),
        (&settings.still, HotkeyAction::CaptureStill),
        (&settings.panic, HotkeyAction::Panic),
        (&settings.timer_toggle, HotkeyAction::TimerToggle),
        (&settings.timer_reset, HotkeyAction::TimerReset),
        (&settings.zoom_100, HotkeyAction::Zoom100),
        (&settings.zoom_150, HotkeyAction::Zoom150),
        (&settings.zoom_200, HotkeyAction::Zoom200),
    ] {
        let Some(text) = key.as_ref().filter(|text| !text.trim().is_empty()) else {
            continue;
        };
        match text.parse::<Shortcut>() {
            Ok(shortcut) => {
                desired.entry(shortcut).or_insert(action);
            }
            Err(err) => eprintln!("hotkey: unusable accelerator {text:?}: {err}"),
        }
    }
    // Macros carry their own accelerators (CAP-N02) — plain or a chord
    // (CAP-N05). Only a chord's LEADER is registered at rest; its follower
    // goes into the follower table and is registered on demand.
    let mut followers: HashMap<Shortcut, (String, String)> = HashMap::new();
    let mut layers: HashMap<String, Option<u8>> = HashMap::new();
    for (name, accelerator, layer) in &macros {
        layers.insert(name.clone(), *layer);
        let Some(text) = accelerator.as_ref().filter(|text| !text.trim().is_empty()) else {
            continue;
        };
        if crate::chords::is_chord(text) {
            let Some(chord) = crate::chords::parse_chord(text) else {
                eprintln!("hotkey: unusable chord {text:?}");
                continue;
            };
            match (
                chord.leader.parse::<Shortcut>(),
                chord.follower.parse::<Shortcut>(),
            ) {
                (Ok(leader), Ok(follower)) => {
                    desired
                        .entry(leader)
                        .or_insert(HotkeyAction::ChordLeader(chord.leader.clone()));
                    followers.insert(follower, (chord.leader.clone(), name.clone()));
                }
                _ => eprintln!("hotkey: unusable chord {text:?}"),
            }
            continue;
        }
        match text.parse::<Shortcut>() {
            Ok(shortcut) => {
                desired
                    .entry(shortcut)
                    .or_insert(HotkeyAction::RunMacro(name.clone()));
            }
            Err(err) => eprintln!("hotkey: unusable macro accelerator {text:?}: {err}"),
        }
    }
    *lock(&state.followers) = followers;
    *lock(&state.macro_layers) = layers;

    let registered_now = lock(&state.registered).clone();
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
    *lock(&state.bindings) = desired;

    if to_register.is_empty() && to_unregister.is_empty() {
        return;
    }
    let handle = app.clone();
    let dispatched = app.run_on_main_thread(move || {
        let shortcuts = handle.global_shortcut();
        let state = handle.state::<ActionHotkeys>();
        let mut registered = lock(&state.registered);
        for shortcut in to_unregister {
            if let Err(err) = shortcuts.unregister(shortcut) {
                eprintln!("hotkey: could not unregister an action hotkey: {err}");
            }
            registered.remove(&shortcut);
        }
        for shortcut in to_register {
            match shortcuts.register(shortcut) {
                Ok(()) => {
                    registered.insert(shortcut);
                }
                Err(err) => eprintln!(
                    "hotkey: could not register an action hotkey ({err}) — global \
                     hotkeys may be unavailable here (e.g. Wayland); the action \
                     still works from the UI"
                ),
            }
        }
    });
    if let Err(err) = dispatched {
        eprintln!("hotkey: action registration could not reach the main thread: {err}");
    }
}

/// Poll settings ~1 Hz and keep the action hotkeys registered. Winds down
/// when the app is gone.
pub fn spawn_reconcile_thread<R: Runtime>(app: AppHandle<R>) {
    std::thread::Builder::new()
        .name("fcap-action-hotkeys".into())
        .spawn(move || loop {
            let settings = app.state::<SettingsStore>().get();
            let macros: Vec<MacroBinding> = settings
                .automation
                .macros
                .iter()
                .map(|entry| (entry.name.clone(), entry.hotkey.clone(), entry.layer))
                .collect();
            reconcile(&app, &settings.hotkeys, &macros);
            std::thread::sleep(Duration::from_secs(1));
        })
        .expect("action-hotkey thread spawns");
}
