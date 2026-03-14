//! Preparation of GPU resources from extracted effect data.

use std::collections::HashMap;

use bevy::prelude::*;
use bevy::render::{
    render_resource::*,
    renderer::{RenderDevice, RenderQueue},
};

use crate::layer::EffectLayer;

use super::extract::ExtractedEffects;
use super::pipeline::*;

/// A single prepared GPU instance of an effect, tagged with its layer mask.
pub struct PreparedEffectInstance {
    pub bind_group: BindGroup,
    pub effect_layer: u32,
}

/// Prepared GPU data for all active effects this frame.
#[derive(Resource, Default)]
pub struct PreparedEffects {
    pub shockwaves: Vec<PreparedEffectInstance>,
    pub radial_blurs: Vec<PreparedEffectInstance>,
    pub raindrops: Vec<PreparedEffectInstance>,
    pub rgb_splits: Vec<PreparedEffectInstance>,
    pub glitches: Vec<PreparedEffectInstance>,
    pub emps: Vec<PreparedEffectInstance>,
    pub vignettes: Vec<PreparedEffectInstance>,
    pub flashes: Vec<PreparedEffectInstance>,
    pub world_heat_shimmers: Vec<PreparedEffectInstance>,
    pub crts: Vec<PreparedEffectInstance>,
}

impl PreparedEffects {
    pub fn has_any(&self) -> bool {
        !self.shockwaves.is_empty()
            || !self.radial_blurs.is_empty()
            || !self.raindrops.is_empty()
            || !self.rgb_splits.is_empty()
            || !self.glitches.is_empty()
            || !self.emps.is_empty()
            || !self.vignettes.is_empty()
            || !self.flashes.is_empty()
            || !self.world_heat_shimmers.is_empty()
            || !self.crts.is_empty()
    }
}

/// Bind group layouts for effect uniforms.
#[derive(Resource)]
pub struct EffectBindGroupLayouts {
    pub shockwave: BindGroupLayout,
    pub shockwave_entries: Vec<BindGroupLayoutEntry>,
    pub radial_blur: BindGroupLayout,
    pub radial_blur_entries: Vec<BindGroupLayoutEntry>,
    pub raindrops: BindGroupLayout,
    pub raindrops_entries: Vec<BindGroupLayoutEntry>,
    pub rgb_split: BindGroupLayout,
    pub rgb_split_entries: Vec<BindGroupLayoutEntry>,
    pub glitch: BindGroupLayout,
    pub glitch_entries: Vec<BindGroupLayoutEntry>,
    pub emp: BindGroupLayout,
    pub emp_entries: Vec<BindGroupLayoutEntry>,
    pub vignette: BindGroupLayout,
    pub vignette_entries: Vec<BindGroupLayoutEntry>,
    pub flash: BindGroupLayout,
    pub flash_entries: Vec<BindGroupLayoutEntry>,
    pub world_heat_shimmer: BindGroupLayout,
    pub world_heat_shimmer_entries: Vec<BindGroupLayoutEntry>,
    pub crt: BindGroupLayout,
    pub crt_entries: Vec<BindGroupLayoutEntry>,
}

impl FromWorld for EffectBindGroupLayouts {
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>();

        let uniform_entry = BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::FRAGMENT,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        };

        let create_uniform_layout = |name: &str| -> (BindGroupLayout, Vec<BindGroupLayoutEntry>) {
            let entries = vec![uniform_entry.clone()];
            let layout = device.create_bind_group_layout(name, &entries);
            (layout, entries)
        };

        let (shockwave, shockwave_entries) = create_uniform_layout("shockwave_uniforms_layout");
        let (radial_blur, radial_blur_entries) = create_uniform_layout("radial_blur_uniforms_layout");
        let (raindrops, raindrops_entries) = create_uniform_layout("raindrops_uniforms_layout");
        let (rgb_split, rgb_split_entries) = create_uniform_layout("rgb_split_uniforms_layout");
        let (glitch, glitch_entries) = create_uniform_layout("glitch_uniforms_layout");
        let (emp, emp_entries) = create_uniform_layout("emp_uniforms_layout");
        let (vignette, vignette_entries) = create_uniform_layout("vignette_uniforms_layout");
        let (flash, flash_entries) = create_uniform_layout("flash_uniforms_layout");
        let (world_heat_shimmer, world_heat_shimmer_entries) = create_uniform_layout("world_heat_shimmer_uniforms_layout");
        let (crt, crt_entries) = create_uniform_layout("crt_uniforms_layout");

        Self {
            shockwave,
            shockwave_entries,
            radial_blur,
            radial_blur_entries,
            raindrops,
            raindrops_entries,
            rgb_split,
            rgb_split_entries,
            glitch,
            glitch_entries,
            emp,
            emp_entries,
            vignette,
            vignette_entries,
            flash,
            flash_entries,
            world_heat_shimmer,
            world_heat_shimmer_entries,
            crt,
            crt_entries,
        }
    }
}

