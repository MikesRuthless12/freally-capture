//! The per-item GPU filter chain.
//!
//! Scene filters ([`fcap_scene::FilterKind`]) are *planned* into concrete
//! render passes (`plan_filter`): parameters become a [`FilterUniform`],
//! separable blur becomes two passes, crop changes the chain's output size,
//! and filters that need a file the app has not (yet) loaded are skipped —
//! the item renders unfiltered rather than black or stale.
//!
//! LUT lattices and mask images arrive through
//! [`Compositor::set_filter_resource`](crate::Compositor::set_filter_resource)
//! keyed by [`FilterId`] — this crate never touches the filesystem.

pub mod cube;

use std::collections::HashMap;
use std::num::NonZeroU64;

use fcap_scene::{Filter, FilterId, FilterKind, MaskMode, Rgba};

use crate::CompositorError;

/// Decoded pixels for a filter that samples an image the app loaded.
pub enum FilterResourceData {
    /// A mask image (straight RGBA).
    Image {
        width: u32,
        height: u32,
        rgba: Vec<u8>,
    },
    /// A parsed 3D LUT lattice.
    Lut3d(cube::Lut3d),
}

/// One uploaded filter resource: the group(2) bind group + LUT metadata.
pub(crate) struct FilterResource {
    bind_group: wgpu::BindGroup,
    lut_size: Option<u32>,
    // Held so the GPU objects outlive the bind group rebuilds.
    _texture: wgpu::Texture,
}

/// Which filter pipeline a pass runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PassKind {
    ChromaKey,
    ColorKey,
    LumaKey,
    ColorCorrection,
    Lut,
    Blur,
    Mask,
    Sharpen,
    Scroll,
    Crop,
    Flip,
    /// Blur family (CAP-N27): motion, spin, zoom, and mosaic.
    DirectionalBlur,
    RadialBlur,
    ZoomBlur,
    Pixelate,
    /// Preview-only alpha→grayscale, appended by the keying workbench (CAP-M26).
    Matte,
    /// A user WGSL effect (CAP-N22), keyed by the hash of its (validated)
    /// source — its pipeline is compiled on demand and cached, not in the
    /// static table.
    UserShader(u64),
}

/// Mirrors `shaders/filters.wgsl` (`FilterUniform`).
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct FilterUniform {
    pub m0: [f32; 4],
    pub m1: [f32; 4],
    pub m2: [f32; 4],
    pub p0: [f32; 4],
    pub p1: [f32; 4],
    pub texel: [f32; 4],
}

impl FilterUniform {
    fn zero() -> Self {
        bytemuck::Zeroable::zeroed()
    }

    fn with_texel(mut self, in_size: (u32, u32)) -> Self {
        self.texel = [
            1.0 / in_size.0.max(1) as f32,
            1.0 / in_size.1.max(1) as f32,
            in_size.0 as f32,
            in_size.1 as f32,
        ];
        self
    }
}

pub(crate) const FILTER_UNIFORM_SIZE: u64 = std::mem::size_of::<FilterUniform>() as u64;

/// One planned render pass of an item's chain.
pub(crate) struct PassPlan {
    pub kind: PassKind,
    pub uniform: FilterUniform,
    /// group(2) resource (LUT lattice / mask image), when the pass needs one.
    pub resource: Option<FilterId>,
    /// The pass's output size (differs from the input only for Crop).
    pub out: (u32, u32),
}

fn srgb_chroma(color: Rgba) -> [f32; 2] {
    let r = color.r as f32 / 255.0;
    let g = color.g as f32 / 255.0;
    let b = color.b as f32 / 255.0;
    let y = 0.2126 * r + 0.7152 * g + 0.0722 * b;
    [(b - y) * 0.5389, (r - y) * 0.6350]
}

/// The combined saturation → hue → contrast/brightness affine color matrix
/// (rows with the offset in `.w`), mirroring the shader's `m0..m2`.
pub(crate) fn color_matrix(
    brightness: f32,
    contrast: f32,
    saturation: f32,
    hue_degrees: f32,
) -> [[f32; 4]; 3] {
    let saturation = saturation.clamp(0.0, 4.0);
    let contrast = contrast.clamp(-1.0, 1.0);
    let brightness = brightness.clamp(-1.0, 1.0);

    // Saturation: mix between luma gray and the original.
    let lr = 0.2126 * (1.0 - saturation);
    let lg = 0.7152 * (1.0 - saturation);
    let lb = 0.0722 * (1.0 - saturation);
    let sat = [
        [lr + saturation, lg, lb],
        [lr, lg + saturation, lb],
        [lr, lg, lb + saturation],
    ];

    // Hue: rotation about the gray axis (Rodrigues, axis = (1,1,1)/√3).
    let theta = hue_degrees.to_radians();
    let (sin, cos) = theta.sin_cos();
    let one_third = 1.0 / 3.0;
    let sqrt_third = (1.0f32 / 3.0).sqrt();
    let hue = [
        [
            cos + (1.0 - cos) * one_third,
            one_third * (1.0 - cos) - sqrt_third * sin,
            one_third * (1.0 - cos) + sqrt_third * sin,
        ],
        [
            one_third * (1.0 - cos) + sqrt_third * sin,
            cos + one_third * (1.0 - cos),
            one_third * (1.0 - cos) - sqrt_third * sin,
        ],
        [
            one_third * (1.0 - cos) - sqrt_third * sin,
            one_third * (1.0 - cos) + sqrt_third * sin,
            cos + one_third * (1.0 - cos),
        ],
    ];

    // hue · sat
    let mut combined = [[0.0f32; 3]; 3];
    for row in 0..3 {
        for col in 0..3 {
            for k in 0..3 {
                combined[row][col] += hue[row][k] * sat[k][col];
            }
        }
    }

    // Contrast (pivot 0.5) + brightness fold into a scale + offset.
    let scale = if contrast >= 0.0 {
        1.0 + 3.0 * contrast
    } else {
        1.0 + contrast
    };
    let offset = 0.5 * (1.0 - scale) + brightness;

    let mut rows = [[0.0f32; 4]; 3];
    for row in 0..3 {
        for col in 0..3 {
            rows[row][col] = combined[row][col] * scale;
        }
        rows[row][3] = offset;
    }
    rows
}

