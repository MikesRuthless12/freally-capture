//! Redacted diagnostics bundle (CAP-M24): a user-initiated "Export
//! diagnostics" zip — config snapshot, encoder/device probes, recent stats,
//! recent (already-scrubbed) crash reports — for attaching to a GitHub
//! issue by hand. **Strictly local**: nothing is sent anywhere (charter),
//! the exact content is previewable before export, and every section is
//! built by an **allowlist** — fields are copied out by name, never "the
//! settings minus the secrets". Stream keys, tokens, passwords, URLs, file
//! paths and user text (source/scene names, dock urls, script paths) are
//! simply never read. `bugreport::scrub` runs over the result as a second
//! layer for the home path + username.

use std::io::Write;
use std::path::PathBuf;

use tauri::{AppHandle, Manager, Runtime};

use crate::settings::Settings;

/// One file inside the bundle.
pub struct BundleFile {
    pub name: &'static str,
    pub content: String,
}

/// The allowlist settings snapshot. Every line here names the exact field
/// it prints; anything not listed does not exist as far as the bundle is
/// concerned. Paths print as `<set>`/`<default>` — never their value.
fn settings_snapshot(settings: &Settings) -> String {
    let set_or_default = |value: &str| {
        if value.trim().is_empty() {
            "<default>"
        } else {
            "<set>"
        }
    };
    let bound_or_not = |value: &Option<String>| match value {
        Some(text) if !text.trim().is_empty() => "<bound>",
        _ => "<unbound>",
    };
    let recording = &settings.recording;
    let mut out = String::new();
    let mut line = |text: String| {
        out.push_str(&text);
        out.push('\n');
    };
    line(format!("language: {}", settings.language));
    line(format!("theme: {:?}", settings.theme.mode));
    line(format!("mixerLayout: {:?}", settings.mixer_layout));
    line(format!(
        "alignment: guides={} safeAreas={} rulers={}",
        settings.alignment.smart_guides, settings.alignment.safe_areas, settings.alignment.rulers
    ));
    line(format!(
        "recording: container={} encoder={} rc={:?}/{} cq={} preset={:?} fps={} keyframe={}s audioKbps={} tracksMask={:#04b} split={}min vertical={} scale={}x{}",
        recording.container.extension(),
        recording.encoder_id,
        recording.rate_control.mode,
        recording.rate_control.bitrate_kbps,
        recording.rate_control.cq,
        recording.preset,
        recording.fps,
        recording.keyframe_sec,
        recording.audio_bitrate_kbps,
        recording.tracks_mask,
        recording.split_minutes,
        recording.record_vertical,
        recording.output_width,
        recording.output_height,
    ));
    line(format!(
        "recording.folders: recordings={} replays={} stills={}",
        set_or_default(&recording.folder),
        set_or_default(&recording.replay_folder),
        set_or_default(&recording.still_folder),
    ));
    // Templates are user text (could carry a name) — presence only.
    line(format!(
        "recording.templates: recording={} replay={} still={} counter={}",
        set_or_default(&recording.template),
        set_or_default(&recording.replay_template),
        set_or_default(&recording.still_template),
        recording.counter,
    ));
    for (index, target) in settings.stream.targets.iter().enumerate() {
        // Service + numbers ONLY. The key and the ingest URL (which can
        // embed the key) are never read.
        line(format!(
            "stream.target[{index}]: service={} enabled={} kbps={} audioKbps={} fps={} keyframe={}s track={} canvas={:?} key={}",
            target.service.label(),
            target.enabled,
            target.bitrate_kbps,
            target.audio_bitrate_kbps,
            target.fps,
            target.keyframe_sec,
            target.track,
            target.canvas,
            if target.stream_key.trim().is_empty() { "<absent>" } else { "<present>" },
        ));
    }
    line(format!(
        "stream.autoRecord: {}",
        settings.stream.auto_record
    ));
    line(format!(
        "replay: seconds={} kbps={}",
        settings.replay.seconds, settings.replay.bitrate_kbps
    ));
    line(format!(
        "transition: kind={:?} durationMs={} lumaImage={} stinger={}",
        settings.transition.kind,
        settings.transition.duration_ms,
        set_or_default(&settings.transition.luma_image),
        set_or_default(&settings.transition.stinger_path),
    ));
    line(format!(
        "hotkeys: record={} goLive={} transition={} saveReplay={} addMarker={} still={} panic={}",
        bound_or_not(&settings.hotkeys.record),
        bound_or_not(&settings.hotkeys.go_live),
        bound_or_not(&settings.hotkeys.transition),
        bound_or_not(&settings.hotkeys.save_replay),
        bound_or_not(&settings.hotkeys.add_marker),
        bound_or_not(&settings.hotkeys.still),
        bound_or_not(&settings.hotkeys.panic),
    ));
    line(format!(
        "panicSlate: color={} image={}",
        settings.panic_slate.color,
        set_or_default(&settings.panic_slate.image),
    ));
    line(format!(
        "remote (TURN): url={} credentials={}",
        set_or_default(&settings.remote.turn_url),
        if settings.remote.turn_credential.trim().is_empty() {
            "<absent>"
        } else {
            "<present>"
        },
    ));
    line(format!(
        "remoteControl: enabled={} port={} lan={} password={}",
        settings.remote_control.enabled,
        settings.remote_control.port,
        settings.remote_control.lan,
        if settings.remote_control.password.trim().is_empty() {
            "<absent>"
        } else {
            "<present>"
        },
    ));
    line(format!("browserDocks: {}", settings.browser_docks.len()));
    line(format!(
        "scripts: {} ({} enabled)",
        settings.scripts.len(),
        settings.scripts.iter().filter(|s| s.enabled).count()
    ));
    out
}

