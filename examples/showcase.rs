//! Showcase example demonstrating all screen effects.
//!
//! Controls:
//! - 1: Spawn shockwave at cursor
//! - 2: Spawn radial blur
//! - 3: Toggle RGB split
//! - 4: Toggle glitch effects
//! - 5: Trigger damage vignette
//! - 6: Trigger screen flash
//! - 7: Toggle raindrops
//! - 8: EMP interference
//! - 9: World-space shockwave (at center sphere)
//! - 0: World heat shimmer (at cube)
//! - C: CRT effect (retro gaming)
//! - Space: Spawn shockwave at center

use bevy::prelude::*;
use bevy_screen_effects::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ScreenEffectsPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_input, update_info_text))
        .run();
}

#[derive(Component)]
struct InfoText;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Light
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, 0.5, 0.0)),
    ));

    // Ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));

    // Some cubes to see effects against
    for x in -2..=2 {
        for z in -2..=2 {
            if x == 0 && z == 0 {
                continue;
            }
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
                MeshMaterial3d(materials.add(Color::srgb(
                    0.5 + x as f32 * 0.1,
                    0.3,
                    0.5 + z as f32 * 0.1,
                ))),
                Transform::from_xyz(x as f32 * 1.5, 0.25, z as f32 * 1.5),
            ));
        }
    }

    // Center sphere
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.2, 0.2))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // Info text
    commands.spawn((
        Text::new("Screen Effects Showcase\n\n\
            1 - Shockwave (at cursor)\n\
            2 - Radial Blur\n\
            3 - RGB Split\n\
            4 - Glitch\n\
            5 - Damage Vignette\n\
            6 - Screen Flash\n\
            7 - Raindrops\n\
            8 - EMP Interference\n\
            9 - World Shockwave (at sphere)\n\
            0 - Heat Shimmer (at cube)\n\
            C - CRT Effect\n\
            Space - Shockwave (center)"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        InfoText,
    ));
}

