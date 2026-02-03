// Combined glitch effects shader (scanlines, block displacement, static noise)

struct FullscreenVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct GlitchUniforms {
    time: f32,
    intensity: f32,
    rgb_split_amount: f32,
    scanline_density: f32,
    block_size: vec2<f32>,
    noise_amount: f32,
    _padding: f32,
}

@group(1) @binding(0) var<uniform> params: GlitchUniforms;

// Pseudo-random function
fn rand(co: vec2<f32>) -> f32 {
    return fract(sin(dot(co, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

// Quantize to grid
fn quantize(uv: vec2<f32>, grid: vec2<f32>) -> vec2<f32> {
    return floor(uv * grid) / grid;
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

    // Block displacement
    if params.block_size.x > 0.0 && params.block_size.y > 0.0 {
        let block_uv = quantize(uv, 1.0 / params.block_size);
        let block_rand = rand(block_uv + floor(time * 15.0));

        if block_rand < intensity * 0.3 {
            let offset = (rand(block_uv + time) - 0.5) * 0.15 * intensity;
            uv.x += offset;
        }
    }

    // Scanline displacement
    if params.scanline_density > 0.0 {
        let line = floor(uv.y * params.scanline_density);
        let line_rand = rand(vec2<f32>(line, floor(time * 30.0)));

        if line_rand < intensity * 0.15 {
            uv.x += (line_rand - 0.5) * 0.08 * intensity;
        }
    }

    // Sample with optional RGB split
    var color: vec4<f32>;
    if params.rgb_split_amount > 0.0 {
        let split = params.rgb_split_amount * intensity;
        let r = textureSample(screen_texture, texture_sampler, uv + vec2<f32>(split, 0.0)).r;
        let g = textureSample(screen_texture, texture_sampler, uv).g;
        let b = textureSample(screen_texture, texture_sampler, uv - vec2<f32>(split, 0.0)).b;
        color = vec4<f32>(r, g, b, 1.0);
    } else {
        color = textureSample(screen_texture, texture_sampler, uv);
    }

    // Static noise overlay
    if params.noise_amount > 0.0 {
        let noise = rand(uv * 1000.0 + time * 100.0);
        let noise_color = vec3<f32>(noise);
        color = vec4<f32>(
            mix(color.rgb, noise_color, params.noise_amount * intensity * 0.5),
            color.a
        );
    }

    return color;
}
