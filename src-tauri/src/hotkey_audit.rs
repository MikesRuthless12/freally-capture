//! CAP-M14 — **Hotkey manager depth**: the audit behind the searchable
//! hotkey table. The existing SettingsHotkeys dialog *binds* keys; this
//! module *audits and documents* them — every binding in one list (global
//! actions + per-source push-to-talk/push-to-mute), with honest conflict
//! signals:
//!
//! - `shared_with` — how many OTHER bindings sit on the same canonical key
//!   (case/format-insensitive: `ctrl+shift+r` == `Ctrl+Shift+R`). A shared
//!   PTT can be deliberate (talk on two mics); an action key shared with a
//!   PTT starves the PTT (actions claim the press first) — the table shows
//!   the share and the operator decides.
//! - `registered` — whether the OS actually accepted the registration. A
//!   desired key that is NOT registered is owned by something else (another
//!   app's global hotkey) or unavailable here (e.g. Wayland).
//! - `valid` — whether the text parses as an accelerator at all.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use serde::Serialize;
use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_global_shortcut::Shortcut;

/// One audited binding, as the table shows it.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HotkeyAuditEntry {
    /// The accelerator exactly as configured.
    pub accelerator: String,
    /// Machine tag: `record`/`goLive`/`transition`/`saveReplay`/`addMarker`/
    /// `still`/`panic`/`timerToggle`/`timerReset`/`pushToTalk`/`pushToMute`.
    pub action: String,
    /// The filter group tag (recording/streaming/studio/replay/markers/
    /// stills/panic/timers/audio).
    pub feature: String,
    /// The audio source's name (PTT/PTM rows only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    pub registered: bool,
    /// How many OTHER bindings share this key.
    pub shared_with: u32,
    pub valid: bool,
}

/// One binding before conflict analysis (pure input — testable).
struct RawBinding {
    accelerator: String,
    action: &'static str,
    feature: &'static str,
    source: Option<String>,
    registered: bool,
}

/// Canonical comparison key: the parsed accelerator's display form, or the
/// lowercased raw text when it doesn't parse (still groups exact repeats).
fn canonical(accelerator: &str) -> (String, bool) {
    match accelerator.parse::<Shortcut>() {
        Ok(shortcut) => (shortcut.to_string(), true),
        Err(_) => (accelerator.trim().to_ascii_lowercase(), false),
    }
}

fn analyze(raw: Vec<RawBinding>) -> Vec<HotkeyAuditEntry> {
    let keys: Vec<(String, bool)> = raw
        .iter()
        .map(|binding| canonical(&binding.accelerator))
        .collect();
    let mut counts: HashMap<&str, u32> = HashMap::new();
    for (key, _) in &keys {
        *counts.entry(key.as_str()).or_insert(0) += 1;
    }
    raw.iter()
        .zip(&keys)
        .map(|(binding, (key, valid))| HotkeyAuditEntry {
            accelerator: binding.accelerator.clone(),
            action: binding.action.to_string(),
            feature: binding.feature.to_string(),
            source: binding.source.clone(),
            registered: binding.registered,
            shared_with: counts[key.as_str()] - 1,
            valid: *valid,
        })
        .collect()
}

fn is_registered(set: &HashSet<Shortcut>, accelerator: &str) -> bool {
    accelerator
        .parse::<Shortcut>()
        .map(|shortcut| set.contains(&shortcut))
        .unwrap_or(false)
}

