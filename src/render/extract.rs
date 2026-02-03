//! Extraction of effect data from the main world to the render world.

use bevy::prelude::*;
use bevy::render::Extract;

use crate::effect::{EffectIntensity, ScreenEffect};
use crate::lifetime::EffectLifetime;

#[cfg(feature = "distortion")]
use crate::distortion::{HeatHaze, RadialBlur, Raindrops, Shockwave};

#[cfg(feature = "glitch")]
use crate::glitch::{BlockDisplacement, EmpInterference, RgbSplit, ScanlineGlitch, StaticNoise};

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

    #[cfg(feature = "distortion")] radial_blurs: Extract<
        Query<(&RadialBlur, &EffectIntensity), With<ScreenEffect>>,
    >,

    #[cfg(feature = "distortion")] raindrops: Extract<
        Query<(&Raindrops, &EffectIntensity), With<ScreenEffect>>,
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
    extracted.rgb_splits.clear();
    extracted.glitches.clear();
    extracted.emp_interferences.clear();
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
