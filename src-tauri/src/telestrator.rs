//! Telestrator (CAP-N57): live draw-on-program annotation.
//!
//! The host draws over the live program from the UI overlay. Because the native
//! preview paints over the DOM, the in-progress stroke is streamed live into
//! this state (begin → extend → commit), so the studio render loop bakes it —
//! and every committed stroke — over the finished program each tick (see
//! [`TelestratorState::snapshot`] → `Compositor::render_telestrator`). The marks
//! therefore reach preview, recording, and stream alike, and grow as they are
//! drawn. Strokes then fade after a set time or persist (whiteboard mode). All
//! state is ephemeral — nothing is persisted to disk.

use std::sync::Mutex;
use std::time::Instant;

use fcap_compositor::{stroke_expired, TelePoint, TeleStroke, TeleTool};
use serde::Deserialize;

/// Most points a single stroke may carry (a defensive allocation bound on the
/// UI-supplied path — a normal free-hand stroke is a few hundred at most).
const MAX_POINTS: usize = 8192;
/// Most committed strokes retained at once; beyond this the oldest are dropped
/// so memory stays bounded even in persistent whiteboard mode.
const MAX_STROKES: usize = 2048;

/// One sampled pointer position (canvas-normalized `0..=1`, plus pen pressure).
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct TelePointInput {
    pub x: f32,
    pub y: f32,
    #[serde(default = "full_pressure")]
    pub pressure: f32,
}

fn full_pressure() -> f32 {
    1.0
}

impl TelePointInput {
    fn clamp(self) -> TelePoint {
        TelePoint {
            x: self.x.clamp(0.0, 1.0),
            y: self.y.clamp(0.0, 1.0),
            pressure: self.pressure.clamp(0.0, 1.0),
        }
    }
}

/// The opening of a stroke: the tool + style + its first point.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeleStrokeBegin {
    /// "pen" | "highlight" | "arrow" | "ellipse".
    pub tool: String,
    /// RGBA, each channel 0..=1.
    pub color: [f32; 4],
    /// Line width as a fraction of the canvas height.
    pub width: f32,
    /// Seconds until the stroke starts fading; `None`/absent = persistent.
    #[serde(default)]
    pub fade_after: Option<f32>,
    pub point: TelePointInput,
}

fn tool_from_str(tool: &str) -> Result<TeleTool, String> {
    match tool {
        "pen" => Ok(TeleTool::Pen),
        "highlight" => Ok(TeleTool::Highlight),
        "arrow" => Ok(TeleTool::Arrow),
        "ellipse" => Ok(TeleTool::Ellipse),
        other => Err(format!("unknown telestrator tool: {other}")),
    }
}

struct Tele {
    /// Finished strokes, oldest first.
    committed: Vec<TeleStroke>,
    /// The stroke currently being drawn (streamed live), if any.
    active: Option<TeleStroke>,
}

/// Managed state: the live stroke set the render loop bakes each tick, plus the
/// telestrator clock the strokes' fade is measured against.
pub struct TelestratorState {
    origin: Instant,
    inner: Mutex<Tele>,
}

impl TelestratorState {
    pub fn new() -> Self {
        Self {
            origin: Instant::now(),
            inner: Mutex::new(Tele {
                committed: Vec::new(),
                active: None,
            }),
        }
    }

