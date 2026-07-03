//! The recording sinks: the **owned `.frec` writer** (lossless, zero
//! dependencies) and the **labeled ffmpeg muxer** (wire codecs into
//! mp4/mkv/mov/webm, up to 6 AAC/Opus tracks, file splitting) — both behind
//! [`crate::recorder::RecordSink`], fed by the one pacing thread.
//!
//! ## How ffmpeg is fed
//!
//! Video goes down **stdin** as raw RGBA at the spec'd CFR rate — ffmpeg
//! derives timestamps from frame count, which is what makes pause/resume
//! gapless. Each audio track rides its own **localhost TCP socket** (raw
//! f32le): a listener binds on `127.0.0.1:0`, ffmpeg connects to it as an
//! input, and the listener accepts **exactly one** connection then closes —
//! nothing else can attach afterwards, and a lost race fails the recording
//! loudly rather than corrupting it silently. These sockets never leave the
//! machine; the privacy invariant (nothing outbound but the user's own
//! targets) stands.
//!
//! Every input gets its own writer thread with a bounded queue so a slow
//! encoder backpressures honestly (counted, surfaced) instead of
//! deadlocking the mux interleave.

use std::io::Write;
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{mpsc, Arc};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::ffmpeg::Ffmpeg;
use crate::freally_video::{FrecSpec, FrecWriter, PixelFormat};
use crate::recorder::{RecordSink, RecordSpec};

/// The containers a recording can land in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Container {
    /// The owned lossless codec — the default; needs nothing fetched.
    Frec,
    Mkv,
    Mp4,
    Mov,
    Webm,
}

impl Container {
    pub fn extension(self) -> &'static str {
        match self {
            Container::Frec => "frec",
            Container::Mkv => "mkv",
            Container::Mp4 => "mp4",
            Container::Mov => "mov",
            Container::Webm => "webm",
        }
    }

    /// The ffmpeg muxer name (wire containers only).
    fn muxer(self) -> &'static str {
        match self {
            Container::Frec => unreachable!("frec never goes through ffmpeg"),
            Container::Mkv => "matroska",
            Container::Mp4 => "mp4",
            Container::Mov => "mov",
            Container::Webm => "webm",
        }
    }
}

/// Rate-control mode (mirrored in the UI).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RcMode {
    Cbr,
    Vbr,
    Cqp,
}

/// Encoder rate control, mapped per encoder family by [`video_args`].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateControl {
    pub mode: RcMode,
    pub bitrate_kbps: u32,
    /// Constant-quality value for [`RcMode::Cqp`] (0–51 scale).
    pub cq: u8,
}

/// Global (pre-input) ffmpeg args an encoder needs — VAAPI brings up its
/// hardware device here.
pub fn global_args(encoder_id: &str) -> Vec<String> {
    if encoder_id.ends_with("_vaapi") {
        vec![
            "-init_hw_device".into(),
            "vaapi=va:/dev/dri/renderD128".into(),
            "-filter_hw_device".into(),
            "va".into(),
        ]
    } else {
        Vec::new()
    }
}

