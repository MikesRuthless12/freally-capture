//! Optional **CEF (Chromium Embedded Framework)** runtime for Browser Sources
//! (Phase 8, TASK-702b/Task-18) — fetched on demand, **never bundled**.
//!
//! CEF is a ~100 MB Chromium runtime with its own (BSD) license and patent
//! surface, so — exactly like the ffmpeg wire-codec component — it is downloaded
//! only on an explicit click, **verified before anything is unpacked**, cached
//! per-user, and clearly labeled.
//!
//! The integrity story is honest for a runtime whose hash we can't embed at
//! build time: we do **not** hardcode a checksum. Instead [`resolve_build`]
//! fetches the official CEF build index over HTTPS
//! (`cef-builds.spotifycdn.com/index.json`), picks the **newest stable
//! `minimal`** distribution for this OS, and takes that entry's authoritative
//! **SHA-1** straight from the index. The download is then verified against it
//! before extraction — the same trusted source vouches for both the file and
//! its hash. A mismatch refuses the install.
//!
//! This module ships the **download + verify + extract** half. The browser
//! source that *renders* through the extracted runtime is the named follow-on
//! milestone (it needs the CEF host process + the compositor tap); the runtime
//! path is exposed via [`installed`] for it.

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use thiserror::Error;

const INDEX_URL: &str = "https://cef-builds.spotifycdn.com/index.json";
const FILE_BASE: &str = "https://cef-builds.spotifycdn.com/";

/// The CEF CDN platform key for this build target (`None` if CEF publishes no
/// build for it — said honestly).
pub fn platform_key() -> Option<&'static str> {
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    {
        Some("windows64")
    }
    #[cfg(all(target_os = "windows", target_arch = "aarch64"))]
    {
        Some("windowsarm64")
    }
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
        Some("linux64")
    }
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    {
        Some("linuxarm64")
    }
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        Some("macosarm64")
    }
    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    {
        Some("macosx64")
    }
    #[cfg(not(any(
        all(target_os = "windows", target_arch = "x86_64"),
        all(target_os = "windows", target_arch = "aarch64"),
        all(target_os = "linux", target_arch = "x86_64"),
        all(target_os = "linux", target_arch = "aarch64"),
        all(target_os = "macos", target_arch = "aarch64"),
        all(target_os = "macos", target_arch = "x86_64"),
    )))]
    {
        None
    }
}

#[derive(Debug, Error)]
pub enum CefError {
    #[error("CEF publishes no browser-source runtime for this platform")]
    Unsupported,
    #[error("could not reach the CEF build index / download: {0}")]
    Http(String),
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
    #[error(
        "the downloaded CEF runtime does not match the index checksum — refusing to install it \
         (expected {expected}, got {got}). Retry later."
    )]
    HashMismatch { expected: String, got: String },
    #[error("could not unpack the CEF archive: {0}")]
    Archive(String),
    #[error("the download was cancelled")]
    Cancelled,
}

// --- the slice of the CDN index.json we read (unknown fields ignored) ---------

#[derive(Debug, Deserialize)]
struct Index(HashMap<String, PlatformEntry>);

#[derive(Debug, Deserialize)]
struct PlatformEntry {
    #[serde(default)]
    versions: Vec<VersionEntry>,
}

#[derive(Debug, Deserialize)]
struct VersionEntry {
    #[serde(default)]
    cef_version: String,
    #[serde(default)]
    channel: String,
    #[serde(default)]
    files: Vec<FileEntry>,
}

#[derive(Debug, Deserialize)]
struct FileEntry {
    #[serde(default)]
    name: String,
    #[serde(default)]
    sha1: String,
    #[serde(default)]
    size: u64,
    #[serde(default, rename = "type")]
    kind: String,
}

/// The resolved build to fetch — the newest stable `minimal` for this OS, with
/// the CDN's authoritative SHA-1.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CefBuild {
    pub cef_version: String,
    pub name: String,
    pub url: String,
    pub sha1: String,
    pub size_bytes: u64,
}

/// Fetch the CEF build index and resolve the newest stable `minimal` build for
/// this platform (with its authoritative SHA-1). Blocking — run off the UI thread.
pub fn resolve_build() -> Result<CefBuild, CefError> {
    let key = platform_key().ok_or(CefError::Unsupported)?;
    let agent = ureq::AgentBuilder::new()
        .timeout_connect(Duration::from_secs(30))
        .timeout_read(Duration::from_secs(60))
        .build();
    let reader = agent
        .get(INDEX_URL)
        .call()
        .map_err(|err| CefError::Http(err.to_string()))?
        .into_reader();
    let index: Index = serde_json::from_reader(BufReader::new(reader))
        .map_err(|err| CefError::Http(format!("could not parse the CEF index: {err}")))?;
    pick_build(&index, key).ok_or(CefError::Unsupported)
}

