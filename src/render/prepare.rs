//! Preparation of GPU resources from extracted effect data.

use std::collections::HashMap;

use bevy::prelude::*;
use bevy::render::{
    render_resource::*,
    renderer::{RenderDevice, RenderQueue},
};

use super::extract::{EffectBucket, ExtractedEffects};
use super::pipeline::*;

/// Prepared GPU data for one set of effects (one camera or the global bucket).
#[derive(Default)]
pub struct PreparedBucket {
    pub shockwave_buffer: Option<Buffer>,
    pub shockwave_bind_group: Option<BindGroup>,
    pub shockwave_count: usize,

    pub radial_blur_buffer: Option<Buffer>,
    pub radial_blur_bind_group: Option<BindGroup>,
    pub radial_blur_count: usize,

    pub raindrops_buffer: Option<Buffer>,
    pub raindrops_bind_group: Option<BindGroup>,
    pub raindrops_count: usize,

    pub rgb_split_buffer: Option<Buffer>,
    pub rgb_split_bind_group: Option<BindGroup>,
    pub rgb_split_count: usize,

    pub glitch_buffer: Option<Buffer>,
    pub glitch_bind_group: Option<BindGroup>,
    pub has_glitch: bool,

    pub emp_buffer: Option<Buffer>,
    pub emp_bind_group: Option<BindGroup>,
    pub emp_count: usize,

    pub vignette_buffer: Option<Buffer>,
    pub vignette_bind_group: Option<BindGroup>,
    pub vignette_count: usize,

    pub flash_buffer: Option<Buffer>,
    pub flash_bind_group: Option<BindGroup>,
    pub flash_count: usize,

    pub world_heat_shimmer_buffer: Option<Buffer>,
    pub world_heat_shimmer_bind_group: Option<BindGroup>,
    pub world_heat_shimmer_count: usize,

    pub crt_buffer: Option<Buffer>,
    pub crt_bind_group: Option<BindGroup>,
    pub crt_count: usize,
}

impl PreparedBucket {
    pub fn has_any_effects(&self) -> bool {
        self.shockwave_count > 0
            || self.radial_blur_count > 0
            || self.raindrops_count > 0
            || self.rgb_split_count > 0
            || self.has_glitch
            || self.emp_count > 0
            || self.vignette_count > 0
            || self.flash_count > 0
            || self.world_heat_shimmer_count > 0
            || self.crt_count > 0
    }

    fn reset(&mut self) {
        self.shockwave_count = 0;
        self.radial_blur_count = 0;
        self.raindrops_count = 0;
        self.rgb_split_count = 0;
        self.has_glitch = false;
        self.emp_count = 0;
        self.vignette_count = 0;
        self.flash_count = 0;
        self.world_heat_shimmer_count = 0;
        self.crt_count = 0;
    }
}

/// Prepared GPU data for all active effects this frame, keyed by camera entity.
///
/// `None` key = effects that apply to all cameras.
/// `Some(entity)` key = effects targeted at a specific camera.
#[derive(Resource, Default)]
pub struct PreparedEffects {
    pub buckets: HashMap<Option<Entity>, PreparedBucket>,
}

impl PreparedEffects {
    /// Get the prepared bucket for a camera, combining global (None) + camera-specific.
    /// Returns None if there are no effects for this camera.
    pub fn global_bucket(&self) -> Option<&PreparedBucket> {
        self.buckets.get(&None)
    }