/// Find the viewport size for a camera whose layer overlaps the given effect layer.
fn viewport_for_layer(
    cameras: &Query<(&bevy::render::camera::ExtractedCamera, Option<&EffectLayer>)>,
    effect_layer: u32,
) -> UVec2 {
    for (cam, cam_layer) in cameras.iter() {
        let cam_mask = cam_layer.map_or(u32::MAX, |l| l.0);
        if (cam_mask & effect_layer) != 0 {
            if let Some(size) = cam.physical_viewport_size {
                return size;
            }
        }
    }
    // Fallback
    UVec2::new(1920, 1080)
}

/// System that prepares GPU resources from extracted effects.
pub fn prepare_effects(
    device: Res<RenderDevice>,
    queue: Res<RenderQueue>,
    extracted: Res<ExtractedEffects>,
    layouts: Res<EffectBindGroupLayouts>,
    mut prepared: ResMut<PreparedEffects>,
    cameras: Query<(&bevy::render::camera::ExtractedCamera, Option<&EffectLayer>)>,
) {
    // Clear all vecs
    prepared.shockwaves.clear();
    prepared.radial_blurs.clear();
    prepared.raindrops.clear();
    prepared.rgb_splits.clear();
    prepared.glitches.clear();
    prepared.emps.clear();
    prepared.vignettes.clear();
    prepared.flashes.clear();
    prepared.world_heat_shimmers.clear();
    prepared.crts.clear();

    // Prepare shockwaves — one instance per unique layer
    {
        let mut seen: HashMap<u32, usize> = HashMap::new();
        for sw in &extracted.shockwaves {
            if seen.contains_key(&sw.effect_layer) {
                continue;
            }
            seen.insert(sw.effect_layer, prepared.shockwaves.len());

            let uniforms = ShockwaveUniforms {
                center: sw.center,
                intensity: sw.intensity,
                progress: sw.progress,
                ring_width: sw.ring_width,
                max_radius: sw.max_radius,
                chromatic: if sw.chromatic { 1 } else { 0 },
                _padding: 0.0,
            };

            let buffer = create_uniform_buffer(&device, &queue, &uniforms, "shockwave_uniforms");
            let bind_group = create_uniform_bind_group(&device, &layouts.shockwave, &buffer, "shockwave_bind_group");

            prepared.shockwaves.push(PreparedEffectInstance {
                bind_group,
                effect_layer: sw.effect_layer,
            });
        }
    }

    // Prepare radial blurs
    {
        let mut seen: HashMap<u32, usize> = HashMap::new();
        for blur in &extracted.radial_blurs {
            if seen.contains_key(&blur.effect_layer) {
                continue;
            }
            seen.insert(blur.effect_layer, prepared.radial_blurs.len());

            let uniforms = RadialBlurUniforms {
                center: blur.center,
                intensity: blur.intensity,
                samples: blur.samples,
            };

            let buffer = create_uniform_buffer(&device, &queue, &uniforms, "radial_blur_uniforms");
            let bind_group = create_uniform_bind_group(&device, &layouts.radial_blur, &buffer, "radial_blur_bind_group");

            prepared.radial_blurs.push(PreparedEffectInstance {
                bind_group,
                effect_layer: blur.effect_layer,
            });
        }
    }

    // Prepare raindrops
    {
        let mut seen: HashMap<u32, usize> = HashMap::new();
        for rain in &extracted.raindrops {
            if seen.contains_key(&rain.effect_layer) {
                continue;
            }
            seen.insert(rain.effect_layer, prepared.raindrops.len());

            let uniforms = RaindropsUniforms {
                time: extracted.time,
                intensity: rain.intensity,
                drop_size: rain.drop_size,
                density: rain.density,
                speed: rain.speed,
                refraction: rain.refraction,
                trail_strength: rain.trail_strength,
                _padding: 0.0,
            };

            let buffer = create_uniform_buffer(&device, &queue, &uniforms, "raindrops_uniforms");
            let bind_group = create_uniform_bind_group(&device, &layouts.raindrops, &buffer, "raindrops_bind_group");

            prepared.raindrops.push(PreparedEffectInstance {
                bind_group,
                effect_layer: rain.effect_layer,
            });
        }
    }

    // Prepare RGB splits
    {
        let mut seen: HashMap<u32, usize> = HashMap::new();
        for split in &extracted.rgb_splits {
            if seen.contains_key(&split.effect_layer) {
                continue;
            }
            seen.insert(split.effect_layer, prepared.rgb_splits.len());

            let uniforms = RgbSplitUniforms {
                red_offset: split.red_offset,
                green_offset: split.green_offset,
                blue_offset: split.blue_offset,
                intensity: split.intensity,
                _padding: 0.0,
            };

            let buffer = create_uniform_buffer(&device, &queue, &uniforms, "rgb_split_uniforms");
            let bind_group = create_uniform_bind_group(&device, &layouts.rgb_split, &buffer, "rgb_split_bind_group");

            prepared.rgb_splits.push(PreparedEffectInstance {
                bind_group,
                effect_layer: split.effect_layer,
            });
        }
    }

    // Prepare glitch effects
    {
        let mut seen: HashMap<u32, usize> = HashMap::new();
        for glitch in &extracted.glitches {
            if seen.contains_key(&glitch.effect_layer) {
                continue;
            }
            seen.insert(glitch.effect_layer, prepared.glitches.len());

            let uniforms = GlitchUniforms {
                time: extracted.time,
                intensity: glitch.intensity,
                rgb_split_amount: glitch.rgb_split_amount,
                scanline_density: glitch.scanline_density,
                block_size: glitch.block_size,
                noise_amount: glitch.noise_amount,
                _padding: 0.0,
            };

            let buffer = create_uniform_buffer(&device, &queue, &uniforms, "glitch_uniforms");
            let bind_group = create_uniform_bind_group(&device, &layouts.glitch, &buffer, "glitch_bind_group");

            prepared.glitches.push(PreparedEffectInstance {
                bind_group,
                effect_layer: glitch.effect_layer,
            });
        }
    }

    // Prepare EMP interference
    {
        let mut seen: HashMap<u32, usize> = HashMap::new();
        for emp in &extracted.emp_interferences {
            if seen.contains_key(&emp.effect_layer) {
                continue;
            }
            seen.insert(emp.effect_layer, prepared.emps.len());

            let uniforms = EmpUniforms {
                time: extracted.time,
                intensity: emp.intensity,
                flicker_rate: emp.flicker_rate,
                flicker_strength: emp.flicker_strength,
                band_count: emp.band_count,
                band_intensity: emp.band_intensity,
                band_speed: emp.band_speed,
                static_intensity: emp.static_intensity,
                burst_probability: emp.burst_probability,
                scanline_displacement: emp.scanline_displacement,
                chromatic_amount: emp.chromatic_amount,
                _padding: 0.0,
            };

            let buffer = create_uniform_buffer(&device, &queue, &uniforms, "emp_uniforms");
            let bind_group = create_uniform_bind_group(&device, &layouts.emp, &buffer, "emp_bind_group");

            prepared.emps.push(PreparedEffectInstance {
                bind_group,
                effect_layer: emp.effect_layer,
            });
        }
    }

    // Prepare damage vignettes
    {
        let mut seen: HashMap<u32, usize> = HashMap::new();
        for vignette in &extracted.damage_vignettes {
            if seen.contains_key(&vignette.effect_layer) {
                continue;
            }
            seen.insert(vignette.effect_layer, prepared.vignettes.len());

            let uniforms = DamageVignetteUniforms {
                color: Vec4::new(
                    vignette.color.red,
                    vignette.color.green,
                    vignette.color.blue,
                    vignette.color.alpha,
                ),
                size: vignette.size,
                softness: vignette.softness,
                pulse_frequency: vignette.pulse_frequency,
                time: extracted.time,
                intensity: vignette.intensity,
                _padding: [0.0; 3],
            };

            let buffer = create_uniform_buffer(&device, &queue, &uniforms, "vignette_uniforms");
            let bind_group = create_uniform_bind_group(&device, &layouts.vignette, &buffer, "vignette_bind_group");

            prepared.vignettes.push(PreparedEffectInstance {
                bind_group,
                effect_layer: vignette.effect_layer,
            });
        }
    }

    // Prepare screen flashes
    {
        let mut seen: HashMap<u32, usize> = HashMap::new();
        for flash in &extracted.screen_flashes {
            if seen.contains_key(&flash.effect_layer) {
                continue;
            }
            seen.insert(flash.effect_layer, prepared.flashes.len());

            let uniforms = ScreenFlashUniforms {
                color: Vec4::new(
                    flash.color.red,
                    flash.color.green,
                    flash.color.blue,
                    flash.color.alpha,
                ),
                blend: flash.blend,
                intensity: flash.intensity,
                _padding: [0.0; 2],
            };

            let buffer = create_uniform_buffer(&device, &queue, &uniforms, "flash_uniforms");
            let bind_group = create_uniform_bind_group(&device, &layouts.flash, &buffer, "flash_bind_group");

            prepared.flashes.push(PreparedEffectInstance {
                bind_group,
                effect_layer: flash.effect_layer,
            });
        }
    }

    // Prepare world heat shimmers
    {
        let mut seen: HashMap<u32, usize> = HashMap::new();
        for shimmer in &extracted.world_heat_shimmers {
            if seen.contains_key(&shimmer.effect_layer) {
                continue;
            }
            seen.insert(shimmer.effect_layer, prepared.world_heat_shimmers.len());

            let uniforms = WorldHeatShimmerUniforms {
                bounds: shimmer.bounds,
                amplitude: shimmer.amplitude,
                frequency: shimmer.frequency,
                speed: shimmer.speed,
                softness: shimmer.softness,
                time: extracted.time,
                intensity: shimmer.intensity,
                _padding: [0.0; 2],
            };

            let buffer = create_uniform_buffer(&device, &queue, &uniforms, "world_heat_shimmer_uniforms");
            let bind_group = create_uniform_bind_group(&device, &layouts.world_heat_shimmer, &buffer, "world_heat_shimmer_bind_group");

            prepared.world_heat_shimmers.push(PreparedEffectInstance {
                bind_group,
                effect_layer: shimmer.effect_layer,
            });
        }
    }

    // Prepare CRT effects — per-layer with per-camera viewport resolution
    {
        let mut seen: HashMap<u32, usize> = HashMap::new();
        for crt in &extracted.crts {
            if seen.contains_key(&crt.effect_layer) {
                continue;
            }
            seen.insert(crt.effect_layer, prepared.crts.len());

            let viewport = viewport_for_layer(&cameras, crt.effect_layer);

            let uniforms = CrtUniforms {
                time: extracted.time,
                intensity: crt.intensity,
                scanline_intensity: crt.scanline_intensity,
                scanline_count: crt.scanline_count,
                curvature: crt.curvature,
                corner_radius: crt.corner_radius,
                phosphor_type: crt.phosphor_type,
                phosphor_intensity: crt.phosphor_intensity,
                bloom: crt.bloom,
                vignette: crt.vignette,
                flicker: crt.flicker,
                color_bleed: crt.color_bleed,
                brightness: crt.brightness,
                saturation: crt.saturation,
                screen_width: viewport.x as f32,
                screen_height: viewport.y as f32,
                mask_shape: crt.mask_shape,
                _padding: [0.0; 3],
            };

            let buffer = create_uniform_buffer(&device, &queue, &uniforms, "crt_uniforms");
            let bind_group = create_uniform_bind_group(&device, &layouts.crt, &buffer, "crt_bind_group");

            prepared.crts.push(PreparedEffectInstance {
                bind_group,
                effect_layer: crt.effect_layer,
            });
        }
    }
}

fn create_uniform_buffer<T: ShaderType + bytemuck::Pod>(
    device: &RenderDevice,
    queue: &RenderQueue,
    data: &T,
    label: &str,
) -> Buffer {
    let buffer = device.create_buffer(&BufferDescriptor {
        label: Some(label),
        size: std::mem::size_of::<T>() as u64,
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    queue.write_buffer(&buffer, 0, bytemuck::bytes_of(data));
    buffer
}

fn create_uniform_bind_group(
    device: &RenderDevice,
    layout: &BindGroupLayout,
    buffer: &Buffer,
    label: &str,
) -> BindGroup {
    device.create_bind_group(
        label,
        layout,
        &[BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        }],
    )
}
