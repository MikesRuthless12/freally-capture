//! CAP-N17 — the **media playlist source**: an ordered, trimmed, gapless
//! media playlist with cue points, loop/shuffle/hold-last, next/previous
//! controls, and a "now playing" name the studio can bind to a variable.
//!
//! **Why it's gapless:** the whole trimmed list plays through ONE decoder
//! process using ffmpeg's concat demuxer (per-item `inpoint`/`outpoint`
//! carry the trims), so item boundaries are frame-exact — no respawn gap.
//! Manual jumps (next/previous/cue/scrub) respawn with an input seek, like
//! the Media source's scrubber.
//!
//! Honest scope notes:
//! - Wire formats only (mp4/mkv/webm/mov/mp3/…), through the labeled
//!   on-demand ffmpeg component — the same rule as the Media source.
//!   `.frec` and still images play through Media/Slideshow instead.
//! - A playlist is either video items or audio-only items; a mix errors
//!   readably at start (the concat demuxer needs one stream shape).
//!   Audio-only playlists paint a small "now playing" face of the current
//!   track's name (background music hides it — pure sound). The name is also
//!   published on a variable any Text source can show.
//! - Shuffle draws one order per session start (a looping shuffle repeats
//!   that order — said in the picker, not hidden).
//! - Mixed resolutions normalize to the first video item's geometry
//!   (fit + pad), so the pipe stays one frame size.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use fcap_capture::{frame_channel, CaptureError, CaptureSession, Frame, FrameSender, PixelFormat};
use fcap_encode::decode::{self, ConcatItem};

use crate::media::{
    clamp_seek, f32_samples_into, media_seek, media_transport, pause_flag, publish_transport,
    read_available, read_exact_or_end, seek_pending, spawn_kill_watchdog, take_seek,
};
use crate::text::{render_text, TextAlign, TextStyle};

/// The audio-only lane's "now playing" face geometry + poll cadence.
const AUDIO_FACE_W: u32 = 640;
const AUDIO_FACE_H: u32 = 160;
const AUDIO_FACE_POLL: Duration = Duration::from_millis(500);

/// One playlist entry as configured (trims in seconds; 0 = edge).
#[derive(Debug, Clone)]
pub struct PlaylistItemSpec {
    pub path: String,
    pub in_s: f32,
    pub out_s: f32,
}

#[derive(Debug, Clone)]
pub struct PlaylistConfig {
    pub items: Vec<PlaylistItemSpec>,
    pub looping: bool,
    pub shuffle: bool,
    pub hold_last: bool,
    pub hw_decode: bool,
    /// Audio-only lane: suppress the on-canvas track-list face (background
    /// music). Ignored for video playlists.
    pub hidden_face: bool,
}

/// The published timeline: played order, names, cumulative starts.
pub struct PlaylistTable {
    /// Played position → original item index (identity unless shuffled).
    pub order: Vec<usize>,
    /// Item display names, in PLAYED order.
    pub names: Vec<String>,
    /// Cumulative start offset of each played item, seconds.
    pub starts: Vec<f32>,
    /// In-trim of each played item, seconds (cue math needs it).
    pub ins: Vec<f32>,
    pub total: f32,
}

impl PlaylistTable {
    /// The played index at `position` seconds.
    pub fn index_at(&self, position: f32) -> usize {
        let wrapped = if self.total > 0.0 {
            position.rem_euclid(self.total)
        } else {
            position
        };
        self.starts
            .iter()
            .rposition(|start| *start <= wrapped + 0.001)
            .unwrap_or(0)
    }
}

static REGISTRY: crate::registry::WeakRegistry<PlaylistTable> =
    crate::registry::WeakRegistry::new();

fn table(id: &str) -> Option<Arc<PlaylistTable>> {
    REGISTRY.get(id)
}

/// Manual transport: jump between items.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaylistAction {
    Next,
    Previous,
}