/// Plan the preview-only matte pass (CAP-M26): alpha → opaque grayscale, no
/// size change. Appended after a keyer so the workbench can show its matte.
pub(crate) fn plan_matte(in_size: (u32, u32)) -> PassPlan {
    PassPlan {
        kind: PassKind::Matte,
        uniform: FilterUniform::zero().with_texel(in_size),
        resource: None,
        out: in_size,
    }
}

/// Plan one scene filter into render passes. `None` = the filter contributes
/// nothing right now (zero-strength, fully-cropped, or its file resource is
/// not loaded) and the chain skips it.
pub(crate) fn plan_filter(
    kind: &FilterKind,
    filter_id: FilterId,
    in_size: (u32, u32),
    time_seconds: f32,
    resources: &HashMap<FilterId, FilterResource>,
) -> Option<Vec<PassPlan>> {
    match kind {
        FilterKind::ChromaKey {
            key,
            similarity,
            smoothness,
            spill,
        } => {
            let chroma = srgb_chroma(*key);
            let mut uniform = FilterUniform::zero().with_texel(in_size);
            uniform.p0 = [
                chroma[0],
                chroma[1],
                similarity.clamp(0.0, 1.0),
                smoothness.clamp(0.0, 1.0).max(1e-3),
            ];
            uniform.p1 = [spill.clamp(0.0, 1.0).max(1e-3), 0.0, 0.0, 0.0];
            Some(vec![PassPlan {
                kind: PassKind::ChromaKey,
                uniform,
                resource: None,
                out: in_size,
            }])
        }
        FilterKind::ColorKey {
            key,
            similarity,
            smoothness,
        } => {
            let mut uniform = FilterUniform::zero().with_texel(in_size);
            uniform.p0 = [
                key.r as f32 / 255.0,
                key.g as f32 / 255.0,
                key.b as f32 / 255.0,
                similarity.clamp(0.0, 1.0),
            ];
            uniform.p1 = [smoothness.clamp(0.0, 1.0).max(1e-3), 0.0, 0.0, 0.0];
            Some(vec![PassPlan {
                kind: PassKind::ColorKey,
                uniform,
                resource: None,
                out: in_size,
            }])
        }
        FilterKind::LumaKey {
            luma_min,
            luma_max,
            smoothness,
        } => {
            let mut uniform = FilterUniform::zero().with_texel(in_size);
            uniform.p0 = [
                luma_min.clamp(0.0, 1.0),
                luma_max.clamp(0.0, 1.0),
                smoothness.clamp(0.0, 1.0).max(1e-3),
                0.0,
            ];
            Some(vec![PassPlan {
                kind: PassKind::LumaKey,
                uniform,
                resource: None,
                out: in_size,
            }])
        }
        // Render delay and freeze-frame are frame-timing effects, not GPU
        // passes — the studio's source-upload stage applies them (a bounded
        // buffer for delay; holding the last texture for freeze).
        FilterKind::RenderDelay { .. } | FilterKind::Freeze => None,
        // User shaders (CAP-N22) are planned in `compose_scene`, which has the
        // device needed to compile + validate them on demand.
        FilterKind::UserShader { .. } => None,
        FilterKind::ColorCorrection {
            gamma,
            brightness,
            contrast,
            saturation,
            hue_shift,
            opacity,
        } => {
            let rows = color_matrix(*brightness, *contrast, *saturation, *hue_shift);
            let mut uniform = FilterUniform::zero().with_texel(in_size);
            uniform.m0 = rows[0];
            uniform.m1 = rows[1];
            uniform.m2 = rows[2];
            uniform.p0 = [
                2.0f32.powf(-gamma.clamp(-3.0, 3.0)),
                opacity.clamp(0.0, 1.0),
                0.0,
                0.0,
            ];
            Some(vec![PassPlan {
                kind: PassKind::ColorCorrection,
                uniform,
                resource: None,
                out: in_size,
            }])
        }
        FilterKind::Lut { amount, .. } => {
            let lut_size = resources.get(&filter_id).and_then(|res| res.lut_size)?;
            let n = lut_size as f32;
            let mut uniform = FilterUniform::zero().with_texel(in_size);
            uniform.p0 = [(n - 1.0) / n, 0.5 / n, amount.clamp(0.0, 1.0), 0.0];
            Some(vec![PassPlan {
                kind: PassKind::Lut,
                uniform,
                resource: Some(filter_id),
                out: in_size,
            }])
        }
        FilterKind::Blur { radius } => {
            let radius = radius.clamp(0.0, 64.0);
            if radius < 0.5 {
                return None;
            }
            let sigma = (radius / 2.5).max(0.25);
            let pass = |dir: (f32, f32)| {
                let mut uniform = FilterUniform::zero().with_texel(in_size);
                uniform.p0 = [radius.round(), sigma, dir.0, dir.1];
                PassPlan {
                    kind: PassKind::Blur,
                    uniform,
                    resource: None,
                    out: in_size,
                }
            };
            Some(vec![pass((1.0, 0.0)), pass((0.0, 1.0))])
        }
        FilterKind::Mask { mode, invert, .. } => {
            if !resources.contains_key(&filter_id) {
                return None;
            }
            let mut uniform = FilterUniform::zero().with_texel(in_size);
            uniform.p0 = [
                if *mode == MaskMode::Luma { 1.0 } else { 0.0 },
                if *invert { 1.0 } else { 0.0 },
                0.0,
                0.0,
            ];
            Some(vec![PassPlan {
                kind: PassKind::Mask,
                uniform,
                resource: Some(filter_id),
                out: in_size,
            }])
        }
        FilterKind::BezierMask { invert, .. } => {
            // CAP-N28: the app rasterizes the path into an alpha mask keyed by
            // this filter's id — rendered through the same GPU mask pass. Until
            // that resource arrives (or with <3 points) it renders unmasked.
            if !resources.contains_key(&filter_id) {
                return None;
            }
            let mut uniform = FilterUniform::zero().with_texel(in_size);
            uniform.p0 = [0.0, if *invert { 1.0 } else { 0.0 }, 0.0, 0.0];
            Some(vec![PassPlan {
                kind: PassKind::Mask,
                uniform,
                resource: Some(filter_id),
                out: in_size,
            }])
        }
        FilterKind::Sharpen { amount } => {
            let amount = amount.clamp(0.0, 2.0);
            if amount < 1e-3 {
                return None;
            }
            let mut uniform = FilterUniform::zero().with_texel(in_size);
            uniform.p0 = [amount, 0.0, 0.0, 0.0];
            Some(vec![PassPlan {
                kind: PassKind::Sharpen,
                uniform,
                resource: None,
                out: in_size,
            }])
        }
        FilterKind::Scroll { speed_x, speed_y } => {
            if speed_x.abs() < 1e-3 && speed_y.abs() < 1e-3 {
                return None;
            }
            let offset_u = (speed_x * time_seconds / in_size.0.max(1) as f32).rem_euclid(1.0);
            let offset_v = (speed_y * time_seconds / in_size.1.max(1) as f32).rem_euclid(1.0);
            let mut uniform = FilterUniform::zero().with_texel(in_size);
            uniform.p0 = [offset_u, offset_v, 0.0, 0.0];
            Some(vec![PassPlan {
                kind: PassKind::Scroll,
                uniform,
                resource: None,
                out: in_size,
            }])
        }
        FilterKind::Flip {
            horizontal,
            vertical,
        } => {
            if !horizontal && !vertical {
                return None;
            }
            let mut uniform = FilterUniform::zero().with_texel(in_size);
            uniform.p0 = [f32::from(*horizontal), f32::from(*vertical), 0.0, 0.0];
            Some(vec![PassPlan {
                kind: PassKind::Flip,
                uniform,
                resource: None,
                out: in_size,
            }])
        }
        FilterKind::Crop {
            left,
            top,
            right,
            bottom,
        } => {
            let (out_w, out_h) = crop_output_size(in_size, *left, *top, *right, *bottom)?;
            let w = in_size.0 as f32;
            let h = in_size.1 as f32;
            let mut uniform = FilterUniform::zero().with_texel(in_size);
            uniform.p0 = [
                *left as f32 / w,
                *top as f32 / h,
                in_size.0.saturating_sub(*right) as f32 / w,
                in_size.1.saturating_sub(*bottom) as f32 / h,
            ];
            Some(vec![PassPlan {
                kind: PassKind::Crop,
                uniform,
                resource: None,
                out: (out_w, out_h),
            }])
        }
        FilterKind::DirectionalBlur { radius, angle } => {
            let radius = radius.clamp(0.0, 64.0);
            if radius < 0.5 {
                return None;
            }
            let sigma = (radius / 2.5).max(0.25);
            // Direction vector from the angle (0° = rightward).
            let (sin, cos) = angle.to_radians().sin_cos();
            let mut uniform = FilterUniform::zero().with_texel(in_size);
            uniform.p0 = [radius.round(), sigma, cos, sin];
            Some(vec![PassPlan {
                kind: PassKind::DirectionalBlur,
                uniform,
                resource: None,
                out: in_size,
            }])
        }
        FilterKind::RadialBlur {
            amount,
            center_x,
            center_y,
        } => {
            let amount = amount.clamp(0.0, 1.0);
            if amount < 1e-3 {
                return None;
            }
            let mut uniform = FilterUniform::zero().with_texel(in_size);
            uniform.p0 = [
                amount,
                0.0,
                center_x.clamp(0.0, 1.0),
                center_y.clamp(0.0, 1.0),
            ];
            Some(vec![PassPlan {
                kind: PassKind::RadialBlur,
                uniform,
                resource: None,
                out: in_size,
            }])
        }
        FilterKind::ZoomBlur {
            amount,
            center_x,
            center_y,
        } => {
            let amount = amount.clamp(0.0, 1.0);
            if amount < 1e-3 {
                return None;
            }
            let mut uniform = FilterUniform::zero().with_texel(in_size);
            uniform.p0 = [
                amount,
                0.0,
                center_x.clamp(0.0, 1.0),
                center_y.clamp(0.0, 1.0),
            ];
            Some(vec![PassPlan {
                kind: PassKind::ZoomBlur,
                uniform,
                resource: None,
                out: in_size,
            }])
        }
        FilterKind::Pixelate { size } => {
            let size = size.clamp(1.0, 128.0);
            if size < 1.5 {
                return None;
            }
            let mut uniform = FilterUniform::zero().with_texel(in_size);
            uniform.p0 = [size.round(), 0.0, 0.0, 0.0];
            Some(vec![PassPlan {
                kind: PassKind::Pixelate,
                uniform,
                resource: None,
                out: in_size,
            }])
        }
    }
}

