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
use fcap_scene::{
    BlendMode, FilterId, ItemId, Scene, SceneId, SourceId, Transform, TransitionKind,
};

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
    /// The scene item this draw is for — its id keys the freeze snapshot.
    item: ItemId,
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
    /// The optional second output canvas (Phase 6: e.g. vertical 9:16),
    /// rendered on demand from its own scene.
    vertical: Option<(wgpu::Texture, wgpu::TextureView, u32, u32)>,
    /// A small reusable target for multiview thumbnails (CAP-M06) — composing
    /// at full canvas dims into a tiny texture just downscales, so the readback
    /// (which dominates) is ~36× cheaper than a full-res program readback.
    thumbnail: Option<(wgpu::Texture, wgpu::TextureView, u32, u32)>,
    /// Readback staging per target size (program / vertical) — keyed so the
    /// two canvases never thrash one buffer.
    readback: HashMap<(u32, u32), wgpu::Buffer>,

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
    /// Per-item held snapshots for the freeze-frame filter (CAP-N25). A frozen
    /// item samples its snapshot instead of the live source, so a clone of the
    /// same source (or another placement) keeps updating.
    frozen: HashMap<ItemId, SourceSlot>,

    /// Nested scenes (Phase 6): the scenes a scene-source can reference and
    /// the source→scene mapping, refreshed by the studio each tick. A
    /// nested source's composed frame lives in `sources` under its id, so
    /// transforms/filters/blends apply to it like any capture.
    scene_pool: Vec<Scene>,
    scene_refs: HashMap<SourceId, SceneId>,
    /// The punch-in zoom lenses (CAP-N71): per-item `(zoom, anchor)` the
    /// studio animates each tick — applied to the *drawn* transform only,
    /// so the model (and undo) never see them.
    lenses: HashMap<ItemId, (f32, (f32, f32))>,

    filters: FilterEngine,
    /// Per-item chain textures, reused frame to frame (keyed by pass index).
    chain_cache: HashMap<ItemId, Vec<ChainTex>>,

    /// Studio Mode's transition machinery, built on first use and rebuilt on
    /// a canvas resize (its scratch textures are canvas-sized).
    transition: Option<TransitionRig>,
    /// The custom luma-wipe image (CPU copy — survives rig rebuilds) and its
    /// change counter.
    transition_luma: Option<(u32, u32, Vec<u8>)>,
    transition_luma_epoch: u64,
    /// The stinger overlay: a fullscreen alpha-over pipeline + the frame
    /// texture, built on first use.
    stinger: Option<StingerRig>,
    /// Floating reactions (TASK-614): the sprite pass + its uploaded emoji
    /// sprites, built on first use.
    reactions: Option<ReactionRig>,

    /// CPU time spent encoding + submitting the last render.
    last_render_cpu_micros: u64,
}

/// Everything one blended transition frame needs: the fullscreen pipeline,
/// its 16-byte uniform, two canvas-sized scratch scenes (A = outgoing,
/// B = incoming) with bind groups the pass samples, and the luma-wipe
/// image slot (a 1×1 white dummy until a custom image is set).
struct TransitionRig {
    pipeline: wgpu::RenderPipeline,
    uniform_buffer: wgpu::Buffer,
    uniform_bind: wgpu::BindGroup,
    scratch: [(wgpu::Texture, wgpu::TextureView, wgpu::BindGroup); 2],
    luma_bind: wgpu::BindGroup,
    /// Which CPU copy the current `luma_bind` was built from (a counter —
    /// bumped by [`Compositor::set_transition_luma`]).
    luma_epoch: u64,
    width: u32,
    height: u32,
}

/// The stinger overlay (Phase 6): a fullscreen textured triangle drawn
/// straight-alpha-over the rendered scene while the stinger video plays.
struct StingerRig {
    pipeline: wgpu::RenderPipeline,
    /// The uploaded stinger frame, recreated on size/format change.
    texture: Option<(wgpu::Texture, wgpu::BindGroup, u32, u32, PixelFormat)>,
}

/// One reaction particle to draw this frame (canvas pixels; alpha 0..1).
#[derive(Debug, Clone)]
pub struct ReactionDraw {
    /// Which uploaded sprite (see [`Compositor::set_reaction_sprite`]).
    pub sprite: String,
    pub x: f32,
    pub y: f32,
    /// Sprite size on the canvas, px (square-ish; the sprite's aspect holds).
    pub size: f32,
    pub alpha: f32,
}

/// One downstream-keyer layer to draw over the finished program (CAP-N24).
#[derive(Debug, Clone)]
pub struct DownstreamDraw {
    /// The overlay's live source (its uploaded texture is composited on top).
    pub source: SourceId,
    /// Where/how it sits — the same transform a scene item uses (2D or 3D).
    pub transform: Transform,
    /// Layer opacity, 0..=1.
    pub opacity: f32,
}

/// The floating-reactions pass (TASK-614): a bounded pool of textured
/// quads drawn alpha-over the composed program.
struct ReactionRig {
    pipeline: wgpu::RenderPipeline,
    uniform_buffer: wgpu::Buffer,
    uniform_bind: wgpu::BindGroup,
    uniform_stride: u64,
    sprites: HashMap<String, SourceSlot>,
}

/// Reactions never draw more than this many sprites per frame — the pool
/// bound that keeps a reaction flood from ever touching the encoder.
pub const REACTION_POOL: usize = 64;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct ReactionUniform {
    rect: [f32; 4],
    tint: [f32; 4],
}

