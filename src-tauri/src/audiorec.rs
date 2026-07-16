//! CAP-N38 audio-only recording ("podcast mode"): the enabled track buses are
//! written to per-track WAV files by the **owned** writer — no video encoder is
//! spun up at all — then optionally transcoded to FLAC/Opus through the labeled
//! ffmpeg component. Long silences are noted in a `.skips.txt` sidecar so an
//! editor can jump the dead air. Captured audio goes only to these local files.

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc::{sync_channel, RecvTimeoutError};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use serde::Serialize;
use tauri::{AppHandle, Manager, Runtime};

use fcap_audio::RecordTap;
use fcap_encode::wav::WavWriter;

use crate::audio::AudioRuntime;
use crate::settings::SettingsStore;

/// Level below which a block counts as silence (~−50 dBFS).
const SILENCE_PEAK: f32 = 0.003;
/// A silence must last this long to earn a skip marker.
const SKIP_MIN_MS: u64 = 2_000;
/// The writer-thread queue depth (10 ms blocks per track). ~1.7 s of slack at
/// six tracks before a disk that can't keep up starts dropping blocks — the
/// engine thread never blocks on the write either way.
const WRITE_QUEUE: usize = 1024;

/// Long-silence tracker for the CAP-N38 skip sidecar.
struct Silence {
    in_silence: bool,
    run_start_ms: u64,
    spans: Vec<(u64, u64)>,
}

impl Silence {
    fn new() -> Self {
        Self {
            in_silence: true,
            run_start_ms: 0,
            spans: Vec::new(),
        }
    }

    fn push(&mut self, peak: f32, pos_ms: u64) {
        let silent = peak < SILENCE_PEAK;
        if silent && !self.in_silence {
            self.in_silence = true;
            self.run_start_ms = pos_ms;
        } else if !silent && self.in_silence {
            self.in_silence = false;
            if pos_ms.saturating_sub(self.run_start_ms) >= SKIP_MIN_MS {
                self.spans.push((self.run_start_ms, pos_ms));
            }
        }
    }
}

/// One block bound for a track's writer (owned copy — the tap can't block the
/// real-time engine thread on file I/O).
type TrackBlock = (usize, Vec<f32>);

struct Session {
    since: Instant,
    display_path: String,
    track_count: u32,
    paused: Arc<AtomicBool>,
    silence: Arc<Mutex<Silence>>,
    format: crate::settings::AudioRecFormat,
    paths: Vec<PathBuf>,
    skips_path: PathBuf,
    /// Signals the writer thread to drain and finalize.
    writer_stop: Arc<AtomicBool>,
    writer_join: Option<JoinHandle<()>>,
}

/// Tauri-managed audio-only recording state.
#[derive(Default)]
pub struct AudioRecState {
    inner: Mutex<Option<Session>>,
    /// Serializes `start` so two concurrent invocations can't both arm the
    /// single shared record tap (the second would clobber the first, leaking
    /// unfinalized WAV headers). Mirrors `recording::RecordingState.starting`.
    starting: AtomicBool,
}

impl AudioRecState {
    fn lock(&self) -> std::sync::MutexGuard<'_, Option<Session>> {
        self.inner
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// Whether an audio-only session is up (the quit guard checks this too).
    pub fn is_active(&self) -> bool {
        self.lock().is_some()
    }
}

/// Clears the `starting` flag when `start` returns on any path.
struct ResetOnDrop<'a>(&'a AtomicBool);
impl Drop for ResetOnDrop<'_> {
    fn drop(&mut self) {
        self.0.store(false, Ordering::SeqCst);
    }
}

/// The audio-only recording status the UI polls.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "state")]
pub enum AudioRecDto {
    Idle,
    Recording {
        duration_sec: f64,
        path: String,
        tracks: u32,
        paused: bool,
    },
}

/// A non-colliding `<stem> track<n>.wav` in `folder`.
fn track_path(folder: &std::path::Path, stem: &str, track: usize) -> PathBuf {
    let base = format!("{stem} track{}", track + 1);
    crate::recording::unique_recording_path(folder, &base, "wav", false)
}