/// One enabled Crop's output size, with the engine's exact skip semantics:
/// `None` = the crop contributes nothing (all-zero, or it would zero an axis)
/// and the chain size is unchanged. The single source of truth for the
/// size-changing filter — [`plan_filter`]'s Crop arm and
/// [`effective_source_size`] both fold through here. Chained saturating subs:
/// hostile values must clamp, not overflow the `left + right` sum.
fn crop_output_size(
    in_size: (u32, u32),
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
) -> Option<(u32, u32)> {
    if left == 0 && top == 0 && right == 0 && bottom == 0 {
        return None;
    }
    let out_w = in_size.0.saturating_sub(left).saturating_sub(right);
    let out_h = in_size.1.saturating_sub(top).saturating_sub(bottom);
    if out_w == 0 || out_h == 0 {
        return None; // cropping everything away = contribute nothing
    }
    Some((out_w, out_h))
}

/// The size an item's enabled filter chain actually composes: the reported
/// source resolution folded through its Crop filters (the only size-changing
/// kind) exactly as `compose_scene` plans them. Exposed so the app reuses the
/// engine's own fold instead of re-implementing it (the UI's
/// `effectiveSourceSize` in `ui/src/lib/transform.ts` mirrors the same
/// semantics in TS).
pub fn effective_source_size(source: (u32, u32), filters: &[Filter]) -> (u32, u32) {
    let mut size = source;
    for filter in filters.iter().filter(|filter| filter.enabled) {
        if let FilterKind::Crop {
            left,
            top,
            right,
            bottom,
        } = filter.kind
        {
            if let Some(out) = crop_output_size(size, left, top, right, bottom) {
                size = out;
            }
        }
    }
    size
}

