// The per-item video filter passes. Each pass draws one fullscreen triangle
// into an intermediate chain texture, sampling the previous stage.
//
// Uniform packing (see filters/mod.rs, FilterUniform):
//   m0..m2 — a 3×4 affine color matrix (color correction), rows
//   p0/p1  — per-filter parameters, documented at each entry point
//   texel  — xy = 1 / input size, zw = input size in px

struct FilterUniform {
    m0: vec4<f32>,
    m1: vec4<f32>,
    m2: vec4<f32>,
    p0: vec4<f32>,
    p1: vec4<f32>,
    texel: vec4<f32>,
};

@group(0) @binding(0) var<uniform> f: FilterUniform;
@group(1) @binding(0) var t_in: texture_2d<f32>;
@group(1) @binding(1) var s_clamp: sampler;
@group(1) @binding(2) var s_repeat: sampler;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

// One fullscreen triangle (vertices 0..3).
@vertex
fn vs_fullscreen(@builtin(vertex_index) vi: u32) -> VsOut {
    let uv = vec2<f32>(f32((vi << 1u) & 2u), f32(vi & 2u));
    var out: VsOut;
    out.pos = vec4<f32>(uv * 2.0 - 1.0, 0.0, 1.0);
    // Flip v: NDC y points up but texture v points down. Without this every
    // pass writes a vertically mirrored copy and odd-length chains render
    // upside-down (caught by `filter_passes_preserve_vertical_orientation`).
    out.uv = vec2<f32>(uv.x, 1.0 - uv.y);
    return out;
}

fn luma(rgb: vec3<f32>) -> f32 {
    return dot(rgb, vec3<f32>(0.2126, 0.7152, 0.0722));
}

// -- Chroma key --------------------------------------------------------------
// p0 = (key_cb, key_cr, similarity, smoothness); p1 = (spill, 0, 0, 0)
fn chroma_of(rgb: vec3<f32>) -> vec2<f32> {
    let y = luma(rgb);
    return vec2<f32>((rgb.b - y) * 0.5389, (rgb.r - y) * 0.6350);
}

@fragment
fn fs_chroma_key(in: VsOut) -> @location(0) vec4<f32> {
    var color = textureSample(t_in, s_clamp, in.uv);
    let dist = distance(chroma_of(color.rgb), f.p0.xy);
    let base = dist - f.p0.z;
    let mask = pow(clamp(base / max(f.p0.w, 1e-4), 0.0, 1.0), 1.5);
    var alpha = color.a * mask;
    // Spill suppression: pull leftover key tint toward gray near the key.
    let spill_mask = pow(clamp(base / max(f.p1.x, 1e-4), 0.0, 1.0), 1.5);
    let desat = vec3<f32>(luma(color.rgb));
    let rgb = mix(desat, color.rgb, clamp(spill_mask, 0.0, 1.0));
    return vec4<f32>(rgb, alpha);
}

// -- Color key ------------------------------------------------------------------
// Key out an arbitrary color by RGB distance (non-green backdrops).
// p0 = (key.r, key.g, key.b, similarity) · p1 = (smoothness, 0, 0, 0)
@fragment
fn fs_color_key(in: VsOut) -> @location(0) vec4<f32> {
    let color = textureSample(t_in, s_clamp, in.uv);
    let dist = distance(color.rgb, f.p0.xyz) / sqrt(3.0);
    let base = dist - f.p0.w;
    let mask = pow(clamp(base / max(f.p1.x, 1e-4), 0.0, 1.0), 1.5);
    return vec4<f32>(color.rgb, color.a * mask);
}

// -- Luma key ---------------------------------------------------------------------
// Key on brightness: outside [min, max] goes transparent, soft edges.
// p0 = (luma_min, luma_max, smoothness, 0)
@fragment
fn fs_luma_key(in: VsOut) -> @location(0) vec4<f32> {
    let color = textureSample(t_in, s_clamp, in.uv);
    let l = luma(color.rgb);
    let soft = max(f.p0.z, 1e-4);
    let above = smoothstep(f.p0.x - soft, f.p0.x, l);
    let below = 1.0 - smoothstep(f.p0.y, f.p0.y + soft, l);
    return vec4<f32>(color.rgb, color.a * above * below);
}

// -- Matte (CAP-M26 keying workbench) ----------------------------------------
// Preview-only: render the incoming alpha as opaque grayscale so a keyer's
// matte is visible (white = kept, black = keyed out). Appended after the key
// pass by the workbench render; never part of a user filter chain.
@fragment
fn fs_matte(in: VsOut) -> @location(0) vec4<f32> {
    let a = textureSample(t_in, s_clamp, in.uv).a;
    return vec4<f32>(a, a, a, 1.0);
}

