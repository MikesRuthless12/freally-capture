//! The compositor core: source textures in, one program frame out.
//!
//! Every visible item of the active scene is drawn back-to-front into the
//! offscreen **program texture** — each with its own transform (from
//! `transform.rs`), blend mode (fixed-function blend state + a small shader
//! prep so transparent pixels never disturb the canvas), and — from P2.3 —
//! its filter chain. The program frame then feeds the preview, and in later
//! phases the encoders and stream.
//!
//! Uploads are honest about capture reality: frames arrive BGRA or RGBA with
//! padded strides; both go straight into a texture of the matching format
//! (no CPU swizzle on the hot path).

use std::collections::HashMap;
use std::num::NonZeroU64;
use std::time::Instant;

use fcap_capture::{Frame, PixelFormat};
use fcap_scene::{BlendMode, FilterId, ItemId, Scene, SourceId};

use crate::filters::{FilterEngine, FilterResourceData, PassPlan};
use crate::gpu::Gpu;
use crate::transform;
use crate::CompositorError;

/// The composed program frame, tightly packed RGBA.
pub struct ProgramFrame {
    pub width: u32,
    pub height: u32,
    /// `width * height * 4` bytes, row-major RGBA.
    pub data: Vec<u8>,
}

/// Per-item uniform, mirrored by `shaders/item.wgsl` (`ItemUniform`).
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct ItemUniform {
    mvp: [[f32; 4]; 4],
    uv_rect: [f32; 4],
    size: [f32; 4],
    misc: [f32; 4],
}

const ITEM_UNIFORM_SIZE: u64 = std::mem::size_of::<ItemUniform>() as u64;
const INITIAL_ITEM_CAPACITY: u64 = 16;

/// One uploaded source: its texture and the bind group sampling it.
struct SourceSlot {
    texture: wgpu::Texture,
    bind_group: wgpu::BindGroup,
    width: u32,
    height: u32,
    format: PixelFormat,
}

/// One intermediate texture of an item's filter chain.
struct ChainTex {
    width: u32,
    height: u32,
    view: wgpu::TextureView,
    /// Samples this texture (the *next* pass's / the composite's input).
    bind_group: wgpu::BindGroup,
}

/// One queued composite draw for the current render.
struct Draw {
    source: SourceId,
    blend: BlendMode,
    uniform_offset: u32,
    /// Sample the item's filter-chain output instead of the raw source.
    chain: Option<ItemId>,
}

/// One queued filter pass for the current render.
struct ChainPass {
    item: ItemId,
    pass_index: usize,
    plan: PassPlan,
    uniform_offset: u32,
}

/// Fixed-function blend state + the shader prep mode for a [`BlendMode`].
fn blend_config(mode: BlendMode) -> (wgpu::BlendState, f32) {
    use wgpu::{BlendComponent, BlendFactor as F, BlendOperation as Op, BlendState};
    let color = |src, dst, op| BlendComponent {
        src_factor: src,
        dst_factor: dst,
        operation: op,
    };
    // The program canvas stays opaque: destination alpha is preserved for the
    // non-normal modes and saturates under Normal.
    let keep_alpha = BlendComponent {
        src_factor: F::Zero,
        dst_factor: F::One,
        operation: Op::Add,
    };
    let over_alpha = BlendComponent {
        src_factor: F::One,
        dst_factor: F::OneMinusSrcAlpha,
        operation: Op::Add,
    };
    match mode {
        BlendMode::Normal => (
            BlendState {
                color: color(F::SrcAlpha, F::OneMinusSrcAlpha, Op::Add),
                alpha: over_alpha,
            },
            0.0,
        ),
        BlendMode::Additive => (
            BlendState {
                color: color(F::SrcAlpha, F::One, Op::Add),
                alpha: keep_alpha,
            },
            0.0,
        ),
        BlendMode::Subtract => (
            BlendState {
                color: color(F::SrcAlpha, F::One, Op::ReverseSubtract),
                alpha: keep_alpha,
            },
            0.0,
        ),
        BlendMode::Screen => (
            BlendState {
                color: color(F::One, F::OneMinusSrc, Op::Add),
                alpha: keep_alpha,
            },
            1.0,
        ),
        BlendMode::Multiply => (
            BlendState {
                color: color(F::Dst, F::Zero, Op::Add),
                alpha: keep_alpha,
            },
            2.0,
        ),
        BlendMode::Lighten => (
            BlendState {
                color: color(F::One, F::One, Op::Max),
                alpha: keep_alpha,
            },
            1.0,
        ),
        BlendMode::Darken => (
            BlendState {
                color: color(F::One, F::One, Op::Min),
                alpha: keep_alpha,
            },
            2.0,
        ),
    }
}