/// The encoder/device probe section (GPU models + the encoder catalog).
fn catalog_snapshot(catalog: &fcap_encode::Catalog) -> String {
    let mut out = String::new();
    for gpu in &catalog.gpus {
        out.push_str(&format!("gpu: {} ({:?})\n", gpu.name, gpu.vendor));
    }
    for encoder in &catalog.encoders {
        out.push_str(&format!(
            "encoder: {} [{}] {:?} verified={:?}\n",
            encoder.id, encoder.label, encoder.engine, encoder.verified
        ));
    }
    out
}

/// Build the whole bundle. Everything user-influencable passes through
/// [`crate::bugreport::scrub`] as the second redaction layer.
pub fn build_bundle<R: Runtime>(app: &AppHandle<R>) -> Vec<BundleFile> {
    let mut files = Vec::new();

    let mut report = crate::bugreport::diagnostics();
    let (running, fps, vertical_fps, dropped, render_micros) =
        app.state::<crate::events::RuntimeStats>().latest();
    report.push_str(&format!(
        "compositor: running={running} fps={fps} verticalFps={vertical_fps} dropped={dropped} renderMicros={render_micros}\n"
    ));
    // Source kinds + states only — names are user text and never included.
    let sources = app
        .state::<crate::studio::StudioState>()
        .with_collection(|collection| {
            let mut lines = String::new();
            lines.push_str(&format!(
                "canvas: {}x{}  scenes: {}  sources: {}\n",
                collection.canvas_width,
                collection.canvas_height,
                collection.scenes.len(),
                collection.sources.len()
            ));
            for source in &collection.sources {
                lines.push_str(&format!("source: {}\n", source.settings.kind_name()));
            }
            lines
        });
    report.push_str(&sources);
    files.push(BundleFile {
        name: "report.txt",
        content: crate::bugreport::scrub(&report),
    });

    let settings = app.state::<crate::settings::SettingsStore>().get();
    files.push(BundleFile {
        name: "settings.txt",
        content: crate::bugreport::scrub(&settings_snapshot(&settings)),
    });

    if let Ok(catalog) = crate::commands::recording::ensure_catalog(app) {
        files.push(BundleFile {
            name: "encoders.txt",
            content: crate::bugreport::scrub(&catalog_snapshot(&catalog)),
        });
    }

    // The app writes no persistent log file; the newest crash report
    // (already scrubbed when written) is the closest thing to logs.
    files.push(BundleFile {
        name: "logs.txt",
        content: match crate::bugreport::pending_crash() {
            Some(crash) => {
                // Bounded by CHARS (a byte cut can split UTF-8) + rescrubbed.
                let bounded: String = crash.chars().take(16_384).collect();
                crate::bugreport::scrub(&bounded)
            }
            None => "Freally Capture writes no persistent log file. No crash reports on disk.\n"
                .to_owned(),
        },
    });

    files
}

