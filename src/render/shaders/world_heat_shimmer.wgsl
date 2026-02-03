// World-space heat shimmer effect shader
// Creates a rising column of wavy distortion tied to a world position

struct FullscreenVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct WorldHeatShimmerUniforms {
    // Screen-space bounds (left, right, top, bottom) in UV coordinates
    bounds: vec4<f32>,
    amplitude: f32,
    frequency: f32,
    speed: f32,
    softness: f32,
    time: f32,
    intensity: f32,
    _padding: vec2<f32>,
}

@group(1) @binding(0) var<uniform> params: WorldHeatShimmerUniforms;

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

    // Extract bounds: (left, right, top, bottom)
    let left = params.bounds.x;
    let right = params.bounds.y;
    let top = params.bounds.z;
    let bottom = params.bounds.w;

    // Check if within column bounds
    let in_x = uv.x >= left && uv.x <= right;
    let in_y = uv.y >= top && uv.y <= bottom;

    if !in_x || !in_y {
        return textureSample(screen_texture, texture_sampler, uv);
    }

    // Calculate normalized position within the bounds
    let width = right - left;
    let height = bottom - top;

    // Calculate falloff from edges (soft edges)
    let dx_left = (uv.x - left) / width;
    let dx_right = (right - uv.x) / width;
    let dy_top = (uv.y - top) / height;
    let dy_bottom = (bottom - uv.y) / height;

    // Use softness to control edge falloff
    let softness_normalized = params.softness * 0.5; // Scale softness to reasonable range
    let edge_x = smoothstep(0.0, softness_normalized, min(dx_left, dx_right));
    let edge_y = smoothstep(0.0, softness_normalized, min(dy_top, dy_bottom));
    let falloff = edge_x * edge_y;

    // Rising wave displacement - wave moves upward (negative y in screen space)
    // Use uv.y in the wave calculation so waves rise from bottom to top
    let wave_phase = (uv.y * params.frequency) - (params.time * params.speed);
    let wave = sin(wave_phase);

    // Add some variation with a secondary wave
    let wave2 = sin(wave_phase * 1.7 + 0.5) * 0.5;
    let combined_wave = (wave + wave2) * 0.67;

    // Horizontal displacement only (heat shimmer wobbles left/right)
    let displacement = vec2<f32>(
        combined_wave * params.amplitude * falloff * params.intensity,
        0.0
    );

    return textureSample(screen_texture, texture_sampler, uv + displacement);
}
