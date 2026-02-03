# bevy_screen_effects

Dynamic, event-driven screen space effects for Bevy games.

Unlike static post-processing, effects in this library are spawned as entities with lifetimes—they animate and despawn automatically, perfect for impacts, abilities, damage feedback, and environmental effects.

**Bevy 0.18** | **Rust 2024 Edition**

## Features

- **Entity-based effects** - Spawn effects as ECS entities, not permanent pipeline passes
- **Automatic lifetime management** - Effects fade in/out and despawn on their own
- **Configurable easing** - Linear, ease in/out, elastic, bounce animations
- **12 built-in effects** across three categories
- **Modular feature flags** - Only compile what you need

## Quick Start

```rust
use bevy::prelude::*;
use bevy_screen_effects::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ScreenEffectsPlugin)
        .add_systems(Update, spawn_effects)
        .run();
}

fn spawn_effects(mut commands: Commands, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        // Spawn a shockwave at screen center - it animates and despawns automatically
        commands.spawn(ShockwaveBundle {
            shockwave: Shockwave::at(0.5, 0.5).with_intensity(0.3),
            lifetime: EffectLifetime::new(0.5),
            ..default()
        });
    }
}
```

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
bevy_screen_effects = "0.1"
```

### Feature Flags

All features are enabled by default. Disable unused categories to reduce compile time:

```toml
[dependencies]
bevy_screen_effects = { version = "0.1", default-features = false, features = ["distortion", "feedback"] }
```

| Feature | Effects |
|---------|---------|
| `distortion` | Shockwave, Radial Blur, Raindrops, Heat Haze |
| `glitch` | RGB Split, Scanline Glitch, Block Displacement, Static Noise, EMP |
| `feedback` | Damage Vignette, Screen Flash, Speed Lines |

## Effects

### Distortion Effects

#### Shockwave

Expanding ring of distortion from a point. Perfect for explosions and impacts.

```rust
commands.spawn(ShockwaveBundle {
    shockwave: Shockwave::at(0.5, 0.5)
        .with_intensity(0.25)
        .with_ring_width(0.1)
        .with_chromatic(true),
    lifetime: EffectLifetime::new(0.5),
    ..default()
});
```

#### Radial Blur

Motion blur radiating from a center point.

```rust
commands.spawn(RadialBlurBundle {
    radial_blur: RadialBlur {
        center: Vec2::new(0.5, 0.5),
        intensity: 0.1,
        samples: 8,
    },
    lifetime: EffectLifetime::new(0.3),
    ..default()
});
```

#### Raindrops

Procedural raindrops with refraction. Includes presets for different intensities.

```rust
// Use a preset
commands.spawn(RaindropsBundle {
    raindrops: Raindrops::storm(),
    lifetime: EffectLifetime::new(5.0),
    ..default()
});

// Or customize
commands.spawn(RaindropsBundle {
    raindrops: Raindrops::default()
        .with_density(0.7)
        .with_drop_size(0.04)
        .with_speed(0.5),
    lifetime: EffectLifetime::new(10.0),
    ..default()
});
```

**Presets:** `light()`, `heavy()`, `storm()`, `drizzle()`

#### Heat Haze

Wavy distortion for heat shimmer, underwater, or dream sequences.

```rust
commands.spawn(HeatHazeBundle {
    heat_haze: HeatHaze {
        amplitude: 0.01,
        frequency: 20.0,
        speed: 2.0,
        direction: Vec2::Y,
    },
    lifetime: EffectLifetime::new(3.0),
    ..default()
});
```

### Glitch Effects

#### RGB Split

Chromatic aberration / channel separation.

```rust
commands.spawn(RgbSplitBundle {
    rgb_split: RgbSplit::horizontal(0.015),
    lifetime: EffectLifetime::new(0.2),
    ..default()
});
```

**Presets:** `horizontal(amount)`, `diagonal(amount)`

#### Scanline Glitch

Horizontal line artifacts with displacement.

```rust
commands.spawn(ScanlineGlitchBundle {
    scanline: ScanlineGlitch {
        density: 0.1,
        displacement: 0.05,
        line_height: 2.0,
        flicker_speed: 30.0,
    },
    lifetime: EffectLifetime::new(0.5),
    ..default()
});
```

#### Block Displacement

Datamosh / video compression artifacts.

```rust
commands.spawn(BlockDisplacementBundle {
    block: BlockDisplacement {
        block_size: Vec2::new(0.1, 0.05),
        max_displacement: 0.1,
        probability: 0.3,
        update_rate: 15.0,
    },
    lifetime: EffectLifetime::new(0.4),
    ..default()
});
```

#### Static Noise

Visual grain and interference.

```rust
commands.spawn(StaticNoiseBundle {
    noise: StaticNoise {
        grain_size: 1.0,
        color_amount: 0.0,  // 0 = mono, 1 = color
        blend_mode: 0.3,    // 0 = additive, 1 = replace
    },
    lifetime: EffectLifetime::new(0.3),
    ..default()
});
```

#### EMP Interference

Complex electromagnetic pulse with multiple layered sub-effects.

```rust
// Use a preset
commands.spawn(EmpInterferenceBundle {
    emp: EmpInterference::heavy(),
    lifetime: EffectLifetime::new(1.0),
    ..default()
});