fn texture_format(format: PixelFormat) -> wgpu::TextureFormat {
    match format {
        PixelFormat::Bgra8 => wgpu::TextureFormat::Bgra8Unorm,
        PixelFormat::Rgba8 => wgpu::TextureFormat::Rgba8Unorm,
    }
}

/// The program canvas format: RGBA so the readback feeds JPEG/encoders directly.
const PROGRAM_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;

pub struct Compositor {
    gpu: Gpu,
    canvas_width: u32,
    canvas_height: u32,

    program: wgpu::Texture,
    program_view: wgpu::TextureView,
    readback: Option<wgpu::Buffer>,

    sampler: wgpu::Sampler,
    repeat_sampler: wgpu::Sampler,
    uniform_layout: wgpu::BindGroupLayout,
    texture_layout: wgpu::BindGroupLayout,
    pipelines: Vec<(BlendMode, wgpu::RenderPipeline)>,

    uniform_buffer: wgpu::Buffer,
    uniform_bind: wgpu::BindGroup,
    uniform_capacity: u64,
    uniform_stride: u64,

    sources: HashMap<SourceId, SourceSlot>,

    filters: FilterEngine,
    /// Per-item chain textures, reused frame to frame (keyed by pass index).
    chain_cache: HashMap<ItemId, Vec<ChainTex>>,

    /// CPU time spent encoding + submitting the last render.
    last_render_cpu_micros: u64,
}

impl Compositor {
    /// Bring up the GPU and the render machinery for a `width`×`height`
    /// program canvas.
    pub fn new(width: u32, height: u32) -> Result<Self, CompositorError> {
        let gpu = Gpu::new()?;
        Self::with_gpu(gpu, width, height)
    }

    /// The program canvas may never exceed the adapter's texture limit — an
    /// oversized (hand-edited) collection clamps instead of panicking the
    /// render thread inside wgpu validation.
    fn clamp_canvas(device: &wgpu::Device, size: u32) -> u32 {
        size.clamp(1, device.limits().max_texture_dimension_2d)
    }

