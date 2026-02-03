//! Effect-specific render pipelines.

use bevy::prelude::*;
use bevy::render::render_resource::*;

use super::pipeline::ScreenTextureBindGroupLayout;
use super::prepare::EffectBindGroupLayouts;

/// Shader handles for all effect types.
#[derive(Resource)]
pub struct EffectShaders {
    pub shockwave: Handle<Shader>,
    pub radial_blur: Handle<Shader>,
    pub raindrops: Handle<Shader>,
    pub rgb_split: Handle<Shader>,
    pub glitch: Handle<Shader>,
    pub emp: Handle<Shader>,
    pub vignette: Handle<Shader>,
    pub flash: Handle<Shader>,
}

/// Cached render pipeline IDs for all effect types.
#[derive(Resource, Default)]
pub struct EffectPipelines {
    pub shockwave: Option<CachedRenderPipelineId>,
    pub radial_blur: Option<CachedRenderPipelineId>,
    pub raindrops: Option<CachedRenderPipelineId>,
    pub rgb_split: Option<CachedRenderPipelineId>,
    pub glitch: Option<CachedRenderPipelineId>,
    pub emp: Option<CachedRenderPipelineId>,
    pub vignette: Option<CachedRenderPipelineId>,
    pub flash: Option<CachedRenderPipelineId>,
}

/// System to queue effect pipelines for compilation.
pub fn queue_effect_pipelines(
    mut pipelines: ResMut<EffectPipelines>,
    shaders: Res<EffectShaders>,
    pipeline_cache: Res<PipelineCache>,
    texture_layout: Res<ScreenTextureBindGroupLayout>,
    uniforms_layouts: Res<EffectBindGroupLayouts>,
) {
    if pipelines.shockwave.is_none() {
        pipelines.shockwave = Some(queue_pipeline(
            &pipeline_cache,
            &texture_layout.entries,
            &uniforms_layouts.shockwave_entries,
            shaders.shockwave.clone(),
            "shockwave_pipeline",
        ));
    }

    if pipelines.radial_blur.is_none() {
        pipelines.radial_blur = Some(queue_pipeline(
            &pipeline_cache,
            &texture_layout.entries,
            &uniforms_layouts.radial_blur_entries,
            shaders.radial_blur.clone(),
            "radial_blur_pipeline",
        ));
    }

    if pipelines.raindrops.is_none() {
        pipelines.raindrops = Some(queue_pipeline(
            &pipeline_cache,
            &texture_layout.entries,
            &uniforms_layouts.raindrops_entries,
            shaders.raindrops.clone(),
            "raindrops_pipeline",
        ));
    }

    if pipelines.rgb_split.is_none() {
        pipelines.rgb_split = Some(queue_pipeline(
            &pipeline_cache,
            &texture_layout.entries,
            &uniforms_layouts.rgb_split_entries,
            shaders.rgb_split.clone(),
            "rgb_split_pipeline",
        ));
    }

    if pipelines.glitch.is_none() {
        pipelines.glitch = Some(queue_pipeline(
            &pipeline_cache,
            &texture_layout.entries,
            &uniforms_layouts.glitch_entries,
            shaders.glitch.clone(),
            "glitch_pipeline",
        ));
    }

    if pipelines.emp.is_none() {
        pipelines.emp = Some(queue_pipeline(
            &pipeline_cache,
            &texture_layout.entries,
            &uniforms_layouts.emp_entries,
            shaders.emp.clone(),
            "emp_pipeline",
        ));
    }

    if pipelines.vignette.is_none() {
        pipelines.vignette = Some(queue_pipeline(
            &pipeline_cache,
            &texture_layout.entries,
            &uniforms_layouts.vignette_entries,
            shaders.vignette.clone(),
            "vignette_pipeline",
        ));
    }

    if pipelines.flash.is_none() {
        pipelines.flash = Some(queue_pipeline(
            &pipeline_cache,
            &texture_layout.entries,
            &uniforms_layouts.flash_entries,
            shaders.flash.clone(),
            "flash_pipeline",
        ));
    }
}

fn queue_pipeline(
    pipeline_cache: &PipelineCache,
    texture_layout_entries: &[BindGroupLayoutEntry],
    uniforms_layout_entries: &[BindGroupLayoutEntry],
    shader: Handle<Shader>,
    label: &'static str,
) -> CachedRenderPipelineId {
    pipeline_cache.queue_render_pipeline(RenderPipelineDescriptor {
        label: Some(label.into()),
        layout: vec![
            BindGroupLayoutDescriptor {
                label: "texture_layout".into(),
                entries: texture_layout_entries.to_vec(),
            },
            BindGroupLayoutDescriptor {
                label: "uniforms_layout".into(),
                entries: uniforms_layout_entries.to_vec(),
            },
        ],
        vertex: VertexState {
            shader: shader.clone(),
            shader_defs: vec![],
            entry_point: Some("vertex".into()),
            buffers: vec![],
        },
        fragment: Some(FragmentState {
            shader,
            shader_defs: vec![],
            entry_point: Some("fragment".into()),
            targets: vec![Some(ColorTargetState {
                // Use standard sRGB format for non-HDR rendering
                // TODO: Add HDR support with pipeline specialization
                format: TextureFormat::Rgba8UnormSrgb,
                blend: Some(BlendState::ALPHA_BLENDING),
                write_mask: ColorWrites::ALL,
            })],
        }),
        primitive: PrimitiveState::default(),
        depth_stencil: None,
        multisample: MultisampleState::default(),
        push_constant_ranges: vec![],
        zero_initialize_workgroup_memory: false,
    })
}