/// Pure selection: newest **stable** version (by Chromium version) whose files
/// include a `minimal` entry with a non-empty sha1. Split out so it is testable
/// without the network.
fn pick_build(index: &Index, key: &str) -> Option<CefBuild> {
    let platform = index.0.get(key)?;
    let best = platform
        .versions
        .iter()
        .filter(|v| v.channel.eq_ignore_ascii_case("stable"))
        .filter(|v| v.files.iter().any(is_usable_minimal))
        .max_by_key(|v| chromium_ord(&v.cef_version))?;
    let file = best.files.iter().find(|f| is_usable_minimal(f))?;
    Some(CefBuild {
        cef_version: best.cef_version.clone(),
        name: file.name.clone(),
        url: format!("{FILE_BASE}{}", urlencode_name(&file.name)),
        sha1: file.sha1.clone(),
        size_bytes: file.size,
    })
}

fn is_usable_minimal(f: &FileEntry) -> bool {
    f.kind.eq_ignore_ascii_case("minimal") && f.sha1.len() == 40 && !f.name.is_empty()
}

/// Order key from the embedded `chromium-X.Y.Z.W` in a `cef_version` string.
fn chromium_ord(cef_version: &str) -> (u32, u32, u32, u32) {
    let Some(rest) = cef_version.split("chromium-").nth(1) else {
        return (0, 0, 0, 0);
    };
    let mut parts = rest.split('.').map(|p| {
        p.trim_matches(|c: char| !c.is_ascii_digit())
            .parse()
            .unwrap_or(0)
    });
    (
        parts.next().unwrap_or(0),
        parts.next().unwrap_or(0),
        parts.next().unwrap_or(0),
        parts.next().unwrap_or(0),
    )
}

/// The CDN encodes `+` in the file name as `%2B` in the URL path.
fn urlencode_name(name: &str) -> String {
    name.replace('+', "%2B")
}

// --- install / status / remove ------------------------------------------------

/// A ready, verified CEF runtime install.
#[derive(Debug, Clone)]
pub struct CefRuntime {
    /// The extracted CEF distribution directory (contains `Release/`, `Resources/`).
    pub runtime_dir: PathBuf,
    pub cef_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Marker {
    sha1: String,
    name: String,
    cef_version: String,
    /// The extracted top-level dir name (e.g. `cef_binary_..._minimal`).
    dir: String,
}

fn cache_root() -> Option<PathBuf> {
    directories::ProjectDirs::from("com", "Freally", "Freally Capture")
        .map(|dirs| dirs.cache_dir().join("cef"))
}

/// The installed-and-verified CEF runtime, if any (reads the marker; no process).
pub fn installed() -> Option<CefRuntime> {
    let root = cache_root()?.join("current");
    let marker: Marker =
        serde_json::from_str(&fs::read_to_string(root.join("verified.json")).ok()?).ok()?;
    let runtime_dir = root.join(&marker.dir);
    if !runtime_dir.is_dir() {
        return None;
    }
    Some(CefRuntime {
        runtime_dir,
        cef_version: marker.cef_version,
    })
}

/// Delete the installed runtime (the panel's Remove action).
pub fn remove() -> std::io::Result<()> {
    if let Some(root) = cache_root() {
        if root.exists() {
            fs::remove_dir_all(root)?;
        }
    }
    Ok(())
}

/// Where a fetch currently is (mirrors the ffmpeg component's phases).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FetchPhase {
    Downloading,
    Verifying,
    Extracting,
}

#[derive(Debug, Clone, Copy)]
pub struct FetchProgress {
    pub phase: FetchPhase,
    pub received: u64,
    pub total: Option<u64>,
    pub bytes_per_sec: u64,
}

/// Download `build`, verify its SHA-1, and extract it. Blocking — run on a
/// worker thread. `cancel` aborts between chunks (the partial file is removed).
pub fn install(
    build: &CefBuild,
    mut progress: impl FnMut(FetchProgress),
    cancel: &AtomicBool,
) -> Result<CefRuntime, CefError> {
    // Already installed → reuse it; never re-download (idempotent, like ffmpeg).
    // To update to a newer build, `remove()` first, then install.
    if let Some(ready) = installed() {
        return Ok(ready);
    }
    let root = cache_root()
        .ok_or_else(|| CefError::Http("no per-user cache directory could be resolved".into()))?;
    let current = root.join("current");
    // A fresh install replaces any prior runtime.
    if current.exists() {
        fs::remove_dir_all(&current)?;
    }
    fs::create_dir_all(&current)?;
    let partial = root.join("download.partial");

    let result = download_and_verify(build, &partial, &mut progress, cancel);
    if result.is_err() {
        let _ = fs::remove_file(&partial);
        result?;
    }

    progress(FetchProgress {
        phase: FetchPhase::Extracting,
        received: build.size_bytes,
        total: Some(build.size_bytes),
        bytes_per_sec: 0,
    });
    let extracted = extract_tar_bz2(&partial, &current);
    let _ = fs::remove_file(&partial);
    extracted?;

    // The archive's single top-level dir is the runtime dir.
    let dir = top_level_dir(&current)?;
    let marker = Marker {
        sha1: build.sha1.clone(),
        name: build.name.clone(),
        cef_version: build.cef_version.clone(),
        dir: dir.clone(),
    };
    fs::write(
        current.join("verified.json"),
        serde_json::to_string_pretty(&marker).expect("marker serializes"),
    )?;
    tracing::info!(version = %build.cef_version, "CEF runtime installed + verified");
    Ok(CefRuntime {
        runtime_dir: current.join(dir),
        cef_version: build.cef_version.clone(),
    })
}

