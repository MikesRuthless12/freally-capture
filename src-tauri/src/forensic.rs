//! CAP-N50 dropped-frame forensic timeline: the session-long recorder that
//! answers "what happened at minute 43?" after the fact.
//!
//! The stats dock shows *now*; this records and correlates the whole
//! session: render ms, encoder queue depth, per-target bitrate/health,
//! reconnects, scene switches, alarms, and encoder fallbacks on one
//! timeline. A dedicated 1 Hz thread samples state the render loop already
//! publishes lock-free (`RuntimeStats`) plus the stream/recording status
//! snapshots — the compose loop itself gains **zero** new work, so the
//! logging overhead on the frame budget is nil by construction. Discrete
//! moments arrive two ways: cheap hooks at the two central emit sites
//! (alarms, encoder fallbacks) and poll-derived edges (scene switches,
//! per-target state changes, reconnect deltas, session start/stop).
//!
//! A session spans "anything is on": it opens when streaming (real or
//! CAP-N49 rehearsal) or recording starts and closes when both are idle —
//! the closed session stays queryable (and feeds the CAP-N51 report) until
//! the next one begins. Everything lives in memory; nothing is written to
//! disk here.

use std::sync::Mutex;
use std::time::Instant;

use serde::Serialize;
use tauri::{AppHandle, Manager, Runtime};

/// Hard caps so a forgotten week-long session cannot eat the machine:
/// 24 h of 1 Hz samples, and a generous event budget.
const MAX_SAMPLES: usize = 86_400;
const MAX_EVENTS: usize = 10_000;

/// One 1 Hz sample of the correlated numbers.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ForensicSample {
    /// Milliseconds since the session began.
    pub t_ms: u64,
    /// Program compose rate (fps) and mean compose time (µs/frame).
    pub fps: u32,
    pub render_us: u64,
    /// Capture-pipeline drops since app start (monotonic; deltas matter).
    pub dropped: u64,
    /// Encoder queue depth: how many frames the recorder's CFR clock is
    /// behind (0 while not recording).
    pub frames_behind: u64,
    /// Per-target publish health at this moment.
    pub targets: Vec<ForensicTarget>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ForensicTarget {
    pub id: usize,
    pub label: String,
    /// "live" | "reconnecting" | "failed" | "ended".
    pub state: String,
    pub kbps: u32,
    pub reconnects: u32,
    pub frames_dropped: u64,
}

/// One discrete moment on the timeline.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ForensicEvent {
    pub t_ms: u64,
    /// "scene" | "alarm" | "alarm-clear" | "fallback" | "reconnect" |
    /// "target" | "recording" | "stream".
    pub kind: String,
    pub label: String,
}

/// The whole recorded session, as the UI and the CAP-N51 report consume it.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionData {
    /// Wall-clock start (unix ms) — lets the UI print real times of day.
    pub started_unix_ms: u64,
    /// CAP-N49: the session was a dry run (with the armed simulator profile,
    /// if any) — the timeline explains *induced* events honestly.
    pub rehearsal: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulator: Option<String>,
    pub samples: Vec<ForensicSample>,
    pub events: Vec<ForensicEvent>,
    /// Every recording file the session produced (CAP-N51 lists them).
    pub recording_paths: Vec<String>,
    /// Total session length (ms) — set when the session closes.
    pub ended_t_ms: Option<u64>,
}

struct LiveSession {
    started: Instant,
    data: SessionData,
    /// Poll-edge memory.
    last_scene: String,
    last_states: Vec<(usize, String, u32)>,
    was_recording: bool,
    was_streaming: bool,
}

/// Managed state: the running session (if any) plus the last closed one.
#[derive(Default)]
pub struct ForensicState {
    live: Mutex<Option<LiveSession>>,
    last: Mutex<Option<SessionData>>,
}

impl ForensicState {
    fn lock_live(&self) -> std::sync::MutexGuard<'_, Option<LiveSession>> {
        self.live
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn lock_last(&self) -> std::sync::MutexGuard<'_, Option<SessionData>> {
        self.last
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// Record a discrete moment (no-op outside a session). Called from the
    /// alarm, marker, and encoder-fallback emit sites — all rare, all off
    /// the per-frame path.
    pub fn note(&self, kind: &str, label: &str) {
        let mut live = self.lock_live();
        if let Some(session) = live.as_mut() {
            push_event(session, kind, label);
        }
    }
}

fn push_event(session: &mut LiveSession, kind: &str, label: &str) {
    if session.data.events.len() >= MAX_EVENTS {
        return;
    }
    session.data.events.push(ForensicEvent {
        t_ms: session.started.elapsed().as_millis() as u64,
        kind: kind.to_string(),
        label: label.to_string(),
    });
}

/// Record a discrete moment from anywhere with an `AppHandle`.
pub fn note<R: Runtime>(app: &AppHandle<R>, kind: &str, label: &str) {
    app.state::<ForensicState>().note(kind, label);
}