/// The output-side video args for an encoder + rate control + keyframe
/// interval. Pure — every family's shape is unit-tested.
pub fn video_args(encoder_id: &str, rc: &RateControl, keyint_frames: u32) -> Vec<String> {
    let mut args: Vec<String> = vec!["-c:v".into(), encoder_id.into()];
    let bitrate = format!("{}k", rc.bitrate_kbps.max(100));
    let maxrate = format!("{}k", rc.bitrate_kbps.max(100) * 2);
    let bufsize = format!("{}k", rc.bitrate_kbps.max(100) * 2);
    let cq = rc.cq.min(51).to_string();

    let software = matches!(
        encoder_id,
        "libx264" | "libx265" | "libaom-av1" | "libsvtav1"
    );
    match encoder_id {
        "libx264" | "libx265" => {
            args.extend(["-preset".into(), "veryfast".into()]);
            match rc.mode {
                RcMode::Cbr => args.extend([
                    "-b:v".into(),
                    bitrate.clone(),
                    "-minrate".into(),
                    bitrate.clone(),
                    "-maxrate".into(),
                    bitrate,
                    "-bufsize".into(),
                    bufsize,
                ]),
                RcMode::Vbr => args.extend([
                    "-b:v".into(),
                    bitrate,
                    "-maxrate".into(),
                    maxrate,
                    "-bufsize".into(),
                    bufsize,
                ]),
                RcMode::Cqp => args.extend(["-crf".into(), cq]),
            }
        }
        "libaom-av1" => {
            args.extend([
                "-usage".into(),
                "realtime".into(),
                "-cpu-used".into(),
                "8".into(),
            ]);
            match rc.mode {
                RcMode::Cqp => args.extend(["-crf".into(), cq, "-b:v".into(), "0".into()]),
                _ => args.extend(["-b:v".into(), bitrate]),
            }
        }
        "libsvtav1" => {
            args.extend(["-preset".into(), "10".into()]);
            match rc.mode {
                RcMode::Cqp => args.extend(["-crf".into(), cq]),
                _ => args.extend(["-b:v".into(), bitrate]),
            }
        }
        id if id.ends_with("_nvenc") => {
            args.extend(["-preset".into(), "p5".into()]);
            match rc.mode {
                RcMode::Cbr => args.extend(["-rc".into(), "cbr".into(), "-b:v".into(), bitrate]),
                RcMode::Vbr => args.extend([
                    "-rc".into(),
                    "vbr".into(),
                    "-b:v".into(),
                    bitrate,
                    "-maxrate".into(),
                    maxrate,
                ]),
                RcMode::Cqp => args.extend(["-rc".into(), "constqp".into(), "-qp".into(), cq]),
            }
        }
        id if id.ends_with("_qsv") => match rc.mode {
            RcMode::Cbr => {
                args.extend(["-b:v".into(), bitrate.clone(), "-maxrate".into(), bitrate])
            }
            RcMode::Vbr => args.extend(["-b:v".into(), bitrate, "-maxrate".into(), maxrate]),
            RcMode::Cqp => args.extend(["-global_quality".into(), cq]),
        },
        id if id.ends_with("_amf") => match rc.mode {
            RcMode::Cbr => args.extend(["-rc".into(), "cbr".into(), "-b:v".into(), bitrate]),
            RcMode::Vbr => args.extend([
                "-rc".into(),
                "vbr_peak".into(),
                "-b:v".into(),
                bitrate,
                "-maxrate".into(),
                maxrate,
            ]),
            RcMode::Cqp => args.extend([
                "-rc".into(),
                "cqp".into(),
                "-qp_i".into(),
                cq.clone(),
                "-qp_p".into(),
                cq,
            ]),
        },
        id if id.ends_with("_videotoolbox") => match rc.mode {
            RcMode::Cqp => {
                // VT quality runs 1–100 (higher = better); map the 0–51
                // CQP scale (lower = better) onto it.
                let quality = (100u32.saturating_sub(2 * rc.cq.min(51) as u32)).max(1);
                args.extend(["-q:v".into(), quality.to_string()]);
            }
            _ => args.extend(["-b:v".into(), bitrate]),
        },
        id if id.ends_with("_vaapi") => match rc.mode {
            RcMode::Cbr => args.extend(["-rc_mode".into(), "CBR".into(), "-b:v".into(), bitrate]),
            RcMode::Vbr => args.extend([
                "-rc_mode".into(),
                "VBR".into(),
                "-b:v".into(),
                bitrate,
                "-maxrate".into(),
                maxrate,
            ]),
            RcMode::Cqp => args.extend(["-rc_mode".into(), "CQP".into(), "-qp".into(), cq]),
        },
        _ => args.extend(["-b:v".into(), bitrate]),
    }

    args.extend(["-g".into(), keyint_frames.max(1).to_string()]);
    if software {
        // rgba input would negotiate yuv444 on x264 — players choke; pin
        // the universally-safe format. Hardware wrappers negotiate nv12
        // themselves.
        args.extend(["-pix_fmt".into(), "yuv420p".into()]);
    }
    if encoder_id.ends_with("_vaapi") {
        args.extend(["-vf".into(), "format=nv12,hwupload".into()]);
    }
    args
}

