//! Next-launch recording salvage (CAP-M11).
//!
//! A `recording-in-progress.json` sidecar (beside `settings.json`) lists the
//! wire-container outputs of the running session — written at start, removed
//! only after every output finalized. At startup, that sidecar surviving AND
//! the previous session's crash marker still present (CAP-M23) means a
//! recording was interrupted: the salvage prompt offers a tolerant
//! stream-copy repair (`fcap_encode::repair_recording`) into a `(repaired)`
//! sibling. The owned `.frec` format is crash-safe by design and is never
//! listed.

use std::path::{Path, PathBuf};
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Runtime, State};

use crate::commands::recording::EncodeState;

/// One in-flight recording output, tracked from start to clean finalize.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InProgressFile {
    pub path: PathBuf,
    /// Splitting writes `{stem} partNNN.{ext}` siblings, never the base
    /// path itself; only the newest segment can be damaged (earlier ones
    /// finalized on rotation).
    pub split: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct InProgress {
    files: Vec<InProgressFile>,
}

fn sidecar_path() -> Option<PathBuf> {
    directories::ProjectDirs::from("com", "Freally", "Freally Capture")
        .map(|dirs| dirs.config_dir().join("recording-in-progress.json"))
}

/// Record the session's outputs (recording start; wire containers only).
/// MERGES with whatever the sidecar already lists: entries only survive a
/// stop when their finalize failed, so a failover restart must not erase
/// the just-died session's damaged file from the salvage list.
pub fn write_in_progress(files: &[InProgressFile]) {
    let Some(path) = sidecar_path() else { return };
    write_to(&path, files);
}

fn write_to(path: &Path, files: &[InProgressFile]) {
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let mut merged = read_from(path);
    for file in files {
        if !merged.iter().any(|kept| kept.path == file.path) {
            merged.push(file.clone());
        }
    }
    let entry = InProgress { files: merged };
    match serde_json::to_string_pretty(&entry) {
        Ok(json) => {
            if let Err(err) = std::fs::write(path, json) {
                eprintln!("salvage: could not write the in-progress sidecar: {err}");
            }
        }
        Err(err) => eprintln!("salvage: could not serialize the sidecar: {err}"),
    }
}

/// Every output finalized — the session needs no salvage.
pub fn clear_in_progress() {
    if let Some(path) = sidecar_path() {
        let _ = std::fs::remove_file(&path);
    }
}

/// Consume the sidecar (read + delete — stale ones never linger): what the
/// interrupted session was writing.
pub fn take_interrupted() -> Vec<InProgressFile> {
    let Some(path) = sidecar_path() else {
        return Vec::new();
    };
    take_from(&path)
}

fn take_from(path: &Path) -> Vec<InProgressFile> {
    let files = read_from(path);
    let _ = std::fs::remove_file(path);
    files
}

fn read_from(path: &Path) -> Vec<InProgressFile> {
    let Ok(text) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    // Ours, but disk content is never trusted blindly: bounded, then parsed.
    if text.len() > 64 * 1024 {
        return Vec::new();
    }
    serde_json::from_str::<InProgress>(&text)
        .map(|entry| entry.files)
        .unwrap_or_default()
}

/// Resolve the files worth repairing: the file itself, or — when splitting —
/// the newest existing `{stem} partNNN.{ext}` sibling.
pub fn candidates(files: &[InProgressFile]) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for file in files {
        if file.split {
            if let Some(newest) = newest_part(&file.path) {
                out.push(newest);
            }
        } else if file.path.is_file() {
            out.push(file.path.clone());
        }
    }
    out
}

/// The newest `{stem} partNNN.{ext}` sibling of `base` (numeric, not
/// lexicographic — `%03d` grows past three digits on very long sessions).
fn newest_part(base: &Path) -> Option<PathBuf> {
    let stem = base.file_stem()?.to_str()?;
    let ext = base.extension()?.to_str()?;
    let dir = base.parent()?;
    let suffix = format!(".{ext}");
    let mut best: Option<(u64, PathBuf)> = None;
    for entry in std::fs::read_dir(dir).ok()? {
        let Ok(entry) = entry else { continue };
        let name = entry.file_name();
        let Some(name) = name.to_str() else { continue };
        let Some(rest) = name.strip_prefix(stem) else {
            continue;
        };
        let Some(digits) = rest
            .strip_prefix(" part")
            .and_then(|tail| tail.strip_suffix(&suffix))
        else {
            continue;
        };
        let Ok(number) = digits.parse::<u64>() else {
            continue;
        };
        if best.as_ref().map_or(true, |(top, _)| number > *top) {
            best = Some((number, dir.join(name)));
        }
    }
    best.map(|(_, path)| path)
}

/// Managed state: the interrupted outputs found at startup, held for the
/// salvage prompt. Repair only ever runs against THIS list — the webview
/// cannot point it at arbitrary paths.
#[derive(Default)]
pub struct SalvageState(Mutex<Vec<PathBuf>>);

