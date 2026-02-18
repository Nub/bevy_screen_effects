//! Extraction of effect data from the main world to the render world.

use bevy::prelude::*;
use bevy::render::Extract;

use std::collections::HashMap;

use crate::effect::{EffectIntensity, EffectTarget, ScreenEffect};
use crate::lifetime::EffectLifetime;

#[cfg(feature = "distortion")]
use crate::distortion::{HeatHaze, RadialBlur, Raindrops, Shockwave, WorldHeatShimmer, WorldShockwave};

#[cfg(feature = "glitch")]
use crate::glitch::{BlockDisplacement, CrtEffect, EmpInterference, RgbSplit, ScanlineGlitch, StaticNoise};

#[cfg(feature = "feedback")]
use crate::feedback::{DamageVignette, ScreenFlash, SpeedLines};

/// Extracted shockwave effect data for the render world.
#[derive(Component, Clone)]
pub struct ExtractedShockwave {
    pub center: Vec2,
    pub intensity: f32,
    pub progress: f32,
    pub ring_width: f32,
    pub max_radius: f32,
    pub chromatic: bool,
}

/// Extracted radial blur effect data.
#[derive(Component, Clone)]
pub struct ExtractedRadialBlur {
    pub center: Vec2,
    pub intensity: f32,
    pub samples: u32,
}

/// Extracted RGB split effect data.
#[derive(Component, Clone)]
pub struct ExtractedRgbSplit {
    pub red_offset: Vec2,
    pub green_offset: Vec2,
    pub blue_offset: Vec2,
    pub intensity: f32,
}

/// Extracted glitch effect data (combined for efficiency).
#[derive(Component, Clone)]
pub struct ExtractedGlitch {
    pub intensity: f32,
    pub rgb_split_amount: f32,
    pub scanline_density: f32,
    pub block_size: Vec2,
    pub noise_amount: f32,
}

/// Extracted damage vignette effect data.
#[derive(Component, Clone)]
pub struct ExtractedDamageVignette {
    pub color: LinearRgba,
    pub size: f32,
    pub softness: f32,
    pub pulse_frequency: f32,
    pub intensity: f32,
}

/// Extracted screen flash effect data.
#[derive(Component, Clone)]
pub struct ExtractedScreenFlash {
    pub color: LinearRgba,
    pub blend: f32,
    pub intensity: f32,
}

/// Extracted raindrops effect data.
#[derive(Component, Clone)]
pub struct ExtractedRaindrops {
    pub drop_size: f32,
    pub density: f32,
    pub speed: f32,
    pub refraction: f32,
    pub trail_strength: f32,
    pub intensity: f32,
}

/// Extracted EMP interference effect data.
#[derive(Component, Clone)]
pub struct ExtractedEmpInterference {
    pub flicker_rate: f32,
    pub flicker_strength: f32,
    pub band_count: f32,
    pub band_intensity: f32,
    pub band_speed: f32,
    pub static_intensity: f32,
    pub burst_probability: f32,
    pub scanline_displacement: f32,
    pub chromatic_amount: f32,
    pub intensity: f32,
}

/// Extracted world-space heat shimmer effect data.
#[derive(Component, Clone)]
pub struct ExtractedWorldHeatShimmer {
    /// Screen-space bounds (left, right, top, bottom) in UV coordinates.
    pub bounds: Vec4,
    pub amplitude: f32,
    pub frequency: f32,
    pub speed: f32,
    pub softness: f32,
    pub intensity: f32,
}

/// Extracted CRT effect data.
#[derive(Component, Clone)]
pub struct ExtractedCrt {
    pub scanline_intensity: f32,
    pub scanline_count: f32,
    pub curvature: f32,
    pub corner_radius: f32,
    pub mask_shape: u32,
    pub phosphor_type: u32,
    pub phosphor_intensity: f32,
    pub bloom: f32,
    pub vignette: f32,
    pub flicker: f32,
    pub color_bleed: f32,
    pub brightness: f32,
    pub saturation: f32,
    pub intensity: f32,
}

/// Per-camera bucket of extracted effects.
#[derive(Default, Clone)]
pub struct EffectBucket {
    pub shockwaves: Vec<ExtractedShockwave>,
    pub radial_blurs: Vec<ExtractedRadialBlur>,
    pub rgb_splits: Vec<ExtractedRgbSplit>,
    pub glitches: Vec<ExtractedGlitch>,
    pub emp_interferences: Vec<ExtractedEmpInterference>,
    pub damage_vignettes: Vec<ExtractedDamageVignette>,
    pub screen_flashes: Vec<ExtractedScreenFlash>,
    pub raindrops: Vec<ExtractedRaindrops>,
    pub world_heat_shimmers: Vec<ExtractedWorldHeatShimmer>,
    pub crts: Vec<ExtractedCrt>,
}

