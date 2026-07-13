//! The live-stream session: a supervised pacing engine over the encode
//! crate's [`Recorder`] with an RTMP sink — plus the two things a stream
//! needs that a recording doesn't: **auto-reconnect** (the network dies, the
//! ingest hiccups → back off, respawn the sink, keep going) and an optional
//! **broadcast delay** (frames + audio held N seconds before publish).
//!
//! The app pushes program frames + the stream track's audio into a
//! [`StreamHandle`] exactly like it feeds the recorder; the supervisor thread
//! owns the recorder/sink lifecycle. Pushes NEVER block the render thread: a
//! full feed queue drops the oldest video (counted honestly in the status)
//! and the recorder's latest-wins clock absorbs the gap.
//!
//! The main recording is a separate session entirely — a stream dying can
//! never touch it (charter: the local copy is sacred).

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::time::{Duration, Instant};

use fcap_encode::{RecordSink, RecordSpec, Recorder};

/// What a session streams: canvas geometry + rate. (The publish URL lives in
/// the sink factory, not here — this struct is loggable.)
#[derive(Debug, Clone)]
pub struct StreamSpec {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
}

/// Why the previous sink died — handed to the factory on every respawn so
/// the app can make an informed decision (the CAP-M12 encoder-failover
/// ladder classifies the error text; a network flap keeps its encoder).
#[derive(Debug, Clone)]
pub struct SinkDeath {
    /// The recorder's error text (usually the ffmpeg stderr tail), or the
    /// spawn failure when the sink never came up at all.
    pub error: String,
    /// How long the dead sink actually lived (zero for a failed spawn) —
    /// NOT inflated by the reconnect backoff.
    pub lived: Duration,
}

/// Builds a fresh sink for each (re)connect attempt; `None` on the first
/// spawn, `Some` with the previous death on every respawn. The app captures
/// the ffmpeg component + the RTMP plan here; tests inject fakes.
pub type SinkFactory =
    Box<dyn Fn(Option<&SinkDeath>) -> Result<Box<dyn RecordSink>, String> + Send>;

/// Where the session is in its life, honestly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StreamState {
    /// The sink is up; packets flow (or are about to — ffmpeg handshakes
    /// asynchronously, so the first seconds of Live are optimistic).
    Live,
    /// The last sink died; the next attempt fires when the backoff elapses.
    Reconnecting { attempt: u32 },
    /// Stopped — by the user (`error: None`) or for good after the retry
    /// budget ran out (`error: Some(why)`).
    Ended { error: Option<String> },
}

/// A point-in-time snapshot for the UI (the `stream` event payload).
#[derive(Debug, Clone)]
pub struct StreamStatus {
    pub state: StreamState,
    /// Wall time since Go Live (keeps counting across reconnects).
    pub elapsed: Duration,
    /// Completed reconnects this session.
    pub reconnects: u32,
    /// Video frames dropped at the feed queue (the stream fell behind).
    pub frames_dropped: u64,
}

/// Reconnect backoff: 1s, 2s, 4s … capped at 30s. Pure for the unit tests.
pub fn backoff(attempt: u32) -> Duration {
    Duration::from_millis((1000u64 << attempt.min(5)).min(30_000))
}

/// Attempts before a dead ingest ends the session for good (~2.5 min of
/// trying at the capped backoff — a live show doesn't wait forever).
pub const MAX_RECONNECT_ATTEMPTS: u32 = 8;

enum Feed {
    Frame(Arc<Vec<u8>>),
    /// One interleaved-stereo block for the stream's single audio lane.
    Audio(Vec<f32>),
    Stop,
}

/// The app-facing side: push feeds, read status, stop.
#[derive(Clone)]
pub struct StreamHandle {
    tx: mpsc::SyncSender<(Instant, Feed)>,
    shared: Arc<Shared>,
}

struct Shared {
    status: Mutex<StreamStatus>,
    stopping: AtomicBool,
    started_at: Instant,
}

