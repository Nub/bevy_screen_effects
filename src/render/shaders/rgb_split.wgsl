// RGB channel split / chromatic aberration effect shader

struct FullscreenVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct RgbSplitUniforms {
    red_offset: vec2<f32>,
    green_offset: vec2<f32>,
    blue_offset: vec2<f32>,
    intensity: f32,
    _padding: f32,
}

@group(1) @binding(0) var<uniform> params: RgbSplitUniforms;

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
    let intensity = params.intensity;

    // Sample each color channel with its own offset
    let r = textureSample(screen_texture, texture_sampler, uv + params.red_offset * intensity).r;
    let g = textureSample(screen_texture, texture_sampler, uv + params.green_offset * intensity).g;
    let b = textureSample(screen_texture, texture_sampler, uv + params.blue_offset * intensity).b;

    // Get alpha from center sample
    let a = textureSample(screen_texture, texture_sampler, uv).a;

    return vec4<f32>(r, g, b, a);
}