fn download_and_verify(
    build: &CefBuild,
    partial: &Path,
    progress: &mut impl FnMut(FetchProgress),
    cancel: &AtomicBool,
) -> Result<(), CefError> {
    let agent = ureq::AgentBuilder::new()
        .timeout_connect(Duration::from_secs(30))
        .timeout_read(Duration::from_secs(60))
        .build();
    let response = agent
        .get(&build.url)
        .call()
        .map_err(|err| CefError::Http(err.to_string()))?;
    let total = response
        .header("Content-Length")
        .and_then(|value| value.parse::<u64>().ok())
        .or(Some(build.size_bytes));
    let mut reader = response.into_reader();

    let mut out = File::create(partial)?;
    let mut hasher = Sha1::new();
    let mut received = 0u64;
    let mut buffer = vec![0u8; 256 * 1024];
    let started = Instant::now();
    let mut last_emit = Instant::now() - Duration::from_secs(1);

    loop {
        if cancel.load(Ordering::Relaxed) {
            return Err(CefError::Cancelled);
        }
        let read = reader
            .read(&mut buffer)
            .map_err(|err| CefError::Http(err.to_string()))?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
        out.write_all(&buffer[..read])?;
        received += read as u64;
        if last_emit.elapsed() >= Duration::from_millis(100) {
            let elapsed = started.elapsed().as_secs_f64().max(0.001);
            progress(FetchProgress {
                phase: FetchPhase::Downloading,
                received,
                total,
                bytes_per_sec: (received as f64 / elapsed) as u64,
            });
            last_emit = Instant::now();
        }
    }
    out.sync_all()?;
    drop(out);

    progress(FetchProgress {
        phase: FetchPhase::Verifying,
        received,
        total,
        bytes_per_sec: 0,
    });
    let got = hex(&hasher.finalize());
    if !got.eq_ignore_ascii_case(&build.sha1) {
        return Err(CefError::HashMismatch {
            expected: build.sha1.clone(),
            got,
        });
    }
    Ok(())
}

fn hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

/// Extract a `.tar.bz2` into `target_dir`. `MultiBzDecoder` handles CEF's
/// multi-stream bzip2 (concatenated streams — a single-stream decoder would stop
/// after the first and truncate the tar). `tar`'s `unpack` sanitizes member
/// paths (traversal entries are refused), and the archive was SHA-1-verified.
fn extract_tar_bz2(archive: &Path, target_dir: &Path) -> Result<(), CefError> {
    let file = File::open(archive)?;
    let decoder = bzip2::read::MultiBzDecoder::new(BufReader::new(file));
    let mut tar = tar::Archive::new(decoder);
    tar.unpack(target_dir)
        .map_err(|err| CefError::Archive(err.to_string()))?;
    Ok(())
}

