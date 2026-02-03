// Fullscreen vertex shader - generates a full-screen triangle
// This is a standard technique that draws a single triangle covering the entire screen

struct FullscreenVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn fullscreen_vertex(@builtin(vertex_index) vertex_index: u32) -> FullscreenVertexOutput {
    // Generate a triangle that covers the full screen
    // Vertices: (0,0), (2,0), (0,2) in clip space maps to full screen
    let uv = vec2<f32>(
        f32(vertex_index & 1u) * 2.0,
        f32((vertex_index >> 1u) & 1u) * 2.0
    );

    var output: FullscreenVertexOutput;
    // Convert from [0,2] to [-1,1] clip space, flip Y for proper UV orientation
    output.position = vec4<f32>(uv.x * 2.0 - 1.0, 1.0 - uv.y * 2.0, 0.0, 1.0);
    output.uv = uv;
    return output;
}
