//! CAP-N49 dry-run mode: the app's **own loopback sink** for "Go Live
//! (rehearsal)".
//!
//! A rehearsal lane publishes MPEG-TS over plain TCP to `127.0.0.1` — the
//! same payload format SRT carries — so the whole real pipeline runs
//! (compositor → encoder → mux → socket → supervisor reconnect logic) while
//! **zero bytes leave the machine**. This server is that sink: it accepts
//! the encoder's connection and drains it, counting what arrives. Killing
//! the connection (or refusing accepts) is exactly how the CAP-N48 network
//! simulator later induces reconnect drills.
//!
//! Loopback-only by construction: the listener binds `127.0.0.1:0` and the
//! encode layer refuses any non-loopback `tcp://` publish URL.

use std::io::Read;
use std::net::TcpListener;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use crate::shaper::{Jitter, ShapeProfile, TokenBucket};

/// How often the accept loop polls for a connection or shutdown.
const ACCEPT_POLL: Duration = Duration::from_millis(25);
/// Read timeout so the drain loop can notice a shutdown promptly.
const READ_POLL: Duration = Duration::from_millis(50);
/// Ceiling on one pacing sleep slice, so shutdown stays responsive.
const PACING_SLICE: Duration = Duration::from_millis(50);

/// One rehearsal lane's loopback sink server. Dropping it stops the thread.
pub struct RehearsalSink {
    port: u16,
    bytes_in: Arc<AtomicU64>,
    accepts: Arc<AtomicU64>,
    shutdown: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl RehearsalSink {
    /// Bind a fresh loopback port and start draining whatever connects.
    /// Accepts one connection at a time and keeps accepting after a
    /// disconnect, so a lane's reconnect attempts always find the sink.
    pub fn start() -> std::io::Result<RehearsalSink> {
        Self::start_shaped(None)
    }

    /// Like [`RehearsalSink::start`], but with a CAP-N48 network profile
    /// shaping the drain: reads are paced by the profile's token bucket and
    /// latency/jitter (TCP backpressure then squeezes the encoder exactly
    /// like a thin uplink), and each scheduled outage severs the connection
    /// so the lane's real reconnect ladder runs. During an outage a
    /// reconnect attempt is accepted and immediately reset — how a flapping
    /// network actually feels to a client.
    pub fn start_shaped(profile: Option<ShapeProfile>) -> std::io::Result<RehearsalSink> {
        let profile = profile.filter(ShapeProfile::is_active);
        let listener = TcpListener::bind("127.0.0.1:0")?;
        listener.set_nonblocking(true)?;
        let port = listener.local_addr()?.port();
        let bytes_in = Arc::new(AtomicU64::new(0));
        let accepts = Arc::new(AtomicU64::new(0));
        let shutdown = Arc::new(AtomicBool::new(false));
        let thread = {
            let bytes_in = Arc::clone(&bytes_in);
            let accepts = Arc::clone(&accepts);
            let shutdown = Arc::clone(&shutdown);
            std::thread::Builder::new()
                .name("fcap-rehearsal-sink".into())
                .spawn(move || {
                    let started = Instant::now();
                    let mut bucket = profile.and_then(|p| TokenBucket::new(p.bandwidth_kbps));
                    let mut jitter = profile.map(|p| Jitter::new(p.seed));
                    let mut buf = vec![0u8; 64 * 1024];
                    while !shutdown.load(Ordering::Relaxed) {
                        let (mut conn, _) = match listener.accept() {
                            Ok(pair) => pair,
                            Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                                std::thread::sleep(ACCEPT_POLL);
                                continue;
                            }
                            Err(_) => return,
                        };
                        // Mid-outage: reset the newcomer straight away.
                        let elapsed = started.elapsed().as_millis() as u64;
                        if profile.is_some_and(|p| p.is_out(elapsed)) {
                            drop(conn);
                            std::thread::sleep(ACCEPT_POLL);
                            continue;
                        }
                        accepts.fetch_add(1, Ordering::Relaxed);
                        if conn.set_read_timeout(Some(READ_POLL)).is_err() {
                            continue;
                        }
                        // Drain until the peer hangs up, an outage severs it,
                        // or we shut down.
                        while !shutdown.load(Ordering::Relaxed) {
                            let elapsed = started.elapsed().as_millis() as u64;
                            if profile.is_some_and(|p| p.is_out(elapsed)) {
                                break; // sever — the drill's reconnect moment
                            }
                            // Pacing wobble: latency ± jitter, in short
                            // slices so shutdown stays prompt. The bucket
                            // accrues meanwhile, so the *average* rate stays
                            // the configured cap — this only makes it bursty.
                            if let (Some(profile), Some(jitter)) = (&profile, jitter.as_mut()) {
                                let mut delay =
                                    jitter.next_delay_ms(profile.latency_ms, profile.jitter_ms);
                                while delay > 0 && !shutdown.load(Ordering::Relaxed) {
                                    let slice = delay.min(PACING_SLICE.as_millis() as u64);
                                    std::thread::sleep(Duration::from_millis(slice));
                                    delay -= slice;
                                }
                            }
                            // The token bucket caps how much this pass may
                            // read; an empty bucket reads nothing and lets
                            // TCP backpressure do its honest work.
                            let budget = match bucket.as_mut() {
                                Some(bucket) => {
                                    let elapsed = started.elapsed().as_millis() as u64;
                                    let grant = bucket.take(elapsed, buf.len());
                                    if grant == 0 {
                                        std::thread::sleep(Duration::from_millis(10));
                                        continue;
                                    }
                                    grant
                                }
                                None => buf.len(),
                            };
                            match conn.read(&mut buf[..budget]) {
                                Ok(0) => break,
                                Ok(n) => {
                                    bytes_in.fetch_add(n as u64, Ordering::Relaxed);
                                }
                                Err(err)
                                    if err.kind() == std::io::ErrorKind::WouldBlock
                                        || err.kind() == std::io::ErrorKind::TimedOut =>
                                {
                                    continue;
                                }
                                Err(_) => break,
                            }
                        }
                    }
                })?
        };
        Ok(RehearsalSink {
            port,
            bytes_in,
            accepts,
            shutdown,
            thread: Some(thread),
        })
    }