/// The writer thread: owns the WAV writers, drains the bounded queue, and
/// finalizes on stop — so no file write ever runs on the audio engine thread.
fn run_writer(
    rx: std::sync::mpsc::Receiver<TrackBlock>,
    mut writers: Vec<(usize, WavWriter)>,
    stop: Arc<AtomicBool>,
) {
    let write = |writers: &mut Vec<(usize, WavWriter)>, index: usize, samples: &[f32]| {
        if let Some((_, writer)) = writers.iter_mut().find(|(i, _)| *i == index) {
            let _ = writer.write_f32(samples);
        }
    };
    loop {
        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok((index, samples)) => write(&mut writers, index, &samples),
            Err(RecvTimeoutError::Timeout) => {
                if stop.load(Ordering::Relaxed) {
                    break;
                }
            }
            Err(RecvTimeoutError::Disconnected) => break,
        }
    }
    // Drain anything still queued, then finalize every writer's header.
    while let Ok((index, samples)) = rx.try_recv() {
        write(&mut writers, index, &samples);
    }
    for (_, writer) in writers {
        let _ = writer.finalize();
    }
}

pub fn start<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    let state = app.state::<AudioRecState>();
    // Serialize the whole start (slow file/thread I/O before the session is
    // registered): a second concurrent start bails here instead of racing onto
    // the single shared record tap.
    if state.starting.swap(true, Ordering::SeqCst) {
        return Err("an audio recording is already starting".to_owned());
    }
    let _reset = ResetOnDrop(&state.starting);
    if state.lock().is_some() {
        return Err("an audio recording is already running".to_owned());
    }
    // The record tap is a single sink — a video recording owns it if one runs.
    if app.state::<crate::recording::RecordingState>().is_active() {
        return Err("stop the video recording before starting an audio-only one".to_owned());
    }
    let settings = app.state::<SettingsStore>().get().recording;
    settings.validate()?;

    let folder = crate::recording::recordings_folder(&settings);
    std::fs::create_dir_all(&folder)
        .map_err(|err| format!("could not create {}: {err}", folder.display()))?;

    let tracks: Vec<usize> = (0..fcap_scene::TRACK_COUNT)
        .filter(|index| settings.tracks_mask & (1 << index) != 0)
        .collect();
    if tracks.is_empty() {
        return Err("no tracks are enabled to record".to_owned());
    }
    let stem = crate::recording::audio_only_stem(app, &settings);

    let mut writers: Vec<(usize, WavWriter)> = Vec::new();
    let mut paths: Vec<PathBuf> = Vec::new();
    for &track in &tracks {
        let path = track_path(&folder, &stem, track);
        let writer = WavWriter::create(&path, 2, 48_000)
            .map_err(|err| format!("could not create {}: {err}", path.display()))?;
        writers.push((track, writer));
        paths.push(path);
    }

    // The writer thread owns the WAV writers; the tap only hands it owned blocks
    // over a bounded queue, so the engine thread never blocks on a disk write.
    let (tx, rx) = sync_channel::<TrackBlock>(WRITE_QUEUE);
    let writer_stop = Arc::new(AtomicBool::new(false));
    let thread_stop = Arc::clone(&writer_stop);
    let writer_join = std::thread::Builder::new()
        .name("audiorec-writer".to_owned())
        .spawn(move || run_writer(rx, writers, thread_stop))
        .map_err(|err| format!("could not start the recording writer: {err}"))?;

    let paused = Arc::new(AtomicBool::new(false));
    let silence = Arc::new(Mutex::new(Silence::new()));

    let tap_paused = Arc::clone(&paused);
    let tap_blocks = Arc::new(AtomicU64::new(0));
    let tap_silence = Arc::clone(&silence);
    app.state::<AudioRuntime>()
        .engine
        .set_record_tap(Some(RecordTap {
            tracks: settings.tracks_mask,
            sink: Box::new(move |blocks| {
                if tap_paused.load(Ordering::Relaxed) {
                    return;
                }
                let pos = tap_blocks.fetch_add(1, Ordering::Relaxed);
                let pos_ms = pos * 10;
                let mut peak = 0.0f32;
                for (index, block) in blocks {
                    peak = peak.max(block.iter().fold(0.0f32, |acc, s| acc.max(s.abs())));
                    // Non-blocking hand-off; a full queue (disk can't keep up)
                    // drops this block rather than stalling the engine thread.
                    let _ = tx.try_send((*index, block.to_vec()));
                }
                tap_silence
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .push(peak, pos_ms);
            }),
        }));

    let display_path = paths
        .first()
        .map(|p| p.display().to_string())
        .unwrap_or_default();
    // Unique so back-to-back recordings on a static filename template don't
    // overwrite the prior sidecar.
    let skips_path =
        crate::recording::unique_recording_path(&folder, &format!("{stem}.skips"), "txt", false);
    *state.lock() = Some(Session {
        since: Instant::now(),
        display_path,
        track_count: tracks.len() as u32,
        paused,
        silence,
        format: settings.audio_format,
        paths,
        skips_path,
        writer_stop,
        writer_join: Some(writer_join),
    });
    Ok(())
}