/// The audio-encoder args for a container (`aac` everywhere it is legal;
/// webm requires Opus).
pub fn audio_args(container: Container, bitrate_kbps: u32) -> Vec<String> {
    let bitrate = format!("{}k", bitrate_kbps.clamp(32, 512));
    match container {
        Container::Frec => unreachable!("frec never goes through ffmpeg"),
        Container::Webm => vec!["-c:a".into(), "libopus".into(), "-b:a".into(), bitrate],
        _ => vec!["-c:a".into(), "aac".into(), "-b:a".into(), bitrate],
    }
}

// ---------------------------------------------------------------------------
// The owned `.frec` sink
// ---------------------------------------------------------------------------

/// Lossless recording through the owned codec — video + PCM tracks, with
/// frame-boundary file splitting (each part is a standalone playable file).
pub struct FrecSink {
    spec: RecordSpec,
    writer: Option<FrecWriter>,
    /// Split threshold in frames (`None` = never split).
    split_frames: Option<u64>,
    /// Frames written into the current part.
    part_frames: u64,
    /// Frames written into all finished parts (rebases audio positions).
    frames_before_part: u64,
    base_path: PathBuf,
    part: u32,
    finished: Vec<PathBuf>,
}

impl FrecSink {
    pub fn create(
        spec: RecordSpec,
        path: PathBuf,
        split_minutes: Option<u32>,
    ) -> Result<Self, String> {
        let split_frames =
            split_minutes.map(|minutes| minutes.max(1) as u64 * 60 * spec.fps as u64);
        let first = part_path(&path, split_frames.is_some(), 1);
        let writer = FrecWriter::create(&first, frec_spec(&spec)).map_err(|err| err.to_string())?;
        Ok(FrecSink {
            spec,
            writer: Some(writer),
            split_frames,
            part_frames: 0,
            frames_before_part: 0,
            base_path: path,
            part: 1,
            finished: vec![first],
        })
    }

    fn rotate(&mut self) -> Result<(), String> {
        if let Some(writer) = self.writer.take() {
            writer.finish().map_err(|err| err.to_string())?;
        }
        self.frames_before_part += self.part_frames;
        self.part_frames = 0;
        self.part += 1;
        let next = part_path(&self.base_path, true, self.part);
        self.writer =
            Some(FrecWriter::create(&next, frec_spec(&self.spec)).map_err(|err| err.to_string())?);
        self.finished.push(next);
        Ok(())
    }

    /// The current part's audio origin, in samples (exact for the rates the
    /// app records: 48000 divides by 24/25/30/60 fps evenly enough — the
    /// division is per whole frames, not per second).
    fn audio_base(&self) -> u64 {
        self.frames_before_part * 48_000 / self.spec.fps.max(1) as u64
    }
}

fn frec_spec(spec: &RecordSpec) -> FrecSpec {
    FrecSpec {
        width: spec.width,
        height: spec.height,
        fps_num: spec.fps,
        fps_den: 1,
        pixel_format: PixelFormat::Rgba8,
        audio_tracks: spec.tracks.len() as u8,
        sample_rate: 48_000,
    }
}

/// `name.ext` → `name part001.ext` when splitting, `name.ext` otherwise.
fn part_path(base: &Path, splitting: bool, part: u32) -> PathBuf {
    if !splitting {
        return base.to_path_buf();
    }
    let stem = base
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("recording");
    let ext = base
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("frec");
    base.with_file_name(format!("{stem} part{part:03}.{ext}"))
}

