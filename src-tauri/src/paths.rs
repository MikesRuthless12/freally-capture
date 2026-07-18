//! CAP-N63: **portable mode** + the app's single path-resolution chokepoint.
//!
//! A marker file (`freally-portable.txt`) sitting next to the executable
//! switches the app to fully self-contained operation: all of its own data —
//! settings, profiles, scene collections, workspace, session/salvage markers,
//! crash reports — lives under `<exe-dir>/FreallyData/` instead of the host OS
//! user profile, so the whole studio can run from a USB stick on a venue or
//! tournament machine without leaving a trace in the host account.
//!
//! Every persistent path in the app resolves through [`config_dir`] /
//! [`data_dir`] here, so portable mode is one decision made once at startup.
//! [`init`] MUST run before any of them is read (it does — first thing in
//! `main`). If it never runs (a code path that returns before it, or a test),
//! these fall back to the normal OS directories — portable mode is strictly
//! opt-in and additive.
//!
//! **Out of scope (by design):** the large, re-downloadable runtime components
//! (the on-demand ffmpeg and CEF binaries) stay in the OS cache dir — they are
//! not user data, are hundreds of MB, and are fetched only on explicit opt-in.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// The marker file that turns portable mode on, looked for next to the exe.
const MARKER: &str = "freally-portable.txt";

/// All app data lives under this subfolder of the exe dir in portable mode.
const PORTABLE_SUBDIR: &str = "FreallyData";

/// `Some(exe_dir)` when portable, `Some(None)`… — flattened: the portable root
/// (the exe's directory) when the marker is present, else `None`.
static PORTABLE_ROOT: OnceLock<Option<PathBuf>> = OnceLock::new();

/// Detect portable mode exactly once. Reads the exe's directory and checks for
/// [`MARKER`]. Safe to call from any startup path; the first call wins.
pub fn init() {
    let root = std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(Path::to_path_buf))
        .filter(|dir| dir.join(MARKER).is_file());
    let _ = PORTABLE_ROOT.set(root);
}

/// The portable root (exe dir) if the marker was found, else `None`. Before
/// [`init`] runs this is `None`, so callers fall back to OS directories.
fn portable_root() -> Option<PathBuf> {
    PORTABLE_ROOT.get().cloned().flatten()
}

/// Is the app running in portable mode?
pub fn is_portable() -> bool {
    portable_root().is_some()
}

fn os_dirs() -> Option<directories::ProjectDirs> {
    directories::ProjectDirs::from("com", "Freally", "Freally Capture")
}

// Resolved once (portable mode is decided once at startup; the OS dirs don't
// change) — these are hit on the LAN-panel poll path, so avoid re-running the
// `ProjectDirs` lookup every call.
static CONFIG_DIR: OnceLock<Option<PathBuf>> = OnceLock::new();
static DATA_DIR: OnceLock<Option<PathBuf>> = OnceLock::new();

/// Where settings / profiles / collections / workspace / session markers live:
/// `<exe>/FreallyData/config` in portable mode, else the OS config dir.
pub fn config_dir() -> Option<PathBuf> {
    CONFIG_DIR
        .get_or_init(|| match portable_root() {
            Some(root) => Some(root.join(PORTABLE_SUBDIR).join("config")),
            None => os_dirs().map(|dirs| dirs.config_dir().to_path_buf()),
        })
        .clone()
}

/// Where crash reports live: `<exe>/FreallyData/data` in portable mode, else the
/// OS data dir.
pub fn data_dir() -> Option<PathBuf> {
    DATA_DIR
        .get_or_init(|| match portable_root() {
            Some(root) => Some(root.join(PORTABLE_SUBDIR).join("data")),
            None => os_dirs().map(|dirs| dirs.data_dir().to_path_buf()),
        })
        .clone()
}

/// Report for the UI: portable on/off and the resolved data locations.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PortableStatus {
    pub portable: bool,
    /// The marker filename to create next to the exe to enable portable mode.
    pub marker: String,
    pub config_dir: String,
    pub data_dir: String,
}

/// CAP-N63: report portable status + where data is stored (Settings → About).
#[tauri::command]
pub fn portable_status() -> PortableStatus {
    PortableStatus {
        portable: is_portable(),
        marker: MARKER.to_string(),
        config_dir: config_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_default(),
        data_dir: data_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn portable_paths_live_under_the_exe_folder() {
        let root = Path::new("E:/FreallyCapture");
        // The pure layout the resolver uses in portable mode.
        assert_eq!(
            root.join(PORTABLE_SUBDIR).join("config"),
            Path::new("E:/FreallyCapture/FreallyData/config")
        );
        assert_eq!(
            root.join(PORTABLE_SUBDIR).join("data"),
            Path::new("E:/FreallyCapture/FreallyData/data")
        );
    }

    #[test]
    fn defaults_to_os_dirs_when_not_portable() {
        // Without init() setting a portable root, resolution uses the OS dirs
        // (non-None on any supported OS in CI).
        assert!(!is_portable());
        assert!(config_dir().is_some());
    }
}
