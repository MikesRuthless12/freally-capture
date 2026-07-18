//! CAP-N68: quick-actions grid.
//!
//! A customizable button grid (Stream-Deck ergonomics) whose buttons fire the
//! same allowlisted studio commands the LAN panel / OSC / MIDI already use, or
//! trigger a soundboard pad. The config lives in `<config_dir>/quick-actions.
//! json` and is **shared with the CAP-N06 LAN panel** — the web panel reads the
//! same file, so a button laid out here appears on the phone panel too. Command
//! buttons route through [`crate::remote_api::dispatch_any`] (the fixed
//! allowlist), so this surface can never reach a command the remote API doesn't
//! already expose. Strictly local.

use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::{AppHandle, Runtime, State};

use crate::profiles::WorkspaceState;

/// A page can hold at most this many buttons; a config at most this many pages.
const MAX_BUTTONS: usize = 64;
const MAX_PAGES: usize = 20;

/// What a button does.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum QuickActionSpec {
    /// An allowlisted studio command (scene switch, transition, start/stop,
    /// marker, run-macro, …) — the exact path the LAN panel uses.
    Command {
        command: String,
        #[serde(default)]
        params: Value,
    },
    /// A soundboard pad by id (desktop only — pads are not on the remote
    /// allowlist, so the LAN panel skips these).
    Soundboard { pad: String },
}

/// One grid button.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickAction {
    pub label: String,
    /// `#rrggbb` accent, or empty for the default.
    #[serde(default)]
    pub color: String,
    pub action: QuickActionSpec,
}

/// A named page of buttons.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickActionPage {
    pub name: String,
    #[serde(default)]
    pub buttons: Vec<QuickAction>,
}

/// The whole grid config.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickActions {
    #[serde(default)]
    pub pages: Vec<QuickActionPage>,
}

fn store_path(base: &Path) -> PathBuf {
    base.join("quick-actions.json")
}

/// In-memory cache of the last-read grid, keyed by its config dir. The LAN panel
/// polls `/api/state` (which calls [`read`]) about once per second per client, and
/// this file only changes when the operator edits the grid — so cache the parse
/// and re-read disk only on a cache miss. [`write`] clears the cache so the next
/// read (from the desktop dock OR the panel) always reflects a fresh save.
fn cache() -> &'static Mutex<Option<(PathBuf, QuickActions)>> {
    static CACHE: OnceLock<Mutex<Option<(PathBuf, QuickActions)>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(None))
}

/// Read the grid config (used by the desktop dock AND the LAN panel state).
pub(crate) fn read(base: &Path) -> QuickActions {
    if let Some((cached_base, config)) = cache().lock().unwrap().as_ref() {
        if cached_base == base {
            return config.clone();
        }
    }
    let config: QuickActions = std::fs::read_to_string(store_path(base))
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_default();
    *cache().lock().unwrap() = Some((base.to_path_buf(), config.clone()));
    config
}

/// Validate + persist the grid, then drop the cache so the next [`read`] reloads
/// it. The only writer of `quick-actions.json`.
fn write(base: &Path, config: &QuickActions) -> Result<(), String> {
    if config.pages.len() > MAX_PAGES {
        return Err(format!("at most {MAX_PAGES} pages"));
    }
    for page in &config.pages {
        if page.buttons.len() > MAX_BUTTONS {
            return Err(format!("at most {MAX_BUTTONS} buttons on a page"));
        }
    }
    let json = serde_json::to_string_pretty(config).map_err(|err| err.to_string())?;
    crate::settings::write_atomic(&store_path(base), &json).map_err(|err| err.to_string())?;
    *cache().lock().unwrap() = None;
    Ok(())
}

/// The saved grid.
#[tauri::command]
pub fn quick_actions_get(state: State<'_, WorkspaceState>) -> Result<QuickActions, String> {
    Ok(read(state.base()?))
}

/// Replace the saved grid.
#[tauri::command]
pub fn quick_actions_set(
    state: State<'_, WorkspaceState>,
    config: QuickActions,
) -> Result<(), String> {
    write(state.base()?, &config)
}

/// Fire an allowlisted studio command from a quick-action button — the same
/// gate the LAN panel, OSC, and MIDI go through. (Soundboard buttons call
/// `soundboard_trigger` directly on the UI side.)
#[tauri::command]
pub fn quick_action_dispatch<R: Runtime>(
    app: AppHandle<R>,
    command: String,
    params: Value,
) -> Result<Value, String> {
    crate::remote_api::dispatch_any(&app, &command, &params)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_invalidates_the_read_cache() {
        let dir = std::env::temp_dir().join(format!("fcap-qa-cache-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        // Empty → default; the read caches it.
        assert_eq!(read(&dir).pages.len(), 0);

        // A save must be visible to the very next read (cache cleared on write).
        let one = QuickActions {
            pages: vec![QuickActionPage {
                name: "P1".into(),
                buttons: vec![],
            }],
        };
        write(&dir, &one).unwrap();
        assert_eq!(read(&dir).pages.len(), 1);

        // Over-limit is rejected and does not disturb the stored grid.
        let too_many = QuickActions {
            pages: (0..MAX_PAGES + 1)
                .map(|i| QuickActionPage {
                    name: format!("P{i}"),
                    buttons: vec![],
                })
                .collect(),
        };
        assert!(write(&dir, &too_many).is_err());
        assert_eq!(read(&dir).pages.len(), 1);

        let _ = std::fs::remove_dir_all(&dir);
    }
}