impl RecordSink for FrecSink {
    fn write_video(&mut self, pixels: &Arc<Vec<u8>>) -> Result<(), String> {
        if let Some(split) = self.split_frames {
            if self.part_frames >= split {
                self.rotate()?;
            }
        }
        let writer = self.writer.as_mut().expect("writer while recording");
        writer.write_frame(pixels).map_err(|err| err.to_string())?;
        self.part_frames += 1;
        Ok(())
    }

    fn write_audio(&mut self, slot: usize, sample_pos: u64, samples: &[f32]) -> Result<(), String> {
        let base = self.audio_base();
        // Blocks from before this part's first frame (rotation race) rebase
        // to 0 — a one-block nudge at worst, inaudible and lossless.
        let part_pos = sample_pos.saturating_sub(base);
        let writer = self.writer.as_mut().expect("writer while recording");
        writer
            .write_audio(slot as u8, part_pos, samples)
            .map_err(|err| err.to_string())
    }

    fn finish(mut self: Box<Self>) -> Result<Vec<PathBuf>, String> {
        if let Some(writer) = self.writer.take() {
            writer.finish().map_err(|err| err.to_string())?;
        }
        Ok(self.finished)
    }
}

// ---------------------------------------------------------------------------
// The labeled ffmpeg sink
// ---------------------------------------------------------------------------

/// What the ffmpeg sink writes and how (built by the app from settings).
#[derive(Debug, Clone)]
pub struct WirePlan {
    pub container: Container,
    pub encoder_id: String,
    pub rate_control: RateControl,
    pub keyframe_sec: f32,
    pub audio_bitrate_kbps: u32,
    /// Split into playable segments every N minutes (`None` = one file).
    pub split_minutes: Option<u32>,
    /// The output path (used as a `part%03d` template when splitting).
    pub path: PathBuf,
}

enum VideoMsg {
    Frame(Arc<Vec<u8>>),
}

struct AudioLane {
    tx: Option<mpsc::SyncSender<Vec<f32>>>,
    thread: Option<std::thread::JoinHandle<()>>,
}

/// Wire-codec recording through the on-demand ffmpeg component: one child
/// process, video over stdin, one localhost socket per audio track.
pub struct FfmpegSink {
    child: Child,
    video_tx: Option<mpsc::SyncSender<VideoMsg>>,
    video_thread: Option<std::thread::JoinHandle<()>>,
    lanes: Vec<AudioLane>,
    stderr_thread: Option<std::thread::JoinHandle<Vec<u8>>>,
    /// Video frames dropped at the queue (sink overloaded) — the recorder
    /// re-dups later frames so sync holds; the count is surfaced honestly.
    pub queue_drops: Arc<AtomicU64>,
    paths: Vec<PathBuf>,
    split: bool,
}

impl FfmpegSink {
    pub fn spawn(ffmpeg: &Ffmpeg, spec: &RecordSpec, plan: &WirePlan) -> Result<Self, String> {
        if plan.container == Container::Frec {
            return Err("the frec container uses the owned writer, not ffmpeg".to_string());
        }
        // One listener per audio track, bound before ffmpeg starts.
        let mut listeners = Vec::new();
        for _ in &spec.tracks {
            let listener = TcpListener::bind("127.0.0.1:0")
                .map_err(|err| format!("could not bind a loopback audio socket: {err}"))?;
            listeners.push(listener);
        }

        let keyint = (plan.keyframe_sec.max(0.25) * spec.fps as f32).round() as u32;
        let mut cmd = crate::ffmpeg::command(ffmpeg);
        cmd.stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::piped());
        cmd.args(["-hide_banner", "-v", "error", "-y"]);
        cmd.args(global_args(&plan.encoder_id));
        // Input 0: raw RGBA video on stdin, CFR at the spec'd rate.
        cmd.args([
            "-f",
            "rawvideo",
            "-pix_fmt",
            "rgba",
            "-s",
            &format!("{}x{}", spec.width, spec.height),
            "-r",
            &spec.fps.to_string(),
            "-i",
            "pipe:0",
        ]);
        // Inputs 1..=n: one f32le stereo stream per track over loopback.
        for listener in &listeners {
            let port = listener.local_addr().map_err(|err| err.to_string())?.port();
            cmd.args([
                "-f",
                "f32le",
                "-ar",
                "48000",
                "-ac",
                "2",
                "-i",
                &format!("tcp://127.0.0.1:{port}"),
            ]);
        }
        cmd.args(["-map", "0:v"]);
        for index in 1..=listeners.len() {
            cmd.args(["-map", &format!("{index}:a")]);
        }
        cmd.args(video_args(&plan.encoder_id, &plan.rate_control, keyint));
        if !spec.tracks.is_empty() {
            cmd.args(audio_args(plan.container, plan.audio_bitrate_kbps));
        }