/// The GPU half: pipelines, the dynamic uniform buffer, and uploaded
/// LUT/mask resources.
pub(crate) struct FilterEngine {
    pub uniform_buffer: wgpu::Buffer,
    pub uniform_bind: wgpu::BindGroup,
    uniform_layout: wgpu::BindGroupLayout,
    uniform_capacity: u64,
    pub uniform_stride: u64,
    pipelines: Vec<(PassKind, wgpu::RenderPipeline)>,
    lut_layout: wgpu::BindGroupLayout,
    mask_layout: wgpu::BindGroupLayout,
    resources: HashMap<FilterId, FilterResource>,
    /// User-WGSL effects (CAP-N22): the shared layout + target format so a
    /// pipeline can be compiled on demand, and a cache keyed by source hash —
    /// `None` marks a shader that failed validation (never retried per frame).
    user_pipeline_layout: wgpu::PipelineLayout,
    user_target_format: wgpu::TextureFormat,
    user_pipelines: HashMap<u64, Option<wgpu::RenderPipeline>>,
}

/// The compile-time WGSL harness wrapped around a user effect (CAP-N22). It
/// declares the exact bindings the filter pipeline provides — the user can only
/// ADD functions/consts and must define `effect(...)`, so a shader can never
/// reach a binding the layout does not carry (that fails validation and the
/// effect is skipped, never rendered as a crash).
const USER_SHADER_HEADER: &str = r#"
struct FilterUniform {
    m0: vec4<f32>, m1: vec4<f32>, m2: vec4<f32>,
    p0: vec4<f32>, p1: vec4<f32>, texel: vec4<f32>,
};
@group(0) @binding(0) var<uniform> f: FilterUniform;
@group(1) @binding(0) var t_in: texture_2d<f32>;
@group(1) @binding(1) var s_clamp: sampler;
@group(1) @binding(2) var s_repeat: sampler;
struct VsOut { @builtin(position) pos: vec4<f32>, @location(0) uv: vec2<f32>, };
@vertex
fn vs_fullscreen(@builtin(vertex_index) vi: u32) -> VsOut {
    let uv = vec2<f32>(f32((vi << 1u) & 2u), f32(vi & 2u));
    var out: VsOut;
    out.pos = vec4<f32>(uv * 2.0 - 1.0, 0.0, 1.0);
    out.uv = vec2<f32>(uv.x, 1.0 - uv.y);
    return out;
}
fn luma(rgb: vec3<f32>) -> f32 { return dot(rgb, vec3<f32>(0.2126, 0.7152, 0.0722)); }
fn sample_at(uv: vec2<f32>) -> vec4<f32> { return textureSample(t_in, s_clamp, uv); }
// ---- define: fn effect(uv: vec2<f32>, color: vec4<f32>, p: vec4<f32>, texel: vec4<f32>, time: f32) -> vec4<f32>
//      uv = pixel 0..1; color = source at uv; p = the four params; texel.zw = size px; time = seconds ----
"#;