/// `[progress, mode, dir.x, dir.y]` for the transition shader. `dir` is the
/// direction the incoming scene enters from (uv space, y down).
fn transition_params(kind: TransitionKind, progress: f32) -> [f32; 4] {
    match kind {
        TransitionKind::Cut | TransitionKind::Fade => [progress, 0.0, 0.0, 0.0],
        TransitionKind::SlideLeft => [progress, 1.0, 1.0, 0.0],
        TransitionKind::SlideRight => [progress, 1.0, -1.0, 0.0],
        TransitionKind::SlideUp => [progress, 1.0, 0.0, 1.0],
        TransitionKind::SlideDown => [progress, 1.0, 0.0, -1.0],
        TransitionKind::SwipeLeft => [progress, 2.0, 1.0, 0.0],
        TransitionKind::SwipeRight => [progress, 2.0, -1.0, 0.0],
        TransitionKind::LumaLinear => [progress, 3.0, 0.0, 0.0],
        TransitionKind::LumaRadial => [progress, 4.0, 0.0, 0.0],
        TransitionKind::LumaHorizontal => [progress, 5.0, 0.0, 0.0],
        TransitionKind::LumaDiamond => [progress, 6.0, 0.0, 0.0],
        TransitionKind::LumaClock => [progress, 7.0, 0.0, 0.0],
        TransitionKind::LumaImage => [progress, 8.0, 0.0, 0.0],
        // The stinger never blends here — the app renders the underlying
        // scene and overlays the video (`render_scene_with_stinger`); a
        // stray call falls back to a fade.
        TransitionKind::Stinger => [progress, 0.0, 0.0, 0.0],
    }
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
                    cache: None,
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: Some("vs_main"),
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
                        entry_point: Some("fs_main"),
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
            vertical: None,
            thumbnail: None,
            readback: HashMap::new(),
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
            frozen: HashMap::new(),
            scene_pool: Vec::new(),
            lenses: HashMap::new(),
            scene_refs: HashMap::new(),
            filters,
            chain_cache: HashMap::new(),
            transition: None,
            transition_luma: None,
            transition_luma_epoch: 0,
            stinger: None,
            reactions: None,
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

    /// The wgpu instance this compositor's device came from — the
    /// DirectComposition overlay builds its surface against the same adapter.
    pub fn instance(&self) -> &wgpu::Instance {
        &self.gpu.instance
    }

    /// True when the compositor is on the DX12 backend — the only backend that
    /// can build a DirectComposition `CompositionVisual` surface. The app gates
    /// the native preview on this so a non-DX12 machine stays on the JPEG path
    /// instead of advertising a native surface it can never present.
    pub fn is_dx12(&self) -> bool {
        self.gpu.is_dx12
    }

    /// True when the compositor is on the Metal backend — the backend that can
    /// build a `CoreAnimationLayer` (CAMetalLayer) surface for the macOS native
    /// preview. The app gates the native preview on DX12-or-Metal so other
    /// backends stay on the JPEG path instead of advertising an unpresentable
    /// native surface.
    pub fn is_metal(&self) -> bool {
        self.gpu.is_metal
    }

    /// True on the Vulkan or GL backend — the Linux native preview renders into
    /// an X11 child window (wgpu `RawHandle`). Gated alongside `is_dx12` /
    /// `is_metal` so only a backend whose surface the OS can composite over the
    /// webview advertises the native path.
    pub fn is_vulkan_or_gl(&self) -> bool {
        self.gpu.is_vulkan_or_gl
    }

    /// Create a [`NativePreview`] from a surface the caller already built (the
    /// DirectComposition overlay path in `fcap-preview`), sharing this
    /// compositor's device.
    pub fn native_preview_from_surface(
        &self,
        surface: wgpu::Surface<'static>,
        width: u32,
        height: u32,
    ) -> Result<crate::NativePreview, CompositorError> {
        crate::NativePreview::from_surface(&self.gpu, surface, width, height)
    }

    /// Blit the current program frame onto the preview surface and present —
    /// no readback. `Ok(false)` if the frame was skipped (surface not ready).
    pub fn present_native(
        &self,
        preview: &mut crate::NativePreview,
        overlay: Option<&crate::PreviewOverlay>,
    ) -> Result<bool, CompositorError> {
        preview.present(&self.gpu, &self.program_view, overlay)
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
        self.readback.clear();
        // The transition scratch textures are canvas-sized — rebuild lazily.
        self.transition = None;
    }

    /// Enable/resize (or drop, with `None`) the second output canvas. A
    /// no-op when the size already matches — safe to call every tick.
    pub fn set_vertical_canvas(&mut self, size: Option<(u32, u32)>) {
        match size {
            None => {
                if self.vertical.take().is_some() {
                    self.readback.clear();
                }
            }
            Some((width, height)) => {
                let width = Self::clamp_canvas(&self.gpu.device, width.max(1));
                let height = Self::clamp_canvas(&self.gpu.device, height.max(1));
                if self
                    .vertical
                    .as_ref()
                    .is_some_and(|(_, _, w, h)| (*w, *h) == (width, height))
                {
                    return;
                }
                let (texture, view) = Self::make_program_texture(&self.gpu.device, width, height);
                self.vertical = Some((texture, view, width, height));
                // The readback map is keyed by target size; a resized vertical
                // canvas orphans its old-size staging buffer. Drop the whole
                // map (the program buffer re-creates on its next readback) so
                // stepping through sizes can't leak GPU staging memory.
                self.readback.clear();
            }
        }
    }

    /// Compose `scene` into the second canvas and read it back — one
    /// vertical-output frame. Errors when no vertical canvas is set.
    pub fn render_vertical(
        &mut self,
        scene: &Scene,
        time_seconds: f32,
    ) -> Result<ProgramFrame, CompositorError> {
        let (texture, view, width, height) = {
            let (texture, view, width, height) = self.vertical.as_ref().ok_or_else(|| {
                CompositorError::BadFrame("no vertical canvas is configured".into())
            })?;
            (texture.clone(), view.clone(), *width, *height)
        };
        self.render_to(
            scene,
            time_seconds,
            view,
            (width as f32, height as f32),
            false,
        )?;
        self.read_texture(&texture)
    }

    /// Build (or rebuild after a resize) the transition pipeline + its two
    /// canvas-sized scratch scenes.
    fn ensure_transition_rig(&mut self) {
        let (width, height) = (self.canvas_width, self.canvas_height);
        if self
            .transition
            .as_ref()
            .is_some_and(|rig| rig.width == width && rig.height == height)
        {
            return;
        }
        let device = &self.gpu.device;

        let uniform_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("fcap transition uniform"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: NonZeroU64::new(16),
                },
                count: None,
            }],
        });
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("fcap transition params"),
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let uniform_bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("fcap transition params"),
            layout: &uniform_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("fcap transition shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/transition.wgsl").into()),
        });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("fcap transition layout"),
            bind_group_layouts: &[
                &uniform_layout,
                &self.texture_layout,
                &self.texture_layout,
                &self.texture_layout, // the luma-wipe image (or its dummy)
            ],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("fcap transition pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: PROGRAM_FORMAT,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let scratch = [0, 1].map(|_| {
            let (texture, view) = Self::make_program_texture(device, width, height);
            let bind = Self::make_texture_bind(
                device,
                &self.texture_layout,
                &self.sampler,
                &self.repeat_sampler,
                &view,
            );
            (texture, view, bind)
        });
        let luma_bind = self.make_transition_luma_bind();

        self.transition = Some(TransitionRig {
            pipeline,
            uniform_buffer,
            uniform_bind,
            scratch,
            luma_bind,
            luma_epoch: self.transition_luma_epoch,
            width,
            height,
        });
    }

    /// Build the group-3 bind for the current luma image (a 1×1 white dummy
    /// when none is set — mode 8 then reads as a hard cut at p=1).
    fn make_transition_luma_bind(&self) -> wgpu::BindGroup {
        let device = &self.gpu.device;
        let (width, height, pixels): (u32, u32, &[u8]) = match &self.transition_luma {
            Some((width, height, data)) => (*width, *height, data.as_slice()),
            None => (1, 1, &[0xff, 0xff, 0xff, 0xff]),
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("fcap transition luma"),
            size: wgpu::Extent3d {
                width: width.max(1),
                height: height.max(1),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        self.gpu.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            pixels,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(width.max(1) * 4),
                rows_per_image: Some(height.max(1)),
            },
            wgpu::Extent3d {
                width: width.max(1),
                height: height.max(1),
                depth_or_array_layers: 1,
            },
        );
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        Self::make_texture_bind(
            device,
            &self.texture_layout,
            &self.sampler,
            &self.repeat_sampler,
            &view,
        )
    }

    /// Set (or clear) the custom luma-wipe image — tight RGBA rows. The CPU
    /// copy survives canvas resizes; the GPU side rebuilds lazily.
    pub fn set_transition_luma(&mut self, image: Option<(u32, u32, Vec<u8>)>) {
        let same = match (&self.transition_luma, &image) {
            (None, None) => true,
            (Some(a), Some(b)) => *a == *b,
            _ => false,
        };
        if same {
            return;
        }
        self.transition_luma = image;
        self.transition_luma_epoch += 1;
    }

    /// Compose BOTH scenes and blend them into the program texture — one
    /// Studio-Mode transition frame at `progress` (0→1). `Cut` (or a finished
    /// progress) renders the incoming scene directly.
    pub fn render_transition(
        &mut self,
        from: &Scene,
        to: &Scene,
        kind: TransitionKind,
        progress: f32,
        time_seconds: f32,
    ) -> Result<(), CompositorError> {
        if matches!(kind, TransitionKind::Cut) || progress >= 1.0 {
            return self.render(to, time_seconds);
        }
        self.ensure_transition_rig();
        // A luma image set/cleared since the rig last built its group-3 bind.
        if self
            .transition
            .as_ref()
            .is_some_and(|rig| rig.luma_epoch != self.transition_luma_epoch)
        {
            let bind = self.make_transition_luma_bind();
            let epoch = self.transition_luma_epoch;
            if let Some(rig) = self.transition.as_mut() {
                rig.luma_bind = bind;
                rig.luma_epoch = epoch;
            }
        }

        let (from_view, to_view) = {
            let rig = self.transition.as_ref().expect("ensured above");
            (rig.scratch[0].1.clone(), rig.scratch[1].1.clone())
        };
        let canvas = (self.canvas_width as f32, self.canvas_height as f32);
        self.render_to(from, time_seconds, from_view, canvas, false)?;
        self.render_to(to, time_seconds, to_view, canvas, false)?;

        let rig = self.transition.as_ref().expect("ensured above");
        self.gpu.queue.write_buffer(
            &rig.uniform_buffer,
            0,
            bytemuck::bytes_of(&transition_params(kind, progress.clamp(0.0, 1.0))),
        );
        let mut encoder = self
            .gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("fcap transition"),
            });
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("fcap transition pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.program_view,
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
            pass.set_pipeline(&rig.pipeline);
            pass.set_bind_group(0, &rig.uniform_bind, &[]);
            pass.set_bind_group(1, &rig.scratch[0].2, &[]);
            pass.set_bind_group(2, &rig.scratch[1].2, &[]);
            pass.set_bind_group(3, &rig.luma_bind, &[]);
            pass.draw(0..3, 0..1);
        }
        self.gpu.queue.submit(Some(encoder.finish()));
        Ok(())
    }

    /// One stinger-transition frame (Phase 6): render `scene` (the side of
    /// the cut the audience should see), then draw the newest stinger video
    /// frame straight-alpha-over it. `frame` is `None` while the stinger
    /// file spins up — the scene shows bare, honestly.
    pub fn render_scene_with_stinger(
        &mut self,
        scene: &Scene,
        time_seconds: f32,
        frame: Option<&Frame>,
    ) -> Result<(), CompositorError> {
        self.render(scene, time_seconds)?;
        self.ensure_stinger_rig();
        if let Some(frame) = frame {
            self.upload_stinger_frame(frame)?;
        }
        // Between video frames the last uploaded one keeps covering the swap
        // (the render loop outpaces the stinger's fps).
        let rig = self.stinger.as_ref().expect("ensured above");
        let Some((_, bind, _, _, _)) = rig.texture.as_ref() else {
            return Ok(());
        };
        let mut encoder = self
            .gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("fcap stinger"),
            });
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("fcap stinger pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.program_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load, // over the rendered scene
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            pass.set_pipeline(&rig.pipeline);
            pass.set_bind_group(0, bind, &[]);
            pass.draw(0..3, 0..1);
        }
        self.gpu.queue.submit(Some(encoder.finish()));
        Ok(())
    }

    /// Forget the previous stinger's last frame (called when a new stinger
    /// transition starts, so it can never flash stale pixels).
    pub fn reset_stinger(&mut self) {
        if let Some(rig) = self.stinger.as_mut() {
            rig.texture = None;
        }
    }

    /// Upload (or replace) one emoji sprite the reaction pass samples —
    /// rasterized app-side (tight or padded rows, like any source frame).
    pub fn set_reaction_sprite(&mut self, key: &str, frame: &Frame) -> Result<(), CompositorError> {
        self.ensure_reaction_rig();
        if frame.width == 0 || frame.height == 0 || frame.stride < frame.width * 4 {
            return Err(CompositorError::BadFrame("bad reaction sprite".into()));
        }
        let needed = frame.stride as usize * frame.height as usize;
        if frame.data.len() < needed {
            return Err(CompositorError::BadFrame(
                "reaction sprite shorter than its geometry".into(),
            ));
        }
        let device = &self.gpu.device;
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("fcap reaction sprite"),
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
        self.gpu.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &frame.data[..needed],
            wgpu::TexelCopyBufferLayout {
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
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let bind_group = Self::make_texture_bind(
            device,
            &self.texture_layout,
            &self.sampler,
            &self.repeat_sampler,
            &view,
        );
        let slot = SourceSlot {
            texture,
            bind_group,
            width: frame.width,
            height: frame.height,
            format: frame.format,
        };
        if let Some(rig) = self.reactions.as_mut() {
            rig.sprites.insert(key.to_string(), slot);
        }
        Ok(())
    }

    /// Whether a sprite is already uploaded (the app rasterizes lazily).
    pub fn has_reaction_sprite(&self, key: &str) -> bool {
        self.reactions
            .as_ref()
            .is_some_and(|rig| rig.sprites.contains_key(key))
    }

    /// Draw this frame's reaction particles alpha-over the composed
    /// program — the pass that bakes them into what records and streams.
    /// Hard-capped at [`REACTION_POOL`] draws; unknown sprites skip.
    pub fn render_reactions(&mut self, draws: &[ReactionDraw]) -> Result<(), CompositorError> {
        if draws.is_empty() {
            return Ok(());
        }
        self.ensure_reaction_rig();
        let (canvas_w, canvas_h) = (self.canvas_width as f32, self.canvas_height as f32);

        // Stage the uniforms for every drawable particle.
        let mut staging: Vec<u8> = Vec::new();
        let mut jobs: Vec<(String, u32)> = Vec::new();
        {
            let rig = self.reactions.as_ref().expect("ensured above");
            for draw in draws.iter().take(REACTION_POOL) {
                let Some(sprite) = rig.sprites.get(&draw.sprite) else {
                    continue;
                };
                let aspect = sprite.height.max(1) as f32 / sprite.width.max(1) as f32;
                let w_clip = (draw.size / canvas_w) * 2.0;
                let h_clip = (draw.size * aspect / canvas_h) * 2.0;
                let x_clip = (draw.x / canvas_w) * 2.0 - 1.0;
                let y_clip = 1.0 - (draw.y / canvas_h) * 2.0;
                let uniform = ReactionUniform {
                    rect: [x_clip, y_clip, w_clip, h_clip],
                    tint: [draw.alpha.clamp(0.0, 1.0), 0.0, 0.0, 0.0],
                };
                let offset = jobs.len() as u64 * rig.uniform_stride;
                staging.resize(offset as usize, 0);
                staging.extend_from_slice(bytemuck::bytes_of(&uniform));
                jobs.push((draw.sprite.clone(), offset as u32));
            }
        }
        if jobs.is_empty() {
            return Ok(());
        }
        {
            let rig = self.reactions.as_ref().expect("ensured above");
            self.gpu
                .queue
                .write_buffer(&rig.uniform_buffer, 0, &staging);
        }

        let mut encoder = self
            .gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("fcap reactions"),
            });
        {
            let rig = self.reactions.as_ref().expect("ensured above");
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("fcap reactions pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.program_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load, // over the composed program
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            pass.set_pipeline(&rig.pipeline);
            for (sprite, offset) in &jobs {
                let slot = rig.sprites.get(sprite).expect("staged above");
                pass.set_bind_group(0, &rig.uniform_bind, &[*offset]);
                pass.set_bind_group(1, &slot.bind_group, &[]);
                pass.draw(0..4, 0..1);
            }
        }
        self.gpu.queue.submit(Some(encoder.finish()));
        Ok(())
    }

    /// Composite the downstream-keyer layers over the finished program
    /// (CAP-N24): persistent overlays that ride ON TOP of every scene and
    /// survive scene cuts. Drawn after compose/transition (and reactions),
    /// before readback, so program, preview, recording, and stream all carry
    /// them. Each layer is a live source drawn through its own transform (2D or
    /// 3D) at its own opacity, straight-alpha over the program — reusing the
    /// item pipeline, so filters/blend behave exactly like a scene item.
    pub fn render_downstream(&mut self, draws: &[DownstreamDraw]) -> Result<(), CompositorError> {
        let canvas = (self.canvas_width as f32, self.canvas_height as f32);
        // Resolve each enabled layer to a live source slot + its item uniform.
        // (Owned data, so the immutable `self.sources` borrow is released before
        // the mutable capacity grow below.)
        let mut jobs: Vec<(SourceId, ItemUniform)> = Vec::new();
        for draw in draws {
            let Some(slot) = self.sources.get(&draw.source) else {
                continue; // no live feed yet — nothing to key
            };
            let (sw, sh) = (slot.width, slot.height);
            let Some(content) = transform::content_size(sw, sh, &draw.transform.crop) else {
                continue; // cropped away
            };
            let mvp = if draw.transform.has_3d() {
                transform::perspective_clip_matrix(&draw.transform, content, canvas)
            } else {
                transform::clip_matrix(&draw.transform, content, canvas)
            };
            jobs.push((
                draw.source,
                ItemUniform {
                    mvp,
                    uv_rect: transform::uv_rect(sw, sh, &draw.transform.crop),
                    size: [content.0, content.1, 0.0, 0.0],
                    misc: [
                        0.0,
                        0.0,
                        draw.transform.scale_x.abs(),
                        draw.opacity.clamp(0.0, 1.0),
                    ],
                },
            ));
        }
        if jobs.is_empty() {
            return Ok(());
        }

        self.ensure_uniform_capacity(jobs.len() as u64);
        let mut staging: Vec<u8> = Vec::new();
        for (index, (_, uniform)) in jobs.iter().enumerate() {
            let offset = index as u64 * self.uniform_stride;
            staging.resize(offset as usize, 0);
            staging.extend_from_slice(bytemuck::bytes_of(uniform));
        }
        self.gpu
            .queue
            .write_buffer(&self.uniform_buffer, 0, &staging);

        let pipeline = self
            .pipelines
            .iter()
            .find(|(mode, _)| *mode == BlendMode::Normal)
            .map(|(_, pipeline)| pipeline)
            .expect("the Normal blend pipeline always exists");

        let mut encoder = self
            .gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("fcap downstream"),
            });
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("fcap downstream pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.program_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load, // over the composed program
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            pass.set_pipeline(pipeline);
            for (index, (source, _)) in jobs.iter().enumerate() {
                let offset = (index as u64 * self.uniform_stride) as u32;
                let bind = &self.sources.get(source).expect("resolved above").bind_group;
                pass.set_bind_group(0, &self.uniform_bind, &[offset]);
                pass.set_bind_group(1, bind, &[]);
                pass.draw(0..4, 0..1);
            }
        }
        self.gpu.queue.submit(Some(encoder.finish()));
        Ok(())
    }

    fn ensure_reaction_rig(&mut self) {
        if self.reactions.is_some() {
            return;
        }
        let device = &self.gpu.device;
        let align = device.limits().min_uniform_buffer_offset_alignment as u64;
        let stride = (std::mem::size_of::<ReactionUniform>() as u64).div_ceil(align) * align;
        let uniform_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("fcap reaction uniform"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: NonZeroU64::new(
                            std::mem::size_of::<ReactionUniform>() as u64
                        ),
                    },
                    count: None,
                }],
            });
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("fcap reaction uniforms"),
            size: stride * REACTION_POOL as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let uniform_bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("fcap reaction uniforms"),
            layout: &uniform_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &uniform_buffer,
                    offset: 0,
                    size: NonZeroU64::new(std::mem::size_of::<ReactionUniform>() as u64),
                }),
            }],
        });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("fcap reaction shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/reaction.wgsl").into()),
        });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("fcap reaction layout"),
            bind_group_layouts: &[&uniform_layout, &self.texture_layout],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("fcap reaction pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: PROGRAM_FORMAT,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });
        self.reactions = Some(ReactionRig {
            pipeline,
            uniform_buffer,
            uniform_bind,
            uniform_stride: stride,
            sprites: HashMap::new(),
        });
    }

    fn ensure_stinger_rig(&mut self) {
        if self.stinger.is_some() {
            return;
        }
        let device = &self.gpu.device;
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("fcap stinger shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/stinger.wgsl").into()),
        });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("fcap stinger layout"),
            bind_group_layouts: &[&self.texture_layout],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("fcap stinger pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: PROGRAM_FORMAT,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });
        self.stinger = Some(StingerRig {
            pipeline,
            texture: None,
        });
    }

    /// Upload the newest stinger frame (recreating the texture on a
    /// size/format change), with the same geometry checks as sources.
    fn upload_stinger_frame(&mut self, frame: &Frame) -> Result<(), CompositorError> {
        if frame.width == 0 || frame.height == 0 || frame.stride < frame.width * 4 {
            return Err(CompositorError::BadFrame("bad stinger frame".into()));
        }
        let needed = frame.stride as usize * frame.height as usize;
        if frame.data.len() < needed {
            return Err(CompositorError::BadFrame(
                "stinger frame shorter than its geometry".into(),
            ));
        }
        let needs_new = match self.stinger.as_ref().and_then(|rig| rig.texture.as_ref()) {
            Some((_, _, width, height, format)) => {
                *width != frame.width || *height != frame.height || *format != frame.format
            }
            None => true,
        };
        if needs_new {
            let device = &self.gpu.device;
            let texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("fcap stinger frame"),
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
            let bind = Self::make_texture_bind(
                device,
                &self.texture_layout,
                &self.sampler,
                &self.repeat_sampler,
                &view,
            );
            if let Some(rig) = self.stinger.as_mut() {
                rig.texture = Some((texture, bind, frame.width, frame.height, frame.format));
            }
        }
        let rig = self.stinger.as_ref().expect("ensured by caller");
        let (texture, _, _, _, _) = rig.texture.as_ref().expect("just ensured");
        self.gpu.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &frame.data[..needed],
            wgpu::TexelCopyBufferLayout {
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

    /// Compose `scene` into the Studio-Mode preview scratch (the second
    /// transition scratch texture) and read it back — the preview pane's
    /// frame. Reuses the transition machinery so no third texture exists.
    pub fn render_preview_scene(
        &mut self,
        scene: &Scene,
        time_seconds: f32,
    ) -> Result<ProgramFrame, CompositorError> {
        self.ensure_transition_rig();
        let (texture, view) = {
            let rig = self.transition.as_ref().expect("ensured above");
            (rig.scratch[1].0.clone(), rig.scratch[1].1.clone())
        };
        let canvas = (self.canvas_width as f32, self.canvas_height as f32);
        self.render_to(scene, time_seconds, view, canvas, false)?;
        self.read_texture(&texture)
    }

    /// Compose a **single-source** workbench scene (CAP-M26) into the preview
    /// scratch and read it back. `matte` appends the alpha→grayscale pass so the
    /// keyer's matte shows instead of the keyed image. The caller builds a
    /// synthetic one-item scene (the source fit to the canvas, optionally with
    /// its filters); reuses the Studio-Mode preview scratch, so it must not run
    /// in the same breath as `render_preview_scene`.
    pub fn render_source_view(
        &mut self,
        scene: &Scene,
        time_seconds: f32,
        matte: bool,
    ) -> Result<ProgramFrame, CompositorError> {
        self.ensure_transition_rig();
        let (texture, view) = {
            let rig = self.transition.as_ref().expect("ensured above");
            (rig.scratch[1].0.clone(), rig.scratch[1].1.clone())
        };
        let canvas = (self.canvas_width as f32, self.canvas_height as f32);
        self.render_to(scene, time_seconds, view, canvas, matte)?;
        self.read_texture(&texture)
    }

    /// Compose `scene` into a small `width`×`height` thumbnail (CAP-M06
    /// multiview) and read it back. Item transforms are in full canvas px, so
    /// composing at the canvas dims into a tiny target just downscales — the
    /// readback shrinks with the target. Reuses one thumbnail texture; must not
    /// interleave with `render_preview_scene`/`render_source_view` (shared work,
    /// but a distinct target here).
    pub fn render_thumbnail(
        &mut self,
        scene: &Scene,
        time_seconds: f32,
        width: u32,
        height: u32,
    ) -> Result<ProgramFrame, CompositorError> {
        let (width, height) = (width.max(1), height.max(1));
        let needs_rebuild = match &self.thumbnail {
            Some((_, _, w, h)) => *w != width || *h != height,
            None => true,
        };
        if needs_rebuild {
            let (texture, view) = Self::make_program_texture(&self.gpu.device, width, height);
            self.thumbnail = Some((texture, view, width, height));
        }
        let (texture, view) = {
            let thumb = self.thumbnail.as_ref().expect("ensured above");
            (thumb.0.clone(), thumb.1.clone())
        };
        let canvas = (self.canvas_width as f32, self.canvas_height as f32);
        self.render_to(scene, time_seconds, view, canvas, false)?;
        self.read_texture(&texture)
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

    /// Freeze-frame filter (CAP-N25): hold a per-item snapshot of each `(item,
    /// source)`'s current texture. An item already held keeps its original
    /// snapshot; a live copy of the same source (a CAP-N26 clone, or another
    /// placement) is untouched, so only the frozen item holds. Call after this
    /// tick's sources are uploaded and before `render`.
    pub fn freeze_items(&mut self, items: &[(ItemId, SourceId)]) {
        // Snapshot only the newly-frozen items whose source is live.
        let pending: Vec<(ItemId, SourceId, u32, u32, PixelFormat)> = items
            .iter()
            .filter(|(item, _)| !self.frozen.contains_key(item))
            .filter_map(|&(item, source)| {
                let slot = self.sources.get(&source)?;
                Some((item, source, slot.width, slot.height, slot.format))
            })
            .collect();
        if pending.is_empty() {
            return;
        }
        let mut encoder = self
            .gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("fcap freeze snapshot"),
            });
        let mut new_slots: Vec<(ItemId, SourceSlot)> = Vec::new();
        for (item, source, width, height, format) in pending {
            let held = self.gpu.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("fcap frozen item"),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: texture_format(format),
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });
            encoder.copy_texture_to_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &self.sources[&source].texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::TexelCopyTextureInfo {
                    texture: &held,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
            );
            let view = held.create_view(&wgpu::TextureViewDescriptor::default());
            let bind_group = Self::make_texture_bind(
                &self.gpu.device,
                &self.texture_layout,
                &self.sampler,
                &self.repeat_sampler,
                &view,
            );
            new_slots.push((
                item,
                SourceSlot {
                    texture: held,
                    bind_group,
                    width,
                    height,
                    format,
                },
            ));
        }
        self.gpu.queue.submit(Some(encoder.finish()));
        for (item, slot) in new_slots {
            self.frozen.insert(item, slot);
        }
    }

    /// Drop held freeze snapshots for items no longer frozen (thaw).
    pub fn retain_frozen(&mut self, keep: &[ItemId]) {
        self.frozen.retain(|item, _| keep.contains(item));
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
                // COPY_SRC lets the freeze-frame filter (CAP-N25) snapshot this
                // source into a per-item held texture.
                usage: wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::COPY_DST
                    | wgpu::TextureUsages::COPY_SRC,
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
            wgpu::TexelCopyTextureInfo {
                texture: &slot.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &frame.data[..needed],
            wgpu::TexelCopyBufferLayout {
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
        let target = self.program_view.clone();
        let canvas = (self.canvas_width as f32, self.canvas_height as f32);
        self.render_to(scene, time_seconds, target, canvas, false)
    }

    /// The scenes a nested-scene source can reference + the source→scene
    /// mapping (refreshed by the studio each tick; empty = no nesting).
    /// Replace the punch-in zoom lenses (CAP-N71) for this tick — a tiny
    /// map; the studio hands over only items actively zoomed or animating.
    pub fn set_lenses(&mut self, lenses: HashMap<ItemId, (f32, (f32, f32))>) {
        self.lenses = lenses;
    }

    pub fn set_scene_pool(&mut self, scenes: Vec<Scene>, refs: HashMap<SourceId, SceneId>) {
        self.scene_pool = scenes;
        self.scene_refs = refs;
    }

    /// Compose every nested-scene source `scene` shows into its slot texture
    /// (children first — a nested scene inside a nested scene renders before
    /// its parent samples it). Depth-capped so a pathological file can never
    /// recurse away; the model already rejects true cycles. Nested frames
    /// render at the **program** canvas size regardless of the target canvas,
    /// so one slot serves the program and vertical canvases without thrash.
    fn ensure_nested(
        &mut self,
        scene: &Scene,
        time_seconds: f32,
        depth: u8,
    ) -> Result<(), CompositorError> {
        if depth >= 4 || self.scene_refs.is_empty() {
            return Ok(());
        }
        let jobs: Vec<(SourceId, SceneId)> = scene
            .items
            .iter()
            .filter(|item| item.visible && !scene.group_hides(item.id))
            .filter_map(|item| {
                self.scene_refs
                    .get(&item.source)
                    .map(|target| (item.source, *target))
            })
            .collect();
        for (source, target) in jobs {
            let Some(inner) = self
                .scene_pool
                .iter()
                .find(|candidate| candidate.id == target)
                .cloned()
            else {
                continue; // the target vanished — keep the last frame
            };
            self.ensure_nested(&inner, time_seconds, depth + 1)?;

            let (width, height) = (self.canvas_width, self.canvas_height);
            let needs_new = match self.sources.get(&source) {
                Some(slot) => slot.width != width || slot.height != height,
                None => true,
            };
            if needs_new {
                let (texture, view) = Self::make_program_texture(&self.gpu.device, width, height);
                let bind_group = Self::make_texture_bind(
                    &self.gpu.device,
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
                        width,
                        height,
                        format: PixelFormat::Rgba8,
                    },
                );
            }
            let view = self
                .sources
                .get(&source)
                .expect("ensured above")
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            self.compose_scene(
                &inner,
                time_seconds,
                view,
                (width as f32, height as f32),
                false,
            )?;
        }
        Ok(())
    }

    /// Compose `scene` into an arbitrary target at `canvas` dimensions — the
    /// program texture normally; a transition's scratch textures, the
    /// Studio-Mode preview pane, or the second (vertical) canvas otherwise.
    /// Nested-scene sources compose first (into their slot textures), then
    /// the scene itself.
    fn render_to(
        &mut self,
        scene: &Scene,
        time_seconds: f32,
        target: wgpu::TextureView,
        canvas: (f32, f32),
        matte: bool,
    ) -> Result<(), CompositorError> {
        self.ensure_nested(scene, time_seconds, 0)?;
        self.compose_scene(scene, time_seconds, target, canvas, matte)
    }

    /// One scene → one target, no nested pre-pass (callers ensure it).
    ///
    /// `matte` (CAP-M26, keying-workbench only) appends an alpha→grayscale pass
    /// to every item's chain so the composite shows the keyer's matte instead of
    /// the keyed image. Always `false` on the program/preview/vertical paths.
    fn compose_scene(
        &mut self,
        scene: &Scene,
        time_seconds: f32,
        target: wgpu::TextureView,
        canvas: (f32, f32),
        matte: bool,
    ) -> Result<(), CompositorError> {
        let started = Instant::now();

        // Plan: filter passes + composite uniforms, all staged up front.
        let mut item_staging: Vec<u8> = Vec::new();
        let mut filter_staging: Vec<u8> = Vec::new();
        let mut draws: Vec<Draw> = Vec::new();
        let mut chain_passes: Vec<ChainPass> = Vec::new();
        let mut live_chains: Vec<ItemId> = Vec::new();

        for item in scene
            .items
            .iter()
            .filter(|item| item.visible && !scene.group_hides(item.id))
        {
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
            // Workbench matte view: append an alpha→grayscale pass so the
            // composite shows the keyer's matte (CAP-M26).
            if matte {
                let matte_plan = crate::filters::plan_matte(chain_size);
                chain_size = matte_plan.out;
                plans.push(matte_plan);
            }

            // The backdrop wallpaper lays itself out every frame: cover-fit
            // (Full) or fit-contain (a half) into its region, reading the
            // item's transform only as clamped zoom/pan — it can never leave
            // blank canvas, and being `items[0]` by model invariant it can
            // never paint above the capture.
            let transform = match item.backdrop {
                Some(split) => {
                    let region = split.region();
                    transform::backdrop_layout(
                        chain_size.0,
                        chain_size.1,
                        (
                            region.x * canvas.0,
                            region.y * canvas.1,
                            region.w * canvas.0,
                            region.h * canvas.1,
                        ),
                        split != fcap_scene::BackdropSplit::Full,
                        item.transform.scale_x,
                        (item.transform.x, item.transform.y),
                    )
                }
                None => item.transform,
            };
            // The punch-in zoom lens (CAP-N71): a runtime zoom/pan the studio
            // animates — drawn only, never written to the model. The backdrop
            // has its own zoom semantics and is excluded.
            let transform = match self.lenses.get(&item.id) {
                Some((zoom, anchor)) if item.backdrop.is_none() => {
                    match transform::content_size(chain_size.0, chain_size.1, &transform.crop) {
                        Some(content) => transform::apply_lens(transform, content, *zoom, *anchor),
                        None => transform,
                    }
                }
                _ => transform,
            };
            // Pixel-perfect scaling (CAP-N70): Integer snaps the drawn scale
            // to whole multiples; Nearest/Integer/SharpBilinear pick their
            // sampling in the item shader (misc.y), with the drawn scale as
            // sharp-bilinear's sharpness (misc.z).
            let transform = match item.scaling {
                fcap_scene::ScaleMode::Integer => transform::integer_snap(transform),
                _ => transform,
            };
            let sampling = match item.scaling {
                fcap_scene::ScaleMode::Auto => 0.0,
                fcap_scene::ScaleMode::Nearest | fcap_scene::ScaleMode::Integer => 1.0,
                fcap_scene::ScaleMode::SharpBilinear => 2.0,
            };
            // The composite sees the chain's output (or the raw source).
            let Some(content) =
                transform::content_size(chain_size.0, chain_size.1, &transform.crop)
            else {
                continue; // fully cropped away
            };
            let (_, prep) = blend_config(item.blend);
            // CAP-N23: a 3D-tilted item uses the projective matrix; a plain 2D
            // transform stays on the exact affine path (pixel-identical).
            let mvp = if transform.has_3d() {
                transform::perspective_clip_matrix(&transform, content, canvas)
            } else {
                transform::clip_matrix(&transform, content, canvas)
            };
            let uniform = ItemUniform {
                mvp,
                uv_rect: transform::uv_rect(chain_size.0, chain_size.1, &transform.crop),
                size: [content.0, content.1, 0.0, 0.0],
                // misc.w = 1.0: scene items are fully opaque (the downstream
                // keyer is the only thing that sets a partial opacity).
                misc: [prep, sampling, transform.scale_x.abs(), 1.0],
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
                item: item.id,
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
                // A frozen item (CAP-N25) feeds its held snapshot into the chain,
                // not the live source — so a clone of the same source stays live.
                if let Some(held) = self.frozen.get(&chain_pass.item) {
                    &held.bind_group
                } else {
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
                }
            } else {
                &chain[chain_pass.pass_index - 1].bind_group
            };
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("fcap filter pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: target,
                    depth_slice: None,
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
                    view: &target,
                    depth_slice: None,
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
                    // A frozen, unfiltered item samples its held snapshot; a live
                    // copy of the same source is unaffected (CAP-N25).
                    None => match self.frozen.get(&draw.item) {
                        Some(held) => &held.bind_group,
                        None => {
                            &self
                                .sources
                                .get(&draw.source)
                                .expect("draws reference live slots")
                                .bind_group
                        }
                    },
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
        let texture = self.program.clone();
        self.read_texture(&texture)
    }

    /// Read a rendered target back to the CPU (tight RGBA rows) — the
    /// program, the Studio-Mode preview scratch, or the vertical canvas.
    /// Sizes come from the texture itself; each size keeps its own staging
    /// buffer so alternating canvases never thrash one.
    fn read_texture(&mut self, texture: &wgpu::Texture) -> Result<ProgramFrame, CompositorError> {
        let width = texture.width();
        let height = texture.height();
        let unpadded = width as u64 * 4;
        let padded = unpadded.div_ceil(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT as u64)
            * wgpu::COPY_BYTES_PER_ROW_ALIGNMENT as u64;
        let size = padded * height as u64;

        let buffer = self.readback.entry((width, height)).or_insert_with(|| {
            self.gpu.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("fcap readback"),
                size,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            })
        });

        let mut encoder = self
            .gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("fcap readback"),
            });
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer,
                layout: wgpu::TexelCopyBufferLayout {
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
        let _ = self.gpu.device.poll(wgpu::PollType::wait_indefinitely());
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
