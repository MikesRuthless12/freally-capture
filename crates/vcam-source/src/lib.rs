//! CAP-N76 — the **virtual-camera frame-server source** (Windows).
//!
//! `MFCreateVirtualCamera` needs a *registered* media source that the OS
//! `frameserver.exe` loads; this crate is that source. It reads the app's
//! composed program frames from the `fcap_stream::vcam::transport` shared buffer
//! and serves them to whatever app (Zoom/Meet/Discord/OBS) opens the camera.
//! `design/vcam-source.md` is normative.
//!
//! Distribution: a per-user self-registering COM DLL (`DllRegisterServer` writes
//! `HKCU\Software\Classes\CLSID\{clsid}` — no admin). Signing is a release step,
//! not a build requirement; the source loads unsigned for the current user.
//!
//! The pure transport reader + color conversion (`reader`) is unit-tested on any
//! OS; the COM source (`win`) compiles and registers on Windows.

pub mod reader;

#[cfg(windows)]
pub mod win;

/// This crate's version (inherited from the workspace).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
