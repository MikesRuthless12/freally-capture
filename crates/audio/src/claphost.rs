//! CAP-N33 CLAP plugin hosting — the **MIT-clean** third-party-plugin path.
//!
//! **CLAP** is a permissively-licensed (MIT) open plugin standard, so it is a
//! path this project can host with zero licensing entanglement. (As of the VST
//! 3.8 SDK — Steinberg, 2025-10-29 — **VST3 is also MIT-licensed**, so it's an
//! equally-clean sibling path; see [`crate::vst`]. Both run through the same
//! crash-isolated host process.) This module is the owned CLAP foundation:
//!
//! - **Discovery is live**: [`scan_clap_plugins`] finds the `.clap` bundles the
//!   user already has installed, from the standard per-OS CLAP directories only
//!   (local-directory, opt-in — nothing is fetched, nothing leaves the machine).
//! - **Hosting is crash-isolated by design**: the plan is a **separate host
//!   process** that loads and runs each plugin, so a misbehaving plugin can
//!   never take down the mix — and its plugin GUI opens in that process too.
//!   That host-process audio bridge + GUI embedding is the active integration
//!   work; it needs real plugins and per-OS window handling to build and smoke,
//!   which is why the chain doesn't yet *insert* a CLAP plugin. Discovery, the
//!   model, and this boundary land first so the wiring is honest, not a fake
//!   toggle.
//!
//! The owned classic-DSP filter set (denoise, gate, compressor, limiter,
//! parametric EQ, de-esser, rumble guard, ducking) already covers the common
//! needs with nothing to install.

use std::path::PathBuf;

/// One installed audio-plugin bundle the host discovered — CLAP or (since the
/// VST 3.8 MIT relicense) VST3, both $0-clean.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClapPluginInfo {
    /// The `.clap` / `.vst3` bundle path.
    pub path: PathBuf,
    /// A display name (the bundle's file stem — the plugin's own name comes
    /// from the metadata the host process reads when it loads it).
    pub name: String,
    /// The plugin format: `"clap"` or `"vst3"`.
    pub format: &'static str,
}

/// The honest status of CLAP hosting, surfaced verbatim in the UI so the
/// boundary is never a silent no-op.
pub const CLAP_STATUS: &str =
    "CLAP and VST3 are both MIT-licensed, $0 plugin paths — your installed \
     plugins are listed below. Live hosting runs each plugin in a separate, \
     crash-isolated process (so a bad plugin can't take down the mix) with its \
     own GUI window; that host-process integration is in progress. Meanwhile the \
     built-in filters (denoise, gate, compressor, limiter, parametric EQ, \
     de-esser, rumble guard, ducking) need nothing installed and stay on this \
     machine.";

/// The standard per-OS directories for a plugin format's bundles + the format
/// tag + the env override to consult. Only these well-known locations are
/// scanned — never an arbitrary path, and never the network.
fn search_dirs(subdir: &str, unix_lib: &str, home_dir: &str, env_override: &str) -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = Vec::new();
    let home = std::env::var_os("HOME").map(PathBuf::from);

    #[cfg(target_os = "windows")]
    {
        let _ = (unix_lib, home_dir);
        if let Some(common) = std::env::var_os("CommonProgramFiles") {
            dirs.push(PathBuf::from(common).join(subdir));
        }
        if let Some(local) = std::env::var_os("LOCALAPPDATA") {
            dirs.push(
                PathBuf::from(local)
                    .join("Programs")
                    .join("Common")
                    .join(subdir),
            );
        }
    }
    #[cfg(target_os = "macos")]
    {
        let _ = unix_lib;
        dirs.push(PathBuf::from(format!("/Library/Audio/Plug-Ins/{subdir}")));
        if let Some(home) = &home {
            dirs.push(home.join(format!("Library/Audio/Plug-Ins/{subdir}")));
        }
    }
    #[cfg(target_os = "linux")]
    {
        let _ = subdir;
        dirs.push(PathBuf::from(format!("/usr/lib/{unix_lib}")));
        dirs.push(PathBuf::from(format!("/usr/local/lib/{unix_lib}")));
        if let Some(home) = &home {
            dirs.push(home.join(home_dir));
        }
    }
    if let Some(extra) = std::env::var_os(env_override) {
        for part in std::env::split_paths(&extra) {
            dirs.push(part);
        }
    }
    let _ = &home;
    dirs
}

fn has_extension(path: &std::path::Path, want: &str) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case(want))
}

fn scan_format(dirs: Vec<PathBuf>, ext: &str, format: &'static str, out: &mut Vec<ClapPluginInfo>) {
    for dir in dirs {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if !has_extension(&path, ext) {
                continue;
            }
            let name = path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or("plugin")
                .to_string();
            if !out.iter().any(|plugin| plugin.path == path) {
                out.push(ClapPluginInfo { path, name, format });
            }
        }
    }
}

/// Scan the standard plugin directories for installed CLAP **and VST3** bundles
/// (opt-in — called only when the user opens the plugins panel). Both are
/// MIT-licensed; local-directory only, no fetch.
pub fn scan_clap_plugins() -> Vec<ClapPluginInfo> {
    let mut found: Vec<ClapPluginInfo> = Vec::new();
    scan_format(
        search_dirs("CLAP", "clap", ".clap", "CLAP_PATH"),
        "clap",
        "clap",
        &mut found,
    );
    scan_format(
        search_dirs("VST3", "vst3", ".vst3", "VST3_PATH"),
        "vst3",
        "vst3",
        &mut found,
    );
    found.sort_by(|a, b| {
        a.name
            .to_lowercase()
            .cmp(&b.name.to_lowercase())
            .then(a.format.cmp(b.format))
    });
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scans_a_directory_for_clap_bundles() {
        let dir = std::env::temp_dir().join(format!(
            "fcap-clap-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("Reverb.clap"), b"x").unwrap();
        std::fs::write(dir.join("EQ.clap"), b"x").unwrap();
        std::fs::write(dir.join("Comp.vst3"), b"x").unwrap();
        std::fs::write(dir.join("notes.txt"), b"x").unwrap();

        std::env::set_var("CLAP_PATH", &dir);
        std::env::set_var("VST3_PATH", &dir);
        let plugins = scan_clap_plugins();
        std::env::remove_var("CLAP_PATH");
        std::env::remove_var("VST3_PATH");

        // Both formats are found (sorted), the .txt ignored.
        assert!(plugins
            .iter()
            .any(|p| p.name == "Reverb" && p.format == "clap"));
        assert!(plugins.iter().any(|p| p.name == "EQ" && p.format == "clap"));
        assert!(plugins
            .iter()
            .any(|p| p.name == "Comp" && p.format == "vst3"));
        assert!(!plugins.iter().any(|p| p.name == "notes"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_status_is_honest_about_the_boundary() {
        assert!(CLAP_STATUS.contains("MIT"));
        assert!(CLAP_STATUS.contains("crash-isolated"));
        // Points at the shipped alternative rather than dead-ending.
        assert!(CLAP_STATUS.to_lowercase().contains("filter"));
    }
}
