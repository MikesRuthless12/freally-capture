//! Simultaneous multistream (Phase 6): one engine supervising several
//! publish targets at once, **direct to each platform** — no restream server.
//!
//! Targets whose encode settings match (equal opaque `signature`) **share a
//! single encoded bitstream**: they ride one ffmpeg process through the tee
//! muxer, so a second platform costs no second encode. Targets with different
//! settings get their own lane (their own supervised [`StreamSession`]).
//!
//! **Independence is the invariant:** when one slave of a shared lane fails
//! (the classic bad/expired key), tee keeps publishing the healthy siblings
//! (`onfail=ignore`) and this engine **splits the failed target out** into
//! its own single-target lane, where the normal reconnect/backoff ladder
//! owns it — one dead target can never take a healthy one down with it. A
//! lane's whole process dying (network cut) reconnects lane-wide exactly
//! like the Phase 5 single-target session did.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::session::{SinkFactory, StreamHandle, StreamSession, StreamSpec, StreamState};

/// One stream target as the engine sees it: identity + the encode signature
/// that decides sharing. Publish URLs and keys stay app-side, captured
/// inside the [`LaneMaker`] — this struct is loggable.
#[derive(Debug, Clone)]
pub struct MemberSpec {
    /// Stable id — the app's index for this target (its settings row).
    pub id: usize,
    /// Display label (the service name) — never the key.
    pub label: String,
    /// The mixer track this target streams (1-based, like the UI dots).
    pub track: u8,
    /// Which canvas feeds this target (0 = program, 1 = vertical). Encoded
    /// into the signature by the app, so lanes never mix canvases.
    pub canvas: u8,
    /// This target's frame geometry (its canvas's dimensions).
    pub width: u32,
    pub height: u32,
    /// This target's frame rate (paces its lane).
    pub fps: u32,
    /// Opaque encode signature: equal signature (and `tee_safe`) = one
    /// shared encode. Must encode everything that shapes the bitstream —
    /// encoder, rates, keyframe interval, fps, **and the audio track**.
    pub signature: String,
    /// Whether the publish URL can ride inside a tee spec (no tee syntax).
    pub tee_safe: bool,
    /// Configured video+audio bitrate — the honest stats fallback when the
    /// muxer can't report measured bytes (tee reports no aggregate).
    pub nominal_kbps: u32,
}

/// The shared cells a lane's factory and the engine both touch. The maker
/// creates them once per lane; the factory re-reads `members` at every
/// (re)spawn so a split-out target stops being published by its old lane.
#[derive(Clone)]
pub struct LaneCells {
    /// The lane's current member ids (the engine shrinks this on split-out).
    pub members: Arc<Mutex<Vec<usize>>>,
    /// The member order the **currently running** process publishes (the
    /// factory snapshots `members` here at each spawn) — maps a tee slave
    /// index back to a member id.
    pub spawn_order: Arc<Mutex<Vec<usize>>>,
    /// Tee slave indexes (into `spawn_order`) reported failed by ffmpeg.
    pub slave_failures: Arc<Mutex<Vec<usize>>>,
    /// The muxer's bytes-out counter (0 = the muxer can't tell).
    pub bytes_out: Arc<AtomicU64>,
}

impl LaneCells {
    pub fn new(ids: &[usize]) -> LaneCells {
        LaneCells {
            members: Arc::new(Mutex::new(ids.to_vec())),
            spawn_order: Arc::new(Mutex::new(ids.to_vec())),
            slave_failures: Arc::new(Mutex::new(Vec::new())),
            bytes_out: Arc::new(AtomicU64::new(0)),
        }
    }
}

/// What the maker returns for a lane over an ordered member set.
pub struct LaneIo {
    pub factory: SinkFactory,
    pub cells: LaneCells,
}

/// Builds the IO for a lane. Called once per initial signature group and
/// once per split-out (always a single id then). Building is infallible —
/// the fallible work (spawning ffmpeg) lives inside the returned factory.
pub type LaneMaker = Box<dyn Fn(&[usize]) -> LaneIo + Send>;

