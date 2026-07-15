// The stinger overlay (Phase 6): one fullscreen textured triangle drawn
// straight-alpha-over the already-rendered program — the stinger video
// covers the scene swap. Files with real alpha (e.g. ProRes 4444) composite
// transparently; opaque files simply cover the frame while they play.
//
// CAP-N29 track matte: when `matte.mode` is non-zero the frame packs the fill
// (color) and its matte (grayscale alpha) side by side — fill first, matte
// second — so per-pixel transparency survives codecs that drop alpha. The
// fill's colour is taken from its half and the alpha from the matte half's
// luminance (Rec.709), giving straight-alpha the ALPHA_BLENDING pipeline
// composites correctly.

@group(0) @binding(0) var tex: texture_2d<f32>;
@group(0) @binding(1) var samp: sampler;

struct Matte {
    // .x = mode (0 none, 1 horizontal split, 2 vertical split); .yzw unused.
    mode: vec4<f32>,
};
@group(1) @binding(0) var<uniform> matte: Matte;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> VsOut {
    var out: VsOut;
    let x = f32(i32(index) / 2) * 4.0 - 1.0;
    let y = f32(i32(index) & 1) * 4.0 - 1.0;
    out.pos = vec4<f32>(x, y, 0.0, 1.0);
    out.uv = vec2<f32>((x + 1.0) * 0.5, 1.0 - (y + 1.0) * 0.5);
    return out;
}

fn luma709(c: vec3<f32>) -> f32 {
    return dot(c, vec3<f32>(0.2126, 0.7152, 0.0722));
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let mode = matte.mode.x;
    if (mode < 0.5) {
        // No matte — the frame is used as-is (opaque or straight alpha).
        return textureSample(tex, samp, in.uv);
    }
    if (mode < 1.5) {
        // Horizontal split: fill in the left half, matte in the right half.
        let fill = textureSample(tex, samp, vec2<f32>(in.uv.x * 0.5, in.uv.y));
        let matte_px = textureSample(tex, samp, vec2<f32>(in.uv.x * 0.5 + 0.5, in.uv.y));
        return vec4<f32>(fill.rgb, luma709(matte_px.rgb));
    }
    // Vertical split: fill in the top half, matte in the bottom half.
    let fill = textureSample(tex, samp, vec2<f32>(in.uv.x, in.uv.y * 0.5));
    let matte_px = textureSample(tex, samp, vec2<f32>(in.uv.x, in.uv.y * 0.5 + 0.5));
    return vec4<f32>(fill.rgb, luma709(matte_px.rgb));
}
