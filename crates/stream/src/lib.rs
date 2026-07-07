//! # fcap-stream
//!
//! Stream orchestration: single-target RTMP/RTMPS (Twitch/YouTube/Kick/
//! Facebook/Trovo/custom) with auto-reconnect and configurable stream delay —
//! the main recording continues regardless of stream state; Pro multistream
//! direct to each platform (no restream server), SRT + WHIP; the per-OS
//! **virtual camera**; the rolling **replay buffer**; and the off-by-default,
//! password-protected **WebSocket remote-control API** bound to `127.0.0.1`.
//!
//! **Phase 5:** single-target RTMP/RTMPS is here — [`rtmp`] (service ingest
//! presets + the secret-safe publish URL) and [`session`] (the supervised
//! stream engine: reconnect with backoff, optional broadcast delay, honest
//! status). Multistream/SRT/WHIP, the virtual camera, the replay buffer, and
//! the remote API land in Phases 6–7.

#![forbid(unsafe_code)]

pub mod multistream;
pub mod rtmp;
pub mod session;

pub use multistream::{
    group_members, LaneCells, LaneIo, LaneMaker, MemberSpec, MemberStatus, MultiHandle,
    MultiSession,
};
pub use rtmp::{StreamProtocol, StreamService, StreamTarget, TargetError};
pub use session::{
    backoff, SinkFactory, StreamHandle, StreamSession, StreamSpec, StreamState, StreamStatus,
    MAX_RECONNECT_ATTEMPTS,
};

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
