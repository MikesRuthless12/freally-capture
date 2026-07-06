// Native-preview blit: draw the program texture onto the window surface.
//
// A single full-screen triangle (3 verts, no vertex buffer) samples the
// composed program frame straight onto the swapchain — no CPU readback, no
// JPEG. The pipeline's target format (the surface's preferred format, often
// Bgra8UnormSrgb) handles any color-space conversion from the Rgba8Unorm
// program texture.

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs(@builtin(vertex_index) vi: u32) -> VsOut {
    // Oversized triangle covering the viewport; uv in [0,1] across the frame.
    var out: VsOut;
    let uv = vec2<f32>(f32((vi << 1u) & 2u), f32(vi & 2u));
    out.uv = uv;
    out.pos = vec4<f32>(uv * 2.0 - 1.0, 0.0, 1.0);
    // Flip Y: clip space is y-up, texture uv is y-down.
    out.pos.y = -out.pos.y;
    return out;
}

@group(0) @binding(0) var program_tex: texture_2d<f32>;
@group(0) @binding(1) var program_sampler: sampler;

@fragment
fn fs(in: VsOut) -> @location(0) vec4<f32> {
    return textureSampleLevel(program_tex, program_sampler, in.uv, 0.0);
}
