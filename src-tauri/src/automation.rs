//! Automation: studio variables, macros, and the rules engine
//! (CAP-N01 + CAP-N02).
//!
//! **The security shape is the whole design.** An action is not arbitrary
//! code and never names a file: it is one entry of the **remote-API command
//! allowlist** (`remote_api::dispatch`) — the very same fixed vocabulary the
//! UI, hotkeys, and the WebSocket API drive. So a rules file cannot read or
//! write the disk, spawn a process, or reach the network by construction; it
//! can only ask the studio to do things the studio already does.
//!
//! Everything is **off by default**: a rule runs only while `enabled`, and
//! the engine's evaluator is a no-op when no rule is enabled.
//!
//! Evaluation is deterministic and edge-triggered: the loop hands the engine
//! a [`Signals`] snapshot each tick, and a trigger fires on the **transition**
//! into its condition (a threshold that stays crossed does not re-fire), so a
//! rule can never machine-gun itself.

use std::collections::HashMap;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The most steps one macro may hold (a hand-edited file can't grow it).
pub const MAX_STEPS: usize = 64;
/// The most macros/rules the settings may hold.
pub const MAX_MACROS: usize = 64;
pub const MAX_RULES: usize = 64;
/// The longest a macro step may wait (10 minutes) — a typo can't wedge a run.
pub const MAX_WAIT_MS: u64 = 600_000;
/// How many times a macro may repeat.
pub const MAX_REPEAT: u32 = 100;
/// Cap on simultaneously running macro instances (a rule storm can't fork
/// unbounded work).
pub const MAX_RUNNING: usize = 16;

/// One step of a macro (CAP-N02).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum MacroStep {
    /// Run one allowlisted studio command (the remote-API vocabulary).
    Action {
        command: String,
        #[serde(default)]
        params: Value,
    },
    /// Hold before the next step.
    Wait { ms: u64 },
    /// Set a studio variable (usable in text sources + rule conditions).
    SetVariable { name: String, value: String },
}

/// One named macro (CAP-N02).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Macro {
    pub name: String,
    #[serde(default)]
    pub steps: Vec<MacroStep>,
    /// Run the whole sequence this many times (1 = once).
    #[serde(default = "default_repeat")]
    pub repeat: u32,
    /// An OS-global accelerator that runs this macro (CAP-N02). `None` = no
    /// hotkey; the macro still runs from the UI, the rules engine, and the
    /// remote API. May be a **chord** (`"Ctrl+K, 3"`, CAP-N05).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hotkey: Option<String>,
    /// The hotkey layer this macro's accelerator belongs to (CAP-N05).
    /// `None` = every layer (like Record or the panic key).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layer: Option<u8>,
}

fn default_repeat() -> u32 {
    1
}

/// What makes a rule fire (CAP-N01). Every trigger is edge-evaluated.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum Trigger {
    /// The program scene became `scene` (by name — names are what a user
    /// edits, and they survive an id churn).
    SceneSwitched { scene: String },
    /// Streaming started / stopped.
    StreamState { live: bool },
    /// Recording started / stopped.
    RecordingState { recording: bool },
    /// A source entered the error state (any source, or one by name).
    SourceError {
        #[serde(default)]
        source: String,
    },
    /// A mixer strip's peak crossed `threshold_db` in the given direction.
    AudioLevel {
        source: String,
        threshold_db: f32,
        /// `true` = fires when the level rises above; `false` = falls below.
        #[serde(default)]
        above: bool,
    },
    /// The user has been idle for at least this long (Windows; elsewhere the
    /// signal is absent and the trigger simply never fires — said honestly).
    SystemIdle { seconds: u32 },
    /// The foreground window's process matches `exe` (Windows; same honesty).
    WindowFocus { exe: String },
    /// A local wall-clock time (`"HH:MM"`, local), once per day.
    TimeOfDay { at: String },
    /// A watched file's contents changed.
    FileChanged { path: String },
}

/// A condition gate — all of them must hold for the actions to run.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum Condition {
    /// A studio variable equals a value.
    VariableEquals { name: String, value: String },
    /// Only while streaming (or only while not).
    Streaming { live: bool },
    /// Only while recording (or only while not).
    Recording { recording: bool },
}

/// One automation rule (CAP-N01).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    pub name: String,
    /// Off by default — a rule does nothing until the user enables it.
    #[serde(default)]
    pub enabled: bool,
    pub trigger: Trigger,
    #[serde(default)]
    pub conditions: Vec<Condition>,
    /// Inline steps to run (the same vocabulary as a macro).
    #[serde(default)]
    pub actions: Vec<MacroStep>,
    /// …and/or a named macro to call.
    #[serde(default)]
    pub macro_name: String,
}

