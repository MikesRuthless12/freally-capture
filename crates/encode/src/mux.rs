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
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::time::{Duration, Instant};

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

/// The quality/speed trade every encoder family maps onto its own knob
/// (x264 `-preset`, NVENC `p1–p7`, AMF `-quality`, …).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EncPreset {
    Quality,
    #[default]
    Balanced,
    Performance,
}

/// Global (pre-input) ffmpeg args an encoder needs — VAAPI brings up its
/// hardware device here. The device is left for ffmpeg to auto-select (bare
/// `vaapi=va`, no node path) so this matches exactly how the encoder was
/// smoke-verified (`crate::ffmpeg::smoke_test_encoder`); pinning a specific
/// `/dev/dri/renderD*` node here could point at a different GPU than the one
/// that verified, making a "confirmed" encoder fail at record time.
pub fn global_args(encoder_id: &str) -> Vec<String> {
    if encoder_id.ends_with("_vaapi") {
        vec![
            "-init_hw_device".into(),
            "vaapi=va".into(),
            "-filter_hw_device".into(),
            "va".into(),
        ]
    } else {
        Vec::new()
    }
}

/// The output-side video args for an encoder + rate control + quality/perf
/// preset + keyframe interval. Pure — every family's shape is unit-tested.
pub fn video_args(
    encoder_id: &str,
    rc: &RateControl,
    preset: EncPreset,
    keyint_frames: u32,
) -> Vec<String> {
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
            let speed = match preset {
                EncPreset::Quality => "slow",
                EncPreset::Balanced => "veryfast",
                EncPreset::Performance => "ultrafast",
            };
            args.extend(["-preset".into(), speed.into()]);
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
            let cpu_used = match preset {
                EncPreset::Quality => "4",
                EncPreset::Balanced => "8",
                EncPreset::Performance => "10",
            };
            args.extend([
                "-usage".into(),
                "realtime".into(),
                "-cpu-used".into(),
                cpu_used.into(),
            ]);
            match rc.mode {
                RcMode::Cqp => args.extend(["-crf".into(), cq, "-b:v".into(), "0".into()]),
                _ => args.extend(["-b:v".into(), bitrate]),
            }
        }
        "libsvtav1" => {
            let speed = match preset {
                EncPreset::Quality => "6",
                EncPreset::Balanced => "10",
                EncPreset::Performance => "12",
            };
            args.extend(["-preset".into(), speed.into()]);
            match rc.mode {
                RcMode::Cqp => args.extend(["-crf".into(), cq]),
                _ => args.extend(["-b:v".into(), bitrate]),
            }
        }
        id if id.ends_with("_nvenc") => {
            let speed = match preset {
                EncPreset::Quality => "p7",
                EncPreset::Balanced => "p5",
                EncPreset::Performance => "p2",
            };
            args.extend(["-preset".into(), speed.into()]);
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
        id if id.ends_with("_qsv") => {
            let speed = match preset {
                EncPreset::Quality => "veryslow",
                EncPreset::Balanced => "medium",
                EncPreset::Performance => "veryfast",
            };
            args.extend(["-preset".into(), speed.into()]);
            match rc.mode {
                RcMode::Cbr => {
                    args.extend(["-b:v".into(), bitrate.clone(), "-maxrate".into(), bitrate])
                }
                RcMode::Vbr => args.extend(["-b:v".into(), bitrate, "-maxrate".into(), maxrate]),
                RcMode::Cqp => args.extend(["-global_quality".into(), cq]),
            }
        }
        id if id.ends_with("_amf") => {
            let quality = match preset {
                EncPreset::Quality => "quality",
                EncPreset::Balanced => "balanced",
                EncPreset::Performance => "speed",
            };
            args.extend(["-quality".into(), quality.into()]);
            match rc.mode {
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
            }
        }
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

/// `base` → a sibling `{stem} part{suffix}.{ext}` — the one split-part naming
/// scheme, shared by the owned `.frec` writer (a formatted number) and the
/// ffmpeg segment muxer (a literal `%03d` template) so the two can never drift.
fn part_sibling(base: &Path, suffix: &str, default_ext: &str) -> PathBuf {
    let stem = base
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("recording");
    let ext = base
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or(default_ext);
    base.with_file_name(format!("{stem} part{suffix}.{ext}"))
}

/// `name.ext` → `name part001.ext` when splitting, `name.ext` otherwise.
fn part_path(base: &Path, splitting: bool, part: u32) -> PathBuf {
    if !splitting {
        return base.to_path_buf();
    }
    part_sibling(base, &format!("{part:03}"), "frec")
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
    pub preset: EncPreset,
    pub keyframe_sec: f32,
    pub audio_bitrate_kbps: u32,
    /// Split into playable segments every N minutes (`None` = one file).
    pub split_minutes: Option<u32>,
    /// The output path (used as a `part%03d` template when splitting).
    pub path: PathBuf,
}

struct AudioLane {
    tx: Option<mpsc::SyncSender<Vec<f32>>>,
    thread: Option<std::thread::JoinHandle<()>>,
    /// The next stereo-frame position this lane expects. A gap (an upstream
    /// block dropped on the engine→pacer hop) is silence-padded to it, so the
    /// stream ffmpeg receives stays positionally exact and A/V never drifts.
    next_pos: u64,
}

/// Wire-codec recording through the on-demand ffmpeg component: one child
/// process, video over stdin, one localhost socket per audio track.
///
/// **Backpressure, never silent drops:** both feeds use *blocking* bounded
/// sends (like [`FrecSink`], which blocks on disk I/O). If ffmpeg falls
/// behind, the writes block and the recorder's pacing thread slows with it —
/// so the frames/samples ffmpeg receives always match what the recorder
/// counted, and neither stream can silently shorten against the other. A dead
/// child surfaces as a `Disconnected` send error, not a lost frame.
pub struct FfmpegSink {
    child: Child,
    video_tx: Option<mpsc::SyncSender<Arc<Vec<u8>>>>,
    video_thread: Option<std::thread::JoinHandle<()>>,
    lanes: Vec<AudioLane>,
    stderr_thread: Option<std::thread::JoinHandle<Vec<u8>>>,
    /// Set on finish/drop so a lane still waiting for ffmpeg to connect
    /// (`accept()`) stops waiting instead of hanging finalize forever.
    connect_cancel: Arc<AtomicBool>,
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
        cmd.args(video_args(
            &plan.encoder_id,
            &plan.rate_control,
            plan.preset,
            keyint,
        ));
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

        let connect_cancel = Arc::new(AtomicBool::new(false));
        let (video_tx, video_thread) = spawn_video_writer(stdin)?;

        let mut lanes = Vec::new();
        for listener in listeners {
            lanes.push(spawn_audio_writer(listener, Arc::clone(&connect_cancel))?);
        }

        Ok(FfmpegSink {
            child,
            video_tx: Some(video_tx),
            video_thread: Some(video_thread),
            lanes,
            stderr_thread: Some(stderr_thread),
            connect_cancel,
            paths: vec![out_path],
            split,
        })
    }
}