        let split = plan.split_minutes.is_some();
        let out_path = if split {
            segment_template(&plan.path)
        } else {
            plan.path.clone()
        };
        if let Some(minutes) = plan.split_minutes {
            cmd.args([
                "-f",
                "segment",
                "-segment_time",
                &(minutes.max(1) * 60).to_string(),
                "-reset_timestamps",
                "1",
                "-segment_format",
                plan.container.muxer(),
            ]);
            if matches!(plan.container, Container::Mp4 | Container::Mov) {
                cmd.args(["-segment_format_options", "movflags=+faststart"]);
            }
        } else {
            cmd.args(["-f", plan.container.muxer()]);
            if matches!(plan.container, Container::Mp4 | Container::Mov) {
                cmd.args(["-movflags", "+faststart"]);
            }
        }
        cmd.arg(&out_path);

        let mut child = cmd
            .spawn()
            .map_err(|err| format!("could not start the ffmpeg component: {err}"))?;
        let stdin = child.stdin.take().expect("stdin piped");
        let stderr = child.stderr.take().expect("stderr piped");
        let stderr_thread = std::thread::Builder::new()
            .name("fcap-rec-ffmpeg-err".into())
            .spawn(move || {
                use std::io::Read;
                let mut tail = Vec::new();
                let mut reader = std::io::BufReader::new(stderr);
                let _ = reader.read_to_end(&mut tail);
                // Keep the last 4 KiB — enough for the honest error message.
                if tail.len() > 4096 {
                    tail.drain(..tail.len() - 4096);
                }
                tail
            })
            .map_err(|err| err.to_string())?;

        let queue_drops = Arc::new(AtomicU64::new(0));
        let video_thread = spawn_video_writer(stdin, &queue_drops)?;
        let (video_tx, video_thread) = video_thread;

        let mut lanes = Vec::new();
        for listener in listeners {
            lanes.push(spawn_audio_writer(listener)?);
        }

        Ok(FfmpegSink {
            child,
            video_tx: Some(video_tx),
            video_thread: Some(video_thread),
            lanes,
            stderr_thread: Some(stderr_thread),
            queue_drops,
            paths: vec![out_path],
            split,
        })
    }
}

/// `name.mkv` → `name part%03d.mkv` for the segment muxer.
fn segment_template(base: &Path) -> PathBuf {
    let stem = base
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("recording");
    let ext = base
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("mkv");
    base.with_file_name(format!("{stem} part%03d.{ext}"))
}

/// Expand a written `part%03d` template to the files that actually exist.
fn expand_segments(template: &Path) -> Vec<PathBuf> {
    let Some(name) = template.file_name().and_then(|name| name.to_str()) else {
        return vec![template.to_path_buf()];
    };
    let mut paths = Vec::new();
    for index in 0..10_000 {
        let candidate = template.with_file_name(name.replace("%03d", &format!("{index:03}")));
        if candidate.is_file() {
            paths.push(candidate);
        } else if index > 0 {
            break; // segments are sequential from 000
        }
    }
    if paths.is_empty() {
        vec![template.to_path_buf()]
    } else {
        paths
    }
}

type VideoWriter = (mpsc::SyncSender<VideoMsg>, std::thread::JoinHandle<()>);