// -- Color correction ---------------------------------------------------------
// m0..m2 = combined contrast/brightness/saturation/hue affine matrix;
// p0 = (gamma_exponent, opacity, 0, 0)
@fragment
fn fs_color_correction(in: VsOut) -> @location(0) vec4<f32> {
    var color = textureSample(t_in, s_clamp, in.uv);
    var rgb = pow(max(color.rgb, vec3<f32>(0.0)), vec3<f32>(f.p0.x));
    rgb = vec3<f32>(
        dot(f.m0.xyz, rgb) + f.m0.w,
        dot(f.m1.xyz, rgb) + f.m1.w,
        dot(f.m2.xyz, rgb) + f.m2.w,
    );
    return vec4<f32>(clamp(rgb, vec3<f32>(0.0), vec3<f32>(1.0)), color.a * f.p0.y);
}

// -- LUT -----------------------------------------------------------------------
// group(2): the 3D lattice. p0 = (scale, offset, amount, 0)
@group(2) @binding(0) var t_lut: texture_3d<f32>;
@group(2) @binding(1) var s_lut: sampler;

@fragment
fn fs_lut(in: VsOut) -> @location(0) vec4<f32> {
    let color = textureSample(t_in, s_clamp, in.uv);
    let coord = clamp(color.rgb, vec3<f32>(0.0), vec3<f32>(1.0)) * f.p0.x + vec3<f32>(f.p0.y);
    let mapped = textureSampleLevel(t_lut, s_lut, coord, 0.0).rgb;
    return vec4<f32>(mix(color.rgb, mapped, f.p0.z), color.a);
}

// -- Gaussian blur (separable) --------------------------------------------------
// p0 = (radius_px, sigma, dir_x, dir_y)
@fragment
fn fs_blur(in: VsOut) -> @location(0) vec4<f32> {
    let radius = i32(f.p0.x);
    let sigma = max(f.p0.y, 0.25);
    let dir = f.p0.zw * f.texel.xy;
    var sum = vec4<f32>(0.0);
    var weight_sum = 0.0;
    for (var i = -radius; i <= radius; i += 1) {
        let t = f32(i) / sigma;
        let w = exp(-0.5 * t * t);
        sum += textureSampleLevel(t_in, s_clamp, in.uv + f32(i) * dir, 0.0) * w;
        weight_sum += w;
    }
    return sum / max(weight_sum, 1e-6);
}

// -- Image mask ------------------------------------------------------------------
// group(2): the mask image (2D). p0 = (mode 0=alpha/1=luma, invert 0/1, 0, 0)
// (Bindings 2/3, distinct from the LUT's 0/1 — one module, no collisions.)
@group(2) @binding(2) var t_mask: texture_2d<f32>;
@group(2) @binding(3) var s_mask: sampler;

@fragment
fn fs_mask(in: VsOut) -> @location(0) vec4<f32> {
    let color = textureSample(t_in, s_clamp, in.uv);
    let mask = textureSample(t_mask, s_mask, in.uv);
    var factor = select(mask.a, luma(mask.rgb), f.p0.x > 0.5);
    factor = select(factor, 1.0 - factor, f.p0.y > 0.5);
    return vec4<f32>(color.rgb, color.a * factor);
}

// -- Sharpen (unsharp mask) ---------------------------------------------------------
// p0 = (amount, 0, 0, 0)
@fragment
fn fs_sharpen(in: VsOut) -> @location(0) vec4<f32> {
    let center = textureSample(t_in, s_clamp, in.uv);
    let up = textureSample(t_in, s_clamp, in.uv + vec2<f32>(0.0, -f.texel.y));
    let down = textureSample(t_in, s_clamp, in.uv + vec2<f32>(0.0, f.texel.y));
    let left = textureSample(t_in, s_clamp, in.uv + vec2<f32>(-f.texel.x, 0.0));
    let right = textureSample(t_in, s_clamp, in.uv + vec2<f32>(f.texel.x, 0.0));
    let edge = 4.0 * center.rgb - up.rgb - down.rgb - left.rgb - right.rgb;
    let rgb = clamp(center.rgb + edge * f.p0.x, vec3<f32>(0.0), vec3<f32>(1.0));
    return vec4<f32>(rgb, center.a);
}

// -- Scroll ----------------------------------------------------------------------
// p0 = (offset_u, offset_v, 0, 0) — precomputed on the CPU from speed × time.
@fragment
fn fs_scroll(in: VsOut) -> @location(0) vec4<f32> {
    return textureSample(t_in, s_repeat, in.uv + f.p0.xy);
}

// -- Crop ------------------------------------------------------------------------
// p0 = the source window as UVs (u0, v0, u1, v1); the pass target is already
// the cropped size, so this is a remapped blit.
@fragment
fn fs_crop(in: VsOut) -> @location(0) vec4<f32> {
    let uv = mix(f.p0.xy, f.p0.zw, in.uv);
    return textureSample(t_in, s_clamp, uv);
}

// -- Flip ------------------------------------------------------------------------
// p0 = (horizontal, vertical, 0, 0) as 0/1 flags — a mirrored blit.
@fragment
fn fs_flip(in: VsOut) -> @location(0) vec4<f32> {
    let u = mix(in.uv.x, 1.0 - in.uv.x, f.p0.x);
    let v = mix(in.uv.y, 1.0 - in.uv.y, f.p0.y);
    return textureSample(t_in, s_clamp, vec2<f32>(u, v));
}
