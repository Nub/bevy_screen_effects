// Screen flash effect shader

struct FullscreenVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct FlashUniforms {
    color: vec4<f32>,
    blend: f32,      // 0.0 = additive, 1.0 = replace
    intensity: f32,
    _padding: vec2<f32>,
}

@group(1) @binding(0) var<uniform> params: FlashUniforms;

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

    // Sample the original screen color
    let screen_color = textureSample(screen_texture, texture_sampler, uv);

    // Calculate flash contribution
    let flash_alpha = params.color.a * params.intensity;

    // Blend based on blend mode
    // blend = 0: additive (add flash color on top)
    // blend = 1: replace (lerp toward flash color)
    let additive_result = screen_color.rgb + params.color.rgb * flash_alpha;
    let replace_result = mix(screen_color.rgb, params.color.rgb, flash_alpha);

    let final_color = mix(additive_result, replace_result, params.blend);

    return vec4<f32>(final_color, screen_color.a);
}
