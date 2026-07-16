//! The show rundown (CAP-N09): an ordered playlist of steps — a scene, how
//! long it holds, and optional actions — with manual or automatic advance and
//! a visible "next up + remaining time".
//!
//! It reuses the automation vocabulary wholesale: a step's actions are the
//! same allowlisted studio commands a macro runs (`remote_api`'s fixed list),
//! so a rundown can no more read a file or spawn a process than a rule can.
//!
//! The rundown is **runtime state, not a mutation**: running it switches
//! scenes through the ordinary command path (undoable, event-driven), and the
//! rundown itself never edits the collection.

use std::sync::Mutex;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager, Runtime};

use crate::automation::MacroStep;

/// The most steps one rundown may hold.
pub const MAX_STEPS: usize = 128;
/// The longest a step may hold (2 hours) — a typo can't strand a show.
pub const MAX_HOLD_SECS: u32 = 7_200;

/// One rundown step (CAP-N09).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RundownStep {
    /// What the step is called on the operator's screen.
    pub name: String,
    /// The scene to cut to (by name — what the user edits). `""` = stay.
    #[serde(default)]
    pub scene: String,
    /// How long the step holds before auto-advance. `0` = manual only.
    #[serde(default)]
    pub hold_secs: u32,
    /// Studio actions fired when the step starts (same allowlist as macros).
    #[serde(default)]
    pub actions: Vec<MacroStep>,
}

/// The persisted rundown (CAP-N09).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct RundownSettings {
    pub steps: Vec<RundownStep>,
    /// Advance automatically when a step's hold expires (off by default —
    /// a show should not run away from its operator unasked).
    pub auto_advance: bool,
}

impl RundownSettings {
    pub fn validate(&self) -> Result<(), String> {
        if self.steps.len() > MAX_STEPS {
            return Err(format!("too many rundown steps ({MAX_STEPS} max)"));
        }
        for step in &self.steps {
            if step.name.len() > 64 || step.scene.len() > 64 {
                return Err("a rundown step's name/scene is too long".to_owned());
            }
            if step.hold_secs > MAX_HOLD_SECS {
                return Err(format!("a step may hold at most {MAX_HOLD_SECS} s"));
            }
            crate::automation::validate_action_steps(&step.actions)?;
        }
        Ok(())
    }
}

/// What the operator's dock shows (CAP-N09's "next up + remaining time").
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RundownStatus {
    /// The running step's index, if the rundown is running.
    pub at: Option<usize>,
    /// Seconds left on the running step (`None` = manual-only step).
    pub remaining_secs: Option<u32>,
    /// The step after the current one, by name.
    pub next_up: Option<String>,
    pub running: bool,
}

/// Managed state: where the rundown is, and when the current step started.
#[derive(Default)]
pub struct RundownState {
    inner: Mutex<Option<Running>>,
}

struct Running {
    at: usize,
    started: Instant,
    hold: Option<Duration>,
}

fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

impl RundownState {
    /// Start (or restart) the rundown at `index`.
    pub fn start<R: Runtime>(&self, app: &AppHandle<R>, index: usize) -> Result<(), String> {
        let settings = app.state::<crate::settings::SettingsStore>().get().rundown;
        let step = settings
            .steps
            .get(index)
            .ok_or_else(|| "no such rundown step".to_string())?
            .clone();
        *lock(&self.inner) = Some(Running {
            at: index,
            started: Instant::now(),
            hold: (step.hold_secs > 0).then(|| Duration::from_secs(u64::from(step.hold_secs))),
        });
        run_step(app, &step);
        // CAP-N43: "new file per rundown step" — cut the recording's frec
        // lanes on every step start when the trigger is armed. (The 1 s
        // minimum part length in the sink absorbs the double-fire when the
        // step ALSO switches scenes and split-on-scene is armed too.)
        {
            let recording = app.state::<crate::recording::RecordingState>();
            if recording.splits_on_rundown() {
                recording.request_split_all();
            }
        }
        emit(app, self, &settings);
        Ok(())
    }

    /// Advance to the next step (the operator's Next button, or auto-advance).
    /// Stops at the end — a rundown never loops behind the operator's back.
    pub fn advance<R: Runtime>(&self, app: &AppHandle<R>) -> Result<(), String> {
        let next = {
            let guard = lock(&self.inner);
            match guard.as_ref() {
                Some(running) => running.at + 1,
                None => return Err("the rundown is not running".to_owned()),
            }
        };
        let settings = app.state::<crate::settings::SettingsStore>().get().rundown;
        if next >= settings.steps.len() {
            self.stop(app);
            return Ok(());
        }
        self.start(app, next)
    }

    /// Stop the rundown (the scene stays where it is — no surprise cuts).
    pub fn stop<R: Runtime>(&self, app: &AppHandle<R>) {
        *lock(&self.inner) = None;
        let settings = app.state::<crate::settings::SettingsStore>().get().rundown;
        emit(app, self, &settings);
    }

    /// The dock's view of the rundown.
    pub fn status(&self, settings: &RundownSettings) -> RundownStatus {
        let guard = lock(&self.inner);
        let Some(running) = guard.as_ref() else {
            return RundownStatus::default();
        };
        let remaining = running.hold.map(|hold| {
            hold.saturating_sub(running.started.elapsed())
                .as_secs()
                .min(u64::from(MAX_HOLD_SECS)) as u32
        });
        RundownStatus {
            at: Some(running.at),
            remaining_secs: remaining,
            next_up: settings
                .steps
                .get(running.at + 1)
                .map(|step| step.name.clone()),
            running: true,
        }
    }