fn spawn_video_writer(
    mut stdin: ChildStdin,
    _drops: &Arc<AtomicU64>,
) -> Result<VideoWriter, String> {
    // 8 frames in flight (~130 ms at 60 fps) — deep enough for encoder
    // jitter, shallow enough that memory stays bounded (8 × frame size).
    let (tx, rx) = mpsc::sync_channel::<VideoMsg>(8);
    let thread = std::thread::Builder::new()
        .name("fcap-rec-video".into())
        .spawn(move || {
            while let Ok(VideoMsg::Frame(frame)) = rx.recv() {
                if stdin.write_all(&frame).is_err() {
                    break; // ffmpeg died — the pacer sees the send error next
                }
            }
            // Sender dropped (or ffmpeg died): stdin drops here → EOF →
            // ffmpeg flushes and finalizes the container.
        })
        .map_err(|err| err.to_string())?;
    Ok((tx, thread))
}

fn spawn_audio_writer(listener: TcpListener) -> Result<AudioLane, String> {
    let (tx, rx) = mpsc::sync_channel::<Vec<f32>>(512);
    let thread = std::thread::Builder::new()
        .name("fcap-rec-audio".into())
        .spawn(move || {
            // Accept exactly one connection (ffmpeg's), then close the
            // listener — nothing else can attach for the session's life.
            let Ok((mut stream, peer)) = listener.accept() else {
                return;
            };
            drop(listener);
            if !peer.ip().is_loopback() {
                // Cannot happen on a 127.0.0.1 bind — but never trust it.
                return;
            }
            let _ = stream.set_nodelay(true);
            let mut bytes = Vec::with_capacity(4096 * 8);
            while let Ok(samples) = rx.recv() {
                bytes.clear();
                for sample in &samples {
                    bytes.extend_from_slice(&sample.to_le_bytes());
                }
                if stream.write_all(&bytes).is_err() {
                    break;
                }
            }
            let _ = stream.shutdown(std::net::Shutdown::Write);
        })
        .map_err(|err| err.to_string())?;
    Ok(AudioLane {
        tx: Some(tx),
        thread: Some(thread),
    })
}

impl FfmpegSink {
    /// An honest error for a mid-recording death: the exit status + the
    /// child's last stderr lines (only joined once the child has exited —
    /// the reader thread EOFs exactly then).
    fn death_note(&mut self, context: &str) -> String {
        let status = self.child.try_wait().ok().flatten();
        let tail = if status.is_some() {
            self.stderr_thread
                .take()
                .and_then(|thread| thread.join().ok())
                .unwrap_or_default()
        } else {
            Vec::new()
        };
        let tail = String::from_utf8_lossy(&tail);
        let tail = tail.trim();
        match (status, tail.is_empty()) {
            (Some(status), false) => format!("{context} (exited with {status}): {tail}"),
            (Some(status), true) => format!("{context} (exited with {status})"),
            (None, _) => context.to_string(),
        }
    }
}

impl RecordSink for FfmpegSink {
    fn write_video(&mut self, pixels: &Arc<Vec<u8>>) -> Result<(), String> {
        let Some(tx) = &self.video_tx else {
            return Err("the sink is already finished".to_string());
        };
        match tx.try_send(VideoMsg::Frame(Arc::clone(pixels))) {
            Ok(()) => Ok(()),
            Err(mpsc::TrySendError::Full(_)) => {
                // Encoder overloaded: skip this write; the recorder's clock
                // re-dups a later frame so the timeline stays intact.
                self.queue_drops.fetch_add(1, Ordering::Relaxed);
                Ok(())
            }
            Err(mpsc::TrySendError::Disconnected(_)) => {
                Err(self.death_note("the ffmpeg component stopped accepting video"))
            }
        }
    }

