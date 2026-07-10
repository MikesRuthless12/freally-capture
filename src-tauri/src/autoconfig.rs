//! First-run auto-configuration (TASK-905).
//!
//! Probe the machine, then pick encoder / fps / bitrate by **deterministic
//! rules** — no ML, per the charter, and no benchmark. The rules are written out
//! below so a user can disagree with them, and every suggestion carries a reason
//! key the UI renders, so nothing is a black box.
//!
//! What this is not: a benchmark. It reads what the machine *has*, not what it
//! can sustain. Measuring real encode throughput is CAP-N52, and it belongs
//! behind an explicit "run the benchmark" button — not on a first launch.

use fcap_encode::encoder::{Catalog, EncoderDesc, EncoderEngine};
use serde::Serialize;

/// The canvas every suggestion targets. 1080p is what the compositor's perf
/// budget is stated against, and what a first-time streamer's platform expects.
const CANVAS_WIDTH: u32 = 1920;
const CANVAS_HEIGHT: u32 = 1080;

/// Hardware encoders leave the CPU free, so 60 fps is safe. Software x264 at
/// 1080p60 will drop frames on a mid-range CPU, so software suggests 30.
const HARDWARE_FPS: u32 = 60;
const SOFTWARE_FPS: u32 = 30;

/// Twitch's practical ceiling for non-partners is 6000 kbps, and it is a sane
/// default everywhere else. Software at 30 fps needs less to look the same.
const HARDWARE_BITRATE_KBPS: u32 = 6000;
const SOFTWARE_BITRATE_KBPS: u32 = 4500;
/// Below this many physical cores, software x264 at 1080p is not worth offering
/// at 30 fps either — suggest 720p-class bitrate and say why.
const LOW_CORE_COUNT: usize = 4;
const LOW_CORE_BITRATE_KBPS: u32 = 3000;

/// One suggested setting plus the reason for it. `reason` is an i18n key, not a
/// sentence — the UI translates it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoConfig {
    /// The `EncoderDesc::id` to select, or `"x264"` when nothing else qualifies.
    pub encoder_id: String,
    pub encoder_label: String,
    /// Whether the chosen encoder runs on the GPU.
    pub hardware: bool,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub bitrate_kbps: u32,
    /// i18n key explaining the encoder choice.
    pub encoder_reason: String,
    /// i18n key explaining the fps/bitrate choice.
    pub quality_reason: String,
    /// What the probe actually found, for the "why" panel.
    pub gpus: Vec<String>,
    pub physical_cores: usize,
}

/// Rank hardware families by how well they hold 1080p60 while the GPU is also
/// compositing. NVENC's dedicated silicon wins; VAAPI is last because its
/// quality varies wildly by driver. Keyed off the engine enum rather than the
/// id string, so a renamed id cannot silently demote an encoder.
fn hardware_rank(engine: EncoderEngine) -> u8 {
    match engine {
        EncoderEngine::Nvenc => 0,
        EncoderEngine::VideoToolbox => 1,
        EncoderEngine::QuickSync => 2,
        EncoderEngine::Amf => 3,
        EncoderEngine::Vaapi => 4,
        EncoderEngine::Software => 5,
    }
}

/// The best encoder this machine can actually use.
///
/// `verified == Some(false)` means the driver refused it during the ffmpeg smoke
/// test. Suggesting it would hand a first-time user a recording that fails on the
/// first frame, so it is skipped even though it is present.
fn pick_encoder(encoders: &[EncoderDesc]) -> Option<&EncoderDesc> {
    encoders
        .iter()
        .filter(|encoder| encoder.hardware && encoder.verified != Some(false))
        .min_by_key(|encoder| hardware_rank(encoder.engine))
        .or_else(|| {
            encoders
                .iter()
                .find(|encoder| !encoder.hardware && encoder.verified != Some(false))
        })
}

/// Deterministic suggestion from a probed catalog and a core count. Split out
/// from the command so the rules are testable without a GPU.
pub fn suggest_from(catalog: &Catalog, physical_cores: usize) -> AutoConfig {
    let gpus = catalog.gpus.iter().map(|gpu| gpu.name.clone()).collect();

    match pick_encoder(&catalog.encoders) {
        Some(encoder) if encoder.hardware => AutoConfig {
            encoder_id: encoder.id.clone(),
            encoder_label: encoder.label.clone(),
            hardware: true,
            width: CANVAS_WIDTH,
            height: CANVAS_HEIGHT,
            fps: HARDWARE_FPS,
            bitrate_kbps: HARDWARE_BITRATE_KBPS,
            encoder_reason: "autoconfig-reason-hardware".to_owned(),
            quality_reason: "autoconfig-reason-quality-hardware".to_owned(),
            gpus,
            physical_cores,
        },
        found => {
            let low_cores = physical_cores < LOW_CORE_COUNT;
            AutoConfig {
                encoder_id: found.map_or_else(|| "x264".to_owned(), |e| e.id.clone()),
                encoder_label: found.map_or_else(|| "x264".to_owned(), |e| e.label.clone()),
                hardware: false,
                width: CANVAS_WIDTH,
                height: CANVAS_HEIGHT,
                fps: SOFTWARE_FPS,
                bitrate_kbps: if low_cores {
                    LOW_CORE_BITRATE_KBPS
                } else {
                    SOFTWARE_BITRATE_KBPS
                },
                encoder_reason: "autoconfig-reason-software".to_owned(),
                quality_reason: if low_cores {
                    "autoconfig-reason-quality-low-cores".to_owned()
                } else {
                    "autoconfig-reason-quality-software".to_owned()
                },
                gpus,
                physical_cores,
            }
        }
    }
}

