// EMP / Electronic interference effect shader
// Combines flickering, color bands, static bursts, and scanline displacement

struct FullscreenVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct EmpUniforms {
    time: f32,
    intensity: f32,
    flicker_rate: f32,
    flicker_strength: f32,
    band_count: f32,
    band_intensity: f32,
    band_speed: f32,
    static_intensity: f32,
    burst_probability: f32,
    scanline_displacement: f32,
    chromatic_amount: f32,
    _padding: f32,
}

@group(1) @binding(0) var<uniform> params: EmpUniforms;

// Hash functions for noise
fn hash11(p: f32) -> f32 {
    var p1 = fract(p * 0.1031);
    p1 *= p1 + 33.33;
    p1 *= p1 + p1;
    return fract(p1);
}

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
    var uv = in.uv;
    let time = params.time;
    let intensity = params.intensity;

    // === FLICKER ===
    // Random brightness fluctuation
    let flicker_time = floor(time * params.flicker_rate);
    let flicker = 1.0 - hash11(flicker_time) * params.flicker_strength * intensity;

    // === SCANLINE DISPLACEMENT ===
    // Random horizontal offset per scanline
    let scanline_y = floor(uv.y * 200.0);
    let scanline_rand = hash21(vec2<f32>(scanline_y, floor(time * 20.0)));

    // Only displace some scanlines based on intensity
    if scanline_rand < intensity * 0.3 {
        let displacement = (hash21(vec2<f32>(scanline_y, time)) - 0.5) * params.scanline_displacement * intensity;
        uv.x += displacement;
    }

    // === COLOR BANDS ===
    // Horizontal bands that shift in color
    let band_y = uv.y + time * params.band_speed * 0.1;
    let band = sin(band_y * params.band_count * 3.14159) * 0.5 + 0.5;
    let band_offset = band * params.band_intensity * intensity;

    // === STATIC BURST ===
    // Occasional full-screen static bursts
    let burst_time = floor(time * 15.0);
    let burst_rand = hash11(burst_time);
    var burst_active = 0.0;
    if burst_rand < params.burst_probability * intensity {
        burst_active = 1.0;
    }

    // === CHROMATIC ABERRATION ===
    let chroma_offset = params.chromatic_amount * intensity;

    // Sample with chromatic aberration
    let r_uv = uv + vec2<f32>(chroma_offset + band_offset * 0.01, 0.0);
    let g_uv = uv;
    let b_uv = uv - vec2<f32>(chroma_offset + band_offset * 0.01, 0.0);

    var r = textureSample(screen_texture, texture_sampler, clamp(r_uv, vec2<f32>(0.0), vec2<f32>(1.0))).r;
    var g = textureSample(screen_texture, texture_sampler, clamp(g_uv, vec2<f32>(0.0), vec2<f32>(1.0))).g;
    var b = textureSample(screen_texture, texture_sampler, clamp(b_uv, vec2<f32>(0.0), vec2<f32>(1.0))).b;

    var color = vec3<f32>(r, g, b);

    // === APPLY COLOR BAND TINT ===
    // Shift colors in bands
    let band_hue = fract(band_y * 0.5 + time * 0.2);
    let band_tint = vec3<f32>(
        sin(band_hue * 6.28) * 0.5 + 0.5,
        sin((band_hue + 0.33) * 6.28) * 0.5 + 0.5,
        sin((band_hue + 0.66) * 6.28) * 0.5 + 0.5
    );
    color = mix(color, color * band_tint, band_offset * 0.5);

    // === STATIC NOISE ===
    let static_noise = hash21(uv * 1000.0 + time * 100.0);
    let static_amount = params.static_intensity * intensity;

    // Mix in static noise
    color = mix(color, vec3<f32>(static_noise), static_amount * 0.3);

    // During bursts, add more intense static
    if burst_active > 0.5 {
        let burst_noise = hash21(uv * 500.0 + time * 200.0);
        color = mix(color, vec3<f32>(burst_noise), 0.4 * intensity);
    }

    // === APPLY FLICKER ===
    color *= flicker;

    // === OCCASIONAL BLACK BARS ===
    // Rolling black bars like old TV interference
    let bar_pos = fract(time * 0.3);
    let bar_dist = abs(uv.y - bar_pos);
    if bar_dist < 0.02 * intensity {
        let bar_strength = 1.0 - bar_dist / (0.02 * intensity);
        color *= 1.0 - bar_strength * 0.7 * intensity;
    }

    // === EDGE DARKENING during interference ===
    let edge_dist = max(abs(uv.x - 0.5), abs(uv.y - 0.5)) * 2.0;
    let edge_vignette = smoothstep(0.8, 1.0, edge_dist);
    color *= 1.0 - edge_vignette * 0.3 * intensity;

    return vec4<f32>(color, 1.0);
}
