//! Quit guard + orderly shutdown (CAP-M23).
//!
//! Closing the main window while live output is running would tear the
//! stream, truncate the recording, and drop the replay ring mid-write. The
//! guard asks first (the `quit-guard` event → the webview's confirm), then
//! winds everything down **in order** — end stream → finalize recordings →
//! flush replay → save state — before exiting.
//!
//! A `session-running` marker file distinguishes clean from unclean exits:
//! created at startup, removed at `RunEvent::Exit` only when nothing needed
//! finalizing. Found at the next startup ⇒ the previous session died (crash,
//! kill, power loss) — the CAP-M11 salvage prompt's signal.

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};

use serde::Serialize;
use tauri::{AppHandle, Manager, Runtime};

/// Managed quit state.
pub struct QuitState {
    /// A confirmed shutdown is underway — close events stop re-prompting.
    quitting: AtomicBool,
    /// A quit-guard confirm has been emitted and not answered. A SECOND
    /// close attempt while armed proceeds with the ordered shutdown — the
    /// escape hatch when the webview is hung or dead and can never show or
    /// answer the confirm (the app must always remain closable).
    prompted: AtomicBool,
    /// The previous session's marker was still present at startup.
    prev_unclean: bool,
}

impl QuitState {
    pub fn new(prev_unclean: bool) -> Self {
        QuitState {
            quitting: AtomicBool::new(false),
            prompted: AtomicBool::new(false),
            prev_unclean,
        }
    }

    pub fn is_quitting(&self) -> bool {
        self.quitting.load(Ordering::SeqCst)
    }

    pub fn arm_prompt(&self) {
        self.prompted.store(true, Ordering::SeqCst);
    }

    pub fn disarm_prompt(&self) {
        self.prompted.store(false, Ordering::SeqCst);
    }

    pub fn prompt_armed(&self) -> bool {
        self.prompted.load(Ordering::SeqCst)
    }

    /// Whether the LAST run ended without an orderly shutdown. The salvage
    /// prompt (CAP-M11) keys off its own sidecar — this annotates the log.
    pub fn previous_exit_was_unclean(&self) -> bool {
        self.prev_unclean
    }
}

/// What quitting right now would interrupt — the confirm dialog's list
/// (mirrored in `ui/src/api/types.ts`).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuitConsequences {
    pub streaming: bool,
    pub recording: bool,
    pub replay: bool,
}

impl QuitConsequences {
    pub fn any(&self) -> bool {
        self.streaming || self.recording || self.replay
    }
}

pub fn consequences<R: Runtime>(app: &AppHandle<R>) -> QuitConsequences {
    QuitConsequences {
        streaming: app.state::<crate::stream::StreamBridgeState>().is_live(),
        recording: app.state::<crate::recording::RecordingState>().is_active()
            || app.state::<crate::audiorec::AudioRecState>().is_active(),
        replay: app.state::<crate::replay::ReplayState>().is_armed(),
    }
}

/// The session marker, beside `settings.json`/`workspace.json`.
fn marker_path() -> Option<PathBuf> {
    directories::ProjectDirs::from("com", "Freally", "Freally Capture")
        .map(|dirs| dirs.config_dir().join("session-running"))
}

/// Create this session's marker; returns whether the previous session's was
/// still there (an unclean exit).
pub fn mark_session_start() -> bool {
    let Some(path) = marker_path() else {
        return false;
    };
    let unclean = path.exists();
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    if let Err(err) = std::fs::write(&path, b"") {
        eprintln!("shutdown: could not write the session marker: {err}");
    }
    unclean
}

/// At `RunEvent::Exit`: remove the marker ONLY when nothing needed
/// finalizing. An exit that left a live recording behind (OS shutdown, a
/// kill) must stay "unclean" so the next launch offers salvage — never mark
/// clean just because the process got to say goodbye.
pub fn mark_clean_if_quiescent<R: Runtime>(app: &AppHandle<R>) {
    if consequences(app).any() {
        return;
    }
    if let Some(path) = marker_path() {
        let _ = std::fs::remove_file(&path);
    }
}

/// The ordered shutdown, then exit. Idempotent; runs off the event-loop
/// thread because every step blocks (child processes, muxer finalize).
pub fn shutdown_and_exit<R: Runtime>(app: AppHandle<R>) {
    if app
        .state::<QuitState>()
        .quitting
        .swap(true, Ordering::SeqCst)
    {
        return; // already on the way down
    }
    std::thread::Builder::new()
        .name("fcap-shutdown".into())
        .spawn(move || {
            // 1. End the stream — a deliberate End Stream, disconnecting the
            //    service cleanly instead of vanishing mid-connection.
            if app.state::<crate::stream::StreamBridgeState>().is_live() {
                if let Err(err) = crate::stream::stop(&app) {
                    eprintln!("shutdown: end stream: {err}");
                }
            }
            // 2. Finalize recordings — blocks until the muxer closes the
            //    file(s), so nothing is left truncated.
            if app.state::<crate::audiorec::AudioRecState>().is_active() {
                if let Err(err) = crate::audiorec::stop(&app) {
                    eprintln!("shutdown: finalize audio recording: {err}");
                }
            }
            if app.state::<crate::recording::RecordingState>().is_active() {
                if let Err(err) = crate::recording::stop(&app) {
                    eprintln!("shutdown: finalize recording: {err}");
                }
            }
            // 3. Flush the replay buffer — stop the encoder child and drop
            //    the ring deliberately.
            if app.state::<crate::replay::ReplayState>().is_armed() {
                if let Err(err) = crate::replay::disarm(&app) {
                    eprintln!("shutdown: disarm replay: {err}");
                }
            }
            // 4. Save state now; `RunEvent::Exit` saves again (harmless) and
            //    removes the marker once quiescent.
            app.state::<crate::studio::StudioState>().save_now();
            app.exit(0);
        })
        .expect("shutdown thread spawns");
}

/// The confirm dialog's "Quit safely" — run the ordered shutdown and exit.
#[tauri::command]
pub fn quit_confirmed<R: Runtime>(app: AppHandle<R>) {
    shutdown_and_exit(app);
}

/// The confirm dialog's Cancel — disarm the prompt so the NEXT close asks
/// again (instead of reading as the hung-webview double-close escape).
#[tauri::command]
pub fn quit_guard_cancel<R: Runtime>(app: AppHandle<R>) {
    app.state::<QuitState>().disarm_prompt();
}