const USER_SHADER_FOOTER: &str = r#"
@fragment
fn fs_user(in: VsOut) -> @location(0) vec4<f32> {
    let color = textureSample(t_in, s_clamp, in.uv);
    return effect(in.uv, color, f.p0, f.texel, f.p1.x);
}
"#;

/// Longest user source accepted (CAP-N22) — a defensive cap; real effects are
/// well under this. Oversized submissions are rejected before compilation.
const MAX_USER_SHADER_LEN: usize = 8192;
/// Cap on cached user pipelines; live-editing churns hashes, so clear (and let
/// the next use recompile) rather than grow without bound.
const MAX_USER_PIPELINES: usize = 32;

fn user_shader_hash(source: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    source.hash(&mut hasher);
    hasher.finish()
}

/// Plan a user-shader pass (CAP-N22): its four params ride `p0`, playback time
/// rides `p1.x`, and it never resizes the chain.
pub(crate) fn user_shader_plan(
    hash: u64,
    params: &[f32],
    in_size: (u32, u32),
    time: f32,
) -> PassPlan {
    let mut uniform = FilterUniform::zero().with_texel(in_size);
    let mut p = [0.0f32; 4];
    for (slot, value) in p.iter_mut().zip(params.iter()) {
        *slot = *value;
    }
    uniform.p0 = p;
    uniform.p1 = [time, 0.0, 0.0, 0.0];
    PassPlan {
        kind: PassKind::UserShader(hash),
        uniform,
        resource: None,
        out: in_size,
    }
}

/// Compile a user effect into a pipeline, returning `None` if it fails naga
/// validation (CAP-N22). The whole compile runs inside a validation error scope
/// so an invalid shader is caught and skipped — it can never surface as a
/// panic or a lost device.
fn compile_user_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    format: wgpu::TextureFormat,
    source: &str,
) -> Option<wgpu::RenderPipeline> {
    let full = format!("{USER_SHADER_HEADER}{source}{USER_SHADER_FOOTER}");
    device.push_error_scope(wgpu::ErrorFilter::Validation);
    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("fcap user shader"),
        source: wgpu::ShaderSource::Wgsl(full.into()),
    });
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("fcap user shader pipeline"),
        cache: None,
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: &module,
            entry_point: Some("vs_fullscreen"),
            compilation_options: Default::default(),
            buffers: &[],
        },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            cull_mode: None,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        fragment: Some(wgpu::FragmentState {
            module: &module,
            entry_point: Some("fs_user"),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview: None,
    });
    // Any validation error (bad WGSL, a binding the layout lacks, a missing
    // `effect`) lands here — the shader is rejected, not run.
    match pollster::block_on(device.pop_error_scope()) {
        Some(_) => None,
        None => Some(pipeline),
    }
}

const INITIAL_PASS_CAPACITY: u64 = 32;