/// What a live stream pushes and how (Phase 5): H.264 + AAC in FLV to an
/// RTMP/RTMPS publish URL. One audio lane — the program mix. The URL embeds
/// the stream key, so this struct redacts it from `Debug` (never logged).
#[derive(Clone)]
pub struct RtmpPlan {
    pub encoder_id: String,
    pub rate_control: RateControl,
    pub preset: EncPreset,
    pub keyframe_sec: f32,
    pub audio_bitrate_kbps: u32,
    /// The full publish URL (`rtmp://…` or `rtmps://…`), key included.
    pub url: String,
}

impl std::fmt::Debug for RtmpPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RtmpPlan")
            .field("encoder_id", &self.encoder_id)
            .field("rate_control", &self.rate_control)
            .field("preset", &self.preset)
            .field("keyframe_sec", &self.keyframe_sec)
            .field("audio_bitrate_kbps", &self.audio_bitrate_kbps)
            .field("url", &"[redacted publish url]")
            .finish()
    }
}

impl FfmpegSink {
    /// Spawn the labeled ffmpeg component publishing FLV to an RTMP(S)
    /// ingest. The streaming twin of [`FfmpegSink::spawn`]: the same
    /// rawvideo-stdin and loopback-audio plumbing and backpressure rules,
    /// with the muxer output aimed at the network instead of a file.
    /// `spec.tracks` must name exactly one mixer track (the stream's
    /// program audio).
    pub fn spawn_rtmp(ffmpeg: &Ffmpeg, spec: &RecordSpec, plan: &RtmpPlan) -> Result<Self, String> {
        if spec.tracks.len() != 1 {
            return Err("a stream carries exactly one audio track".to_string());
        }
        if !(plan.url.starts_with("rtmp://") || plan.url.starts_with("rtmps://")) {
            return Err("the publish URL must start with rtmp:// or rtmps://".to_string());
        }

        let listener = TcpListener::bind("127.0.0.1:0")
            .map_err(|err| format!("could not bind a loopback audio socket: {err}"))?;
        let port = listener.local_addr().map_err(|err| err.to_string())?.port();

        let keyint = (plan.keyframe_sec.max(0.25) * spec.fps as f32).round() as u32;
        let mut cmd = crate::ffmpeg::command(ffmpeg);
        cmd.stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::piped());
        cmd.args(["-hide_banner", "-v", "error", "-y"]);
        cmd.args(global_args(&plan.encoder_id));
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
        cmd.args(["-map", "0:v", "-map", "1:a"]);
        cmd.args(video_args(
            &plan.encoder_id,
            &plan.rate_control,
            plan.preset,
            keyint,
        ));
        // FLV requires AAC on this path (the Webm/Opus branch never applies).
        cmd.args(audio_args(Container::Mp4, plan.audio_bitrate_kbps));
        cmd.args(["-f", "flv", "-flvflags", "no_duration_filesize"]);
        cmd.arg(&plan.url);

