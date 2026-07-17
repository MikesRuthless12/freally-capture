//! CAP-N48 network condition simulator: deterministic shaping for the
//! CAP-N49 rehearsal sink.
//!
//! The real outbound socket of a live stream lives inside the ffmpeg
//! component, so the honest place to rehearse bad networks is the one
//! socket the app **owns outright**: the loopback sink a dry run publishes
//! to. Shaping happens server-side, exactly like a congested uplink would:
//! the sink *reads* at the shaped rate, TCP backpressure does the rest
//! (the encoder's queues fill, measured kbps drops, frames drop honestly),
//! and a scheduled outage severs the connection so the supervisor's real
//! reconnect ladder runs.
//!
//! Everything here is **deterministic**: the token bucket and the outage
//! schedule are pure functions of the profile + elapsed time, and jitter
//! comes from a seeded xorshift — the same drill replays identically, so
//! rehearsals are comparable across runs. No wall clock is read in this
//! module; callers feed elapsed milliseconds.

/// One rehearsal network profile. All zeros = shape nothing.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShapeProfile {
    /// Uplink cap in kilobits/second; 0 = uncapped.
    pub bandwidth_kbps: u32,
    /// Base pacing delay in ms — with `jitter_ms`, perturbs *when* budget
    /// arrives (throughput wobble), the way a long/noisy path feels to an
    /// encoder. (A drain sink has no viewer, so glass-to-glass delay is
    /// out of scope — said honestly in the UI.)
    pub latency_ms: u32,
    /// Extra ± jitter in ms applied to the pacing delay, seeded.
    pub jitter_ms: u32,
    /// Sever the connection every ~this many seconds; 0 = never.
    pub outage_every_s: u32,
    /// How long each outage lasts (connection refused throughout).
    pub outage_len_s: u32,
    /// The jitter seed — same seed, same drill.
    pub seed: u64,
}

impl ShapeProfile {
    /// The "hotel Wi-Fi" preset the DoD's dress rehearsal uses: a thin,
    /// wobbly uplink that drops out for a few seconds every couple of
    /// minutes.
    pub fn hotel_wifi() -> Self {
        ShapeProfile {
            bandwidth_kbps: 2_500,
            latency_ms: 80,
            jitter_ms: 60,
            outage_every_s: 90,
            outage_len_s: 6,
            seed: 0x4841_564f_4331,
        }
    }

    /// A tethered phone: more headroom, rarer but longer dropouts.
    pub fn mobile_hotspot() -> Self {
        ShapeProfile {
            bandwidth_kbps: 4_000,
            latency_ms: 60,
            jitter_ms: 40,
            outage_every_s: 180,
            outage_len_s: 10,
            seed: 0x4841_564f_4332,
        }
    }

    /// Whether this profile shapes anything at all.
    pub fn is_active(&self) -> bool {
        self.bandwidth_kbps > 0
            || self.latency_ms > 0
            || self.jitter_ms > 0
            || (self.outage_every_s > 0 && self.outage_len_s > 0)
    }

    /// Whether the connection is severed at `elapsed_ms` into the session.
    /// Outages tile the timeline deterministically: each period of
    /// `outage_every_s + outage_len_s` ends with `outage_len_s` of outage,
    /// so the first one hits after a full healthy interval.
    pub fn is_out(&self, elapsed_ms: u64) -> bool {
        if self.outage_every_s == 0 || self.outage_len_s == 0 {
            return false;
        }
        let period_ms = u64::from(self.outage_every_s + self.outage_len_s) * 1_000;
        let healthy_ms = u64::from(self.outage_every_s) * 1_000;
        elapsed_ms % period_ms >= healthy_ms
    }
}

/// The deterministic token bucket: bytes of read budget accrue at the
/// capped rate, up to one second of burst.
#[derive(Debug, Clone)]
pub struct TokenBucket {
    rate_bytes_per_s: u64,
    capacity: u64,
    tokens: u64,
    last_ms: u64,
}

impl TokenBucket {
    /// A bucket for `bandwidth_kbps`; `None` when uncapped.
    pub fn new(bandwidth_kbps: u32) -> Option<TokenBucket> {
        if bandwidth_kbps == 0 {
            return None;
        }
        let rate = u64::from(bandwidth_kbps) * 1_000 / 8;
        Some(TokenBucket {
            rate_bytes_per_s: rate,
            capacity: rate, // a 1-second burst ceiling
            tokens: rate,
            last_ms: 0,
        })
    }

