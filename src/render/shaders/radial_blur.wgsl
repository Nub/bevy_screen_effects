// Radial blur effect shader

struct FullscreenVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct RadialBlurUniforms {
    center: vec2<f32>,
    intensity: f32,
    samples: u32,
}

@group(1) @binding(0) var<uniform> params: RadialBlurUniforms;

@vertex
fn vertex(@builtin(vertex_index) vertex_index: u32) -> FullscreenVertexOutput {
    let uv = vec2<f32>(
        f32(vertex_index & 1u) * 2.0,
        f32((vertex_index >> 1u) & 1u) * 2.0
    );
    var output: FullscreenVertexOutput;
    output.position = vec4<f32>(uv.x * 2.0 - 1.0, 1.0 - uv.y * 2.0, 0.0, 1.0);
    output.uv = uv;
    return output;
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let center = params.center;

    // Direction from center to this pixel
    let dir = uv - center;
    let dist = length(dir);

    // Blur amount increases with distance from center
    let blur_amount = params.intensity * dist;

    var color = vec4<f32>(0.0);
    let samples_f = f32(params.samples);

    // Sample along the radial direction
    for (var i = 0u; i < params.samples; i++) {
        let t = f32(i) / samples_f;
        let offset = dir * blur_amount * t;
        color += textureSample(screen_texture, texture_sampler, uv - offset);
    }

    return color / samples_f;
}