    /// The telestrator clock, in seconds since the app started.
    pub fn now(&self) -> f64 {
        self.origin.elapsed().as_secs_f64()
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Tele> {
        self.inner
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// Start a new stroke (replacing any un-committed one). Validated + clamped.
    pub fn begin(&self, begin: TeleStrokeBegin) -> Result<(), String> {
        let tool = tool_from_str(&begin.tool)?;
        let color = [
            begin.color[0].clamp(0.0, 1.0),
            begin.color[1].clamp(0.0, 1.0),
            begin.color[2].clamp(0.0, 1.0),
            begin.color[3].clamp(0.0, 1.0),
        ];
        let now = self.now();
        let stroke = TeleStroke {
            tool,
            color,
            width: begin.width.clamp(0.0005, 0.2),
            points: vec![begin.point.clamp()],
            // While drawing, the stroke never fades; the window is (re)started
            // from the commit time so a mark fades N seconds after it's finished.
            fade_after: begin.fade_after.map(|s| s.clamp(0.1, 3600.0)),
            born_seconds: now,
        };
        self.lock().active = Some(stroke);
        Ok(())
    }

    /// Append sampled points to the in-progress stroke (a no-op if none).
    pub fn extend(&self, points: Vec<TelePointInput>) {
        let mut guard = self.lock();
        if let Some(active) = guard.active.as_mut() {
            for p in points {
                if active.points.len() >= MAX_POINTS {
                    break;
                }
                active.points.push(p.clamp());
            }
        }
    }

    /// Finish the in-progress stroke, moving it into the committed set. Its fade
    /// window restarts from now, so a timed mark fades after it's drawn.
    pub fn commit(&self) {
        let now = self.now();
        let mut guard = self.lock();
        if let Some(mut stroke) = guard.active.take() {
            if stroke.points.is_empty() {
                return;
            }
            stroke.born_seconds = now;
            guard.committed.push(stroke);
            let overflow = guard.committed.len().saturating_sub(MAX_STROKES);
            if overflow > 0 {
                guard.committed.drain(0..overflow);
            }
        }
    }

    /// Discard the in-progress stroke (pointer cancel / escape).
    pub fn cancel(&self) {
        self.lock().active = None;
    }

    /// Remove every stroke, in-progress and committed (the hotkey-bound clear).
    pub fn clear(&self) {
        let mut guard = self.lock();
        guard.committed.clear();
        guard.active = None;
    }

    /// Remove the most recent committed stroke (undo).
    pub fn undo(&self) {
        self.lock().committed.pop();
    }

    /// The live strokes for this tick: committed (faded ones GC'd) with the
    /// in-progress stroke appended so it renders as it's drawn. `now` is
    /// [`TelestratorState::now`].
    pub fn snapshot(&self, now: f64) -> Vec<TeleStroke> {
        let mut guard = self.lock();
        guard.committed.retain(|s| !stroke_expired(s, now));
        let mut out = guard.committed.clone();
        if let Some(active) = guard.active.as_ref() {
            out.push(active.clone());
        }
        out
    }
}

impl Default for TelestratorState {
    fn default() -> Self {
        Self::new()
    }
}

// -- commands -----------------------------------------------------------------

/// Start drawing a stroke (pointer down on the telestrator overlay).
#[tauri::command]
pub fn telestrator_begin_stroke(
    state: tauri::State<'_, TelestratorState>,
    stroke: TeleStrokeBegin,
) -> Result<(), String> {
    state.begin(stroke)
}

/// Stream more sampled points into the in-progress stroke (pointer move).
#[tauri::command]
pub fn telestrator_extend_stroke(
    state: tauri::State<'_, TelestratorState>,
    points: Vec<TelePointInput>,
) {
    state.extend(points);
}

/// Finish the in-progress stroke (pointer up).
#[tauri::command]
pub fn telestrator_commit_stroke(state: tauri::State<'_, TelestratorState>) {
    state.commit();
}

/// Discard the in-progress stroke (pointer cancel).
#[tauri::command]
pub fn telestrator_cancel_stroke(state: tauri::State<'_, TelestratorState>) {
    state.cancel();
}

/// Clear all telestrator marks.
#[tauri::command]
pub fn telestrator_clear(state: tauri::State<'_, TelestratorState>) {
    state.clear();
}

/// Undo the last committed telestrator stroke.
#[tauri::command]
pub fn telestrator_undo(state: tauri::State<'_, TelestratorState>) {
    state.undo();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn begin(tool: &str, fade: Option<f32>) -> TeleStrokeBegin {
        TeleStrokeBegin {
            tool: tool.to_string(),
            color: [1.0, 0.0, 0.0, 1.0],
            width: 0.01,
            fade_after: fade,
            point: TelePointInput {
                x: 0.1,
                y: 0.1,
                pressure: 1.0,
            },
        }
    }

    fn pts(n: usize) -> Vec<TelePointInput> {
        (0..n)
            .map(|i| TelePointInput {
                x: (i as f32) / 100.0,
                y: 0.5,
                pressure: 1.0,
            })
            .collect()
    }

    #[test]
    fn begin_extend_commit_flow() {
        let state = TelestratorState::new();
        state.begin(begin("pen", None)).expect("begin");
        // The in-progress stroke is already visible in the snapshot.
        assert_eq!(state.snapshot(state.now()).len(), 1);
        state.extend(pts(3));
        state.commit();
        // Committed; nothing in-progress now.
        let strokes = state.snapshot(state.now());
        assert_eq!(strokes.len(), 1);
        assert_eq!(strokes[0].points.len(), 4, "first point + three extended");
    }

    #[test]
    fn cancel_drops_the_active_stroke() {
        let state = TelestratorState::new();
        state.begin(begin("arrow", None)).expect("begin");
        state.extend(pts(2));
        state.cancel();
        assert!(state.snapshot(state.now()).is_empty());
    }

    #[test]
    fn undo_and_clear() {
        let state = TelestratorState::new();
        for tool in ["pen", "arrow"] {
            state.begin(begin(tool, None)).expect("begin");
            state.commit();
        }
        assert_eq!(state.snapshot(state.now()).len(), 2);
        state.undo();
        assert_eq!(state.snapshot(state.now()).len(), 1);
        state.clear();
        assert!(state.snapshot(state.now()).is_empty());
    }

    #[test]
    fn unknown_tool_is_refused() {
        let state = TelestratorState::new();
        assert!(state.begin(begin("scribble", None)).is_err());
    }

    #[test]
    fn faded_strokes_drop_from_the_snapshot() {
        let state = TelestratorState::new();
        state.begin(begin("pen", Some(0.1))).expect("begin");
        state.commit();
        // Live now…
        assert_eq!(state.snapshot(state.now()).len(), 1);
        // …gone far in the future.
        assert!(state.snapshot(state.now() + 10.0).is_empty());
    }

    #[test]
    fn the_stroke_cap_bounds_memory() {
        let state = TelestratorState::new();
        for _ in 0..MAX_STROKES + 50 {
            state.begin(begin("pen", None)).expect("begin");
            state.commit();
        }
        assert_eq!(state.snapshot(state.now()).len(), MAX_STROKES);
    }

    #[test]
    fn an_over_long_path_is_truncated() {
        let state = TelestratorState::new();
        state.begin(begin("pen", None)).expect("begin");
        state.extend(pts(MAX_POINTS + 500));
        state.commit();
        let strokes = state.snapshot(state.now());
        assert_eq!(strokes[0].points.len(), MAX_POINTS);
    }
}
