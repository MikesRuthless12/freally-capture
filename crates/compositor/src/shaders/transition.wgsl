// Studio Mode transitions (Phase 5): blend two composed scenes — A = the
// outgoing program, B = the committed preview — into the program texture in
// one fullscreen pass. `mode` picks the math; `progress` runs 0→1 over the
// transition's duration. Every mode reduces to "sample A at uv_a, B at uv_b,
// mix by k", so the texture samples stay in uniform control flow (WGSL
// requires that for implicit derivatives).

struct TransitionUniform {
    // x = progress (0..1) · y = mode · zw = the direction B enters from.
    params: vec4<f32>,
};

@group(0) @binding(0) var<uniform> u: TransitionUniform;
@group(1) @binding(0) var tex_a: texture_2d<f32>;
@group(1) @binding(1) var samp_a: sampler;
@group(2) @binding(0) var tex_b: texture_2d<f32>;
@group(2) @binding(1) var samp_b: sampler;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> VsOut {
    // One oversized triangle covering the canvas.
    var out: VsOut;
    let x = f32(i32(index) / 2) * 4.0 - 1.0;
    let y = f32(i32(index) & 1) * 4.0 - 1.0;
    out.pos = vec4<f32>(x, y, 0.0, 1.0);
    out.uv = vec2<f32>((x + 1.0) * 0.5, 1.0 - (y + 1.0) * 0.5);
    return out;
}

fn in_unit(uv: vec2<f32>) -> f32 {
    let clamped = clamp(uv, vec2<f32>(0.0), vec2<f32>(1.0));
    return select(0.0, 1.0, all(clamped == uv));
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let p = clamp(u.params.x, 0.0, 1.0);
    let mode = u32(u.params.y + 0.5);
    let dir = u.params.zw;
    let uv = in.uv;

    var uv_a = uv;
    var uv_b = uv;
    var k = p; // B's weight — the fade default.

    if (mode == 1u) {
        // Slide: A moves out while B moves in — exact complementary coverage.
        uv_a = uv + dir * p;
        uv_b = uv - dir * (1.0 - p);
        k = in_unit(uv_b);
    } else if (mode == 2u) {
        // Swipe: B slides in over a static A.
        uv_b = uv - dir * (1.0 - p);
        k = in_unit(uv_b);
    } else if (mode == 3u) {
        // Luma wipe, built-in linear pattern (a soft diagonal edge).
        let luma = dot(uv, vec2<f32>(0.7071068, 0.7071068));
        k = smoothstep(luma - 0.08, luma + 0.08, p * 1.16 - 0.08);
    } else if (mode == 4u) {
        // Luma wipe, built-in radial pattern (center out).
        let d = distance(uv, vec2<f32>(0.5, 0.5)) / 0.7071068;
        k = smoothstep(d - 0.08, d + 0.08, p * 1.16 - 0.08);
    }

    let a = textureSample(tex_a, samp_a, clamp(uv_a, vec2<f32>(0.0), vec2<f32>(1.0)));
    let b = textureSample(tex_b, samp_b, clamp(uv_b, vec2<f32>(0.0), vec2<f32>(1.0)));
    return mix(a, b, k);
}
