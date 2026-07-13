//! Core → UI push events.
//!
//! The `stats` emitter reports the real render fps + dropped-frame count the
//! studio loop measures, this process's CPU% and memory (via `sysinfo`), and
//! the compositor adapter. Payload shapes are mirrored in
//! `ui/src/api/types.ts` — keep in lockstep.

use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::thread;
use std::time::Duration;

use serde::Serialize;
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System};
use tauri::{AppHandle, Emitter, Manager};

/// Live runtime numbers the studio render loop publishes for the stats dock.
/// Read every ~500 ms by the emitter; a lock-free hand-off (no contention on
/// the 60 fps loop).
#[derive(Default)]
pub struct RuntimeStats {
    fps: AtomicU32,
    /// The second (vertical) canvas's compose rate — 0 when none runs.
    vertical_fps: AtomicU32,
    dropped: AtomicU64,
    render_micros: AtomicU64,
    /// Sources in the "error" state right now (CAP-M09 pre-flight hold).
    errored_sources: AtomicU32,
    /// Set once the compose loop is actually running.
    running: std::sync::atomic::AtomicBool,
}

impl RuntimeStats {
    /// Publish the newest render numbers (called ~1 Hz from the studio loop).
    pub fn publish(&self, fps: u32, vertical_fps: u32, dropped: u64, render_micros: u64) {
        self.fps.store(fps, Ordering::Relaxed);
        self.vertical_fps.store(vertical_fps, Ordering::Relaxed);
        self.dropped.store(dropped, Ordering::Relaxed);
        self.render_micros.store(render_micros, Ordering::Relaxed);
        self.running.store(true, Ordering::Relaxed);
    }

    /// Sources currently in the "error" state, from the studio loop's last
    /// program-event build — the backend half of the CAP-M09 pre-flight
    /// hold (the hotkey and the remote API never see the dialog).
    pub fn set_errored_sources(&self, count: u32) {
        self.errored_sources.store(count, Ordering::Relaxed);
    }

    pub fn errored_sources(&self) -> u32 {
        self.errored_sources.load(Ordering::Relaxed)
    }

    /// The latest render numbers — (running, fps, vertical fps, dropped,
    /// render µs). The diagnostics bundle's "recent stats" (CAP-M24).
    pub fn latest(&self) -> (bool, u32, u32, u64, u64) {
        (
            self.running.load(Ordering::Relaxed),
            self.fps.load(Ordering::Relaxed),
            self.vertical_fps.load(Ordering::Relaxed),
            self.dropped.load(Ordering::Relaxed),
            self.render_micros.load(Ordering::Relaxed),
        )
    }
}

/// The `stats` event payload.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsPayload {
    /// Composed frames per second (the program render rate).
    pub fps: f32,
    /// The second (vertical) canvas's compose rate (0 = none running).
    pub vertical_fps: f32,
    /// This process's CPU usage, percent of one core-second.
    pub cpu: f32,
    /// This process's resident memory, MiB.
    pub memory_mb: f32,
    /// Frames the capture pipeline dropped since the session began.
    pub dropped: u64,
    /// Mean GPU compose time per frame, milliseconds.
    pub render_ms: f32,
    /// Now false — the numbers are real (kept so the UI can still flag a
    /// pre-compose-loop startup window honestly).
    pub placeholder: bool,
}

/// Spawn the ~2 Hz stats emitter thread. It samples this process's CPU/memory
/// and folds in the studio loop's fps/dropped, then pushes `stats`. Ends
/// itself once the app shuts down and emitting fails.
pub fn spawn_stats_emitter(app: AppHandle) {
    thread::spawn(move || {
        let pid = Pid::from_u32(std::process::id());
        let mut system = System::new();
        // CPU% needs two samples spaced by the refresh interval to be real.
        system.refresh_processes_specifics(
            ProcessesToUpdate::Some(&[pid]),
            true,
            ProcessRefreshKind::nothing().with_cpu().with_memory(),
        );
        let cpu_count = system.physical_core_count().unwrap_or(1).max(1) as f32;

        loop {
            thread::sleep(Duration::from_millis(500));
            system.refresh_processes_specifics(
                ProcessesToUpdate::Some(&[pid]),
                true,
                ProcessRefreshKind::nothing().with_cpu().with_memory(),
            );
            let (cpu, memory_mb) = match system.process(pid) {
                // sysinfo reports CPU% summed across cores (0..100·N); scale
                // to a single-machine 0..100 so the dock reads like Task Mgr.
                Some(process) => (
                    process.cpu_usage() / cpu_count,
                    process.memory() as f32 / (1024.0 * 1024.0),
                ),
                None => (0.0, 0.0),
            };

            let stats = app.state::<RuntimeStats>();
            let running = stats.running.load(Ordering::Relaxed);
            let payload = StatsPayload {
                fps: stats.fps.load(Ordering::Relaxed) as f32,
                vertical_fps: stats.vertical_fps.load(Ordering::Relaxed) as f32,
                cpu,
                memory_mb,
                dropped: stats.dropped.load(Ordering::Relaxed),
                render_ms: stats.render_micros.load(Ordering::Relaxed) as f32 / 1000.0,
                placeholder: !running,
            };
            if app.emit("stats", &payload).is_err() {
                break;
            }
        }
    });
}