    pub fn camera_bucket(&self, entity: Entity) -> Option<&PreparedBucket> {
        self.buckets.get(&Some(entity))
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

/// System that prepares GPU resources from extracted effects.
pub fn prepare_effects(
    device: Res<RenderDevice>,
    queue: Res<RenderQueue>,
    extracted: Res<ExtractedEffects>,
    layouts: Res<EffectBindGroupLayouts>,
    mut prepared: ResMut<PreparedEffects>,
    cameras: Query<&bevy::render::camera::ExtractedCamera>,
) {
    // Reset all buckets
    for bucket in prepared.buckets.values_mut() {
        bucket.reset();
    }

    // Prepare each extracted bucket
    for (target, ext_bucket) in &extracted.buckets {
        if !ext_bucket.has_any() {
            continue;
        }

        let prep = prepared.buckets.entry(*target).or_default();
        prepare_bucket(&device, &queue, &layouts, prep, ext_bucket, extracted.time, &cameras);
    }
}

fn prepare_bucket(
    device: &RenderDevice,
    queue: &RenderQueue,
    layouts: &EffectBindGroupLayouts,
    prepared: &mut PreparedBucket,
    extracted: &EffectBucket,
    time: f32,
    cameras: &Query<&bevy::render::camera::ExtractedCamera>,
) {
    // Prepare shockwaves
    if !extracted.shockwaves.is_empty() {
        let sw = &extracted.shockwaves[0];
        let uniforms = ShockwaveUniforms {
            center: sw.center,
            intensity: sw.intensity,
            progress: sw.progress,
            ring_width: sw.ring_width,
            max_radius: sw.max_radius,
            chromatic: if sw.chromatic { 1 } else { 0 },
            _padding: 0.0,
        };

        let buffer = create_uniform_buffer(device, queue, &uniforms, "shockwave_uniforms");
        let bind_group = create_uniform_bind_group(device, &layouts.shockwave, &buffer, "shockwave_bind_group");

        prepared.shockwave_buffer = Some(buffer);
        prepared.shockwave_bind_group = Some(bind_group);
        prepared.shockwave_count = extracted.shockwaves.len();
    }

    // Prepare radial blurs
    if !extracted.radial_blurs.is_empty() {
        let blur = &extracted.radial_blurs[0];
        let uniforms = RadialBlurUniforms {
            center: blur.center,
            intensity: blur.intensity,
            samples: blur.samples,
        };

        let buffer = create_uniform_buffer(device, queue, &uniforms, "radial_blur_uniforms");
        let bind_group = create_uniform_bind_group(device, &layouts.radial_blur, &buffer, "radial_blur_bind_group");

        prepared.radial_blur_buffer = Some(buffer);
        prepared.radial_blur_bind_group = Some(bind_group);
        prepared.radial_blur_count = extracted.radial_blurs.len();
    }

    // Prepare raindrops
    if !extracted.raindrops.is_empty() {
        let rain = &extracted.raindrops[0];
        let uniforms = RaindropsUniforms {
            time,
            intensity: rain.intensity,
            drop_size: rain.drop_size,
            density: rain.density,
            speed: rain.speed,
            refraction: rain.refraction,
            trail_strength: rain.trail_strength,
            _padding: 0.0,
        };

        let buffer = create_uniform_buffer(device, queue, &uniforms, "raindrops_uniforms");
        let bind_group = create_uniform_bind_group(device, &layouts.raindrops, &buffer, "raindrops_bind_group");

        prepared.raindrops_buffer = Some(buffer);
        prepared.raindrops_bind_group = Some(bind_group);
        prepared.raindrops_count = extracted.raindrops.len();
    }

    // Prepare RGB splits
    if !extracted.rgb_splits.is_empty() {
        let split = &extracted.rgb_splits[0];
        let uniforms = RgbSplitUniforms {
            red_offset: split.red_offset,
            green_offset: split.green_offset,
            blue_offset: split.blue_offset,
            intensity: split.intensity,
            _padding: 0.0,
        };

        let buffer = create_uniform_buffer(device, queue, &uniforms, "rgb_split_uniforms");
        let bind_group = create_uniform_bind_group(device, &layouts.rgb_split, &buffer, "rgb_split_bind_group");

        prepared.rgb_split_buffer = Some(buffer);
        prepared.rgb_split_bind_group = Some(bind_group);
        prepared.rgb_split_count = extracted.rgb_splits.len();
    }

    // Prepare glitch effects
    if !extracted.glitches.is_empty() {
        let glitch = &extracted.glitches[0];
        let uniforms = GlitchUniforms {
            time,
            intensity: glitch.intensity,
            rgb_split_amount: glitch.rgb_split_amount,
            scanline_density: glitch.scanline_density,
            block_size: glitch.block_size,
            noise_amount: glitch.noise_amount,
            _padding: 0.0,
        };

        let buffer = create_uniform_buffer(device, queue, &uniforms, "glitch_uniforms");
        let bind_group = create_uniform_bind_group(device, &layouts.glitch, &buffer, "glitch_bind_group");

        prepared.glitch_buffer = Some(buffer);
        prepared.glitch_bind_group = Some(bind_group);
        prepared.has_glitch = true;
    }

    // Prepare EMP interference
    if !extracted.emp_interferences.is_empty() {
        let emp = &extracted.emp_interferences[0];
        let uniforms = EmpUniforms {
            time,
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

        let buffer = create_uniform_buffer(device, queue, &uniforms, "emp_uniforms");
        let bind_group = create_uniform_bind_group(device, &layouts.emp, &buffer, "emp_bind_group");

        prepared.emp_buffer = Some(buffer);
        prepared.emp_bind_group = Some(bind_group);
        prepared.emp_count = extracted.emp_interferences.len();
    }

    // Prepare damage vignettes
    if !extracted.damage_vignettes.is_empty() {
        let vignette = &extracted.damage_vignettes[0];
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
            time,
            intensity: vignette.intensity,
            _padding: [0.0; 3],
        };

        let buffer = create_uniform_buffer(device, queue, &uniforms, "vignette_uniforms");
        let bind_group = create_uniform_bind_group(device, &layouts.vignette, &buffer, "vignette_bind_group");

        prepared.vignette_buffer = Some(buffer);
        prepared.vignette_bind_group = Some(bind_group);
        prepared.vignette_count = extracted.damage_vignettes.len();
    }

    // Prepare screen flashes
    if !extracted.screen_flashes.is_empty() {
        let flash = &extracted.screen_flashes[0];
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

        let buffer = create_uniform_buffer(device, queue, &uniforms, "flash_uniforms");
        let bind_group = create_uniform_bind_group(device, &layouts.flash, &buffer, "flash_bind_group");

        prepared.flash_buffer = Some(buffer);
        prepared.flash_bind_group = Some(bind_group);
        prepared.flash_count = extracted.screen_flashes.len();
    }

    // Prepare world heat shimmers
    if !extracted.world_heat_shimmers.is_empty() {
        let shimmer = &extracted.world_heat_shimmers[0];
        let uniforms = WorldHeatShimmerUniforms {
            bounds: shimmer.bounds,
            amplitude: shimmer.amplitude,
            frequency: shimmer.frequency,
            speed: shimmer.speed,
            softness: shimmer.softness,
            time,
            intensity: shimmer.intensity,
            _padding: [0.0; 2],
        };

        let buffer = create_uniform_buffer(device, queue, &uniforms, "world_heat_shimmer_uniforms");
        let bind_group = create_uniform_bind_group(device, &layouts.world_heat_shimmer, &buffer, "world_heat_shimmer_bind_group");

        prepared.world_heat_shimmer_buffer = Some(buffer);
        prepared.world_heat_shimmer_bind_group = Some(bind_group);
        prepared.world_heat_shimmer_count = extracted.world_heat_shimmers.len();
    }

    // Prepare CRT effects
    if !extracted.crts.is_empty() {
        let crt = &extracted.crts[0];
        let uniforms = CrtUniforms {
            time,
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
            screen_width: cameras.iter().next()
                .and_then(|c| c.physical_viewport_size)
                .map(|s| s.x as f32)
                .unwrap_or(1920.0),
            screen_height: cameras.iter().next()
                .and_then(|c| c.physical_viewport_size)
                .map(|s| s.y as f32)
                .unwrap_or(1080.0),
            mask_shape: crt.mask_shape,
            _padding: [0.0; 3],
        };

        let buffer = create_uniform_buffer(device, queue, &uniforms, "crt_uniforms");
        let bind_group = create_uniform_bind_group(device, &layouts.crt, &buffer, "crt_bind_group");

        prepared.crt_buffer = Some(buffer);
        prepared.crt_bind_group = Some(bind_group);
        prepared.crt_count = extracted.crts.len();
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