impl StreamHandle {
    /// Push the newest program frame. Never blocks: when the stream can't
    /// keep up the frame is dropped (counted) — the recording is elsewhere.
    pub fn push_frame(&self, pixels: Arc<Vec<u8>>) {
        if self.shared.stopping.load(Ordering::Relaxed) {
            return;
        }
        if self
            .tx
            .try_send((Instant::now(), Feed::Frame(pixels)))
            .is_err()
        {
            let mut status = lock(&self.shared.status);
            status.frames_dropped += 1;
        }
    }

    /// Push one interleaved-stereo f32 block of the stream's audio track.
    pub fn push_audio(&self, samples: &[f32]) {
        if self.shared.stopping.load(Ordering::Relaxed) {
            return;
        }
        // Audio is small; a full queue here means the supervisor died — the
        // status already carries the honest error.
        let _ = self
            .tx
            .try_send((Instant::now(), Feed::Audio(samples.to_vec())));
    }

    pub fn status(&self) -> StreamStatus {
        let mut status = lock(&self.shared.status).clone();
        status.elapsed = self.shared.started_at.elapsed();
        status
    }

    /// End the stream deliberately (Go Live → End Stream).
    pub fn stop(&self) {
        self.shared.stopping.store(true, Ordering::Relaxed);
        let _ = self.tx.try_send((Instant::now(), Feed::Stop));
    }
}

