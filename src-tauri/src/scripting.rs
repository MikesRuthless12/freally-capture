//! The app half of Lua scripting (Phase 7, TASK-703): the manager that loads
//! the enabled scripts from settings, derives studio events from state
//! changes, and delivers them — every script call routes through the same
//! `remote_api::dispatch` allowlist the WebSocket API serves, so a script
//! can do exactly what a controller can, nothing more.
//!
//! Scripts (and their `mlua::Lua` states, which are not `Send`) live entirely
//! on the one scripting thread: loaded there, events emitted there.

use std::time::Duration;

use serde_json::{json, Value};
use tauri::AppHandle;
use tauri::Manager;

use fcap_script::{CommandFn, LogFn, Script};

use crate::settings::{ScriptSettings, SettingsStore};

/// Scripts larger than this are refused (a studio script is small).
const MAX_SCRIPT_BYTES: u64 = 256 * 1024;

/// The events derived from one coarse-state change (name, data).
fn derive_events(prev: &Value, next: &Value) -> Vec<(String, Value)> {
    let mut events = Vec::new();
    let (prev_stream, next_stream) = (prev["stream"].as_str(), next["stream"].as_str());
    if prev_stream != next_stream {
        if next_stream == Some("live") {
            events.push(("streamStarted".to_owned(), Value::Null));
        } else if prev_stream == Some("live") {
            events.push(("streamEnded".to_owned(), Value::Null));
        }
    }
    let was_recording = matches!(prev["recording"].as_str(), Some("recording" | "paused"));
    let is_recording = matches!(next["recording"].as_str(), Some("recording" | "paused"));
    if !was_recording && is_recording {
        events.push(("recordingStarted".to_owned(), Value::Null));
    } else if was_recording && !is_recording {
        events.push(("recordingStopped".to_owned(), Value::Null));
    }
    if prev["programScene"] != next["programScene"] {
        events.push((
            "sceneChanged".to_owned(),
            json!({ "scene": next["programScene"] }),
        ));
    }
    events
}

/// Load one enabled script file (bounded, honest errors).
fn load_script(app: &AppHandle, entry: &ScriptSettings) -> Result<Script, String> {
    let meta = std::fs::metadata(&entry.path).map_err(|err| format!("{}: {err}", entry.path))?;
    if meta.len() > MAX_SCRIPT_BYTES {
        return Err(format!("{}: script larger than 256 KB", entry.path));
    }
    let source =
        std::fs::read_to_string(&entry.path).map_err(|err| format!("{}: {err}", entry.path))?;
    let dispatch_app = app.clone();
    let command: CommandFn = std::sync::Arc::new(move |name, params| {
        crate::remote_api::dispatch(&dispatch_app, name, params)
    });
    let name = std::path::Path::new(&entry.path)
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| entry.path.clone());
    let log: LogFn = std::sync::Arc::new(move |line| println!("script[{name}]: {line}"));
    Script::load(&source, command, log)
}

/// The scripting thread: reconcile loaded scripts against settings (~1 s)
/// and deliver derived events on every coarse-state change (~4 Hz).
pub fn spawn_manager(app: AppHandle) {
    std::thread::Builder::new()
        .name("fcap-scripting".into())
        .spawn(move || {
            let mut seen: Option<Vec<ScriptSettings>> = None;
            let mut loaded: Vec<Script> = Vec::new();
            let mut last_state: Option<Value> = None;
            loop {
                let entries = app.state::<SettingsStore>().get().scripts;
                if seen.as_ref() != Some(&entries) {
                    loaded.clear();
                    for entry in entries.iter().filter(|entry| entry.enabled) {
                        match load_script(&app, entry) {
                            Ok(script) => {
                                println!("script loaded: {}", entry.path);
                                loaded.push(script);
                            }
                            Err(err) => eprintln!("script failed to load: {err}"),
                        }
                    }
                    seen = Some(entries);
                }

                if !loaded.is_empty() {
                    let state = crate::remote_api::coarse_state(&app);
                    if let Some(prev) = &last_state {
                        if prev != &state {
                            let mut events = derive_events(prev, &state);
                            events.push(("state".to_owned(), state.clone()));
                            for (event, data) in &events {
                                for script in &loaded {
                                    if let Err(err) = script.emit(event, data) {
                                        eprintln!("script error: {err}");
                                    }
                                }
                            }
                        }
                    }
                    last_state = Some(state);
                }

                std::thread::sleep(Duration::from_millis(250));
            }
        })
        .expect("scripting thread spawns");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state(stream: &str, recording: &str, scene: &str) -> Value {
        json!({ "stream": stream, "recording": recording, "programScene": scene })
    }

    #[test]
    fn transitions_derive_the_right_events() {
        let idle = state("idle", "idle", "a");
        let live = state("live", "idle", "a");
        let events = derive_events(&idle, &live);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].0, "streamStarted");

        let ended = derive_events(&live, &idle);
        assert_eq!(ended[0].0, "streamEnded");

        let recording = state("idle", "recording", "a");
        assert_eq!(derive_events(&idle, &recording)[0].0, "recordingStarted");
        // Pause is NOT a stop — no event for recording→paused.
        let paused = state("idle", "paused", "a");
        assert!(derive_events(&recording, &paused).is_empty());
        assert_eq!(derive_events(&paused, &idle)[0].0, "recordingStopped");

        let scene_b = state("idle", "idle", "b");
        let scene_events = derive_events(&idle, &scene_b);
        assert_eq!(scene_events[0].0, "sceneChanged");
        assert_eq!(scene_events[0].1["scene"], "b");

        assert!(derive_events(&idle, &idle).is_empty());
    }
}
