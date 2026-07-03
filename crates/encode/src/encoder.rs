//! The encoder catalog: which video encoders this machine can offer, which
//! is best, and what each honestly costs.
//!
//! Every **wire** encoder here — hardware (NVENC / Quick Sync / AMF / VAAPI /
//! VideoToolbox) or software (x264/x265/SVT-AV1) — runs through the
//! clearly-labeled, on-demand **ffmpeg component** (never bundled; see
//! `crate::ffmpeg`). The hardware probe decides what to *offer*; because
//! encoder support varies by GPU generation and driver, an entry is only
//! *confirmed* once the fetched ffmpeg smoke-tests it. The owned
//! **`freally-video`** lossless codec is deliberately not in this catalog:
//! it is a recording format, always exists, and needs nothing fetched.

use serde::{Deserialize, Serialize};

use crate::hardware::{detect_gpus, has_va_render_node, GpuInfo, GpuVendor};

/// The codec families a recording can encode to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VideoCodec {
    H264,
    Hevc,
    Av1,
}

impl VideoCodec {
    pub fn label(self) -> &'static str {
        match self {
            VideoCodec::H264 => "H.264",
            VideoCodec::Hevc => "HEVC",
            VideoCodec::Av1 => "AV1",
        }
    }
}

/// The engine driving a codec — ranked; lower is better when picking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EncoderEngine {
    Nvenc,
    QuickSync,
    Amf,
    VideoToolbox,
    Vaapi,
    Software,
}

impl EncoderEngine {
    fn rank(self) -> u8 {
        match self {
            EncoderEngine::Nvenc => 0,
            EncoderEngine::QuickSync => 1,
            EncoderEngine::Amf => 2,
            EncoderEngine::VideoToolbox => 3,
            EncoderEngine::Vaapi => 4,
            EncoderEngine::Software => 5,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            EncoderEngine::Nvenc => "NVIDIA NVENC",
            EncoderEngine::QuickSync => "Intel Quick Sync",
            EncoderEngine::Amf => "AMD AMF",
            EncoderEngine::VideoToolbox => "Apple VideoToolbox",
            EncoderEngine::Vaapi => "VAAPI",
            EncoderEngine::Software => "Software (CPU)",
        }
    }
}

/// One encoder the picker can offer. `id` is the ffmpeg encoder name — the
/// stable identifier settings persist ("libx264", "h264_nvenc", …).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EncoderDesc {
    pub id: String,
    pub codec: VideoCodec,
    pub engine: EncoderEngine,
    /// Picker label, e.g. "NVIDIA NVENC (H.264)".
    pub label: String,
    pub hardware: bool,
    /// The honest capability note the picker shows.
    pub note: String,
    /// `None` until the fetched ffmpeg smoke-tested it; `Some(false)` means
    /// the hardware/driver refused — the picker shows it greyed with the
    /// reason, and auto-pick skips it.
    pub verified: Option<bool>,
}

/// Everything encoder detection found, ready for the picker.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Catalog {
    pub gpus: Vec<GpuInfo>,
    pub encoders: Vec<EncoderDesc>,
}

impl Catalog {
    /// Probe this machine (GPU enumeration + per-OS rules).
    pub fn detect() -> Self {
        let gpus = detect_gpus();
        let encoders = catalog_for(std::env::consts::OS, &gpus, has_va_render_node());
        Catalog { gpus, encoders }
    }

    /// The best usable encoder for a codec: hardware over software, skipping
    /// entries a smoke test refused. H.264 always resolves (x264 is always
    /// present); HEVC/AV1 may not.
    pub fn best(&self, codec: VideoCodec) -> Option<&EncoderDesc> {
        self.encoders
            .iter()
            .filter(|enc| enc.codec == codec && enc.verified != Some(false))
            .min_by_key(|enc| enc.engine.rank())
    }

