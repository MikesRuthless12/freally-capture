// Telestrator (CAP-N57): solid-colored triangles drawn straight-alpha-over the
// finished program, so live free-hand annotation is BAKED into what viewers see
// and what is recorded. Geometry (position in clip space, straight-alpha color)
// arrives in one vertex buffer — see `telestrator::tessellate`.

struct VsIn {
    @location(0) pos: vec2<f32>,
    @location(1) color: vec4<f32>,
};

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(in: VsIn) -> VsOut {
    var out: VsOut;
    out.pos = vec4<f32>(in.pos, 0.0, 1.0);
    out.color = in.color;
    return out;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    // Straight alpha; the pipeline's ALPHA_BLENDING multiplies rgb by a.
    return in.color;
}
