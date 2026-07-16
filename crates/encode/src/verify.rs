//! CAP-N46: the recording integrity verifier — a fast local pass that
//! answers "did the show land on disk intact?" minutes after the stop, not
//! during next week's edit.
//!
//! The owned `.frec` format gets the deep treatment (its chunk layout makes
//! every check exact): container structure, finalization trailer, video
//! frame continuity, per-track audio sample continuity, A/V interleave skew,
//! and duration vs the recorder's wall-clock. Wire containers get what
//! ffmpeg can honestly answer — a banner/structure probe, a decode error
//! scan (full or tail), and the duration check; the frec-only checks report
//! themselves as skipped rather than pretending.

use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use std::time::Duration;

use serde::Serialize;

use crate::decode::probe_media;
use crate::ffmpeg::{command, run_with_timeout, Ffmpeg};
use crate::freally_video::{FrecChunk, FrecReader};

/// One check's outcome. `verdict` of a report is the worst of its checks
/// (Fail > Warn > Pass; Skipped never worsens a verdict).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CheckStatus {
    Pass,
    Warn,
    Fail,
    Skipped,
}

/// One named check: `id` is a stable key the UI localizes; `detail` carries
/// the numbers/messages (plain English — file-adjacent content).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyCheck {
    pub id: &'static str,
    pub status: CheckStatus,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyReport {
    pub verdict: CheckStatus,
    pub checks: Vec<VerifyCheck>,
}

impl VerifyReport {
    fn from_checks(checks: Vec<VerifyCheck>) -> Self {
        let verdict =
            checks
                .iter()
                .map(|check| check.status)
                .fold(CheckStatus::Pass, |worst, status| {
                    match (worst, status) {
                        (_, CheckStatus::Fail) | (CheckStatus::Fail, _) => CheckStatus::Fail,
                        (_, CheckStatus::Warn) | (CheckStatus::Warn, _) => CheckStatus::Warn,
                        _ => worst, // Pass/Skipped never worsen
                    }
                });
        VerifyReport { verdict, checks }
    }
}

/// The recorder writes 10 ms blocks (480 frames at 48 kHz); a gap of more
/// than two blocks in a track's sample positions is a real discontinuity.
const AUDIO_GAP_SAMPLES: u64 = 480 * 2;
/// A/V skew beyond this at any point of the walk is worth flagging — the
/// writer interleaves close to real time.
const INTERLEAVE_SKEW_SECS: f64 = 2.0;
/// Duration tolerance vs the recorder's wall-clock: 2% + half a second.
fn duration_ok(actual: f64, expected: f64) -> bool {
    (actual - expected).abs() <= expected * 0.02 + 0.5
}