// Or customize
commands.spawn(EmpInterferenceBundle {
    emp: EmpInterference::default()
        .with_flicker(30.0, 0.3)
        .with_bands(8.0, 0.4, 2.0)
        .with_static(0.2, 0.1)
        .with_chromatic(0.01),
    lifetime: EffectLifetime::new(1.5),
    ..default()
});
```

**Presets:** `light()`, `heavy()`, `critical()`, `radio_static()`

### Feedback Effects

#### Damage Vignette

Pulsing colored vignette at screen edges.

```rust
commands.spawn(DamageVignetteBundle {
    vignette: DamageVignette {
        color: Color::srgba(0.8, 0.0, 0.0, 0.6),
        size: 0.4,
        softness: 0.3,
        pulse_frequency: 8.0,
    },
    lifetime: EffectLifetime::new(0.8),
    ..default()
});
```

**Presets:** `with_color(color)`, `healing()` (green), `shield()` (blue, no pulse)

#### Screen Flash

Full-screen color flash for impacts and transitions.

```rust
commands.spawn(ScreenFlashBundle {
    flash: ScreenFlash::white(),
    lifetime: EffectLifetime::new(0.15).with_fade_out(0.15),
    ..default()
});
```

**Presets:** `white()`, `impact()`, `with_color(color)`

#### Speed Lines

Manga/anime-style radial motion lines.

```rust
commands.spawn(SpeedLinesBundle {
    speed_lines: SpeedLines::centered()
        .with_line_count(32)
        .with_thickness(0.002)
        .with_length(0.5),
    lifetime: EffectLifetime::new(0.5),
    ..default()
});
```

## Lifetime & Animation

Every effect uses `EffectLifetime` to control its duration and animation:

```rust
EffectLifetime::new(1.0)                    // 1 second duration
    .with_fade_in(0.1)                      // 0.1s fade in
    .with_fade_out(0.3)                     // 0.3s fade out
    .with_easing(EasingFunction::EaseOut)   // Easing curve
```

**Easing Functions:**
- `Linear` - Constant rate
- `EaseIn` - Slow start, fast end
- `EaseOut` - Fast start, slow end
- `EaseInOut` - Slow start and end
- `Elastic` - Overshoot then settle
- `Bounce` - Bounces at the end

## Combining Effects

Spawn multiple effects simultaneously for complex visuals:

```rust
fn big_impact(commands: &mut Commands, position: Vec2) {
    // Shockwave
    commands.spawn(ShockwaveBundle {
        shockwave: Shockwave::at(position.x, position.y).with_intensity(0.4),
        lifetime: EffectLifetime::new(0.6),
        ..default()
    });

    // Screen flash
    commands.spawn(ScreenFlashBundle {
        flash: ScreenFlash::impact(),
        lifetime: EffectLifetime::new(0.1),
        ..default()
    });

    // Brief glitch
    commands.spawn(RgbSplitBundle {
        rgb_split: RgbSplit::horizontal(0.02),
        lifetime: EffectLifetime::new(0.15),
        ..default()
    });
}
```

## Running the Example

```bash
cargo run --example showcase
```

**Controls:**
- **1** or **Left Click** - Shockwave at cursor
- **2** - Radial blur
- **3** - RGB split
- **4** - Combined glitch (scanline + block + static)
- **5** - Damage vignette
- **6** - Screen flash
- **7** - Raindrops
- **8** - EMP interference
- **Space** - Shockwave at center

## License

MIT OR Apache-2.0
