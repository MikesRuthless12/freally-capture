//! GPU bring-up: instance → adapter → device/queue.
//!
//! The compositor renders into an offscreen program texture that is read back
//! (preview JPEG, encoders) — and, for the native preview surface, blitted
//! straight to a window with no readback. The `instance` and `adapter` are
//! **kept alive** so a [`wgpu::Surface`] can be created on demand from a
//! window handle (a surface borrows the instance and must be validated
//! against the adapter). Adapter choice prefers real hardware and falls back
//! to software rasterizers (WARP on Windows, lavapipe/llvmpipe on Linux) so
//! the engine still runs — slowly but honestly — inside VMs and CI runners.

use crate::CompositorError;

/// The shared device handle the compositor and its filter passes draw with.
pub struct Gpu {
    /// Retained so surfaces can be created from a window handle later.
    pub instance: wgpu::Instance,
    /// Retained so a surface's supported formats can be queried.
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    /// "<name> (<backend>, <device type>)" — for logs and the stats dock.
    pub adapter_summary: String,
    /// True when the device is the DX12 backend — the DirectComposition
    /// preview path is only possible then.
    pub is_dx12: bool,
}

impl Gpu {
    /// Bring up a headless device. `Err(NoAdapter)` when the machine (or CI
    /// runner) exposes no usable GPU at all — callers surface that honestly
    /// instead of pretending to render.
    pub fn new() -> Result<Self, CompositorError> {
        // On Windows the native preview composites over the WebView2 (which is
        // DirectComposition-hosted) via a DComp visual — and DComp only accepts
        // **DirectX** swapchains. So prefer the DX12 backend there, sharing one
        // device between the compositor and that composition swapchain. If DX12
        // yields no adapter we fall back to the broad set (native preview then
        // stays on the JPEG path); everywhere else PRIMARY|GL as before.
        let (instance, adapter) = Self::pick_adapter();
        let adapter = adapter.ok_or(CompositorError::NoAdapter)?;

        let info = adapter.get_info();
        let is_dx12 = info.backend == wgpu::Backend::Dx12;
        let adapter_summary = format!(
            "{} ({:?}, {:?})",
            if info.name.is_empty() {
                "unnamed adapter"
            } else {
                &info.name
            },
            info.backend,
            info.device_type
        );
        tracing::info!(adapter = %adapter_summary, "compositor GPU selected");

        // Broadly-supported baseline limits, but with the adapter's real
        // maximum texture size so ultrawide/5K captures upload intact.
        let limits = wgpu::Limits::downlevel_defaults().using_resolution(adapter.limits());
        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("fcap-compositor"),
            required_features: wgpu::Features::empty(),
            required_limits: limits,
            memory_hints: wgpu::MemoryHints::default(),
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
            trace: wgpu::Trace::Off,
        }))
        .map_err(|err| CompositorError::Device(err.to_string()))?;

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
            adapter_summary,
            is_dx12,
        })
    }

    /// Bring up an instance + adapter, preferring the DX12 backend on Windows
    /// (needed for the DirectComposition preview), with an honest fallback to
    /// the broad backend set. Returns the instance that produced the adapter.
    fn pick_adapter() -> (wgpu::Instance, Option<wgpu::Adapter>) {
        let request = |instance: &wgpu::Instance, fallback: bool| {
            pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: if fallback {
                    wgpu::PowerPreference::LowPower
                } else {
                    wgpu::PowerPreference::HighPerformance
                },
                force_fallback_adapter: fallback,
                compatible_surface: None,
            }))
        };

        // Windows: try DX12 alone first so the compositor and the DComp
        // composition swapchain share one DirectX device.
        #[cfg(windows)]
        {
            let dx12 = wgpu::Instance::new(&wgpu::InstanceDescriptor {
                backends: wgpu::Backends::DX12,
                ..Default::default()
            });
            if let Ok(adapter) = request(&dx12, false) {
                return (dx12, Some(adapter));
            }
            tracing::warn!("no DX12 adapter — native preview will use the JPEG path");
        }

        // Broad set (all platforms; the Windows fallback path too).
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            // PRIMARY = Vulkan / Metal / DX12; GL catches older Linux setups.
            backends: wgpu::Backends::PRIMARY | wgpu::Backends::GL,
            ..Default::default()
        });
        let adapter = request(&instance, false)
            .or_else(|_| request(&instance, true))
            .ok();
        (instance, adapter)
    }
}
