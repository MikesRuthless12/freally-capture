//! # fcap-capture
//!
//! Per-OS screen / window / (game) capture behind a single `Capture` interface:
//! **Windows** DXGI Desktop Duplication + Windows.Graphics.Capture (per-window),
//! **macOS** ScreenCaptureKit via `objc2`, **Linux** PipeWire portal via `ashpd`
//! plus an X11 path. Wayland capture is portal-only — the user picks the source;
//! that limit is surfaced honestly, never papered over.
//!
//! **Phase 0 stub** — the crate boundary exists; the capture pipeline lands in
//! Phase 1 (→ 0.25.0). When OS capture arrives, this crate moves from
//! `forbid(unsafe_code)` to `deny(unsafe_code)` with the unavoidable OS `unsafe`
//! isolated in small, audited `#[allow(unsafe_code)]` modules.

#![forbid(unsafe_code)]

/// This crate's version (inherited from the workspace).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::VERSION;

    #[test]
    fn version_is_a_semver_triple() {
        assert_eq!(
            VERSION.split('.').count(),
            3,
            "workspace version should be MAJOR.MINOR.PATCH"
        );
    }
}