/// The persisted automation config (CAP-N01 + CAP-N02).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct AutomationSettings {
    pub macros: Vec<Macro>,
    pub rules: Vec<Rule>,
}

impl AutomationSettings {
    /// Bounded, well-formed, and every action inside the allowlist.
    pub fn validate(&self) -> Result<(), String> {
        if self.macros.len() > MAX_MACROS {
            return Err(format!("too many macros ({MAX_MACROS} max)"));
        }
        if self.rules.len() > MAX_RULES {
            return Err(format!("too many rules ({MAX_RULES} max)"));
        }
        for entry in &self.macros {
            if entry.name.trim().is_empty() || entry.name.len() > 64 {
                return Err("a macro needs a name (64 chars max)".to_owned());
            }
            if entry.repeat == 0 || entry.repeat > MAX_REPEAT {
                return Err(format!("macro repeat must be 1–{MAX_REPEAT}"));
            }
            validate_steps(&entry.steps)?;
        }
        for rule in &self.rules {
            if rule.name.trim().is_empty() || rule.name.len() > 64 {
                return Err("a rule needs a name (64 chars max)".to_owned());
            }
            validate_steps(&rule.actions)?;
            if !rule.macro_name.is_empty()
                && !self
                    .macros
                    .iter()
                    .any(|entry| entry.name == rule.macro_name)
            {
                return Err(format!("rule \"{}\" calls an unknown macro", rule.name));
            }
            if let Trigger::TimeOfDay { at } = &rule.trigger {
                parse_hhmm(at).ok_or_else(|| format!("bad time in rule \"{}\"", rule.name))?;
            }
        }
        Ok(())
    }
}

/// Validate a step list (macros, rules, and the rundown all share it):
/// every Action must be on the remote-API allowlist, and every bound holds.
pub fn validate_action_steps(steps: &[MacroStep]) -> Result<(), String> {
    validate_steps(steps)
}

fn validate_steps(steps: &[MacroStep]) -> Result<(), String> {
    if steps.len() > MAX_STEPS {
        return Err(format!("too many steps ({MAX_STEPS} max)"));
    }
    for step in steps {
        match step {
            MacroStep::Action { command, .. } => {
                if !crate::remote_api::is_allowed_command(command) {
                    return Err(format!("not an allowed studio command: {command}"));
                }
                // A macro step may NOT run another macro: `runMacro` is on the
                // allowlist for the remote API / hotkeys, but as a step it lets
                // a macro re-queue itself forever (there is no depth guard).
                // Macros are flat sequences by construction, so the cycle can't
                // exist. (Rules and the rundown reuse this validator.)
                if command == "runMacro" {
                    return Err("a macro cannot run another macro".to_owned());
                }
            }
            MacroStep::Wait { ms } => {
                if *ms > MAX_WAIT_MS {
                    return Err(format!("a wait may not exceed {MAX_WAIT_MS} ms"));
                }
            }
            MacroStep::SetVariable { name, value } => {
                if name.trim().is_empty() || name.len() > 64 || value.len() > 512 {
                    return Err("bad variable name/value".to_owned());
                }
            }
        }
    }
    Ok(())
}

/// `"HH:MM"` → minutes past midnight.
pub fn parse_hhmm(text: &str) -> Option<u32> {
    let (hh, mm) = text.split_once(':')?;
    let hours: u32 = hh.parse().ok()?;
    let minutes: u32 = mm.parse().ok()?;
    (hours < 24 && minutes < 60).then_some(hours * 60 + minutes)
}

/// The signals the render loop samples each tick — everything a trigger can
/// see. Absent optional signals (idle/focus off Windows) simply never fire
/// their triggers.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Signals {
    /// The program scene's name.
    pub scene: String,
    pub streaming: bool,
    pub recording: bool,
    /// Source names currently in the error state.
    pub errored_sources: Vec<String>,
    /// Mixer strip peaks in dBFS, by source name.
    pub audio_peaks: HashMap<String, f32>,
    /// Seconds the user has been idle (`None` where the OS won't say).
    pub idle_seconds: Option<u32>,
    /// The foreground window's exe (`None` where the OS won't say).
    pub focused_exe: Option<String>,
    /// Local minutes past midnight.
    pub minute_of_day: u32,
    /// Watched file → a cheap content fingerprint.
    pub file_stamps: HashMap<String, u64>,
}

