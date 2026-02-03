// Damage vignette effect shader

struct FullscreenVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct VignetteUniforms {
    color: vec4<f32>,
    size: f32,
    softness: f32,
    pulse_frequency: f32,
    time: f32,
    intensity: f32,
    // Use separate f32 values to avoid vec3's 16-byte alignment requirement
    _padding0: f32,
    _padding1: f32,
    _padding2: f32,
}

@group(1) @binding(0) var<uniform> params: VignetteUniforms;

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

    // Vignette factor (1 at edges, 0 at center)
    let edge_dist = max(abs(uv.x - 0.5), abs(uv.y - 0.5)) * 2.0; // 0 to 1
    let vignette_raw = smoothstep(1.0 - params.size, 1.0 - params.size + params.softness, edge_dist);

    // Apply pulsing if enabled
    var vignette = vignette_raw;
    if params.pulse_frequency > 0.0 {
        let pulse = sin(params.time * params.pulse_frequency) * 0.5 + 0.5;
        vignette *= 0.7 + pulse * 0.3;
    }

    // Apply intensity
    vignette *= params.intensity;

    // Blend vignette color with screen color
    let vignette_color = vec4<f32>(params.color.rgb, params.color.a * vignette);

    // Overlay blend - vignette on top of screen
    return vec4<f32>(
        mix(screen_color.rgb, vignette_color.rgb, vignette_color.a),
        screen_color.a
    );
}