        let mut child = cmd
            .spawn()
            .map_err(|err| format!("could not start the ffmpeg component: {err}"))?;
        let stdin = child.stdin.take().expect("stdin piped");
        let stderr = child.stderr.take().expect("stderr piped");
        // ffmpeg echoes the full output URL — **key included** — into stderr
        // when the ingest rejects the connection (a wrong/expired key, the
        // single most common failure). That tail flows into the user-visible
        // error, so scrub the secret out at the source: the publish URL never
        // survives into any string this sink hands back.
        let secret = plan.url.clone();
        let stderr_thread = std::thread::Builder::new()
            .name("fcap-stream-ffmpeg-err".into())
            .spawn(move || {
                use std::io::Read;
                let mut tail = Vec::new();
                let mut reader = std::io::BufReader::new(stderr);
                let _ = reader.read_to_end(&mut tail);
                if tail.len() > 4096 {
                    tail.drain(..tail.len() - 4096);
                }
                scrub_secret(&tail, secret.as_bytes())
            })
            .map_err(|err| err.to_string())?;

        let connect_cancel = Arc::new(AtomicBool::new(false));
        let (video_tx, video_thread) = spawn_video_writer(stdin)?;
        let lanes = vec![spawn_audio_writer(listener, Arc::clone(&connect_cancel))?];

        Ok(FfmpegSink {
            child,
            video_tx: Some(video_tx),
            video_thread: Some(video_thread),
            lanes,
            stderr_thread: Some(stderr_thread),
            connect_cancel,
            paths: Vec::new(), // a stream produces no files
            split: false,
        })
    }
}

/// Replace every occurrence of `secret` in `haystack` with `[redacted]` — the
/// stream key must never survive an ffmpeg stderr tail into a visible error.
/// A byte-level replace (stderr is arbitrary bytes; the URL is ASCII).
fn scrub_secret(haystack: &[u8], secret: &[u8]) -> Vec<u8> {
    if secret.is_empty() || haystack.len() < secret.len() {
        return haystack.to_vec();
    }
    const MASK: &[u8] = b"[redacted]";
    let mut out = Vec::with_capacity(haystack.len());
    let mut i = 0;
    while i < haystack.len() {
        if haystack[i..].starts_with(secret) {
            out.extend_from_slice(MASK);
            i += secret.len();
        } else {
            out.push(haystack[i]);
            i += 1;
        }
    }
    out
}