/// A point-in-time, per-target snapshot for the UI.
#[derive(Debug, Clone)]
pub struct MemberStatus {
    pub id: usize,
    pub label: String,
    pub state: StreamState,
    pub reconnects: u32,
    pub frames_dropped: u64,
    /// Measured publish bitrate where the muxer reports bytes; the
    /// configured bitrate while live on a shared (tee) lane; 0 otherwise.
    pub kbps: u32,
    /// How many **other** targets share this target's encode right now.
    pub shared_with: usize,
}

struct Lane {
    session: Option<StreamSession>,
    handle: StreamHandle,
    cells: LaneCells,
    track: u8,
    canvas: u8,
    /// Measured bitrate bookkeeping (from `cells.bytes_out` deltas).
    last_bytes: u64,
    last_sample: Instant,
    kbps: u32,
}

struct MultiShared {
    lanes: Mutex<Vec<Lane>>,
    started_at: Instant,
    stopping: AtomicBool,
}

fn lock<'a, T>(mutex: &'a Mutex<T>) -> std::sync::MutexGuard<'a, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// Group members into lanes: equal signatures share (in first-seen order),
/// tee-unsafe URLs always get their own lane. Pure for the unit tests.
pub fn group_members(members: &[MemberSpec]) -> Vec<Vec<usize>> {
    let mut groups: Vec<(Option<&str>, Vec<usize>)> = Vec::new();
    for member in members {
        if !member.tee_safe {
            groups.push((None, vec![member.id]));
            continue;
        }
        match groups
            .iter_mut()
            .find(|(sig, _)| *sig == Some(member.signature.as_str()))
        {
            Some((_, ids)) => ids.push(member.id),
            None => groups.push((Some(member.signature.as_str()), vec![member.id])),
        }
    }
    groups.into_iter().map(|(_, ids)| ids).collect()
}

/// The app-facing side of a running multistream: push feeds, read per-target
/// statuses, stop. Cheap to clone.
#[derive(Clone)]
pub struct MultiHandle {
    shared: Arc<MultiShared>,
    members: Arc<Vec<MemberSpec>>,
}

impl MultiHandle {
    /// Push the newest frame of `canvas` (0 = program, 1 = vertical) to the
    /// lanes streaming that canvas (an `Arc` clone each — never a pixel
    /// copy, never blocking).
    pub fn push_frame(&self, canvas: u8, pixels: Arc<Vec<u8>>) {
        for lane in lock(&self.shared.lanes).iter() {
            if lane.canvas == canvas {
                lane.handle.push_frame(Arc::clone(&pixels));
            }
        }
    }

    /// Whether any lane currently streams `canvas`.
    pub fn wants_canvas(&self, canvas: u8) -> bool {
        lock(&self.shared.lanes)
            .iter()
            .any(|lane| lane.canvas == canvas)
    }

    /// Push one 10 ms interleaved-stereo block of mixer track
    /// `track_index` (0-based) to the lanes streaming that track.
    pub fn push_audio(&self, track_index: usize, samples: &[f32]) {
        for lane in lock(&self.shared.lanes).iter() {
            if usize::from(lane.track.saturating_sub(1)) == track_index {
                lane.handle.push_audio(samples);
            }
        }
    }

    /// Wall time since Go Live.
    pub fn elapsed(&self) -> Duration {
        self.shared.started_at.elapsed()
    }

    /// Per-target snapshots, ordered by target id.
    pub fn statuses(&self) -> Vec<MemberStatus> {
        let mut out = Vec::new();
        for lane in lock(&self.shared.lanes).iter() {
            let status = lane.handle.status();
            let ids = lock(&lane.cells.members).clone();
            for id in &ids {
                let Some(member) = self.members.iter().find(|m| m.id == *id) else {
                    continue;
                };
                let kbps = if lane.kbps > 0 {
                    lane.kbps
                } else if status.state == StreamState::Live {
                    member.nominal_kbps
                } else {
                    0
                };
                out.push(MemberStatus {
                    id: *id,
                    label: member.label.clone(),
                    state: status.state.clone(),
                    reconnects: status.reconnects,
                    frames_dropped: status.frames_dropped,
                    kbps,
                    shared_with: ids.len() - 1,
                });
            }
        }
        out.sort_by_key(|status| status.id);
        out
    }