    /// Look an encoder up by its stable id.
    pub fn get(&self, id: &str) -> Option<&EncoderDesc> {
        self.encoders.iter().find(|enc| enc.id == id)
    }
}

/// The note every hardware entry carries (probe honesty).
fn hw_note(gpu_name: &str, extra: &str) -> String {
    let mut note = format!(
        "Hardware encode on {gpu_name} — near-zero CPU cost. Runs via the on-demand ffmpeg \
         component; confirmed on first use."
    );
    if !extra.is_empty() {
        note.push(' ');
        note.push_str(extra);
    }
    note
}

fn push_family(
    encoders: &mut Vec<EncoderDesc>,
    engine: EncoderEngine,
    suffix: &str,
    gpu_name: &str,
    hevc_extra: &str,
    av1: Option<&str>,
) {
    let families: &[(VideoCodec, &str, &str)] = &[
        (VideoCodec::H264, "h264", ""),
        (VideoCodec::Hevc, "hevc", hevc_extra),
    ];
    for (codec, prefix, extra) in families {
        push_unique(
            encoders,
            EncoderDesc {
                id: format!("{prefix}_{suffix}"),
                codec: *codec,
                engine,
                label: format!("{} ({})", engine.label(), codec.label()),
                hardware: true,
                note: hw_note(gpu_name, extra),
                verified: None,
            },
        );
    }
    if let Some(av1_extra) = av1 {
        push_unique(
            encoders,
            EncoderDesc {
                id: format!("av1_{suffix}"),
                codec: VideoCodec::Av1,
                engine,
                label: format!("{} (AV1)", engine.label()),
                hardware: true,
                note: hw_note(gpu_name, av1_extra),
                verified: None,
            },
        );
    }
}

/// Two GPUs of one vendor must not duplicate entries — ids stay unique.
fn push_unique(encoders: &mut Vec<EncoderDesc>, desc: EncoderDesc) {
    if !encoders.iter().any(|enc| enc.id == desc.id) {
        encoders.push(desc);
    }
}

