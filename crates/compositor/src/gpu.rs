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
}

impl Gpu {
    /// Bring up a headless device. `Err(NoAdapter)` when the machine (or CI
    /// runner) exposes no usable GPU at all — callers surface that honestly
    /// instead of pretending to render.
    pub fn new() -> Result<Self, CompositorError> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            // PRIMARY = Vulkan / Metal / DX12; GL catches older Linux setups.
            backends: wgpu::Backends::PRIMARY | wgpu::Backends::GL,
            ..Default::default()
        });

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: None,
        }))
        .or_else(|| {
            // No hardware adapter — accept a software rasterizer.
            pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                force_fallback_adapter: true,
                compatible_surface: None,
            }))
        })
        .ok_or(CompositorError::NoAdapter)?;

        let info = adapter.get_info();
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
        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("fcap-compositor"),
                required_features: wgpu::Features::empty(),
                required_limits: limits,
            },
            None,
        ))
        .map_err(|err| CompositorError::Device(err.to_string()))?;

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
            adapter_summary,
        })
    }
}