    /// Lane-deduplicated totals (a shared lane counts once).
    pub fn totals(&self) -> (u32, u64) {
        let mut reconnects = 0;
        let mut dropped = 0;
        for lane in lock(&self.shared.lanes).iter() {
            let status = lane.handle.status();
            reconnects += status.reconnects;
            dropped += status.frames_dropped;
        }
        (reconnects, dropped)
    }

    /// End the whole multistream (all lanes) deliberately.
    pub fn stop(&self) {
        self.shared.stopping.store(true, Ordering::Relaxed);
        for lane in lock(&self.shared.lanes).iter() {
            lane.handle.stop();
        }
    }
}

/// A running multistream: the lanes + the engine thread that polls slave
/// failures (split-out) and measures bitrates.
pub struct MultiSession {
    handle: MultiHandle,
    engine: Option<std::thread::JoinHandle<()>>,
}

impl MultiSession {
    /// Go live to every member. Each lane paces at its group's geometry/fps
    /// (per-member — a vertical-canvas lane records its own dimensions).
    pub fn start(members: Vec<MemberSpec>, maker: LaneMaker) -> MultiSession {
        let members = Arc::new(members);
        let mut lanes = Vec::new();
        for ids in group_members(&members) {
            lanes.push(spawn_lane(&members, &maker, &ids));
        }
        let shared = Arc::new(MultiShared {
            lanes: Mutex::new(lanes),
            started_at: Instant::now(),
            stopping: AtomicBool::new(false),
        });
        let handle = MultiHandle {
            shared: Arc::clone(&shared),
            members: Arc::clone(&members),
        };
        let engine = std::thread::Builder::new()
            .name("fcap-multistream-engine".into())
            .spawn(move || engine_loop(members, maker, shared))
            .expect("multistream engine thread spawns");
        MultiSession {
            handle,
            engine: Some(engine),
        }
    }

    pub fn handle(&self) -> MultiHandle {
        self.handle.clone()
    }

    /// Stop every lane (each flushes its RTMP goodbye) and join the engine.
    pub fn stop(mut self) -> Vec<MemberStatus> {
        self.shut_down();
        self.handle.statuses()
    }

    fn shut_down(&mut self) {
        self.handle.stop();
        if let Some(engine) = self.engine.take() {
            let _ = engine.join();
        }
        // Take the sessions out under the lock, join them outside it — in
        // parallel, so N goodbyes cost one goodbye's wait.
        let sessions: Vec<StreamSession> = lock(&self.handle.shared.lanes)
            .iter_mut()
            .filter_map(|lane| lane.session.take())
            .collect();
        std::thread::scope(|scope| {
            for session in sessions {
                scope.spawn(move || {
                    let _ = session.stop();
                });
            }
        });
    }
}

impl Drop for MultiSession {
    fn drop(&mut self) {
        self.shut_down();
    }
}

fn spawn_lane(members: &[MemberSpec], maker: &LaneMaker, ids: &[usize]) -> Lane {
    let io = maker(ids);
    let first = members
        .iter()
        .find(|member| member.id == ids[0])
        .expect("lane ids come from the member list");
    let session = StreamSession::start(
        StreamSpec {
            width: first.width,
            height: first.height,
            fps: first.fps,
        },
        io.factory,
    );
    let handle = session.handle();
    Lane {
        session: Some(session),
        handle,
        cells: io.cells,
        track: first.track,
        canvas: first.canvas,
        last_bytes: 0,
        last_sample: Instant::now(),
        kbps: 0,
    }
}