/// Deep-verify an owned `.frec` recording. `expected_secs` is the recorded
/// wall-clock when the caller knows it (post-record); `None` skips that
/// check honestly (on-demand verification of an old file).
pub fn verify_frec(path: &Path, expected_secs: Option<f64>) -> Result<VerifyReport, String> {
    let mut checks: Vec<VerifyCheck> = Vec::new();

    let mut reader = match FrecReader::open(path) {
        Ok(reader) => reader,
        Err(err) => {
            checks.push(VerifyCheck {
                id: "container",
                status: CheckStatus::Fail,
                detail: format!("the file does not open as freally-video: {err}"),
            });
            return Ok(VerifyReport::from_checks(checks));
        }
    };
    let spec = *reader.spec();
    let fps = f64::from(spec.fps_num) / f64::from(spec.fps_den.max(1));

    // Finalization trailer: a crashed/power-cut recording has no index —
    // it still plays to the last complete chunk, but say so.
    checks.push(match has_trailer(path) {
        Ok(true) => VerifyCheck {
            id: "container",
            status: CheckStatus::Pass,
            detail: "header, chunk stream and finalization trailer are intact".into(),
        },
        Ok(false) => VerifyCheck {
            id: "container",
            status: CheckStatus::Warn,
            detail: "the recording was never finalized (crash or power loss?) — it plays to \
                     the last complete chunk"
                .into(),
        },
        Err(err) => VerifyCheck {
            id: "container",
            status: CheckStatus::Fail,
            detail: format!("could not inspect the trailer: {err}"),
        },
    });

    // The full walk: every chunk decodes; frame indices stay contiguous;
    // per-track audio positions stay contiguous; A/V skew stays bounded.
    let mut frames = 0u64;
    let mut index_breaks = 0u64;
    let mut audio_next: std::collections::BTreeMap<u8, u64> = Default::default();
    let mut audio_gaps = 0u64;
    let mut max_skew: f64 = 0.0;
    let mut torn: Option<String> = None;
    loop {
        match reader.next_chunk() {
            Ok(None) => break,
            Ok(Some(FrecChunk::Video { frame_index, .. })) => {
                if frame_index != frames {
                    index_breaks += 1;
                }
                frames += 1;
            }
            Ok(Some(FrecChunk::Audio {
                track,
                sample_pos,
                samples,
            })) => {
                let next = audio_next.entry(track).or_insert(0);
                if sample_pos > *next + AUDIO_GAP_SAMPLES {
                    audio_gaps += 1;
                }
                *next = sample_pos + (samples.len() / 2) as u64;
                if fps > 0.0 {
                    let video_time = frames as f64 / fps;
                    let audio_time = *next as f64 / f64::from(spec.sample_rate.max(1));
                    max_skew = max_skew.max((video_time - audio_time).abs());
                }
            }
            Err(err) => {
                torn = Some(err.to_string());
                break;
            }
        }
    }

    checks.push(match (&torn, index_breaks) {
        (Some(err), _) => VerifyCheck {
            id: "video-continuity",
            status: CheckStatus::Fail,
            detail: format!("the chunk stream breaks after frame {frames}: {err}"),
        },
        (None, 0) => VerifyCheck {
            id: "video-continuity",
            status: CheckStatus::Pass,
            detail: format!("{frames} frames, indices contiguous"),
        },
        (None, breaks) => VerifyCheck {
            id: "video-continuity",
            status: CheckStatus::Fail,
            detail: format!("{breaks} frame-index break(s) across {frames} frames"),
        },
    });

    checks.push(if spec.audio_tracks == 0 {
        VerifyCheck {
            id: "audio-continuity",
            status: CheckStatus::Skipped,
            detail: "no audio tracks in this file".into(),
        }
    } else if audio_gaps == 0 {
        VerifyCheck {
            id: "audio-continuity",
            status: CheckStatus::Pass,
            detail: format!(
                "{} track(s), sample positions contiguous",
                audio_next.len().max(spec.audio_tracks as usize)
            ),
        }
    } else {
        VerifyCheck {
            id: "audio-continuity",
            status: CheckStatus::Warn,
            detail: format!("{audio_gaps} audio gap(s) larger than two blocks"),
        }
    });

    checks.push(if spec.audio_tracks == 0 || frames == 0 {
        VerifyCheck {
            id: "av-interleave",
            status: CheckStatus::Skipped,
            detail: "needs both video and audio".into(),
        }
    } else if max_skew <= INTERLEAVE_SKEW_SECS {
        VerifyCheck {
            id: "av-interleave",
            status: CheckStatus::Pass,
            detail: format!("worst A/V skew {max_skew:.2} s"),
        }
    } else {
        VerifyCheck {
            id: "av-interleave",
            status: CheckStatus::Warn,
            detail: format!(
                "audio and video drift {max_skew:.2} s apart in the stream (threshold \
                 {INTERLEAVE_SKEW_SECS:.0} s)"
            ),
        }
    });

    let actual_secs = if fps > 0.0 { frames as f64 / fps } else { 0.0 };
    checks.push(match expected_secs {
        Some(expected) if duration_ok(actual_secs, expected) => VerifyCheck {
            id: "duration",
            status: CheckStatus::Pass,
            detail: format!("{actual_secs:.1} s recorded, {expected:.1} s expected"),
        },
        Some(expected) => VerifyCheck {
            id: "duration",
            status: CheckStatus::Warn,
            detail: format!(
                "{actual_secs:.1} s in the file but the recorder ran {expected:.1} s — \
                 frames are missing"
            ),
        },
        None => VerifyCheck {
            id: "duration",
            status: CheckStatus::Skipped,
            detail: format!("{actual_secs:.1} s in the file (no wall-clock to compare)"),
        },
    });

    Ok(VerifyReport::from_checks(checks))
}