fn handle_input(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    // Get normalized cursor position (y=0 at top in both window and shader coords)
    let cursor_pos = window
        .cursor_position()
        .map(|pos| Vec2::new(pos.x / window.width(), pos.y / window.height()))
        .unwrap_or(Vec2::new(0.5, 0.5));

    // 1 or Left Click: Shockwave at cursor
    if input.just_pressed(KeyCode::Digit1) || mouse.just_pressed(MouseButton::Left) {
        commands.spawn(ShockwaveBundle {
            shockwave: Shockwave::at(cursor_pos.x, cursor_pos.y)
                .with_intensity(0.3)
                .with_chromatic(true),
            lifetime: EffectLifetime::new(0.6).with_easing(EasingFunction::EaseOut),
            ..default()
        });
    }

    // Space: Shockwave at center
    if input.just_pressed(KeyCode::Space) {
        commands.spawn(ShockwaveBundle {
            shockwave: Shockwave::at(0.5, 0.5).with_intensity(0.4).with_max_radius(1.0),
            lifetime: EffectLifetime::new(0.8),
            ..default()
        });
    }

    // 2: Radial blur
    if input.just_pressed(KeyCode::Digit2) {
        commands.spawn(RadialBlurBundle {
            radial_blur: RadialBlur {
                center: cursor_pos,
                intensity: 0.15,
                samples: 12,
            },
            lifetime: EffectLifetime::new(0.4).with_fades(0.05, 0.3),
            ..default()
        });
    }

    // 3: RGB split
    if input.just_pressed(KeyCode::Digit3) {
        commands.spawn(RgbSplitBundle {
            rgb_split: RgbSplit::horizontal(0.015),
            lifetime: EffectLifetime::new(0.5).with_fades(0.05, 0.4),
            ..default()
        });
    }

    // 4: Glitch effects
    if input.just_pressed(KeyCode::Digit4) {
        // Spawn multiple glitch components for combined effect
        commands.spawn(ScanlineGlitchBundle {
            scanline: ScanlineGlitch {
                density: 0.15,
                displacement: 0.08,
                ..default()
            },
            lifetime: EffectLifetime::new(0.3),
            ..default()
        });

        commands.spawn(BlockDisplacementBundle {
            block_displacement: BlockDisplacement {
                probability: 0.4,
                ..default()
            },
            lifetime: EffectLifetime::new(0.25),
            ..default()
        });

        commands.spawn(StaticNoiseBundle {
            static_noise: StaticNoise {
                grain_size: 1.0,
                color_amount: 0.0,
                blend_mode: 0.3,
            },
            lifetime: EffectLifetime::new(0.2),
            ..default()
        });
    }

    // 5: Damage vignette
    if input.just_pressed(KeyCode::Digit5) {
        commands.spawn(DamageVignetteBundle {
            vignette: DamageVignette::default(),
            lifetime: EffectLifetime::new(0.8).with_fades(0.05, 0.6),
            ..default()
        });
    }

    // 6: Screen flash
    if input.just_pressed(KeyCode::Digit6) {
        commands.spawn(ScreenFlashBundle {
            flash: ScreenFlash::impact(),
            lifetime: EffectLifetime::new(0.15).with_fades(0.0, 0.15),
            ..default()
        });
    }

    // 7: Raindrops (longer duration to see the effect)
    if input.just_pressed(KeyCode::Digit7) {
        commands.spawn(RaindropsBundle {
            raindrops: Raindrops::default(),
            lifetime: EffectLifetime::new(5.0).with_fades(0.3, 1.0),
            ..default()
        });
    }

    // 8: EMP Interference
    if input.just_pressed(KeyCode::Digit8) {
        commands.spawn(EmpInterferenceBundle {
            emp: EmpInterference::default(),
            lifetime: EffectLifetime::new(2.0).with_fades(0.1, 0.5),
            ..default()
        });
    }

    // 9: World-space shockwave (at the center sphere position)
    if input.just_pressed(KeyCode::Digit9) {
        commands.spawn(WorldShockwaveBundle {
            shockwave: WorldShockwave::at(Vec3::new(0.0, 0.5, 0.0))
                .with_intensity(0.4)
                .with_chromatic(true),
            lifetime: EffectLifetime::new(0.8),
            ..default()
        });
    }

    // C: CRT effect
    if input.just_pressed(KeyCode::KeyC) {
        commands.spawn(CrtEffectBundle {
            crt: CrtEffect::arcade(),
            lifetime: EffectLifetime::new(5.0).with_fades(0.3, 1.0),
            ..default()
        });
    }

    // 0: World heat shimmer (at a cube position)
    if input.just_pressed(KeyCode::Digit0) {
        commands.spawn(WorldHeatShimmerBundle {
            shimmer: WorldHeatShimmer::at(Vec3::new(1.5, 0.0, 0.0))
                .with_size(0.8, 1.5)
                .with_amplitude(0.006)
                .with_frequency(50.0)
                .with_speed(0.8),
            lifetime: EffectLifetime::new(5.0).with_fades(0.3, 1.0),
            ..default()
        });
    }
}

fn update_info_text(
    effects: Query<(), With<ScreenEffect>>,
    mut text: Query<&mut Text, With<InfoText>>,
) {
    let count = effects.iter().count();
    if let Ok(mut text) = text.single_mut() {
        // Update the last line to show active effect count
        let base = "Screen Effects Showcase\n\n\
            1 - Shockwave (at cursor)\n\
            2 - Radial Blur\n\
            3 - RGB Split\n\
            4 - Glitch\n\
            5 - Damage Vignette\n\
            6 - Screen Flash\n\
            7 - Raindrops\n\
            8 - EMP Interference\n\
            9 - World Shockwave (at sphere)\n\
            0 - Heat Shimmer (at cube)\n\
            Space - Shockwave (center)\n\n";

        **text = format!("{}Active effects: {}", base, count);
    }
}
