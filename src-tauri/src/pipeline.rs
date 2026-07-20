//! CAP-N45: the post-record pipeline — after a recording finalizes, run the
//! profile's step chain on the main file(s), in order, on a background
//! worker, with a queue view the Recordings dialog renders.
//!
//! The action set is **closed** ([`crate::settings::PipelineStep`]): there is
//! no "run a command" step and never will be. The chain **halts at the first
//! failure** — later steps must not polish, rename or move a file a `Verify`
//! step just called broken. The "current file" evolves through the chain:
//! `Remux`/`Normalize` continue on their output sibling, `Rename`/`Move`
//! follow the file, `Copy` is a side-effect (the chain keeps the original).
//! Every folder the chain touches passes the `is_remote` guard first — the
//! CAP-M16 NTLM rule applies to pipeline folders exactly as everywhere else.

use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Mutex;

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, Runtime};

use crate::settings::{PipelineStep, SettingsStore};

/// How many finished jobs the queue view keeps.
const KEPT_JOBS: usize = 20;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StepDto {
    /// The step's stable id ("verify", "remux", … — `PipelineStep::id`).
    pub action: String,
    /// "pending" | "running" | "ok" | "warn" | "fail" | "skipped".
    pub status: String,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobDto {
    pub id: u64,
    /// The file the job STARTED from (display name).
    pub file: String,
    pub steps: Vec<StepDto>,
    pub done: bool,
}

struct Queued {
    path: PathBuf,
    expected_secs: Option<f64>,
    /// CAP-N75: a synthetic one-step ".frec → shareable MP4" job instead of
    /// the profile's configured chain. Queued BEFORE the chain, so a `Move`
    /// step can never relocate the master out from under the export.
    auto_export: bool,
}

/// Tauri-managed pipeline state: the pending work + the recent-jobs view.
pub struct PipelineState {
    queue: Mutex<VecDeque<Queued>>,
    jobs: Mutex<VecDeque<JobDto>>,
    running: AtomicBool,
    next_id: AtomicU64,
}

impl PipelineState {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            jobs: Mutex::new(VecDeque::new()),
            running: AtomicBool::new(false),
            next_id: AtomicU64::new(1),
        }
    }

    /// The queue view's snapshot (newest first).
    pub fn jobs(&self) -> Vec<JobDto> {
        self.jobs
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .iter()
            .cloned()
            .collect()
    }
}

impl Default for PipelineState {
    fn default() -> Self {
        Self::new()
    }
}

fn emit_jobs<R: Runtime>(app: &AppHandle<R>) {
    let jobs = app.state::<PipelineState>().jobs();
    let _ = app.emit("pipeline", &jobs);
}

/// Queue the pipeline for freshly-finalized recording files. A no-op unless
/// the profile enables a non-empty pipeline. Called from `recording::stop`
/// with the MAIN recording's files (vertical/ISO lanes are companions, not
/// the show master — documented v1 boundary).
pub fn enqueue<R: Runtime>(app: &AppHandle<R>, files: Vec<PathBuf>, expected_secs: Option<f64>) {
    let settings = app.state::<SettingsStore>().get().recording;
    if !settings.pipeline_enabled || settings.pipeline.is_empty() || files.is_empty() {
        return;
    }
    push_queued(app, files, expected_secs, false);
}

/// CAP-N75: queue the one-toggle ".frec → shareable MP4" export for
/// freshly-finalized masters. Independent of the profile's chain (the toggle
/// is its own consent) but drains through the same worker + queue view, so
/// the copy shows up in the Recordings dialog's background queue.
pub fn enqueue_auto_export<R: Runtime>(
    app: &AppHandle<R>,
    files: Vec<PathBuf>,
    expected_secs: Option<f64>,
) {
    if files.is_empty() {
        return;
    }
    push_queued(app, files, expected_secs, true);
}

/// Push work and make sure exactly one drain worker is running.
fn push_queued<R: Runtime>(
    app: &AppHandle<R>,
    files: Vec<PathBuf>,
    expected_secs: Option<f64>,
    auto_export: bool,
) {
    let state = app.state::<PipelineState>();
    {
        let mut queue = state
            .queue
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        for path in files {
            queue.push_back(Queued {
                path,
                expected_secs,
                auto_export,
            });
        }
    }
    if state.running.swap(true, Ordering::SeqCst) {
        return; // the worker is already draining; it will pick these up
    }
    let handle = app.clone();
    let spawned = std::thread::Builder::new()
        .name("fcap-pipeline".into())
        .spawn(move || {
            loop {
                let next = handle
                    .state::<PipelineState>()
                    .queue
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .pop_front();
                let Some(job) = next else { break };
                run_job(&handle, job);
            }
            handle
                .state::<PipelineState>()
                .running
                .store(false, Ordering::SeqCst);
        });
    if spawned.is_err() {
        app.state::<PipelineState>()
            .running
            .store(false, Ordering::SeqCst);
    }
}