/// The preview — the EXACT text the zip will contain, headed per file.
#[tauri::command]
pub fn diagnostics_preview<R: Runtime>(app: AppHandle<R>) -> String {
    build_bundle(&app)
        .iter()
        .map(|file| format!("===== {} =====\n{}", file.name, file.content))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Zip the bundle into the Downloads folder (home as fallback) and return
/// the path. Nothing is sent anywhere — the user attaches it by hand.
#[tauri::command]
pub async fn diagnostics_export<R: Runtime>(app: AppHandle<R>) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let files = build_bundle(&app);
        let dir = directories::UserDirs::new()
            .and_then(|dirs| {
                dirs.download_dir()
                    .map(PathBuf::from)
                    .or_else(|| Some(dirs.home_dir().to_path_buf()))
            })
            .ok_or("no folder to write the bundle into")?;
        let stamp = chrono::Local::now().format("%Y-%m-%d %H-%M-%S");
        let path = dir.join(format!("freally-capture-diagnostics {stamp}.zip"));

        let file = std::fs::File::create(&path)
            .map_err(|err| format!("could not create {}: {err}", path.display()))?;
        let mut zip = zip::ZipWriter::new(file);
        let options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);
        for entry in &files {
            zip.start_file(entry.name, options)
                .map_err(|err| format!("zip: {err}"))?;
            zip.write_all(entry.content.as_bytes())
                .map_err(|err| format!("zip: {err}"))?;
        }
        zip.finish().map_err(|err| format!("zip: {err}"))?;
        println!("diagnostics: exported {}", path.display());
        Ok(path.display().to_string())
    })
    .await
    .map_err(|err| format!("diagnostics task failed: {err}"))?
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::{
        BrowserDockSettings, PanicSlateSettings, RemoteControlSettings, RemoteSettings,
        ScriptSettings, StreamSettings, StreamTargetSettings,
    };

    /// THE security test: plant a secret in every sensitive field and prove
    /// none of them survive into the snapshot.
    #[test]
    fn no_secret_survives_the_allowlist() {
        let settings = Settings {
            stream: StreamSettings {
                targets: vec![StreamTargetSettings {
                    service: fcap_stream::StreamService::Twitch,
                    stream_key: "live_SECRETKEY_123".to_owned(),
                    ingest_url: "rtmp://in.example/app/live_SECRETKEY_123".to_owned(),
                    ..StreamTargetSettings::default()
                }],
                ..StreamSettings::default()
            },
            remote: RemoteSettings {
                turn_url: "turns:relay.example.net:5349".to_owned(),
                turn_username: "turnuser77".to_owned(),
                turn_credential: "turnpass77".to_owned(),
            },
            remote_control: RemoteControlSettings {
                enabled: true,
                port: 4455,
                lan: true,
                password: "deckpass99".to_owned(),
            },
            browser_docks: vec![BrowserDockSettings {
                name: "Chat".to_owned(),
                url: "https://chat.example/?token=dockTOKEN".to_owned(),
            }],
            scripts: vec![ScriptSettings {
                path: "C:/Users/someone/private/go.lua".to_owned(),
                enabled: true,
            }],
            panic_slate: PanicSlateSettings {
                color: "#10141a".to_owned(),
                image: "C:/Users/someone/private/brb.png".to_owned(),
            },
            ..Settings::default()
        };
        let mut settings = settings;
        settings.recording.folder = "D:/Streams/someone-private".to_owned();
        settings.recording.template = "{prefix} by someone-private {date}".to_owned();
        settings.hotkeys.panic = Some("Ctrl+Shift+F12".to_owned());

        let snapshot = settings_snapshot(&settings);
        for secret in [
            "SECRETKEY",
            "rtmp://",
            "turnuser77",
            "turnpass77",
            "deckpass99",
            "dockTOKEN",
            "chat.example",
            "go.lua",
            "brb.png",
            "someone-private",
            "Ctrl+Shift+F12", // even accelerators print as <bound>, not text
        ] {
            assert!(
                !snapshot.contains(secret),
                "the snapshot leaked {secret:?}:\n{snapshot}"
            );
        }
        // …while the harmless facts DO land.
        assert!(snapshot.contains("service=Twitch"));
        assert!(snapshot.contains("key=<present>"));
        assert!(snapshot.contains("credentials=<present>"));
        assert!(snapshot.contains("password=<present>"));
        assert!(snapshot.contains("browserDocks: 1"));
        assert!(snapshot.contains("scripts: 1 (1 enabled)"));
        assert!(snapshot.contains("panic=<bound>"));
    }

    #[test]
    fn defaults_read_as_defaults_not_values() {
        let snapshot = settings_snapshot(&Settings::default());
        assert!(snapshot.contains("recordings=<default>"));
        // A default target row (if any) still never prints a key value.
        assert!(!snapshot.contains("key=<present>"));
        assert!(snapshot.contains("browserDocks: 0"));
    }
}
