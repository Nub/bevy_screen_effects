//! Preparation of GPU resources from extracted effect data.

use bevy::prelude::*;
use bevy::render::{
    render_resource::*,
    renderer::{RenderDevice, RenderQueue},
};

use super::extract::ExtractedEffects;
use super::pipeline::*;

/// Prepared GPU data for all active effects this frame.
#[derive(Resource)]
pub struct PreparedEffects {
    /// Uniform buffer for shockwave instances.
    pub shockwave_buffer: Option<Buffer>,
    pub shockwave_bind_group: Option<BindGroup>,
    pub shockwave_count: usize,

    /// Uniform buffer for radial blur instances.
    pub radial_blur_buffer: Option<Buffer>,
    pub radial_blur_bind_group: Option<BindGroup>,
    pub radial_blur_count: usize,

    /// Uniform buffer for raindrops instances.
    pub raindrops_buffer: Option<Buffer>,
    pub raindrops_bind_group: Option<BindGroup>,
    pub raindrops_count: usize,

    /// Uniform buffer for RGB split instances.
    pub rgb_split_buffer: Option<Buffer>,
    pub rgb_split_bind_group: Option<BindGroup>,
    pub rgb_split_count: usize,

    /// Uniform buffer for combined glitch effect.
    pub glitch_buffer: Option<Buffer>,
    pub glitch_bind_group: Option<BindGroup>,
    pub has_glitch: bool,

    /// Uniform buffer for EMP interference effect.
    pub emp_buffer: Option<Buffer>,
    pub emp_bind_group: Option<BindGroup>,
    pub emp_count: usize,

    /// Uniform buffer for damage vignette instances.
    pub vignette_buffer: Option<Buffer>,
    pub vignette_bind_group: Option<BindGroup>,
    pub vignette_count: usize,

    /// Uniform buffer for screen flash instances.
    pub flash_buffer: Option<Buffer>,
    pub flash_bind_group: Option<BindGroup>,
    pub flash_count: usize,
}

impl Default for PreparedEffects {
    fn default() -> Self {
        Self {
            shockwave_buffer: None,
            shockwave_bind_group: None,
            shockwave_count: 0,
            radial_blur_buffer: None,
            radial_blur_bind_group: None,
            radial_blur_count: 0,
            raindrops_buffer: None,
            raindrops_bind_group: None,
            raindrops_count: 0,
            rgb_split_buffer: None,
            rgb_split_bind_group: None,
            rgb_split_count: 0,
            glitch_buffer: None,
            glitch_bind_group: None,
            has_glitch: false,
            emp_buffer: None,
            emp_bind_group: None,
            emp_count: 0,
            vignette_buffer: None,
            vignette_bind_group: None,
            vignette_count: 0,
            flash_buffer: None,
            flash_bind_group: None,
            flash_count: 0,
        }
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
) {
    // Reset counts
    prepared.shockwave_count = 0;
    prepared.radial_blur_count = 0;
    prepared.raindrops_count = 0;
    prepared.rgb_split_count = 0;
    prepared.has_glitch = false;
    prepared.emp_count = 0;
    prepared.vignette_count = 0;
    prepared.flash_count = 0;

    // Prepare shockwaves
    if !extracted.shockwaves.is_empty() {
        // For now, just handle the first shockwave (can batch later)
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

        let buffer = create_uniform_buffer(&device, &queue, &uniforms, "shockwave_uniforms");
        let bind_group = create_uniform_bind_group(&device, &layouts.shockwave, &buffer, "shockwave_bind_group");

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

        let buffer = create_uniform_buffer(&device, &queue, &uniforms, "radial_blur_uniforms");
        let bind_group = create_uniform_bind_group(&device, &layouts.radial_blur, &buffer, "radial_blur_bind_group");

        prepared.radial_blur_buffer = Some(buffer);
        prepared.radial_blur_bind_group = Some(bind_group);
        prepared.radial_blur_count = extracted.radial_blurs.len();
    }

    // Prepare raindrops
    if !extracted.raindrops.is_empty() {
        let rain = &extracted.raindrops[0];
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

        let buffer = create_uniform_buffer(&device, &queue, &uniforms, "rgb_split_uniforms");
        let bind_group = create_uniform_bind_group(&device, &layouts.rgb_split, &buffer, "rgb_split_bind_group");

        prepared.rgb_split_buffer = Some(buffer);
        prepared.rgb_split_bind_group = Some(bind_group);
        prepared.rgb_split_count = extracted.rgb_splits.len();
    }

    // Prepare glitch effects
    if !extracted.glitches.is_empty() {
        let glitch = &extracted.glitches[0];
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

        prepared.glitch_buffer = Some(buffer);
        prepared.glitch_bind_group = Some(bind_group);
        prepared.has_glitch = true;
    }

    // Prepare EMP interference
    if !extracted.emp_interferences.is_empty() {
        let emp = &extracted.emp_interferences[0];
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
            time: extracted.time,
            intensity: vignette.intensity,
            _padding: [0.0; 3],
        };

        let buffer = create_uniform_buffer(&device, &queue, &uniforms, "vignette_uniforms");
        let bind_group = create_uniform_bind_group(&device, &layouts.vignette, &buffer, "vignette_bind_group");

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

        let buffer = create_uniform_buffer(&device, &queue, &uniforms, "flash_uniforms");
        let bind_group = create_uniform_bind_group(&device, &layouts.flash, &buffer, "flash_bind_group");

        prepared.flash_buffer = Some(buffer);
        prepared.flash_bind_group = Some(bind_group);
        prepared.flash_count = extracted.screen_flashes.len();
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