/// Where a next/previous jump lands from `position` (music-player rules:
/// "previous" restarts the current item unless it just started).
pub fn jump_target(table: &PlaylistTable, position: f32, action: PlaylistAction) -> f32 {
    let index = table.index_at(position);
    match action {
        PlaylistAction::Next => {
            if index + 1 < table.starts.len() {
                table.starts[index + 1]
            } else {
                0.0 // past the last item — wrap to the top
            }
        }
        PlaylistAction::Previous => {
            let wrapped = if table.total > 0.0 {
                position.rem_euclid(table.total)
            } else {
                position
            };
            if wrapped - table.starts[index] > 2.0 || index == 0 {
                table.starts[index]
            } else {
                table.starts[index - 1]
            }
        }
    }
}

/// Drive one playlist (the properties dialog). `false` = not running.
pub fn control(id: &str, action: PlaylistAction) -> bool {
    let Some(table) = table(id) else {
        return false;
    };
    let (position, _) = media_transport(id);
    media_seek(id, jump_target(&table, position, action));
    true
}

/// Drive EVERY live playlist (the global hotkeys).
pub fn control_all(action: PlaylistAction) {
    for id in REGISTRY.live_ids() {
        control(&id, action);
    }
}

/// Seek to a cue: `cue_s` seconds into ORIGINAL item `item_index`'s file.
/// `false` = not running / unknown item.
pub fn cue(id: &str, item_index: usize, cue_s: f32) -> bool {
    let Some(table) = table(id) else {
        return false;
    };
    let Some(played) = table
        .order
        .iter()
        .position(|original| *original == item_index)
    else {
        return false;
    };
    let offset = (cue_s - table.ins[played]).max(0.0);
    media_seek(id, table.starts[played] + offset);
    true
}

/// The playing item's display name right now (`None` = not running).
pub fn now_playing(id: &str) -> Option<String> {
    let table = table(id)?;
    let (position, _) = media_transport(id);
    table.names.get(table.index_at(position)).cloned()
}

/// A file's display name: the stem, or the whole name when it has none.
fn display_name(path: &str) -> String {
    std::path::Path::new(path)
        .file_stem()
        .or_else(|| std::path::Path::new(path).file_name())
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string())
}

/// The slideshow's tiny LCG, reused for the shuffle draw (no rand dep).
fn shuffled_order(count: usize, seed: u64) -> Vec<usize> {
    let mut order: Vec<usize> = (0..count).collect();
    crate::slideshow::shuffled(&mut order, &mut crate::slideshow::Lcg(seed | 1));
    order
}