/// The single top-level directory the CEF archive unpacked into.
fn top_level_dir(dir: &Path) -> Result<String, CefError> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            if let Some(name) = entry.file_name().to_str() {
                if name.starts_with("cef_binary") {
                    return Ok(name.to_string());
                }
            }
        }
    }
    Err(CefError::Archive(
        "the CEF archive had no cef_binary_* directory".into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn platform_key_is_a_known_cdn_key_or_none() {
        // Whatever this build target is, the key (if any) is one the CDN uses.
        if let Some(key) = platform_key() {
            assert!(matches!(
                key,
                "windows64" | "windowsarm64" | "linux64" | "linuxarm64" | "macosarm64" | "macosx64"
            ));
        }
    }

    #[test]
    fn chromium_ord_parses_and_orders() {
        let a = chromium_ord("119.4.7+g55e15c8+chromium-119.0.6045.124");
        let b = chromium_ord("120.1.0+gabcdef0+chromium-120.0.6099.109");
        assert_eq!(a, (119, 0, 6045, 124));
        assert!(b > a, "newer chromium sorts higher");
        assert_eq!(chromium_ord("no-chromium-here"), (0, 0, 0, 0));
    }

    #[test]
    fn urlencode_encodes_plus() {
        assert_eq!(
            urlencode_name("cef_binary_1+2+chromium.tar.bz2"),
            "cef_binary_1%2B2%2Bchromium.tar.bz2"
        );
    }

    #[test]
    fn pick_build_takes_newest_stable_minimal_with_a_real_sha1() {
        let json = r#"{
          "windows64": { "versions": [
            { "cef_version": "118.0.0+gaaaa+chromium-118.0.5993.88", "channel": "stable",
              "files": [ {"name":"old_minimal.tar.bz2","sha1":"1111111111111111111111111111111111111111","size":100,"type":"minimal"} ] },
            { "cef_version": "120.0.0+gbbbb+chromium-120.0.6099.109", "channel": "stable",
              "files": [
                {"name":"new_standard.tar.bz2","sha1":"2222222222222222222222222222222222222222","size":200,"type":"standard"},
                {"name":"new+minimal.tar.bz2","sha1":"3333333333333333333333333333333333333333","size":150,"type":"minimal"}
              ] },
            { "cef_version": "121.0.0+gcccc+chromium-121.0.6167.85", "channel": "beta",
              "files": [ {"name":"beta_minimal.tar.bz2","sha1":"4444444444444444444444444444444444444444","size":150,"type":"minimal"} ] }
          ] }
        }"#;
        let index: Index = serde_json::from_str(json).unwrap();
        let build = pick_build(&index, "windows64").expect("resolves a build");
        // Newest STABLE (120, not the 121 beta), the minimal file, sha1 intact,
        // and the URL has the + encoded.
        assert_eq!(build.cef_version, "120.0.0+gbbbb+chromium-120.0.6099.109");
        assert_eq!(build.sha1, "3333333333333333333333333333333333333333");
        assert_eq!(build.size_bytes, 150);
        assert_eq!(build.url, format!("{FILE_BASE}new%2Bminimal.tar.bz2"));
    }

    #[test]
    fn pick_build_is_none_for_unknown_platform() {
        let index: Index = serde_json::from_str(r#"{"windows64":{"versions":[]}}"#).unwrap();
        assert!(pick_build(&index, "solaris").is_none());
    }

    /// The real thing (network + ~100 MB): resolve the newest stable build from
    /// the CEF index, download it, SHA-1-verify, extract, and confirm the runnable
    /// runtime files + idempotency. On demand:
    /// `cargo test -p fcap-encode --release -- --ignored --nocapture cef_end_to_end_download`
    #[test]
    #[ignore = "network + ~100 MB download — run on demand"]
    fn cef_end_to_end_download() {
        // Start clean so we exercise a real download, not a cached short-circuit.
        let _ = remove();
        let build = resolve_build().expect("resolve a build from the CEF index");
        eprintln!(
            "resolved CEF {} ({} MB): {}",
            build.cef_version,
            build.size_bytes / 1_000_000,
            build.url
        );
        assert_eq!(build.sha1.len(), 40, "index carries a real sha1");

        let cancel = AtomicBool::new(false);
        let mut last = 0u64;
        let ready = install(
            &build,
            |p| {
                if p.received.saturating_sub(last) > 20_000_000 {
                    last = p.received;
                    eprintln!(
                        "  {:?}: {} MB ({} MB/s)",
                        p.phase,
                        p.received / 1_000_000,
                        p.bytes_per_sec / 1_000_000
                    );
                }
            },
            &cancel,
        )
        .expect("CEF install succeeds (download + sha1 verify + extract)");

        eprintln!(
            "installed CEF {} at {}",
            ready.cef_version,
            ready.runtime_dir.display()
        );
        assert!(ready.runtime_dir.is_dir());
        // The minimal distribution must carry the runnable Release binaries.
        let release = ready.runtime_dir.join("Release");
        assert!(release.is_dir(), "Release/ present");
        assert!(ready.runtime_dir.join("Resources").is_dir(), "Resources/");
        let has_libcef = fs::read_dir(&release).unwrap().flatten().any(|e| {
            let n = e.file_name().to_string_lossy().to_lowercase();
            n.contains("libcef") || n.contains("chromium embedded framework")
        });
        assert!(has_libcef, "libcef present in Release/");
        assert!(installed().is_some(), "marker round-trips");

        // Idempotent: a second install returns the SAME runtime, never
        // re-downloading (the progress closure must never fire).
        let again = install(
            &build,
            |_| panic!("must not re-download an already-installed runtime"),
            &AtomicBool::new(false),
        )
        .expect("idempotent install");
        assert_eq!(again.runtime_dir, ready.runtime_dir);
    }
}
