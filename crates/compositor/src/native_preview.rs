//! The **native preview surface**: the composed program frame blitted
//! straight to a window, with no GPU-readback → JPEG → webview-canvas round
//! trip. This is the "OBS feel" path — the preview *is* the GPU output,
//! painted on the GPU.
//!
//! The compositor stays `#![forbid(unsafe_code)]`: the DirectComposition
//! overlay (`fcap-preview`) builds the wgpu surface via the unsafe
//! `CompositionVisual` target and hands the finished `Surface` here, so this
//! module only ever configures a surface it is given. On acquire failure
//! (minimised, device lost) the frame is skipped honestly rather than
//! panicking; the caller keeps the JPEG `preview://` path as the fallback.
//!
//! Because the native surface is **opaque** and composited *above* the webview,
//! the HTML selection box + handles are hidden behind it. So the selection
//! chrome is drawn *into* this frame too (a [`PreviewOverlay`] the caller
//! computes from the model each tick) — a line + triangle pass over the blit.
//! It is preview-only: it never touches the program texture the recorder and
//! stream read.

use crate::gpu::Gpu;
use crate::CompositorError;

/// The selection overlay to draw on top of the native preview frame. `corners`
/// are in **canvas pixels** (y-down, origin top-left), in the local corner
/// order `(0,0) (w,0) (0,h) (w,h)` — exactly what [`crate::transform::corners`]
/// produces and what the UI draws its box through. The blit stretches the whole
/// canvas onto the whole surface, so the same corners map straight to the
/// surface.
pub struct PreviewOverlay {
    /// The item's four content corners, canvas px, order `(0,0)(w,0)(0,h)(w,h)`.
    pub corners: [[f32; 2]; 4],
    /// The canvas size those corners are expressed in.
    pub canvas: (f32, f32),
    /// Locked items show the box but no interactive handles.
    pub locked: bool,
}

/// One overlay vertex: a clip-space (NDC) position + an opaque RGBA color.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct OverlayVertex {
    pos: [f32; 2],
    color: [f32; 4],
}

/// Handle half-extent, surface px (a 10px square) — matches the HTML handles'
/// visual weight at typical DPI.
const HANDLE_HALF_PX: f32 = 5.0;
/// Rotate-handle distance out from the top-edge midpoint, surface px.
const ROTATE_OFFSET_PX: f32 = 24.0;
/// Vertex-buffer capacities. Worst case: 5 line segments = 10 verts; 9 handle
/// squares × 6 = 54 verts. Rounded up.
const OVERLAY_LINE_CAP: u64 = 16;
const OVERLAY_TRI_CAP: u64 = 64;

// Overlay colors (written to a non-sRGB Unorm surface, so these are the on-
// screen sRGB values 1:1). Box #4a9eff, handles near-white, rotate #00d4ff.
const BOX_BLUE: [f32; 4] = [0.29, 0.62, 1.0, 1.0];
const HANDLE_WHITE: [f32; 4] = [0.95, 0.97, 1.0, 1.0];
const ROTATE_CYAN: [f32; 4] = [0.0, 0.83, 1.0, 1.0];

/// A window surface the program frame is blitted onto.
pub struct NativePreview {
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    pipeline: wgpu::RenderPipeline,
    bind_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    overlay_line_pipeline: wgpu::RenderPipeline,
    overlay_tri_pipeline: wgpu::RenderPipeline,
    overlay_line_buf: wgpu::Buffer,
    overlay_tri_buf: wgpu::Buffer,
    /// Reused CPU staging for the overlay vertices — cleared and refilled each
    /// present so the hot render path allocates nothing per frame.
    overlay_line_scratch: Vec<OverlayVertex>,
    overlay_tri_scratch: Vec<OverlayVertex>,
}

impl NativePreview {
    /// Configure a surface the caller already created, and build the blit
    /// pipeline. The DirectComposition overlay (`fcap-preview`) needs wgpu's
    /// *unsafe* `CompositionVisual` surface target — which can't live in this
    /// `#![forbid(unsafe_code)]` crate — so it hands the finished `Surface`
    /// here.
    pub fn from_surface(
        gpu: &Gpu,
        surface: wgpu::Surface<'static>,
        width: u32,
        height: u32,
    ) -> Result<Self, CompositorError> {
        Self::finish(gpu, surface, width, height)
    }