impl FilterEngine {
    pub fn new(
        device: &wgpu::Device,
        input_layout: &wgpu::BindGroupLayout,
        target_format: wgpu::TextureFormat,
    ) -> Self {
        let uniform_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("fcap filter uniforms"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size: NonZeroU64::new(FILTER_UNIFORM_SIZE),
                },
                count: None,
            }],
        });

        let lut_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("fcap filter lut"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D3,
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

        let mask_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("fcap filter mask"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("fcap filter shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/filters.wgsl").into()),
        });

        let basic_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("fcap filter layout"),
            bind_group_layouts: &[&uniform_layout, input_layout],
            push_constant_ranges: &[],
        });
        let with_lut = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("fcap filter layout (lut)"),
            bind_group_layouts: &[&uniform_layout, input_layout, &lut_layout],
            push_constant_ranges: &[],
        });
        let with_mask = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("fcap filter layout (mask)"),
            bind_group_layouts: &[&uniform_layout, input_layout, &mask_layout],
            push_constant_ranges: &[],
        });
        // User WGSL effects (CAP-N22) share the basic layout: the uniform (with
        // their params) + the source texture/samplers — nothing else.
        let user_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("fcap user shader layout"),
            bind_group_layouts: &[&uniform_layout, input_layout],
            push_constant_ranges: &[],
        });

        let entries: [(PassKind, &str, &wgpu::PipelineLayout); 16] = [
            (PassKind::ChromaKey, "fs_chroma_key", &basic_layout),
            (PassKind::ColorKey, "fs_color_key", &basic_layout),
            (PassKind::LumaKey, "fs_luma_key", &basic_layout),
            (
                PassKind::ColorCorrection,
                "fs_color_correction",
                &basic_layout,
            ),
            (PassKind::Lut, "fs_lut", &with_lut),
            (PassKind::Blur, "fs_blur", &basic_layout),
            (PassKind::Mask, "fs_mask", &with_mask),
            (PassKind::Sharpen, "fs_sharpen", &basic_layout),
            (PassKind::Scroll, "fs_scroll", &basic_layout),
            (PassKind::Crop, "fs_crop", &basic_layout),
            (PassKind::Flip, "fs_flip", &basic_layout),
            (
                PassKind::DirectionalBlur,
                "fs_directional_blur",
                &basic_layout,
            ),
            (PassKind::RadialBlur, "fs_radial_blur", &basic_layout),
            (PassKind::ZoomBlur, "fs_zoom_blur", &basic_layout),
            (PassKind::Pixelate, "fs_pixelate", &basic_layout),
            (PassKind::Matte, "fs_matte", &basic_layout),
        ];
        let pipelines = entries
            .iter()
            .map(|(kind, entry, layout)| {
                let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("fcap filter pipeline"),
                    cache: None,
                    layout: Some(layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: Some("vs_fullscreen"),
                        compilation_options: Default::default(),
                        buffers: &[],
                    },
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        cull_mode: None,
                        ..Default::default()
                    },
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState::default(),
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: Some(entry),
                        compilation_options: Default::default(),
                        targets: &[Some(wgpu::ColorTargetState {
                            format: target_format,
                            blend: None,
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    multiview: None,
                });
                (*kind, pipeline)
            })
            .collect();

        let (uniform_buffer, uniform_bind, uniform_stride) =
            Self::make_uniforms(device, &uniform_layout, INITIAL_PASS_CAPACITY);

        Self {
            uniform_buffer,
            uniform_bind,
            uniform_layout,
            uniform_capacity: INITIAL_PASS_CAPACITY,
            uniform_stride,
            pipelines,
            lut_layout,
            mask_layout,
            resources: HashMap::new(),
            user_pipeline_layout,
            user_target_format: target_format,
            user_pipelines: HashMap::new(),
        }
    }

    fn make_uniforms(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        capacity: u64,
    ) -> (wgpu::Buffer, wgpu::BindGroup, u64) {
        let align = device.limits().min_uniform_buffer_offset_alignment as u64;
        let stride = FILTER_UNIFORM_SIZE.div_ceil(align) * align;
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("fcap filter uniforms"),
            size: stride * capacity,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("fcap filter uniforms"),
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &buffer,
                    offset: 0,
                    size: NonZeroU64::new(FILTER_UNIFORM_SIZE),
                }),
            }],
        });
        (buffer, bind, stride)
    }

    pub fn ensure_capacity(&mut self, device: &wgpu::Device, passes: u64) {
        if passes <= self.uniform_capacity {
            return;
        }
        let capacity = passes.next_power_of_two();
        let (buffer, bind, stride) = Self::make_uniforms(device, &self.uniform_layout, capacity);
        self.uniform_buffer = buffer;
        self.uniform_bind = bind;
        self.uniform_stride = stride;
        self.uniform_capacity = capacity;
    }

    pub fn pipeline(&self, kind: PassKind) -> &wgpu::RenderPipeline {
        if let PassKind::UserShader(hash) = kind {
            return self
                .user_pipelines
                .get(&hash)
                .and_then(|entry| entry.as_ref())
                .expect("a planned user-shader pass has a compiled pipeline");
        }
        self.pipelines
            .iter()
            .find(|(candidate, _)| *candidate == kind)
            .map(|(_, pipeline)| pipeline)
            .expect("every pass kind has a pipeline")
    }

    /// Compile + cache a user WGSL effect (CAP-N22), returning its pass hash if
    /// it validated. `None` = empty, oversized, or a shader that failed naga
    /// validation (a bad shader is cached as a failure so it is never retried
    /// every frame, and the item simply renders unfiltered).
    pub fn ensure_user_pipeline(&mut self, device: &wgpu::Device, source: &str) -> Option<u64> {
        if source.trim().is_empty() || source.len() > MAX_USER_SHADER_LEN {
            return None;
        }
        let hash = user_shader_hash(source);
        match self.user_pipelines.get(&hash) {
            Some(Some(_)) => return Some(hash),
            Some(None) => return None,
            None => {}
        }
        if self.user_pipelines.len() >= MAX_USER_PIPELINES {
            self.user_pipelines.clear(); // bound memory under heavy live-editing
        }
        let pipeline = compile_user_pipeline(
            device,
            &self.user_pipeline_layout,
            self.user_target_format,
            source,
        );
        let ok = pipeline.is_some();
        self.user_pipelines.insert(hash, pipeline);
        ok.then_some(hash)
    }

    pub fn resources(&self) -> &HashMap<FilterId, FilterResource> {
        &self.resources
    }

    pub fn resource_bind(&self, id: FilterId) -> Option<&wgpu::BindGroup> {
        self.resources.get(&id).map(|res| &res.bind_group)
    }

    pub fn remove_resource(&mut self, id: FilterId) {
        self.resources.remove(&id);
    }

    pub fn retain_resources(&mut self, keep: &[FilterId]) {
        self.resources.retain(|id, _| keep.contains(id));
    }

    /// Upload a LUT lattice / mask image for `filter_id`.
    pub fn set_resource(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        sampler: &wgpu::Sampler,
        filter_id: FilterId,
        data: &FilterResourceData,
    ) -> Result<(), CompositorError> {
        let (texture, lut_size) = match data {
            FilterResourceData::Image {
                width,
                height,
                rgba,
            } => {
                if *width == 0 || *height == 0 {
                    return Err(CompositorError::BadFrame("empty mask image".into()));
                }
                if rgba.len() < (*width as usize) * (*height as usize) * 4 {
                    return Err(CompositorError::BadFrame(
                        "mask image data shorter than its geometry".into(),
                    ));
                }
                let texture = device.create_texture(&wgpu::TextureDescriptor {
                    label: Some("fcap mask"),
                    size: wgpu::Extent3d {
                        width: *width,
                        height: *height,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                    view_formats: &[],
                });
                queue.write_texture(
                    wgpu::TexelCopyTextureInfo {
                        texture: &texture,
                        mip_level: 0,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                    },
                    rgba,
                    wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(width * 4),
                        rows_per_image: Some(*height),
                    },
                    wgpu::Extent3d {
                        width: *width,
                        height: *height,
                        depth_or_array_layers: 1,
                    },
                );
                (texture, None)
            }
            FilterResourceData::Lut3d(lut) => {
                let texture = device.create_texture(&wgpu::TextureDescriptor {
                    label: Some("fcap lut"),
                    size: wgpu::Extent3d {
                        width: lut.size,
                        height: lut.size,
                        depth_or_array_layers: lut.size,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D3,
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                    view_formats: &[],
                });
                queue.write_texture(
                    wgpu::TexelCopyTextureInfo {
                        texture: &texture,
                        mip_level: 0,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                    },
                    &lut.rgba,
                    wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(lut.size * 4),
                        rows_per_image: Some(lut.size),
                    },
                    wgpu::Extent3d {
                        width: lut.size,
                        height: lut.size,
                        depth_or_array_layers: lut.size,
                    },
                );
                (texture, Some(lut.size))
            }
        };

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let bind_group = if lut_size.is_some() {
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("fcap lut"),
                layout: &self.lut_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(sampler),
                    },
                ],
            })
        } else {
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("fcap mask"),
                layout: &self.mask_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(&view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: wgpu::BindingResource::Sampler(sampler),
                    },
                ],
            })
        };

        self.resources.insert(
            filter_id,
            FilterResource {
                bind_group,
                lut_size,
                _texture: texture,
            },
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn no_resources() -> HashMap<FilterId, FilterResource> {
        HashMap::new()
    }

    #[test]
    fn blur_plans_two_separable_passes() {
        let plan = plan_filter(
            &FilterKind::Blur { radius: 8.0 },
            FilterId::new(),
            (64, 64),
            0.0,
            &no_resources(),
        )
        .expect("planned");
        assert_eq!(plan.len(), 2);
        assert_eq!(plan[0].uniform.p0[2..4], [1.0, 0.0]);
        assert_eq!(plan[1].uniform.p0[2..4], [0.0, 1.0]);
    }

    #[test]
    fn zero_strength_filters_plan_nothing() {
        let id = FilterId::new();
        let empty = no_resources();
        assert!(
            plan_filter(&FilterKind::Blur { radius: 0.0 }, id, (64, 64), 0.0, &empty).is_none()
        );
        assert!(plan_filter(
            &FilterKind::Sharpen { amount: 0.0 },
            id,
            (64, 64),
            0.0,
            &empty
        )
        .is_none());
        assert!(plan_filter(
            &FilterKind::Scroll {
                speed_x: 0.0,
                speed_y: 0.0
            },
            id,
            (64, 64),
            5.0,
            &empty
        )
        .is_none());
        assert!(plan_filter(
            &FilterKind::Crop {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0
            },
            id,
            (64, 64),
            0.0,
            &empty
        )
        .is_none());
    }

    #[test]
    fn blur_family_plans_and_clamps() {
        let id = FilterId::new();
        let empty = no_resources();

        // Directional: single pass; direction from the angle (90° ≈ (0, 1)).
        let plan = plan_filter(
            &FilterKind::DirectionalBlur {
                radius: 10.0,
                angle: 90.0,
            },
            id,
            (64, 64),
            0.0,
            &empty,
        )
        .expect("planned");
        assert_eq!(plan.len(), 1);
        assert_eq!(plan[0].kind, PassKind::DirectionalBlur);
        assert!(plan[0].uniform.p0[2].abs() < 1e-3, "cos 90° ≈ 0");
        assert!((plan[0].uniform.p0[3] - 1.0).abs() < 1e-3, "sin 90° ≈ 1");

        // Pixelate: block size clamped + rounded into p0.x.
        let plan = plan_filter(
            &FilterKind::Pixelate { size: 12.4 },
            id,
            (64, 64),
            0.0,
            &empty,
        )
        .unwrap();
        assert_eq!(plan[0].kind, PassKind::Pixelate);
        assert_eq!(plan[0].uniform.p0[0], 12.0);

        // Zoom: an out-of-range center clamps into 0..1.
        let plan = plan_filter(
            &FilterKind::ZoomBlur {
                amount: 0.5,
                center_x: 2.0,
                center_y: -1.0,
            },
            id,
            (64, 64),
            0.0,
            &empty,
        )
        .unwrap();
        assert_eq!(plan[0].kind, PassKind::ZoomBlur);
        assert_eq!(plan[0].uniform.p0[2], 1.0, "center_x clamped to 1");
        assert_eq!(plan[0].uniform.p0[3], 0.0, "center_y clamped to 0");

        // Inert strength plans nothing (no wasted pass).
        assert!(plan_filter(
            &FilterKind::DirectionalBlur {
                radius: 0.0,
                angle: 0.0
            },
            id,
            (64, 64),
            0.0,
            &empty
        )
        .is_none());
        for inert in [
            FilterKind::RadialBlur {
                amount: 0.0,
                center_x: 0.5,
                center_y: 0.5,
            },
            FilterKind::ZoomBlur {
                amount: 0.0,
                center_x: 0.5,
                center_y: 0.5,
            },
            FilterKind::Pixelate { size: 1.0 },
        ] {
            assert!(plan_filter(&inert, id, (64, 64), 0.0, &empty).is_none());
        }
    }

    #[test]
    fn freeze_is_not_a_gpu_pass() {
        // Freeze is a source-frame effect (the studio stops uploading), so it
        // must never plan a GPU pass.
        assert!(plan_filter(
            &FilterKind::Freeze,
            FilterId::new(),
            (64, 64),
            0.0,
            &no_resources()
        )
        .is_none());
    }

    #[test]
    fn hostile_crop_filter_values_clamp_instead_of_overflowing() {
        // A hand-edited file with u32::MAX crops must plan to "skip", never
        // panic on the `left + right` sum.
        assert!(plan_filter(
            &FilterKind::Crop {
                left: u32::MAX,
                top: 0,
                right: 1,
                bottom: 0,
            },
            FilterId::new(),
            (64, 32),
            0.0,
            &no_resources(),
        )
        .is_none());
    }

    #[test]
    fn crop_changes_the_chain_size() {
        let plan = plan_filter(
            &FilterKind::Crop {
                left: 4,
                top: 2,
                right: 8,
                bottom: 6,
            },
            FilterId::new(),
            (64, 32),
            0.0,
            &no_resources(),
        )
        .expect("planned");
        assert_eq!(plan[0].out, (52, 24));
    }

    #[test]
    fn effective_source_size_folds_like_the_engine() {
        let crop = |left, top, right, bottom| Filter {
            id: FilterId::new(),
            enabled: true,
            kind: FilterKind::Crop {
                left,
                top,
                right,
                bottom,
            },
        };
        // Chained crops fold in order, exactly as compose_scene plans them.
        assert_eq!(
            effective_source_size((64, 32), &[crop(4, 2, 8, 6), crop(2, 0, 2, 0)]),
            (48, 24)
        );
        // A disabled crop contributes nothing.
        let mut disabled = crop(4, 2, 8, 6);
        disabled.enabled = false;
        assert_eq!(effective_source_size((64, 32), &[disabled]), (64, 32));
        // The engine's skip semantics: an axis-zeroing crop is skipped, the
        // size unchanged — never zero.
        assert_eq!(
            effective_source_size((64, 32), &[crop(64, 0, 1, 0)]),
            (64, 32)
        );
        // Non-crop filters never change the chain size.
        let blur = Filter {
            id: FilterId::new(),
            enabled: true,
            kind: FilterKind::Blur { radius: 8.0 },
        };
        assert_eq!(effective_source_size((64, 32), &[blur]), (64, 32));
    }

    #[test]
    fn unloaded_lut_and_mask_are_skipped_not_black() {
        let id = FilterId::new();
        let empty = no_resources();
        assert!(plan_filter(
            &FilterKind::Lut {
                path: "missing.cube".into(),
                amount: 1.0
            },
            id,
            (64, 64),
            0.0,
            &empty
        )
        .is_none());
        assert!(plan_filter(
            &FilterKind::Mask {
                path: "missing.png".into(),
                mode: MaskMode::Alpha,
                invert: false
            },
            id,
            (64, 64),
            0.0,
            &empty
        )
        .is_none());
        // CAP-N28: a bezier mask whose rasterized resource has not arrived is
        // skipped too — the item renders unmasked, never black.
        assert!(plan_filter(
            &FilterKind::BezierMask {
                points: vec![[0.2, 0.2], [0.8, 0.2], [0.5, 0.8]],
                feather: 0.03,
                invert: false,
            },
            id,
            (64, 64),
            0.0,
            &empty,
        )
        .is_none());
    }

    #[test]
    fn neutral_color_matrix_is_identity() {
        let rows = color_matrix(0.0, 0.0, 1.0, 0.0);
        for (row_index, row) in rows.iter().enumerate() {
            for col in 0..3 {
                let expected = if col == row_index { 1.0 } else { 0.0 };
                assert!(
                    (row[col] - expected).abs() < 1e-4,
                    "row {row_index} col {col}: {row:?}"
                );
            }
            assert!(row[3].abs() < 1e-4, "no offset: {row:?}");
        }
    }

    #[test]
    fn saturation_zero_weights_luma() {
        let rows = color_matrix(0.0, 0.0, 0.0, 0.0);
        for row in rows {
            assert!((row[0] - 0.2126).abs() < 1e-4);
            assert!((row[1] - 0.7152).abs() < 1e-4);
            assert!((row[2] - 0.0722).abs() < 1e-4);
        }
    }

    #[test]
    fn hue_rotation_by_120_degrees_permutes_channels() {
        // Rotating hue by 120° about the gray axis maps R→G→B→R.
        let rows = color_matrix(0.0, 0.0, 1.0, 120.0);
        let apply = |rgb: [f32; 3]| {
            [
                rows[0][0] * rgb[0] + rows[0][1] * rgb[1] + rows[0][2] * rgb[2] + rows[0][3],
                rows[1][0] * rgb[0] + rows[1][1] * rgb[1] + rows[1][2] * rgb[2] + rows[1][3],
                rows[2][0] * rgb[0] + rows[2][1] * rgb[1] + rows[2][2] * rgb[2] + rows[2][3],
            ]
        };
        let out = apply([1.0, 0.0, 0.0]);
        assert!(
            out[0].abs() < 1e-3 && (out[1] - 1.0).abs() < 1e-3 && out[2].abs() < 1e-3,
            "red → green, got {out:?}"
        );
    }

    #[test]
    fn scroll_offset_wraps_with_time() {
        let plan = plan_filter(
            &FilterKind::Scroll {
                speed_x: 32.0,
                speed_y: 0.0,
            },
            FilterId::new(),
            (64, 64),
            3.0, // 96 px = 1.5 widths → wraps to 0.5
            &no_resources(),
        )
        .expect("planned");
        assert!((plan[0].uniform.p0[0] - 0.5).abs() < 1e-4);
    }
}
