//! # fcap-sources
//!
//! The source implementations Freally Capture composes onto the canvas:
//! webcam / capture card (via `nokhwa`), image, text (rustybuzz shaping +
//! bundled fonts, RTL), color, media (hardware-decoded video/image), browser
//! (embedded webview at a set resolution/fps), image slideshow, and source
//! groups. (No ML-based sources — AI features are excluded by charter.)
//!
//! Webcam / capture-card (Phase 1) lives in [`video_device`]. Phase 2 adds
//! the static sources: [`image`] (still files, also mask/LUT loading for the
//! filter chain), [`color`] (solid blocks), and [`text`] (rustybuzz shaping +
//! bidi RTL + tiny-skia rasterization over system fonts). Media (video files,
//! hardware-decoded) waits on the Phase 4 wire-codec architecture, and the
//! Browser source needs an offscreen-webview design — both stay out until
//! they can be real (no fakes, per the honesty invariant).

#![forbid(unsafe_code)]

pub mod color;
pub mod image;
pub mod static_source;
pub mod text;
pub mod video_device;

pub use static_source::{StaticSourceError, MAX_STATIC_DIMENSION};

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