/// What the engine decided to do this tick — a flat, ordered plan the caller
/// executes. The engine itself performs no side effects (that keeps it a
/// pure, testable function).
#[derive(Debug, Clone, PartialEq)]
pub struct Plan {
    /// The rule that fired (for logs).
    pub rule: String,
    pub steps: Vec<MacroStep>,
    pub repeat: u32,
}

/// The rules engine's edge state (what the last tick saw).
#[derive(Debug, Default)]
pub struct Engine {
    last: Option<Signals>,
    /// Variables set by macros — also what text sources interpolate.
    variables: HashMap<String, String>,
}

impl Engine {
    /// Evaluate every enabled rule against the new signals and return the
    /// plans to run. Edge-triggered: a condition that was already true last
    /// tick does not fire again.
    pub fn evaluate(&mut self, settings: &AutomationSettings, now: &Signals) -> Vec<Plan> {
        let previous = self.last.replace(now.clone());
        let Some(previous) = previous else {
            return Vec::new(); // the first tick establishes a baseline only
        };
        let mut plans = Vec::new();
        for rule in settings.rules.iter().filter(|rule| rule.enabled) {
            if !self.fires(rule, &previous, now) {
                continue;
            }
            if !rule.conditions.iter().all(|c| self.holds(c, now)) {
                continue;
            }
            let mut steps = rule.actions.clone();
            let mut repeat = 1;
            if !rule.macro_name.is_empty() {
                if let Some(entry) = settings
                    .macros
                    .iter()
                    .find(|entry| entry.name == rule.macro_name)
                {
                    if steps.is_empty() {
                        repeat = entry.repeat;
                    }
                    steps.extend(entry.steps.iter().cloned());
                }
            }
            if steps.is_empty() {
                continue;
            }
            plans.push(Plan {
                rule: rule.name.clone(),
                steps,
                repeat,
            });
            if plans.len() >= MAX_RUNNING {
                break; // never fork unbounded work in one tick
            }
        }
        plans
    }

    fn fires(&mut self, rule: &Rule, was: &Signals, now: &Signals) -> bool {
        match &rule.trigger {
            Trigger::SceneSwitched { scene } => now.scene == *scene && was.scene != *scene,
            Trigger::StreamState { live } => now.streaming == *live && was.streaming != *live,
            Trigger::RecordingState { recording } => {
                now.recording == *recording && was.recording != *recording
            }
            Trigger::SourceError { source } => {
                let matches = |list: &[String]| {
                    if source.is_empty() {
                        !list.is_empty()
                    } else {
                        list.iter().any(|name| name == source)
                    }
                };
                matches(&now.errored_sources) && !matches(&was.errored_sources)
            }
            Trigger::AudioLevel {
                source,
                threshold_db,
                above,
            } => {
                let (Some(new_peak), Some(old_peak)) =
                    (now.audio_peaks.get(source), was.audio_peaks.get(source))
                else {
                    return false;
                };
                if *above {
                    *new_peak > *threshold_db && *old_peak <= *threshold_db
                } else {
                    *new_peak < *threshold_db && *old_peak >= *threshold_db
                }
            }
            Trigger::SystemIdle { seconds } => {
                let (Some(new_idle), Some(old_idle)) = (now.idle_seconds, was.idle_seconds) else {
                    return false; // the OS won't say — never fires (honest)
                };
                new_idle >= *seconds && old_idle < *seconds
            }
            Trigger::WindowFocus { exe } => {
                let focused = |signals: &Signals| {
                    signals
                        .focused_exe
                        .as_deref()
                        .is_some_and(|current| current.eq_ignore_ascii_case(exe))
                };
                focused(now) && !focused(was)
            }
            Trigger::TimeOfDay { at } => {
                let Some(target) = parse_hhmm(at) else {
                    return false;
                };
                // Edge-triggered on the clock rolling INTO the target minute:
                // fires the one tick the minute becomes `target`, so it fires
                // once per day for as long as the app runs — not once per run.
                now.minute_of_day == target && was.minute_of_day != target
            }
            Trigger::FileChanged { path } => {
                match (now.file_stamps.get(path), was.file_stamps.get(path)) {
                    (Some(new_stamp), Some(old_stamp)) => new_stamp != old_stamp,
                    _ => false,
                }
            }
        }
    }

    fn holds(&self, condition: &Condition, now: &Signals) -> bool {
        match condition {
            Condition::VariableEquals { name, value } => {
                self.variables.get(name).map(String::as_str) == Some(value.as_str())
            }
            Condition::Streaming { live } => now.streaming == *live,
            Condition::Recording { recording } => now.recording == *recording,
        }
    }