/// The engine pass, ~4×/s: drain slave-failure reports into split-outs and
/// refresh each lane's measured bitrate.
fn engine_loop(members: Arc<Vec<MemberSpec>>, maker: LaneMaker, shared: Arc<MultiShared>) {
    const POLL: Duration = Duration::from_millis(250);
    while !shared.stopping.load(Ordering::Relaxed) {
        let mut splits: Vec<usize> = Vec::new();
        {
            let mut lanes = lock(&shared.lanes);
            for lane in lanes.iter_mut() {
                // 1. Split-outs: a failed tee slave leaves its shared lane
                //    (which keeps publishing the rest) for its own session.
                let failed: Vec<usize> = lock(&lane.cells.slave_failures).drain(..).collect();
                for slave in failed {
                    let member = lock(&lane.cells.spawn_order).get(slave).copied();
                    let Some(member) = member else { continue };
                    let mut ids = lock(&lane.cells.members);
                    if ids.len() > 1 && ids.contains(&member) {
                        ids.retain(|id| *id != member);
                        splits.push(member);
                    }
                    // A failed sole member stays: its lane's own process-level
                    // reconnect ladder owns it.
                }
                // 2. Measured bitrate from the muxer's bytes-out delta.
                let elapsed = lane.last_sample.elapsed();
                if elapsed >= Duration::from_secs(1) {
                    let bytes = lane.cells.bytes_out.load(Ordering::Relaxed);
                    if bytes >= lane.last_bytes && bytes > 0 {
                        let delta = bytes - lane.last_bytes;
                        lane.kbps = ((delta * 8) as f64 / elapsed.as_secs_f64() / 1000.0) as u32;
                    } else {
                        lane.kbps = 0; // the process respawned — resync
                    }
                    lane.last_bytes = bytes;
                    lane.last_sample = Instant::now();
                }
            }
            for member in &splits {
                let lane = spawn_lane(&members, &maker, &[*member]);
                lanes.push(lane);
            }
        }
        std::thread::sleep(POLL);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fcap_encode::RecordSink;
    use std::path::PathBuf;
    use std::sync::atomic::AtomicU32;

    fn member(id: usize, signature: &str, tee_safe: bool) -> MemberSpec {
        MemberSpec {
            id,
            label: format!("svc{id}"),
            track: 1,
            canvas: 0,
            width: 64,
            height: 36,
            fps: 30,
            signature: signature.to_string(),
            tee_safe,
            nominal_kbps: 6_160,
        }
    }

    #[test]
    fn grouping_shares_equal_signatures_and_isolates_tee_unsafe() {
        let members = vec![
            member(0, "a", true),
            member(1, "b", true),
            member(2, "a", true),
            member(3, "a", false),
        ];
        assert_eq!(
            group_members(&members),
            vec![vec![0, 2], vec![1], vec![3]],
            "equal signatures share one lane; tee-unsafe rides alone"
        );
    }

    /// A sink that accepts everything until its kill switch flips.
    struct FakeSink {
        fail: Arc<AtomicBool>,
    }

    impl RecordSink for FakeSink {
        fn write_video(&mut self, _pixels: &Arc<Vec<u8>>) -> Result<(), String> {
            if self.fail.load(Ordering::Relaxed) {
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

    fn frame() -> Arc<Vec<u8>> {
        Arc::new(vec![0u8; 64 * 36 * 4])
    }

    /// A maker whose sinks never fail; counts lane builds.
    fn healthy_maker(builds: Arc<AtomicU32>) -> LaneMaker {
        Box::new(move |ids| {
            builds.fetch_add(1, Ordering::Relaxed);
            let cells = LaneCells::new(ids);
            let members = Arc::clone(&cells.members);
            let spawn_order = Arc::clone(&cells.spawn_order);
            LaneIo {
                factory: Box::new(move || {
                    *lock(&spawn_order) = lock(&members).clone();
                    Ok(Box::new(FakeSink {
                        fail: Arc::new(AtomicBool::new(false)),
                    }) as Box<dyn RecordSink>)
                }),
                cells,
            }
        })
    }

    fn wait_until(deadline_ms: u64, mut check: impl FnMut() -> bool) -> bool {
        let deadline = Instant::now() + Duration::from_millis(deadline_ms);
        while Instant::now() < deadline {
            if check() {
                return true;
            }
            std::thread::sleep(Duration::from_millis(25));
        }
        false
    }

    #[test]
    fn different_signatures_go_live_on_their_own_lanes() {
        let builds = Arc::new(AtomicU32::new(0));
        let session = MultiSession::start(
            vec![member(0, "a", true), member(1, "b", true)],
            healthy_maker(Arc::clone(&builds)),
        );
        let handle = session.handle();
        assert!(
            wait_until(3_000, || {
                handle.push_frame(0, frame());
                let statuses = handle.statuses();
                statuses.len() == 2 && statuses.iter().all(|s| s.state == StreamState::Live)
            }),
            "both targets live: {:?}",
            handle.statuses()
        );
        assert_eq!(builds.load(Ordering::Relaxed), 2, "one lane per signature");
        assert!(handle
            .statuses()
            .iter()
            .all(|status| status.shared_with == 0));
        let final_statuses = session.stop();
        assert!(final_statuses
            .iter()
            .all(|s| s.state == (StreamState::Ended { error: None })));
    }

    #[test]
    fn one_dead_target_reconnects_without_touching_the_other() {
        // Two lanes; lane B's sink dies. Only member 1 leaves Live.
        let fail_b = Arc::new(AtomicBool::new(false));
        let fail_b_for_maker = Arc::clone(&fail_b);
        let maker: LaneMaker = Box::new(move |ids| {
            let cells = LaneCells::new(ids);
            let members = Arc::clone(&cells.members);
            let spawn_order = Arc::clone(&cells.spawn_order);
            let fail = if ids == [1] {
                Arc::clone(&fail_b_for_maker)
            } else {
                Arc::new(AtomicBool::new(false))
            };
            LaneIo {
                factory: Box::new(move || {
                    *lock(&spawn_order) = lock(&members).clone();
                    Ok(Box::new(FakeSink {
                        fail: Arc::clone(&fail),
                    }) as Box<dyn RecordSink>)
                }),
                cells,
            }
        });
        let session = MultiSession::start(vec![member(0, "a", true), member(1, "b", true)], maker);
        let handle = session.handle();
        assert!(wait_until(3_000, || {
            handle.push_frame(0, frame());
            handle
                .statuses()
                .iter()
                .all(|s| s.state == StreamState::Live)
        }));
        fail_b.store(true, Ordering::Relaxed);
        assert!(
            wait_until(5_000, || {
                handle.push_frame(0, frame());
                let statuses = handle.statuses();
                statuses[0].state == StreamState::Live
                    && matches!(statuses[1].state, StreamState::Reconnecting { .. })
            }),
            "member 1 reconnects alone: {:?}",
            handle.statuses()
        );
        drop(session);
    }

    #[test]
    fn a_failed_tee_slave_splits_out_and_the_lane_keeps_publishing() {
        let builds = Arc::new(AtomicU32::new(0));
        let session = MultiSession::start(
            vec![member(0, "a", true), member(1, "a", true)],
            healthy_maker(Arc::clone(&builds)),
        );
        let handle = session.handle();
        assert!(wait_until(3_000, || {
            handle.push_frame(0, frame());
            let statuses = handle.statuses();
            statuses.len() == 2 && statuses.iter().all(|s| s.state == StreamState::Live)
        }));
        assert_eq!(builds.load(Ordering::Relaxed), 1, "one shared lane");
        assert!(handle.statuses().iter().all(|s| s.shared_with == 1));

        // ffmpeg reports slave #1 (member 1) failed.
        {
            let lanes = lock(&handle.shared.lanes);
            lock(&lanes[0].cells.slave_failures).push(1);
        }
        assert!(
            wait_until(3_000, || {
                handle.push_frame(0, frame());
                let statuses = handle.statuses();
                statuses.len() == 2
                    && statuses.iter().all(|s| s.shared_with == 0)
                    && builds.load(Ordering::Relaxed) == 2
            }),
            "member 1 split out to its own lane: {:?}",
            handle.statuses()
        );
        // Both end up live again — the split-out lane respawns healthy.
        assert!(wait_until(3_000, || {
            handle.push_frame(0, frame());
            handle
                .statuses()
                .iter()
                .all(|s| s.state == StreamState::Live)
        }));
        drop(session);
    }
}
