// The shipped Windows .exe is a GUI app with NO console window (debug builds
// keep the console for logs). Never regress this — see the release checklist.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![forbid(unsafe_code)]

//! Freally Capture — the Tauri v2 app shell.
//!
//! Local-first, cross-platform live-streaming & recording studio. This crate
//! hosts the window shell, the UI ↔ core command/event bridge, and the
//! settings store; the engine lives in the owned `fcap-*` workspace crates.

mod commands;
mod events;
mod preview;
mod settings;

use preview::PreviewState;
use settings::SettingsStore;
use tauri::Manager;

fn main() {
    // Version banner on launch (visible in dev consoles and CI logs; the
    // release Windows build has no console by design).
    println!(
        "Freally Capture v{} — local-first streaming & recording studio (© 2026 Mike Weaver)",
        env!("CARGO_PKG_VERSION")
    );

    let settings = SettingsStore::load_default();
    println!("settings: language={}", settings.get().language);

    tauri::Builder::default()
        .manage(settings)
        .manage(PreviewState::default())
        // The preview frame pipe: the UI polls `preview://` for the newest
        // JPEG. In-process only — frames never touch a socket or disk — and
        // CORS-pinned to the app's own origins.
        .register_uri_scheme_protocol("preview", |ctx, request| {
            let origin = request
                .headers()
                .get("origin")
                .and_then(|value| value.to_str().ok());
            ctx.app_handle()
                .state::<PreviewState>()
                .protocol_response(origin)
        })
        .invoke_handler(tauri::generate_handler![
            commands::health,
            commands::settings_get,
            commands::settings_set,
            commands::capture_list_sources,
            commands::video_devices_list,
            commands::video_device_formats,
            commands::preview_start,
            commands::preview_stop,
            commands::open_privacy_settings
        ])
        .setup(|app| {
            events::spawn_stats_emitter(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Freally Capture");
}
