//! # fcap-ndi — optional NDI output/input (Phase 8, TASK-804)
//!
//! NDI (Network Device Interface, Vizrt) is a **proprietary runtime** we never
//! bundle — it ships separately (the free *NDI Tools* / runtime redistributable)
//! and has its own license and ~tens-of-MB footprint. This crate instead
//! **detects** a user-installed NDI runtime at load time via the documented
//! `NDI_RUNTIME_DIR_V*` environment variables and the per-OS default install
//! locations, confirms the runtime links (its `NDIlib_v*_load` entry point
//! resolves), and exposes the typed output interface **behind that flag**.
//!
//! The one `unsafe` — `libloading` opening the user's runtime library — is
//! isolated here so `fcap-sources` / `fcap-audio` stay `#![forbid(unsafe_code)]`.
//! We only *link-probe* the runtime; sending frames through it needs the (free)
//! NDI SDK headers to bind `NDIlib_send_*`, which is the named follow-on this
//! interface is stable for. Nothing here reaches the network.

#![deny(unsafe_code)]

use std::path::{Path, PathBuf};

use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum NdiError {
    #[error("no NDI runtime is installed (install the free NDI Tools / runtime)")]
    RuntimeMissing,
    #[error("NDI output is not enabled")]
    Disabled,
}

/// The result of probing for an installed NDI runtime.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NdiStatus {
    /// Whether a usable NDI runtime was found and links.
    pub available: bool,
    /// The runtime API generation that resolved (e.g. `"v5"`), if any.
    pub version: Option<String>,
    /// The runtime library that satisfied the probe, if any.
    pub runtime_path: Option<PathBuf>,
    /// Honest guidance shown when unavailable (where to get the free runtime).
    pub guidance: String,
}

impl NdiStatus {
    fn unavailable() -> Self {
        Self {
            available: false,
            version: None,
            runtime_path: None,
            guidance: guidance(),
        }
    }
}

/// Where to get the free runtime — shown when NDI isn't installed.
pub fn guidance() -> String {
    "NDI output/input needs the NDI runtime, which Freally Capture never bundles \
     (it's Vizrt's separate, licensed runtime). Install the free NDI Tools from \
     ndi.video, then this turns on automatically — no app update needed."
        .to_string()
}

/// The environment variables the NDI redistributable sets to its runtime dir,
/// newest generation first.
fn runtime_env_vars() -> &'static [&'static str] {
    &[
        "NDI_RUNTIME_DIR_V6",
        "NDI_RUNTIME_DIR_V5",
        "NDI_RUNTIME_DIR_V4",
    ]
}

/// The runtime library file names to look for, per OS.
fn library_names() -> &'static [&'static str] {
    #[cfg(target_os = "windows")]
    {
        &["Processing.NDI.Lib.x64.dll"]
    }
    #[cfg(target_os = "macos")]
    {
        &["libndi.dylib"]
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        &["libndi.so.6", "libndi.so.5", "libndi.so.4", "libndi.so"]
    }
}

/// The per-OS default directories the runtime installs into.
fn default_dirs() -> Vec<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        // The Windows loader also searches PATH; bare names cover that.
        vec![PathBuf::new()]
    }
    #[cfg(target_os = "macos")]
    {
        vec![
            PathBuf::from("/usr/local/lib"),
            PathBuf::from("/Library/NDI SDK for Apple/lib/macOS"),
        ]
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        vec![
            PathBuf::from("/usr/lib"),
            PathBuf::from("/usr/local/lib"),
            PathBuf::from("/usr/lib/x86_64-linux-gnu"),
        ]
    }
}

/// Build the ordered candidate library paths from the env dirs + default dirs ×
/// the per-OS library names. Pure (env lookup injected) so it is unit-testable.
fn resolve_candidates(
    get_env: impl Fn(&str) -> Option<String>,
    default_dirs: &[PathBuf],
    names: &[&str],
) -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = Vec::new();
    for var in runtime_env_vars() {
        if let Some(dir) = get_env(var) {
            if !dir.is_empty() {
                dirs.push(PathBuf::from(dir));
            }
        }
    }
    dirs.extend_from_slice(default_dirs);

    let mut out = Vec::new();
    for dir in &dirs {
        for name in names {
            // An empty dir yields the bare name (relies on the OS loader path).
            out.push(if dir.as_os_str().is_empty() {
                PathBuf::from(name)
            } else {
                dir.join(name)
            });
        }
    }
    out
}

/// The NDI runtime's versioned load entry points, newest first. If any resolves
/// the runtime is usable.
const LOAD_SYMBOLS: &[(&str, &str)] = &[
    ("NDIlib_v6_load", "v6"),
    ("NDIlib_v5_load", "v5"),
    ("NDIlib_v4_load", "v4"),
    ("NDIlib_v3_load", "v3"),
];

