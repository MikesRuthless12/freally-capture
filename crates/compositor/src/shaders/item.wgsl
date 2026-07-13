// Composite one scene item onto the program canvas.
//
// The vertex stage expands a 4-vertex triangle strip into the item's local
// content rectangle and pushes it through the precomputed clip matrix
// (transform.rs). The fragment stage samples the (possibly filtered) source
// and applies the blend-mode prep the fixed-function blender needs:
//   prep 0 — straight alpha (normal / additive / subtract use SrcAlpha factors)
//   prep 1 — premultiply by alpha        (screen, lighten)
//   prep 2 — mix toward white by alpha   (multiply, darken)
// so transparent pixels leave the canvas untouched in every mode.

struct ItemUniform {
    mvp: mat4x4<f32>,
    // u0, v0, u1, v1 — the transform crop's window into the source texture.
    uv_rect: vec4<f32>,
    // xy = content size in local px; zw reserved.
    size: vec4<f32>,
    // x = blend prep mode (0/1/2);
    // y = sampling mode (0 smooth, 1 nearest, 2 sharp-bilinear — CAP-N70);
    // z = the drawn scale (sharp-bilinear's sharpness); w reserved.
    misc: vec4<f32>,
};

@group(0) @binding(0) var<uniform> item: ItemUniform;
@group(1) @binding(0) var t_source: texture_2d<f32>;
@group(1) @binding(1) var s_source: sampler;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vi: u32) -> VsOut {
    // vi 0..3 → corners (0,0) (1,0) (0,1) (1,1) as a triangle strip.
    let corner = vec2<f32>(f32(vi & 1u), f32(vi >> 1u));
    var out: VsOut;
    out.pos = item.mvp * vec4<f32>(corner * item.size.xy, 0.0, 1.0);
    out.uv = mix(item.uv_rect.xy, item.uv_rect.zw, corner);
    return out;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    // Pixel-perfect sampling (CAP-N70), all on the one linear sampler:
    // nearest = snap the UV to the texel center; sharp-bilinear = snap with
    // a half-texel-wide linear edge (fract sharpened by the drawn scale) —
    // crisp pixels without raw nearest's shimmer under motion.
    var uv = in.uv;
    let sampling = u32(item.misc.y + 0.5);
    if sampling != 0u {
        let dims = vec2<f32>(textureDimensions(t_source));
        let texel = uv * dims;
        if sampling == 1u {
            uv = (floor(texel) + 0.5) / dims;
        } else {
            let sharp = max(item.misc.z, 1.0);
            let edge = clamp((fract(texel) - 0.5) * sharp, vec2<f32>(-0.5), vec2<f32>(0.5));
            uv = (floor(texel) + 0.5 + edge) / dims;
        }
    }
    var color = textureSample(t_source, s_source, uv);
    let prep = u32(item.misc.x + 0.5);
    if prep == 1u {
        color = vec4<f32>(color.rgb * color.a, color.a);
    } else if prep == 2u {
        color = vec4<f32>(mix(vec3<f32>(1.0), color.rgb, color.a), color.a);
    }
    return color;
}