    fn with_gpu(gpu: Gpu, width: u32, height: u32) -> Result<Self, CompositorError> {
        let device = &gpu.device;
        let width = Self::clamp_canvas(device, width);
        let height = Self::clamp_canvas(device, height);
        let (program, program_view) = Self::make_program_texture(device, width, height);

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("fcap item sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        // The Scroll filter wraps its content — it samples through this one.
        let repeat_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("fcap repeat sampler"),
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let uniform_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("fcap item uniforms"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size: NonZeroU64::new(ITEM_UNIFORM_SIZE),
                },
                count: None,
            }],
        });

        // Shared by the composite pass and every filter pass: the sampled
        // input texture + a clamp sampler + a repeat sampler (Scroll). A
        // shader may use a subset of the layout's bindings.
        let texture_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("fcap item texture"),
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
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("fcap item shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/item.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("fcap item pipeline layout"),
            bind_group_layouts: &[&uniform_layout, &texture_layout],
            push_constant_ranges: &[],
        });

        let pipelines = BlendMode::ALL
            .iter()
            .map(|&mode| {
                let (blend, _) = blend_config(mode);
                let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("fcap item pipeline"),
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: "vs_main",
                        compilation_options: Default::default(),
                        buffers: &[],
                    },
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleStrip,
                        cull_mode: None,
                        ..Default::default()
                    },
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState::default(),
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: "fs_main",
                        compilation_options: Default::default(),
                        targets: &[Some(wgpu::ColorTargetState {
                            format: PROGRAM_FORMAT,
                            blend: Some(blend),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    multiview: None,
                });
                (mode, pipeline)
            })
            .collect();

        let align = device.limits().min_uniform_buffer_offset_alignment as u64;
        let uniform_stride = ITEM_UNIFORM_SIZE.div_ceil(align) * align;
        let (uniform_buffer, uniform_bind) = Self::make_uniform_buffer(
            device,
            &uniform_layout,
            uniform_stride,
            INITIAL_ITEM_CAPACITY,
        );

        let filters = FilterEngine::new(device, &texture_layout, PROGRAM_FORMAT);

        Ok(Self {
            gpu,
            canvas_width: width,
            canvas_height: height,
            program,
            program_view,
            readback: None,
            sampler,
            repeat_sampler,
            uniform_layout,
            texture_layout,
            pipelines,
            uniform_buffer,
            uniform_bind,
            uniform_capacity: INITIAL_ITEM_CAPACITY,
            uniform_stride,
            sources: HashMap::new(),
            filters,
            chain_cache: HashMap::new(),
            last_render_cpu_micros: 0,
        })
    }

    /// A bind group sampling `view` through the shared texture layout.
    fn make_texture_bind(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        sampler: &wgpu::Sampler,
        repeat_sampler: &wgpu::Sampler,
        view: &wgpu::TextureView,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("fcap sampled texture"),
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(repeat_sampler),
                },
            ],
        })
    }

    fn make_program_texture(
        device: &wgpu::Device,
        width: u32,
        height: u32,
    ) -> (wgpu::Texture, wgpu::TextureView) {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("fcap program"),
            size: wgpu::Extent3d {
                width: width.max(1),
                height: height.max(1),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: PROGRAM_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::COPY_SRC
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        (texture, view)
    }

    fn make_uniform_buffer(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        stride: u64,
        capacity: u64,
    ) -> (wgpu::Buffer, wgpu::BindGroup) {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("fcap item uniforms"),
            size: stride * capacity,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("fcap item uniforms"),
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &buffer,
                    offset: 0,
                    size: NonZeroU64::new(ITEM_UNIFORM_SIZE),
                }),
            }],
        });
        (buffer, bind)
    }

    /// "<name> (<backend>, <device type>)" of the adapter in use.
    pub fn adapter_summary(&self) -> &str {
        &self.gpu.adapter_summary
    }

    // -- native preview surface (the "OBS feel" path) -----------------------

    /// Create a [`NativePreview`] surface on `window` (the child window the
    /// app placed over the preview region), sharing this compositor's device.
    pub fn create_native_preview<W>(
        &self,
        window: W,
        width: u32,
        height: u32,
    ) -> Result<crate::NativePreview, CompositorError>
    where
        W: raw_window_handle::HasWindowHandle
            + raw_window_handle::HasDisplayHandle
            + Send
            + Sync
            + 'static,
    {
        crate::NativePreview::new(&self.gpu, window, width, height)
    }

    /// Blit the current program frame onto the preview surface and present —
    /// no readback. `Ok(false)` if the frame was skipped (surface not ready).
    pub fn present_native(
        &self,
        preview: &mut crate::NativePreview,
    ) -> Result<bool, CompositorError> {
        preview.present(&self.gpu, &self.program_view)
    }

    /// Reconfigure the preview surface after its window resized.
    pub fn resize_native(&self, preview: &mut crate::NativePreview, width: u32, height: u32) {
        preview.resize(&self.gpu, width, height);
    }

    /// CPU cost of the last [`render`](Self::render) (encode + submit).
    pub fn last_render_cpu_micros(&self) -> u64 {
        self.last_render_cpu_micros
    }

    pub fn canvas_size(&self) -> (u32, u32) {
        (self.canvas_width, self.canvas_height)
    }

    /// Resize the program canvas (drops the readback buffer; textures for
    /// sources are untouched). Oversized requests clamp to the adapter limit.
    pub fn set_canvas_size(&mut self, width: u32, height: u32) {
        let width = Self::clamp_canvas(&self.gpu.device, width);
        let height = Self::clamp_canvas(&self.gpu.device, height);
        if (width, height) == (self.canvas_width, self.canvas_height) {
            return;
        }
        self.canvas_width = width;
        self.canvas_height = height;
        let (program, view) = Self::make_program_texture(&self.gpu.device, width, height);
        self.program = program;
        self.program_view = view;
        self.readback = None;
    }

    /// The last uploaded size of a source, if any frame arrived yet.
    pub fn source_size(&self, source: SourceId) -> Option<(u32, u32)> {
        self.sources
            .get(&source)
            .map(|slot| (slot.width, slot.height))
    }

    /// Drop a source's texture (its item disappeared / its session stopped).
    pub fn remove_source(&mut self, source: SourceId) {
        self.sources.remove(&source);
    }

    /// Drop every source texture not in `keep` (scene switches).
    pub fn retain_sources(&mut self, keep: &[SourceId]) {
        self.sources.retain(|id, _| keep.contains(id));
    }

    /// Upload a captured frame into `source`'s texture, (re)creating it when
    /// the size or pixel format changed. Rejects frames whose geometry does
    /// not hold together rather than reading out of bounds.
    pub fn upload_frame(&mut self, source: SourceId, frame: &Frame) -> Result<(), CompositorError> {
        if frame.width == 0 || frame.height == 0 {
            return Err(CompositorError::BadFrame("zero-sized frame".into()));
        }
        if frame.stride < frame.width * 4 {
            return Err(CompositorError::BadFrame(format!(
                "stride {} shorter than a {}px row",
                frame.stride, frame.width
            )));
        }
        let needed = frame.stride as usize * frame.height as usize;
        if frame.data.len() < needed {
            return Err(CompositorError::BadFrame(format!(
                "frame holds {} bytes, geometry needs {needed}",
                frame.data.len()
            )));
        }
        let max_dim = self.gpu.device.limits().max_texture_dimension_2d;
        if frame.width > max_dim || frame.height > max_dim {
            return Err(CompositorError::BadFrame(format!(
                "{}×{} exceeds the adapter's {max_dim}px texture limit",
                frame.width, frame.height
            )));
        }

        let needs_new = match self.sources.get(&source) {
            Some(slot) => {
                slot.width != frame.width
                    || slot.height != frame.height
                    || slot.format != frame.format
            }
            None => true,
        };
        if needs_new {
            let device = &self.gpu.device;
            let texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("fcap source"),
                size: wgpu::Extent3d {
                    width: frame.width,
                    height: frame.height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: texture_format(frame.format),
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });
            let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
            let bind_group = Self::make_texture_bind(
                device,
                &self.texture_layout,
                &self.sampler,
                &self.repeat_sampler,
                &view,
            );
            self.sources.insert(
                source,
                SourceSlot {
                    texture,
                    bind_group,
                    width: frame.width,
                    height: frame.height,
                    format: frame.format,
                },
            );
        }

        let slot = self.sources.get(&source).expect("slot was just ensured");
        self.gpu.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &slot.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &frame.data[..needed],
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(frame.stride),
                rows_per_image: Some(frame.height),
            },
            wgpu::Extent3d {
                width: frame.width,
                height: frame.height,
                depth_or_array_layers: 1,
            },
        );
        Ok(())
    }

    fn ensure_uniform_capacity(&mut self, items: u64) {
        if items <= self.uniform_capacity {
            return;
        }
        let capacity = items.next_power_of_two();
        let (buffer, bind) = Self::make_uniform_buffer(
            &self.gpu.device,
            &self.uniform_layout,
            self.uniform_stride,
            capacity,
        );
        self.uniform_buffer = buffer;
        self.uniform_bind = bind;
        self.uniform_capacity = capacity;
    }

    /// Make sure `item`'s chain has a texture of `size` at `pass_index`,
    /// (re)creating it when missing or mis-sized.
    fn ensure_chain_texture(&mut self, item: ItemId, pass_index: usize, size: (u32, u32)) {
        let chain = self.chain_cache.entry(item).or_default();
        if let Some(existing) = chain.get(pass_index) {
            if (existing.width, existing.height) == size {
                return;
            }
        }
        let device = &self.gpu.device;
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("fcap chain"),
            size: wgpu::Extent3d {
                width: size.0,
                height: size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: PROGRAM_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let bind_group = Self::make_texture_bind(
            device,
            &self.texture_layout,
            &self.sampler,
            &self.repeat_sampler,
            &view,
        );
        let entry = ChainTex {
            width: size.0,
            height: size.1,
            view,
            bind_group,
        };
        if pass_index < chain.len() {
            chain[pass_index] = entry;
        } else {
            chain.push(entry);
        }
    }

    /// Compose `scene` into the program texture. Items render back-to-front
    /// (`items[0]` first); invisible items, items whose source has produced
    /// no frame yet, and fully-cropped items are skipped. Enabled filters run
    /// first, each item through its own GPU chain; `time_seconds` drives the
    /// time-based ones (Scroll).
    pub fn render(&mut self, scene: &Scene, time_seconds: f32) -> Result<(), CompositorError> {
        let started = Instant::now();
        let canvas = (self.canvas_width as f32, self.canvas_height as f32);

        // Plan: filter passes + composite uniforms, all staged up front.
        let mut item_staging: Vec<u8> = Vec::new();
        let mut filter_staging: Vec<u8> = Vec::new();
        let mut draws: Vec<Draw> = Vec::new();
        let mut chain_passes: Vec<ChainPass> = Vec::new();
        let mut live_chains: Vec<ItemId> = Vec::new();

        for item in scene.items.iter().filter(|item| item.visible) {
            let Some(slot) = self.sources.get(&item.source) else {
                continue; // no frame yet
            };
            let source_size = (slot.width, slot.height);

            // Plan this item's enabled filters into concrete passes.
            let mut plans: Vec<PassPlan> = Vec::new();
            let mut chain_size = source_size;
            for filter in item.filters.iter().filter(|filter| filter.enabled) {
                if let Some(passes) = crate::filters::plan_filter(
                    &filter.kind,
                    filter.id,
                    chain_size,
                    time_seconds,
                    self.filters.resources(),
                ) {
                    chain_size = passes.last().expect("plans are non-empty").out;
                    plans.extend(passes);
                }
            }

            // The composite sees the chain's output (or the raw source).
            let Some(content) =
                transform::content_size(chain_size.0, chain_size.1, &item.transform.crop)
            else {
                continue; // fully cropped away
            };
            let (_, prep) = blend_config(item.blend);
            let uniform = ItemUniform {
                mvp: transform::clip_matrix(&item.transform, content, canvas),
                uv_rect: transform::uv_rect(chain_size.0, chain_size.1, &item.transform.crop),
                size: [content.0, content.1, 0.0, 0.0],
                misc: [prep, 0.0, 0.0, 0.0],
            };
            let offset = draws.len() as u64 * self.uniform_stride;
            item_staging.resize(offset as usize, 0);
            item_staging.extend_from_slice(bytemuck::bytes_of(&uniform));

            let has_chain = !plans.is_empty();
            if has_chain {
                live_chains.push(item.id);
                let pass_count = plans.len();
                for (pass_index, plan) in plans.into_iter().enumerate() {
                    self.ensure_chain_texture(item.id, pass_index, plan.out);
                    let filter_offset = chain_passes.len() as u64 * self.filters.uniform_stride;
                    filter_staging.resize(filter_offset as usize, 0);
                    filter_staging.extend_from_slice(bytemuck::bytes_of(&plan.uniform));
                    chain_passes.push(ChainPass {
                        item: item.id,
                        pass_index,
                        plan,
                        uniform_offset: filter_offset as u32,
                    });
                }
                // A shrunken chain must not leave stale textures behind — the
                // composite samples chain.last(), which has to be this
                // frame's final pass, not a leftover from a longer chain.
                if let Some(chain) = self.chain_cache.get_mut(&item.id) {
                    chain.truncate(pass_count);
                }
            }
            draws.push(Draw {
                source: item.source,
                blend: item.blend,
                uniform_offset: offset as u32,
                chain: has_chain.then_some(item.id),
            });
        }

        // Chain textures for items that no longer filter are dropped so a
        // toggled-off chain never pins GPU memory.
        self.chain_cache.retain(|id, _| live_chains.contains(id));

        self.ensure_uniform_capacity(draws.len() as u64);
        self.filters
            .ensure_capacity(&self.gpu.device, chain_passes.len() as u64);
        if !item_staging.is_empty() {
            self.gpu
                .queue
                .write_buffer(&self.uniform_buffer, 0, &item_staging);
        }
        if !filter_staging.is_empty() {
            self.gpu
                .queue
                .write_buffer(&self.filters.uniform_buffer, 0, &filter_staging);
        }

        let mut encoder = self
            .gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("fcap compose"),
            });

        // 1. Filter chains — one pass per planned filter stage.
        for chain_pass in &chain_passes {
            let chain = &self.chain_cache[&chain_pass.item];
            let target = &chain[chain_pass.pass_index].view;
            let input_bind = if chain_pass.pass_index == 0 {
                let source = draws
                    .iter()
                    .find(|draw| draw.chain == Some(chain_pass.item))
                    .map(|draw| draw.source)
                    .expect("chained items have a draw");
                &self
                    .sources
                    .get(&source)
                    .expect("chained items have a live slot")
                    .bind_group
            } else {
                &chain[chain_pass.pass_index - 1].bind_group
            };
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("fcap filter pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: target,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            pass.set_pipeline(self.filters.pipeline(chain_pass.plan.kind));
            pass.set_bind_group(0, &self.filters.uniform_bind, &[chain_pass.uniform_offset]);
            pass.set_bind_group(1, input_bind, &[]);
            if let Some(resource) = chain_pass.plan.resource {
                let bind = self
                    .filters
                    .resource_bind(resource)
                    .expect("planned passes only reference loaded resources");
                pass.set_bind_group(2, bind, &[]);
            }
            pass.draw(0..3, 0..1);
        }

        // 2. The program pass — composite every item in z-order.
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("fcap program pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.program_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            let mut bound_blend = None;
            for draw in &draws {
                if bound_blend != Some(draw.blend) {
                    let pipeline = self
                        .pipelines
                        .iter()
                        .find(|(mode, _)| *mode == draw.blend)
                        .map(|(_, pipeline)| pipeline)
                        .expect("every blend mode has a pipeline");
                    pass.set_pipeline(pipeline);
                    bound_blend = Some(draw.blend);
                }
                let texture_bind = match draw.chain {
                    Some(item) => {
                        let chain = &self.chain_cache[&item];
                        &chain.last().expect("chains are non-empty").bind_group
                    }
                    None => {
                        &self
                            .sources
                            .get(&draw.source)
                            .expect("draws reference live slots")
                            .bind_group
                    }
                };
                pass.set_bind_group(0, &self.uniform_bind, &[draw.uniform_offset]);
                pass.set_bind_group(1, texture_bind, &[]);
                pass.draw(0..4, 0..1);
            }
        }
        self.gpu.queue.submit(Some(encoder.finish()));
        self.last_render_cpu_micros = started.elapsed().as_micros() as u64;
        Ok(())
    }

    // -- filter resources (LUT lattices, mask images) ------------------------

    /// Upload the decoded file a filter samples (LUT lattice / mask image),
    /// keyed by the filter's id. The app layer loads and decodes; filters
    /// whose resource has not arrived are skipped, never rendered black.
    pub fn set_filter_resource(
        &mut self,
        filter: FilterId,
        data: &FilterResourceData,
    ) -> Result<(), CompositorError> {
        self.filters.set_resource(
            &self.gpu.device,
            &self.gpu.queue,
            &self.sampler,
            filter,
            data,
        )
    }

    /// Drop one filter's uploaded resource.
    pub fn remove_filter_resource(&mut self, filter: FilterId) {
        self.filters.remove_resource(filter);
    }

    /// Drop every uploaded resource not in `keep`.
    pub fn retain_filter_resources(&mut self, keep: &[FilterId]) {
        self.filters.retain_resources(keep);
    }

    /// Whether a resource is currently uploaded for `filter`.
    pub fn has_filter_resource(&self, filter: FilterId) -> bool {
        self.filters.resource_bind(filter).is_some()
    }

    /// Read the current program texture back to the CPU (tight RGBA rows).
    /// Blocking — call from the render thread, at the consumer's pace.
    pub fn read_program(&mut self) -> Result<ProgramFrame, CompositorError> {
        let width = self.canvas_width;
        let height = self.canvas_height;
        let unpadded = width as u64 * 4;
        let padded = unpadded.div_ceil(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT as u64)
            * wgpu::COPY_BYTES_PER_ROW_ALIGNMENT as u64;
        let size = padded * height as u64;

        // (`map_or`, not `is_none_or` — the declared MSRV is 1.80.)
        if self
            .readback
            .as_ref()
            .map_or(true, |buffer| buffer.size() != size)
        {
            self.readback = Some(self.gpu.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("fcap readback"),
                size,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            }));
        }
        let buffer = self.readback.as_ref().expect("readback buffer ensured");

        let mut encoder = self
            .gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("fcap readback"),
            });
        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                texture: &self.program,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::ImageCopyBuffer {
                buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(padded as u32),
                    rows_per_image: Some(height),
                },
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );
        self.gpu.queue.submit(Some(encoder.finish()));

        let slice = buffer.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |result| {
            let _ = tx.send(result);
        });
        self.gpu.device.poll(wgpu::Maintain::Wait);
        rx.recv()
            .map_err(|_| CompositorError::Readback("map callback dropped".into()))?
            .map_err(|err| CompositorError::Readback(err.to_string()))?;

        let mut data = Vec::with_capacity((unpadded * height as u64) as usize);
        {
            let mapped = slice.get_mapped_range();
            for row in 0..height as usize {
                let start = row * padded as usize;
                data.extend_from_slice(&mapped[start..start + unpadded as usize]);
            }
        }
        buffer.unmap();

        Ok(ProgramFrame {
            width,
            height,
            data,
        })
    }
}