/// The 1 Hz sampler. Opens a session when streaming or recording starts,
/// samples + derives edges while one runs, and closes it (into `last`)
/// when everything stops.
pub fn spawn_forensic_thread<R: Runtime>(app: AppHandle<R>) {
    std::thread::Builder::new()
        .name("fcap-forensic".into())
        .spawn(move || loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            let state = app.state::<ForensicState>();
            let streaming = app.state::<crate::stream::StreamBridgeState>();
            let recording = app.state::<crate::recording::RecordingState>();
            let stream_dto = streaming.status();
            let is_streaming = streaming.is_live();
            let is_recording = recording.is_active();
            let mut live = state.lock_live();
            if !is_streaming && !is_recording {
                if let Some(mut session) = live.take() {
                    let ended = session.started.elapsed().as_millis() as u64;
                    session.data.ended_t_ms = Some(ended);
                    // CAP-N51: the optional post-show report, written next
                    // to the recording once everything has wound down —
                    // local only, shareable by the user only.
                    if app
                        .state::<crate::settings::SettingsStore>()
                        .get()
                        .stream
                        .session_report
                    {
                        crate::report::write_session_report(&app, &session.data);
                    }
                    *state.lock_last() = Some(session.data);
                }
                continue;
            }
            let scene_name = app
                .state::<crate::studio::StudioState>()
                .with_collection(|collection| collection.active_scene().name.clone());
            let session = live.get_or_insert_with(|| {
                let started_unix_ms = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_millis() as u64)
                    .unwrap_or(0);
                LiveSession {
                    started: Instant::now(),
                    data: SessionData {
                        started_unix_ms,
                        rehearsal: stream_dto.rehearsal,
                        simulator: stream_dto.simulator.clone(),
                        samples: Vec::new(),
                        events: Vec::new(),
                        recording_paths: Vec::new(),
                        ended_t_ms: None,
                    },
                    last_scene: scene_name.clone(),
                    last_states: Vec::new(),
                    was_recording: false,
                    was_streaming: false,
                }
            });

            // Poll-derived edges: session pieces coming and going...
            if is_recording != session.was_recording {
                session.was_recording = is_recording;
                let label = if is_recording { "start" } else { "stop" };
                push_event(session, "recording", label);
            }
            if is_streaming != session.was_streaming {
                session.was_streaming = is_streaming;
                let label = if is_streaming {
                    if stream_dto.rehearsal {
                        "rehearsal start"
                    } else {
                        "start"
                    }
                } else {
                    "stop"
                };
                push_event(session, "stream", label);
            }
            // ...the program scene switching...
            if scene_name != session.last_scene {
                session.last_scene = scene_name.clone();
                push_event(session, "scene", &scene_name);
            }
            // ...and per-target health changes / reconnect attempts.
            let mut target_events: Vec<(&'static str, String)> = Vec::new();
            for target in &stream_dto.targets {
                let seen = session
                    .last_states
                    .iter_mut()
                    .find(|(id, _, _)| *id == target.id);
                match seen {
                    Some((_, last_state, last_reconnects)) => {
                        if *last_state != target.state {
                            *last_state = target.state.clone();
                            target_events
                                .push(("target", format!("{}: {}", target.label, target.state)));
                        }
                        if target.reconnects > *last_reconnects {
                            *last_reconnects = target.reconnects;
                            target_events.push((
                                "reconnect",
                                format!("{} (#{})", target.label, target.reconnects),
                            ));
                        }
                    }
                    None => {
                        session.last_states.push((
                            target.id,
                            target.state.clone(),
                            target.reconnects,
                        ));
                    }
                }
            }
            for (kind, label) in target_events {
                push_event(session, kind, &label);
            }

            // The 1 Hz correlated sample.
            if session.data.samples.len() < MAX_SAMPLES {
                let (_, fps, _, dropped, render_us) =
                    app.state::<crate::events::RuntimeStats>().latest();
                let recording_dto = recording.status();
                // CAP-N51: remember every file this session produced.
                if let crate::recording::RecordingDto::Recording { path, .. } = &recording_dto {
                    if !session.data.recording_paths.contains(path) {
                        session.data.recording_paths.push(path.clone());
                    }
                }
                let frames_behind = match recording_dto {
                    crate::recording::RecordingDto::Recording { frames_behind, .. } => {
                        frames_behind
                    }
                    _ => 0,
                };
                session.data.samples.push(ForensicSample {
                    t_ms: session.started.elapsed().as_millis() as u64,
                    fps,
                    render_us,
                    dropped,
                    frames_behind,
                    targets: stream_dto
                        .targets
                        .iter()
                        .map(|target| ForensicTarget {
                            id: target.id,
                            label: target.label.clone(),
                            state: target.state.clone(),
                            kbps: target.kbps,
                            reconnects: target.reconnects,
                            frames_dropped: target.frames_dropped,
                        })
                        .collect(),
                });
            }
        })
        .expect("forensic thread spawns");
}

/// The timeline the UI graphs: the running session, or the last closed one.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimelineDto {
    /// True while the shown session is still recording new samples.
    pub active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<SessionData>,
}

#[tauri::command]
pub fn forensic_timeline(state: tauri::State<'_, ForensicState>) -> TimelineDto {
    let live = state.lock_live();
    match live.as_ref() {
        Some(session) => TimelineDto {
            active: true,
            session: Some(session.data.clone()),
        },
        None => TimelineDto {
            active: false,
            session: state.lock_last().clone(),
        },
    }
}
