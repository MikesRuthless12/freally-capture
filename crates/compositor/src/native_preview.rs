//! The **native preview surface**: the composed program frame blitted
//! straight to a window, with no GPU-readback → JPEG → webview-canvas round
//! trip. This is the "OBS feel" path — the preview *is* the GPU output,
//! painted on the GPU.
//!
//! The compositor stays `#![forbid(unsafe_code)]`: this module takes a window
//! that already implements the safe `raw-window-handle` traits (the
//! unavoidable `unsafe` of building a handle from a native window lives in
//! the windowing crate that constructs it) and uses wgpu's **safe**
//! `create_surface`. On acquire failure (minimised, device lost) the frame is
//! skipped honestly rather than panicking; the caller keeps the JPEG
//! `preview://` path as the cross-platform fallback.

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use crate::gpu::Gpu;
use crate::CompositorError;

/// A window surface the program frame is blitted onto.
pub struct NativePreview {
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    pipeline: wgpu::RenderPipeline,
    bind_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
}

impl NativePreview {
    /// Create a surface on `window` and the blit pipeline. `window` must
    /// outlive the surface — the caller (the windowing layer) owns it for the
    /// preview's lifetime.
    pub fn new<W>(gpu: &Gpu, window: W, width: u32, height: u32) -> Result<Self, CompositorError>
    where
        W: HasWindowHandle + HasDisplayHandle + Send + Sync + 'static,
    {
        let surface = gpu
            .instance
            .create_surface(window)
            .map_err(|err| CompositorError::Device(format!("preview surface: {err}")))?;

        let caps = surface.get_capabilities(&gpu.adapter);
        if caps.formats.is_empty() {
            return Err(CompositorError::Device(
                "the preview surface reports no supported formats".into(),
            ));
        }
        // Prefer a plain (non-sRGB) 8-bit BGRA/RGBA format so the blit writes
        // the program frame's colors 1:1; fall back to whatever is offered.
        let format = caps
            .formats
            .iter()
            .copied()
            .find(|f| {
                matches!(
                    f,
                    wgpu::TextureFormat::Bgra8Unorm | wgpu::TextureFormat::Rgba8Unorm
                )
            })
            .unwrap_or(caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: width.max(1),
            height: height.max(1),
            present_mode: wgpu::PresentMode::AutoVsync,
            desired_maximum_frame_latency: 2,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&gpu.device, &config);

        let (pipeline, bind_layout) = build_pipeline(&gpu.device, format);
        let sampler = gpu.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("fcap-preview-sampler"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        Ok(Self {
            surface,
            config,
            pipeline,
            bind_layout,
            sampler,
        })
    }

    /// Reconfigure after the window resized. A no-op for a zero dimension
    /// (minimised) — reconfiguring to 0 is invalid.
    pub fn resize(&mut self, gpu: &Gpu, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        if (width, height) == (self.config.width, self.config.height) {
            return;
        }
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&gpu.device, &self.config);
    }

    pub fn size(&self) -> (u32, u32) {
        (self.config.width, self.config.height)
    }

    /// Blit the program texture onto the surface and present. `program_view`
    /// is the compositor's current program-frame view (recreated on canvas
    /// resize, so the bind group is rebuilt each present — cheap). Returns
    /// `Ok(false)` when the frame had to be skipped (surface not ready);
    /// `Ok(true)` when presented.
    pub fn present(
        &mut self,
        gpu: &Gpu,
        program_view: &wgpu::TextureView,
    ) -> Result<bool, CompositorError> {
        let frame = match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(wgpu::SurfaceError::Outdated | wgpu::SurfaceError::Lost) => {
                // Window resized/occluded between configure and acquire —
                // reconfigure and skip this frame; the next one presents.
                self.surface.configure(&gpu.device, &self.config);
                return Ok(false);
            }
            Err(wgpu::SurfaceError::Timeout) => return Ok(false),
            Err(wgpu::SurfaceError::OutOfMemory) => {
                return Err(CompositorError::Device("preview surface OOM".into()))
            }
            Err(wgpu::SurfaceError::Other) => {
                return Err(CompositorError::Device("preview surface error".into()))
            }
        };
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("fcap-preview-bind"),
            layout: &self.bind_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(program_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
        });

        let mut encoder = gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("fcap-preview-blit"),
            });
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("fcap-preview-pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.draw(0..3, 0..1);
        }
        gpu.queue.submit(Some(encoder.finish()));
        frame.present();
        Ok(true)
    }
}

fn build_pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
) -> (wgpu::RenderPipeline, wgpu::BindGroupLayout) {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("fcap-preview-blit"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shaders/blit.wgsl").into()),
    });
    let bind_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("fcap-preview-bind-layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    });
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("fcap-preview-layout"),
        bind_group_layouts: &[&bind_layout],
        push_constant_ranges: &[],
    });
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("fcap-preview-pipeline"),
        cache: None,
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs"),
            buffers: &[],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs"),
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });
    (pipeline, bind_layout)
}