impl SalvageState {
    fn lock(&self) -> std::sync::MutexGuard<'_, Vec<PathBuf>> {
        self.0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    pub fn set_pending(&self, pending: Vec<PathBuf>) {
        *self.lock() = pending;
    }
}

/// The interrupted recordings awaiting a decision (startup prompt).
#[tauri::command]
pub fn salvage_pending(state: State<'_, SalvageState>) -> Vec<String> {
    state
        .lock()
        .iter()
        .map(|path| path.display().to_string())
        .collect()
}

/// Repair one pending file into a `(repaired)` sibling; the original is
/// never touched. Off the UI thread — a long recording copies for minutes.
#[tauri::command]
pub async fn salvage_repair<R: Runtime>(app: AppHandle<R>, path: String) -> Result<String, String> {
    let target = {
        let state = app.state::<SalvageState>();
        let found = state
            .lock()
            .iter()
            .find(|candidate| candidate.display().to_string() == path)
            .cloned();
        found.ok_or("that file is not awaiting salvage")?
    };
    let repaired = tauri::async_runtime::spawn_blocking(move || {
        let ready = app.state::<EncodeState>().ready_ffmpeg().ok_or_else(|| {
            "repair needs the ffmpeg component — install it from Components".to_string()
        })?;
        let repaired = fcap_encode::repair_recording(&ready, &target)?;
        // Done — off the pending list (the original stays on disk untouched).
        app.state::<SalvageState>()
            .lock()
            .retain(|candidate| *candidate != target);
        Ok::<PathBuf, String>(repaired)
    })
    .await
    .map_err(|err| format!("repair task failed: {err}"))??;
    Ok(repaired.display().to_string())
}

/// The user declined — drop the list (the files themselves stay).
#[tauri::command]
pub fn salvage_dismiss(state: State<'_, SalvageState>) {
    state.lock().clear();
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
            std::env::temp_dir().join(format!("fcap-salvage-{}-{nanos}-{tag}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("mkdir");
        dir
    }

    #[test]
    fn sidecar_round_trips_and_is_consumed_once() {
        let dir = temp_dir("roundtrip");
        let sidecar = dir.join("recording-in-progress.json");
        let files = vec![
            InProgressFile {
                path: dir.join("Take 1.mp4"),
                split: false,
            },
            InProgressFile {
                path: dir.join("Take 1 (vertical).mp4"),
                split: false,
            },
        ];
        write_to(&sidecar, &files);
        assert_eq!(take_from(&sidecar), files);
        // Consumed: a second take finds nothing (stale sidecars never linger).
        assert!(take_from(&sidecar).is_empty());
        assert!(!sidecar.exists());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn writes_merge_so_a_failover_restart_keeps_the_dead_sessions_file() {
        let dir = temp_dir("merge");
        let sidecar = dir.join("recording-in-progress.json");
        let damaged = InProgressFile {
            path: dir.join("Take A.mp4"),
            split: false,
        };
        let fresh = InProgressFile {
            path: dir.join("Take B.mp4"),
            split: false,
        };
        // The dead session's entry survives its finalize failure…
        write_to(&sidecar, std::slice::from_ref(&damaged));
        // …and the failover restart's write must ADD, not replace.
        write_to(&sidecar, std::slice::from_ref(&fresh));
        // Re-listing the same path never duplicates it.
        write_to(&sidecar, std::slice::from_ref(&fresh));
        assert_eq!(take_from(&sidecar), vec![damaged, fresh]);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn garbage_or_oversized_sidecars_yield_nothing() {
        let dir = temp_dir("garbage");
        let sidecar = dir.join("recording-in-progress.json");
        std::fs::write(&sidecar, b"not json").expect("write");
        assert!(take_from(&sidecar).is_empty());
        std::fs::write(&sidecar, vec![b'x'; 65 * 1024]).expect("write");
        assert!(take_from(&sidecar).is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn candidates_keep_only_files_that_exist() {
        let dir = temp_dir("exists");
        let real = dir.join("Take 1.mkv");
        std::fs::write(&real, b"x").expect("write");
        let files = vec![
            InProgressFile {
                path: real.clone(),
                split: false,
            },
            InProgressFile {
                path: dir.join("never-created.mkv"),
                split: false,
            },
        ];
        assert_eq!(candidates(&files), vec![real]);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn split_sessions_resolve_to_the_newest_segment_numerically() {
        let dir = temp_dir("split");
        // The base path is never written by a split session — its `partNNN`
        // siblings are. part1000 proves numeric (not lexicographic) order.
        for name in ["Take part000.mp4", "Take part999.mp4", "Take part1000.mp4"] {
            std::fs::write(dir.join(name), b"x").expect("write");
        }
        std::fs::write(dir.join("Take unrelated.mp4"), b"x").expect("write");
        let files = vec![InProgressFile {
            path: dir.join("Take.mp4"),
            split: true,
        }];
        assert_eq!(candidates(&files), vec![dir.join("Take part1000.mp4")]);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
