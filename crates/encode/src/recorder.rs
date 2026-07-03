//! The recording session engine: a strict-CFR frame clock over
//! latest-wins video frames + tapped audio blocks, feeding one
//! [`RecordSink`] (the owned `.frec` writer or the labeled ffmpeg muxer).
//!
//! ## Clocking (what keeps A/V in sync — and pause gapless)
//!
//! The pacing thread owns a **recording clock** that only advances while
//! unpaused. Every tick it computes how many frames are *due*
//! (`elapsed × fps`) and writes exactly that many — taking the newest
//! pushed program frame, or **duplicating the last one** when the studio
//! has nothing newer (static scenes, stalls). Frame count therefore stays
//! locked to recorded time: sinks that derive timestamps from frame count
//! (rawvideo → ffmpeg, `.frec`) can never drift against the audio, whose
//! sample positions are counted by the same pause gate in
//! [`RecorderHandle::push_audio_blocks`]. Pausing stops both counters;
//! resuming continues them — one contiguous, playable timeline.
//!
//! Frames and audio go **only** to the sink the session was started with —
//! a file the user chose (or, later, the stream they configured).

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::time::{Duration, Instant};

/// What a session records: canvas geometry, the CFR rate, and which of the
/// mixer's 6 tracks land in the file (ascending order = sink slot order).
#[derive(Debug, Clone)]
pub struct RecordSpec {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    /// Enabled mixer track indices (0-based, ascending). Slot `i` of the
    /// sink is `tracks[i]`.
    pub tracks: Vec<usize>,
}

impl RecordSpec {
    pub fn frame_bytes(&self) -> usize {
        self.width as usize * self.height as usize * 4
    }
}

/// Where finished/failed recordings report their numbers.
#[derive(Debug, Clone, Copy, Default)]
pub struct RecorderStats {
    /// Frames written to the sink (including duplicates).
    pub frames_written: u64,
    /// Writes that reused the previous frame (static scene or a stall).
    pub frames_duplicated: u64,
    /// High-water frame deficit (how far the sink fell behind the clock at
    /// its worst — the deficit itself always drains back to zero).
    pub frames_behind: u64,
    /// Pushed frames whose geometry did not match the spec (canvas change).
    pub frames_wrong_size: u64,
    /// Audio blocks dropped because the queue was full.
    pub audio_blocks_dropped: u64,
}

/// One audio delivery: `(sink slot, absolute sample position, samples)`.
struct AudioMsg {
    slot: usize,
    sample_pos: u64,
    samples: Vec<f32>,
}

/// Everything a running session writes into. Implementations live in
/// [`crate::mux`]. All calls arrive from the one pacing thread.
pub trait RecordSink: Send {
    /// One tightly-packed RGBA frame, exactly `spec.frame_bytes()` long.
    fn write_video(&mut self, pixels: &Arc<Vec<u8>>) -> Result<(), String>;
    /// One interleaved stereo f32 block for sink slot `slot`.
    fn write_audio(&mut self, slot: usize, sample_pos: u64, samples: &[f32]) -> Result<(), String>;
    /// Flush + finalize; returns every file the session produced.
    fn finish(self: Box<Self>) -> Result<Vec<PathBuf>, String>;
}

struct Shared {
    spec: RecordSpec,
    /// Newest program frame, replaced on every push (latest wins).
    video_slot: Mutex<Option<Arc<Vec<u8>>>>,
    paused: AtomicBool,
    stop: AtomicBool,
    /// Audio sample position for the *next* block (advances while unpaused).
    audio_pos: AtomicU64,
    audio_tx: mpsc::SyncSender<AudioMsg>,
    stats: Mutex<RecorderStats>,
    error: Mutex<Option<String>>,
}

/// Cloneable feeding/control handle. The studio thread pushes frames; the
/// audio tap pushes blocks; the UI commands pause/resume.
#[derive(Clone)]
pub struct RecorderHandle {
    shared: Arc<Shared>,
}

