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

mod alarms;
mod audio;
mod autoconfig;
mod bugreport;
mod buildinfo;
mod commands;
mod diagnostics;
mod docks;
mod eula;
mod events;
mod filename;
mod hotkeys;
mod native_preview;
mod openfile;
mod preview;
mod profiles;
mod projector;
mod reactions;
mod recording;
mod remote;
mod remote_api;
mod replay;
mod salvage;
mod scripting;
mod settings;
mod shutdown;
mod stream;
mod studio;

use audio::{AudioRuntime, HotkeyRegistry};
use preview::PreviewState;
use settings::SettingsStore;
use studio::StudioState;
use tauri::{Emitter, Manager};

fn main() {
    // `--crash-notice <pid>`: we are the tiny helper a dying studio spawned, not
    // the studio. Show the native error window, relaunch if the user says yes,
    // and leave. Must come before everything else — the helper never builds a
    // Tauri app, so it never trips the single-instance guard.
    let args: Vec<String> = std::env::args().collect();
    if bugreport::run_crash_notice(&args) {
        return;
    }
    // `--test-crash`: drill the crash loop on the shipped exe. No UI, no command.
    bugreport::arm_test_crash(&args);

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
    // Opt-in bug reporting: capture a SCRUBBED crash report locally so the
    // next launch can offer to report it. Nothing is ever sent automatically
    // (charter: no telemetry) — this only writes a local file. Chains the
    // hook above (write → print → default backtrace).
    bugreport::install_panic_hook();

    // Version banner on launch (visible in dev consoles and CI logs; the
    // release Windows build has no console by design).
    println!(
        "Freally Capture v{} — local-first streaming & recording studio (© 2026 Mike Weaver)",
        env!("CARGO_PKG_VERSION")
    );

    let settings = SettingsStore::load_default();
    println!("settings: language={}", settings.get().language);
    println!("init: building the Tauri app (creates the webview, then runs setup)...");

    // Single instance is normally ON: a second launch — e.g. a clicked
    // freally:// invite link — focuses the running app and forwards the link.
    // Setting FCAP_ALLOW_MULTI lets several windows coexist on one machine so
    // a host + guest remote-session drill can run locally (single instance
    // otherwise collapses the second launch into the first). Test-only escape
    // hatch; the shipped default is single instance.
    let allow_multi = std::env::var_os("FCAP_ALLOW_MULTI").is_some();
    if allow_multi {
        println!("init: FCAP_ALLOW_MULTI set — single-instance guard DISABLED (local test mode)");
    }

    let mut builder = tauri::Builder::default();
    if !allow_multi {
        // Single instance FIRST (the plugins' documented order).
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, argv, _cwd| {
            use tauri::{Emitter, Manager};
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_focus();
            }
            // A second launch opening a .frec (double-click while running):
            // forward the path so the UI offers to export it.
            if let Some(frec) = openfile::frec_in_args(argv) {
                openfile::store(frec.clone());
                let _ = app.emit("open-frec", frec);
            }
        }));
    }
    let app = builder
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        // PTT/PTM global shortcuts (the full hotkey map lands in Phase 5).
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    // Global action hotkeys (record / go live / transition)
                    // claim their shortcut first; the mixer's PTT/PTM handles
                    // the rest.
                    if !hotkeys::dispatch(app, shortcut, event.state()) {
                        audio::on_hotkey(app, shortcut, event.state());
                    }
                })
                .build(),
        )
        .manage(settings)
        .manage(PreviewState::default())
        .manage(StudioState::load_default())
        .manage(AudioRuntime::new())
        .manage(HotkeyRegistry::default())
        .manage(commands::recording::EncodeState::new())
        .manage(commands::cef::CefState::new())
        .manage(commands::recording::ExportState::default())
        .manage(recording::RecordingState::new())
        // CAP-M23: drop this session's crash marker, remembering whether the
        // previous one exited uncleanly (CAP-M11's salvage signal).
        .manage(shutdown::QuitState::new(shutdown::mark_session_start()))
        .manage(salvage::SalvageState::default())
        .manage(stream::StreamBridgeState::new())
        .manage(replay::ReplayState::new())
        .manage(reactions::ReactionState::new())
        .manage(events::RuntimeStats::default())
        .manage(hotkeys::ActionHotkeys::default())
        .manage(profiles::WorkspaceState::load_default())
        .manage(native_preview::NativePreviewState::new())
        .manage(remote_api::RemoteApiState::default())
        // The program-frame pipe: the UI polls `preview://` for the newest
        // composed JPEG. In-process only — frames never touch a socket or
        // disk — and CORS-pinned to the app's own origins.
        .register_uri_scheme_protocol("preview", |ctx, request| {
            let origin = request
                .headers()
                .get("origin")
                .and_then(|value| value.to_str().ok());
            let path = request.uri().path().to_string();
            ctx.app_handle()
                .state::<PreviewState>()
                .protocol_response(origin, &path)
        })
        .on_window_event(|window, event| match event {
            // Closing the main studio window quits the app — and does it while the
            // projector windows are still alive, so `ExitRequested` can remember
            // them (on the default last-window-closes path they would be destroyed
            // first, and the session snapshot would be empty). CAP-M07 extension.
            // CAP-M23: quitting while live/recording/replay-armed asks first
            // (`quit-guard` → the webview confirm), and every exit goes through
            // the ordered shutdown (a fast no-op when nothing is running).
            tauri::WindowEvent::CloseRequested { api, .. } if window.label() == "main" => {
                api.prevent_close();
                let app = window.app_handle();
                let quit = app.state::<shutdown::QuitState>();
                let pending = shutdown::consequences(app);
                // Proceed straight to the ordered shutdown when nothing
                // needs guarding, when one is already underway, or when a
                // prompt is already up unanswered — a hung/dead webview
                // must never make the app unclosable: the SECOND close
                // click is the confirmation.
                if !pending.any() || quit.is_quitting() || quit.prompt_armed() {
                    shutdown::shutdown_and_exit(app.clone());
                } else {
                    quit.arm_prompt();
                    if app.emit("quit-guard", &pending).is_err() {
                        shutdown::shutdown_and_exit(app.clone());
                    }
                }
            }
            // When a scene/source projector window closes (its own Esc, or the OS),
            // tell the render loop to stop rendering its slot (CAP-M07 extension).
            tauri::WindowEvent::Destroyed => {
                if let Some(target) = projector::parse_target(window.label()) {
                    window
                        .app_handle()
                        .state::<StudioState>()
                        .set_projector(target, false);
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            commands::health,
            shutdown::quit_confirmed,
            shutdown::quit_guard_cancel,
            salvage::salvage_pending,
            salvage::salvage_repair,
            salvage::salvage_dismiss,
            commands::integrations_status,
            commands::game_capture_status,
            eula::eula_status,
            eula::eula_accept,
            commands::settings_get,
            commands::settings_set,
            commands::capture_list_sources,
            commands::capture_window_thumbnail,
            commands::video_devices_list,
            commands::video_device_formats,
            commands::open_privacy_settings,
            docks::browser_dock_open,
            projector::list_displays,
            projector::aux_window_open,
            projector::aux_window_close,
            autoconfig::autoconfig_suggest,
            commands::settings_complete_onboarding,
            buildinfo::build_info,
            bugreport::bug_report_context,
            bugreport::bug_report_submit,
            bugreport::bug_report_clear_crash,
            diagnostics::diagnostics_preview,
            diagnostics::diagnostics_export,
            alarms::preflight_disk,
            commands::studio::studio_get,
            commands::studio::studio_undo,
            commands::studio::studio_redo,
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
            commands::studio::studio_apply_layout,
            commands::studio::studio_set_item_slot,
            commands::studio::studio_set_center_view,
            commands::studio::studio_set_focus,
            commands::studio::studio_create_group,
            commands::studio::studio_ungroup,
            commands::studio::studio_set_group_visible,
            commands::studio::studio_set_scene_audio_override,
            commands::studio::studio_set_vertical,
            commands::studio::studio_set_studio_mode,
            commands::studio::studio_set_preview_scene,
            commands::studio::studio_transition,
            commands::studio::studio_rename_source,
            commands::studio::studio_update_source_settings,
            commands::studio::studio_retry_source,
            commands::studio::studio_panic_set,
            commands::studio::studio_media_set_paused,
            commands::studio::studio_media_paused,
            commands::studio::studio_add_filter,
            commands::studio::studio_remove_filter,
            commands::studio::studio_reorder_filter,
            commands::studio::studio_update_filter,
            commands::studio::studio_set_filter_enabled,
            commands::studio::studio_paste_filters,
            commands::studio::studio_workbench_set,
            commands::studio::studio_workbench_close,
            commands::studio::studio_multiview_set,
            commands::studio::studio_capture_still,
            commands::studio::collection_missing_files,
            commands::studio::collection_relink,
            commands::studio::collection_relink_folder,
            commands::studio::studio_set_item_transforms,
            commands::studio::studio_set_guides,
            remote::remote_guest_push_frame,
            remote::remote_guest_push_audio,
            remote::remote_pending_invite,
            stream::stream_start,
            stream::stream_stop,
            stream::stream_status,
            replay::replay_arm,
            replay::replay_disarm,
            replay::replay_save,
            replay::replay_status,
            reactions::studio_send_reaction,
            profiles::profiles_list,
            profiles::profile_create,
            profiles::profile_switch,
            profiles::collections_list,
            profiles::collection_create,
            profiles::collection_switch,
            profiles::collection_import_obs,
            commands::audio::audio_input_devices,
            commands::audio::audio_output_devices,
            commands::audio::app_audio_apps,
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
            commands::cef::cef_status,
            commands::cef::cef_install,
            commands::cef::cef_cancel,
            commands::cef::cef_remove,
            commands::recording::ffmpeg_cancel,
            commands::recording::ffmpeg_remove,
            commands::recording::recording_start,
            commands::recording::recording_stop,
            commands::recording::recording_add_marker,
            commands::recording::recording_pause,
            commands::recording::recording_resume,
            commands::recording::recording_status,
            commands::recording::recordings_list,
            commands::recording::recording_remux,
            commands::recording::recording_export,
            commands::recording::recording_export_cancel,
            commands::recording::open_frec_export,
            openfile::open_frec_pending,
            commands::native_preview_set_region,
            commands::native_preview_active,
            commands::native_preview_set_selection,
            commands::native_preview_set_overlay
        ])
        .setup(|app| {
            println!("init: setup entered");
            // TASK-R2: freally://join?token=… — forward opened links to the
            // webview, which shows a JOIN PROMPT (nothing auto-connects; the
            // URL is untrusted input parsed by the invite validator there).
            {
                use tauri::Emitter;
                use tauri_plugin_deep_link::DeepLinkExt;
                let handle = app.handle().clone();
                app.deep_link().on_open_url(move |event| {
                    for url in event.urls() {
                        // Running app: the webview listener handles it live.
                        // Also stash it — a cold-start URL fires this before
                        // the webview exists, and the UI takes the stash via
                        // `remote_pending_invite` once it loads.
                        remote::store_pending_invite(url.to_string());
                        let _ = handle.emit("remote-invite", url.to_string());
                    }
                });
                // The launch-args URL (app opened BY a link click) was
                // consumed by the plugin during init — before the handler
                // above existed. Pick it up explicitly.
                if let Ok(Some(urls)) = app.deep_link().get_current() {
                    if let Some(url) = urls.first() {
                        remote::store_pending_invite(url.to_string());
                    }
                }
                // Installers register the scheme; dev/portable runs register
                // best-effort at launch (Windows/Linux support runtime
                // registration; macOS registers via the bundle only).
                #[cfg(any(windows, target_os = "linux"))]
                {
                    let _ = app.deep_link().register_all();
                }
            }
            // Opened with a .frec on the command line (cold start / OS
            // double-click): stash it for the UI to offer an export.
            if let Some(frec) = openfile::frec_in_args(std::env::args()) {
                openfile::store(frec);
            }
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
            // The stream's ~1 Hz status/elapsed events (Phase 5).
            stream::spawn_status_thread(app.handle().clone());
            replay::spawn_status_thread(app.handle().clone());
            // Global action hotkeys: record / go live / transition (Phase 5).
            hotkeys::spawn_reconcile_thread(app.handle().clone());
            // The WebSocket remote-control API (Phase 7) — off by default;
            // the manager starts/stops the server as settings change.
            remote_api::spawn_manager(app.handle().clone());
            // Sandboxed Lua scripts (Phase 7) — loaded from settings, same
            // command allowlist as the remote API.
            scripting::spawn_manager(app.handle().clone());
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
            // Reopen the projectors that were open last session (CAP-M07
            // extension) — stale scene/source targets are skipped.
            projector::reopen_saved(app.handle());
            // CAP-M11: the in-progress sidecar is consumed every launch (so
            // stale ones never linger). Its presence IS the signal — stop()
            // clears it only when every output finalized — so the salvage
            // prompt fires regardless of the CAP-M23 marker: a session whose
            // finalize failed followed by a clean app exit still deserves
            // its repair offer. The marker just annotates the log.
            let interrupted = salvage::take_interrupted();
            let found = salvage::candidates(&interrupted);
            if !found.is_empty() {
                println!(
                    "salvage: {} interrupted recording(s) found (previous exit unclean: {})",
                    found.len(),
                    app.handle()
                        .state::<shutdown::QuitState>()
                        .previous_exit_was_unclean()
                );
                app.handle()
                    .state::<salvage::SalvageState>()
                    .set_pending(found);
            }
            println!("init: setup complete");
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building Freally Capture");
    println!("init: build() returned — entering the event loop");

    app.run(|app_handle, event| match event {
        // While the windows are still alive, remember which projectors are open
        // so they reopen next launch (CAP-M07 extension).
        tauri::RunEvent::ExitRequested { .. } => projector::remember_open(app_handle),
        tauri::RunEvent::Exit => {
            // Never lose the last edit: the autosave debounce may still be
            // pending when the user quits.
            app_handle.state::<StudioState>().save_now();
            // CAP-M23: the exit was orderly only if nothing was left running —
            // then (and only then) the crash marker comes off.
            shutdown::mark_clean_if_quiescent(app_handle);
        }
        _ => {}
    });
}