/// `name.mkv` → `name part%03d.mkv` for the segment muxer.
fn segment_template(base: &Path) -> PathBuf {
    part_sibling(base, "%03d", "mkv")
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

type VideoWriter = (mpsc::SyncSender<Arc<Vec<u8>>>, std::thread::JoinHandle<()>);

fn spawn_video_writer(mut stdin: ChildStdin) -> Result<VideoWriter, String> {
    // 8 frames in flight (~130 ms at 60 fps) — deep enough to absorb encoder
    // jitter, shallow enough that memory stays bounded (8 × frame size). A
    // full queue *blocks* the sender (the pacer), never drops.
    let (tx, rx) = mpsc::sync_channel::<Arc<Vec<u8>>>(8);
    let thread = std::thread::Builder::new()
        .name("fcap-rec-video".into())
        .spawn(move || {
            while let Ok(frame) = rx.recv() {
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

/// How long a lane waits for ffmpeg to connect before giving up (ffmpeg
/// connects within milliseconds in practice; this only bounds a child that
/// died at launch and will never connect).
const AUDIO_CONNECT_TIMEOUT: Duration = Duration::from_secs(30);

/// How long a feed will backpressure before declaring ffmpeg wedged. Normal
/// backpressure drains within a frame period; this only trips if ffmpeg stops
/// reading entirely while still alive (e.g. a full recording disk), so the
/// pacer surfaces an error instead of blocking forever.
const WRITE_STALL_TIMEOUT: Duration = Duration::from_secs(10);

/// The result of a bounded send onto a feed channel.
enum SendOutcome {
    Sent,
    /// The receiver is gone — the child died / closed the pipe.
    Gone,
    /// The queue stayed full past [`WRITE_STALL_TIMEOUT`] — ffmpeg is alive
    /// but not draining (wedged).
    Wedged,
}

/// Send `msg`, backpressuring (never dropping) but bounded so a wedged ffmpeg
/// can't block the pacer — and therefore `recording_stop` — forever.
fn send_bounded<T>(tx: &mpsc::SyncSender<T>, mut msg: T) -> SendOutcome {
    let deadline = Instant::now() + WRITE_STALL_TIMEOUT;
    loop {
        match tx.try_send(msg) {
            Ok(()) => return SendOutcome::Sent,
            Err(mpsc::TrySendError::Disconnected(_)) => return SendOutcome::Gone,
            Err(mpsc::TrySendError::Full(returned)) => {
                if Instant::now() >= deadline {
                    return SendOutcome::Wedged;
                }
                msg = returned;
                std::thread::sleep(Duration::from_millis(2));
            }
        }
    }
}

fn spawn_audio_writer(listener: TcpListener, cancel: Arc<AtomicBool>) -> Result<AudioLane, String> {
    // Non-blocking accept so the wait for ffmpeg's connect can be cancelled
    // (on finish/drop) or time out — a blocking accept() would hang finalize
    // forever if the child died before connecting.
    listener
        .set_nonblocking(true)
        .map_err(|err| err.to_string())?;
    let (tx, rx) = mpsc::sync_channel::<Vec<f32>>(512);
    let thread = std::thread::Builder::new()
        .name("fcap-rec-audio".into())
        .spawn(move || {
            let deadline = Instant::now() + AUDIO_CONNECT_TIMEOUT;
            let mut stream = loop {
                match listener.accept() {
                    // Accept exactly one connection (ffmpeg's), then drop the
                    // listener — nothing else can attach for the session.
                    Ok((stream, peer)) if peer.ip().is_loopback() => break stream,
                    Ok(_) => return, // never on a 127.0.0.1 bind, but don't trust it
                    Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                        if cancel.load(Ordering::Relaxed) || Instant::now() >= deadline {
                            return; // ffmpeg never connected — do not hang
                        }
                        std::thread::sleep(Duration::from_millis(15));
                    }
                    Err(_) => return,
                }
            };
            drop(listener);
            let _ = stream.set_nonblocking(false); // blocking writes from here
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
        next_pos: 0,
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

    fn audio_send_error(&mut self, outcome: SendOutcome) -> String {
        match outcome {
            SendOutcome::Sent => String::new(), // never called on success
            SendOutcome::Gone => self.death_note("the ffmpeg component stopped accepting audio"),
            SendOutcome::Wedged => {
                "the ffmpeg component stopped reading audio — the recording disk may be full"
                    .to_string()
            }
        }
    }
}

impl RecordSink for FfmpegSink {
    fn write_video(&mut self, pixels: &Arc<Vec<u8>>) -> Result<(), String> {
        let Some(tx) = self.video_tx.clone() else {
            return Err("the sink is already finished".to_string());
        };
        // Bounded backpressure: if the encoder is behind, slow the pacer
        // rather than drop a frame — every frame the recorder counts is a
        // frame ffmpeg receives, so the CFR-by-count video can never shorten
        // against the audio. Bounded so a wedged (alive-but-not-reading)
        // ffmpeg surfaces an error instead of blocking `recording_stop`.
        match send_bounded(&tx, Arc::clone(pixels)) {
            SendOutcome::Sent => Ok(()),
            SendOutcome::Gone => {
                Err(self.death_note("the ffmpeg component stopped accepting video"))
            }
            SendOutcome::Wedged => Err(
                "the ffmpeg component stopped reading video — the recording disk may be full"
                    .to_string(),
            ),
        }
    }

    fn write_audio(&mut self, slot: usize, sample_pos: u64, samples: &[f32]) -> Result<(), String> {
        // Own a clone of the sender + a copy of the position so no borrow of
        // `self` is held across `death_note` (which needs `&mut self`).
        let (tx, next_pos) = match self.lanes.get(slot) {
            Some(lane) => match &lane.tx {
                Some(tx) => (tx.clone(), lane.next_pos),
                None => return Err("the sink is already finished".to_string()),
            },
            None => return Err(format!("no audio lane {slot}")),
        };
        // Silence-pad any gap: if an upstream block was dropped on the
        // engine→pacer hop, this block's absolute position jumps ahead of what
        // we've sent. Filling the gap with silence keeps the positionless
        // stream ffmpeg reads exactly aligned, so audio never drifts against
        // the (CFR-by-count) video. `sample_pos`/`next_pos` are stereo frames;
        // interleaved samples are twice that.
        if sample_pos > next_pos {
            let silence = vec![0.0f32; (sample_pos - next_pos) as usize * 2];
            match send_bounded(&tx, silence) {
                SendOutcome::Sent => {}
                outcome => return Err(self.audio_send_error(outcome)),
            }
        }
        // Bounded backpressure (never drop) — same contract as video.
        match send_bounded(&tx, samples.to_vec()) {
            SendOutcome::Sent => {}
            outcome => return Err(self.audio_send_error(outcome)),
        }
        self.lanes[slot].next_pos = sample_pos + (samples.len() / 2) as u64;
        Ok(())
    }

    fn finish(mut self: Box<Self>) -> Result<Vec<PathBuf>, String> {
        // Close every feed: the writer threads drain what's queued, then EOF
        // stdin / the sockets, and ffmpeg finalizes the container (faststart
        // can take a while on a long recording — the wait is generous). Cancel
        // first so a lane still waiting for ffmpeg to connect stops.
        self.connect_cancel.store(true, Ordering::Relaxed);
        self.video_tx.take();
        for lane in &mut self.lanes {
            lane.tx.take();
        }

        // Wait for the child to exit **before** joining the writer threads: a
        // writer can be blocked writing to a wedged (alive, not-reading) child,
        // so killing the child on the deadline is what unblocks the joins —
        // joining first would hang finalize forever.
        let deadline = Instant::now() + Duration::from_secs(180);
        let mut timed_out = false;
        let status = loop {
            match self.child.try_wait() {
                Ok(Some(status)) => break Some(status),
                Ok(None) => {
                    if Instant::now() > deadline {
                        let _ = self.child.kill();
                        let _ = self.child.wait();
                        timed_out = true;
                        break None;
                    }
                    std::thread::sleep(Duration::from_millis(50));
                }
                Err(err) => return Err(format!("waiting for the ffmpeg component: {err}")),
            }
        };

        // The child is gone now (exited or killed) → the writers' stdin/socket
        // writes fail and their loops end, so these joins can't hang.
        if let Some(thread) = self.video_thread.take() {
            let _ = thread.join();
        }
        for lane in &mut self.lanes {
            if let Some(thread) = lane.thread.take() {
                let _ = thread.join();
            }
        }
        let stderr_tail = self
            .stderr_thread
            .take()
            .and_then(|thread| thread.join().ok())
            .unwrap_or_default();

        if timed_out {
            return Err("the ffmpeg component did not finalize the file in time".to_string());
        }
        let status = status.expect("set unless timed out");
        if !status.success() {
            return Err(format!(
                "the ffmpeg component exited with {status}: {}",
                String::from_utf8_lossy(&stderr_tail).trim()
            ));
        }
        if self.paths.is_empty() {
            return Ok(Vec::new()); // a stream sink produces no files
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
        // Belt-and-braces: a dropped-unfinished sink must not leak a child or
        // strand a lane parked in accept().
        self.connect_cancel.store(true, Ordering::Relaxed);
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

    #[test]
    fn scrub_secret_removes_the_publish_url_from_stderr() {
        let url = b"rtmp://live.twitch.tv/app/live_abc123_secretkey";
        let stderr = b"[flv @ 0x1] Error opening output rtmp://live.twitch.tv/app/live_abc123_secretkey: I/O error\n";
        let scrubbed = scrub_secret(stderr, url);
        let text = String::from_utf8_lossy(&scrubbed);
        assert!(!text.contains("secretkey"), "key leaked: {text}");
        assert!(text.contains("[redacted]"));
        assert!(text.contains("Error opening output"), "the rest survives");
    }

    #[test]
    fn scrub_secret_is_a_noop_without_the_secret() {
        assert_eq!(scrub_secret(b"clean line", b"nope"), b"clean line");
        assert_eq!(scrub_secret(b"anything", b""), b"anything");
    }

    fn rc(mode: RcMode) -> RateControl {
        RateControl {
            mode,
            bitrate_kbps: 8000,
            cq: 23,
        }
    }

    #[test]
    fn x264_pins_yuv420p_and_maps_rate_control() {
        let cbr = video_args("libx264", &rc(RcMode::Cbr), EncPreset::Balanced, 120);
        assert!(cbr.windows(2).any(|w| w == ["-b:v", "8000k"]));
        assert!(cbr.windows(2).any(|w| w == ["-minrate", "8000k"]));
        assert!(cbr.windows(2).any(|w| w == ["-g", "120"]));
        assert!(cbr.windows(2).any(|w| w == ["-pix_fmt", "yuv420p"]));

        let cqp = video_args("libx264", &rc(RcMode::Cqp), EncPreset::Balanced, 120);
        assert!(cqp.windows(2).any(|w| w == ["-crf", "23"]));
        assert!(!cqp.iter().any(|arg| arg == "-b:v"));
    }

    #[test]
    fn nvenc_uses_its_rc_modes_and_no_forced_pix_fmt() {
        let cbr = video_args("h264_nvenc", &rc(RcMode::Cbr), EncPreset::Balanced, 60);
        assert!(cbr.windows(2).any(|w| w == ["-rc", "cbr"]));
        assert!(!cbr.iter().any(|arg| arg == "-pix_fmt"), "hw negotiates");

        let cqp = video_args("hevc_nvenc", &rc(RcMode::Cqp), EncPreset::Balanced, 60);
        assert!(cqp.windows(2).any(|w| w == ["-rc", "constqp"]));
        assert!(cqp.windows(2).any(|w| w == ["-qp", "23"]));
    }

    #[test]
    fn presets_change_every_family_knob() {
        let quality = video_args("libx264", &rc(RcMode::Cqp), EncPreset::Quality, 60);
        assert!(quality.windows(2).any(|w| w == ["-preset", "slow"]));
        let perf = video_args("libx264", &rc(RcMode::Cqp), EncPreset::Performance, 60);
        assert!(perf.windows(2).any(|w| w == ["-preset", "ultrafast"]));

        let nv_quality = video_args("h264_nvenc", &rc(RcMode::Cbr), EncPreset::Quality, 60);
        assert!(nv_quality.windows(2).any(|w| w == ["-preset", "p7"]));
        let nv_perf = video_args("h264_nvenc", &rc(RcMode::Cbr), EncPreset::Performance, 60);
        assert!(nv_perf.windows(2).any(|w| w == ["-preset", "p2"]));

        let amf = video_args("h264_amf", &rc(RcMode::Cbr), EncPreset::Quality, 60);
        assert!(amf.windows(2).any(|w| w == ["-quality", "quality"]));
        let qsv = video_args("h264_qsv", &rc(RcMode::Cbr), EncPreset::Performance, 60);
        assert!(qsv.windows(2).any(|w| w == ["-preset", "veryfast"]));
    }

    #[test]
    fn vaapi_brings_up_its_device_and_upload_chain() {
        let global = global_args("h264_vaapi");
        // Bare `vaapi=va` (no node path) so it matches the smoke-test device;
        // ffmpeg auto-selects the render node.
        assert!(global
            .windows(2)
            .any(|w| w == ["-init_hw_device", "vaapi=va"]));
        assert!(!global.iter().any(|arg| arg.contains("renderD")));
        let video = video_args("h264_vaapi", &rc(RcMode::Cbr), EncPreset::Balanced, 60);
        assert!(video
            .windows(2)
            .any(|w| w == ["-vf", "format=nv12,hwupload"]));
        assert!(global_args("h264_nvenc").is_empty());
    }

    #[test]
    fn videotoolbox_maps_cq_onto_its_quality_scale() {
        let cqp = video_args(
            "h264_videotoolbox",
            &rc(RcMode::Cqp),
            EncPreset::Balanced,
            60,
        );
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