    /// Whether the running step's hold has expired (the loop's auto-advance
    /// check — cheap, and `false` whenever nothing is running).
    fn hold_expired(&self) -> bool {
        let guard = lock(&self.inner);
        guard.as_ref().is_some_and(|running| {
            running
                .hold
                .is_some_and(|hold| running.started.elapsed() >= hold)
        })
    }
}

/// Fire a step: cut to its scene (through the ordinary, undoable command) and
/// run its actions (the automation allowlist).
fn run_step<R: Runtime>(app: &AppHandle<R>, step: &RundownStep) {
    if !step.scene.trim().is_empty() {
        let scene = step.scene.clone();
        let handle = app.clone();
        std::thread::spawn(move || {
            if let Err(err) = crate::remote_api::dispatch_any(
                &handle,
                "setProgramScene",
                &serde_json::json!({ "scene": scene }),
            ) {
                eprintln!("rundown: could not switch to {scene}: {err}");
            }
        });
    }
    for action in &step.actions {
        if let MacroStep::Action { command, params } = action {
            let (command, params) = (command.clone(), params.clone());
            let handle = app.clone();
            std::thread::spawn(move || {
                if let Err(err) = crate::remote_api::dispatch_any(&handle, &command, &params) {
                    eprintln!("rundown: action {command} failed: {err}");
                }
            });
        }
    }
}

fn emit<R: Runtime>(app: &AppHandle<R>, state: &RundownState, settings: &RundownSettings) {
    let _ = app.emit("rundown", state.status(settings));
}

/// The loop's 1 Hz tick: auto-advance an expired step (only when the user
/// enabled it) and keep the dock's countdown live.
pub fn tick<R: Runtime>(app: &AppHandle<R>) {
    let state = app.state::<RundownState>();
    let settings = app.state::<crate::settings::SettingsStore>().get().rundown;
    if !state.status(&settings).running {
        return; // nothing running: no work, no events
    }
    if settings.auto_advance && state.hold_expired() {
        if let Err(err) = state.advance(app) {
            eprintln!("rundown: auto-advance failed: {err}");
        }
        return; // advance() already emitted
    }
    emit(app, &state, &settings);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn step(name: &str, hold: u32) -> RundownStep {
        RundownStep {
            name: name.to_owned(),
            scene: name.to_owned(),
            hold_secs: hold,
            actions: Vec::new(),
        }
    }

    #[test]
    fn validation_bounds_steps_holds_and_actions() {
        let ok = RundownSettings {
            steps: vec![step("Intro", 30), step("Main", 0)],
            auto_advance: true,
        };
        assert!(ok.validate().is_ok());

        let long_hold = RundownSettings {
            steps: vec![step("Forever", MAX_HOLD_SECS + 1)],
            auto_advance: false,
        };
        assert!(long_hold.validate().is_err());

        // A step's actions ride the same allowlist as a macro's.
        let evil = RundownSettings {
            steps: vec![RundownStep {
                name: "x".to_owned(),
                scene: String::new(),
                hold_secs: 0,
                actions: vec![MacroStep::Action {
                    command: "readFile".to_owned(),
                    params: serde_json::Value::Null,
                }],
            }],
            auto_advance: false,
        };
        assert!(
            evil.validate().is_err(),
            "off-allowlist actions are rejected"
        );
    }

    #[test]
    fn status_reports_next_up_and_countdown() {
        let settings = RundownSettings {
            steps: vec![step("Intro", 60), step("Main", 0), step("Outro", 0)],
            auto_advance: false,
        };
        let state = RundownState::default();
        // Nothing running: the dock shows an idle rundown.
        let idle = state.status(&settings);
        assert!(!idle.running && idle.at.is_none() && idle.next_up.is_none());

        // Simulate the running state the app's `start` would install.
        *lock(&state.inner) = Some(Running {
            at: 0,
            started: Instant::now(),
            hold: Some(Duration::from_secs(60)),
        });
        let live = state.status(&settings);
        assert!(live.running);
        assert_eq!(live.at, Some(0));
        assert_eq!(live.next_up.as_deref(), Some("Main"));
        assert!(live
            .remaining_secs
            .is_some_and(|left| (59..=60).contains(&left)));
        assert!(!state.hold_expired(), "a fresh step has not expired");

        // A manual-only step (hold 0) never expires and shows no countdown.
        *lock(&state.inner) = Some(Running {
            at: 1,
            started: Instant::now(),
            hold: None,
        });
        let manual = state.status(&settings);
        assert_eq!(manual.remaining_secs, None);
        assert!(!state.hold_expired());
        assert_eq!(manual.next_up.as_deref(), Some("Outro"));

        // An expired hold is visible to the auto-advance check.
        *lock(&state.inner) = Some(Running {
            at: 0,
            started: Instant::now() - Duration::from_secs(61),
            hold: Some(Duration::from_secs(60)),
        });
        assert!(state.hold_expired());
        assert_eq!(state.status(&settings).remaining_secs, Some(0));
    }
}
