// The shipped Windows .exe is a GUI app with NO console window (debug builds
// keep the console for logs). Never regress this — see the release checklist.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![forbid(unsafe_code)]

//! Freally Capture — the Tauri v2 app shell.
//!
//! Local-first, cross-platform live-streaming & recording studio. This crate
//! hosts the window shell, the UI ↔ core command/event bridge, the settings
//! store, and (Phase 2) the studio runtime — the scene collection + the
//! 60 fps compose loop; the engine lives in the owned `fcap-*` crates.

mod commands;
mod events;
mod preview;
mod settings;
mod studio;

use preview::PreviewState;
use settings::SettingsStore;
use studio::StudioState;
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

    let app = tauri::Builder::default()
        .manage(settings)
        .manage(PreviewState::default())
        .manage(StudioState::load_default())
        // The program-frame pipe: the UI polls `preview://` for the newest
        // composed JPEG. In-process only — frames never touch a socket or
        // disk — and CORS-pinned to the app's own origins.
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
            commands::open_privacy_settings,
            commands::studio::studio_get,
            commands::studio::studio_add_scene,
            commands::studio::studio_rename_scene,
            commands::studio::studio_remove_scene,
            commands::studio::studio_select_scene,
            commands::studio::studio_reorder_scene,
            commands::studio::studio_add_item,
            commands::studio::studio_add_existing_source,
            commands::studio::studio_remove_item,
            commands::studio::studio_reorder_item,
            commands::studio::studio_set_item_transform,
            commands::studio::studio_set_item_visible,
            commands::studio::studio_set_item_locked,
            commands::studio::studio_set_item_blend,
            commands::studio::studio_rename_source,
            commands::studio::studio_update_source_settings,
            commands::studio::studio_add_filter,
            commands::studio::studio_remove_filter,
            commands::studio::studio_reorder_filter,
            commands::studio::studio_update_filter,
            commands::studio::studio_set_filter_enabled
        ])
        .setup(|app| {
            events::spawn_stats_emitter(app.handle().clone());
            // The compose loop: capture sessions + static sources → the
            // compositor → the program frame behind `preview://`.
            studio::spawn_studio_thread(
                app.handle().clone(),
                &app.state::<StudioState>(),
            );
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building Freally Capture");

    app.run(|app_handle, event| {
        if let tauri::RunEvent::Exit = event {
            // Never lose the last edit: the autosave debounce may still be
            // pending when the user quits.
            app_handle.state::<StudioState>().save_now();
        }
    });
}
