# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`bevy_screen_effects` is a Bevy plugin library providing dynamic, event-driven screen space effects for games. Unlike static post-processing, these effects are spawned as entities with lifetimes - they animate and despawn automatically.

**Target:** Bevy 0.18, Rust 2024 edition

## Development Commands

```bash
# Enter dev environment
nix develop

# Build
cargo build

# Run example
cargo run --example showcase

# Check with all features
cargo check --all-features

# Test
cargo test
```

## Architecture

### Effect Spawning Pattern

Effects are ECS entities with components, not persistent post-processing passes:

```rust
// Spawn an effect - it animates and auto-despawns
commands.spawn(ShockwaveBundle {
    shockwave: Shockwave::at(0.5, 0.5).with_intensity(0.3),
    lifetime: EffectLifetime::new(0.5),
    ..default()
});
```

### Core Components

- `ScreenEffect` - Marker component for all active effects
- `EffectIntensity` - Current intensity (0.0-1.0), driven by lifetime
- `EffectLifetime` - Handles duration, fade in/out, easing, auto-despawn
- `EffectOrigin` - Screen position in normalized coords (0.0-1.0)

### Module Structure

```
src/
├── lib.rs              # ScreenEffectsPlugin entry point
├── effect.rs           # Core marker components
├── lifetime.rs         # Timing, animation, auto-despawn systems
├── render/
│   ├── mod.rs          # Plugin setup, shader loading, render graph
│   ├── node.rs         # ScreenEffectsNode (ViewNode implementation)
│   ├── pipeline.rs     # Bind group layouts, uniform types
│   ├── pipelines.rs    # Effect-specific pipeline creation
│   ├── extract.rs      # Main world -> render world extraction
│   ├── prepare.rs      # GPU buffer/bind group creation
│   └── shaders/        # Embedded WGSL shaders
├── distortion/         # Shockwave, RadialBlur, WaterDrops, HeatHaze
├── glitch/             # RgbSplit, ScanlineGlitch, BlockDisplacement, StaticNoise
└── feedback/           # DamageVignette, ScreenFlash, SpeedLines
```

### Feature Flags

- `distortion` - Shockwave, radial blur, water drops, heat haze
- `glitch` - RGB split, scanlines, block displacement, static noise
- `feedback` - Damage vignette, screen flash, speed lines

All enabled by default. Users can disable unused categories to reduce compile time.

### Render Pipeline

**Data Flow:**
1. **Extract** (`extract.rs`) - Copy effect data from main world to render world each frame
2. **Prepare** (`prepare.rs`) - Create GPU buffers and bind groups from extracted data
3. **Queue** (`pipelines.rs`) - Compile render pipelines (cached after first use)
4. **Render** (`node.rs`) - Apply effects via fullscreen passes

**Render Graph Position:**
Effects run after `Node3d::Tonemapping`, before `Node3d::EndMainPassPostProcessing`. This means HDR is already resolved but UI hasn't been composited yet.

**Effect Application Order:**
1. Distortion effects (shockwave, radial blur) - applied first as they sample nearby pixels
2. Glitch effects (RGB split, scanlines, etc.) - applied to distorted image
3. Feedback effects (vignette, flash) - applied last as overlays

**Ping-Pong Rendering:**
Uses `ViewTarget::post_process_write()` which automatically handles double-buffering. Each effect reads from `source` and writes to `destination`, then swaps for the next effect.

### Adding a New Effect

1. Create component in appropriate category module (e.g., `distortion/my_effect.rs`)
2. Implement `ExtractComponent` derive for GPU extraction
3. Create bundle with `ScreenEffect`, `EffectIntensity`, `EffectLifetime`
4. Add extraction logic in `render/extract.rs`
5. Add uniform struct in `render/pipeline.rs`
6. Add prepare logic in `render/prepare.rs`
7. Add pipeline creation in `render/pipelines.rs`
8. Add render pass in `render/node.rs`
9. Add shader to `src/render/shaders/`
10. Register in category's plugin and re-export from `prelude`

### Shader Conventions

- Shaders are embedded via `embedded_asset!` macro
- Import `bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput`
- Group 0: screen texture + sampler (shared layout)
- Group 1: effect-specific uniforms
- Use normalized screen coords (0.0-1.0)
- Intensity should scale effect strength for smooth fade in/out
- Draw with `draw(0..3, 0..1)` - fullscreen triangle

### Uniform Alignment

All uniform structs must:
- Derive `ShaderType` and `bytemuck::Pod`/`Zeroable`
- Use `#[repr(C)]`
- Pad to 16-byte alignment (add `_padding` fields as needed)
- Match WGSL struct layout exactly
