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

use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

use crate::settings::{HotkeySettings, SettingsStore};

/// What a global hotkey triggers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotkeyAction {
    RecordToggle,
    GoLiveToggle,
    Transition,
    SaveReplay,
    AddMarker,
}

/// Live accelerator → action bindings + the OS-registered set. Managed state.
#[derive(Default)]
pub struct ActionHotkeys {
    bindings: Mutex<HashMap<Shortcut, HotkeyAction>>,
    registered: Mutex<HashSet<Shortcut>>,
    /// Fingerprint of the last-reconciled settings, so the poll is cheap.
    seen: Mutex<Option<HotkeySettings>>,
}

fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
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
        .copied()
    else {
        return true;
    };
    // Off the event-loop thread: record/stream stop do blocking I/O (faststart
    // remux, RTMP flush) that must never freeze the window.
    let handle = app.clone();
    std::thread::spawn(move || run_action(&handle, action));
    true
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
    }
}

/// Reconcile the OS-registered action hotkeys against the settings. Cheap
/// no-op when the settings haven't changed since last call.
fn reconcile<R: Runtime>(app: &AppHandle<R>, settings: &HotkeySettings) {
    let state = app.state::<ActionHotkeys>();
    if lock(&state.seen).as_ref() == Some(settings) {
        return;
    }
    *lock(&state.seen) = Some(settings.clone());

    let mut desired: HashMap<Shortcut, HotkeyAction> = HashMap::new();
    for (key, action) in [
        (&settings.record, HotkeyAction::RecordToggle),
        (&settings.go_live, HotkeyAction::GoLiveToggle),
        (&settings.transition, HotkeyAction::Transition),
        (&settings.save_replay, HotkeyAction::SaveReplay),
        (&settings.add_marker, HotkeyAction::AddMarker),
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
            let settings = app.state::<SettingsStore>().get().hotkeys;
            reconcile(&app, &settings);
            std::thread::sleep(Duration::from_secs(1));
        })
        .expect("action-hotkey thread spawns");
}