    /// Set a studio variable (CAP-N02) — macros write these; text sources
    /// interpolate them; rule conditions read them.
    pub fn set_variable(&mut self, name: &str, value: &str) {
        if self.variables.len() >= 256 && !self.variables.contains_key(name) {
            return; // bounded: a runaway macro can't grow this forever
        }
        self.variables
            .insert(name.to_owned(), value.chars().take(512).collect());
    }

    /// Every variable (for the UI + text interpolation).
    pub fn variables(&self) -> HashMap<String, String> {
        self.variables.clone()
    }

    /// Substitute `{{name}}` tokens in `text` with variable values (unknown
    /// names are left verbatim — a typo shows itself instead of vanishing).
    pub fn interpolate(&self, text: &str) -> String {
        if !text.contains("{{") {
            return text.to_owned();
        }
        let mut out = String::with_capacity(text.len());
        let mut rest = text;
        while let Some(start) = rest.find("{{") {
            out.push_str(&rest[..start]);
            let after = &rest[start + 2..];
            let Some(end) = after.find("}}") else {
                out.push_str(&rest[start..]);
                return out;
            };
            let name = after[..end].trim();
            match self.variables.get(name) {
                Some(value) => out.push_str(value),
                None => {
                    out.push_str("{{");
                    out.push_str(&after[..end]);
                    out.push_str("}}");
                }
            }
            rest = &after[end + 2..];
        }
        out.push_str(rest);
        out
    }
}

/// A macro run in flight (the executor drives these off the loop thread).
pub struct Run {
    pub rule: String,
    pub steps: Vec<MacroStep>,
    pub repeat: u32,
    pub at: usize,
    pub pass: u32,
    /// When the current Wait step ends.
    pub resume_at: Option<Instant>,
}

impl Run {
    pub fn new(plan: Plan) -> Self {
        Self {
            rule: plan.rule,
            steps: plan.steps,
            repeat: plan.repeat.clamp(1, MAX_REPEAT),
            at: 0,
            pass: 0,
            resume_at: None,
        }
    }

    /// The next step to execute right now, if any. `None` = still waiting, or
    /// the run is finished (`is_done`).
    pub fn next_step(&mut self, now: Instant) -> Option<MacroStep> {
        if let Some(resume) = self.resume_at {
            if now < resume {
                return None;
            }
            self.resume_at = None;
        }
        // End of a pass: start the next one, or finish.
        if self.at >= self.steps.len() {
            self.pass += 1;
            if self.pass >= self.repeat || self.steps.is_empty() {
                return None;
            }
            self.at = 0;
        }
        let step = self.steps[self.at].clone();
        self.at += 1;
        // A Wait parks the run instead of blocking the caller.
        if let MacroStep::Wait { ms } = &step {
            self.resume_at = Some(now + Duration::from_millis((*ms).min(MAX_WAIT_MS)));
            return None;
        }
        Some(step)
    }

