//! # fcap-audio
//!
//! The `cpal` audio graph: system/loopback + mic + per-source capture, mixed
//! into up to 6 tracks with per-source assignment and sync offset; the ordered
//! per-source filter chain (classic-DSP denoise — spectral suppression, no ML
//! — plus noise gate, compressor, limiter, EQ, gain); monitoring, sidechain
//! ducking, push-to-talk / push-to-mute, and LUFS metering. macOS system audio
//! is guided honestly (ScreenCaptureKit 13+ or a virtual device — never
//! silently installed).
//!
//! **Phase 0 stub** — the crate boundary exists; the audio graph lands in
//! Phase 3 (→ 0.55.0).

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
