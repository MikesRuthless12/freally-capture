//! The **on-demand, clearly-labeled ffmpeg bridge** — the only road to the
//! patent-encumbered wire codecs (H.264/AAC/HEVC/AV1) the platforms require.
//!
//! ffmpeg is **never bundled**. When a wire codec is first needed, the exact
//! build pinned below — one per OS, with its **SHA-256 baked into this
//! source** and cross-checked against the publisher's own checksum at pin
//! time — is downloaded to the per-user cache, **hash-verified before
//! anything runs**, extracted, and driven as a **separate process** (which
//! also keeps its LGPL/GPL licensing isolated from the proprietary app; see
//! `THIRD-PARTY-NOTICES.md`). A mismatched hash aborts the install: the
//! bridge refuses to run bytes it cannot vouch for. The owned
//! `freally-video` path never touches any of this.
//!
//! The only network action here is the fetch of the pinned URL, started by
//! an explicit user click in the clearly-labeled Components panel — the
//! privacy invariant stands.

use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::encoder::Catalog;

/// How a pinned archive is packed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveKind {
    Zip,
    TarXz,
}

/// One pinned ffmpeg build: the exact URL + SHA-256 this app will accept.
#[derive(Debug, Clone)]
pub struct PinnedBuild {
    /// Version label shown to the user ("8.1.2").
    pub version: &'static str,
    /// The publisher, honestly named ("gyan.dev (essentials build)").
    pub source: &'static str,
    pub url: &'static str,
    /// Lowercase hex SHA-256 of the archive, verified before extraction.
    pub sha256: &'static str,
    /// Archive size (progress display; the server's Content-Length wins).
    pub size_bytes: u64,
    pub archive: ArchiveKind,
}

/// The build pinned for this OS/arch, or `None` where no build is pinned yet
/// (said honestly in the panel; the owned `.frec` path is unaffected).
pub fn pinned_build() -> Option<&'static PinnedBuild> {
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    {
        static PIN: PinnedBuild = PinnedBuild {
            version: "8.1.2",
            source: "gyan.dev (essentials build)",
            url: "https://www.gyan.dev/ffmpeg/builds/packages/ffmpeg-8.1.2-essentials_build.zip",
            sha256: "db580001caa24ac104c8cb856cd113a87b0a443f7bdf47d8c12b1d740584a2ec",
            size_bytes: 109_728_040,
            archive: ArchiveKind::Zip,
        };
        Some(&PIN)
    }
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
        static PIN: PinnedBuild = PinnedBuild {
            version: "8.1.2",
            source: "BtbN FFmpeg-Builds (linux64-gpl)",
            url: "https://github.com/BtbN/FFmpeg-Builds/releases/download/autobuild-2026-07-02-16-09/ffmpeg-n8.1.2-21-gce3c09c101-linux64-gpl-8.1.tar.xz",
            sha256: "1565cc9709a34cea919e4e1931c5e94497532d905ece28c92d85c791e81de484",
            size_bytes: 124_748_856,
            archive: ArchiveKind::TarXz,
        };
        Some(&PIN)
    }
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        static PIN: PinnedBuild = PinnedBuild {
            version: "8.1.2",
            source: "martin-riedl.de (macOS arm64)",
            url: "https://ffmpeg.martin-riedl.de/download/macos/arm64/1783011502_8.1.2/ffmpeg.zip",
            sha256: "ef1aa60006c7b77ce170c1608c08d8e4ba1c30c5746f2ac986ded932d0ac2c3c",
            size_bytes: 28_196_358,
            archive: ArchiveKind::Zip,
        };
        Some(&PIN)
    }
    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    {
        static PIN: PinnedBuild = PinnedBuild {
            version: "8.1.2",
            source: "martin-riedl.de (macOS Intel)",
            url: "https://ffmpeg.martin-riedl.de/download/macos/amd64/1783018342_8.1.2/ffmpeg.zip",
            sha256: "a52ef43883f44c219766d4b3bdde4e635b35465d0b704c01c3a0566b59775df9",
            size_bytes: 33_586_778,
            archive: ArchiveKind::Zip,
        };
        Some(&PIN)
    }
    #[cfg(not(any(
        all(target_os = "windows", target_arch = "x86_64"),
        all(target_os = "linux", target_arch = "x86_64"),
        all(target_os = "macos", target_arch = "aarch64"),
        all(target_os = "macos", target_arch = "x86_64"),
    )))]
    {
        None
    }
}

