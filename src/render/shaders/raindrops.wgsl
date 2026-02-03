// Raindrops effect shader - procedural raindrops with refraction

struct FullscreenVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct RaindropsUniforms {
    time: f32,
    intensity: f32,
    drop_size: f32,
    density: f32,
    speed: f32,
    refraction: f32,
    trail_strength: f32,
    _padding: f32,
}

@group(1) @binding(0) var<uniform> params: RaindropsUniforms;

// Hash functions for procedural generation
fn hash21(p: vec2<f32>) -> f32 {
    var p3 = fract(vec3<f32>(p.x, p.y, p.x) * 0.1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}

fn hash22(p: vec2<f32>) -> vec2<f32> {
    let n = sin(dot(p, vec2<f32>(41.0, 289.0)));
    return fract(vec2<f32>(262144.0, 32768.0) * n);
}

// Smooth noise
fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);

    return mix(
        mix(hash21(i), hash21(i + vec2<f32>(1.0, 0.0)), u.x),
        mix(hash21(i + vec2<f32>(0.0, 1.0)), hash21(i + vec2<f32>(1.0, 1.0)), u.x),
        u.y
    );
}

// Single raindrop shape with refraction
fn raindrop(uv: vec2<f32>, center: vec2<f32>, size: f32) -> vec2<f32> {
    let d = uv - center;
    let aspect = vec2<f32>(1.0, 1.6); // Elongated drops
    let scaled_d = d * aspect;
    let dist = length(scaled_d);

    if dist > size {
        return vec2<f32>(0.0);
    }

    // Drop shape - sphere-like with tapered top
    let drop_shape = 1.0 - smoothstep(0.0, size, dist);
    let height = drop_shape * drop_shape; // Curved surface

    // Refraction direction (lens effect)
    let refract_dir = normalize(d) * height * params.refraction * size;

    return refract_dir;
}

// Trail behind a falling drop
fn drop_trail(uv: vec2<f32>, center: vec2<f32>, size: f32, trail_len: f32) -> vec2<f32> {
    let d = uv - center;

    // Only above the drop center
    if d.y < 0.0 || d.y > trail_len {
        return vec2<f32>(0.0);
    }

    // Width narrows toward top
    let width = size * 0.3 * (1.0 - d.y / trail_len);
    if abs(d.x) > width {
        return vec2<f32>(0.0);
    }

    let strength = (1.0 - d.y / trail_len) * (1.0 - abs(d.x) / width);
    return vec2<f32>(d.x * 0.5, 0.0) * strength * params.trail_strength * params.refraction;
}

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
    var total_refract = vec2<f32>(0.0);

    // Grid-based raindrop spawning for better distribution
    let grid_size = 1.0 / (params.density * 4.0 + 1.0);
    let grid_pos = floor(uv / grid_size);

    // Check surrounding grid cells
    for (var dy = -1; dy <= 1; dy++) {
        for (var dx = -1; dx <= 1; dx++) {
            let cell = grid_pos + vec2<f32>(f32(dx), f32(dy));
            let cell_hash = hash22(cell);

            // Random position within cell
            let drop_base = (cell + cell_hash) * grid_size;

            // Animate falling
            let fall_speed = params.speed * (0.5 + cell_hash.y * 0.5);
            let fall_offset = fract(params.time * fall_speed + cell_hash.x);
            let drop_pos = vec2<f32>(
                drop_base.x,
                1.0 - fall_offset
            );

            // Random size variation
            let size = params.drop_size * (0.5 + hash21(cell * 7.0) * 0.5);

            // Only render some drops based on density
            if hash21(cell * 13.0) < params.density {
                // Add drop refraction
                total_refract += raindrop(uv, drop_pos, size);

                // Add trail
                if params.trail_strength > 0.0 {
                    let trail_len = size * 4.0 * fall_speed;
                    total_refract += drop_trail(uv, drop_pos, size, trail_len);
                }
            }
        }
    }

    // Apply intensity
    total_refract *= params.intensity;

    // Sample with refraction offset
    let refracted_uv = clamp(uv + total_refract, vec2<f32>(0.0), vec2<f32>(1.0));
    let color = textureSample(screen_texture, texture_sampler, refracted_uv);

    return color;
}