    /// The publish URL a rehearsal lane hands ffmpeg (`url_format` maps
    /// `tcp://` to MPEG-TS).
    pub fn url(&self) -> String {
        format!("tcp://127.0.0.1:{}", self.port)
    }

    /// Total wire bytes this sink has drained — the server-side truth the
    /// stats/report side can cross-check against the muxer's own count.
    pub fn bytes_received(&self) -> u64 {
        self.bytes_in.load(Ordering::Relaxed)
    }

    /// How many times a lane (re)connected — each reconnect drill shows up
    /// here.
    pub fn connections(&self) -> u64 {
        self.accepts.load(Ordering::Relaxed)
    }

    /// Stop accepting and join the thread. Idempotent; also runs on drop.
    pub fn stop(&mut self) {
        self.shutdown.store(true, Ordering::Relaxed);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

impl Drop for RehearsalSink {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::net::TcpStream;
    use std::time::Instant;

    fn wait_until(deadline_ms: u64, mut check: impl FnMut() -> bool) -> bool {
        let start = Instant::now();
        while start.elapsed() < Duration::from_millis(deadline_ms) {
            if check() {
                return true;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        check()
    }

    #[test]
    fn drains_and_counts_across_reconnects() {
        let sink = RehearsalSink::start().expect("sink starts");
        assert!(sink.url().starts_with("tcp://127.0.0.1:"));

        let addr = format!("127.0.0.1:{}", sink.url().rsplit(':').next().unwrap());
        let mut first = TcpStream::connect(&addr).expect("first connect");
        first.write_all(&[7u8; 1000]).expect("write");
        drop(first); // the lane "died"

        // A reconnect finds the sink again — the drill's whole point.
        let mut second = TcpStream::connect(&addr).expect("reconnect");
        second.write_all(&[7u8; 500]).expect("write");
        drop(second);

        assert!(
            wait_until(2000, || sink.bytes_received() == 1500),
            "expected 1500 drained bytes, saw {}",
            sink.bytes_received()
        );
        assert!(sink.connections() >= 2);
    }

    #[test]
    fn shaped_sink_caps_throughput_and_severs_on_the_outage_schedule() {
        // 800 kbps cap = 100_000 bytes/s; outage after 1 healthy second,
        // lasting 1 s, repeating. Short enough for a unit test, real enough
        // to prove the drill mechanics end to end.
        let profile = ShapeProfile {
            bandwidth_kbps: 800,
            latency_ms: 0,
            jitter_ms: 0,
            outage_every_s: 1,
            outage_len_s: 1,
            seed: 42,
        };
        let sink = RehearsalSink::start_shaped(Some(profile)).expect("sink starts");
        let addr = format!("127.0.0.1:{}", sink.url().rsplit(':').next().unwrap());

        let mut conn = TcpStream::connect(&addr).expect("connect");
        conn.set_nodelay(true).ok();
        conn.set_write_timeout(Some(Duration::from_millis(100)))
            .ok();
        // Blast for ~0.9 s of the healthy window; the cap must hold the
        // drained amount near 100 kB (+ the 1 s burst allowance) — far
        // below the ~several MB an unshaped loopback would swallow.
        let start = Instant::now();
        let chunk = [7u8; 8 * 1024];
        let mut offered = 0u64;
        while start.elapsed() < Duration::from_millis(900) {
            match conn.write(&chunk) {
                Ok(n) => offered += n as u64,
                Err(_) => break, // buffers full — backpressure working
            }
        }
        assert!(
            wait_until(1000, || sink.bytes_received() > 0),
            "some shaped bytes must drain"
        );
        let drained = sink.bytes_received();
        assert!(
            drained <= 320_000,
            "the cap must hold: drained {drained} of {offered} offered"
        );

        // Ride into the outage: the connection gets severed...
        assert!(
            wait_until(2500, || conn.write(&chunk).is_err()),
            "the outage must sever the connection"
        );
        // ...and after the outage window a reconnect succeeds again (the
        // schedule tiles: healthy 1 s → out 1 s → healthy ...).
        let reconnected = wait_until(3000, || match TcpStream::connect(&addr) {
            Ok(mut fresh) => {
                fresh
                    .set_write_timeout(Some(Duration::from_millis(50)))
                    .ok();
                fresh.write_all(&[1u8; 64]).is_ok()
            }
            Err(_) => false,
        });
        assert!(reconnected, "a reconnect must land in a healthy window");
    }

    #[test]
    fn stop_refuses_further_connections() {
        let mut sink = RehearsalSink::start().expect("sink starts");
        let addr = format!("127.0.0.1:{}", sink.url().rsplit(':').next().unwrap());
        sink.stop();
        // The listener is gone with the thread — a connect now fails (or is
        // instantly reset); either way no rehearsal traffic flows.
        let refused = match TcpStream::connect(&addr) {
            Err(_) => true,
            Ok(mut conn) => {
                let _ = conn.set_read_timeout(Some(Duration::from_millis(200)));
                let mut buf = [0u8; 1];
                matches!(conn.read(&mut buf), Ok(0) | Err(_))
            }
        };
        assert!(refused, "a stopped sink must not keep a live listener");
        assert_eq!(sink.bytes_received(), 0);
    }
}