/// Build the offer list for an OS + GPU set. Pure, so every per-OS rule is
/// unit-tested on any host; `Catalog::detect` feeds it reality.
pub fn catalog_for(os: &str, gpus: &[GpuInfo], va_render_node: bool) -> Vec<EncoderDesc> {
    let mut encoders: Vec<EncoderDesc> = Vec::new();
    let name_of = |vendor: GpuVendor| -> String {
        gpus.iter()
            .find(|gpu| gpu.vendor == vendor)
            .map(|gpu| gpu.name.clone())
            .unwrap_or_else(|| "this GPU".to_string())
    };

    for gpu in gpus {
        match gpu.vendor {
            GpuVendor::Nvidia if os == "windows" || os == "linux" => push_family(
                &mut encoders,
                EncoderEngine::Nvenc,
                "nvenc",
                &gpu.name,
                "(HEVC needs a GTX 900-series or newer.)",
                Some("(AV1 encode needs an RTX 40-series or newer.)"),
            ),
            GpuVendor::Intel if os == "windows" => push_family(
                &mut encoders,
                EncoderEngine::QuickSync,
                "qsv",
                &gpu.name,
                "",
                Some("(AV1 encode needs Intel Arc or a 14th-gen iGPU or newer.)"),
            ),
            GpuVendor::Amd if os == "windows" => push_family(
                &mut encoders,
                EncoderEngine::Amf,
                "amf",
                &gpu.name,
                "",
                Some("(AV1 encode needs an RX 7000-series / RDNA 3 or newer.)"),
            ),
            _ => {}
        }
    }

    // Linux: Intel/AMD encode through VAAPI (needs a DRM render node).
    if os == "linux"
        && va_render_node
        && gpus
            .iter()
            .any(|gpu| matches!(gpu.vendor, GpuVendor::Intel | GpuVendor::Amd))
    {
        let vendor = if gpus.iter().any(|gpu| gpu.vendor == GpuVendor::Intel) {
            GpuVendor::Intel
        } else {
            GpuVendor::Amd
        };
        push_family(
            &mut encoders,
            EncoderEngine::Vaapi,
            "vaapi",
            &name_of(vendor),
            "",
            Some("(AV1 encode needs a recent GPU + driver.)"),
        );
    }

    // macOS: VideoToolbox is the system encoder — always offered (Apple
    // silicon and Intel Macs both expose it; no AV1 encode via ffmpeg).
    if os == "macos" {
        push_family(
            &mut encoders,
            EncoderEngine::VideoToolbox,
            "videotoolbox",
            "the Apple media engine",
            "",
            None,
        );
    }

    // The universal CPU fallbacks — always offered, on every machine.
    encoders.push(EncoderDesc {
        id: "libx264".to_string(),
        codec: VideoCodec::H264,
        engine: EncoderEngine::Software,
        label: "x264 (H.264, CPU)".to_string(),
        hardware: false,
        note: "Software H.264 — the universal fallback, always available. Higher CPU load at \
               high resolutions. Runs via the on-demand ffmpeg component."
            .to_string(),
        verified: None,
    });
    encoders.push(EncoderDesc {
        id: "libx265".to_string(),
        codec: VideoCodec::Hevc,
        engine: EncoderEngine::Software,
        label: "x265 (HEVC, CPU)".to_string(),
        hardware: false,
        note: "Software HEVC — very CPU-heavy at 1080p60; prefer a hardware HEVC encoder when \
               one is available. Runs via the on-demand ffmpeg component."
            .to_string(),
        verified: None,
    });
    // The software AV1 encoder differs per pinned ffmpeg build: the Windows
    // essentials build ships libaom; the Linux/macOS builds ship SVT-AV1.
    let (av1_id, av1_label) = if os == "windows" {
        ("libaom-av1", "AOM AV1 (CPU)")
    } else {
        ("libsvtav1", "SVT-AV1 (AV1, CPU)")
    };
    encoders.push(EncoderDesc {
        id: av1_id.to_string(),
        codec: VideoCodec::Av1,
        engine: EncoderEngine::Software,
        label: av1_label.to_string(),
        hardware: false,
        note: "Software AV1 — CPU-heavy; prefer a hardware AV1 encoder when one is available. \
               Runs via the on-demand ffmpeg component."
            .to_string(),
        verified: None,
    });

    encoders
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gpu(vendor: GpuVendor, name: &str) -> GpuInfo {
        GpuInfo {
            name: name.to_string(),
            vendor,
            backend: "Vulkan".to_string(),
        }
    }

    fn ids(encoders: &[EncoderDesc]) -> Vec<&str> {
        encoders.iter().map(|enc| enc.id.as_str()).collect()
    }

    #[test]
    fn windows_nvidia_plus_intel_offers_both_plus_x264() {
        let gpus = [
            gpu(GpuVendor::Nvidia, "NVIDIA GeForce RTX 4070"),
            gpu(GpuVendor::Intel, "Intel(R) UHD Graphics 770"),
        ];
        let encoders = catalog_for("windows", &gpus, false);
        let ids = ids(&encoders);
        assert!(ids.contains(&"h264_nvenc"));
        assert!(ids.contains(&"h264_qsv"));
        assert!(ids.contains(&"libx264"));
        assert!(!ids.contains(&"h264_vaapi"), "VAAPI is Linux-only");

        let catalog = Catalog {
            gpus: gpus.to_vec(),
            encoders,
        };
        assert_eq!(
            catalog.best(VideoCodec::H264).expect("h264 resolves").id,
            "h264_nvenc",
            "NVENC outranks Quick Sync and x264"
        );
    }

    #[test]
    fn linux_amd_needs_a_render_node_for_vaapi() {
        let gpus = [gpu(GpuVendor::Amd, "AMD Radeon RX 7800 XT")];
        let with_node = catalog_for("linux", &gpus, true);
        assert!(ids(&with_node).contains(&"h264_vaapi"));
        assert!(
            !ids(&with_node).contains(&"h264_amf"),
            "AMF is Windows-only"
        );

        let without_node = catalog_for("linux", &gpus, false);
        assert!(
            !ids(&without_node).contains(&"h264_vaapi"),
            "no render node → no VAAPI offer"
        );
        assert!(ids(&without_node).contains(&"libx264"));
    }

    #[test]
    fn macos_always_offers_videotoolbox() {
        let encoders = catalog_for("macos", &[], false);
        let ids = ids(&encoders);
        assert!(ids.contains(&"h264_videotoolbox"));
        assert!(ids.contains(&"hevc_videotoolbox"));
        assert!(!ids.contains(&"av1_videotoolbox"), "no AV1 encode via VT");
        let catalog = Catalog {
            gpus: vec![],
            encoders,
        };
        assert_eq!(
            catalog.best(VideoCodec::H264).expect("resolves").id,
            "h264_videotoolbox"
        );
    }

    #[test]
    fn a_gpu_less_machine_still_gets_the_cpu_fallbacks() {
        for os in ["windows", "linux", "macos"] {
            let encoders = catalog_for(os, &[], false);
            let ids = ids(&encoders);
            assert!(ids.contains(&"libx264"), "{os}: x264 is always available");
            assert!(ids.contains(&"libx265"));
            // The software AV1 id tracks what each OS's pinned build ships.
            if os == "windows" {
                assert!(ids.contains(&"libaom-av1"));
            } else {
                assert!(ids.contains(&"libsvtav1"));
            }
        }
    }

    #[test]
    fn a_failed_smoke_test_drops_an_encoder_from_best() {
        let gpus = [gpu(GpuVendor::Nvidia, "NVIDIA GeForce GTX 1060")];
        let mut encoders = catalog_for("windows", &gpus, false);
        let av1 = encoders
            .iter_mut()
            .find(|enc| enc.id == "av1_nvenc")
            .expect("AV1 NVENC is offered as a candidate");
        av1.verified = Some(false); // a Pascal card has no AV1 encode
        let catalog = Catalog {
            gpus: gpus.to_vec(),
            encoders,
        };
        assert_eq!(
            catalog.best(VideoCodec::Av1).expect("software remains").id,
            "libaom-av1",
            "refused hardware falls back to the CPU encoder"
        );
    }

    #[test]
    fn two_gpus_of_one_vendor_do_not_duplicate_entries() {
        let gpus = [
            gpu(GpuVendor::Nvidia, "NVIDIA GeForce RTX 4090"),
            gpu(GpuVendor::Nvidia, "NVIDIA GeForce RTX 3060"),
        ];
        let encoders = catalog_for("windows", &gpus, false);
        let nvenc_count = encoders.iter().filter(|enc| enc.id == "h264_nvenc").count();
        assert_eq!(nvenc_count, 1);
    }

    #[test]
    fn every_id_is_unique() {
        let gpus = [
            gpu(GpuVendor::Nvidia, "A"),
            gpu(GpuVendor::Intel, "B"),
            gpu(GpuVendor::Amd, "C"),
        ];
        for os in ["windows", "linux", "macos"] {
            let encoders = catalog_for(os, &gpus, true);
            let mut seen = std::collections::HashSet::new();
            for enc in &encoders {
                assert!(seen.insert(enc.id.clone()), "{os}: duplicate {}", enc.id);
            }
        }
    }

    #[test]
    fn hardware_notes_admit_the_ffmpeg_dependency() {
        let gpus = [gpu(GpuVendor::Nvidia, "NVIDIA GeForce RTX 4070")];
        let encoders = catalog_for("windows", &gpus, false);
        for enc in &encoders {
            assert!(
                enc.note.contains("ffmpeg"),
                "{}: every wire encoder must label the ffmpeg path",
                enc.id
            );
        }
    }
}
