//! Core → UI push events.
//!
//! P0.3 proves the event pipe with a placeholder `stats` emitter; real
//! fps/CPU/GPU/encoder sampling lands with the stats dock work in Phase 5.
//! Payload shapes are mirrored in `ui/src/api/types.ts` — keep in lockstep.

use std::thread;
use std::time::Duration;

use serde::Serialize;
use tauri::{AppHandle, Emitter};

/// The `stats` event payload.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsPayload {
    pub fps: f32,
    pub cpu: f32,
    /// True until real sampling lands (P5.5) — the UI labels the data honestly.
    pub placeholder: bool,
}

/// Spawn the ~2 Hz stats emitter thread. The thread ends itself once the app
/// shuts down and emitting fails.
pub fn spawn_stats_emitter(app: AppHandle) {
    thread::spawn(move || {
        let mut tick: u32 = 0;
        loop {
            // A gentle oscillation so the dock visibly ticks (placeholder!).
            let phase = tick as f32 * 0.35;
            let payload = StatsPayload {
                fps: 60.0 + phase.sin(),
                cpu: 4.0 + (phase * 0.7).cos().abs() * 3.0,
                placeholder: true,
            };
            if app.emit("stats", &payload).is_err() {
                break;
            }
            tick = tick.wrapping_add(1);
            thread::sleep(Duration::from_millis(500));
        }
    });
}
