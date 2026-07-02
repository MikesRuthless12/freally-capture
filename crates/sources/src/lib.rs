//! # fcap-sources
//!
//! The source implementations Freally Capture composes onto the canvas:
//! webcam / capture card (via `nokhwa`), image, text (rustybuzz shaping +
//! bundled fonts, RTL), color, media (hardware-decoded video/image), browser
//! (embedded webview at a set resolution/fps), image slideshow, and source
//! groups. (No ML-based sources — AI features are excluded by charter.)
//!
//! **Phase 0 stub** — the crate boundary exists; webcam/capture-card lands in
//! Phase 1 (→ 0.25.0), the remaining sources in Phase 2 (→ 0.40.0).

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