/// Whether the file ends with the finalization trailer (`index_offset u64`
/// + `"FRECIDX1"`).
fn has_trailer(path: &Path) -> Result<bool, String> {
    let mut file = std::fs::File::open(path).map_err(|err| err.to_string())?;
    let len = file.seek(SeekFrom::End(0)).map_err(|err| err.to_string())?;
    if len < 16 {
        return Ok(false);
    }
    file.seek(SeekFrom::End(-8))
        .map_err(|err| err.to_string())?;
    let mut magic = [0u8; 8];
    file.read_exact(&mut magic).map_err(|err| err.to_string())?;
    Ok(&magic == b"FRECIDX1")
}

/// Verify a wire-container recording with what ffmpeg can honestly answer:
/// the banner/structure probe, a decode error scan (`deep` = the whole file;
/// otherwise the last ten seconds — fast, catches truncated tails), and the
/// duration check. The frec-only checks report themselves skipped.
pub fn verify_wire(
    ffmpeg: &Ffmpeg,
    path: &Path,
    expected_secs: Option<f64>,
    deep: bool,
) -> Result<VerifyReport, String> {
    let mut checks: Vec<VerifyCheck> = Vec::new();

    let media = match probe_media(ffmpeg, path) {
        Ok(media) => media,
        Err(err) => {
            checks.push(VerifyCheck {
                id: "container",
                status: CheckStatus::Fail,
                detail: format!("the container does not probe: {err}"),
            });
            return Ok(VerifyReport::from_checks(checks));
        }
    };
    checks.push(VerifyCheck {
        id: "container",
        status: CheckStatus::Pass,
        detail: format!(
            "{}x{} video stream{}",
            media.width,
            media.height,
            if media.has_audio { " + audio" } else { "" }
        ),
    });

    // The decode scan: ffmpeg reports every bitstream error on stderr.
    let mut cmd = command(ffmpeg);
    cmd.args(["-hide_banner", "-v", "error"]);
    if !deep {
        cmd.args(["-sseof", "-10"]);
    }
    cmd.arg("-i").arg(path);
    cmd.args(["-f", "null", "-"]);
    let scan = run_with_timeout(cmd, Duration::from_secs(if deep { 1800 } else { 60 }))?;
    let stderr = String::from_utf8_lossy(&scan.stderr);
    let errors: Vec<&str> = stderr
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    checks.push(if errors.is_empty() {
        VerifyCheck {
            id: "video-continuity",
            status: CheckStatus::Pass,
            detail: if deep {
                "full decode scan — no bitstream errors".into()
            } else {
                "tail decode scan (last 10 s) — no bitstream errors".into()
            },
        }
    } else {
        VerifyCheck {
            id: "video-continuity",
            status: CheckStatus::Fail,
            detail: format!(
                "{} decode error(s), first: {}",
                errors.len(),
                errors[0].chars().take(160).collect::<String>()
            ),
        }
    });

    checks.push(VerifyCheck {
        id: "audio-continuity",
        status: CheckStatus::Skipped,
        detail: "per-track sample continuity is a freally-video (.frec) check".into(),
    });
    checks.push(VerifyCheck {
        id: "av-interleave",
        status: CheckStatus::Skipped,
        detail: "interleave inspection is a freally-video (.frec) check".into(),
    });

    let actual = media.duration_secs.map(f64::from);
    checks.push(match (actual, expected_secs) {
        (Some(actual), Some(expected)) if duration_ok(actual, expected) => VerifyCheck {
            id: "duration",
            status: CheckStatus::Pass,
            detail: format!("{actual:.1} s in the file, {expected:.1} s expected"),
        },
        (Some(actual), Some(expected)) => VerifyCheck {
            id: "duration",
            status: CheckStatus::Warn,
            detail: format!("{actual:.1} s in the file but the recorder ran {expected:.1} s"),
        },
        (Some(actual), None) => VerifyCheck {
            id: "duration",
            status: CheckStatus::Skipped,
            detail: format!("{actual:.1} s in the file (no wall-clock to compare)"),
        },
        (None, _) => VerifyCheck {
            id: "duration",
            status: CheckStatus::Warn,
            detail: "the container states no duration — an unfinalized file".into(),
        },
    });

    Ok(VerifyReport::from_checks(checks))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::freally_video::{FrecSpec, FrecWriter, PixelFormat};

    fn temp_file(tag: &str) -> std::path::PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "fcap-verify-{}-{nanos}-{tag}.frec",
            std::process::id()
        ))
    }

    fn spec() -> FrecSpec {
        FrecSpec {
            width: 16,
            height: 8,
            fps_num: 10,
            fps_den: 1,
            pixel_format: PixelFormat::Rgba8,
            audio_tracks: 1,
            sample_rate: 48_000,
            alpha: false,
        }
    }

    /// Write `frames` video frames with contiguous audio; return the path.
    fn write_clean(tag: &str, frames: u32) -> std::path::PathBuf {
        let path = temp_file(tag);
        let spec = spec();
        let mut writer = FrecWriter::create(&path, spec).expect("create");
        let frame = vec![9u8; 16 * 8 * 4];
        let block = vec![0.25f32; 9_600]; // 4800 frames = 0.1 s per video frame
        for index in 0..frames {
            writer.write_frame(&frame).expect("frame");
            writer
                .write_audio(0, u64::from(index) * 4_800, &block)
                .expect("audio");
        }
        writer.finish().expect("finish");
        path
    }

    #[test]
    fn a_clean_finalized_recording_passes_every_check() {
        let path = write_clean("clean", 20); // 2.0 s at 10 fps
        let report = verify_frec(&path, Some(2.0)).expect("verifies");
        assert_eq!(report.verdict, CheckStatus::Pass, "{:#?}", report.checks);
        let _ = std::fs::remove_file(&path);
    }

    /// Seeded corruption 1: a truncated tail (crash / power loss) — the
    /// trailer is gone and the walk ends early → warnings, never a panic.
    #[test]
    fn a_truncated_recording_warns_about_finalization() {
        let path = write_clean("truncated", 20);
        let full = std::fs::read(&path).expect("read");
        std::fs::write(&path, &full[..full.len() - full.len() / 3]).expect("truncate");
        let report = verify_frec(&path, Some(2.0)).expect("verifies");
        assert_ne!(report.verdict, CheckStatus::Pass);
        assert!(
            report
                .checks
                .iter()
                .any(|c| c.id == "container" && c.status == CheckStatus::Warn),
            "the missing trailer is called out: {:#?}",
            report.checks
        );
        assert!(
            report
                .checks
                .iter()
                .any(|c| c.id == "duration" && c.status == CheckStatus::Warn),
            "the missing time is called out: {:#?}",
            report.checks
        );
        let _ = std::fs::remove_file(&path);
    }

    /// Seeded corruption 2: flipped bytes inside the chunk stream — the walk
    /// reports the break instead of pretending the file is whole.
    #[test]
    fn corrupted_chunk_bytes_fail_the_walk() {
        let path = write_clean("corrupt", 20);
        let mut bytes = std::fs::read(&path).expect("read");
        let mid = bytes.len() / 2;
        for byte in &mut bytes[mid..mid + 64] {
            *byte ^= 0xA5;
        }
        std::fs::write(&path, &bytes).expect("write");
        let report = verify_frec(&path, Some(2.0)).expect("verifies");
        assert_eq!(report.verdict, CheckStatus::Fail, "{:#?}", report.checks);
        let _ = std::fs::remove_file(&path);
    }

    /// Seeded corruption 3: an audio hole — positions jump by a second.
    #[test]
    fn an_audio_gap_warns_on_continuity() {
        let path = temp_file("gap");
        let spec = spec();
        let mut writer = FrecWriter::create(&path, spec).expect("create");
        let frame = vec![9u8; 16 * 8 * 4];
        let block = vec![0.25f32; 9_600];
        writer.write_frame(&frame).expect("frame");
        writer.write_audio(0, 0, &block).expect("audio");
        writer.write_frame(&frame).expect("frame");
        // The next block lands a full second late — a real hole.
        writer
            .write_audio(0, 4_800 + 48_000, &block)
            .expect("audio");
        writer.finish().expect("finish");

        let report = verify_frec(&path, None).expect("verifies");
        assert!(
            report
                .checks
                .iter()
                .any(|c| c.id == "audio-continuity" && c.status == CheckStatus::Warn),
            "{:#?}",
            report.checks
        );
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn duration_tolerance_is_two_percent_plus_half_a_second() {
        assert!(duration_ok(100.0, 100.0));
        assert!(duration_ok(98.0, 100.0));
        assert!(!duration_ok(96.0, 100.0));
        assert!(
            duration_ok(1.4, 1.0),
            "short files get the flat half-second"
        );
    }
}