/// Start the playlist session.
pub fn start_playlist(
    hub_id: &str,
    config: PlaylistConfig,
) -> Result<CaptureSession, CaptureError> {
    let Some(ffmpeg) = fcap_encode::ffmpeg::installed() else {
        return Err(CaptureError::Backend(
            "playlists need the ffmpeg component — install it from Components".into(),
        ));
    };
    if config.items.is_empty() {
        return Err(CaptureError::Backend("the playlist is empty".into()));
    }

    // Probe every item up front: geometry lane, durations, audio presence.
    let mut infos = Vec::with_capacity(config.items.len());
    for item in &config.items {
        let info = decode::probe_media_any(&ffmpeg, std::path::Path::new(&item.path))
            .map_err(CaptureError::Backend)?;
        infos.push(info);
    }
    let video_items = infos.iter().filter(|info| info.width > 0).count();
    if video_items != 0 && video_items != infos.len() {
        return Err(CaptureError::Backend(
            "the playlist mixes video and audio-only files — split them into two playlists".into(),
        ));
    }
    let video_lane = video_items > 0;
    let has_audio = infos.first().map(|info| info.has_audio).unwrap_or(false);

    // Effective trimmed duration per item (the table's spine).
    let mut durations = Vec::with_capacity(config.items.len());
    for (item, info) in config.items.iter().zip(&infos) {
        let Some(full) = info.duration_secs else {
            return Err(CaptureError::Backend(format!(
                "could not read the duration of {} — playlists need seekable files",
                item.path
            )));
        };
        let end = if item.out_s > 0.0 {
            item.out_s.min(full)
        } else {
            full
        };
        let trimmed = end - item.in_s.max(0.0);
        if trimmed <= 0.05 {
            return Err(CaptureError::Backend(format!(
                "the trims on {} leave nothing to play",
                item.path
            )));
        }
        durations.push(trimmed);
    }

    // The played order: identity, or one shuffle draw per session start.
    let order = if config.shuffle && config.items.len() > 1 {
        shuffled_order(
            config.items.len(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|since| since.as_nanos() as u64)
                .unwrap_or(1),
        )
    } else {
        (0..config.items.len()).collect()
    };

    let mut starts = Vec::with_capacity(order.len());
    let mut names = Vec::with_capacity(order.len());
    let mut ins = Vec::with_capacity(order.len());
    let mut concat_items = Vec::with_capacity(order.len());
    let mut total = 0.0f32;
    for original in &order {
        let item = &config.items[*original];
        starts.push(total);
        names.push(display_name(&item.path));
        ins.push(item.in_s.max(0.0));
        total += durations[*original];
        concat_items.push(ConcatItem {
            path: std::path::PathBuf::from(&item.path),
            inpoint: item.in_s.max(0.0),
            outpoint: item.out_s.max(0.0),
        });
    }
    let table = Arc::new(PlaylistTable {
        order,
        names,
        starts,
        ins,
        total,
    });

    // The concat script lives in a per-process, per-source workdir.
    let workdir =
        std::env::temp_dir().join(format!("fcap-playlist-{}-{}", std::process::id(), hub_id));
    let script =
        decode::write_concat_script(&workdir, &concat_items).map_err(CaptureError::Backend)?;

    // Geometry: the first VIDEO item's, every other item fits/pads into it.
    let (width, height) = if video_lane {
        let first = infos
            .iter()
            .find(|info| info.width > 0)
            .expect("video lane");
        (first.width, first.height)
    } else {
        // The audio-only "now playing" face (background music renders it
        // transparent, but the buffer size is the same).
        (AUDIO_FACE_W, AUDIO_FACE_H)
    };
    // The transport clock is `frames / fps`, so the decoder normalizes the
    // whole pipe to ONE rate — the fastest item's (slower items duplicate
    // frames, which is invisible). Anchoring on the first item's native
    // rate drifted the clock across mixed-fps (and shuffled) playlists.
    let probed_max = infos
        .iter()
        .filter_map(|info| info.fps)
        .fold(0.0_f32, f32::max);
    let fps = if probed_max > 0.0 { probed_max } else { 30.0 };

    REGISTRY.insert(hub_id, &table);

    let (sender, receiver) = frame_channel();
    let stop = Arc::new(AtomicBool::new(false));
    let thread_stop = Arc::clone(&stop);
    let hub_id = hub_id.to_string();
    let config = PlaylistRun {
        ffmpeg,
        script,
        table,
        width,
        height,
        fps,
        has_audio,
        video_lane,
        looping: config.looping,
        hold_last: config.hold_last,
        hw_decode: config.hw_decode,
        hidden_face: config.hidden_face,
    };
    let join = std::thread::Builder::new()
        .name("fcap-playlist".into())
        .spawn(move || run_playlist(config, hub_id, sender, thread_stop))
        .map_err(|err| CaptureError::Backend(err.to_string()))?;
    Ok(CaptureSession::from_parts(receiver, stop, join))
}

struct PlaylistRun {
    ffmpeg: fcap_encode::ffmpeg::Ffmpeg,
    script: std::path::PathBuf,
    table: Arc<PlaylistTable>,
    width: u32,
    height: u32,
    fps: f32,
    has_audio: bool,
    video_lane: bool,
    looping: bool,
    hold_last: bool,
    hw_decode: bool,
    hidden_face: bool,
}

enum StretchEnd {
    Seek(f32),
    Ended,
    Stopped,
}

