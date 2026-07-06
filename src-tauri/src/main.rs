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

mod audio;
mod commands;
mod events;
mod native_preview;
mod preview;
mod recording;
mod remote;
mod settings;
mod studio;

use audio::{AudioRuntime, HotkeyRegistry};
use preview::PreviewState;
use settings::SettingsStore;
use studio::StudioState;
use tauri::Manager;

fn main() {
    // Surface wgpu's `log` diagnostics when RUST_LOG is set (silent otherwise),
    // and loudly print any panic — main *or* worker thread — to stdout before
    // the (`panic = "abort"`) exit, so an init failure lands in the logs instead
    // of vanishing. The hook still chains the default (backtrace) behaviour.
    env_logger::init();
    let default_panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        println!("PANIC: {info}");
        default_panic_hook(info);
    }));

    // Version banner on launch (visible in dev consoles and CI logs; the
    // release Windows build has no console by design).
    println!(
        "Freally Capture v{} — local-first streaming & recording studio (© 2026 Mike Weaver)",
        env!("CARGO_PKG_VERSION")
    );

    let settings = SettingsStore::load_default();
    println!("settings: language={}", settings.get().language);
    println!("init: building the Tauri app (creates the webview, then runs setup)...");

    let app = tauri::Builder::default()
        // PTT/PTM global shortcuts (the full hotkey map lands in Phase 5).
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    audio::on_hotkey(app, shortcut, event.state());
                })
                .build(),
        )
        .manage(settings)
        .manage(PreviewState::default())
        .manage(StudioState::load_default())
        .manage(AudioRuntime::new())
        .manage(HotkeyRegistry::default())
        .manage(commands::recording::EncodeState::new())
        .manage(recording::RecordingState::new())
        .manage(native_preview::NativePreviewState::new())
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
            commands::capture_window_thumbnail,
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
            commands::studio::studio_retry_source,
            commands::studio::studio_add_filter,
            commands::studio::studio_remove_filter,
            commands::studio::studio_reorder_filter,
            commands::studio::studio_update_filter,
            commands::studio::studio_set_filter_enabled,
            remote::remote_guest_push_frame,
            commands::audio::audio_input_devices,
            commands::audio::audio_output_devices,
            commands::audio::audio_loopback_devices,
            commands::audio::studio_set_audio_volume,
            commands::audio::studio_set_audio_muted,
            commands::audio::studio_set_audio_monitor,
            commands::audio::studio_set_audio_tracks,
            commands::audio::studio_set_audio_sync_offset,
            commands::audio::studio_set_audio_hotkeys,
            commands::audio::studio_add_audio_filter,
            commands::audio::studio_remove_audio_filter,
            commands::audio::studio_reorder_audio_filter,
            commands::audio::studio_update_audio_filter,
            commands::audio::studio_set_audio_filter_enabled,
            commands::recording::encoders_list,
            commands::recording::ffmpeg_status,
            commands::recording::ffmpeg_install,
            commands::recording::ffmpeg_cancel,
            commands::recording::ffmpeg_remove,
            commands::recording::recording_start,
            commands::recording::recording_stop,
            commands::recording::recording_pause,
            commands::recording::recording_resume,
            commands::recording::recording_status,
            commands::recording::recordings_list,
            commands::recording::recording_remux,
            commands::native_preview_set_region,
            commands::native_preview_active,
            commands::native_preview_set_selection
        ])
        .setup(|app| {
            println!("init: setup entered");
            events::spawn_stats_emitter(app.handle().clone());
            // The compose loop: capture sessions + static sources → the
            // compositor → the program frame behind `preview://`.
            studio::spawn_studio_thread(
                app.handle().clone(),
                &app.state::<StudioState>(),
            );
            println!("init: studio thread spawned");
            // The audio bridge: model → engine reconcile + the `audio`
            // levels event + PTT/PTM hotkey registration.
            audio::spawn_audio_thread(app.handle().clone());
            // The recording status emitter (~2 Hz while a session runs) +
            // the dead-sink watchdog.
            recording::spawn_status_thread(app.handle().clone());
            println!("init: bridges spawned — calling native_preview::try_create");
            // The native preview child window (Windows). Created here on the
            // main thread; the studio thread presents the GPU surface onto it.
            // Failure (or any non-Windows OS) leaves it absent → the JPEG
            // `preview://` path stays in charge.
            native_preview::try_create(app.handle());
            // Test-only (env `FCAP_SMOKE`, set by the screenshot-smoke CI): seed
            // a magenta scene + force the preview region so the headless
            // screenshot actually exercises the native GPU surface. No-op
            // otherwise — see `studio::seed_smoke_scene`.
            if std::env::var_os("FCAP_SMOKE").is_some() {
                studio::seed_smoke_scene(app.handle());
            }
            println!("init: setup complete");
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building Freally Capture");
    println!("init: build() returned — entering the event loop");

    app.run(|app_handle, event| {
        if let tauri::RunEvent::Exit = event {
            // Never lose the last edit: the autosave debounce may still be
            // pending when the user quits.
            app_handle.state::<StudioState>().save_now();
        }
    });
}