/// Probe for an installed, linkable NDI runtime.
pub fn detect() -> NdiStatus {
    let candidates = resolve_candidates(
        |k| std::env::var(k).ok(),
        &default_dirs(),
        library_names(),
    );
    for path in candidates {
        // Bare names (Windows PATH search) can't be existence-checked; only
        // skip absolute/relative paths we can see are absent.
        if path.parent().is_some_and(|p| !p.as_os_str().is_empty()) && !path_exists(&path) {
            continue;
        }
        if let Some((path, version)) = probe_library(&path) {
            return NdiStatus {
                available: true,
                version: Some(version),
                runtime_path: Some(path),
                guidance: String::new(),
            };
        }
    }
    NdiStatus::unavailable()
}

fn path_exists(path: &Path) -> bool {
    path.exists()
}

/// Try to load `path` and resolve any NDI load entry point. Returns the path +
/// the matched version on success. The `unsafe` (loading a user library) is
/// contained here; we never *call* into the runtime.
#[allow(unsafe_code)] // libloading probe of a user-installed runtime
fn probe_library(path: &Path) -> Option<(PathBuf, String)> {
    // SAFETY: opening a shared library and resolving a symbol's presence. We do
    // not invoke it, so no foreign code runs from this probe.
    unsafe {
        let lib = libloading::Library::new(path.as_os_str()).ok()?;
        for (symbol, version) in LOAD_SYMBOLS {
            if lib
                .get::<unsafe extern "C" fn() -> *const std::ffi::c_void>(symbol.as_bytes())
                .is_ok()
            {
                return Some((path.to_path_buf(), (*version).to_string()));
            }
        }
        None
    }
}

/// The opt-in NDI **output** configuration — the flag + interface the app
/// persists. Sending the program feed lights up once the runtime is present and
/// the SDK-header frame binding lands (the named follow-on).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NdiOutputConfig {
    /// The NDI source name other machines see on the network.
    pub name: String,
    /// Whether NDI output is enabled (still gated on a detected runtime).
    pub enabled: bool,
}

impl Default for NdiOutputConfig {
    fn default() -> Self {
        Self {
            name: "Freally Capture".to_string(),
            enabled: false,
        }
    }
}

impl NdiOutputConfig {
    /// Whether output should actually run: enabled *and* a runtime is present.
    pub fn effective(&self, status: &NdiStatus) -> Result<(), NdiError> {
        if !self.enabled {
            return Err(NdiError::Disabled);
        }
        if !status.available {
            return Err(NdiError::RuntimeMissing);
        }
        Ok(())
    }
}

/// This crate's version (inherited from the workspace).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_dirs_come_before_defaults_and_cover_all_names() {
        let names = &["libndi.so.6", "libndi.so"];
        let defaults = vec![PathBuf::from("/usr/lib")];
        let got = resolve_candidates(
            |k| (k == "NDI_RUNTIME_DIR_V5").then(|| "/opt/ndi".to_string()),
            &defaults,
            names,
        );
        // env dir first, both names, then the default dir, both names.
        assert_eq!(got[0], PathBuf::from("/opt/ndi/libndi.so.6"));
        assert_eq!(got[1], PathBuf::from("/opt/ndi/libndi.so"));
        assert_eq!(got[2], PathBuf::from("/usr/lib/libndi.so.6"));
        assert_eq!(got[3], PathBuf::from("/usr/lib/libndi.so"));
    }

    #[test]
    fn empty_dir_yields_bare_name_for_loader_search() {
        let got = resolve_candidates(|_| None, &[PathBuf::new()], &["Processing.NDI.Lib.x64.dll"]);
        assert_eq!(got, vec![PathBuf::from("Processing.NDI.Lib.x64.dll")]);
    }

    #[test]
    fn empty_env_value_is_ignored() {
        let got = resolve_candidates(|_| Some(String::new()), &[PathBuf::from("/usr/lib")], &["x"]);
        assert_eq!(got, vec![PathBuf::from("/usr/lib/x")]);
    }

    #[test]
    fn detect_is_honest_when_absent() {
        // On CI/dev without an NDI runtime, detect() must not claim availability
        // and must carry the guidance. (If a runtime IS installed, this simply
        // reports it — either way it never lies.)
        let status = detect();
        if !status.available {
            assert!(status.runtime_path.is_none());
            assert!(!status.guidance.is_empty());
        } else {
            assert!(status.version.is_some());
        }
    }

    #[test]
    fn output_is_gated_on_both_enable_and_runtime() {
        let present = NdiStatus {
            available: true,
            version: Some("v5".into()),
            runtime_path: Some(PathBuf::from("/x")),
            guidance: String::new(),
        };
        let absent = NdiStatus::unavailable();
        let cfg_on = NdiOutputConfig {
            enabled: true,
            ..Default::default()
        };
        let cfg_off = NdiOutputConfig::default();
        assert!(cfg_on.effective(&present).is_ok());
        assert!(matches!(
            cfg_on.effective(&absent),
            Err(NdiError::RuntimeMissing)
        ));
        assert!(matches!(cfg_off.effective(&present), Err(NdiError::Disabled)));
    }
}