fn run_playlist(run: PlaylistRun, hub_id: String, sender: FrameSender, stop: Arc<AtomicBool>) {
    let duration = run.table.total;
    let mut seek_base = 0.0f32;
    'session: loop {
        let video = if run.video_lane {
            match decode::spawn_concat_video_decoder(
                &run.ffmpeg,
                &run.script,
                run.width,
                run.height,
                run.fps,
                run.looping,
                run.hw_decode,
                seek_base,
            ) {
                Ok(child) => Some(child),
                Err(err) => {
                    sender.close(Some(CaptureError::Backend(err)));
                    return;
                }
            }
        } else {
            None
        };
        let audio = if run.has_audio {
            match decode::spawn_concat_audio_decoder(
                &run.ffmpeg,
                &run.script,
                run.looping,
                seek_base,
            ) {
                Ok(child) => Some(child),
                Err(err) => {
                    sender.close(Some(CaptureError::Backend(err)));
                    return;
                }
            }
        } else {
            None
        };
        match run_stretch(
            &run, video, audio, duration, seek_base, &hub_id, &sender, &stop,
        ) {
            StretchEnd::Seek(target) => {
                seek_base = clamp_seek(target, duration);
                publish_transport(&hub_id, seek_base, duration);
                continue 'session;
            }
            StretchEnd::Ended => {
                if run.looping {
                    // `-stream_loop -1` never EOFs on its own.
                    sender.close(Some(CaptureError::Backend(
                        "the playlist decoder stopped unexpectedly".into(),
                    )));
                    return;
                }
                if !run.hold_last {
                    // End-of-list without hold: clear to transparency.
                    sender.send(Frame {
                        width: run.width,
                        height: run.height,
                        stride: run.width * 4,
                        format: PixelFormat::Rgba8,
                        data: vec![0u8; (run.width * run.height * 4) as usize],
                        captured_at: Instant::now(),
                    });
                }
                loop {
                    if stop.load(Ordering::Relaxed) || !sender.is_open() {
                        sender.close(None);
                        return;
                    }
                    if let Some(target) = take_seek(&hub_id) {
                        seek_base = clamp_seek(target, duration);
                        publish_transport(&hub_id, seek_base, duration);
                        continue 'session;
                    }
                    std::thread::sleep(Duration::from_millis(50));
                }
            }
            StretchEnd::Stopped => {
                sender.close(None);
                return;
            }
        }
    }
}