pub fn set_paused<R: Runtime>(app: &AppHandle<R>, paused: bool) -> Result<(), String> {
    let state = app.state::<AudioRecState>();
    let guard = state.lock();
    let session = guard.as_ref().ok_or("no audio recording is running")?;
    session.paused.store(paused, Ordering::Relaxed);
    Ok(())
}

pub fn status<R: Runtime>(app: &AppHandle<R>) -> AudioRecDto {
    let state = app.state::<AudioRecState>();
    let guard = state.lock();
    match guard.as_ref() {
        None => AudioRecDto::Idle,
        Some(session) => AudioRecDto::Recording {
            duration_sec: session.since.elapsed().as_secs_f64(),
            path: session.display_path.clone(),
            tracks: session.track_count,
            paused: session.paused.load(Ordering::Relaxed),
        },
    }
}

pub fn stop<R: Runtime>(app: &AppHandle<R>) -> Result<Vec<String>, String> {
    let mut session = app
        .state::<AudioRecState>()
        .lock()
        .take()
        .ok_or("no audio recording is running")?;

    // Detach the tap first so nothing else is queued, then stop + join the
    // writer thread, which drains the queue and finalizes every WAV header.
    app.state::<AudioRuntime>().engine.set_record_tap(None);
    session.writer_stop.store(true, Ordering::Relaxed);
    if let Some(join) = session.writer_join.take() {
        let _ = join.join();
    }

    // Write the skip-marker sidecar (long silences), if any.
    let spans = std::mem::take(
        &mut session
            .silence
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .spans,
    );
    if !spans.is_empty() {
        let mut text = String::from("# CAP-N38 silent-skip markers (start_ms end_ms)\n");
        for (start, end) in &spans {
            text.push_str(&format!("{start} {end}\n"));
        }
        let _ = std::fs::write(&session.skips_path, text);
    }

    // Owned WAV is done. FLAC/Opus needs the labeled ffmpeg component.
    let mut outputs: Vec<String> = Vec::new();
    if let Some(codec) = session.format.codec() {
        let ready = app
            .state::<crate::commands::recording::EncodeState>()
            .ready_ffmpeg();
        match ready {
            Some(ffmpeg) => {
                for path in &session.paths {
                    match fcap_encode::remux::transcode_wav(&ffmpeg, path, codec) {
                        Ok(out) => {
                            let _ = std::fs::remove_file(path); // the WAV was intermediate
                            outputs.push(out.display().to_string());
                        }
                        Err(err) => {
                            eprintln!("audiorec: transcode failed, keeping WAV: {err}");
                            outputs.push(path.display().to_string());
                        }
                    }
                }
            }
            None => {
                // No component: keep the owned WAVs and say so honestly.
                for path in &session.paths {
                    outputs.push(path.display().to_string());
                }
                eprintln!(
                    "audiorec: {codec} needs the ffmpeg component — kept the owned WAV files"
                );
            }
        }
    } else {
        for path in &session.paths {
            outputs.push(path.display().to_string());
        }
    }
    Ok(outputs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn silence_records_long_gaps_only() {
        let mut silence = Silence::new();
        // 1 s of sound, 3 s of silence, then sound again → one skip span.
        for block in 0..500 {
            let peak = if (100..400).contains(&block) {
                0.0
            } else {
                0.5
            };
            silence.push(peak, block * 10);
        }
        assert_eq!(silence.spans.len(), 1, "one long silence recorded");
        assert_eq!(silence.spans[0], (1_000, 4_000));

        // A short gap (0.5 s) earns no marker.
        let mut silence = Silence::new();
        for block in 0..200 {
            let peak = if (100..150).contains(&block) {
                0.0
            } else {
                0.5
            };
            silence.push(peak, block * 10);
        }
        assert!(silence.spans.is_empty(), "short gaps are ignored");
    }
}
