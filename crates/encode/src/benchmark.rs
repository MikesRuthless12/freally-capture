//! CAP-N52 encoder benchmark: actually *run* short encode ladders on this
//! machine and measure what it can sustain.
//!
//! The first-run wizard (TASK-905) probes capabilities and applies
//! heuristics; this measures. Each case encodes a few seconds of a
//! synthetic `testsrc2` picture through the **real per-family encoder
//! arguments** ([`video_args`]) into a null sink and takes achieved fps
//! from the wall clock. Short ladders include encoder startup, so the
//! numbers are deliberately conservative — a setting that clears the
//! headroom bar here has real margin on show day. Fully offline: the only
//! process involved is the labeled ffmpeg component, and nothing is
//! written or sent anywhere.
//!
//! The ladder covers **every H.264 encoder the catalog offers on this
//! machine** (streaming and recording default to H.264); an entry that
//! fails records its error string, so a gap is documented rather than
//! silently skipped.

use std::time::{Duration, Instant};

use crate::encoder::{Catalog, VideoCodec};
use crate::ffmpeg::{command, run_with_timeout, Ffmpeg};
use crate::mux::{video_args, EncPreset, RateControl, RcMode};

/// Seconds of synthetic video each case encodes.
const CASE_SECONDS: u32 = 3;
/// Hard ceiling per case — a wedged driver must not hang the wizard.
const CASE_TIMEOUT: Duration = Duration::from_secs(60);

/// One rung of the ladder: encoder × preset × resolution × fps.
#[derive(Debug, Clone, PartialEq)]
pub struct BenchCase {
    pub encoder_id: String,
    pub encoder_label: String,
    pub hardware: bool,
    pub preset: EncPreset,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
}

/// One rung's measured outcome.
#[derive(Debug, Clone)]
pub struct BenchResult {
    pub case: BenchCase,
    /// Achieved encode fps (Ok) or why the rung failed (Err) — the honest
    /// record of a family this machine offers but cannot actually run.
    pub outcome: Result<f32, String>,
}

impl BenchResult {
    /// Achieved fps over the target — >1.0 = faster than realtime.
    pub fn headroom(&self) -> Option<f32> {
        self.outcome
            .as_ref()
            .ok()
            .map(|achieved| achieved / self.case.fps as f32)
    }
}

/// The nominal CBR bitrate a rung encodes at (also the recommendation's
/// suggested streaming bitrate) — the usual platform guidance per size.
pub fn bitrate_for(width: u32, height: u32, fps: u32) -> u32 {
    match (width, height) {
        (w, h) if w >= 2560 || h >= 1440 => {
            if fps > 30 {
                9_000
            } else {
                6_500
            }
        }
        _ => {
            if fps > 30 {
                6_000
            } else {
                4_500
            }
        }
    }
}

/// Build the ladder: every offered H.264 encoder (skipping only entries
/// already proven broken) × the three presets × the two rungs that matter
/// for a streaming canvas (1080p60 and 1440p60).
pub fn ladder(catalog: &Catalog) -> Vec<BenchCase> {
    let mut cases = Vec::new();
    for desc in &catalog.encoders {
        if desc.codec != VideoCodec::H264 || desc.verified == Some(false) {
            continue;
        }
        for preset in [
            EncPreset::Quality,
            EncPreset::Balanced,
            EncPreset::Performance,
        ] {
            for (width, height, fps) in [(1920, 1080, 60), (2560, 1440, 60)] {
                cases.push(BenchCase {
                    encoder_id: desc.id.clone(),
                    encoder_label: desc.label.clone(),
                    hardware: desc.hardware,
                    preset,
                    width,
                    height,
                    fps,
                });
            }
        }
    }
    cases
}

/// Encode one rung and measure achieved fps from the wall clock.
pub fn run_case(ffmpeg: &Ffmpeg, case: &BenchCase) -> Result<f32, String> {
    let frames = case.fps * CASE_SECONDS;
    let mut cmd = command(ffmpeg);
    cmd.args(["-hide_banner", "-v", "error", "-f", "lavfi", "-i"]);
    cmd.arg(format!(
        "testsrc2=size={}x{}:rate={}",
        case.width, case.height, case.fps
    ));
    if case.encoder_id.ends_with("_vaapi") {
        // VAAPI encodes hardware frames — same bring-up as the smoke test.
        cmd.args([
            "-init_hw_device",
            "vaapi=va",
            "-filter_hw_device",
            "va",
            "-vf",
            "format=nv12,hwupload",
        ]);
    }
    let rate_control = RateControl {
        mode: RcMode::Cbr,
        bitrate_kbps: bitrate_for(case.width, case.height, case.fps),
        cq: 23,
    };
    cmd.args(["-frames:v", &frames.to_string()]);
    cmd.args(video_args(
        &case.encoder_id,
        &rate_control,
        case.preset,
        case.fps * 2,
    ));
    cmd.args(["-f", "null", "-"]);
    let started = Instant::now();
    let output = run_with_timeout(cmd, CASE_TIMEOUT)?;
    let wall = started.elapsed().as_secs_f32().max(1e-3);
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let reason = stderr
            .lines()
            .find(|line| !line.trim().is_empty())
            .unwrap_or("the encoder failed")
            .trim();
        return Err(reason.chars().take(200).collect());
    }
    Ok(frames as f32 / wall)
}