    pub fn is_done(&self) -> bool {
        self.resume_at.is_none() && self.pass >= self.repeat
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn signals(scene: &str) -> Signals {
        Signals {
            scene: scene.to_owned(),
            ..Signals::default()
        }
    }

    fn rule(name: &str, trigger: Trigger) -> Rule {
        Rule {
            name: name.to_owned(),
            enabled: true,
            trigger,
            conditions: Vec::new(),
            actions: vec![MacroStep::SetVariable {
                name: "fired".to_owned(),
                value: "yes".to_owned(),
            }],
            macro_name: String::new(),
        }
    }

    #[test]
    fn the_first_tick_only_establishes_a_baseline() {
        let mut engine = Engine::default();
        let settings = AutomationSettings {
            rules: vec![rule(
                "r",
                Trigger::SceneSwitched {
                    scene: "Live".to_owned(),
                },
            )],
            ..AutomationSettings::default()
        };
        // Even though the scene already matches, the first evaluate must not
        // fire — otherwise every rule would fire on launch.
        assert!(engine.evaluate(&settings, &signals("Live")).is_empty());
    }

    #[test]
    fn triggers_are_edge_not_level() {
        let mut engine = Engine::default();
        let settings = AutomationSettings {
            rules: vec![rule(
                "r",
                Trigger::SceneSwitched {
                    scene: "Live".to_owned(),
                },
            )],
            ..AutomationSettings::default()
        };
        engine.evaluate(&settings, &signals("Intro"));
        assert_eq!(
            engine.evaluate(&settings, &signals("Live")).len(),
            1,
            "fires on the switch"
        );
        assert!(
            engine.evaluate(&settings, &signals("Live")).is_empty(),
            "does NOT re-fire while it stays there"
        );
    }

    #[test]
    fn disabled_rules_never_fire() {
        let mut engine = Engine::default();
        let mut r = rule(
            "r",
            Trigger::SceneSwitched {
                scene: "Live".to_owned(),
            },
        );
        r.enabled = false;
        let settings = AutomationSettings {
            rules: vec![r],
            ..AutomationSettings::default()
        };
        engine.evaluate(&settings, &signals("Intro"));
        assert!(engine.evaluate(&settings, &signals("Live")).is_empty());
    }

    #[test]
    fn audio_threshold_fires_on_the_crossing_only() {
        let mut engine = Engine::default();
        let settings = AutomationSettings {
            rules: vec![rule(
                "loud",
                Trigger::AudioLevel {
                    source: "Mic".to_owned(),
                    threshold_db: -20.0,
                    above: true,
                },
            )],
            ..AutomationSettings::default()
        };
        let with_peak = |db: f32| Signals {
            audio_peaks: HashMap::from([("Mic".to_owned(), db)]),
            ..Signals::default()
        };
        engine.evaluate(&settings, &with_peak(-40.0));
        assert!(
            engine.evaluate(&settings, &with_peak(-30.0)).is_empty(),
            "still below"
        );
        assert_eq!(
            engine.evaluate(&settings, &with_peak(-10.0)).len(),
            1,
            "crossed up"
        );
        assert!(
            engine.evaluate(&settings, &with_peak(-5.0)).is_empty(),
            "stays loud: no re-fire"
        );
        // Falling back below and rising again fires once more.
        engine.evaluate(&settings, &with_peak(-40.0));
        assert_eq!(engine.evaluate(&settings, &with_peak(-10.0)).len(), 1);
    }

    #[test]
    fn absent_os_signals_never_fire_their_triggers() {
        let mut engine = Engine::default();
        let settings = AutomationSettings {
            rules: vec![
                rule("idle", Trigger::SystemIdle { seconds: 60 }),
                rule(
                    "focus",
                    Trigger::WindowFocus {
                        exe: "game.exe".to_owned(),
                    },
                ),
            ],
            ..AutomationSettings::default()
        };
        // idle_seconds / focused_exe are None (non-Windows, or unavailable).
        engine.evaluate(&settings, &Signals::default());
        assert!(engine.evaluate(&settings, &Signals::default()).is_empty());
    }

    #[test]
    fn conditions_gate_the_actions() {
        let mut engine = Engine::default();
        let mut r = rule(
            "r",
            Trigger::SceneSwitched {
                scene: "Live".to_owned(),
            },
        );
        r.conditions = vec![Condition::Streaming { live: true }];
        let settings = AutomationSettings {
            rules: vec![r],
            ..AutomationSettings::default()
        };
        engine.evaluate(&settings, &signals("Intro"));
        // Not streaming: the trigger fires but the condition blocks it.
        assert!(engine.evaluate(&settings, &signals("Live")).is_empty());

        let mut live_intro = signals("Intro");
        live_intro.streaming = true;
        let mut live_on = signals("Live");
        live_on.streaming = true;
        engine.evaluate(&settings, &live_intro);
        assert_eq!(engine.evaluate(&settings, &live_on).len(), 1);
    }

    #[test]
    fn variables_interpolate_and_bound() {
        let mut engine = Engine::default();
        engine.set_variable("score", "3");
        assert_eq!(engine.interpolate("Score: {{score}}"), "Score: 3");
        assert_eq!(
            engine.interpolate("Hi {{missing}}"),
            "Hi {{missing}}",
            "unknown names stay visible instead of vanishing"
        );
        assert_eq!(engine.interpolate("no tokens"), "no tokens");
        assert_eq!(engine.interpolate("unclosed {{x"), "unclosed {{x");
        // Values are length-bounded.
        engine.set_variable("big", &"x".repeat(1000));
        assert_eq!(engine.variables()["big"].len(), 512);
    }

    #[test]
    fn a_run_walks_steps_waits_and_repeats() {
        let plan = Plan {
            rule: "r".to_owned(),
            steps: vec![
                MacroStep::SetVariable {
                    name: "a".to_owned(),
                    value: "1".to_owned(),
                },
                MacroStep::Wait { ms: 50 },
                MacroStep::SetVariable {
                    name: "b".to_owned(),
                    value: "2".to_owned(),
                },
            ],
            repeat: 2,
        };
        let mut run = Run::new(plan);
        let start = Instant::now();
        assert!(matches!(
            run.next_step(start),
            Some(MacroStep::SetVariable { .. })
        ));
        // The Wait parks the run…
        assert!(run.next_step(start).is_none(), "waiting");
        assert!(run.next_step(start).is_none(), "still waiting");
        // …and it resumes once the wait is over.
        let later = start + Duration::from_millis(60);
        assert!(matches!(
            run.next_step(later),
            Some(MacroStep::SetVariable { .. })
        ));
        // Pass 2 of 2 replays the sequence.
        assert!(matches!(
            run.next_step(later),
            Some(MacroStep::SetVariable { .. })
        ));
        assert!(
            run.next_step(later).is_none(),
            "the second Wait parks again"
        );
        let done_at = later + Duration::from_millis(60);
        assert!(matches!(
            run.next_step(done_at),
            Some(MacroStep::SetVariable { .. })
        ));
        assert!(run.next_step(done_at).is_none());
        assert!(run.is_done());
    }

    #[test]
    fn validation_rejects_unlisted_commands_and_unbounded_files() {
        let bad = AutomationSettings {
            macros: vec![Macro {
                name: "evil".to_owned(),
                steps: vec![MacroStep::Action {
                    // Not in the remote-API allowlist → rejected. There is no
                    // step kind that names a file or runs a process at all.
                    command: "readFile".to_owned(),
                    params: Value::Null,
                }],
                repeat: 1,
                hotkey: None,
                layer: None,
            }],
            rules: Vec::new(),
        };
        assert!(bad.validate().is_err(), "an unlisted command is rejected");

        let good = AutomationSettings {
            macros: vec![Macro {
                name: "go".to_owned(),
                steps: vec![
                    MacroStep::Action {
                        command: "startRecording".to_owned(),
                        params: Value::Null,
                    },
                    MacroStep::Wait { ms: 1000 },
                ],
                repeat: 1,
                hotkey: None,
                layer: None,
            }],
            rules: vec![Rule {
                name: "on-live".to_owned(),
                enabled: true,
                trigger: Trigger::StreamState { live: true },
                conditions: Vec::new(),
                actions: Vec::new(),
                macro_name: "go".to_owned(),
            }],
        };
        assert!(good.validate().is_ok());

        // A rule calling a macro that doesn't exist is a config error.
        let dangling = AutomationSettings {
            macros: Vec::new(),
            rules: vec![Rule {
                name: "r".to_owned(),
                enabled: true,
                trigger: Trigger::StreamState { live: true },
                conditions: Vec::new(),
                actions: Vec::new(),
                macro_name: "nope".to_owned(),
            }],
        };
        assert!(dangling.validate().is_err());

        // Waits are bounded.
        let long_wait = AutomationSettings {
            macros: vec![Macro {
                name: "m".to_owned(),
                steps: vec![MacroStep::Wait {
                    ms: MAX_WAIT_MS + 1,
                }],
                repeat: 1,
                hotkey: None,
                layer: None,
            }],
            rules: Vec::new(),
        };
        assert!(long_wait.validate().is_err());
    }

    #[test]
    fn a_rule_calling_a_macro_runs_its_steps() {
        let mut engine = Engine::default();
        let settings = AutomationSettings {
            macros: vec![Macro {
                name: "intro".to_owned(),
                steps: vec![MacroStep::Action {
                    command: "startRecording".to_owned(),
                    params: Value::Null,
                }],
                repeat: 3,
                hotkey: None,
                layer: None,
            }],
            rules: vec![Rule {
                name: "r".to_owned(),
                enabled: true,
                trigger: Trigger::StreamState { live: true },
                conditions: Vec::new(),
                actions: Vec::new(),
                macro_name: "intro".to_owned(),
            }],
        };
        engine.evaluate(&settings, &Signals::default());
        let plans = engine.evaluate(
            &settings,
            &Signals {
                streaming: true,
                ..Signals::default()
            },
        );
        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].steps.len(), 1);
        assert_eq!(plans[0].repeat, 3, "the macro's repeat carries");
    }

    #[test]
    fn time_of_day_fires_once_per_day() {
        let mut engine = Engine::default();
        let settings = AutomationSettings {
            rules: vec![rule(
                "showtime",
                Trigger::TimeOfDay {
                    at: "20:00".to_owned(),
                },
            )],
            ..AutomationSettings::default()
        };
        let at = |minute: u32| Signals {
            minute_of_day: minute,
            ..Signals::default()
        };
        engine.evaluate(&settings, &at(19 * 60 + 59));
        assert_eq!(engine.evaluate(&settings, &at(20 * 60)).len(), 1);
        assert!(
            engine.evaluate(&settings, &at(20 * 60)).is_empty(),
            "the same minute never fires twice"
        );
        // …but it fires AGAIN the next day: the clock leaves 20:00 and returns.
        engine.evaluate(&settings, &at(0)); // midnight, a new day
        assert_eq!(
            engine.evaluate(&settings, &at(20 * 60)).len(),
            1,
            "a daily rule fires every day, not once per app run"
        );
        assert_eq!(parse_hhmm("20:00"), Some(1200));
        assert_eq!(parse_hhmm("25:00"), None);
        assert_eq!(parse_hhmm("nope"), None);
    }

    #[test]
    fn a_macro_may_not_run_another_macro() {
        // CAP-N02 regression: `runMacro` is allowlisted for the remote API and
        // hotkeys, but as a macro STEP it lets a macro re-queue itself forever
        // (no depth guard). Validation rejects it, so the cycle can't exist.
        let recursive = AutomationSettings {
            macros: vec![Macro {
                name: "loop".to_owned(),
                steps: vec![MacroStep::Action {
                    command: "runMacro".to_owned(),
                    params: serde_json::json!({ "name": "loop" }),
                }],
                repeat: 1,
                hotkey: None,
                layer: None,
            }],
            rules: Vec::new(),
        };
        assert!(
            recursive.validate().is_err(),
            "a macro step cannot be runMacro"
        );
    }
}

