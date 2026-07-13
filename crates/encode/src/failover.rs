//! Mid-session encoder failover (CAP-M12): when the sink driving a stream
//! or recording dies, decide — honestly — whether the ENCODER is to blame
//! and, if so, which encoder to try next. Pure state machine, no processes;
//! the app owns the actual respawn.
//!
//! The ladder for a configured encoder: the encoder itself → the best
//! verified hardware encoder of a *different* family (same codec) → the
//! software encoder (x264/x265/AV1 — always present for H.264). Every
//! encoder here runs inside the ffmpeg child that also owns the muxer and
//! the connection, so a swap is a supervised respawn — the session's
//! reconnect machinery keeps the show up while the file/stream continues
//! under the fallback.

use std::time::Duration;

use crate::encoder::{Catalog, VideoCodec};

/// What a sink-death error message points at.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Blame {
    /// The encoder (driver reset, device refusal, encoder init failure).
    Encoder,
    /// The wire (ingest down, DNS, resets) — a different encoder won't help.
    Network,
    /// Nothing recognizable either way.
    Unknown,
}

/// Substring heuristics over ffmpeg's stderr tail. Checked lowercase.
const ENCODER_SIGNS: &[&str] = &[
    "cannot open encoder",
    "error while opening encoder",
    "error initializing output stream",
    "no capable devices",
    "no nvenc capable devices",
    "nvenc",
    "cuda",
    "qsv",
    "mfx",
    "amf",
    "vaapi",
    "videotoolbox",
    "hwaccel",
    "device creation failed",
    "driver",
    "generic error in an external library",
];

const NETWORK_SIGNS: &[&str] = &[
    "connection refused",
    "connection reset",
    "connection timed out",
    "broken pipe",
    "timed out",
    "timeout",
    "rtmp",
    "srt:",
    "handshake",
    "tls",
    "network",
    "unreachable",
    "getaddrinfo",
    "name resolution",
    "end of file",
    "econnreset",
    "server returned 4",
    "server returned 5",
];

/// Classify a sink-death message. Signs from BOTH sides (or neither) are an
/// honest [`Blame::Unknown`] — never guess the encoder on a wire error.
pub fn classify_fault(error: &str) -> Blame {
    let text = error.to_ascii_lowercase();
    let encoder = ENCODER_SIGNS.iter().any(|sign| text.contains(sign));
    let network = NETWORK_SIGNS.iter().any(|sign| text.contains(sign));
    match (encoder, network) {
        (true, false) => Blame::Encoder,
        (false, true) => Blame::Network,
        _ => Blame::Unknown,
    }
}

/// A sink that survived at least this long proved the encoder itself opens
/// and encodes — later faults are less likely the encoder's.
pub const HEALTHY_RUN: Duration = Duration::from_secs(20);

/// The ladder's verdict for one fault.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FailoverDecision {
    /// Keep the current encoder (the existing retry path handles it).
    Stay,
    /// Respawn with `to` — surface the honest toast/note.
    Switch { from: String, to: String },
    /// Already on the last rung — nothing left to switch to.
    Exhausted,
}

/// The failover state machine. One instance per session; feed it every
/// sink death via [`FailoverLadder::on_fault`].
#[derive(Debug)]
pub struct FailoverLadder {
    rungs: Vec<String>,
    index: usize,
    /// One fast `Unknown` fault arms; a second on the same rung switches.
    /// (A single fast death could be a network refusal — don't jump yet.)
    armed: bool,
}

impl FailoverLadder {
    /// Build the ladder for `configured` against the machine's catalog:
    /// configured → best different-family verified hardware (same codec) →
    /// software floor. Duplicates collapse; an unknown id still gets the
    /// H.264 software floor.
    pub fn new(configured: &str, catalog: &Catalog) -> Self {
        let desc = catalog.get(configured);
        let codec = desc.map(|d| d.codec).unwrap_or(VideoCodec::H264);
        let engine = desc.map(|d| d.engine);
        let mut rungs = vec![configured.to_string()];
        if let Some(other) = catalog
            .encoders
            .iter()
            .filter(|d| {
                d.codec == codec
                    && d.hardware
                    && d.verified != Some(false)
                    && Some(d.engine) != engine
            })
            .min_by_key(|d| d.engine.rank())
        {
            rungs.push(other.id.clone());
        }
        if let Some(software) = catalog
            .encoders
            .iter()
            .find(|d| d.codec == codec && !d.hardware && d.verified != Some(false))
        {
            rungs.push(software.id.clone());
        }
        // First occurrence wins (configured may BE the software floor).
        let mut seen = Vec::new();
        rungs.retain(|id| {
            let fresh = !seen.contains(id);
            seen.push(id.clone());
            fresh
        });
        FailoverLadder {
            rungs,
            index: 0,
            armed: false,
        }
    }

    /// The encoder the next (re)spawn should use.
    pub fn current(&self) -> &str {
        &self.rungs[self.index]
    }