    /// Accrue budget up to `elapsed_ms`, then take at most `want` bytes.
    /// Pure: the same call sequence always yields the same grants.
    pub fn take(&mut self, elapsed_ms: u64, want: usize) -> usize {
        if elapsed_ms > self.last_ms {
            let accrued = (elapsed_ms - self.last_ms) * self.rate_bytes_per_s / 1_000;
            self.tokens = (self.tokens + accrued).min(self.capacity);
            self.last_ms = elapsed_ms;
        }
        let grant = (want as u64).min(self.tokens);
        self.tokens -= grant;
        grant as usize
    }
}

/// The seeded jitter source (xorshift64*) — deterministic, dependency-free.
#[derive(Debug, Clone)]
pub struct Jitter {
    state: u64,
}

impl Jitter {
    pub fn new(seed: u64) -> Jitter {
        Jitter {
            state: seed.max(1), // xorshift dies on 0
        }
    }

    /// The next pacing delay for a profile: `latency_ms ± jitter_ms`,
    /// clamped at 0.
    pub fn next_delay_ms(&mut self, latency_ms: u32, jitter_ms: u32) -> u64 {
        self.state ^= self.state >> 12;
        self.state ^= self.state << 25;
        self.state ^= self.state >> 27;
        let rnd = self.state.wrapping_mul(0x2545_F491_4F6C_DD1D);
        if jitter_ms == 0 {
            return u64::from(latency_ms);
        }
        let span = u64::from(jitter_ms) * 2 + 1;
        let offset = (rnd % span) as i64 - i64::from(jitter_ms);
        (i64::from(latency_ms) + offset).max(0) as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_bucket_grants_exactly_the_configured_rate() {
        // 2500 kbps = 312_500 bytes/s.
        let mut bucket = TokenBucket::new(2_500).expect("capped");
        // Drain the initial burst allowance first.
        assert_eq!(bucket.take(0, usize::MAX), 312_500);
        // Then exactly the rate accrues, second by second.
        assert_eq!(bucket.take(1_000, usize::MAX), 312_500);
        assert_eq!(bucket.take(1_500, usize::MAX), 156_250);
        // Idle time accrues at most one second of burst.
        assert_eq!(bucket.take(60_000, usize::MAX), 312_500);
        // Uncapped = no bucket at all.
        assert!(TokenBucket::new(0).is_none());
    }

    #[test]
    fn token_bucket_is_deterministic_across_runs() {
        let timeline: Vec<(u64, usize)> = (0..200).map(|i| (i * 37, 4_096)).collect();
        let run = || {
            let mut bucket = TokenBucket::new(1_000).expect("capped");
            timeline
                .iter()
                .map(|(t, want)| bucket.take(*t, *want))
                .collect::<Vec<usize>>()
        };
        assert_eq!(run(), run(), "same timeline, same grants — always");
    }

    #[test]
    fn outage_schedule_tiles_the_timeline_deterministically() {
        let profile = ShapeProfile {
            outage_every_s: 90,
            outage_len_s: 6,
            ..ShapeProfile::hotel_wifi()
        };
        // Healthy through the first 90 s...
        assert!(!profile.is_out(0));
        assert!(!profile.is_out(89_999));
        // ...out for exactly 6 s...
        assert!(profile.is_out(90_000));
        assert!(profile.is_out(95_999));
        // ...healthy again, and the pattern repeats forever.
        assert!(!profile.is_out(96_000));
        assert!(profile.is_out(96_000 + 90_000));
        // No outage configured = never out.
        let calm = ShapeProfile {
            outage_every_s: 0,
            ..profile
        };
        assert!(!calm.is_out(90_000));
    }

    #[test]
    fn jitter_is_seeded_bounded_and_reproducible() {
        let sequence = |seed: u64| {
            let mut jitter = Jitter::new(seed);
            (0..100)
                .map(|_| jitter.next_delay_ms(80, 60))
                .collect::<Vec<u64>>()
        };
        let a = sequence(7);
        assert_eq!(a, sequence(7), "same seed, same drill");
        assert_ne!(a, sequence(8), "different seed, different drill");
        assert!(a.iter().all(|d| *d <= 140), "delay stays within ±jitter");
        // Zero jitter is exactly the base latency.
        let mut flat = Jitter::new(7);
        assert_eq!(flat.next_delay_ms(80, 0), 80);
    }
}
