//! The app half of the WebSocket remote-control API (Phase 7, TASK-701):
//! the command allowlist a controller (Stream Deck / Companion-style) may
//! drive, and the manager that keeps the `fcap-stream` server in sync with
//! Settings → Remote Control.
//!
//! Security shape: the server exists only while enabled in settings (off =
//! port closed), binds loopback unless LAN is explicitly on, and every
//! command below is a **fixed allowlist** that dispatches into the same
//! studio functions the UI and hotkeys use — no command takes a file path,
//! so the API cannot read or write arbitrary files by construction.

use std::sync::Mutex;
use std::time::Duration;

use serde_json::{json, Value};
use tauri::{AppHandle, Manager};

use fcap_scene::{FilterId, ItemId, SceneId, SourceId};
use fcap_stream::remote::{RemoteHandler, RemoteServer};

use crate::settings::{RemoteControlSettings, SettingsStore};
use crate::studio::StudioState;

/// Managed state: the running server (when enabled) + change detection.
#[derive(Default)]
pub struct RemoteApiState {
    server: Mutex<Option<RemoteServer>>,
    seen: Mutex<Option<RemoteControlSettings>>,
    last_pushed: Mutex<Option<Value>>,
}

fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// The `RemoteHandler` the server calls on a client's request.
struct AppHandler {
    app: AppHandle,
}

impl RemoteHandler for AppHandler {
    fn handle(&self, command: &str, params: &Value) -> Result<Value, String> {
        dispatch(&self.app, command, params)
    }
}

/// The fixed command allowlist. Everything routes through the same
/// functions the UI/hotkeys use, so validation and side effects stay
/// identical no matter who asks.
pub(crate) fn dispatch(app: &AppHandle, command: &str, params: &Value) -> Result<Value, String> {
    match command {
        "getStatus" => Ok(status_value(app)),
        "listScenes" => Ok(scenes_value(app)),
        "setProgramScene" => {
            let scene = resolve_scene(app, params)?;
            crate::commands::studio::studio_select_scene(app.clone(), app.state(), scene)?;
            Ok(Value::Null)
        }
        "setPreviewScene" => {
            let scene = resolve_scene(app, params)?;
            crate::commands::studio::studio_set_preview_scene(app.clone(), app.state(), scene)?;
            Ok(Value::Null)
        }
        "setStudioMode" => {
            let on = p_bool(params, "on")?;
            crate::commands::studio::studio_set_studio_mode(app.clone(), app.state(), on)?;
            Ok(Value::Null)
        }
        "transition" => {
            crate::commands::studio::studio_transition(app.clone(), app.state(), app.state())?;
            Ok(Value::Null)
        }
        "startStream" => crate::stream::start(app).map(|()| Value::Null),
        "stopStream" => {
            let dto = crate::stream::stop(app)?;
            Ok(serde_json::to_value(dto).unwrap_or(Value::Null))
        }
        "startRecording" => crate::recording::start(app).map(|()| Value::Null),
        "stopRecording" => {
            let paths = crate::recording::stop(app)?;
            Ok(json!({ "paths": paths }))
        }
        "pauseRecording" => {
            let paused = p_bool(params, "paused")?;
            crate::recording::set_paused(app, paused).map(|()| Value::Null)
        }
        "addMarker" => {
            let count = crate::recording::add_marker(app)?;
            Ok(json!({ "markers": count }))
        }
        "armReplay" => {
            if p_bool(params, "armed")? {
                crate::replay::arm(app).map(|()| Value::Null)
            } else {
                crate::replay::disarm(app).map(|()| Value::Null)
            }
        }
        "saveReplay" => {
            let path = crate::replay::save(app)?;
            Ok(json!({ "path": path.display().to_string() }))
        }
        "setAudioMuted" => {
            let source: SourceId = p_id(params, "sourceId")?;
            let muted = p_bool(params, "muted")?;
            crate::commands::audio::studio_set_audio_muted(
                app.clone(),
                app.state(),
                source,
                muted,
            )?;
            Ok(Value::Null)
        }
        "setAudioVolume" => {
            let source: SourceId = p_id(params, "sourceId")?;
            let volume = params
                .get("volumeDb")
                .and_then(Value::as_f64)
                .ok_or("missing number param: volumeDb")? as f32;
            crate::commands::audio::studio_set_audio_volume(
                app.clone(),
                app.state(),
                source,
                volume,
            )?;
            Ok(Value::Null)
        }
        "setFilterEnabled" => {
            let scene = resolve_scene(app, params)?;
            let item: ItemId = p_id(params, "itemId")?;
            let filter: FilterId = p_id(params, "filterId")?;
            let enabled = p_bool(params, "enabled")?;
            crate::commands::studio::studio_set_filter_enabled(
                app.clone(),
                app.state(),
                scene,
                item,
                filter,
                enabled,
            )?;
            Ok(Value::Null)
        }
        other => Err(format!("unknown command: {other}")),
    }
}

// -- params helpers -----------------------------------------------------------

fn p_bool(params: &Value, key: &str) -> Result<bool, String> {
    params
        .get(key)
        .and_then(Value::as_bool)
        .ok_or_else(|| format!("missing boolean param: {key}"))
}

fn p_id<T: serde::de::DeserializeOwned>(params: &Value, key: &str) -> Result<T, String> {
    let value = params
        .get(key)
        .cloned()
        .ok_or_else(|| format!("missing param: {key}"))?;
    serde_json::from_value(value).map_err(|err| format!("bad {key}: {err}"))
}