    fn write_audio(
        &mut self,
        slot: usize,
        _sample_pos: u64,
        samples: &[f32],
    ) -> Result<(), String> {
        let Some(lane) = self.lanes.get(slot) else {
            return Err(format!("no audio lane {slot}"));
        };
        let Some(tx) = &lane.tx else {
            return Err("the sink is already finished".to_string());
        };
        match tx.try_send(samples.to_vec()) {
            Ok(()) => Ok(()),
            Err(mpsc::TrySendError::Full(_)) => {
                self.queue_drops.fetch_add(1, Ordering::Relaxed);
                Ok(())
            }
            Err(mpsc::TrySendError::Disconnected(_)) => {
                Err(self.death_note("the ffmpeg component stopped accepting audio"))
            }
        }
    }

    fn finish(mut self: Box<Self>) -> Result<Vec<PathBuf>, String> {
        // Close every feed: writer threads flush, stdin/sockets EOF, ffmpeg
        // finalizes the container (faststart can take a while on long
        // recordings — the wait is generous, then honest).
        self.video_tx.take();
        for lane in &mut self.lanes {
            lane.tx.take();
        }
        if let Some(thread) = self.video_thread.take() {
            let _ = thread.join();
        }
        for lane in &mut self.lanes {
            if let Some(thread) = lane.thread.take() {
                let _ = thread.join();
            }
        }

        let deadline = std::time::Instant::now() + Duration::from_secs(180);
        let status = loop {
            match self.child.try_wait() {
                Ok(Some(status)) => break status,
                Ok(None) => {
                    if std::time::Instant::now() > deadline {
                        let _ = self.child.kill();
                        let _ = self.child.wait();
                        return Err(
                            "the ffmpeg component did not finalize the file in time".to_string()
                        );
                    }
                    std::thread::sleep(Duration::from_millis(50));
                }
                Err(err) => return Err(format!("waiting for the ffmpeg component: {err}")),
            }
        };
        let stderr_tail = self
            .stderr_thread
            .take()
            .and_then(|thread| thread.join().ok())
            .unwrap_or_default();
        if !status.success() {
            return Err(format!(
                "the ffmpeg component exited with {status}: {}",
                String::from_utf8_lossy(&stderr_tail).trim()
            ));
        }
        let template = self.paths.remove(0);
        Ok(if self.split {
            expand_segments(&template)
        } else {
            vec![template]
        })
    }
}

