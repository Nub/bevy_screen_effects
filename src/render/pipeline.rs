//! Render pipeline infrastructure for screen effects.

use bevy::prelude::*;
use bevy::render::{
    render_resource::*,
    renderer::RenderDevice,
};

/// Bind group layout for the screen texture (shared by all effects).
#[derive(Resource)]
pub struct ScreenTextureBindGroupLayout {
    pub layout: BindGroupLayout,
    pub entries: Vec<BindGroupLayoutEntry>,
}

impl FromWorld for ScreenTextureBindGroupLayout {
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>();

        let entries = vec![
            // Screen texture
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            // Sampler
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Sampler(SamplerBindingType::Filtering),
                count: None,
            },
        ];

        let layout = device.create_bind_group_layout(
            "screen_effects_texture_layout",
            &entries,
        );

        Self { layout, entries }
    }
}

/// GPU representation of shockwave effect parameters.
#[derive(Clone, Copy, ShaderType, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct ShockwaveUniforms {
    pub center: Vec2,
    pub intensity: f32,
    pub progress: f32,
    pub ring_width: f32,
    pub max_radius: f32,
    pub chromatic: u32,
    pub _padding: f32,
}

/// GPU representation of radial blur parameters.
#[derive(Clone, Copy, ShaderType, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct RadialBlurUniforms {
    pub center: Vec2,
    pub intensity: f32,
    pub samples: u32,
}

/// GPU representation of RGB split parameters.
#[derive(Clone, Copy, ShaderType, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct RgbSplitUniforms {
    pub red_offset: Vec2,
    pub green_offset: Vec2,
    pub blue_offset: Vec2,
    pub intensity: f32,
    pub _padding: f32,
}

/// GPU representation of glitch effect parameters.
#[derive(Clone, Copy, ShaderType, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct GlitchUniforms {
    pub time: f32,
    pub intensity: f32,
    pub rgb_split_amount: f32,
    pub scanline_density: f32,
    pub block_size: Vec2,
    pub noise_amount: f32,
    pub _padding: f32,
}

/// GPU representation of damage vignette parameters.
#[derive(Clone, Copy, ShaderType, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct DamageVignetteUniforms {
    pub color: Vec4,
    pub size: f32,
    pub softness: f32,
    pub pulse_frequency: f32,
    pub time: f32,
    pub intensity: f32,
    pub _padding: [f32; 3],
}

/// GPU representation of screen flash parameters.
#[derive(Clone, Copy, ShaderType, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct ScreenFlashUniforms {
    pub color: Vec4,
    pub blend: f32,
    pub intensity: f32,
    pub _padding: [f32; 2],
}

/// GPU representation of raindrops parameters.
#[derive(Clone, Copy, ShaderType, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct RaindropsUniforms {
    pub time: f32,
    pub intensity: f32,
    pub drop_size: f32,
    pub density: f32,
    pub speed: f32,
    pub refraction: f32,
    pub trail_strength: f32,
    pub _padding: f32,
}

/// GPU representation of EMP interference parameters.
#[derive(Clone, Copy, ShaderType, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct EmpUniforms {
    pub time: f32,
    pub intensity: f32,
    pub flicker_rate: f32,
    pub flicker_strength: f32,
    pub band_count: f32,
    pub band_intensity: f32,
    pub band_speed: f32,
    pub static_intensity: f32,
    pub burst_probability: f32,
    pub scanline_displacement: f32,
    pub chromatic_amount: f32,
    pub _padding: f32,
}

/// GPU representation of CRT effect parameters.
#[derive(Clone, Copy, ShaderType, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct CrtUniforms {
    // Row 1 (16 bytes)
    pub time: f32,
    pub intensity: f32,
    pub scanline_intensity: f32,
    pub scanline_count: f32,
    // Row 2 (16 bytes)
    pub curvature: f32,
    pub corner_radius: f32,
    pub phosphor_type: u32,
    pub phosphor_intensity: f32,
    // Row 3 (16 bytes)
    pub bloom: f32,
    pub vignette: f32,
    pub flicker: f32,
    pub color_bleed: f32,
    // Row 4 (16 bytes)
    pub brightness: f32,
    pub saturation: f32,
    pub screen_width: f32,
    pub screen_height: f32,
    // Row 5 (16 bytes)
    pub mask_shape: u32,
    pub _padding: [f32; 3],
}

/// GPU representation of world heat shimmer parameters.
#[derive(Clone, Copy, ShaderType, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct WorldHeatShimmerUniforms {
    /// Screen-space bounds (left, right, top, bottom) in UV coordinates.
    pub bounds: Vec4,
    pub amplitude: f32,
    pub frequency: f32,
    pub speed: f32,
    pub softness: f32,
    pub time: f32,
    pub intensity: f32,
    pub _padding: [f32; 2],
}