/// `params.scene` is a scene **id or exact name** — controllers usually key
/// by name, the UI by id.
fn resolve_scene(app: &AppHandle, params: &Value) -> Result<SceneId, String> {
    let raw = params
        .get("scene")
        .cloned()
        .ok_or("missing param: scene (a scene id or exact name)")?;
    if let Ok(id) = serde_json::from_value::<SceneId>(raw.clone()) {
        let snapshot = app.state::<StudioState>().snapshot();
        if snapshot
            .collection
            .scenes
            .iter()
            .any(|scene| scene.id == id)
        {
            return Ok(id);
        }
    }
    let name = raw.as_str().ok_or("param scene must be a string")?;
    let snapshot = app.state::<StudioState>().snapshot();
    snapshot
        .collection
        .scenes
        .iter()
        .find(|scene| scene.name == name)
        .map(|scene| scene.id)
        .ok_or_else(|| format!("unknown scene: {name}"))
}

// -- status + events ----------------------------------------------------------

/// The full status a controller polls on connect.
fn status_value(app: &AppHandle) -> Value {
    let stream = crate::stream::stream_status(app.state());
    let recording = app.state::<crate::recording::RecordingState>().status();
    let replay = app.state::<crate::replay::ReplayState>().status();
    json!({
        "version": env!("CARGO_PKG_VERSION"),
        "scenes": scenes_value(app),
        "stream": serde_json::to_value(&stream).unwrap_or(Value::Null),
        "recording": serde_json::to_value(&recording).unwrap_or(Value::Null),
        "replay": serde_json::to_value(&replay).unwrap_or(Value::Null),
    })
}

fn scenes_value(app: &AppHandle) -> Value {
    let snapshot = app.state::<StudioState>().snapshot();
    let preview = snapshot.studio_mode.as_ref().map(|mode| mode.preview_scene);
    let scenes: Vec<Value> = snapshot
        .collection
        .scenes
        .iter()
        .map(|scene| {
            json!({
                "id": scene.id,
                "name": scene.name,
                "program": scene.id == snapshot.collection.active_scene,
                "preview": Some(scene.id) == preview,
            })
        })
        .collect();
    json!({ "scenes": scenes, "studioMode": snapshot.studio_mode.is_some() })
}

/// The compact snapshot pushed as the `state` event whenever it changes.
pub(crate) fn coarse_state(app: &AppHandle) -> Value {
    let snapshot = app.state::<StudioState>().snapshot();
    let stream = crate::stream::stream_status(app.state());
    let recording = match app.state::<crate::recording::RecordingState>().status() {
        crate::recording::RecordingDto::Idle { .. } => "idle",
        crate::recording::RecordingDto::Recording { .. } => "recording",
        crate::recording::RecordingDto::Paused { .. } => "paused",
        crate::recording::RecordingDto::Finalizing { .. } => "finalizing",
    };
    json!({
        "stream": stream.state,
        "recording": recording,
        "replayArmed": app.state::<crate::replay::ReplayState>().status().armed,
        "programScene": snapshot.collection.active_scene,
        "previewScene": snapshot.studio_mode.as_ref().map(|mode| mode.preview_scene),
        "studioMode": snapshot.studio_mode.is_some(),
    })
}

/// Keep the server matched to settings (start/stop/reconfigure) and push the
/// `state` event on change. One thread, ~4 Hz — same pattern as the hotkey
/// reconcile thread.
pub fn spawn_manager(app: AppHandle) {
    std::thread::Builder::new()
        .name("fcap-remote-api".into())
        .spawn(move || loop {
            let settings = app.state::<SettingsStore>().get().remote_control;
            let state = app.state::<RemoteApiState>();

            if lock(&state.seen).as_ref() != Some(&settings) {
                // Any change tears the old server down (port closed) before a
                // new one starts with the fresh port/bind/password.
                if let Some(server) = lock(&state.server).take() {
                    server.stop();
                    println!("remote api: stopped (settings changed)");
                }
                if settings.enabled && settings.validate().is_ok() {
                    let handler = std::sync::Arc::new(AppHandler { app: app.clone() });
                    match RemoteServer::start(
                        settings.port,
                        settings.lan,
                        &settings.password,
                        handler,
                    ) {
                        Ok(server) => {
                            println!(
                                "remote api: listening on {}:{} (password-protected)",
                                if server.lan() { "0.0.0.0" } else { "127.0.0.1" },
                                server.port()
                            );
                            *lock(&state.server) = Some(server);
                        }
                        Err(err) => eprintln!("remote api: could not start: {err}"),
                    }
                }
                *lock(&state.seen) = Some(settings);
                *lock(&state.last_pushed) = None;
            }

            // Push the coarse state on change (server present + anyone may
            // be listening — publish drops dead clients itself).
            let running = lock(&state.server).is_some();
            if running {
                let snapshot = coarse_state(&app);
                let mut last = lock(&state.last_pushed);
                if last.as_ref() != Some(&snapshot) {
                    if let Some(server) = lock(&state.server).as_ref() {
                        server.publish_event("state", snapshot.clone());
                    }
                    *last = Some(snapshot);
                }
            }

            std::thread::sleep(Duration::from_millis(250));
        })
        .expect("remote-api manager thread spawns");
}
