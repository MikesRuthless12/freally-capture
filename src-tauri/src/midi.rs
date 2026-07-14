//! MIDI control surfaces (CAP-N03): MIDI-learn any pad, knob, or fader onto a
//! studio action — with **output feedback**, so the pad's LED and a motor
//! fader mirror the studio's real state (the REC light turns red because the
//! studio is recording, not because you pressed something).
//!
//! Posture, same as every other surface here:
//! - **Off by default.** No MIDI port is opened until the operator picks one.
//! - Actions come from the **fixed remote-API allowlist** — a MIDI pad can
//!   ask for nothing the app's own buttons can't.
//! - Nothing is discovered or connected behind the operator's back: ports are
//!   listed on request, and only the chosen one is opened.
//!
//! Dependency note: `midir` (MIT) is the only new dependency in this run; it
//! is recorded in THIRD-PARTY-NOTICES.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use midir::{MidiInput, MidiInputConnection, MidiOutput, MidiOutputConnection};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tauri::{AppHandle, Emitter, Manager};

/// The most bindings one profile may hold.
pub const MAX_BINDINGS: usize = 128;

/// What a MIDI control is bound to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum MidiTarget {
    /// Run one allowlisted studio command.
    Action {
        command: String,
        #[serde(default)]
        params: Value,
    },
    /// Run a named macro (CAP-N02).
    Macro { name: String },
    /// Drive a mixer strip's fader (a CC's 0–127 maps to −60…+6 dB).
    Volume { source: String },
    /// Toggle a mixer strip's mute (a pad).
    Mute { source: String },
    /// Switch the program scene (a pad).
    Scene { scene: String },
}

/// Which physical control this is (what MIDI-learn captured).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum MidiControl {
    /// A pad / key (note-on).
    Note { channel: u8, note: u8 },
    /// A knob / fader (continuous controller).
    Cc { channel: u8, cc: u8 },
}

/// One learned binding.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MidiBinding {
    pub control: MidiControl,
    pub target: MidiTarget,
    /// Light this pad's LED (send note-on) while the bound state is "on" —
    /// REC lit while recording, a scene pad lit while it is on program.
    #[serde(default)]
    pub feedback: bool,
}

/// The persisted MIDI config (CAP-N03). Off until a port is chosen.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct MidiSettings {
    /// The input port name the operator picked (`""` = none, nothing opens).
    pub input: String,
    /// The output port for LED/fader feedback (`""` = no feedback).
    pub output: String,
    pub bindings: Vec<MidiBinding>,
}

impl MidiSettings {
    pub fn validate(&self) -> Result<(), String> {
        if self.input.len() > 128 || self.output.len() > 128 {
            return Err("a MIDI port name is too long".to_owned());
        }
        if self.bindings.len() > MAX_BINDINGS {
            return Err(format!("too many MIDI bindings ({MAX_BINDINGS} max)"));
        }
        for binding in &self.bindings {
            if let MidiTarget::Action { command, .. } = &binding.target {
                if !crate::remote_api::is_allowed_command(command) {
                    return Err(format!("not an allowed studio command: {command}"));
                }
            }
        }
        Ok(())
    }
}

/// A control-change / note value (0–127) → a mixer fader position in dB.
/// The taper is linear over the strip's usable range, which is what a motor
/// fader expects to read back.
pub fn cc_to_db(value: u8) -> f32 {
    let t = f32::from(value.min(127)) / 127.0;
    -60.0 + t * 66.0 // −60 dB … +6 dB
}

/// The inverse (for motor-fader feedback).
pub fn db_to_cc(db: f32) -> u8 {
    let t = ((db + 60.0) / 66.0).clamp(0.0, 1.0);
    (t * 127.0).round() as u8
}

/// Decode one raw MIDI message into the control it names, plus its value.
/// Note-offs and note-ons with velocity 0 are "release" and are ignored (a
/// pad fires on press, like every other button in the studio).
pub fn decode(message: &[u8]) -> Option<(MidiControl, u8)> {
    let status = *message.first()?;
    let channel = status & 0x0F;
    match status & 0xF0 {
        0x90 => {
            let note = *message.get(1)?;
            let velocity = *message.get(2)?;
            (velocity > 0).then_some((MidiControl::Note { channel, note }, velocity))
        }
        0xB0 => {
            let cc = *message.get(1)?;
            let value = *message.get(2)?;
            Some((MidiControl::Cc { channel, cc }, value))
        }
        _ => None, // note-off, pitch bend, clock, sysex: not ours
    }
}