impl RecorderHandle {
    /// Push the newest program frame (latest wins; never blocks).
    pub fn push_frame(&self, pixels: Arc<Vec<u8>>) {
        if pixels.len() != self.shared.spec.frame_bytes() {
            self.shared
                .stats
                .lock()
                .expect("stats lock")
                .frames_wrong_size += 1;
            return;
        }
        *self.shared.video_slot.lock().expect("video slot lock") = Some(pixels);
    }

    /// Push one mixed block per enabled track (`(track_index, samples)` in
    /// ascending track order — the audio engine's tap shape). While paused,
    /// blocks are discarded and the sample position holds — that is the
    /// pause gate that keeps resume gapless.
    pub fn push_audio_blocks(&self, blocks: &[(usize, &[f32])]) {
        if blocks.is_empty() {
            return;
        }
        if self.shared.paused.load(Ordering::Relaxed) || self.shared.stop.load(Ordering::Relaxed) {
            return;
        }
        let frames = (blocks[0].1.len() / 2) as u64;
        let pos = self.shared.audio_pos.fetch_add(frames, Ordering::Relaxed);
        for (slot, (_track, samples)) in blocks.iter().enumerate() {
            let msg = AudioMsg {
                slot,
                sample_pos: pos,
                samples: samples.to_vec(),
            };
            if self.shared.audio_tx.try_send(msg).is_err() {
                self.shared
                    .stats
                    .lock()
                    .expect("stats lock")
                    .audio_blocks_dropped += 1;
            }
        }
    }

    pub fn set_paused(&self, paused: bool) {
        self.shared.paused.store(paused, Ordering::Relaxed);
    }

    pub fn is_paused(&self) -> bool {
        self.shared.paused.load(Ordering::Relaxed)
    }

    pub fn stats(&self) -> RecorderStats {
        *self.shared.stats.lock().expect("stats lock")
    }

    /// Recorded duration (excludes pauses) — derived from what was written.
    pub fn duration(&self) -> Duration {
        let stats = self.stats();
        Duration::from_secs_f64(stats.frames_written as f64 / self.shared.spec.fps.max(1) as f64)
    }

    /// A write error the pacing thread hit (the session is effectively dead
    /// and should be stopped).
    pub fn error(&self) -> Option<String> {
        self.shared.error.lock().expect("error lock").clone()
    }
}

/// A running recording session (owns the pacing thread).
pub struct Recorder {
    shared: Arc<Shared>,
    pacing: Option<std::thread::JoinHandle<Result<Vec<PathBuf>, String>>>,
}

impl Recorder {
    /// Start recording into `sink`. The recording clock starts now; the
    /// first written frame is the first pushed frame.
    pub fn start(spec: RecordSpec, sink: Box<dyn RecordSink>) -> Self {
        // ~2.5 s of audio per track — recorder hiccups absorb, runaway fails.
        let (audio_tx, audio_rx) = mpsc::sync_channel::<AudioMsg>(256 * spec.tracks.len().max(1));
        let shared = Arc::new(Shared {
            spec,
            video_slot: Mutex::new(None),
            paused: AtomicBool::new(false),
            stop: AtomicBool::new(false),
            audio_pos: AtomicU64::new(0),
            audio_tx,
            stats: Mutex::new(RecorderStats::default()),
            error: Mutex::new(None),
        });
        let thread_shared = Arc::clone(&shared);
        let pacing = std::thread::Builder::new()
            .name("fcap-recorder".into())
            .spawn(move || run_pacing(thread_shared, audio_rx, sink))
            .expect("recorder pacing thread spawns");
        Recorder {
            shared,
            pacing: Some(pacing),
        }
    }

    pub fn handle(&self) -> RecorderHandle {
        RecorderHandle {
            shared: Arc::clone(&self.shared),
        }
    }

