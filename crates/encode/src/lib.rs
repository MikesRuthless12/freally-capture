//! # fcap-encode
//!
//! The `Encoder` interface and its implementations: detected hardware encoders
//! (NVENC / Quick Sync / AMF on Windows+Linux, VAAPI on Linux, VideoToolbox on
//! macOS) with **x264** as the universal CPU fallback; the **owned
//! `freally-video`** lossless codec (`.frec`, shared with Freally Snipper) as
//! the default local-recording format needing no external tool; muxing into
//! mp4/mkv/mov/webm with up to 6 audio tracks, file splitting, pause/resume,
//! and remux.
//!
//! The patent-encumbered wire codecs (H.264/AAC/HEVC/AV1) run through the
//! clearly-labeled **on-demand ffmpeg bridge** — never bundled, fetched to a
//! per-user cache, hash-verified before use.
//!
//! **Phase 0 stub** — the crate boundary exists; encoding lands in Phase 4
//! (→ 0.55.0).

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