impl EffectBucket {
    pub fn has_any(&self) -> bool {
        !self.shockwaves.is_empty()
            || !self.radial_blurs.is_empty()
            || !self.rgb_splits.is_empty()
            || !self.glitches.is_empty()
            || !self.emp_interferences.is_empty()
            || !self.damage_vignettes.is_empty()
            || !self.screen_flashes.is_empty()
            || !self.raindrops.is_empty()
            || !self.world_heat_shimmers.is_empty()
            || !self.crts.is_empty()
    }

    fn clear(&mut self) {
        self.shockwaves.clear();
        self.radial_blurs.clear();
        self.rgb_splits.clear();
        self.glitches.clear();
        self.emp_interferences.clear();
        self.damage_vignettes.clear();
        self.screen_flashes.clear();
        self.raindrops.clear();
        self.world_heat_shimmers.clear();
        self.crts.clear();
    }
}

/// Resource holding all extracted effects for the current frame, keyed by camera.
///
/// `None` key = effects that apply to all cameras (no `EffectTarget`).
/// `Some(entity)` key = effects targeted at a specific camera.
#[derive(Resource, Default)]
pub struct ExtractedEffects {
    pub buckets: HashMap<Option<Entity>, EffectBucket>,
    pub time: f32,
    pub delta_time: f32,
}

impl ExtractedEffects {
    pub fn bucket_mut(&mut self, target: Option<Entity>) -> &mut EffectBucket {
        self.buckets.entry(target).or_default()
    }

    pub fn clear_all(&mut self) {
        for bucket in self.buckets.values_mut() {
            bucket.clear();
        }
    }
}