    /// Stop and finalize. Blocking (container trailers can take a moment) —
    /// call from a worker thread. Returns the finished file paths.
    pub fn stop(mut self) -> Result<Vec<PathBuf>, String> {
        self.shared.stop.store(true, Ordering::Relaxed);
        match self.pacing.take() {
            Some(handle) => handle
                .join()
                .unwrap_or_else(|_| Err("the recorder thread panicked".to_string())),
            None => Err("the recorder was already stopped".to_string()),
        }
    }
}

impl Drop for Recorder {
    fn drop(&mut self) {
        // A dropped-without-stop recorder still winds its thread down.
        self.shared.stop.store(true, Ordering::Relaxed);
        if let Some(handle) = self.pacing.take() {
            let _ = handle.join();
        }
    }
}

fn run_pacing(
    shared: Arc<Shared>,
    audio_rx: mpsc::Receiver<AudioMsg>,
    mut sink: Box<dyn RecordSink>,
) -> Result<Vec<PathBuf>, String> {
    let fps = shared.spec.fps.max(1) as u64;
    let tick = Duration::from_micros(1_000_000 / fps / 2).max(Duration::from_millis(2));
    /// Catch-up cap per tick: a long stall recovers over several ticks
    /// instead of hammering the sink with a burst.
    const MAX_CATCH_UP: u64 = 4;

    let mut last_frame: Option<Arc<Vec<u8>>> = None;
    // The recording clock: accumulated unpaused time. `epoch` is Some while
    // running (unpaused) and marks when the current run segment began.
    let mut recorded = Duration::ZERO;
    let mut epoch: Option<Instant> = None;
    let mut result: Result<(), String> = Ok(());

    'session: loop {
        let stopping = shared.stop.load(Ordering::Relaxed);
        let paused = shared.paused.load(Ordering::Relaxed);

        // -- audio first (keeps sink-side interleave close to real time) ----
        while let Ok(msg) = audio_rx.try_recv() {
            if let Err(err) = sink.write_audio(msg.slot, msg.sample_pos, &msg.samples) {
                result = Err(err);
                break 'session;
            }
        }

        if stopping {
            break;
        }

        // -- the recording clock -------------------------------------------
        if paused {
            if let Some(started) = epoch.take() {
                recorded += started.elapsed();
            }
            // Discard anything staged pre-pause so resume records what is
            // on the canvas *then*, not a stale frame.
            shared.video_slot.lock().expect("video slot lock").take();
            std::thread::sleep(tick);
            continue;
        }

        // First frame arms the clock — recording starts when content does.
        if epoch.is_none() {
            let has_frame = shared.video_slot.lock().expect("video slot lock").is_some();
            if !has_frame && last_frame.is_none() {
                std::thread::sleep(tick);
                continue;
            }
            epoch = Some(Instant::now());
        }

        let elapsed = recorded + epoch.map(|started| started.elapsed()).unwrap_or_default();
        let due = (elapsed.as_secs_f64() * fps as f64) as u64;
        let written = shared.stats.lock().expect("stats lock").frames_written;
        let deficit = due.saturating_sub(written);
        // Cap the per-tick burst; the remaining deficit carries over and
        // drains on later ticks (at up to ~cap × tick rate), so the frame
        // count always reconverges with the clock — audio sample positions
        // advanced through any stall, and abandoning frames would desync.
        let to_write = deficit.min(MAX_CATCH_UP);
        {
            let mut stats = shared.stats.lock().expect("stats lock");
            stats.frames_behind = stats.frames_behind.max(deficit.saturating_sub(1));
        }

        for _ in 0..to_write {
            let newest = shared.video_slot.lock().expect("video slot lock").take();
            let duplicated = newest.is_none();
            let Some(frame) = newest.or_else(|| last_frame.clone()) else {
                break; // nothing ever pushed (unreachable once armed)
            };
            if let Err(err) = sink.write_video(&frame) {
                result = Err(err);
                break 'session;
            }
            let mut stats = shared.stats.lock().expect("stats lock");
            stats.frames_written += 1;
            if duplicated {
                stats.frames_duplicated += 1;
            }
            drop(stats);
            last_frame = Some(frame);
        }

        std::thread::sleep(tick);
    }

    // Drain what the tap already delivered, then finalize.
    while let Ok(msg) = audio_rx.try_recv() {
        if result.is_ok() {
            if let Err(err) = sink.write_audio(msg.slot, msg.sample_pos, &msg.samples) {
                result = Err(err);
            }
        }
    }
    let finish = sink.finish();
    if let Err(err) = &result {
        *shared.error.lock().expect("error lock") = Some(err.clone());
        return Err(err.clone());
    }
    finish
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A sink that records what reaches it.
    #[derive(Default)]
    struct MockSink {
        videos: Arc<Mutex<Vec<Arc<Vec<u8>>>>>,
        audio: Arc<Mutex<Vec<(usize, u64, usize)>>>,
        finished: Arc<AtomicBool>,
    }

    impl RecordSink for MockSink {
        fn write_video(&mut self, pixels: &Arc<Vec<u8>>) -> Result<(), String> {
            self.videos.lock().expect("lock").push(Arc::clone(pixels));
            Ok(())
        }
        fn write_audio(
            &mut self,
            slot: usize,
            sample_pos: u64,
            samples: &[f32],
        ) -> Result<(), String> {
            self.audio
                .lock()
                .expect("lock")
                .push((slot, sample_pos, samples.len()));
            Ok(())
        }
        fn finish(self: Box<Self>) -> Result<Vec<PathBuf>, String> {
            self.finished.store(true, Ordering::Relaxed);
            Ok(vec![PathBuf::from("mock.out")])
        }
    }

    fn spec_30fps() -> RecordSpec {
        RecordSpec {
            width: 4,
            height: 2,
            fps: 30,
            tracks: vec![0, 2],
        }
    }

    fn frame(spec: &RecordSpec, fill: u8) -> Arc<Vec<u8>> {
        Arc::new(vec![fill; spec.frame_bytes()])
    }

    #[test]
    fn cfr_pacing_duplicates_static_content() {
        let spec = spec_30fps();
        let sink = MockSink::default();
        let videos = Arc::clone(&sink.videos);
        let finished = Arc::clone(&sink.finished);
        let recorder = Recorder::start(spec.clone(), Box::new(sink));
        let handle = recorder.handle();

        handle.push_frame(frame(&spec, 1)); // one frame, then nothing new
        std::thread::sleep(Duration::from_millis(400));
        let paths = recorder.stop().expect("stops clean");
        assert_eq!(paths.len(), 1);
        assert!(finished.load(Ordering::Relaxed));

        let written = videos.lock().expect("lock").len() as u64;
        // ~400 ms at 30 fps ≈ 12 frames; generous window for CI jitter.
        assert!(
            (6..=20).contains(&written),
            "CFR should hold near 12 frames, wrote {written}"
        );
        let stats = handle.stats();
        assert_eq!(stats.frames_written, written);
        assert!(
            stats.frames_duplicated >= written - 1,
            "all but the first frame are duplicates of the static push"
        );
    }

    #[test]
    fn pause_freezes_both_clocks_and_resume_continues() {
        let spec = spec_30fps();
        let sink = MockSink::default();
        let videos = Arc::clone(&sink.videos);
        let audio = Arc::clone(&sink.audio);
        let recorder = Recorder::start(spec.clone(), Box::new(sink));
        let handle = recorder.handle();

        let block = vec![0.25f32; 960];
        let blocks: Vec<(usize, &[f32])> = vec![(0, &block), (2, &block)];

        handle.push_frame(frame(&spec, 1));
        handle.push_audio_blocks(&blocks);
        std::thread::sleep(Duration::from_millis(200));

        handle.set_paused(true);
        std::thread::sleep(Duration::from_millis(50)); // let the pacer see it
        let frames_at_pause = videos.lock().expect("lock").len();
        let pos_at_pause = handle.shared.audio_pos.load(Ordering::Relaxed);
        // Paused: pushed audio is discarded, the position holds.
        handle.push_audio_blocks(&blocks);
        handle.push_audio_blocks(&blocks);
        std::thread::sleep(Duration::from_millis(200));
        assert_eq!(
            handle.shared.audio_pos.load(Ordering::Relaxed),
            pos_at_pause,
            "sample position must not advance while paused"
        );
        let frames_during_pause = videos.lock().expect("lock").len() - frames_at_pause;
        assert!(
            frames_during_pause <= 1,
            "no frames written while paused (got {frames_during_pause})"
        );

        handle.set_paused(false);
        handle.push_frame(frame(&spec, 2));
        handle.push_audio_blocks(&blocks);
        std::thread::sleep(Duration::from_millis(200));
        let recorder_stats = handle.stats();
        recorder.stop().expect("stops clean");

        assert!(
            videos.lock().expect("lock").len() > frames_at_pause,
            "resume writes again"
        );
        // Audio positions are contiguous: pos of the post-resume block ==
        // pos at pause (the pause gap does not exist on the timeline).
        let positions: Vec<u64> = audio
            .lock()
            .expect("lock")
            .iter()
            .filter(|(slot, _, _)| *slot == 0)
            .map(|(_, pos, _)| *pos)
            .collect();
        assert!(
            positions.windows(2).all(|w| w[1] == w[0] + 480),
            "sample positions stay contiguous across the pause: {positions:?}"
        );
        assert_eq!(recorder_stats.frames_wrong_size, 0);
    }

    #[test]
    fn wrong_sized_frames_are_counted_not_written() {
        let spec = spec_30fps();
        let sink = MockSink::default();
        let videos = Arc::clone(&sink.videos);
        let recorder = Recorder::start(spec, Box::new(sink));
        let handle = recorder.handle();

        handle.push_frame(Arc::new(vec![0u8; 7])); // wrong geometry
        std::thread::sleep(Duration::from_millis(120));
        recorder.stop().expect("stops clean");

        assert_eq!(videos.lock().expect("lock").len(), 0);
        assert_eq!(handle.stats().frames_wrong_size, 1);
    }

    #[test]
    fn audio_slots_map_ascending_tracks() {
        let spec = spec_30fps(); // tracks 0 and 2 → slots 0 and 1
        let sink = MockSink::default();
        let audio = Arc::clone(&sink.audio);
        let recorder = Recorder::start(spec.clone(), Box::new(sink));
        let handle = recorder.handle();

        let block = vec![0.5f32; 960];
        handle.push_frame(frame(&spec, 3));
        handle.push_audio_blocks(&[(0, &block), (2, &block)]);
        std::thread::sleep(Duration::from_millis(120));
        recorder.stop().expect("stops clean");

        let slots: Vec<usize> = audio
            .lock()
            .expect("lock")
            .iter()
            .map(|(slot, _, _)| *slot)
            .collect();
        assert_eq!(slots, vec![0, 1], "track 0 → slot 0, track 2 → slot 1");
    }

    #[test]
    fn a_sink_error_surfaces_on_the_handle_and_stop() {
        struct FailingSink;
        impl RecordSink for FailingSink {
            fn write_video(&mut self, _pixels: &Arc<Vec<u8>>) -> Result<(), String> {
                Err("disk full".to_string())
            }
            fn write_audio(&mut self, _: usize, _: u64, _: &[f32]) -> Result<(), String> {
                Ok(())
            }
            fn finish(self: Box<Self>) -> Result<Vec<PathBuf>, String> {
                Ok(vec![])
            }
        }
        let spec = spec_30fps();
        let recorder = Recorder::start(spec.clone(), Box::new(FailingSink));
        let handle = recorder.handle();
        handle.push_frame(frame(&spec, 1));
        std::thread::sleep(Duration::from_millis(150));
        assert_eq!(handle.error().as_deref(), Some("disk full"));
        assert!(recorder.stop().is_err(), "stop reports the write error");
    }
}