#[derive(Debug, Error)]
pub enum FfmpegError {
    #[error("no ffmpeg build is pinned for this platform yet")]
    Unsupported,
    #[error("download failed: {0}")]
    Http(String),
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
    #[error(
        "the downloaded archive does not match the pinned checksum — refusing to install it \
         (expected {expected}, got {got}). Retry later; if this persists the pinned build may \
         have been re-published and the app needs an update."
    )]
    HashMismatch { expected: String, got: String },
    #[error("could not unpack the archive: {0}")]
    Archive(String),
    #[error("the download was cancelled")]
    Cancelled,
    #[error("the installed ffmpeg did not respond: {0}")]
    Probe(String),
}

/// A ready, verified ffmpeg installation.
#[derive(Debug, Clone)]
pub struct Ffmpeg {
    pub path: PathBuf,
    /// First line of `ffmpeg -version`, recorded at install time.
    pub version: String,
}

/// Written next to the extracted binary after a successful verify+extract.
#[derive(Debug, Serialize, Deserialize)]
struct Marker {
    sha256: String,
    url: String,
    version: String,
}

fn cache_root() -> Option<PathBuf> {
    directories::ProjectDirs::from("com", "Freally", "Freally Capture")
        .map(|dirs| dirs.cache_dir().join("ffmpeg"))
}

fn binary_name() -> &'static str {
    if cfg!(windows) {
        "ffmpeg.exe"
    } else {
        "ffmpeg"
    }
}

/// The installed-and-verified ffmpeg, if any. Reads the install marker —
/// no process is spawned, so this is cheap enough for status queries.
pub fn installed() -> Option<Ffmpeg> {
    let pinned = pinned_build()?;
    let dir = cache_root()?.join(pinned.version);
    let path = dir.join(binary_name());
    if !path.is_file() {
        return None;
    }
    let marker: Marker =
        serde_json::from_str(&fs::read_to_string(dir.join("verified.json")).ok()?).ok()?;
    if marker.sha256 != pinned.sha256 {
        // A leftover from an older pin — treat as not installed.
        return None;
    }
    Some(Ffmpeg {
        path,
        version: marker.version,
    })
}

/// Delete the installed component (the labeled panel's Remove action).
pub fn remove() -> std::io::Result<()> {
    if let Some(root) = cache_root() {
        if root.exists() {
            fs::remove_dir_all(root)?;
        }
    }
    Ok(())
}

/// Where a fetch currently is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FetchPhase {
    Downloading,
    Verifying,
    Extracting,
}

/// Progress for the panel: received/total bytes + a smoothed rate.
#[derive(Debug, Clone, Copy)]
pub struct FetchProgress {
    pub phase: FetchPhase,
    pub received: u64,
    pub total: Option<u64>,
    pub bytes_per_sec: u64,
}

/// Download, verify, and install the pinned build. Blocking — run it on a
/// worker thread. `progress` is throttled to ~10 Hz; `cancel` aborts between
/// chunks (the partial file is removed).
pub fn install(
    mut progress: impl FnMut(FetchProgress),
    cancel: &AtomicBool,
) -> Result<Ffmpeg, FfmpegError> {
    let pinned = pinned_build().ok_or(FfmpegError::Unsupported)?;
    if let Some(ready) = installed() {
        return Ok(ready);
    }
    let root = cache_root()
        .ok_or_else(|| FfmpegError::Http("no per-user cache directory could be resolved".into()))?;
    fs::create_dir_all(&root)?;
    let partial = root.join("download.partial");

    let result = download_and_verify(pinned, &partial, &mut progress, cancel);
    if result.is_err() {
        let _ = fs::remove_file(&partial);
        result?;
    }

    progress(FetchProgress {
        phase: FetchPhase::Extracting,
        received: pinned.size_bytes,
        total: Some(pinned.size_bytes),
        bytes_per_sec: 0,
    });
    let dir = root.join(pinned.version);
    fs::create_dir_all(&dir)?;
    let bin = dir.join(binary_name());
    let extracted = extract_ffmpeg(pinned.archive, &partial, &bin);
    let _ = fs::remove_file(&partial);
    extracted?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&bin, fs::Permissions::from_mode(0o755))?;
    }

    // Prove the binary answers before declaring it ready.
    let version = probe_version(&bin)?;
    let marker = Marker {
        sha256: pinned.sha256.to_string(),
        url: pinned.url.to_string(),
        version: version.clone(),
    };
    fs::write(
        dir.join("verified.json"),
        serde_json::to_string_pretty(&marker).expect("marker serializes"),
    )?;
    tracing::info!(%version, "ffmpeg component installed + verified");
    Ok(Ffmpeg { path: bin, version })
}