fn gather<R: Runtime>(app: &AppHandle<R>) -> Vec<RawBinding> {
    let settings = app.state::<crate::settings::SettingsStore>().get().hotkeys;
    let action_registered = app.state::<crate::hotkeys::ActionHotkeys>().registered();
    let audio_registered = app.state::<crate::audio::HotkeyRegistry>().registered();

    let mut raw = Vec::new();
    for (key, action, feature) in [
        (&settings.record, "record", "recording"),
        (&settings.go_live, "goLive", "streaming"),
        (&settings.transition, "transition", "studio"),
        (&settings.save_replay, "saveReplay", "replay"),
        (&settings.add_marker, "addMarker", "markers"),
        (&settings.still, "still", "stills"),
        (&settings.panic, "panic", "panic"),
        (&settings.timer_toggle, "timerToggle", "timers"),
        (&settings.timer_reset, "timerReset", "timers"),
        (&settings.split_timer_split, "splitTimerSplit", "splitTimer"),
        (&settings.split_timer_undo, "splitTimerUndo", "splitTimer"),
        (&settings.split_timer_skip, "splitTimerSkip", "splitTimer"),
        (&settings.split_timer_reset, "splitTimerReset", "splitTimer"),
        (&settings.playlist_next, "playlistNext", "playlist"),
        (&settings.playlist_previous, "playlistPrevious", "playlist"),
        (&settings.replay_roll, "replayRoll", "replay"),
    ] {
        let Some(text) = key.as_ref().filter(|text| !text.trim().is_empty()) else {
            continue;
        };
        raw.push(RawBinding {
            accelerator: text.clone(),
            action,
            feature,
            source: None,
            registered: is_registered(&action_registered, text),
        });
    }
    app.state::<crate::studio::StudioState>()
        .with_collection(|collection| {
            for source in &collection.sources {
                let Some(audio) = &source.audio else { continue };
                for (key, action) in [
                    (&audio.push_to_talk, "pushToTalk"),
                    (&audio.push_to_mute, "pushToMute"),
                ] {
                    let Some(text) = key.as_ref().filter(|text| !text.trim().is_empty()) else {
                        continue;
                    };
                    raw.push(RawBinding {
                        accelerator: text.clone(),
                        action,
                        feature: "audio",
                        source: Some(source.name.clone()),
                        registered: is_registered(&audio_registered, text),
                    });
                }
            }
        });
    raw
}

/// Every binding in the studio, with conflict analysis.
#[tauri::command]
pub fn hotkey_audit<R: Runtime>(app: AppHandle<R>) -> Vec<HotkeyAuditEntry> {
    analyze(gather(&app))
}

/// Save the UI-composed cheat sheet (Markdown, localized client-side) into
/// Downloads under a fixed timestamped name and return the path. The caller
/// controls only the content (bounded) — never the location or filename.
#[tauri::command]
pub fn hotkey_cheatsheet_save(content: String) -> Result<String, String> {
    if content.len() > 65_536 {
        return Err("the cheat sheet is unexpectedly large".to_string());
    }
    let dir = directories::UserDirs::new()
        .and_then(|dirs| {
            dirs.download_dir()
                .map(PathBuf::from)
                .or_else(|| Some(dirs.home_dir().to_path_buf()))
        })
        .ok_or("no folder to write the cheat sheet into")?;
    let stamp = chrono::Local::now().format("%Y-%m-%d %H-%M-%S");
    let path = dir.join(format!("freally-hotkeys {stamp}.md"));
    std::fs::write(&path, content).map_err(|err| err.to_string())?;
    Ok(path.display().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn binding(accelerator: &str, action: &'static str) -> RawBinding {
        RawBinding {
            accelerator: accelerator.to_string(),
            action,
            feature: "recording",
            source: None,
            registered: true,
        }
    }

    #[test]
    fn shared_keys_are_counted_across_formats() {
        // The same chord written two ways is ONE key — the classic silent
        // conflict the audit exists to catch.
        let entries = analyze(vec![
            binding("Ctrl+Shift+R", "record"),
            binding("ctrl+shift+r", "pushToTalk"),
            binding("F13", "panic"),
        ]);
        assert_eq!(entries[0].shared_with, 1);
        assert_eq!(entries[1].shared_with, 1);
        assert_eq!(entries[2].shared_with, 0);
    }

    #[test]
    fn unparsable_accelerators_are_flagged_not_dropped() {
        let entries = analyze(vec![
            binding("not a chord!!", "record"),
            binding("NOT A CHORD!!", "panic"),
        ]);
        assert!(!entries[0].valid);
        // Exact repeats of invalid text still group (lowercased).
        assert_eq!(entries[0].shared_with, 1);
        assert_eq!(entries[1].shared_with, 1);
    }

    #[test]
    fn distinct_keys_do_not_conflict() {
        let entries = analyze(vec![
            binding("Ctrl+Shift+R", "record"),
            binding("Ctrl+Shift+S", "saveReplay"),
        ]);
        assert!(entries.iter().all(|entry| entry.shared_with == 0));
        assert!(entries.iter().all(|entry| entry.valid));
    }
}