/// Map a bound control + its value onto a studio command (the allowlist).
pub fn to_command(binding: &MidiBinding, value: u8) -> Option<(String, Value)> {
    match &binding.target {
        MidiTarget::Action { command, params } => Some((command.clone(), params.clone())),
        MidiTarget::Macro { name } => Some(("runMacro".into(), json!({ "name": name }))),
        MidiTarget::Scene { scene } => Some(("setProgramScene".into(), json!({ "scene": scene }))),
        MidiTarget::Volume { source } => Some((
            "setAudioVolume".into(),
            json!({ "sourceName": source, "volumeDb": cc_to_db(value) }),
        )),
        MidiTarget::Mute { source } => Some((
            "setAudioMuted".into(),
            // The absolute value here is a placeholder — `on_message` computes
            // the real toggle (invert the strip's live mute) before dispatch,
            // because a note-on carries no press/release to encode a toggle.
            json!({ "sourceName": source, "muted": value > 0 }),
        )),
    }
}

/// Managed state: the open ports (when configured) + MIDI-learn.
#[derive(Default)]
pub struct MidiState {
    input: Mutex<Option<MidiInputConnection<()>>>,
    output: Mutex<Option<Arc<Mutex<MidiOutputConnection>>>>,
    seen: Mutex<Option<MidiSettings>>,
    /// While true, the next control captured is reported to the UI instead of
    /// being dispatched (MIDI-learn).
    learning: Mutex<bool>,
}

fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

impl MidiState {
    pub fn set_learning(&self, on: bool) {
        *lock(&self.learning) = on;
    }
}

/// The available MIDI ports (listed on request — nothing is opened).
pub fn list_ports() -> (Vec<String>, Vec<String>) {
    let inputs = MidiInput::new("freally-capture")
        .map(|midi| {
            midi.ports()
                .iter()
                .filter_map(|port| midi.port_name(port).ok())
                .collect()
        })
        .unwrap_or_default();
    let outputs = MidiOutput::new("freally-capture")
        .map(|midi| {
            midi.ports()
                .iter()
                .filter_map(|port| midi.port_name(port).ok())
                .collect()
        })
        .unwrap_or_default();
    (inputs, outputs)
}

/// Open (or close) the configured ports to match settings. Cheap no-op when
/// nothing changed; closes everything when no input is chosen.
pub fn reconcile(app: &AppHandle) {
    let settings = app.state::<crate::settings::SettingsStore>().get().midi;
    let state = app.state::<MidiState>();
    if lock(&state.seen).as_ref() == Some(&settings) {
        return;
    }
    *lock(&state.seen) = Some(settings.clone());
    // Any change closes the old ports first.
    *lock(&state.input) = None;
    *lock(&state.output) = None;
    if settings.input.trim().is_empty() {
        return; // off by default: no device is opened at all
    }

    // The output first, so the very first input event can already light a pad.
    if !settings.output.trim().is_empty() {
        match open_output(&settings.output) {
            Ok(connection) => {
                *lock(&state.output) = Some(Arc::new(Mutex::new(connection)));
            }
            Err(err) => eprintln!("midi: could not open the output port: {err}"),
        }
    }
    match open_input(app.clone(), &settings.input) {
        Ok(connection) => {
            println!("midi: listening on {}", settings.input);
            *lock(&state.input) = Some(connection);
        }
        Err(err) => eprintln!("midi: could not open the input port: {err}"),
    }
}

fn open_output(name: &str) -> Result<MidiOutputConnection, String> {
    let midi = MidiOutput::new("freally-capture").map_err(|err| err.to_string())?;
    let port = midi
        .ports()
        .into_iter()
        .find(|port| midi.port_name(port).as_deref() == Ok(name))
        .ok_or_else(|| format!("no MIDI output named {name}"))?;
    midi.connect(&port, "freally-out")
        .map_err(|err| err.to_string())
}

fn open_input(app: AppHandle, name: &str) -> Result<MidiInputConnection<()>, String> {
    let mut midi = MidiInput::new("freally-capture").map_err(|err| err.to_string())?;
    midi.ignore(midir::Ignore::All); // clock/sysex/active-sensing are noise here
    let port = midi
        .ports()
        .into_iter()
        .find(|port| midi.port_name(port).as_deref() == Ok(name))
        .ok_or_else(|| format!("no MIDI input named {name}"))?;
    midi.connect(
        &port,
        "freally-in",
        move |_stamp, message, _| on_message(&app, message),
        (),
    )
    .map_err(|err| err.to_string())
}