    /// A sink died after `lived` with this blame — stay, switch, or admit
    /// exhaustion. Encoder blame switches immediately (a driver reset hours
    /// in still needs a working encoder NOW); network blame never switches;
    /// unknown blame switches only on two consecutive fast deaths.
    pub fn on_fault(&mut self, blame: Blame, lived: Duration) -> FailoverDecision {
        let advance = match blame {
            Blame::Encoder => true,
            Blame::Network => {
                self.armed = false;
                false
            }
            Blame::Unknown => {
                if lived >= HEALTHY_RUN {
                    self.armed = false;
                    false
                } else if self.armed {
                    true
                } else {
                    self.armed = true;
                    false
                }
            }
        };
        if !advance {
            return FailoverDecision::Stay;
        }
        self.armed = false;
        if self.index + 1 >= self.rungs.len() {
            return FailoverDecision::Exhausted;
        }
        let from = self.rungs[self.index].clone();
        self.index += 1;
        FailoverDecision::Switch {
            from,
            to: self.rungs[self.index].clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoder::{EncoderDesc, EncoderEngine};

    fn desc(id: &str, engine: EncoderEngine, verified: Option<bool>) -> EncoderDesc {
        EncoderDesc {
            id: id.to_string(),
            codec: VideoCodec::H264,
            engine,
            label: id.to_string(),
            hardware: engine != EncoderEngine::Software,
            note: String::new(),
            verified,
        }
    }

    fn catalog() -> Catalog {
        Catalog {
            gpus: Vec::new(),
            encoders: vec![
                desc("h264_nvenc", EncoderEngine::Nvenc, Some(true)),
                desc("h264_qsv", EncoderEngine::QuickSync, Some(true)),
                desc("libx264", EncoderEngine::Software, None),
            ],
        }
    }

    const FAST: Duration = Duration::from_secs(2);
    const HEALTHY: Duration = Duration::from_secs(3600);

    #[test]
    fn classification_reads_the_stderr_tail() {
        assert_eq!(
            classify_fault("Cannot open encoder h264_nvenc"),
            Blame::Encoder
        );
        assert_eq!(
            classify_fault("OpenEncodeSessionEx failed: no NVENC capable devices found"),
            Blame::Encoder
        );
        assert_eq!(
            classify_fault("rtmp://live: Connection refused"),
            Blame::Network
        );
        assert_eq!(classify_fault("Broken pipe"), Blame::Network);
        assert_eq!(classify_fault("something exploded"), Blame::Unknown);
        // Signs from both sides stay honest.
        assert_eq!(
            classify_fault("nvenc packet lost: rtmp handshake failed"),
            Blame::Unknown
        );
    }

    #[test]
    fn ladder_runs_configured_then_other_family_then_software() {
        let ladder = FailoverLadder::new("h264_nvenc", &catalog());
        assert_eq!(ladder.rungs, vec!["h264_nvenc", "h264_qsv", "libx264"]);
    }

    #[test]
    fn ladder_skips_refused_hardware_and_collapses_duplicates() {
        let mut cat = catalog();
        cat.encoders[1].verified = Some(false); // qsv refused by the probe
        let ladder = FailoverLadder::new("h264_nvenc", &cat);
        assert_eq!(ladder.rungs, vec!["h264_nvenc", "libx264"]);

        // Already on the software floor: nowhere to go.
        let floor = FailoverLadder::new("libx264", &catalog());
        assert_eq!(floor.rungs.first().map(String::as_str), Some("libx264"));
        assert!(!floor.rungs.contains(&"h264_qsv".to_string()) || floor.rungs[0] == "libx264");
    }

    #[test]
    fn encoder_blame_switches_immediately_even_after_hours() {
        let mut ladder = FailoverLadder::new("h264_nvenc", &catalog());
        assert_eq!(
            ladder.on_fault(Blame::Encoder, HEALTHY),
            FailoverDecision::Switch {
                from: "h264_nvenc".into(),
                to: "h264_qsv".into()
            }
        );
        assert_eq!(ladder.current(), "h264_qsv");
    }

    #[test]
    fn network_blame_never_switches() {
        let mut ladder = FailoverLadder::new("h264_nvenc", &catalog());
        for _ in 0..10 {
            assert_eq!(
                ladder.on_fault(Blame::Network, FAST),
                FailoverDecision::Stay
            );
        }
        assert_eq!(ladder.current(), "h264_nvenc");
    }

    #[test]
    fn unknown_blame_needs_two_consecutive_fast_deaths() {
        let mut ladder = FailoverLadder::new("h264_nvenc", &catalog());
        assert_eq!(
            ladder.on_fault(Blame::Unknown, FAST),
            FailoverDecision::Stay
        );
        assert_eq!(
            ladder.on_fault(Blame::Unknown, FAST),
            FailoverDecision::Switch {
                from: "h264_nvenc".into(),
                to: "h264_qsv".into()
            }
        );
    }

    #[test]
    fn a_healthy_run_disarms_the_unknown_strike() {
        let mut ladder = FailoverLadder::new("h264_nvenc", &catalog());
        assert_eq!(
            ladder.on_fault(Blame::Unknown, FAST),
            FailoverDecision::Stay
        );
        // It ran fine for an hour — the fast death before was noise.
        assert_eq!(
            ladder.on_fault(Blame::Unknown, HEALTHY),
            FailoverDecision::Stay
        );
        // The strike counter restarted: one more fast death only arms again.
        assert_eq!(
            ladder.on_fault(Blame::Unknown, FAST),
            FailoverDecision::Stay
        );
        assert_eq!(ladder.current(), "h264_nvenc");
    }

    #[test]
    fn the_floor_admits_exhaustion() {
        let mut ladder = FailoverLadder::new("h264_nvenc", &catalog());
        assert!(matches!(
            ladder.on_fault(Blame::Encoder, FAST),
            FailoverDecision::Switch { .. }
        ));
        assert!(matches!(
            ladder.on_fault(Blame::Encoder, FAST),
            FailoverDecision::Switch { .. }
        ));
        assert_eq!(ladder.current(), "libx264");
        assert_eq!(
            ladder.on_fault(Blame::Encoder, FAST),
            FailoverDecision::Exhausted
        );
    }
}
