//! # fcap-appaudio — per-application audio capture (Phase 8, TASK-805)
//!
//! Capture **one application's** audio as its own mixer source.
//!
//! **Per-OS reality, told honestly (the charter):**
//! - **Windows** (10 build 2004+ / 19041): first-class — WASAPI **process
//!   loopback** (`ActivateAudioInterfaceAsync` with a process-loopback
//!   activation) captures exactly the target process tree's render audio,
//!   nothing else. [`list_audio_apps`] enumerates the apps currently playing.
//! - **Linux**: PipeWire can route one app's output to a null sink you then
//!   capture; there's no single-call per-app grab, so we return honest
//!   guidance (route in `pavucontrol`/`qpwgraph`, capture the null-sink
//!   monitor) rather than a fake toggle.
//! - **macOS**: no public per-application audio API without a virtual device;
//!   said plainly.
//!
//! The intricate WASAPI COM `unsafe` is isolated in the `windows` submodule so
//! `fcap-audio` stays `#![forbid(unsafe_code)]`; this crate hands the app raw
//! stereo/48k-agnostic f32 frames, which the app resamples + pushes into the
//! mixer's existing external-ring seam (the same path the Media source uses).

#![cfg_attr(not(target_os = "windows"), allow(dead_code))]

use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum AppAudioError {
    /// Per-app capture isn't available on this OS/build — the message is the
    /// honest per-OS guidance (never a fake success).
    #[error("{0}")]
    Unsupported(String),
    #[error("no application with process id {0} is producing audio")]
    AppNotFound(u32),
    #[error("per-app audio backend error: {0}")]
    Backend(String),
}

/// One application currently producing render audio.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioApp {
    /// The OS process id — the stable capture key within a session.
    pub pid: u32,
    /// A human label (the session's display name, or the exe stem).
    pub name: String,
    /// The executable file name (e.g. `chrome.exe`), for a durable match.
    pub exe: String,
}

/// The honest per-OS message when single-call per-app capture isn't available.
pub fn per_app_guidance() -> String {
    if cfg!(target_os = "windows") {
        "Per-app audio needs Windows 10 build 2004 (19041) or newer.".to_string()
    } else if cfg!(target_os = "macos") {
        "macOS has no public per-application audio API — route the app through a \
         virtual audio device (e.g. BlackHole) and add that as an Audio Input."
            .to_string()
    } else {
        "Linux has no single-call per-app capture — in pavucontrol/qpwgraph route \
         the app's output to a null sink, then add that sink's monitor as an \
         Audio Input."
            .to_string()
    }
}

/// Apps currently producing audio (Windows only; elsewhere an honest error the
/// UI turns into the guidance above). Sorted by name for stable pickers.
pub fn list_audio_apps() -> Result<Vec<AudioApp>, AppAudioError> {
    #[cfg(target_os = "windows")]
    {
        windows::list_audio_apps()
    }
    #[cfg(not(target_os = "windows"))]
    {
        Err(AppAudioError::Unsupported(per_app_guidance()))
    }
}

/// A running per-app capture; dropping it stops the capture + releases WASAPI.
pub struct AppCapture {
    #[cfg(target_os = "windows")]
    _inner: windows::ProcessCapture,
    #[cfg(not(target_os = "windows"))]
    _unsupported: (),
}

/// Start capturing process `pid`'s render audio. `on_frames(samples, rate,
/// channels)` is called on the capture thread with interleaved f32 at the
/// device's native rate/layout; the caller resamples to the mix format. The
/// callback must not block. Dropping the returned [`AppCapture`] stops it.
pub fn start_capture(
    pid: u32,
    on_frames: impl FnMut(&[f32], u32, u16) + Send + 'static,
) -> Result<AppCapture, AppAudioError> {
    #[cfg(target_os = "windows")]
    {
        Ok(AppCapture {
            _inner: windows::ProcessCapture::start(pid, on_frames)?,
        })
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = (pid, on_frames);
        Err(AppAudioError::Unsupported(per_app_guidance()))
    }
}

/// This crate's version (inherited from the workspace).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(target_os = "windows")]
mod windows;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guidance_is_honest_per_os() {
        let g = per_app_guidance();
        assert!(!g.is_empty());
        // Never claims a capability the OS doesn't have.
        if cfg!(target_os = "macos") {
            assert!(g.contains("virtual audio device"));
        }
    }

    #[test]
    fn non_windows_capture_is_refused_honestly() {
        // On non-Windows the public entry points return Unsupported, never a
        // silent no-op that looks live.
        #[cfg(not(target_os = "windows"))]
        {
            assert!(matches!(
                start_capture(1234, |_, _, _| {}),
                Err(AppAudioError::Unsupported(_))
            ));
            assert!(matches!(list_audio_apps(), Err(AppAudioError::Unsupported(_))));
        }
    }

    #[test]
    fn version_is_a_semver_triple() {
        assert_eq!(VERSION.split('.').count(), 3);
    }
}