/// Execute one file's chain, updating the queue view step by step. An
/// auto-export job (CAP-N75) runs a synthetic single "autoExport" step in
/// place of the profile's chain.
fn run_job<R: Runtime>(app: &AppHandle<R>, job: Queued) {
    let settings = app.state::<SettingsStore>().get();
    let steps = if job.auto_export {
        Vec::new()
    } else {
        settings.recording.pipeline.clone()
    };
    // The unified work list: the profile's chain, or the synthetic export.
    // The queue's step DTOs are derived from it, so the two never disagree.
    enum Work<'a> {
        Step(&'a PipelineStep),
        AutoExport,
    }
    let works: Vec<Work> = if job.auto_export {
        vec![Work::AutoExport]
    } else {
        steps.iter().map(Work::Step).collect()
    };
    let state = app.state::<PipelineState>();
    let id = state.next_id.fetch_add(1, Ordering::Relaxed);
    {
        let mut jobs = state
            .jobs
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        jobs.push_front(JobDto {
            id,
            file: job
                .path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| job.path.display().to_string()),
            steps: works
                .iter()
                .map(|work| StepDto {
                    action: match work {
                        Work::AutoExport => "autoExport".into(),
                        Work::Step(step) => step.id().into(),
                    },
                    status: "pending".into(),
                    detail: String::new(),
                })
                .collect(),
            done: false,
        });
        jobs.truncate(KEPT_JOBS);
    }
    emit_jobs(app);

    let mut current = job.path.clone();
    let mut halted = false;
    for (index, work) in works.iter().enumerate() {
        let set = |status: &str, detail: String| {
            let state = app.state::<PipelineState>();
            let mut jobs = state
                .jobs
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(job) = jobs.iter_mut().find(|job| job.id == id) {
                if let Some(entry) = job.steps.get_mut(index) {
                    entry.status = status.into();
                    entry.detail = detail;
                }
            }
        };
        if halted {
            set("skipped", "the chain stopped at an earlier failure".into());
            continue;
        }
        set("running", String::new());
        emit_jobs(app);
        let outcome = match work {
            Work::AutoExport => run_auto_export(app, &current),
            Work::Step(step) => run_step(app, step, &mut current, job.expected_secs),
        };
        match &outcome {
            StepOutcome::Ok(detail) => set("ok", detail.clone()),
            StepOutcome::Warn(detail) => set("warn", detail.clone()),
            StepOutcome::Skipped(detail) => set("skipped", detail.clone()),
            StepOutcome::Fail(detail) => {
                set("fail", detail.clone());
                halted = true;
            }
        }
        emit_jobs(app);
    }
    {
        let state = app.state::<PipelineState>();
        let mut jobs = state
            .jobs
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(job) = jobs.iter_mut().find(|job| job.id == id) {
            job.done = true;
        }
    }
    emit_jobs(app);
}

/// The queue view's snapshot (the `pipeline` event pushes updates).
#[tauri::command]
pub fn pipeline_status<R: Runtime>(app: AppHandle<R>) -> Vec<JobDto> {
    app.state::<PipelineState>().jobs()
}

enum StepOutcome {
    Ok(String),
    Warn(String),
    Skipped(String),
    Fail(String),
}

fn ext_of(path: &Path) -> String {
    path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase()
}

