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
    pub world_heat_shimmer: Handle<Shader>,
    pub crt: Handle<Shader>,
}

/// LDR + HDR pipeline pair for a single effect.
#[derive(Default, Clone, Copy)]
pub struct FormatPipeline {
    pub ldr: Option<CachedRenderPipelineId>,
    pub hdr: Option<CachedRenderPipelineId>,
}

impl FormatPipeline {
    /// Select the pipeline matching the given texture format.
    pub fn for_format(&self, format: TextureFormat) -> Option<CachedRenderPipelineId> {
        match format {
            TextureFormat::Rgba16Float => self.hdr,
            _ => self.ldr,
        }
    }
}

/// Cached render pipeline IDs for all effect types.
#[derive(Resource, Default)]
pub struct EffectPipelines {
    pub shockwave: FormatPipeline,
    pub radial_blur: FormatPipeline,
    pub raindrops: FormatPipeline,
    pub rgb_split: FormatPipeline,
    pub glitch: FormatPipeline,
    pub emp: FormatPipeline,
    pub vignette: FormatPipeline,
    pub flash: FormatPipeline,
    pub world_heat_shimmer: FormatPipeline,
    pub crt: FormatPipeline,
}

/// Queue both LDR and HDR variants of a pipeline if not already cached.
fn queue_both(
    fp: &mut FormatPipeline,
    pipeline_cache: &PipelineCache,
    texture_entries: &[BindGroupLayoutEntry],
    uniforms_entries: &[BindGroupLayoutEntry],
    shader: Handle<Shader>,
    label: &'static str,
) {
    if fp.ldr.is_none() {
        fp.ldr = Some(queue_pipeline(
            pipeline_cache, texture_entries, uniforms_entries,
            shader.clone(), label, TextureFormat::Rgba8UnormSrgb,
        ));
    }
    if fp.hdr.is_none() {
        fp.hdr = Some(queue_pipeline(
            pipeline_cache, texture_entries, uniforms_entries,
            shader, label, TextureFormat::Rgba16Float,
        ));
    }
}

/// System to queue effect pipelines for compilation.
pub fn queue_effect_pipelines(
    mut pipelines: ResMut<EffectPipelines>,
    shaders: Res<EffectShaders>,
    pipeline_cache: Res<PipelineCache>,
    texture_layout: Res<ScreenTextureBindGroupLayout>,
    uniforms_layouts: Res<EffectBindGroupLayouts>,
) {
    queue_both(&mut pipelines.shockwave, &pipeline_cache, &texture_layout.entries,
        &uniforms_layouts.shockwave_entries, shaders.shockwave.clone(), "shockwave_pipeline");
    queue_both(&mut pipelines.radial_blur, &pipeline_cache, &texture_layout.entries,
        &uniforms_layouts.radial_blur_entries, shaders.radial_blur.clone(), "radial_blur_pipeline");
    queue_both(&mut pipelines.raindrops, &pipeline_cache, &texture_layout.entries,
        &uniforms_layouts.raindrops_entries, shaders.raindrops.clone(), "raindrops_pipeline");
    queue_both(&mut pipelines.rgb_split, &pipeline_cache, &texture_layout.entries,
        &uniforms_layouts.rgb_split_entries, shaders.rgb_split.clone(), "rgb_split_pipeline");
    queue_both(&mut pipelines.glitch, &pipeline_cache, &texture_layout.entries,
        &uniforms_layouts.glitch_entries, shaders.glitch.clone(), "glitch_pipeline");
    queue_both(&mut pipelines.emp, &pipeline_cache, &texture_layout.entries,
        &uniforms_layouts.emp_entries, shaders.emp.clone(), "emp_pipeline");
    queue_both(&mut pipelines.vignette, &pipeline_cache, &texture_layout.entries,
        &uniforms_layouts.vignette_entries, shaders.vignette.clone(), "vignette_pipeline");
    queue_both(&mut pipelines.flash, &pipeline_cache, &texture_layout.entries,
        &uniforms_layouts.flash_entries, shaders.flash.clone(), "flash_pipeline");
    queue_both(&mut pipelines.world_heat_shimmer, &pipeline_cache, &texture_layout.entries,
        &uniforms_layouts.world_heat_shimmer_entries, shaders.world_heat_shimmer.clone(), "world_heat_shimmer_pipeline");
    queue_both(&mut pipelines.crt, &pipeline_cache, &texture_layout.entries,
        &uniforms_layouts.crt_entries, shaders.crt.clone(), "crt_pipeline");
}

fn queue_pipeline(
    pipeline_cache: &PipelineCache,
    texture_layout_entries: &[BindGroupLayoutEntry],
    uniforms_layout_entries: &[BindGroupLayoutEntry],
    shader: Handle<Shader>,
    label: &'static str,
    format: TextureFormat,
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
                format,
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
