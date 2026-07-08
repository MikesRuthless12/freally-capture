//! Stream targets: the per-service RTMP/RTMPS ingest presets, the SRT and
//! WHIP protocol targets (Phase 6), and the publish URL builder. The
//! **stream key is a secret** — redacted from `Debug`, masked in the UI,
//! never logged; the full publish URL exists only in memory on its way into
//! the muxer.

use serde::{Deserialize, Serialize};

/// How a target's bits leave the machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamProtocol {
    /// FLV over RTMP/RTMPS (the platform presets + custom ingests).
    Rtmp,
    /// MPEG-TS over SRT to a self-hosted ingest.
    Srt,
    /// WebRTC-HTTP ingestion (WHIP) to an `https://` endpoint. Audio is
    /// Opus (WebRTC takes no AAC), so a WHIP target never shares an encode.
    Whip,
}

/// The services the target picker offers. `Custom` takes any RTMP(S) ingest
/// (self-hosted, a restream provider, a platform we don't preset); `Srt`
/// and `Whip` are the Phase 6 protocol targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StreamService {
    Twitch,
    YouTube,
    Kick,
    Facebook,
    Trovo,
    Custom,
    Srt,
    Whip,
}

impl StreamService {
    /// The service's default ingest URL (no key). Empty for `Custom`, `Srt`
    /// and `Whip` — the user supplies their own. These are the public,
    /// well-known ingests; the settings let the user override any of them
    /// (regional ingests).
    pub fn default_ingest(self) -> &'static str {
        match self {
            StreamService::Twitch => "rtmp://live.twitch.tv/app",
            StreamService::YouTube => "rtmp://a.rtmp.youtube.com/live2",
            StreamService::Kick => "rtmps://fa723fc1b171.global-contribute.live-video.net/app",
            StreamService::Facebook => "rtmps://live-api-s.facebook.com:443/rtmp",
            StreamService::Trovo => "rtmp://livepush.trovo.live/live",
            StreamService::Custom | StreamService::Srt | StreamService::Whip => "",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            StreamService::Twitch => "Twitch",
            StreamService::YouTube => "YouTube",
            StreamService::Kick => "Kick",
            StreamService::Facebook => "Facebook",
            StreamService::Trovo => "Trovo",
            StreamService::Custom => "Custom (RTMP/RTMPS)",
            StreamService::Srt => "SRT (self-hosted)",
            StreamService::Whip => "WHIP (WebRTC)",
        }
    }

    pub fn protocol(self) -> StreamProtocol {
        match self {
            StreamService::Srt => StreamProtocol::Srt,
            StreamService::Whip => StreamProtocol::Whip,
            _ => StreamProtocol::Rtmp,
        }
    }
}

/// Where a session publishes: a service (or custom ingest) + the secret key.
#[derive(Clone, PartialEq, Eq)]
pub struct StreamTarget {
    pub service: StreamService,
    /// Overrides the service preset when non-empty (regional/custom ingest).
    pub ingest_url: String,
    /// The stream key — SECRET.
    pub key: String,
}

impl std::fmt::Debug for StreamTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StreamTarget")
            .field("service", &self.service)
            .field("ingest_url", &self.ingest_url)
            .field(
                "key",
                &if self.key.is_empty() {
                    ""
                } else {
                    "[redacted]"
                },
            )
            .finish()
    }
}

/// Why a target can't publish (each names the fix, honestly).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetError {
    MissingKey,
    MissingIngest,
    BadIngestScheme,
    BadCharacters,
}

impl std::fmt::Display for TargetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            TargetError::MissingKey => "the stream key is empty — paste it from your dashboard",
            TargetError::MissingIngest => {
                "this target needs an ingest URL (rtmp:// / rtmps://, srt://, or https:// for WHIP)"
            }
            TargetError::BadIngestScheme => {
                "the ingest URL scheme doesn't match the service (rtmp:///rtmps:// for RTMP, srt:// for SRT, https:// for WHIP)"
            }
            TargetError::BadCharacters => {
                "the ingest URL or key contains characters that can't be published"
            }
        };
        f.write_str(message)
    }
}

impl StreamTarget {
    /// The effective ingest: the override when set, else the service preset.
    pub fn ingest(&self) -> &str {
        if self.ingest_url.trim().is_empty() {
            self.service.default_ingest()
        } else {
            self.ingest_url.trim()
        }
    }