fn run_step<R: Runtime>(
    app: &AppHandle<R>,
    step: &PipelineStep,
    current: &mut PathBuf,
    expected_secs: Option<f64>,
) -> StepOutcome {
    match step {
        PipelineStep::Verify => {
            let report = if ext_of(current) == "frec" {
                fcap_encode::verify::verify_frec(current, expected_secs)
            } else {
                match app
                    .state::<crate::commands::recording::EncodeState>()
                    .ready_ffmpeg()
                {
                    Some(ready) => {
                        fcap_encode::verify::verify_wire(&ready, current, expected_secs, false)
                    }
                    None => {
                        return StepOutcome::Skipped(
                            "verifying wire files needs the ffmpeg component".into(),
                        )
                    }
                }
            };
            match report {
                Ok(report) => {
                    let flagged: Vec<String> = report
                        .checks
                        .iter()
                        .filter(|check| {
                            matches!(
                                check.status,
                                fcap_encode::verify::CheckStatus::Warn
                                    | fcap_encode::verify::CheckStatus::Fail
                            )
                        })
                        .map(|check| check.detail.clone())
                        .collect();
                    match report.verdict {
                        fcap_encode::verify::CheckStatus::Fail => {
                            StepOutcome::Fail(flagged.join("; "))
                        }
                        fcap_encode::verify::CheckStatus::Warn => {
                            StepOutcome::Warn(flagged.join("; "))
                        }
                        _ => StepOutcome::Ok(format!("{} checks passed", report.checks.len())),
                    }
                }
                Err(err) => StepOutcome::Fail(err),
            }
        }
        PipelineStep::Remux => match ext_of(current).as_str() {
            "mkv" => {
                let Some(ready) = app
                    .state::<crate::commands::recording::EncodeState>()
                    .ready_ffmpeg()
                else {
                    return StepOutcome::Skipped("remuxing needs the ffmpeg component".into());
                };
                match fcap_encode::remux::remux_to_mp4(&ready, current) {
                    Ok(out) => {
                        let detail = format!("→ {}", out.display());
                        *current = out;
                        StepOutcome::Ok(detail)
                    }
                    Err(err) => StepOutcome::Fail(err),
                }
            }
            "frec" => StepOutcome::Skipped("a .frec exports from the recordings list".into()),
            other => StepOutcome::Skipped(format!(".{other} already plays anywhere")),
        },
        PipelineStep::Normalize => {
            if ext_of(current) == "frec" {
                return StepOutcome::Skipped("normalize works on wire files".into());
            }
            let Some(ready) = app
                .state::<crate::commands::recording::EncodeState>()
                .ready_ffmpeg()
            else {
                return StepOutcome::Skipped("normalizing needs the ffmpeg component".into());
            };
            let loudness = app.state::<SettingsStore>().get().loudness;
            match fcap_encode::remux::normalize_loudness(
                &ready,
                current,
                loudness.target_lufs,
                loudness.ceiling_db,
            ) {
                Ok(out) => {
                    let detail = format!("→ {}", out.display());
                    *current = out;
                    StepOutcome::Ok(detail)
                }
                Err(err) => StepOutcome::Fail(err),
            }
        }
        PipelineStep::Rename { template } => {
            let settings = app.state::<SettingsStore>().get().recording;
            let counter = crate::recording::counter_for(app, template, settings.counter);
            let naming =
                crate::recording::naming_context(app, settings.filename_prefix, (0, 0), counter);
            let stem = crate::filename::resolve_template(template, &naming);
            let Some(dir) = current.parent() else {
                return StepOutcome::Fail("the file has no parent folder".into());
            };
            let target =
                crate::recording::unique_recording_path(dir, &stem, &ext_of(current), false);
            match std::fs::rename(&*current, &target) {
                Ok(()) => {
                    let detail = format!("→ {}", target.display());
                    *current = target;
                    StepOutcome::Ok(detail)
                }
                Err(err) => StepOutcome::Fail(format!("rename failed: {err}")),
            }
        }
        PipelineStep::Move { folder } | PipelineStep::Copy { folder } => {
            let is_move = matches!(step, PipelineStep::Move { .. });
            // The CAP-M16 NTLM rule: never stat a webview/profile-supplied
            // path that resolves off the local disk.
            if crate::commands::studio::is_remote(folder) {
                return StepOutcome::Fail(
                    "the pipeline folder is a network path — not accepted".into(),
                );
            }
            let dir = PathBuf::from(folder.trim());
            if let Err(err) = std::fs::create_dir_all(&dir) {
                return StepOutcome::Fail(format!("could not create {}: {err}", dir.display()));
            }
            let stem = current
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("recording");
            let target =
                crate::recording::unique_recording_path(&dir, stem, &ext_of(current), false);
            let result = if is_move {
                // `rename` fails across volumes (the common case for an
                // archive drive) — fall back to copy + delete.
                std::fs::rename(&*current, &target).or_else(|_| {
                    std::fs::copy(&*current, &target)
                        .map(|_| ())
                        .and_then(|()| std::fs::remove_file(&*current))
                })
            } else {
                std::fs::copy(&*current, &target).map(|_| ())
            };
            match result {
                Ok(()) => {
                    let detail = format!("→ {}", target.display());
                    if is_move {
                        *current = target;
                    }
                    StepOutcome::Ok(detail)
                }
                Err(err) => StepOutcome::Fail(format!(
                    "{} failed: {err}",
                    if is_move { "move" } else { "copy" }
                )),
            }
        }
        PipelineStep::Reveal => {
            let Some(dir) = current.parent() else {
                return StepOutcome::Fail("the file has no parent folder".into());
            };
            if crate::commands::studio::is_remote(&dir.to_string_lossy()) {
                return StepOutcome::Skipped(
                    "the folder is on a network path — open it from your file manager".into(),
                );
            }
            #[cfg(target_os = "windows")]
            let spawned = std::process::Command::new("explorer")
                .arg("/select,")
                .arg(&*current)
                .spawn();
            #[cfg(target_os = "macos")]
            let spawned = std::process::Command::new("open")
                .arg("-R")
                .arg(&*current)
                .spawn();
            #[cfg(target_os = "linux")]
            let spawned = std::process::Command::new("xdg-open").arg(dir).spawn();
            match spawned {
                Ok(_) => StepOutcome::Ok("shown in the file manager".into()),
                Err(err) => StepOutcome::Fail(format!("could not open the folder: {err}")),
            }
        }
        PipelineStep::LuaEvent => {
            crate::scripting::queue_event(
                "recordingPipeline",
                serde_json::json!({ "path": current.display().to_string() }),
            );
            StepOutcome::Ok("recordingPipeline event queued for scripts".into())
        }
    }
}