/// Probe this machine and suggest defaults. Reads hardware; changes nothing.
#[tauri::command]
pub fn autoconfig_suggest() -> AutoConfig {
    let cores = sysinfo::System::new()
        .physical_core_count()
        .unwrap_or(1)
        .max(1);
    suggest_from(&Catalog::detect(), cores)
}

#[cfg(test)]
mod tests {
    use super::*;
    use fcap_encode::encoder::VideoCodec;

    fn encoder(id: &str, engine: EncoderEngine, verified: Option<bool>) -> EncoderDesc {
        EncoderDesc {
            id: id.to_owned(),
            codec: VideoCodec::H264,
            engine,
            label: id.to_owned(),
            hardware: engine != EncoderEngine::Software,
            note: String::new(),
            verified,
        }
    }

    fn catalog(encoders: Vec<EncoderDesc>) -> Catalog {
        Catalog {
            gpus: Vec::new(),
            encoders,
        }
    }

    #[test]
    fn hardware_beats_software_and_nvenc_beats_the_rest() {
        let config = suggest_from(
            &catalog(vec![
                encoder("x264", EncoderEngine::Software, None),
                encoder("vaapi_h264", EncoderEngine::Vaapi, None),
                encoder("nvenc_h264", EncoderEngine::Nvenc, None),
            ]),
            16,
        );
        assert_eq!(config.encoder_id, "nvenc_h264");
        assert!(config.hardware);
        assert_eq!(config.fps, 60);
        assert_eq!(config.bitrate_kbps, 6000);
    }

    /// `verified == Some(false)` means the driver refused it. Suggesting it would
    /// hand a first-time user a recording that dies on the first frame.
    #[test]
    fn an_encoder_the_driver_refused_is_never_suggested() {
        let config = suggest_from(
            &catalog(vec![
                encoder("nvenc_h264", EncoderEngine::Nvenc, Some(false)),
                encoder("qsv_h264", EncoderEngine::QuickSync, None),
            ]),
            8,
        );
        assert_eq!(config.encoder_id, "qsv_h264");

        // Every hardware encoder refused → fall back to software, not to a broken one.
        let config = suggest_from(
            &catalog(vec![
                encoder("nvenc_h264", EncoderEngine::Nvenc, Some(false)),
                encoder("x264", EncoderEngine::Software, None),
            ]),
            8,
        );
        assert_eq!(config.encoder_id, "x264");
        assert!(!config.hardware);
    }

    /// An unverified encoder (`None`) has not been smoke-tested, not refused.
    #[test]
    fn an_unverified_encoder_is_still_offered() {
        let config = suggest_from(
            &catalog(vec![encoder("amf_h264", EncoderEngine::Amf, None)]),
            8,
        );
        assert_eq!(config.encoder_id, "amf_h264");
        assert!(config.hardware);
    }

    #[test]
    fn software_drops_to_thirty_fps_and_a_lower_bitrate() {
        let config = suggest_from(
            &catalog(vec![encoder("x264", EncoderEngine::Software, None)]),
            8,
        );
        assert_eq!(config.fps, 30);
        assert_eq!(config.bitrate_kbps, 4500);
        assert_eq!(config.quality_reason, "autoconfig-reason-quality-software");
    }

    #[test]
    fn a_low_core_machine_gets_a_lower_bitrate_and_says_why() {
        let config = suggest_from(
            &catalog(vec![encoder("x264", EncoderEngine::Software, None)]),
            2,
        );
        assert_eq!(config.bitrate_kbps, 3000);
        assert_eq!(config.quality_reason, "autoconfig-reason-quality-low-cores");
    }

    /// No encoders at all (a stripped build, a broken probe) must still produce a
    /// usable suggestion rather than panicking or offering nothing.
    #[test]
    fn an_empty_catalog_still_suggests_x264() {
        let config = suggest_from(&catalog(Vec::new()), 8);
        assert_eq!(config.encoder_id, "x264");
        assert!(!config.hardware);
        assert_eq!(config.width, 1920);
        assert_eq!(config.height, 1080);
    }

    #[test]
    fn the_suggestion_is_deterministic() {
        let cat = catalog(vec![
            encoder("qsv_h264", EncoderEngine::QuickSync, None),
            encoder("x264", EncoderEngine::Software, None),
        ]);
        assert_eq!(suggest_from(&cat, 8), suggest_from(&cat, 8));
    }
}