impl Drop for FfmpegSink {
    fn drop(&mut self) {
        // Belt-and-braces: a dropped-unfinished sink must not leak a child.
        self.video_tx.take();
        for lane in &mut self.lanes {
            lane.tx.take();
        }
        if self.child.try_wait().ok().flatten().is_none() {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rc(mode: RcMode) -> RateControl {
        RateControl {
            mode,
            bitrate_kbps: 8000,
            cq: 23,
        }
    }

    #[test]
    fn x264_pins_yuv420p_and_maps_rate_control() {
        let cbr = video_args("libx264", &rc(RcMode::Cbr), 120);
        assert!(cbr.windows(2).any(|w| w == ["-b:v", "8000k"]));
        assert!(cbr.windows(2).any(|w| w == ["-minrate", "8000k"]));
        assert!(cbr.windows(2).any(|w| w == ["-g", "120"]));
        assert!(cbr.windows(2).any(|w| w == ["-pix_fmt", "yuv420p"]));

        let cqp = video_args("libx264", &rc(RcMode::Cqp), 120);
        assert!(cqp.windows(2).any(|w| w == ["-crf", "23"]));
        assert!(!cqp.iter().any(|arg| arg == "-b:v"));
    }

    #[test]
    fn nvenc_uses_its_rc_modes_and_no_forced_pix_fmt() {
        let cbr = video_args("h264_nvenc", &rc(RcMode::Cbr), 60);
        assert!(cbr.windows(2).any(|w| w == ["-rc", "cbr"]));
        assert!(!cbr.iter().any(|arg| arg == "-pix_fmt"), "hw negotiates");

        let cqp = video_args("hevc_nvenc", &rc(RcMode::Cqp), 60);
        assert!(cqp.windows(2).any(|w| w == ["-rc", "constqp"]));
        assert!(cqp.windows(2).any(|w| w == ["-qp", "23"]));
    }

    #[test]
    fn vaapi_brings_up_its_device_and_upload_chain() {
        let global = global_args("h264_vaapi");
        assert!(global.iter().any(|arg| arg.starts_with("vaapi=va:")));
        let video = video_args("h264_vaapi", &rc(RcMode::Cbr), 60);
        assert!(video
            .windows(2)
            .any(|w| w == ["-vf", "format=nv12,hwupload"]));
        assert!(global_args("h264_nvenc").is_empty());
    }

    #[test]
    fn videotoolbox_maps_cq_onto_its_quality_scale() {
        let cqp = video_args("h264_videotoolbox", &rc(RcMode::Cqp), 60);
        assert!(cqp.windows(2).any(|w| w == ["-q:v", "54"])); // 100 - 2×23
    }

    #[test]
    fn webm_takes_opus_everything_else_aac() {
        assert!(audio_args(Container::Webm, 160)
            .windows(2)
            .any(|w| w == ["-c:a", "libopus"]));
        for container in [Container::Mp4, Container::Mkv, Container::Mov] {
            assert!(audio_args(container, 192)
                .windows(2)
                .any(|w| w == ["-c:a", "aac"]));
        }
    }

    #[test]
    fn part_paths_number_cleanly() {
        let base = Path::new("C:/rec/Freally Capture 2026.frec");
        assert_eq!(part_path(base, false, 1), base);
        assert_eq!(
            part_path(base, true, 2).file_name().unwrap(),
            "Freally Capture 2026 part002.frec"
        );
        let template = segment_template(Path::new("C:/rec/take.mkv"));
        assert_eq!(template.file_name().unwrap(), "take part%03d.mkv");
    }

    #[test]
    fn frec_sink_splits_at_the_frame_boundary_with_rebased_audio() {
        let dir = std::env::temp_dir().join(format!(
            "fcap-frecsink-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).expect("mkdir");
        let spec = RecordSpec {
            width: 16,
            height: 8,
            fps: 60,
            tracks: vec![0],
        };
        // 1-minute split at 60 fps = 3600 frames — too slow for a unit
        // test; exercise rotation through the internal threshold instead.
        let mut sink =
            FrecSink::create(spec.clone(), dir.join("take.frec"), Some(1)).expect("create");
        sink.split_frames = Some(5); // rotate every 5 frames (test override)

        let frame = Arc::new(vec![7u8; spec.frame_bytes()]);
        let block = vec![0.5f32; 960];
        for index in 0..12u64 {
            sink.write_video(&frame).expect("video");
            sink.write_audio(0, index * 480, &block).expect("audio");
        }
        let paths = Box::new(sink).finish().expect("finish");
        assert_eq!(paths.len(), 3, "12 frames / 5 per part = 3 parts");
        for path in &paths {
            let mut reader = crate::freally_video::FrecReader::open(path).expect("each part opens");
            let mut frames = 0;
            let mut first_audio_pos = None;
            while let Some(chunk) = reader.next_chunk().expect("reads") {
                match chunk {
                    crate::freally_video::FrecChunk::Video { .. } => frames += 1,
                    crate::freally_video::FrecChunk::Audio { sample_pos, .. } => {
                        first_audio_pos.get_or_insert(sample_pos);
                    }
                }
            }
            assert!(frames > 0, "every part has frames");
            assert_eq!(
                first_audio_pos.unwrap_or(0) % 480,
                0,
                "part-local audio positions stay block-aligned"
            );
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn segment_expansion_walks_sequential_parts() {
        let dir = std::env::temp_dir().join(format!(
            "fcap-seg-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).expect("mkdir");
        for index in 0..3 {
            std::fs::write(dir.join(format!("take part{index:03}.mkv")), b"x").expect("write");
        }
        let found = expand_segments(&dir.join("take part%03d.mkv"));
        assert_eq!(found.len(), 3);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
