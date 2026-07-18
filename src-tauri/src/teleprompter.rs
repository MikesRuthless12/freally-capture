//! Teleprompter (CAP-N58): a smooth-scrolling script shared across surfaces.
//!
//! One session-scoped state holds the script, scroll speed, font size, and
//! mirror flag, plus a **lazily computed** scroll offset — `base_offset` at the
//! last control change, advanced by `speed × elapsed` while playing (an
//! `Instant`, so no timer thread and pause reads the exact position). Every
//! surface — the dock, a fullscreen projector, and the LAN panel — reads the
//! same state and animates locally between control changes, resyncing whenever
//! a `teleprompter` event fires. Scroll is controllable from the dock, hotkeys,
//! MIDI/OSC, and the LAN panel (the shared command allowlist).
//!
//! Offset is measured in **lines** (line-heights); each surface maps it to
//! pixels via its own font size, so a bigger projector scrolls the same text at
//! the same reading pace. The script is session-scoped (not persisted).

use std::sync::Mutex;
use std::time::Instant;

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, Runtime};

/// A script this big is almost certainly a paste error — cap the allocation.
const MAX_SCRIPT: usize = 200_000;
const MIN_SPEED: f32 = 0.2;
const MAX_SPEED: f32 = 40.0;
const MIN_FONT: f32 = 12.0;
const MAX_FONT: f32 = 240.0;
/// Multiplier applied by the faster/slower control steps.
const SPEED_STEP: f32 = 1.25;

struct Inner {
    script: String,
    /// Scroll speed in lines per second.
    speed: f32,
    /// Reading font size in px (the reference the dock/projector scale from).
    font_size: f32,
    /// Mirror horizontally (for a beam-splitter teleprompter glass).
    mirror: bool,
    /// Scroll offset (lines) at the last control change.
    base_offset: f32,
    /// `Some(play-start instant)` while playing; `None` when paused.
    play_started: Option<Instant>,
}

impl Inner {
    /// The current scroll offset (lines): the base plus what has scrolled since
    /// play began, or just the base while paused.
    fn offset(&self) -> f32 {
        match self.play_started {
            Some(started) => self.base_offset + self.speed * started.elapsed().as_secs_f32(),
            None => self.base_offset,
        }
    }

    /// Freeze the current offset into `base_offset` and restart the play clock,
    /// so a change to `speed` (or a resume) is continuous.
    fn rebase(&mut self) {
        self.base_offset = self.offset().max(0.0);
        if self.play_started.is_some() {
            self.play_started = Some(Instant::now());
        }
    }

    /// Stop scrolling, freezing the exact current position.
    fn pause(&mut self) {
        if self.play_started.is_some() {
            self.base_offset = self.offset().max(0.0);
            self.play_started = None;
        }
    }

    /// Start scrolling from the current position (continuous from a pause).
    fn resume(&mut self) {
        if self.play_started.is_none() {
            self.rebase();
            self.play_started = Some(Instant::now());
        }
    }

    /// Jump back to the top, keeping the play/pause state.
    fn rewind(&mut self) {
        self.base_offset = 0.0;
        if self.play_started.is_some() {
            self.play_started = Some(Instant::now());
        }
    }
}

/// Managed state: the shared teleprompter every surface renders.
pub struct TeleprompterState {
    inner: Mutex<Inner>,
}

/// The teleprompter snapshot every surface renders (emitted on every change).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeleprompterDto {
    pub script: String,
    pub speed: f32,
    pub font_size: f32,
    pub mirror: bool,
    /// Current scroll offset in lines (each surface maps it to pixels).
    pub offset: f32,
    pub playing: bool,
}

impl TeleprompterState {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(Inner {
                script: String::new(),
                speed: 2.0,
                font_size: 48.0,
                mirror: false,
                base_offset: 0.0,
                play_started: None,
            }),
        }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Inner> {
        self.inner
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// The current snapshot for a surface to render.
    pub fn dto(&self) -> TeleprompterDto {
        let inner = self.lock();
        TeleprompterDto {
            script: inner.script.clone(),
            speed: inner.speed,
            font_size: inner.font_size,
            mirror: inner.mirror,
            offset: inner.offset().max(0.0),
            playing: inner.play_started.is_some(),
        }
    }

    /// Replace the script (truncated to the cap) and rewind to the top.
    pub fn set_script(&self, text: String) {
        let mut inner = self.lock();
        inner.script = if text.len() > MAX_SCRIPT {
            text.chars().take(MAX_SCRIPT).collect()
        } else {
            text
        };
        inner.rewind();
    }

    pub fn set_speed(&self, speed: f32) {
        let mut inner = self.lock();
        inner.rebase();
        inner.speed = speed.clamp(MIN_SPEED, MAX_SPEED);
    }

    pub fn set_font(&self, font_size: f32) {
        self.lock().font_size = font_size.clamp(MIN_FONT, MAX_FONT);
    }

    pub fn set_mirror(&self, mirror: bool) {
        self.lock().mirror = mirror;
    }

    /// Apply a control action (play / pause / toggle / faster / slower / top).
    /// `value` is the target speed for `setSpeed`.
    pub fn apply(&self, action: &str, value: Option<f32>) -> Result<(), String> {
        let mut inner = self.lock();
        match action {
            "play" | "start" => inner.resume(),
            "pause" | "stop" => inner.pause(),
            "toggle" => {
                if inner.play_started.is_some() {
                    inner.pause();
                } else {
                    inner.resume();
                }
            }
            "faster" => {
                inner.rebase();
                inner.speed = (inner.speed * SPEED_STEP).clamp(MIN_SPEED, MAX_SPEED);
            }
            "slower" => {
                inner.rebase();
                inner.speed = (inner.speed / SPEED_STEP).clamp(MIN_SPEED, MAX_SPEED);
            }
            "setSpeed" => {
                let v = value.ok_or("setSpeed needs a value")?;
                inner.rebase();
                inner.speed = v.clamp(MIN_SPEED, MAX_SPEED);
            }
            "top" | "reset" | "rewind" => inner.rewind(),
            other => return Err(format!("unknown teleprompter action: {other}")),
        }
        Ok(())
    }
}

