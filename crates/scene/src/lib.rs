//! # fcap-scene
//!
//! The **owned** scene / source / filter data model: scenes contain ordered
//! source items with z-order; sources are shareable across scenes (and nest —
//! a scene can be a source in another); each item carries transform, blend,
//! visibility, lock, and an ordered filter chain. Serde-serialized — this model
//! is the scene-collection project format on disk and the scripting surface.
//!
//! **Phase 0 stub** — the crate boundary exists; the model lands in Phase 2
//! (→ 0.40.0) with round-trip tests.

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