/// Stream the pinned URL to `partial`, hashing as it lands; error on any
/// mismatch. Network reads are chunked so cancel stays responsive.
fn download_and_verify(
    pinned: &PinnedBuild,
    partial: &Path,
    progress: &mut impl FnMut(FetchProgress),
    cancel: &AtomicBool,
) -> Result<(), FfmpegError> {
    let agent = ureq::AgentBuilder::new()
        .timeout_connect(Duration::from_secs(30))
        .timeout_read(Duration::from_secs(60))
        .build();
    let response = agent
        .get(pinned.url)
        .call()
        .map_err(|err| FfmpegError::Http(err.to_string()))?;
    let total = response
        .header("Content-Length")
        .and_then(|value| value.parse::<u64>().ok())
        .or(Some(pinned.size_bytes));
    let mut reader = response.into_reader();

    let mut out = File::create(partial)?;
    let mut hasher = Sha256::new();
    let mut received = 0u64;
    let mut buffer = vec![0u8; 256 * 1024];
    let started = Instant::now();
    let mut last_emit = Instant::now() - Duration::from_secs(1);

    loop {
        if cancel.load(Ordering::Relaxed) {
            return Err(FfmpegError::Cancelled);
        }
        let read = reader
            .read(&mut buffer)
            .map_err(|err| FfmpegError::Http(err.to_string()))?;
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
    if !got.eq_ignore_ascii_case(pinned.sha256) {
        return Err(FfmpegError::HashMismatch {
            expected: pinned.sha256.to_string(),
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

/// Pull the one `ffmpeg` binary out of the verified archive. The output path
/// is always ours — archive-supplied paths are never used for writing, so
/// path-traversal entries are inert.
fn extract_ffmpeg(kind: ArchiveKind, archive: &Path, target: &Path) -> Result<(), FfmpegError> {
    match kind {
        ArchiveKind::Zip => extract_from_zip(archive, target),
        ArchiveKind::TarXz => extract_from_tar_xz(archive, target),
    }
}

fn is_ffmpeg_member(path: &Path) -> bool {
    matches!(
        path.file_name().and_then(|name| name.to_str()),
        Some("ffmpeg") | Some("ffmpeg.exe")
    )
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn extract_from_zip(archive: &Path, target: &Path) -> Result<(), FfmpegError> {
    let file = File::open(archive)?;
    let mut zip =
        zip::ZipArchive::new(file).map_err(|err| FfmpegError::Archive(err.to_string()))?;
    for index in 0..zip.len() {
        let mut member = zip
            .by_index(index)
            .map_err(|err| FfmpegError::Archive(err.to_string()))?;
        if member.is_file() && is_ffmpeg_member(Path::new(member.name())) {
            let mut out = File::create(target)?;
            std::io::copy(&mut member, &mut out)?;
            out.sync_all()?;
            return Ok(());
        }
    }
    Err(FfmpegError::Archive(
        "no ffmpeg binary inside the archive".into(),
    ))
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn extract_from_zip(_archive: &Path, _target: &Path) -> Result<(), FfmpegError> {
    Err(FfmpegError::Archive(
        "zip archives are not expected on this platform".into(),
    ))
}

/// BtbN's Linux archive is tar-in-xz: stream-decode the xz layer to a
/// sibling temp file (bounded by disk, not RAM), then pull the one member.
#[cfg(target_os = "linux")]
fn extract_from_tar_xz(archive: &Path, target: &Path) -> Result<(), FfmpegError> {
    let tar_path = archive.with_extension("tar.tmp");
    let result = (|| -> Result<(), FfmpegError> {
        {
            let mut reader = std::io::BufReader::new(File::open(archive)?);
            let mut writer = std::io::BufWriter::new(File::create(&tar_path)?);
            lzma_rs::xz_decompress(&mut reader, &mut writer)
                .map_err(|err| FfmpegError::Archive(format!("xz: {err:?}")))?;
            writer.flush()?;
        }
        let mut tar = tar::Archive::new(File::open(&tar_path)?);
        for entry in tar
            .entries()
            .map_err(|err| FfmpegError::Archive(err.to_string()))?
        {
            let mut entry = entry.map_err(|err| FfmpegError::Archive(err.to_string()))?;
            let path = entry
                .path()
                .map_err(|err| FfmpegError::Archive(err.to_string()))?
                .into_owned();
            if entry.header().entry_type().is_file() && is_ffmpeg_member(&path) {
                let mut out = File::create(target)?;
                std::io::copy(&mut entry, &mut out)?;
                out.sync_all()?;
                return Ok(());
            }
        }
        Err(FfmpegError::Archive(
            "no ffmpeg binary inside the archive".into(),
        ))
    })();
    let _ = fs::remove_file(&tar_path);
    result
}

#[cfg(not(target_os = "linux"))]
fn extract_from_tar_xz(_archive: &Path, _target: &Path) -> Result<(), FfmpegError> {
    Err(FfmpegError::Archive(
        "tar.xz archives are not expected on this platform".into(),
    ))
}

// ---------------------------------------------------------------------------
// Driving the installed binary
// ---------------------------------------------------------------------------

/// A pre-configured command for the installed binary: stdin closed, and on
/// Windows **no console window ever flashes** (the app is a GUI app — child
/// processes must honor that too).
pub fn command(ffmpeg: &Ffmpeg) -> Command {
    let mut cmd = Command::new(&ffmpeg.path);
    cmd.stdin(Stdio::null());
    hide_window(&mut cmd);
    cmd
}

fn hide_window(cmd: &mut Command) {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    #[cfg(not(windows))]
    {
        let _ = cmd;
    }
}

/// What the installed ffmpeg can stream beyond RTMP (Phase 6): probed
/// honestly from its own `-protocols` / `-muxers` listings at Go Live —
/// never assumed from the pinned build's feature list.
#[derive(Debug, Clone, Copy)]
pub struct StreamSupport {
    /// The `srt` protocol (needs a libsrt-enabled build).
    pub srt: bool,
    /// The `whip` muxer (needs an ffmpeg 7.1+ build with DTLS support).
    pub whip: bool,
}

/// Probe the installed binary's SRT/WHIP support, **memoized per binary
/// path + version**: the answer cannot change without a component
/// reinstall, so repeat Go Lives are instant instead of re-spawning two
/// hard-bounded listing children (which could stall ~30 s where child
/// spawns are slow, e.g. AV interception).
pub fn stream_support(ffmpeg: &Ffmpeg) -> StreamSupport {
    use std::sync::{Mutex, OnceLock};
    static CACHE: OnceLock<Mutex<std::collections::HashMap<(PathBuf, String), StreamSupport>>> =
        OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new(std::collections::HashMap::new()));
    let key = (ffmpeg.path.clone(), ffmpeg.version.clone());
    if let Some(support) = cache
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .get(&key)
    {
        return *support;
    }
    let support = StreamSupport {
        srt: protocol_listed(&capability_listing(ffmpeg, "-protocols"), "srt"),
        whip: muxer_listed(&capability_listing(ffmpeg, "-muxers"), "whip"),
    };
    cache
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .insert(key, support);
    support
}

fn capability_listing(ffmpeg: &Ffmpeg, flag: &str) -> String {
    let mut cmd = command(ffmpeg);
    cmd.args(["-hide_banner", flag]);
    match run_with_timeout(cmd, Duration::from_secs(15)) {
        Ok(output) => String::from_utf8_lossy(&output.stdout).into_owned(),
        Err(_) => String::new(),
    }
}

/// `-protocols` lists one bare protocol name per line under `Input:` /
/// `Output:` headers — only an **Output** entry means we can publish.
fn protocol_listed(listing: &str, name: &str) -> bool {
    let mut in_output = false;
    for line in listing.lines() {
        let token = line.trim();
        if token.eq_ignore_ascii_case("Output:") {
            in_output = true;
            continue;
        }
        if in_output && token == name {
            return true;
        }
    }
    false
}

/// `-muxers` lines read ` E name  Description` (`E` = muxing supported).
fn muxer_listed(listing: &str, name: &str) -> bool {
    for line in listing.lines() {
        let mut tokens = line.split_whitespace();
        let (Some(flags), Some(names)) = (tokens.next(), tokens.next()) else {
            continue;
        };
        if flags == "E" && names.split(',').any(|n| n == name) {
            return true;
        }
    }
    false
}

/// Run a command to completion with a hard timeout (a wedged driver must
/// never hang the app). Output is drained on threads so full pipes can't
/// deadlock the child.
pub fn run_with_timeout(
    mut cmd: Command,
    timeout: Duration,
) -> Result<std::process::Output, String> {
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = cmd.spawn().map_err(|err| format!("spawn failed: {err}"))?;
    let mut stdout_pipe = child.stdout.take().expect("stdout piped");
    let mut stderr_pipe = child.stderr.take().expect("stderr piped");
    let stdout_thread = std::thread::spawn(move || {
        let mut buffer = Vec::new();
        let _ = stdout_pipe.read_to_end(&mut buffer);
        buffer
    });
    let stderr_thread = std::thread::spawn(move || {
        let mut buffer = Vec::new();
        let _ = stderr_pipe.read_to_end(&mut buffer);
        buffer
    });

    let started = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) => {
                if started.elapsed() > timeout {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(format!("timed out after {} s", timeout.as_secs()));
                }
                std::thread::sleep(Duration::from_millis(25));
            }
            Err(err) => return Err(format!("wait failed: {err}")),
        }
    };
    Ok(std::process::Output {
        status,
        stdout: stdout_thread.join().unwrap_or_default(),
        stderr: stderr_thread.join().unwrap_or_default(),
    })
}

fn probe_version(path: &Path) -> Result<String, FfmpegError> {
    let mut cmd = Command::new(path);
    cmd.stdin(Stdio::null()).arg("-version");
    hide_window(&mut cmd);
    let output = run_with_timeout(cmd, Duration::from_secs(15)).map_err(FfmpegError::Probe)?;
    if !output.status.success() {
        return Err(FfmpegError::Probe(format!(
            "-version exited with {}",
            output.status
        )));
    }
    let first = String::from_utf8_lossy(&output.stdout);
    Ok(first
        .lines()
        .next()
        .unwrap_or("ffmpeg (unknown)")
        .to_string())
}

/// The encoder names the installed build ships (`ffmpeg -encoders`).
pub fn list_encoders(ffmpeg: &Ffmpeg) -> Result<HashSet<String>, FfmpegError> {
    let mut cmd = command(ffmpeg);
    cmd.args(["-hide_banner", "-encoders"]);
    let output = run_with_timeout(cmd, Duration::from_secs(15)).map_err(FfmpegError::Probe)?;
    if !output.status.success() {
        return Err(FfmpegError::Probe(format!(
            "-encoders exited with {}",
            output.status
        )));
    }
    let text = String::from_utf8_lossy(&output.stdout);
    Ok(parse_encoder_list(&text))
}

/// Parse `-encoders` output: ` V....D h264_nvenc   NVIDIA NVENC…` lines.
fn parse_encoder_list(text: &str) -> HashSet<String> {
    text.lines()
        .filter_map(|line| {
            let mut fields = line.split_whitespace();
            let flags = fields.next()?;
            let name = fields.next()?;
            (flags.starts_with('V') || flags.starts_with('A')).then(|| name.to_string())
        })
        .collect()
}

/// Encode 3 tiny synthetic frames with `id` and throw them away — the honest
/// answer to "does this encoder actually work on this machine/driver?".
pub fn smoke_test_encoder(ffmpeg: &Ffmpeg, id: &str) -> Result<(), String> {
    let mut cmd = command(ffmpeg);
    cmd.args(["-hide_banner", "-v", "error", "-f", "lavfi", "-i"]);
    cmd.arg("color=c=black:s=256x144:r=30");
    if id.ends_with("_vaapi") {
        // VAAPI encodes hardware frames: bring up the device + upload.
        cmd.args([
            "-init_hw_device",
            "vaapi=va",
            "-filter_hw_device",
            "va",
            "-vf",
            "format=nv12,hwupload",
        ]);
    }
    cmd.args(["-frames:v", "3", "-c:v", id, "-f", "null", "-"]);
    let output = run_with_timeout(cmd, Duration::from_secs(20))?;
    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let reason = stderr
            .lines()
            .find(|line| !line.trim().is_empty())
            .unwrap_or("the encoder failed to initialize")
            .trim();
        Err(reason.chars().take(200).collect())
    }
}

/// Fill in the catalog's `verified` column against the installed build:
/// entries the build doesn't ship are refused; hardware entries are
/// smoke-tested (support varies by GPU generation + driver — the only honest
/// answer is to try); software entries the build ships just work.
pub fn verify_catalog(catalog: &mut Catalog, ffmpeg: &Ffmpeg) -> Result<(), FfmpegError> {
    let shipped = list_encoders(ffmpeg)?;
    for encoder in &mut catalog.encoders {
        if !shipped.contains(&encoder.id) {
            encoder.verified = Some(false);
            encoder.note = format!(
                "Not shipped in the installed ffmpeg component ({}).",
                ffmpeg.version
            );
            continue;
        }
        if encoder.hardware {
            match smoke_test_encoder(ffmpeg, &encoder.id) {
                Ok(()) => {
                    encoder.verified = Some(true);
                    encoder.note.push_str(" Confirmed on this machine.");
                }
                Err(reason) => {
                    encoder.verified = Some(false);
                    encoder
                        .note
                        .push_str(&format!(" Unavailable here: {reason}"));
                }
            }
        } else {
            encoder.verified = Some(true);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_pinned_build_exists_on_release_platforms() {
        // On every OS this workspace CI-builds for, a pin must resolve —
        // otherwise the panel would dead-end. (Non-x86_64 Windows/Linux are
        // honestly unpinned for now.)
        if cfg!(any(
            all(target_os = "windows", target_arch = "x86_64"),
            all(target_os = "linux", target_arch = "x86_64"),
            target_os = "macos",
        )) {
            let pin = pinned_build().expect("a build is pinned here");
            assert_eq!(pin.sha256.len(), 64, "sha256 is 32 hex bytes");
            assert!(pin.sha256.chars().all(|c| c.is_ascii_hexdigit()));
            assert!(pin.url.starts_with("https://"), "pins are https-only");
            assert!(pin.size_bytes > 10_000_000);
        }
    }

    #[test]
    fn protocol_listing_only_counts_the_output_section() {
        let sample = "Supported file protocols:\nInput:\n  async\n  srtp\nOutput:\n  crypto\n  rtmp\n  srt\n  tee\n";
        assert!(protocol_listed(sample, "srt"));
        assert!(protocol_listed(sample, "rtmp"));
        assert!(
            !protocol_listed(sample, "async"),
            "input-only protocols can't publish"
        );
        assert!(!protocol_listed(sample, "srtp"), "srtp is not srt");
        assert!(!protocol_listed(sample, "whip"));
    }

    #[test]
    fn muxer_listing_reads_the_e_column() {
        let sample = "File formats:\n D. = Demuxing supported\n .E = Muxing supported\n --\n  E flv             FLV (Flash Video)\n  E whip            WHIP (WebRTC-HTTP ingestion protocol) muxer\n D  whip_demux_only fake\n";
        assert!(muxer_listed(sample, "whip"));
        assert!(muxer_listed(sample, "flv"));
        assert!(!muxer_listed(sample, "mpegts"));
        assert!(
            !muxer_listed(sample, "whip_demux_only"),
            "demux-only rows don't count"
        );
    }

    #[test]
    fn encoder_list_parsing_reads_video_and_audio_names() {
        let sample = "Encoders:\n V..... = Video\n ------\n V....D libx264              libx264 H.264\n V....D h264_nvenc           NVIDIA NVENC H.264 encoder\n A....D aac                  AAC (Advanced Audio Coding)\n S..... dvbsub               DVB subtitles\n";
        let names = parse_encoder_list(sample);
        assert!(names.contains("libx264"));
        assert!(names.contains("h264_nvenc"));
        assert!(names.contains("aac"));
        assert!(!names.contains("dvbsub"), "subtitle encoders are ignored");
        assert!(!names.contains("Encoders:"));
    }

    #[test]
    fn ffmpeg_member_matching_is_exact() {
        assert!(is_ffmpeg_member(Path::new(
            "ffmpeg-8.1.2-essentials_build/bin/ffmpeg.exe"
        )));
        assert!(is_ffmpeg_member(Path::new("ffmpeg")));
        assert!(!is_ffmpeg_member(Path::new("bin/ffprobe.exe")));
        assert!(!is_ffmpeg_member(Path::new("docs/ffmpeg.txt")));
    }

    #[test]
    fn hex_encodes_lowercase() {
        assert_eq!(hex(&[0xDB, 0x58, 0x00, 0x01]), "db580001");
    }

    /// The real thing, end-to-end: fetch the pinned build (~100 MB),
    /// hash-verify, extract, probe, then verify the encoder catalog against
    /// this machine's GPU (smoke encodes). Network + hardware — on demand:
    /// `cargo test -p fcap-encode --release -- --ignored --nocapture`
    #[test]
    #[ignore = "network + ~100 MB download — run on demand"]
    fn ffmpeg_end_to_end_install_and_verify() {
        let cancel = AtomicBool::new(false);
        let mut last = 0u64;
        let ready = install(
            |progress| {
                if progress.received - last > 20_000_000 {
                    last = progress.received;
                    eprintln!(
                        "  {:?}: {} MB ({} MB/s)",
                        progress.phase,
                        progress.received / 1_000_000,
                        progress.bytes_per_sec / 1_000_000
                    );
                }
            },
            &cancel,
        )
        .expect("install succeeds");
        eprintln!("installed: {} at {}", ready.version, ready.path.display());
        assert!(ready.path.is_file());

        let listed = list_encoders(&ready).expect("encoder list");
        assert!(listed.contains("libx264"), "x264 ships in the pinned build");
        assert!(listed.contains("aac"));

        let mut catalog = Catalog::detect();
        verify_catalog(&mut catalog, &ready).expect("catalog verifies");
        for encoder in &catalog.encoders {
            eprintln!(
                "  {:<18} hardware={} verified={:?}",
                encoder.id, encoder.hardware, encoder.verified
            );
        }
        assert_eq!(
            catalog
                .get("libx264")
                .expect("x264 is always offered")
                .verified,
            Some(true)
        );
        assert!(installed().is_some(), "marker round-trips");
    }

    #[cfg(any(target_os = "windows", target_os = "macos"))]
    #[test]
    fn zip_extraction_finds_the_binary_and_ignores_decoys() {
        use std::io::Write as _;
        let dir = std::env::temp_dir().join(format!(
            "fcap-ffmpeg-zip-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        ));
        fs::create_dir_all(&dir).expect("mkdir");
        let archive_path = dir.join("build.zip");
        {
            let file = File::create(&archive_path).expect("create zip");
            let mut writer = zip::ZipWriter::new(file);
            let options = zip::write::SimpleFileOptions::default();
            writer
                .start_file("build/docs/readme.txt", options)
                .expect("member");
            writer.write_all(b"not it").expect("write");
            writer
                .start_file(format!("build/bin/{}", binary_name()), options)
                .expect("member");
            writer.write_all(b"FAKE-FFMPEG-BYTES").expect("write");
            writer.finish().expect("finish zip");
        }
        let target = dir.join(binary_name());
        extract_from_zip(&archive_path, &target).expect("extracts");
        assert_eq!(fs::read(&target).expect("read"), b"FAKE-FFMPEG-BYTES");
        let _ = fs::remove_dir_all(&dir);
    }
}