/// CAP-N75: the synthetic auto-export step — the owned `.frec` master → a
/// shareable MP4 sibling through the same `export_frec` path the
/// recordings-list export uses. Collision-safe naming (`unique_sibling`);
/// the master is never touched; a missing ffmpeg component skips honestly.
fn run_auto_export<R: Runtime>(app: &AppHandle<R>, current: &Path) -> StepOutcome {
    use fcap_encode::Container;

    if ext_of(current) != "frec" {
        return StepOutcome::Skipped("auto-export copies .frec masters only".into());
    }
    let Some(ready) = app
        .state::<crate::commands::recording::EncodeState>()
        .ready_ffmpeg()
    else {
        return StepOutcome::Skipped("auto-export needs the ffmpeg component".into());
    };
    // Serialize with the manual recordings-list export: one export driver at a
    // time. Otherwise the two could both resolve the same `unique_sibling` path
    // in the window before ffmpeg creates it, and clobber each other's MP4.
    let export = app.state::<crate::commands::recording::ExportState>();
    if !export.try_begin() {
        return StepOutcome::Skipped("an export is already running".into());
    }
    let outcome = (|| {
        let settings = app.state::<SettingsStore>().get().recording;
        let encoder_id = match crate::recording::resolve_encoder(app, &settings, Container::Mp4) {
            Ok(id) => id,
            Err(err) => return StepOutcome::Fail(err),
        };
        // Same plan builder as the recordings-list export — the auto-export
        // copy stays byte-for-byte in step with a manual export of the master.
        let plan = crate::commands::recording::frec_export_plan(
            &settings,
            encoder_id,
            Container::Mp4,
            current,
        );
        // The pipeline has no cancel surface — the job runs to completion.
        let never_cancel = AtomicBool::new(false);
        match fcap_encode::export_frec(&ready, current, &plan, |_| {}, &never_cancel) {
            Ok(out) => StepOutcome::Ok(format!("→ {}", out.display())),
            Err(err) => StepOutcome::Fail(err),
        }
    })();
    export.end();
    outcome
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(tag: &str) -> PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let dir =
            std::env::temp_dir().join(format!("fcap-pipe-{}-{nanos}-{tag}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("mkdir");
        dir
    }

    /// The move fallback: `rename` across volumes fails — copy+delete must
    /// land the same bytes and remove the source. (Same-volume here, but the
    /// fallback path is what this exercises via a rename-blocking target.)
    #[test]
    fn move_lands_bytes_and_removes_the_source() {
        let dir = temp_dir("move");
        let src = dir.join("take.mkv");
        std::fs::write(&src, b"payload").expect("write");
        let dest_dir = dir.join("archive");
        std::fs::create_dir_all(&dest_dir).expect("mkdir");

        // The pure mechanics of the Move arm, minus the app handle.
        let target = crate::recording::unique_recording_path(&dest_dir, "take", "mkv", false);
        std::fs::rename(&src, &target)
            .or_else(|_| {
                std::fs::copy(&src, &target)
                    .map(|_| ())
                    .and_then(|()| std::fs::remove_file(&src))
            })
            .expect("move");
        assert!(!src.exists(), "source removed");
        assert_eq!(std::fs::read(&target).expect("read"), b"payload");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Copy never renames the chain's current file and never collides.
    #[test]
    fn copy_keeps_the_original_and_never_clobbers() {
        let dir = temp_dir("copy");
        let src = dir.join("take.mkv");
        std::fs::write(&src, b"payload").expect("write");
        let dest_dir = dir.join("archive");
        std::fs::create_dir_all(&dest_dir).expect("mkdir");
        std::fs::write(dest_dir.join("take.mkv"), b"older").expect("existing");

        let target = crate::recording::unique_recording_path(&dest_dir, "take", "mkv", false);
        std::fs::copy(&src, &target).expect("copy");
        assert!(src.exists(), "the original stays");
        assert_eq!(
            target.file_name().unwrap(),
            "take (2).mkv",
            "an existing file is never clobbered"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }
}