    /// The full publish URL — validated, built at Go Live time only. Both
    /// parts are user input: bound their shape defensively.
    ///
    /// - RTMP: `ingest/key` (the key is required).
    /// - SRT: the `srt://` ingest as-is; a non-empty key becomes the
    ///   `streamid` query parameter (how SLS/MediaMTX-style ingests route).
    /// - WHIP: the `https://` endpoint as-is — the key is a **bearer
    ///   token** and rides the `Authorization` header, never the URL.
    pub fn publish_url(&self) -> Result<String, TargetError> {
        let key = self.key.trim();
        let ingest = self.ingest();
        if ingest.is_empty() {
            return Err(TargetError::MissingIngest);
        }
        let sane = |s: &str| {
            s.len() <= 512
                && s.chars().all(|c| c.is_ascii_graphic())
                && !s.contains(char::is_whitespace)
        };
        if !sane(ingest) || !(key.is_empty() || sane(key)) {
            return Err(TargetError::BadCharacters);
        }
        match self.service.protocol() {
            StreamProtocol::Rtmp => {
                if key.is_empty() {
                    return Err(TargetError::MissingKey);
                }
                if !(ingest.starts_with("rtmp://") || ingest.starts_with("rtmps://")) {
                    return Err(TargetError::BadIngestScheme);
                }
                Ok(format!("{}/{}", ingest.trim_end_matches('/'), key))
            }
            StreamProtocol::Srt => {
                if !ingest.starts_with("srt://") {
                    return Err(TargetError::BadIngestScheme);
                }
                if key.is_empty() {
                    Ok(ingest.to_string())
                } else {
                    let sep = if ingest.contains('?') { '&' } else { '?' };
                    Ok(format!("{ingest}{sep}streamid={key}"))
                }
            }
            StreamProtocol::Whip => {
                if !(ingest.starts_with("https://") || ingest.starts_with("http://")) {
                    return Err(TargetError::BadIngestScheme);
                }
                Ok(ingest.to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn target(service: StreamService, ingest: &str, key: &str) -> StreamTarget {
        StreamTarget {
            service,
            ingest_url: ingest.to_string(),
            key: key.to_string(),
        }
    }

    #[test]
    fn presets_build_the_publish_url() {
        let url = target(StreamService::Twitch, "", "live_123_abc")
            .publish_url()
            .expect("valid");
        assert_eq!(url, "rtmp://live.twitch.tv/app/live_123_abc");
    }

    #[test]
    fn custom_ingest_overrides_and_trailing_slash_is_tolerated() {
        let url = target(
            StreamService::Custom,
            "rtmps://ingest.example.com/live/",
            "k",
        )
        .publish_url()
        .expect("valid");
        assert_eq!(url, "rtmps://ingest.example.com/live/k");
    }

    #[test]
    fn invalid_targets_error_honestly() {
        assert_eq!(
            target(StreamService::Twitch, "", "").publish_url(),
            Err(TargetError::MissingKey)
        );
        assert_eq!(
            target(StreamService::Custom, "", "k").publish_url(),
            Err(TargetError::MissingIngest)
        );
        assert_eq!(
            target(StreamService::Custom, "http://nope", "k").publish_url(),
            Err(TargetError::BadIngestScheme)
        );
        assert_eq!(
            target(StreamService::Custom, "rtmp://ok", "bad key").publish_url(),
            Err(TargetError::BadCharacters)
        );
    }

    #[test]
    fn debug_never_prints_the_key() {
        let t = target(StreamService::Twitch, "", "hunter2");
        let printed = format!("{t:?}");
        assert!(!printed.contains("hunter2"), "key leaked: {printed}");
        assert!(printed.contains("[redacted]"));
    }

    #[test]
    fn every_preset_service_has_a_valid_ingest() {
        for service in [
            StreamService::Twitch,
            StreamService::YouTube,
            StreamService::Kick,
            StreamService::Facebook,
            StreamService::Trovo,
        ] {
            let ingest = service.default_ingest();
            assert!(
                ingest.starts_with("rtmp://") || ingest.starts_with("rtmps://"),
                "{service:?}: {ingest}"
            );
        }
    }

    #[test]
    fn srt_appends_the_streamid_and_respects_existing_queries() {
        let plain = target(StreamService::Srt, "srt://relay.lan:8890", "");
        assert_eq!(plain.publish_url().expect("valid"), "srt://relay.lan:8890");

        let with_id = target(StreamService::Srt, "srt://relay.lan:8890", "publish:cam");
        assert_eq!(
            with_id.publish_url().expect("valid"),
            "srt://relay.lan:8890?streamid=publish:cam"
        );

        let with_query = target(
            StreamService::Srt,
            "srt://relay.lan:8890?latency=200",
            "publish:cam",
        );
        assert_eq!(
            with_query.publish_url().expect("valid"),
            "srt://relay.lan:8890?latency=200&streamid=publish:cam"
        );

        assert_eq!(
            target(StreamService::Srt, "rtmp://nope", "k").publish_url(),
            Err(TargetError::BadIngestScheme)
        );
        assert_eq!(
            target(StreamService::Srt, "", "k").publish_url(),
            Err(TargetError::MissingIngest)
        );
    }

    #[test]
    fn whip_takes_the_endpoint_as_is_and_keeps_the_token_out_of_the_url() {
        let endpoint = target(
            StreamService::Whip,
            "https://sfu.example.net/whip/room1",
            "bearer-token-shh",
        );
        let url = endpoint.publish_url().expect("valid");
        assert_eq!(url, "https://sfu.example.net/whip/room1");
        assert!(!url.contains("shh"), "the token never rides the URL");

        assert_eq!(
            target(StreamService::Whip, "rtmp://nope", "").publish_url(),
            Err(TargetError::BadIngestScheme)
        );
    }

    #[test]
    fn protocols_map_by_service() {
        assert_eq!(StreamService::Twitch.protocol(), StreamProtocol::Rtmp);
        assert_eq!(StreamService::Custom.protocol(), StreamProtocol::Rtmp);
        assert_eq!(StreamService::Srt.protocol(), StreamProtocol::Srt);
        assert_eq!(StreamService::Whip.protocol(), StreamProtocol::Whip);
    }
}
