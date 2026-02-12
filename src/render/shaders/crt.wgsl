// CRT screen effect shader
// Simulates cathode ray tube display characteristics in a single pass

struct FullscreenVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct CrtUniforms {
    // Row 1
    time: f32,
    intensity: f32,
    scanline_intensity: f32,
    scanline_count: f32,
    // Row 2
    curvature: f32,
    corner_radius: f32,
    phosphor_type: u32,
    phosphor_intensity: f32,
    // Row 3
    bloom: f32,
    vignette: f32,
    flicker: f32,
    color_bleed: f32,
    // Row 4
    brightness: f32,
    saturation: f32,
    screen_width: f32,
    screen_height: f32,
    // Row 5
    mask_shape: u32,   // 0 = rounded_rect, 1 = ellipse
    _padding0: f32,
    _padding1: f32,
    _padding2: f32,
}

@group(1) @binding(0) var<uniform> params: CrtUniforms;

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

// Barrel distortion: remap UVs from center outward
fn barrel_distort(uv: vec2<f32>, amount: f32) -> vec2<f32> {
    let centered = uv - 0.5;
    let r2 = dot(centered, centered);
    return uv + centered * amount * r2;
}

// Rounded rectangle SDF for corner masking
fn rounded_rect(uv: vec2<f32>, radius: f32) -> f32 {
    let centered = abs(uv - 0.5) * 2.0;
    let size = vec2<f32>(1.0, 1.0);
    let d = centered - size + vec2<f32>(radius);
    return length(max(d, vec2<f32>(0.0))) - radius;
}

// Ellipse SDF for round mask
fn ellipse_sdf(uv: vec2<f32>, radius: f32) -> f32 {
    let centered = (uv - 0.5) * 2.0;
    // Shrink the ellipse by corner_radius to create a border
    let scale = 1.0 - radius;
    return length(centered / scale) - 1.0;
}

// Shadow mask: dot triad pattern
fn shadow_mask(pixel: vec2<f32>) -> vec3<f32> {
    let col = i32(pixel.x) % 3;
    let row = i32(pixel.y) % 3;
    var mask = vec3<f32>(0.5);

    if col == 0 {
        mask.r = 1.0;
    } else if col == 1 {
        mask.g = 1.0;
    } else {
        mask.b = 1.0;
    }

    // Offset every other row for triangle pattern
    if row % 2 == 1 {
        let shifted_col = (i32(pixel.x) + 1) % 3;
        mask = vec3<f32>(0.5);
        if shifted_col == 0 {
            mask.r = 1.0;
        } else if shifted_col == 1 {
            mask.g = 1.0;
        } else {
            mask.b = 1.0;
        }
    }

    return mask;
}

// Aperture grille: vertical RGB stripes
fn aperture_grille(pixel: vec2<f32>) -> vec3<f32> {
    let col = i32(pixel.x) % 3;
    var mask = vec3<f32>(0.4);

    if col == 0 {
        mask.r = 1.0;
    } else if col == 1 {
        mask.g = 1.0;
    } else {
        mask.b = 1.0;
    }

    return mask;
}