/// One inbound MIDI message (on midir's callback thread).
fn on_message(app: &AppHandle, message: &[u8]) {
    let Some((control, value)) = decode(message) else {
        return;
    };
    let state = app.state::<MidiState>();
    // MIDI-learn: report the control to the UI instead of acting on it.
    if *lock(&state.learning) {
        state.set_learning(false);
        let _ = app.emit("midi-learned", control);
        return;
    }
    let settings = app.state::<crate::settings::SettingsStore>().get().midi;
    let Some(binding) = settings
        .bindings
        .iter()
        .find(|binding| binding.control == control)
    else {
        return; // an unbound control: ignored, like any surface does
    };
    // A Mute pad TOGGLES: a note-on has no release, so the value can't carry
    // an on/off — read the strip's live mute and dispatch the inverse, so
    // each press flips it instead of only ever muting.
    let (command, params) = if let MidiTarget::Mute { source } = &binding.target {
        let studio = app.state::<crate::studio::StudioState>().snapshot();
        let muted = studio
            .collection
            .sources
            .iter()
            .find(|entry| entry.name == *source)
            .and_then(|entry| entry.audio.as_ref())
            .is_some_and(|audio| audio.muted);
        (
            "setAudioMuted".to_string(),
            json!({ "sourceName": source, "muted": !muted }),
        )
    } else {
        match to_command(binding, value) {
            Some(pair) => pair,
            None => return,
        }
    };
    let handle = app.clone();
    std::thread::spawn(move || {
        // `dispatch_any` re-checks the allowlist itself.
        if let Err(err) = crate::remote_api::dispatch_any(&handle, &command, &params) {
            eprintln!("midi: {command} failed: {err}");
        }
    });
}

/// Push the studio's state back out to the surface (CAP-N03's feedback): pad
/// LEDs and motor faders mirror what the studio is actually doing. Called
/// from the loop's 1 Hz tick; a no-op with no output port.
pub fn feedback(app: &AppHandle) {
    let state = app.state::<MidiState>();
    let Some(output) = lock(&state.output).clone() else {
        return;
    };
    let settings = app.state::<crate::settings::SettingsStore>().get().midi;
    if settings.bindings.is_empty() {
        return;
    }
    let studio = app.state::<crate::studio::StudioState>().snapshot();
    let recording = app
        .state::<crate::recording::RecordingState>()
        .wants_frames();
    let live = app
        .state::<crate::stream::StreamBridgeState>()
        .wants_frames();
    let program = studio.collection.active_scene;
    let muted: HashMap<&str, bool> = studio
        .collection
        .sources
        .iter()
        .filter_map(|source| {
            source
                .audio
                .as_ref()
                .map(|audio| (source.name.as_str(), audio.muted))
        })
        .collect();
    let volumes: HashMap<&str, f32> = studio
        .collection
        .sources
        .iter()
        .filter_map(|source| {
            source
                .audio
                .as_ref()
                .map(|audio| (source.name.as_str(), audio.volume_db))
        })
        .collect();

    let mut connection = lock(&output);
    for binding in settings.bindings.iter().filter(|binding| binding.feedback) {
        let message = match (&binding.control, &binding.target) {
            // A pad's LED: lit while the bound state is on.
            (MidiControl::Note { channel, note }, target) => {
                let on = match target {
                    MidiTarget::Scene { scene } => studio
                        .collection
                        .scenes
                        .iter()
                        .any(|entry| entry.name == *scene && entry.id == program),
                    MidiTarget::Mute { source } => {
                        muted.get(source.as_str()).copied().unwrap_or(false)
                    }
                    MidiTarget::Action { command, .. } => match command.as_str() {
                        "startRecording" | "stopRecording" => recording,
                        "startStream" | "stopStream" => live,
                        _ => false,
                    },
                    MidiTarget::Macro { .. } | MidiTarget::Volume { .. } => false,
                };
                let velocity = if on { 127 } else { 0 };
                vec![0x90 | (channel & 0x0F), *note, velocity]
            }
            // A motor fader follows the strip's real level.
            (MidiControl::Cc { channel, cc }, MidiTarget::Volume { source }) => {
                let db = volumes.get(source.as_str()).copied().unwrap_or(-60.0);
                vec![0xB0 | (channel & 0x0F), *cc, db_to_cc(db)]
            }
            _ => continue,
        };
        let _ = connection.send(&message);
    }
}