// ---------------------------------------------------------------------------
// The runtime: managed state, the loop's tick, and the executor
// ---------------------------------------------------------------------------

use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager, Runtime as TauriRuntime};

/// Managed state: the engine + the in-flight macro runs.
#[derive(Default)]
pub struct AutomationState {
    engine: Mutex<Engine>,
    runs: Mutex<Vec<Run>>,
    /// The last content fingerprint of each watched file (for FileChanged).
    stamps: Mutex<HashMap<String, u64>>,
    /// Bumped on every variable write (CAP-N02). The studio loop watches it so
    /// a variable change repaints the Text sources that interpolate it — those
    /// writes don't touch the collection revision, so this is their signal.
    variables_revision: std::sync::atomic::AtomicU64,
}

fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

impl AutomationState {
    /// Every studio variable (CAP-N02) — the UI reads these; text sources
    /// interpolate them.
    pub fn variables(&self) -> HashMap<String, String> {
        lock(&self.engine).variables()
    }

    /// Set one variable directly (the UI + the macro executor).
    pub fn set_variable(&self, name: &str, value: &str) {
        lock(&self.engine).set_variable(name, value);
        self.variables_revision
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    /// A monotonically-increasing counter the studio loop compares each tick;
    /// a change means a Text source may need repainting (CAP-N02).
    pub fn variables_revision(&self) -> u64 {
        self.variables_revision
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Substitute `{{name}}` tokens (text sources, CAP-N02).
    pub fn interpolate(&self, text: &str) -> String {
        lock(&self.engine).interpolate(text)
    }

    /// Queue a plan (a rule fired, or the user pressed Run).
    fn queue(&self, plan: Plan) {
        let mut runs = lock(&self.runs);
        if runs.len() >= MAX_RUNNING {
            eprintln!(
                "automation: too many macros running — dropping \"{}\"",
                plan.rule
            );
            return;
        }
        runs.push(Run::new(plan));
    }
}

/// Which watched paths the rules reference (FileChanged triggers).
fn watched_paths(settings: &AutomationSettings) -> Vec<String> {
    settings
        .rules
        .iter()
        .filter(|rule| rule.enabled)
        .filter_map(|rule| match &rule.trigger {
            Trigger::FileChanged { path } => Some(path.clone()),
            _ => None,
        })
        .collect()
}

/// A cheap fingerprint of a file's content (size + mtime). Never reads the
/// body — and never probes a remote path (the NTLM-leak rule).
fn file_stamp(path: &str) -> Option<u64> {
    if crate::commands::studio::is_remote(path) {
        return None;
    }
    let meta = std::fs::metadata(path).ok()?;
    let modified = meta
        .modified()
        .ok()?
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?
        .as_secs();
    Some(meta.len() ^ (modified << 20))
}

// The OS signals live in fcap-capture (the audited FFI crate — this crate is
// `forbid(unsafe_code)`). Both are `None` off Windows, so their triggers
// simply never fire; the UI says so honestly.
use fcap_capture::signals::{foreground_exe as focused_exe, idle_seconds};

/// One evaluation tick, called from the render loop (~1 Hz — triggers are
/// human-scale, and a cheap tick keeps the 60 fps budget untouched).
/// Cheap no-op when no rule is enabled.
pub fn tick<R: TauriRuntime>(
    app: &AppHandle<R>,
    scene: &str,
    errored: Vec<String>,
    source_names: &HashMap<fcap_scene::SourceId, String>,
) {
    let settings = app
        .state::<crate::settings::SettingsStore>()
        .get()
        .automation;
    let state = app.state::<AutomationState>();
    // Evaluating rules is the expensive part (signal-gathering) — skip it when
    // nothing is enabled. But ALWAYS advance in-flight runs below: a macro run
    // by hand (CAP-N02) can be waiting even when no rule is enabled, and its
    // Wait only resumes on a later drive().
    if settings.rules.iter().all(|rule| !rule.enabled) {
        drive(app);
        return;
    }

    let streaming = app
        .state::<crate::stream::StreamBridgeState>()
        .wants_frames();
    let recording = app
        .state::<crate::recording::RecordingState>()
        .wants_frames();
    let audio_peaks = app
        .state::<crate::audio::AudioRuntime>()
        .peaks_by_name(source_names);
    let now = chrono::Local::now();
    let minute_of_day = {
        use chrono::Timelike;
        now.hour() * 60 + now.minute()
    };
    let mut file_stamps = HashMap::new();
    {
        let mut stamps = lock(&state.stamps);
        for path in watched_paths(&settings) {
            if let Some(stamp) = file_stamp(&path) {
                stamps.insert(path.clone(), stamp);
                file_stamps.insert(path, stamp);
            }
        }
    }

    let signals = Signals {
        scene: scene.to_owned(),
        streaming,
        recording,
        errored_sources: errored,
        audio_peaks,
        idle_seconds: idle_seconds(),
        focused_exe: focused_exe(),
        minute_of_day,
        file_stamps,
    };

    let plans = lock(&state.engine).evaluate(&settings, &signals);
    for plan in plans {
        let _ = app.emit("automation-fired", plan.rule.clone());
        state.queue(plan);
    }
    drive(app);
}

/// Advance every in-flight run: execute the steps that are due (Waits park a
/// run without blocking the loop), then drop the finished ones.
pub fn drive<R: TauriRuntime>(app: &AppHandle<R>) {
    let state = app.state::<AutomationState>();
    let now = Instant::now();
    let mut due: Vec<(String, MacroStep)> = Vec::new();
    {
        let mut runs = lock(&state.runs);
        for run in runs.iter_mut() {
            // Bounded work per tick: a tight macro can't starve the loop.
            for _ in 0..MAX_STEPS {
                match run.next_step(now) {
                    Some(step) => due.push((run.rule.clone(), step)),
                    None => break,
                }
            }
        }
        runs.retain(|run| !run.is_done());
    }
    if due.is_empty() {
        return;
    }
    // Run this tick's due steps on ONE worker thread, IN ORDER — a macro's
    // steps were authored as a sequence, so `setProgramScene` must commit
    // before the `startRecording` that follows it. A blocking dispatch here
    // never stalls the studio loop (that's why this is off-thread), but the
    // per-run ordering the operator wrote is preserved.
    let handle = app.clone();
    std::thread::spawn(move || {
        let state = handle.state::<AutomationState>();
        for (rule, step) in due {
            match step {
                MacroStep::SetVariable { name, value } => {
                    let value = state.interpolate(&value);
                    state.set_variable(&name, &value);
                }
                MacroStep::Action { command, params } => {
                    // Every action is an allowlisted studio command — the same
                    // vocabulary the remote API exposes, nothing more.
                    if let Err(err) = crate::remote_api::dispatch_any(&handle, &command, &params) {
                        eprintln!("automation: {rule}: action {command} failed: {err}");
                    }
                }
                MacroStep::Wait { .. } => {} // handled inside Run
            }
        }
    });
}

/// Run a macro by name (a hotkey, the UI's Run button, or the remote API).
pub fn run_macro_by_name<R: TauriRuntime>(app: &AppHandle<R>, name: &str) {
    let settings = app
        .state::<crate::settings::SettingsStore>()
        .get()
        .automation;
    let Some(entry) = settings.macros.iter().find(|entry| entry.name == name) else {
        eprintln!("automation: no macro named {name}");
        return;
    };
    app.state::<AutomationState>().queue(Plan {
        rule: entry.name.clone(),
        steps: entry.steps.clone(),
        repeat: entry.repeat,
    });
    drive(app);
}
