// Studio Mode transitions: blend two composed scenes — A = the outgoing
// program, B = the committed preview — into the program texture in one
// fullscreen pass. `mode` picks the math; `progress` runs 0→1 over the
// transition's duration. Every mode reduces to "sample A at uv_a, B at uv_b,
// mix by k", so the scene samples stay in uniform control flow (WGSL
// requires that for implicit derivatives). The luma-image mode samples the
// wipe pattern with an explicit LOD, which is legal anywhere; a 1×1 white
// dummy is bound when no image is set.

struct TransitionUniform {
    // x = progress (0..1) · y = mode · zw = the direction B enters from.
    params: vec4<f32>,
};

@group(0) @binding(0) var<uniform> u: TransitionUniform;
@group(1) @binding(0) var tex_a: texture_2d<f32>;
@group(1) @binding(1) var samp_a: sampler;
@group(2) @binding(0) var tex_b: texture_2d<f32>;
@group(2) @binding(1) var samp_b: sampler;
@group(3) @binding(0) var tex_luma: texture_2d<f32>;
@group(3) @binding(1) var samp_luma: sampler;

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

// A soft-edged wipe: reveal where the pattern value is below the sweeping
// threshold (the classic luma-wipe response).
fn wipe(luma: f32, p: f32) -> f32 {
    return smoothstep(luma - 0.08, luma + 0.08, p * 1.16 - 0.08);
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
        k = wipe(luma, p);
    } else if (mode == 4u) {
        // Luma wipe, built-in radial pattern (center out).
        let d = distance(uv, vec2<f32>(0.5, 0.5)) / 0.7071068;
        k = wipe(d, p);
    } else if (mode == 5u) {
        // Luma wipe, horizontal sweep (left → right).
        k = wipe(uv.x, p);
    } else if (mode == 6u) {
        // Luma wipe, diamond (center out along |x|+|y|).
        let d = (abs(uv.x - 0.5) + abs(uv.y - 0.5));
        k = wipe(d, p);
    } else if (mode == 7u) {
        // Luma wipe, clock sweep (12 o'clock, clockwise).
        let v = uv - vec2<f32>(0.5, 0.5);
        let angle = atan2(v.x, -v.y); // 0 at 12 o'clock, clockwise positive
        let sweep = (angle + 3.1415927) / 6.2831853;
        k = wipe(sweep, p);
    } else if (mode == 8u) {
        // Luma wipe from the custom image's luminance (explicit LOD — legal
        // in divergent flow; a white dummy makes this a hard cut at p=1).
        let sample = textureSampleLevel(tex_luma, samp_luma, uv, 0.0);
        let luma = dot(sample.rgb, vec3<f32>(0.2126, 0.7152, 0.0722));
        k = wipe(luma, p);
    }

    let a = textureSample(tex_a, samp_a, clamp(uv_a, vec2<f32>(0.0), vec2<f32>(1.0)));
    let b = textureSample(tex_b, samp_b, clamp(uv_b, vec2<f32>(0.0), vec2<f32>(1.0)));
    return mix(a, b, k);
}