/// Poll settings ~1 Hz, keep the ports in sync, and push feedback out.
pub fn spawn_manager(app: AppHandle) {
    std::thread::Builder::new()
        .name("fcap-midi".into())
        .spawn(move || loop {
            reconcile(&app);
            feedback(&app);
            std::thread::sleep(Duration::from_secs(1));
        })
        .expect("midi manager thread spawns");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notes_and_ccs_decode_releases_are_ignored() {
        // Note-on, channel 1, note 60, velocity 100.
        assert_eq!(
            decode(&[0x90, 60, 100]),
            Some((
                MidiControl::Note {
                    channel: 0,
                    note: 60
                },
                100
            ))
        );
        // A note-on with velocity 0 IS a release: a pad fires on press only.
        assert_eq!(decode(&[0x90, 60, 0]), None);
        // A real note-off, likewise.
        assert_eq!(decode(&[0x80, 60, 64]), None);
        // CC 7 (volume) on channel 3.
        assert_eq!(
            decode(&[0xB2, 7, 64]),
            Some((MidiControl::Cc { channel: 2, cc: 7 }, 64))
        );
        // Clock, sysex, and truncated messages are not ours.
        assert_eq!(decode(&[0xF8]), None);
        assert_eq!(decode(&[0x90, 60]), None, "truncated");
        assert_eq!(decode(&[]), None);
    }

    #[test]
    fn the_fader_taper_round_trips() {
        assert!((cc_to_db(127) - 6.0).abs() < 0.01, "top of the fader");
        assert!((cc_to_db(0) + 60.0).abs() < 0.01, "bottom of the fader");
        // A motor fader reads back what it sent (within one step).
        for value in [0u8, 32, 64, 96, 127] {
            let round = db_to_cc(cc_to_db(value));
            assert!(
                round.abs_diff(value) <= 1,
                "cc {value} → {} dB → cc {round}",
                cc_to_db(value)
            );
        }
        assert_eq!(db_to_cc(999.0), 127, "clamps");
        assert_eq!(db_to_cc(-999.0), 0);
    }

    #[test]
    fn every_binding_maps_onto_the_allowlist() {
        let cases = vec![
            MidiTarget::Action {
                command: "startRecording".to_owned(),
                params: Value::Null,
            },
            MidiTarget::Macro {
                name: "Intro".to_owned(),
            },
            MidiTarget::Scene {
                scene: "Live".to_owned(),
            },
            MidiTarget::Volume {
                source: "Mic".to_owned(),
            },
            MidiTarget::Mute {
                source: "Mic".to_owned(),
            },
        ];
        for target in cases {
            let binding = MidiBinding {
                control: MidiControl::Note {
                    channel: 0,
                    note: 60,
                },
                target,
                feedback: true,
            };
            let (command, _) = to_command(&binding, 100).expect("maps");
            assert!(
                crate::remote_api::is_allowed_command(&command),
                "{command} must be on the allowlist"
            );
        }
    }

    #[test]
    fn settings_are_off_by_default_and_validated() {
        let settings = MidiSettings::default();
        assert!(settings.input.is_empty(), "no port opens by default");
        assert!(settings.validate().is_ok());

        let evil = MidiSettings {
            input: "Pad".to_owned(),
            output: String::new(),
            bindings: vec![MidiBinding {
                control: MidiControl::Note {
                    channel: 0,
                    note: 1,
                },
                target: MidiTarget::Action {
                    command: "readFile".to_owned(),
                    params: Value::Null,
                },
                feedback: false,
            }],
        };
        assert!(
            evil.validate().is_err(),
            "an off-allowlist command is rejected at save"
        );
    }

    #[test]
    fn a_volume_cc_carries_its_decibels() {
        let binding = MidiBinding {
            control: MidiControl::Cc { channel: 0, cc: 7 },
            target: MidiTarget::Volume {
                source: "Mic".to_owned(),
            },
            feedback: false,
        };
        let (command, params) = to_command(&binding, 127).expect("maps");
        assert_eq!(command, "setAudioVolume");
        assert_eq!(params["sourceName"], "Mic");
        assert!((params["volumeDb"].as_f64().expect("f64") - 6.0).abs() < 0.01);
    }
}
