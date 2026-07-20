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
//! bidi RTL + tiny-skia rasterization over the **bundled Noto Sans family**
//! with per-run script fallback; system fonts stay selectable). Phase 4 adds [`media`]:
//! video/image files composed onto the canvas with their audio in the mixer —
//! `.frec` through the owned codec, the wire formats through the labeled
//! on-demand ffmpeg component, `-hwaccel auto` hardware decode. The Browser
//! source lands with the Phase 6 source depth after an offscreen-webview
//! spike (TASK-612) — nothing here fakes it.

#![forbid(unsafe_code)]

pub mod blackbar;
pub mod browser;
pub mod camera_controls;
pub mod chat;
pub mod color;
pub(crate) mod compose;
pub mod countdown;
pub mod deinterlace;
pub mod image;
pub mod inputoverlay;
pub mod laningest;
pub mod link;
pub mod media;
pub mod playlist;
pub(crate) mod registry;
pub mod replaysrc;
pub mod slideshow;
pub mod socialbar;
pub mod splits;
pub mod static_source;
pub mod testsignal;
pub mod text;
pub mod textfile;
pub mod title;
pub mod video_device;
pub mod visualizer;

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