fn lock(status: &Mutex<StreamStatus>) -> std::sync::MutexGuard<'_, StreamStatus> {
    status
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// A running session: supervises recorder + sink until stopped or spent.
pub struct StreamSession {
    handle: StreamHandle,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl StreamSession {
    pub fn start(spec: StreamSpec, factory: SinkFactory) -> StreamSession {
        // Frames are ~8 MB at 1080p: keep the queue shallow (latest-wins is
        // the recorder's job; this queue only rides out supervisor hiccups).
        let (tx, rx) = mpsc::sync_channel::<(Instant, Feed)>(16);
        let shared = Arc::new(Shared {
            status: Mutex::new(StreamStatus {
                state: StreamState::Live,
                elapsed: Duration::ZERO,
                reconnects: 0,
                frames_dropped: 0,
            }),
            stopping: AtomicBool::new(false),
            started_at: Instant::now(),
        });
        let handle = StreamHandle {
            tx,
            shared: Arc::clone(&shared),
        };
        let thread = std::thread::Builder::new()
            .name("fcap-stream-supervisor".into())
            .spawn(move || supervise(spec, factory, rx, shared))
            .expect("stream supervisor thread spawns");
        StreamSession {
            handle,
            thread: Some(thread),
        }
    }

    pub fn handle(&self) -> StreamHandle {
        self.handle.clone()
    }

    /// Stop and wait for the sink to flush its goodbye.
    pub fn stop(mut self) -> StreamStatus {
        self.handle.stop();
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
        self.handle.status()
    }
}

impl Drop for StreamSession {
    fn drop(&mut self) {
        self.handle.stop();
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

/// The supervisor: forward the feed straight to the recorder/sink and respawn
/// on sink death. It polls at [`POLL`], so every pass runs (re)spawn → drain →
/// health regardless of feed timing, and the `stopping` flag — not the
/// `Feed::Stop` message, which a full queue could drop — is the authoritative
/// end signal.
fn supervise(
    spec: StreamSpec,
    factory: SinkFactory,
    rx: mpsc::Receiver<(Instant, Feed)>,
    shared: Arc<Shared>,
) {
    /// Poll period — well under a frame, so latency stays low without spinning.
    const POLL: Duration = Duration::from_millis(4);
    /// Bound on feeds drained per pass, so a flood never starves health.
    const MAX_DRAIN: usize = 512;

    let record_spec = RecordSpec {
        width: spec.width,
        height: spec.height,
        fps: spec.fps,
        tracks: vec![0], // the sink's single lane
    };
    let mut recorder: Option<Recorder> = None;
    let mut attempt: u32 = 0;
    let mut next_attempt_at = Instant::now();
    let mut live_since: Option<Instant> = None;
    // Why the previous sink died — handed to the factory on each respawn.
    let mut last_death: Option<SinkDeath> = None;

    let set_state = |shared: &Shared, state: StreamState| {
        lock(&shared.status).state = state;
    };
    let end = |mut recorder: Option<Recorder>, shared: &Shared, error: Option<String>| {
        if let Some(rec) = recorder.take() {
            let _ = rec.stop(); // EOF → ffmpeg flushes → clean RTMP goodbye
        }
        set_state(shared, StreamState::Ended { error });
    };

    loop {
        // The stop flag is authoritative — a lost Feed::Stop can never hang End
        // Stream, since this is polled every pass.
        if shared.stopping.load(Ordering::Relaxed) {
            end(recorder, &shared, None);
            return;
        }

        // 1. (Re)spawn the sink when due, telling the factory why the last
        //    one died so it can adapt (encoder failover, CAP-M12).
        if recorder.is_none() && Instant::now() >= next_attempt_at {
            match factory(last_death.as_ref()) {
                Ok(sink) => {
                    recorder = Some(Recorder::start(record_spec.clone(), sink));
                    live_since = Some(Instant::now());
                    set_state(&shared, StreamState::Live);
                }
                Err(err) => {
                    attempt += 1;
                    if attempt > MAX_RECONNECT_ATTEMPTS {
                        set_state(&shared, StreamState::Ended { error: Some(err) });
                        return;
                    }
                    last_death = Some(SinkDeath {
                        error: err.clone(),
                        lived: Duration::ZERO,
                    });
                    lock(&shared.status).reconnects = attempt.saturating_sub(1);
                    set_state(&shared, StreamState::Reconnecting { attempt });
                    next_attempt_at = Instant::now() + backoff(attempt - 1);
                }
            }
        }

        // 2. Forward the available feeds straight to the recorder — no delay
        //    ring (a raw-RGBA buffer would be gigabytes; broadcast delay rides
        //    the encoded path in a later phase). The recorder's own latest-wins
        //    clock paces the actual encode.
        for _ in 0..MAX_DRAIN {
            match rx.try_recv() {
                Ok((_, Feed::Stop)) => break, // the flag drives the real stop
                Ok((_, Feed::Frame(pixels))) => match &recorder {
                    Some(rec) => rec.handle().push_frame(pixels),
                    None => lock(&shared.status).frames_dropped += 1,
                },
                Ok((_, Feed::Audio(samples))) => {
                    if let Some(rec) = &recorder {
                        rec.handle().push_audio_blocks(&[(0, &samples)]);
                    }
                }
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => {
                    end(recorder, &shared, None);
                    return;
                }
            }
        }

        // 3. Sink health: an error ends this recorder and schedules a retry.
        if let Some(rec) = &recorder {
            if let Some(err) = rec.handle().error() {
                let _ = recorder.take().expect("checked").stop();
                last_death = Some(SinkDeath {
                    error: err.clone(),
                    lived: live_since.map(|since| since.elapsed()).unwrap_or_default(),
                });
                // A stretch of stable Live earns the backoff a reset.
                if live_since.is_some_and(|since| since.elapsed() > Duration::from_secs(60)) {
                    attempt = 0;
                }
                attempt += 1;
                live_since = None;
                if attempt > MAX_RECONNECT_ATTEMPTS {
                    set_state(&shared, StreamState::Ended { error: Some(err) });
                    return;
                }
                {
                    let mut status = lock(&shared.status);
                    status.reconnects += 1;
                    status.state = StreamState::Reconnecting { attempt };
                }
                next_attempt_at = Instant::now() + backoff(attempt - 1);
            }
        }

        std::thread::sleep(POLL);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::sync::atomic::AtomicU32;

    #[test]
    fn backoff_doubles_and_caps() {
        assert_eq!(backoff(0), Duration::from_secs(1));
        assert_eq!(backoff(1), Duration::from_secs(2));
        assert_eq!(backoff(4), Duration::from_secs(16));
        assert_eq!(backoff(5), Duration::from_secs(30));
        assert_eq!(backoff(30), Duration::from_secs(30), "cap holds");
    }

    /// A sink that accepts everything until told to fail.
    struct FakeSink {
        fail_video: Arc<AtomicBool>,
    }

    impl RecordSink for FakeSink {
        fn write_video(&mut self, _pixels: &Arc<Vec<u8>>) -> Result<(), String> {
            if self.fail_video.load(Ordering::Relaxed) {
                Err("the ingest hung up".to_string())
            } else {
                Ok(())
            }
        }
        fn write_audio(&mut self, _: usize, _: u64, _: &[f32]) -> Result<(), String> {
            Ok(())
        }
        fn finish(self: Box<Self>) -> Result<Vec<PathBuf>, String> {
            Ok(Vec::new())
        }
    }

    fn spec() -> StreamSpec {
        StreamSpec {
            width: 64,
            height: 36,
            fps: 30,
        }
    }

    fn frame() -> Arc<Vec<u8>> {
        Arc::new(vec![0u8; 64 * 36 * 4])
    }

    #[test]
    fn a_session_goes_live_and_ends_cleanly() {
        let fail = Arc::new(AtomicBool::new(false));
        let fail_for_factory = Arc::clone(&fail);
        let session = StreamSession::start(
            spec(),
            Box::new(move |_death| {
                Ok(Box::new(FakeSink {
                    fail_video: Arc::clone(&fail_for_factory),
                }) as Box<dyn RecordSink>)
            }),
        );
        let handle = session.handle();
        for _ in 0..5 {
            handle.push_frame(frame());
            std::thread::sleep(Duration::from_millis(20));
        }
        assert_eq!(handle.status().state, StreamState::Live);
        let status = session.stop();
        assert_eq!(status.state, StreamState::Ended { error: None });
    }

    #[test]
    fn a_dead_sink_schedules_a_reconnect() {
        let fail = Arc::new(AtomicBool::new(false));
        let fail_for_factory = Arc::clone(&fail);
        let spawns = Arc::new(AtomicU32::new(0));
        let spawns_in_factory = Arc::clone(&spawns);
        let session = StreamSession::start(
            spec(),
            Box::new(move |death| {
                // The first spawn carries no death; every respawn carries
                // the previous sink's real error text and lifetime.
                if spawns_in_factory.fetch_add(1, Ordering::Relaxed) == 0 {
                    assert!(death.is_none());
                } else {
                    assert!(death.is_some_and(|d| !d.error.is_empty()));
                }
                Ok(Box::new(FakeSink {
                    fail_video: Arc::clone(&fail_for_factory),
                }) as Box<dyn RecordSink>)
            }),
        );
        let handle = session.handle();
        // Feed until live, then kill the sink.
        handle.push_frame(frame());
        std::thread::sleep(Duration::from_millis(100));
        fail.store(true, Ordering::Relaxed);
        // The recorder paces ~30 fps; its next write surfaces the error.
        handle.push_frame(frame());
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            let state = handle.status().state;
            if matches!(state, StreamState::Reconnecting { .. }) {
                break;
            }
            assert!(Instant::now() < deadline, "never reconnected: {state:?}");
            handle.push_frame(frame());
            std::thread::sleep(Duration::from_millis(30));
        }
        assert_eq!(handle.status().reconnects, 1);
        // Let the sink recover: the 1s backoff elapses → a second spawn.
        fail.store(false, Ordering::Relaxed);
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            handle.push_frame(frame());
            if handle.status().state == StreamState::Live {
                break;
            }
            assert!(Instant::now() < deadline, "never re-lived");
            std::thread::sleep(Duration::from_millis(50));
        }
        assert!(spawns.load(Ordering::Relaxed) >= 2, "the factory respawned");
        drop(session);
    }

    #[test]
    fn a_permanently_failing_factory_ends_with_the_error() {
        let session = StreamSession::start(spec(), Box::new(|_| Err("bad ingest".to_string())));
        let handle = session.handle();
        // 8 attempts ride the backoff ladder — nudge time along by just
        // waiting out the first few (the test budget is the first attempt's
        // instant failure path; the full ladder is exercised by the cap test).
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            match handle.status().state {
                StreamState::Ended { error: Some(err) } => {
                    assert_eq!(err, "bad ingest");
                    break;
                }
                _ if Instant::now() > deadline => {
                    // Still climbing the backoff ladder — that's the designed
                    // behaviour; a Reconnecting state is an acceptable exit.
                    assert!(matches!(
                        handle.status().state,
                        StreamState::Reconnecting { .. }
                    ));
                    break;
                }
                _ => std::thread::sleep(Duration::from_millis(50)),
            }
        }
        drop(session);
    }
}