/// What the wizard recommends after the ladder ran.
#[derive(Debug, Clone, PartialEq)]
pub struct Recommendation {
    pub encoder_id: String,
    pub encoder_label: String,
    pub preset: EncPreset,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub bitrate_kbps: u32,
    /// Measured achieved-fps ÷ target — the margin behind the pick.
    pub headroom: f32,
}

fn preset_rank(preset: EncPreset) -> u8 {
    match preset {
        EncPreset::Quality => 2,
        EncPreset::Balanced => 1,
        EncPreset::Performance => 0,
    }
}

/// Pick the best measured rung: prefer ≥1.5× realtime headroom (falling
/// back to ≥1.1× when nothing clears it), then the biggest picture, the
/// best preset, hardware over software (frees the CPU for the show), and
/// finally raw speed. `None` = nothing sustains realtime — an honest
/// "lower your canvas" answer, never a guess.
pub fn recommend(results: &[BenchResult]) -> Option<Recommendation> {
    let pick = |bar: f32| {
        let mut best: Option<(&BenchResult, f32)> = None;
        for result in results {
            let Some(headroom) = result.headroom() else {
                continue;
            };
            if headroom < bar {
                continue;
            }
            let better = match best {
                None => true,
                Some((current, current_headroom)) => {
                    let a = &result.case;
                    let b = &current.case;
                    (
                        a.width * a.height,
                        a.fps,
                        preset_rank(a.preset),
                        a.hardware,
                        headroom,
                    ) > (
                        b.width * b.height,
                        b.fps,
                        preset_rank(b.preset),
                        b.hardware,
                        current_headroom,
                    )
                }
            };
            if better {
                best = Some((result, headroom));
            }
        }
        best
    };
    let (result, headroom) = pick(1.5).or_else(|| pick(1.1))?;
    let case = &result.case;
    Some(Recommendation {
        encoder_id: case.encoder_id.clone(),
        encoder_label: case.encoder_label.clone(),
        preset: case.preset,
        width: case.width,
        height: case.height,
        fps: case.fps,
        bitrate_kbps: bitrate_for(case.width, case.height, case.fps),
        headroom,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn case(id: &str, hardware: bool, preset: EncPreset, width: u32, height: u32) -> BenchCase {
        BenchCase {
            encoder_id: id.to_string(),
            encoder_label: id.to_string(),
            hardware,
            preset,
            width,
            height,
            fps: 60,
        }
    }

    fn ok(case: BenchCase, achieved: f32) -> BenchResult {
        BenchResult {
            case,
            outcome: Ok(achieved),
        }
    }

    #[test]
    fn recommends_the_biggest_sustained_picture_at_the_best_preset() {
        let results = vec![
            // 1440p only sustains on Performance — good headroom.
            ok(
                case("h264_nvenc", true, EncPreset::Performance, 2560, 1440),
                120.0,
            ),
            // 1440p Quality is too slow (below the 1.1 bar).
            ok(
                case("h264_nvenc", true, EncPreset::Quality, 2560, 1440),
                55.0,
            ),
            // 1080p Quality flies.
            ok(
                case("h264_nvenc", true, EncPreset::Quality, 1920, 1080),
                240.0,
            ),
            // Software also clears 1080p but must lose to hardware.
            ok(
                case("libx264", false, EncPreset::Quality, 1920, 1080),
                150.0,
            ),
        ];
        let rec = recommend(&results).expect("something sustains");
        // The biggest sustained picture wins even at a faster preset.
        assert_eq!(rec.width, 2560);
        assert_eq!(rec.preset, EncPreset::Performance);
        assert_eq!(rec.encoder_id, "h264_nvenc");
        assert_eq!(rec.bitrate_kbps, 9_000);
        assert!((rec.headroom - 2.0).abs() < 1e-3);
    }

    #[test]
    fn falls_back_below_the_comfort_bar_and_admits_total_defeat() {
        // Nothing reaches 1.5×, one squeaks past 1.1× — recommend it.
        let squeaky = vec![ok(
            case("libx264", false, EncPreset::Performance, 1920, 1080),
            70.0,
        )];
        let rec = recommend(&squeaky).expect("1.1× fallback");
        assert_eq!(rec.encoder_id, "libx264");
        // Nothing sustains realtime at all → an honest None.
        let hopeless = vec![
            ok(case("libx264", false, EncPreset::Quality, 1920, 1080), 30.0),
            BenchResult {
                case: case("h264_amf", true, EncPreset::Quality, 1920, 1080),
                outcome: Err("no AMD device".to_string()),
            },
        ];
        assert!(recommend(&hopeless).is_none());
    }

    #[test]
    fn ladder_covers_every_offered_h264_family_and_documents_none_silently() {
        use crate::encoder::catalog_for;
        // A Windows/NVIDIA machine offers NVENC + the software fallbacks.
        let catalog = Catalog {
            gpus: Vec::new(),
            encoders: catalog_for(
                "windows",
                &[crate::hardware::GpuInfo {
                    name: "RTX".to_string(),
                    vendor: crate::hardware::GpuVendor::Nvidia,
                    backend: "vulkan".to_string(),
                }],
                false,
            ),
        };
        let cases = ladder(&catalog);
        let ids: std::collections::HashSet<&str> =
            cases.iter().map(|case| case.encoder_id.as_str()).collect();
        assert!(ids.contains("h264_nvenc"));
        assert!(ids.contains("libx264"));
        // 3 presets × 2 rungs per offered H.264 encoder — nothing sampled away.
        for id in ids {
            assert_eq!(
                cases.iter().filter(|case| case.encoder_id == id).count(),
                6,
                "every family runs the full grid ({id})"
            );
        }
    }
}