// Slot mask: 2D repeating RGB grid
fn slot_mask(pixel: vec2<f32>) -> vec3<f32> {
    let col = i32(pixel.x) % 3;
    let row = i32(pixel.y) % 2;
    var mask = vec3<f32>(0.4);

    if row == 0 {
        if col == 0 {
            mask.r = 1.0;
        } else if col == 1 {
            mask.g = 1.0;
        } else {
            mask.b = 1.0;
        }
    } else {
        // Slightly dimmer between slots
        mask = vec3<f32>(0.6);
    }

    return mask;
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let intensity = params.intensity;
    let screen_res = vec2<f32>(params.screen_width, params.screen_height);

    // === 1. BARREL DISTORTION ===
    let distorted_uv = barrel_distort(in.uv, params.curvature * intensity);

    // === 2. SCREEN MASK ===
    var corner_dist: f32;
    if params.mask_shape == 1u {
        corner_dist = ellipse_sdf(distorted_uv, params.corner_radius * intensity);
    } else {
        corner_dist = rounded_rect(distorted_uv, params.corner_radius * intensity);
    }
    if corner_dist > 0.0 {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }

    // Out-of-bounds check after distortion
    if distorted_uv.x < 0.0 || distorted_uv.x > 1.0 || distorted_uv.y < 0.0 || distorted_uv.y > 1.0 {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }

    // === 5. COLOR BLEED (sample before other color ops) ===
    let bleed = params.color_bleed * intensity;
    let r = textureSample(screen_texture, texture_sampler, distorted_uv + vec2<f32>(bleed, 0.0)).r;
    let g = textureSample(screen_texture, texture_sampler, distorted_uv).g;
    let b = textureSample(screen_texture, texture_sampler, distorted_uv - vec2<f32>(bleed, 0.0)).b;
    var color = vec3<f32>(r, g, b);

    // === 6. BLOOM (cheap 5-tap cross blur of bright areas) ===
    let bloom_amount = params.bloom * intensity;
    if bloom_amount > 0.0 {
        let texel = 1.0 / screen_res;
        let bloom_offset = texel * 2.0;
        let center = textureSample(screen_texture, texture_sampler, distorted_uv).rgb;
        let t = textureSample(screen_texture, texture_sampler, distorted_uv + vec2<f32>(0.0, -bloom_offset.y)).rgb;
        let b2 = textureSample(screen_texture, texture_sampler, distorted_uv + vec2<f32>(0.0, bloom_offset.y)).rgb;
        let l = textureSample(screen_texture, texture_sampler, distorted_uv + vec2<f32>(-bloom_offset.x, 0.0)).rgb;
        let r2 = textureSample(screen_texture, texture_sampler, distorted_uv + vec2<f32>(bloom_offset.x, 0.0)).rgb;
        let blur = (center + t + b2 + l + r2) * 0.2;
        // Only add bloom from bright areas
        let luma = dot(blur, vec3<f32>(0.299, 0.587, 0.114));
        let bloom_contrib = blur * smoothstep(0.4, 1.0, luma);
        color += bloom_contrib * bloom_amount;
    }

    // === 3. SCANLINES ===
    let scanline_pos = distorted_uv.y * params.scanline_count;
    let scanline = sin(scanline_pos * 3.14159265) * sin(scanline_pos * 3.14159265);
    color *= 1.0 - params.scanline_intensity * intensity * (1.0 - scanline);

    // === 4. PHOSPHOR MASK ===
    let pixel_pos = distorted_uv * screen_res;
    if params.phosphor_type == 1u {
        let mask = shadow_mask(pixel_pos);
        color *= mix(vec3<f32>(1.0), mask, params.phosphor_intensity * intensity);
    } else if params.phosphor_type == 2u {
        let mask = aperture_grille(pixel_pos);
        color *= mix(vec3<f32>(1.0), mask, params.phosphor_intensity * intensity);
    } else if params.phosphor_type == 3u {
        let mask = slot_mask(pixel_pos);
        color *= mix(vec3<f32>(1.0), mask, params.phosphor_intensity * intensity);
    }

    // === 7. VIGNETTE ===
    let vig_uv = (distorted_uv - 0.5) * 2.0;
    let vig_dist = length(vig_uv);
    let vig = smoothstep(1.0, 1.0 - params.vignette * intensity, vig_dist);
    color *= vig;

    // === 8. FLICKER ===
    color *= 1.0 + sin(params.time * 120.0) * params.flicker * intensity;

    // === 9. BRIGHTNESS & SATURATION ===
    let luma = dot(color, vec3<f32>(0.299, 0.587, 0.114));
    color = mix(vec3<f32>(luma), color, params.saturation) * params.brightness;

    // Soften the corner edge slightly
    let corner_edge = smoothstep(0.0, -0.005, corner_dist);
    color *= corner_edge;

    return vec4<f32>(color, 1.0);
}