/// One stretch: pump the pipes until stop/seek/EOF. The video lane sends
/// decoded frames; the audio-only lane paints the current track's name (or
/// one transparent frame when the face is hidden for background music).
#[allow(clippy::too_many_arguments)]
fn run_stretch(
    run: &PlaylistRun,
    video: Option<std::process::Child>,
    audio: Option<std::process::Child>,
    duration: f32,
    seek_base: f32,
    hub_id: &str,
    sender: &FrameSender,
    stop: &Arc<AtomicBool>,
) -> StretchEnd {
    let mut video = video;
    let mut audio = audio;
    let video_stdout = video.as_mut().and_then(|child| child.stdout.take());
    let audio_stdout = audio.as_mut().and_then(|child| child.stdout.take());

    // Stop watchdog: kills the children so pipe reads always unblock.
    let children: Vec<std::process::Child> = video.into_iter().chain(audio).collect();
    let (kill_tx, watchdog) = spawn_kill_watchdog("fcap-playlist-watchdog", stop, children);

    let pause = pause_flag(hub_id);
    // Audio-lane position clock: samples pushed → seconds (the audio-only
    // lane has no frame clock to count).
    let audio_seconds = Arc::new(Mutex::new(0.0f32));
    let audio_thread = audio_stdout.and_then(|mut stdout| {
        let ring = fcap_audio::media_hub::ring(hub_id);
        ring.clear();
        let audio_stop = Arc::clone(stop);
        let audio_pause = Arc::clone(&pause);
        let clock = Arc::clone(&audio_seconds);
        std::thread::Builder::new()
            .name("fcap-playlist-audio".into())
            .spawn(move || {
                let mut bytes = [0u8; 3840];
                let mut samples = Vec::with_capacity(960);
                let mut pushed_frames: u64 = 0;
                loop {
                    while audio_pause.load(Ordering::Relaxed) && !audio_stop.load(Ordering::Relaxed)
                    {
                        std::thread::sleep(Duration::from_millis(30));
                    }
                    let (filled, done) = read_available(&mut stdout, &mut bytes);
                    f32_samples_into(&bytes[..filled], &mut samples);
                    if !samples.is_empty() {
                        ring.push(&samples);
                        pushed_frames += (samples.len() / 2) as u64;
                        *clock
                            .lock()
                            .unwrap_or_else(std::sync::PoisonError::into_inner) =
                            pushed_frames as f32 / 48_000.0;
                    }
                    if done || audio_stop.load(Ordering::Relaxed) {
                        break;
                    }
                }
            })
            .ok()
    });

    let mut end = if let Some(mut stdout) = video_stdout {
        // Video lane: exact frames off the pipe, position from frame count.
        let frame_bytes = run.width as usize * run.height as usize * 4;
        let mut data = vec![0u8; frame_bytes];
        let mut frames: u64 = 0;
        let mut force_frame = true;
        loop {
            if stop.load(Ordering::Relaxed) || !sender.is_open() {
                break StretchEnd::Stopped;
            }
            if let Some(target) = take_seek(hub_id) {
                break StretchEnd::Seek(target);
            }
            while !force_frame
                && pause.load(Ordering::Relaxed)
                && !stop.load(Ordering::Relaxed)
                && sender.is_open()
                && !seek_pending(hub_id)
            {
                std::thread::sleep(Duration::from_millis(30));
            }
            if stop.load(Ordering::Relaxed) || !sender.is_open() {
                break StretchEnd::Stopped;
            }
            if pause.load(Ordering::Relaxed) && !force_frame {
                continue;
            }
            if !read_exact_or_end(&mut stdout, &mut data) {
                break StretchEnd::Ended;
            }
            sender.send(Frame {
                width: run.width,
                height: run.height,
                stride: run.width * 4,
                format: PixelFormat::Rgba8,
                data: data.clone(),
                captured_at: Instant::now(),
            });
            force_frame = false;
            frames += 1;
            let raw = seek_base + frames as f32 / run.fps;
            let position = if duration > 0.0 { raw % duration } else { raw };
            publish_transport(hub_id, position, duration);
        }
    } else {
        // Audio-only lane: the position clock is the pushed sample count.
        // Unless it is background music (`hidden_face`), the source paints a
        // "now playing" face of the CURRENT track's name — repainted only
        // when the playing index changes. Background music instead emits one
        // transparent frame (an invisible source) while its audio flows from
        // the decode ring; the current track is still on the transport for a
        // "now playing" Text-source variable either way.
        if run.hidden_face {
            sender.send(Frame {
                width: run.width,
                height: run.height,
                stride: run.width * 4,
                format: PixelFormat::Rgba8,
                data: vec![0u8; (run.width * run.height * 4) as usize],
                captured_at: Instant::now(),
            });
        }
        let mut last_index: Option<usize> = None;
        loop {
            if stop.load(Ordering::Relaxed) || !sender.is_open() {
                break StretchEnd::Stopped;
            }
            if let Some(target) = take_seek(hub_id) {
                break StretchEnd::Seek(target);
            }
            let pushed = *audio_seconds
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            let raw = seek_base + pushed;
            let position = if duration > 0.0 { raw % duration } else { raw };
            publish_transport(hub_id, position, duration);
            if !run.hidden_face {
                let index = run.table.index_at(position);
                if last_index != Some(index) {
                    // A blank display name (whitespace-only stem) would render
                    // the 1×1 transparent frame — keep a visible marker so the
                    // source never silently disappears from the canvas.
                    let mut name = run.table.names.get(index).cloned().unwrap_or_default();
                    if name.trim().is_empty() {
                        name = "-".to_string();
                    }
                    if let Ok(raster) = render_text(&TextStyle {
                        text: name,
                        font_family: None,
                        font_file: None,
                        size_px: 40.0,
                        color: [255, 255, 255, 255],
                        align: TextAlign::Left,
                        line_spacing: 1.0,
                        force_rtl: false,
                        wrap_width: Some(run.width.saturating_sub(24)),
                        ..TextStyle::default()
                    }) {
                        sender.send(raster);
                    }
                    last_index = Some(index);
                }
            }
            // The audio pump ending is this lane's EOF signal.
            if let Some(handle) = audio_thread.as_ref() {
                if handle.is_finished() && !stop.load(Ordering::Relaxed) {
                    break StretchEnd::Ended;
                }
            }
            std::thread::sleep(AUDIO_FACE_POLL);
        }
    };

    let _ = kill_tx.send(());
    if let Some(handle) = watchdog {
        let _ = handle.join();
    }
    if let Some(handle) = audio_thread {
        let _ = handle.join();
    }
    if matches!(end, StretchEnd::Ended) && stop.load(Ordering::Relaxed) {
        end = StretchEnd::Stopped;
    }
    end
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_table() -> PlaylistTable {
        PlaylistTable {
            order: vec![2, 0, 1],
            names: vec!["c".into(), "a".into(), "b".into()],
            starts: vec![0.0, 10.0, 25.0],
            ins: vec![0.0, 3.0, 0.0],
            total: 40.0,
        }
    }

    #[test]
    fn index_at_walks_the_cumulative_table_and_wraps() {
        let table = test_table();
        assert_eq!(table.index_at(0.0), 0);
        assert_eq!(table.index_at(9.9), 0);
        assert_eq!(table.index_at(10.0), 1);
        assert_eq!(table.index_at(39.0), 2);
        assert_eq!(table.index_at(41.0), 0, "wraps past the total");
    }

    #[test]
    fn next_jumps_to_the_following_item_and_wraps_at_the_end() {
        let table = test_table();
        assert_eq!(jump_target(&table, 5.0, PlaylistAction::Next), 10.0);
        assert_eq!(jump_target(&table, 30.0, PlaylistAction::Next), 0.0);
    }

    #[test]
    fn previous_restarts_late_in_an_item_and_steps_back_early() {
        let table = test_table();
        // 5 s into item 0 → restart it.
        assert_eq!(jump_target(&table, 15.0, PlaylistAction::Previous), 10.0);
        // Just entered item 1 → step back to item 0.
        assert_eq!(jump_target(&table, 10.5, PlaylistAction::Previous), 0.0);
        // Start of the list → stay at the top.
        assert_eq!(jump_target(&table, 0.5, PlaylistAction::Previous), 0.0);
    }

    #[test]
    fn cues_map_through_the_shuffle_order_and_the_in_trim() {
        let table = Arc::new(test_table());
        REGISTRY.insert("playlist-test-cue", &table);
        // Original item 0 plays second (starts 10.0) with a 3 s in-trim: a
        // cue at 7 s into the FILE is 4 s into the played span.
        assert!(cue("playlist-test-cue", 0, 7.0));
        assert_eq!(
            media_transport("playlist-test-cue").0,
            0.0,
            "cue queues a seek; it does not move the published playhead"
        );
        // Unknown item: refused.
        assert!(!cue("playlist-test-cue", 9, 0.0));
        drop(table);
        assert!(!cue("playlist-test-cue", 0, 0.0), "dead is dead");
    }

    #[test]
    fn shuffle_is_a_permutation() {
        let order = shuffled_order(16, 12345);
        let mut sorted = order.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, (0..16).collect::<Vec<_>>());
    }

    #[test]
    fn display_names_are_file_stems() {
        assert_eq!(display_name("C:/music/song.mp3"), "song");
        assert_eq!(display_name("clip.final.mp4"), "clip.final");
    }
}