    fn finish(
        gpu: &Gpu,
        surface: wgpu::Surface<'static>,
        width: u32,
        height: u32,
    ) -> Result<Self, CompositorError> {
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

        let (overlay_line_pipeline, overlay_tri_pipeline) =
            build_overlay_pipelines(&gpu.device, format);
        let vertex = std::mem::size_of::<OverlayVertex>() as u64;
        let overlay_line_buf = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("fcap-preview-overlay-lines"),
            size: OVERLAY_LINE_CAP * vertex,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let overlay_tri_buf = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("fcap-preview-overlay-tris"),
            size: OVERLAY_TRI_CAP * vertex,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Ok(Self {
            surface,
            config,
            pipeline,
            bind_layout,
            sampler,
            overlay_line_pipeline,
            overlay_tri_pipeline,
            overlay_line_buf,
            overlay_tri_buf,
            overlay_line_scratch: Vec::with_capacity(OVERLAY_LINE_CAP as usize),
            overlay_tri_scratch: Vec::with_capacity(OVERLAY_TRI_CAP as usize),
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

    /// Blit the program texture onto the surface, draw the selection `overlay`
    /// (if any) over it, and present. `program_view` is the compositor's
    /// current program-frame view (recreated on canvas resize, so the bind
    /// group is rebuilt each present — cheap). Returns `Ok(false)` when the
    /// frame had to be skipped (surface not ready); `Ok(true)` when presented.
    pub fn present(
        &mut self,
        gpu: &Gpu,
        program_view: &wgpu::TextureView,
        overlay: Option<&PreviewOverlay>,
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

        // Build the overlay geometry and upload it *before* the pass, so the
        // draw counts are known and the queue writes land ahead of the submit.
        let (line_count, tri_count) = match overlay {
            Some(ov) => {
                self.overlay_line_scratch.clear();
                self.overlay_tri_scratch.clear();
                overlay_vertices(
                    ov,
                    self.config.width as f32,
                    self.config.height as f32,
                    &mut self.overlay_line_scratch,
                    &mut self.overlay_tri_scratch,
                );
                if !self.overlay_line_scratch.is_empty() {
                    gpu.queue.write_buffer(
                        &self.overlay_line_buf,
                        0,
                        bytemuck::cast_slice(&self.overlay_line_scratch),
                    );
                }
                if !self.overlay_tri_scratch.is_empty() {
                    gpu.queue.write_buffer(
                        &self.overlay_tri_buf,
                        0,
                        bytemuck::cast_slice(&self.overlay_tri_scratch),
                    );
                }
                (
                    self.overlay_line_scratch.len() as u32,
                    self.overlay_tri_scratch.len() as u32,
                )
            }
            None => (0, 0),
        };

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

            // The selection chrome, over the program frame (same attachment).
            if line_count > 0 {
                pass.set_pipeline(&self.overlay_line_pipeline);
                pass.set_vertex_buffer(0, self.overlay_line_buf.slice(..));
                pass.draw(0..line_count, 0..1);
            }
            if tri_count > 0 {
                pass.set_pipeline(&self.overlay_tri_pipeline);
                pass.set_vertex_buffer(0, self.overlay_tri_buf.slice(..));
                pass.draw(0..tri_count, 0..1);
            }
        }
        gpu.queue.submit(Some(encoder.finish()));
        frame.present();
        Ok(true)
    }
}

/// Append the overlay's line + triangle vertices for the selection box into the
/// caller's (pre-cleared) buffers. Everything is computed in **surface pixels**
/// (mapping the canvas-space box through the same stretch the blit applies), so
/// handles are square and a fixed size regardless of canvas aspect, then
/// projected to NDC.
fn overlay_vertices(
    overlay: &PreviewOverlay,
    surface_w: f32,
    surface_h: f32,
    lines: &mut Vec<OverlayVertex>,
    tris: &mut Vec<OverlayVertex>,
) {
    let (cw, ch) = overlay.canvas;
    if cw <= 0.0 || ch <= 0.0 || surface_w <= 0.0 || surface_h <= 0.0 {
        return;
    }
    // Canvas px → surface px (the blit stretches the whole canvas onto the
    // whole surface); surface px → NDC (y-down px to y-up clip).
    let to_surface = |p: [f32; 2]| [p[0] * surface_w / cw, p[1] * surface_h / ch];
    let ndc = |p: [f32; 2]| [2.0 * p[0] / surface_w - 1.0, 1.0 - 2.0 * p[1] / surface_h];
    let seg = |a: [f32; 2], b: [f32; 2], color: [f32; 4]| {
        [
            OverlayVertex { pos: ndc(a), color },
            OverlayVertex { pos: ndc(b), color },
        ]
    };
    let square = |p: [f32; 2], color: [f32; 4]| {
        let h = HANDLE_HALF_PX;
        let tl = ndc([p[0] - h, p[1] - h]);
        let tr = ndc([p[0] + h, p[1] - h]);
        let bl = ndc([p[0] - h, p[1] + h]);
        let br = ndc([p[0] + h, p[1] + h]);
        [
            OverlayVertex { pos: tl, color },
            OverlayVertex { pos: tr, color },
            OverlayVertex { pos: bl, color },
            OverlayVertex { pos: tr, color },
            OverlayVertex { pos: br, color },
            OverlayVertex { pos: bl, color },
        ]
    };

    let c = [
        to_surface(overlay.corners[0]),
        to_surface(overlay.corners[1]),
        to_surface(overlay.corners[2]),
        to_surface(overlay.corners[3]),
    ];

    // Box edges: (0,0)-(w,0)-(w,h)-(0,h)-(0,0) → corners 0,1,3,2.
    lines.extend_from_slice(&seg(c[0], c[1], BOX_BLUE));
    lines.extend_from_slice(&seg(c[1], c[3], BOX_BLUE));
    lines.extend_from_slice(&seg(c[3], c[2], BOX_BLUE));
    lines.extend_from_slice(&seg(c[2], c[0], BOX_BLUE));

    if overlay.locked {
        return;
    }

    let mid = |a: [f32; 2], b: [f32; 2]| [(a[0] + b[0]) * 0.5, (a[1] + b[1]) * 0.5];
    let left = mid(c[0], c[2]);
    let right = mid(c[1], c[3]);
    let top = mid(c[0], c[1]);
    let bottom = mid(c[2], c[3]);

    // Rotate handle: out from the top-edge midpoint, away from the box center.
    let center = [
        (c[0][0] + c[1][0] + c[2][0] + c[3][0]) * 0.25,
        (c[0][1] + c[1][1] + c[2][1] + c[3][1]) * 0.25,
    ];
    let dir = [top[0] - center[0], top[1] - center[1]];
    let len = (dir[0] * dir[0] + dir[1] * dir[1]).sqrt();
    if len > 1e-3 {
        let n = [dir[0] / len, dir[1] / len];
        let rotate = [
            top[0] + n[0] * ROTATE_OFFSET_PX,
            top[1] + n[1] * ROTATE_OFFSET_PX,
        ];
        lines.extend_from_slice(&seg(top, rotate, ROTATE_CYAN));
        tris.extend_from_slice(&square(rotate, ROTATE_CYAN));
    }

    // Corner + edge handles (white squares).
    for p in [c[0], c[1], c[2], c[3], left, right, top, bottom] {
        tris.extend_from_slice(&square(p, HANDLE_WHITE));
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

/// Build the two overlay pipelines (lines for the box + rotate stem, triangles
/// for the handle squares) — one shared shader + vertex layout, differing only
/// in primitive topology.
fn build_overlay_pipelines(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
) -> (wgpu::RenderPipeline, wgpu::RenderPipeline) {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("fcap-preview-overlay"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shaders/overlay.wgsl").into()),
    });
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("fcap-preview-overlay-layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });
    const ATTRS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x4];
    let make = |topology: wgpu::PrimitiveTopology| {
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("fcap-preview-overlay-pipeline"),
            cache: None,
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<OverlayVertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &ATTRS,
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        })
    };
    (
        make(wgpu::PrimitiveTopology::LineList),
        make(wgpu::PrimitiveTopology::TriangleList),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn overlay(corners: [[f32; 2]; 4], locked: bool) -> PreviewOverlay {
        PreviewOverlay {
            corners,
            canvas: (100.0, 100.0),
            locked,
        }
    }

    #[test]
    fn unlocked_box_emits_box_lines_and_handles() {
        // A centered 20×20 box on a 100×100 canvas.
        let ov = overlay(
            [[40.0, 40.0], [60.0, 40.0], [40.0, 60.0], [60.0, 60.0]],
            false,
        );
        let mut lines = Vec::new();
        let mut tris = Vec::new();
        overlay_vertices(&ov, 200.0, 200.0, &mut lines, &mut tris);
        // 4 box edges (8 verts) + 1 rotate stem (2 verts).
        assert_eq!(lines.len(), 10);
        // 8 corner/edge handles + 1 rotate handle, 6 verts each.
        assert_eq!(tris.len(), 9 * 6);
        // Every vertex lands inside clip space.
        for v in lines.iter().chain(tris.iter()) {
            assert!(v.pos[0] >= -1.5 && v.pos[0] <= 1.5);
            assert!(v.pos[1] >= -1.5 && v.pos[1] <= 1.5);
        }
    }

    #[test]
    fn locked_box_draws_the_outline_but_no_handles() {
        let ov = overlay(
            [[40.0, 40.0], [60.0, 40.0], [40.0, 60.0], [60.0, 60.0]],
            true,
        );
        let mut lines = Vec::new();
        let mut tris = Vec::new();
        overlay_vertices(&ov, 200.0, 200.0, &mut lines, &mut tris);
        assert_eq!(lines.len(), 8); // box only
        assert!(tris.is_empty());
    }

    #[test]
    fn degenerate_canvas_emits_nothing() {
        let ov = PreviewOverlay {
            corners: [[0.0, 0.0]; 4],
            canvas: (0.0, 0.0),
            locked: false,
        };
        let mut lines = Vec::new();
        let mut tris = Vec::new();
        overlay_vertices(&ov, 200.0, 200.0, &mut lines, &mut tris);
        assert!(lines.is_empty() && tris.is_empty());
    }
}