impl Default for TeleprompterState {
    fn default() -> Self {
        Self::new()
    }
}

/// Emit the current snapshot so every surface resyncs.
fn broadcast<R: Runtime>(app: &AppHandle<R>) {
    let dto = app.state::<TeleprompterState>().dto();
    if let Err(err) = app.emit("teleprompter", dto) {
        eprintln!("teleprompter: emit failed: {err}");
    }
}

/// The control entry point shared by hotkeys, MIDI/OSC, and the LAN panel
/// (through the command allowlist). Applies the action and broadcasts.
pub fn control<R: Runtime>(
    app: &AppHandle<R>,
    action: &str,
    value: Option<f32>,
) -> Result<(), String> {
    app.state::<TeleprompterState>().apply(action, value)?;
    broadcast(app);
    Ok(())
}

// -- commands -----------------------------------------------------------------

/// The current teleprompter snapshot (a surface's initial read).
#[tauri::command]
pub fn teleprompter_get(state: tauri::State<'_, TeleprompterState>) -> TeleprompterDto {
    state.dto()
}

#[tauri::command]
pub fn teleprompter_set_script(
    app: AppHandle,
    state: tauri::State<'_, TeleprompterState>,
    text: String,
) {
    state.set_script(text);
    broadcast(&app);
}

#[tauri::command]
pub fn teleprompter_set_speed(
    app: AppHandle,
    state: tauri::State<'_, TeleprompterState>,
    speed: f32,
) {
    state.set_speed(speed);
    broadcast(&app);
}

#[tauri::command]
pub fn teleprompter_set_font(
    app: AppHandle,
    state: tauri::State<'_, TeleprompterState>,
    size: f32,
) {
    state.set_font(size);
    broadcast(&app);
}

#[tauri::command]
pub fn teleprompter_set_mirror(
    app: AppHandle,
    state: tauri::State<'_, TeleprompterState>,
    mirror: bool,
) {
    state.set_mirror(mirror);
    broadcast(&app);
}

/// Play / pause / toggle / faster / slower / top — also reachable from the
/// command allowlist (hotkeys, MIDI, OSC, LAN panel).
#[tauri::command]
pub fn teleprompter_control(
    app: AppHandle,
    action: String,
    value: Option<f32>,
) -> Result<(), String> {
    control(&app, &action, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_advances_and_pause_freezes() {
        let s = TeleprompterState::new();
        s.set_speed(10.0);
        assert_eq!(s.dto().offset, 0.0);
        s.apply("play", None).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(50));
        let mid = s.dto().offset;
        assert!(mid > 0.0, "playing advances the offset");
        s.apply("pause", None).unwrap();
        let paused = s.dto().offset;
        std::thread::sleep(std::time::Duration::from_millis(30));
        assert_eq!(s.dto().offset, paused, "paused offset is frozen");
        assert!(paused >= mid);
    }

    #[test]
    fn top_rewinds_and_toggle_flips() {
        let s = TeleprompterState::new();
        s.apply("play", None).unwrap();
        assert!(s.dto().playing);
        s.apply("toggle", None).unwrap();
        assert!(!s.dto().playing, "toggle pauses");
        s.apply("top", None).unwrap();
        assert_eq!(s.dto().offset, 0.0);
    }

    #[test]
    fn speed_steps_stay_bounded_and_continuous() {
        let s = TeleprompterState::new();
        for _ in 0..40 {
            s.apply("faster", None).unwrap();
        }
        assert!(s.dto().speed <= MAX_SPEED);
        for _ in 0..80 {
            s.apply("slower", None).unwrap();
        }
        assert!(s.dto().speed >= MIN_SPEED);
    }

    #[test]
    fn set_script_caps_length_and_rewinds() {
        let s = TeleprompterState::new();
        // Scroll a little, then pause at a nonzero offset.
        s.apply("play", None).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(30));
        s.apply("pause", None).unwrap();
        assert!(s.dto().offset > 0.0, "scrolled to a nonzero offset");
        // A new script rewinds to the top — checked while paused, so the
        // assertion is exact and free of any scheduling-timing dependency.
        s.set_script("x".repeat(MAX_SCRIPT + 1000));
        assert_eq!(s.dto().script.len(), MAX_SCRIPT);
        assert_eq!(s.dto().offset, 0.0, "a new script rewinds to the top");
    }

    #[test]
    fn unknown_action_is_refused() {
        let s = TeleprompterState::new();
        assert!(s.apply("explode", None).is_err());
    }
}
