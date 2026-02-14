//! Extraction of effect data from the main world to the render world.

use bevy::prelude::*;
use bevy::render::Extract;

use crate::effect::{EffectIntensity, ScreenEffect};
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

/// Resource holding all extracted effects for the current frame.
#[derive(Resource, Default)]
pub struct ExtractedEffects {
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
    pub time: f32,
    pub delta_time: f32,
}

impl ExtractedEffects {
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
}

/// System that extracts all effect data to the render world.
#[allow(clippy::too_many_arguments)]
pub fn extract_effects(
    mut extracted: ResMut<ExtractedEffects>,
    time: Extract<Res<Time>>,

    #[cfg(feature = "distortion")] shockwaves: Extract<
        Query<(&Shockwave, &EffectIntensity, &EffectLifetime), With<ScreenEffect>>,
    >,

    #[cfg(feature = "distortion")] world_shockwaves: Extract<
        Query<(&WorldShockwave, &EffectIntensity, &EffectLifetime), With<ScreenEffect>>,
    >,

    #[cfg(feature = "distortion")] cameras: Extract<
        Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    >,

    #[cfg(feature = "distortion")] radial_blurs: Extract<
        Query<(&RadialBlur, &EffectIntensity), With<ScreenEffect>>,
    >,

    #[cfg(feature = "distortion")] raindrops: Extract<
        Query<(&Raindrops, &EffectIntensity), With<ScreenEffect>>,
    >,

    #[cfg(feature = "distortion")] world_heat_shimmers: Extract<
        Query<(&WorldHeatShimmer, &EffectIntensity), With<ScreenEffect>>,
    >,

    #[cfg(feature = "glitch")] rgb_splits: Extract<
        Query<(&RgbSplit, &EffectIntensity), With<ScreenEffect>>,
    >,

    #[cfg(feature = "glitch")] scanlines: Extract<
        Query<(&ScanlineGlitch, &EffectIntensity), With<ScreenEffect>>,
    >,

    #[cfg(feature = "glitch")] blocks: Extract<
        Query<(&BlockDisplacement, &EffectIntensity), With<ScreenEffect>>,
    >,

    #[cfg(feature = "glitch")] statics: Extract<
        Query<(&StaticNoise, &EffectIntensity), With<ScreenEffect>>,
    >,

    #[cfg(feature = "glitch")] emps: Extract<
        Query<(&EmpInterference, &EffectIntensity), With<ScreenEffect>>,
    >,

    #[cfg(feature = "glitch")] crts: Extract<
        Query<(&CrtEffect, &EffectIntensity), With<ScreenEffect>>,
    >,

    #[cfg(feature = "feedback")] vignettes: Extract<
        Query<(&DamageVignette, &EffectIntensity), With<ScreenEffect>>,
    >,

    #[cfg(feature = "feedback")] flashes: Extract<
        Query<(&ScreenFlash, &EffectIntensity), With<ScreenEffect>>,
    >,
) {
    // Clear previous frame's data
    extracted.shockwaves.clear();
    extracted.radial_blurs.clear();
    extracted.raindrops.clear();
    extracted.world_heat_shimmers.clear();
    extracted.rgb_splits.clear();
    extracted.glitches.clear();
    extracted.emp_interferences.clear();
    extracted.crts.clear();
    extracted.damage_vignettes.clear();
    extracted.screen_flashes.clear();

    extracted.time = time.elapsed_secs();
    extracted.delta_time = time.delta_secs();


    // Extract shockwaves
    #[cfg(feature = "distortion")]
    for (shockwave, intensity, lifetime) in shockwaves.iter() {
        if intensity.get() > 0.001 {
            extracted.shockwaves.push(ExtractedShockwave {
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
        for (shockwave, intensity, lifetime) in world_shockwaves.iter() {
            if intensity.get() > 0.001 {
                let center_ndc = camera.world_to_ndc(cam_transform, shockwave.world_pos);
                if let Some(ndc) = center_ndc {
                    // Convert NDC to screen coords (y=0 at top, y=1 at bottom)
                    let screen_pos = Vec2::new(ndc.x * 0.5 + 0.5, -ndc.y * 0.5 + 0.5);

                    // Project a point offset by max_radius to get screen-space radius
                    // Use camera's right vector for the offset
                    let cam_right = cam_transform.right();
                    let offset_pos = shockwave.world_pos + cam_right * shockwave.max_radius;
                    let screen_radius = if let Some(offset_ndc) =
                        camera.world_to_ndc(cam_transform, offset_pos)
                    {
                        let offset_screen =
                            Vec2::new(offset_ndc.x * 0.5 + 0.5, -offset_ndc.y * 0.5 + 0.5);
                        (offset_screen - screen_pos).length()
                    } else {
                        shockwave.max_radius // Fallback if offset is off-screen
                    };

                    // Scale ring width proportionally
                    let scale = screen_radius / shockwave.max_radius;

                    extracted.shockwaves.push(ExtractedShockwave {
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
    for (blur, intensity) in radial_blurs.iter() {
        if intensity.get() > 0.001 {
            extracted.radial_blurs.push(ExtractedRadialBlur {
                center: blur.center,
                intensity: blur.intensity * intensity.get(),
                samples: blur.samples,
            });
        }
    }

    // Extract raindrops
    #[cfg(feature = "distortion")]
    for (rain, intensity) in raindrops.iter() {
        if intensity.get() > 0.001 {
            extracted.raindrops.push(ExtractedRaindrops {
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
        for (shimmer, intensity) in world_heat_shimmers.iter() {
            if intensity.get() > 0.001 {
                // Project column corners to screen space
                let base = shimmer.world_pos;
                let top = base + Vec3::Y * shimmer.height;
                let half_width = shimmer.width / 2.0;

                // Use camera's right vector for width offset
                let cam_right = cam_transform.right();

                // Project 4 corners: base-left, base-right, top-left, top-right
                let corners = [
                    base - cam_right * half_width,
                    base + cam_right * half_width,
                    top - cam_right * half_width,
                    top + cam_right * half_width,
                ];

                // Find screen-space bounding box
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

                // Only add if at least some corners are visible
                if valid_corners >= 2 {
                    // bounds = (left, right, top, bottom)
                    let bounds = Vec4::new(min_x, max_x, min_y, max_y);

                    extracted.world_heat_shimmers.push(ExtractedWorldHeatShimmer {
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
    for (split, intensity) in rgb_splits.iter() {
        if intensity.get() > 0.001 {
            extracted.rgb_splits.push(ExtractedRgbSplit {
                red_offset: split.red_offset,
                green_offset: split.green_offset,
                blue_offset: split.blue_offset,
                intensity: intensity.get(),
            });
        }
    }

    // Combine glitch effects into single passes where possible
    #[cfg(feature = "glitch")]
    {
        let mut total_scanline_intensity = 0.0;
        let mut total_scanline_density = 0.0;

        for (scanline, intensity) in scanlines.iter() {
            if intensity.get() > 0.001 {
                total_scanline_intensity += intensity.get();
                total_scanline_density = scanline.density; // Use last one's density
            }
        }

        let mut total_block_intensity = 0.0;
        let mut block_size = Vec2::new(0.1, 0.05);

        for (block, intensity) in blocks.iter() {
            if intensity.get() > 0.001 {
                total_block_intensity += intensity.get();
                block_size = block.block_size;
            }
        }

        let mut total_noise_intensity = 0.0;
        for (_, intensity) in statics.iter() {
            if intensity.get() > 0.001 {
                total_noise_intensity += intensity.get();
            }
        }

        // If any glitch effects are active, create combined glitch entry
        if total_scanline_intensity > 0.0
            || total_block_intensity > 0.0
            || total_noise_intensity > 0.0
        {
            extracted.glitches.push(ExtractedGlitch {
                intensity: (total_scanline_intensity + total_block_intensity + total_noise_intensity)
                    .min(1.0),
                rgb_split_amount: 0.0, // Handled separately
                scanline_density: if total_scanline_intensity > 0.0 {
                    total_scanline_density
                } else {
                    0.0
                },
                block_size: if total_block_intensity > 0.0 {
                    block_size
                } else {
                    Vec2::ZERO
                },
                noise_amount: total_noise_intensity.min(1.0),
            });
        }
    }

    // Extract EMP interference effects
    #[cfg(feature = "glitch")]
    for (emp, intensity) in emps.iter() {
        if intensity.get() > 0.001 {
            extracted.emp_interferences.push(ExtractedEmpInterference {
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
    for (crt, intensity) in crts.iter() {
        if intensity.get() > 0.001 {
            extracted.crts.push(ExtractedCrt {
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
    for (vignette, intensity) in vignettes.iter() {
        if intensity.get() > 0.001 {
            extracted.damage_vignettes.push(ExtractedDamageVignette {
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
    for (flash, intensity) in flashes.iter() {
        if intensity.get() > 0.001 {
            extracted.screen_flashes.push(ExtractedScreenFlash {
                color: flash.color.into(),
                blend: flash.blend,
                intensity: intensity.get(),
            });
        }
    }
}
