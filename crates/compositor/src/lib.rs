//! # fcap-compositor
//!
//! The **owned** real-time GPU compositor on `wgpu` — the engine of Freally
//! Capture. Uploads each visible source frame as a texture and composes the
//! single program frame with per-source transform (move/scale/rotate/crop),
//! blend modes, the ordered per-source video filter chain (chroma key, color
//! correction, LUT, blur, mask, sharpen, scroll, color/luma key, render delay),
//! and scene transitions (cut/fade/slide/swipe/stinger/luma wipe).
//! Budget: sustained 60 fps at 1080p with ≥4 sources on a mid-range GPU.
//!
//! **Phase 0 stub** — the crate boundary exists; the compositor core lands in
//! Phase 2 (→ 0.40.0), transitions in Phase 5 (→ 0.70.0).

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
