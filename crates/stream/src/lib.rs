//! # fcap-stream
//!
//! Stream orchestration: single-target RTMP/RTMPS (Twitch/YouTube/Kick/
//! Facebook/Trovo/custom) with auto-reconnect and configurable stream delay —
//! the main recording continues regardless of stream state; Pro multistream
//! direct to each platform (no restream server), SRT + WHIP; the per-OS
//! **virtual camera**; the rolling **replay buffer**; and the off-by-default,
//! password-protected **WebSocket remote-control API** bound to `127.0.0.1`.
//!
//! **Phase 0 stub** — the crate boundary exists; streaming lands in Phase 5
//! (→ 0.70.0, first public), depth in Phases 6–7.

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
