// Shockwave distortion effect shader

struct FullscreenVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct ShockwaveUniforms {
    center: vec2<f32>,
    intensity: f32,
    progress: f32,
    ring_width: f32,
    max_radius: f32,
    chromatic: u32,
    _padding: f32,
}

@group(1) @binding(0) var<uniform> params: ShockwaveUniforms;

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

    // Current radius of the shockwave ring
    let current_radius = params.progress * params.max_radius;

    // Distance from center
    let dist = distance(uv, center);

    // Distance from the ring edge
    let ring_dist = abs(dist - current_radius);

    // Ring falloff (1.0 at ring center, 0.0 outside)
    let ring_factor = 1.0 - smoothstep(0.0, params.ring_width, ring_dist);

    // Skip if outside ring
    if ring_factor < 0.001 {
        return textureSample(screen_texture, texture_sampler, uv);
    }

    // Displacement direction (away from center)
    let dir = normalize(uv - center);

    // Displacement amount (stronger at ring, fades with progress)
    let displacement = dir * ring_factor * params.intensity * (1.0 - params.progress);

    if params.chromatic != 0u {
        // Chromatic aberration version - sample each channel with different offsets
        let r = textureSample(screen_texture, texture_sampler, uv + displacement * 1.2).r;
        let g = textureSample(screen_texture, texture_sampler, uv + displacement * 0.6).g;
        let b = textureSample(screen_texture, texture_sampler, uv).b;
        return vec4<f32>(r, g, b, 1.0);
    } else {
        // Simple displacement
        return textureSample(screen_texture, texture_sampler, uv + displacement);
    }
}
