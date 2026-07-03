//! The GPU probe behind encoder detection.
//!
//! Enumerates every real adapter (via `wgpu`, so one probe covers
//! DX12/Vulkan/Metal/GL) and reduces it to what encoder detection needs:
//! **which vendors' silicon is present**. Software rasterizers (WARP,
//! llvmpipe) are skipped — they can't encode. The probe only decides what to
//! *offer*; whether an encoder actually works on this driver is confirmed by
//! the ffmpeg smoke test before first use (see `crate::ffmpeg`), because
//! encoder support varies by GPU generation and driver in ways a vendor id
//! cannot know.

use serde::Serialize;

/// The GPU vendors that ship hardware video encoders we drive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GpuVendor {
    Nvidia,
    Amd,
    Intel,
    Apple,
    Other,
}

impl GpuVendor {
    /// Classify an adapter by PCI vendor id, falling back to the adapter
    /// name (GL backends sometimes report a zero vendor id).
    pub fn from_ids(vendor_id: u32, name: &str) -> Self {
        match vendor_id {
            0x10DE => GpuVendor::Nvidia,
            0x1002 => GpuVendor::Amd,
            0x8086 => GpuVendor::Intel,
            0x106B => GpuVendor::Apple,
            _ => {
                let upper = name.to_uppercase();
                if upper.contains("NVIDIA") || upper.contains("GEFORCE") {
                    GpuVendor::Nvidia
                } else if upper.contains("AMD") || upper.contains("RADEON") {
                    GpuVendor::Amd
                } else if upper.contains("INTEL") {
                    GpuVendor::Intel
                } else if upper.contains("APPLE") {
                    GpuVendor::Apple
                } else {
                    GpuVendor::Other
                }
            }
        }
    }
}

/// One physical GPU, as encoder detection sees it.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GpuInfo {
    pub name: String,
    pub vendor: GpuVendor,
    /// The backend the adapter was found through ("Vulkan", "Dx12", …).
    pub backend: String,
}

/// Enumerate the machine's real GPUs. The same silicon often appears under
/// several backends (DX12 + Vulkan + GL); entries are deduplicated by
/// (vendor id, device id) so one card is one entry. Returns empty on a
/// GPU-less machine (CI, headless VM) — the catalog then offers only the
/// CPU paths, honestly.
pub fn detect_gpus() -> Vec<GpuInfo> {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY | wgpu::Backends::GL,
        ..Default::default()
    });
    let mut seen: Vec<(u32, u32)> = Vec::new();
    let mut gpus = Vec::new();
    for adapter in instance.enumerate_adapters(wgpu::Backends::PRIMARY | wgpu::Backends::GL) {
        let info = adapter.get_info();
        // Software rasterizers can't hardware-encode; skip them.
        if info.device_type == wgpu::DeviceType::Cpu {
            continue;
        }
        let key = (info.vendor, info.device);
        if key != (0, 0) && seen.contains(&key) {
            continue;
        }
        // Zero ids (some GL stacks): dedupe by name instead.
        if key == (0, 0) && gpus.iter().any(|gpu: &GpuInfo| gpu.name == info.name) {
            continue;
        }
        seen.push(key);
        gpus.push(GpuInfo {
            vendor: GpuVendor::from_ids(info.vendor, &info.name),
            name: info.name,
            backend: format!("{:?}", info.backend),
        });
    }
    tracing::info!(count = gpus.len(), "encoder probe: GPUs enumerated");
    gpus
}

/// Linux: whether a DRM render node exists (`/dev/dri/renderD*`) — VAAPI
/// needs one. Always false elsewhere.
pub fn has_va_render_node() -> bool {
    #[cfg(target_os = "linux")]
    {
        if let Ok(entries) = std::fs::read_dir("/dev/dri") {
            for entry in entries.flatten() {
                if entry.file_name().to_string_lossy().starts_with("renderD") {
                    return true;
                }
            }
        }
        false
    }
    #[cfg(not(target_os = "linux"))]
    {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vendor_ids_classify() {
        assert_eq!(GpuVendor::from_ids(0x10DE, ""), GpuVendor::Nvidia);
        assert_eq!(GpuVendor::from_ids(0x1002, ""), GpuVendor::Amd);
        assert_eq!(GpuVendor::from_ids(0x8086, ""), GpuVendor::Intel);
        assert_eq!(GpuVendor::from_ids(0x106B, ""), GpuVendor::Apple);
    }

    #[test]
    fn names_classify_when_the_id_is_missing() {
        assert_eq!(
            GpuVendor::from_ids(0, "NVIDIA GeForce RTX 4070"),
            GpuVendor::Nvidia
        );
        assert_eq!(GpuVendor::from_ids(0, "AMD Radeon RX 7800"), GpuVendor::Amd);
        assert_eq!(
            GpuVendor::from_ids(0, "Intel(R) UHD Graphics 770"),
            GpuVendor::Intel
        );
        assert_eq!(GpuVendor::from_ids(0, "Apple M3"), GpuVendor::Apple);
        assert_eq!(GpuVendor::from_ids(0, "SomeVirtualGPU"), GpuVendor::Other);
    }
}