/// System that extracts all effect data to the render world.
#[allow(clippy::too_many_arguments)]
pub fn extract_effects(
    mut extracted: ResMut<ExtractedEffects>,
    time: Extract<Res<Time>>,

    #[cfg(feature = "distortion")] shockwaves: Extract<
        Query<(&Shockwave, &EffectIntensity, &EffectLifetime, Option<&EffectTarget>), With<ScreenEffect>>,
    >,

    #[cfg(feature = "distortion")] world_shockwaves: Extract<
        Query<(&WorldShockwave, &EffectIntensity, &EffectLifetime, Option<&EffectTarget>), With<ScreenEffect>>,
    >,

    #[cfg(feature = "distortion")] cameras: Extract<
        Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    >,

    #[cfg(feature = "distortion")] radial_blurs: Extract<
        Query<(&RadialBlur, &EffectIntensity, Option<&EffectTarget>), With<ScreenEffect>>,
    >,

    #[cfg(feature = "distortion")] raindrops: Extract<
        Query<(&Raindrops, &EffectIntensity, Option<&EffectTarget>), With<ScreenEffect>>,
    >,

    #[cfg(feature = "distortion")] world_heat_shimmers: Extract<
        Query<(&WorldHeatShimmer, &EffectIntensity, Option<&EffectTarget>), With<ScreenEffect>>,
    >,

    #[cfg(feature = "glitch")] rgb_splits: Extract<
        Query<(&RgbSplit, &EffectIntensity, Option<&EffectTarget>), With<ScreenEffect>>,
    >,

    #[cfg(feature = "glitch")] scanlines: Extract<
        Query<(&ScanlineGlitch, &EffectIntensity, Option<&EffectTarget>), With<ScreenEffect>>,
    >,

    #[cfg(feature = "glitch")] blocks: Extract<
        Query<(&BlockDisplacement, &EffectIntensity, Option<&EffectTarget>), With<ScreenEffect>>,
    >,

    #[cfg(feature = "glitch")] statics: Extract<
        Query<(&StaticNoise, &EffectIntensity, Option<&EffectTarget>), With<ScreenEffect>>,
    >,

    #[cfg(feature = "glitch")] emps: Extract<
        Query<(&EmpInterference, &EffectIntensity, Option<&EffectTarget>), With<ScreenEffect>>,
    >,

    #[cfg(feature = "glitch")] crts: Extract<
        Query<(&CrtEffect, &EffectIntensity, Option<&EffectTarget>), With<ScreenEffect>>,
    >,

    #[cfg(feature = "feedback")] vignettes: Extract<
        Query<(&DamageVignette, &EffectIntensity, Option<&EffectTarget>), With<ScreenEffect>>,
    >,

    #[cfg(feature = "feedback")] flashes: Extract<
        Query<(&ScreenFlash, &EffectIntensity, Option<&EffectTarget>), With<ScreenEffect>>,
    >,
) {
    // Clear previous frame's data
    extracted.clear_all();

    extracted.time = time.elapsed_secs();
    extracted.delta_time = time.delta_secs();


    // Helper to get the target key from an optional EffectTarget
    let target_key = |t: Option<&EffectTarget>| -> Option<Entity> { t.map(|et| et.0) };

    // Extract shockwaves
    #[cfg(feature = "distortion")]
    for (shockwave, intensity, lifetime, target) in shockwaves.iter() {
        if intensity.get() > 0.001 {
            extracted.bucket_mut(target_key(target)).shockwaves.push(ExtractedShockwave {
                center: shockwave.center,
                intensity: shockwave.intensity * intensity.get(),
                progress: lifetime.progress(),
                ring_width: shockwave.ring_width,
                max_radius: shockwave.max_radius,
                chromatic: shockwave.chromatic,
            });
        }
    }

    // Extract world-space shockwaves (project to screen space each frame)
    #[cfg(feature = "distortion")]
    if let Some((camera, cam_transform)) = cameras.iter().next() {
        for (shockwave, intensity, lifetime, target) in world_shockwaves.iter() {
            if intensity.get() > 0.001 {
                let center_ndc = camera.world_to_ndc(cam_transform, shockwave.world_pos);
                if let Some(ndc) = center_ndc {
                    let screen_pos = Vec2::new(ndc.x * 0.5 + 0.5, -ndc.y * 0.5 + 0.5);

                    let cam_right = cam_transform.right();
                    let offset_pos = shockwave.world_pos + cam_right * shockwave.max_radius;
                    let screen_radius = if let Some(offset_ndc) =
                        camera.world_to_ndc(cam_transform, offset_pos)
                    {
                        let offset_screen =
                            Vec2::new(offset_ndc.x * 0.5 + 0.5, -offset_ndc.y * 0.5 + 0.5);
                        (offset_screen - screen_pos).length()
                    } else {
                        shockwave.max_radius
                    };

                    let scale = screen_radius / shockwave.max_radius;

                    extracted.bucket_mut(target_key(target)).shockwaves.push(ExtractedShockwave {
                        center: screen_pos,
                        intensity: shockwave.intensity * intensity.get(),
                        progress: lifetime.progress(),
                        ring_width: shockwave.ring_width * scale,
                        max_radius: screen_radius,
                        chromatic: shockwave.chromatic,
                    });
                }
            }
        }
    }

    // Extract radial blurs
    #[cfg(feature = "distortion")]
    for (blur, intensity, target) in radial_blurs.iter() {
        if intensity.get() > 0.001 {
            extracted.bucket_mut(target_key(target)).radial_blurs.push(ExtractedRadialBlur {
                center: blur.center,
                intensity: blur.intensity * intensity.get(),
                samples: blur.samples,
            });
        }
    }

    // Extract raindrops
    #[cfg(feature = "distortion")]
    for (rain, intensity, target) in raindrops.iter() {
        if intensity.get() > 0.001 {
            extracted.bucket_mut(target_key(target)).raindrops.push(ExtractedRaindrops {
                drop_size: rain.drop_size,
                density: rain.density,
                speed: rain.speed,
                refraction: rain.refraction,
                trail_strength: rain.trail_strength,
                intensity: intensity.get(),
            });
        }
    }

    // Extract world-space heat shimmers (project column to screen space)
    #[cfg(feature = "distortion")]
    if let Some((camera, cam_transform)) = cameras.iter().next() {
        for (shimmer, intensity, target) in world_heat_shimmers.iter() {
            if intensity.get() > 0.001 {
                let base = shimmer.world_pos;
                let top = base + Vec3::Y * shimmer.height;
                let half_width = shimmer.width / 2.0;
                let cam_right = cam_transform.right();

                let corners = [
                    base - cam_right * half_width,
                    base + cam_right * half_width,
                    top - cam_right * half_width,
                    top + cam_right * half_width,
                ];

                let mut min_x = f32::MAX;
                let mut max_x = f32::MIN;
                let mut min_y = f32::MAX;
                let mut max_y = f32::MIN;
                let mut valid_corners = 0;

                for corner in corners {
                    if let Some(ndc) = camera.world_to_ndc(cam_transform, corner) {
                        let screen = Vec2::new(ndc.x * 0.5 + 0.5, -ndc.y * 0.5 + 0.5);
                        min_x = min_x.min(screen.x);
                        max_x = max_x.max(screen.x);
                        min_y = min_y.min(screen.y);
                        max_y = max_y.max(screen.y);
                        valid_corners += 1;
                    }
                }

                if valid_corners >= 2 {
                    let bounds = Vec4::new(min_x, max_x, min_y, max_y);
                    extracted.bucket_mut(target_key(target)).world_heat_shimmers.push(ExtractedWorldHeatShimmer {
                        bounds,
                        amplitude: shimmer.amplitude,
                        frequency: shimmer.frequency,
                        speed: shimmer.speed,
                        softness: shimmer.softness,
                        intensity: intensity.get(),
                    });
                }
            }
        }
    }

    // Extract RGB splits
    #[cfg(feature = "glitch")]
    for (split, intensity, target) in rgb_splits.iter() {
        if intensity.get() > 0.001 {
            extracted.bucket_mut(target_key(target)).rgb_splits.push(ExtractedRgbSplit {
                red_offset: split.red_offset,
                green_offset: split.green_offset,
                blue_offset: split.blue_offset,
                intensity: intensity.get(),
            });
        }
    }

    // Combine glitch effects into single passes where possible
    // Note: glitch sub-effects (scanlines, blocks, statics) are combined per-target
    #[cfg(feature = "glitch")]
    {
        // Collect per-target glitch data
        let mut glitch_data: HashMap<Option<Entity>, (f32, f32, f32, f32, Vec2)> = HashMap::new();

        for (scanline, intensity, target) in scanlines.iter() {
            if intensity.get() > 0.001 {
                let entry = glitch_data.entry(target_key(target)).or_insert((0.0, 0.0, 0.0, 0.0, Vec2::new(0.1, 0.05)));
                entry.0 += intensity.get();
                entry.1 = scanline.density;
            }
        }

        for (block, intensity, target) in blocks.iter() {
            if intensity.get() > 0.001 {
                let entry = glitch_data.entry(target_key(target)).or_insert((0.0, 0.0, 0.0, 0.0, Vec2::new(0.1, 0.05)));
                entry.2 += intensity.get();
                entry.4 = block.block_size;
            }
        }

        for (_, intensity, target) in statics.iter() {
            if intensity.get() > 0.001 {
                let entry = glitch_data.entry(target_key(target)).or_insert((0.0, 0.0, 0.0, 0.0, Vec2::new(0.1, 0.05)));
                entry.3 += intensity.get();
            }
        }

        for (target, (scanline_int, scanline_dens, block_int, noise_int, block_sz)) in glitch_data {
            if scanline_int > 0.0 || block_int > 0.0 || noise_int > 0.0 {
                extracted.bucket_mut(target).glitches.push(ExtractedGlitch {
                    intensity: (scanline_int + block_int + noise_int).min(1.0),
                    rgb_split_amount: 0.0,
                    scanline_density: if scanline_int > 0.0 { scanline_dens } else { 0.0 },
                    block_size: if block_int > 0.0 { block_sz } else { Vec2::ZERO },
                    noise_amount: noise_int.min(1.0),
                });
            }
        }
    }

    // Extract EMP interference effects
    #[cfg(feature = "glitch")]
    for (emp, intensity, target) in emps.iter() {
        if intensity.get() > 0.001 {
            extracted.bucket_mut(target_key(target)).emp_interferences.push(ExtractedEmpInterference {
                flicker_rate: emp.flicker_rate,
                flicker_strength: emp.flicker_strength,
                band_count: emp.band_count,
                band_intensity: emp.band_intensity,
                band_speed: emp.band_speed,
                static_intensity: emp.static_intensity,
                burst_probability: emp.burst_probability,
                scanline_displacement: emp.scanline_displacement,
                chromatic_amount: emp.chromatic_amount,
                intensity: intensity.get(),
            });
        }
    }

    // Extract CRT effects
    #[cfg(feature = "glitch")]
    for (crt, intensity, target) in crts.iter() {
        if intensity.get() > 0.001 {
            extracted.bucket_mut(target_key(target)).crts.push(ExtractedCrt {
                scanline_intensity: crt.scanline_intensity,
                scanline_count: crt.scanline_count,
                curvature: crt.curvature,
                corner_radius: crt.corner_radius,
                mask_shape: crt.mask_shape_u32(),
                phosphor_type: crt.phosphor_type_u32(),
                phosphor_intensity: crt.phosphor_intensity,
                bloom: crt.bloom,
                vignette: crt.vignette,
                flicker: crt.flicker,
                color_bleed: crt.color_bleed,
                brightness: crt.brightness,
                saturation: crt.saturation,
                intensity: intensity.get(),
            });
        }
    }

    // Extract damage vignettes
    #[cfg(feature = "feedback")]
    for (vignette, intensity, target) in vignettes.iter() {
        if intensity.get() > 0.001 {
            extracted.bucket_mut(target_key(target)).damage_vignettes.push(ExtractedDamageVignette {
                color: vignette.color.into(),
                size: vignette.size,
                softness: vignette.softness,
                pulse_frequency: vignette.pulse_frequency,
                intensity: intensity.get(),
            });
        }
    }

    // Extract screen flashes
    #[cfg(feature = "feedback")]
    for (flash, intensity, target) in flashes.iter() {
        if intensity.get() > 0.001 {
            extracted.bucket_mut(target_key(target)).screen_flashes.push(ExtractedScreenFlash {
                color: flash.color.into(),
                blend: flash.blend,
                intensity: intensity.get(),
            });
        }
    }
}
