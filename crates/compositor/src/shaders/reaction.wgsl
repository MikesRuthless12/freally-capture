// Floating reactions (Phase 6, TASK-614): one small textured quad per
// live particle, drawn straight-alpha-over the composed program — so the
// emoji are BAKED into what is recorded and streamed, at the exact moment
// viewers reacted. Positions arrive per-draw via a dynamic-offset uniform.

struct ReactionUniform {
    // center.xy, size.xy — all in clip space.
    rect: vec4<f32>,
    // x = alpha; yzw unused.
    tint: vec4<f32>,
};

@group(0) @binding(0) var<uniform> u: ReactionUniform;
@group(1) @binding(0) var tex: texture_2d<f32>;
@group(1) @binding(1) var samp: sampler;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> VsOut {
    // A unit quad as a triangle strip: (0,0)(1,0)(0,1)(1,1).
    let corner = vec2<f32>(f32(index & 1u), f32(index >> 1u));
    var out: VsOut;
    let offset = (corner - vec2<f32>(0.5, 0.5)) * u.rect.zw;
    out.pos = vec4<f32>(u.rect.xy + offset, 0.0, 1.0);
    out.uv = vec2<f32>(corner.x, 1.0 - corner.y);
    return out;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let color = textureSample(tex, samp, in.uv);
    return vec4<f32>(color.rgb, color.a * u.tint.x);
}
