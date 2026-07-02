//! The UI ↔ core command surface.
//!
//! Typed on both sides: `ui/src/api/` mirrors these signatures and payload
//! shapes — keep them in lockstep.

use serde::Serialize;
use tauri::State;

use crate::settings::{Settings, SettingsStore};

/// One linked core crate, as reported by [`health`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CrateHealth {
    pub name: &'static str,
    pub version: &'static str,
}

/// The [`health`] report: proves the command bridge and the owned workspace
/// crates are linked and alive.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Health {
    pub app_version: &'static str,
    pub os: &'static str,
    pub core_ok: bool,
    pub crates: Vec<CrateHealth>,
}

/// Bridge liveness probe.
#[tauri::command]
pub fn health() -> Health {
    Health {
        app_version: env!("CARGO_PKG_VERSION"),
        os: std::env::consts::OS,
        core_ok: true,
        crates: vec![
            CrateHealth {
                name: "fcap-capture",
                version: fcap_capture::VERSION,
            },
            CrateHealth {
                name: "fcap-sources",
                version: fcap_sources::VERSION,
            },
            CrateHealth {
                name: "fcap-compositor",
                version: fcap_compositor::VERSION,
            },
            CrateHealth {
                name: "fcap-scene",
                version: fcap_scene::VERSION,
            },
            CrateHealth {
                name: "fcap-audio",
                version: fcap_audio::VERSION,
            },
            CrateHealth {
                name: "fcap-encode",
                version: fcap_encode::VERSION,
            },
            CrateHealth {
                name: "fcap-stream",
                version: fcap_stream::VERSION,
            },
        ],
    }
}

/// Read the current settings.
#[tauri::command]
pub fn settings_get(store: State<'_, SettingsStore>) -> Settings {
    store.get()
}

/// Replace and persist the settings.
#[tauri::command]
pub fn settings_set(store: State<'_, SettingsStore>, settings: Settings) -> Result<(), String> {
    store
        .set(settings)
        .map_err(|err| format!("could not save settings: {err}"))
}

#[cfg(test)]
mod tests {
    use super::health;

    #[test]
    fn health_reports_every_owned_crate() {
        let report = health();
        assert!(report.core_ok);
        assert_eq!(report.app_version, env!("CARGO_PKG_VERSION"));
        assert_eq!(report.crates.len(), 7, "all owned fcap-* crates are linked");
    }
}
