//! Render graph node for applying screen effects.

use bevy::prelude::*;
use bevy::render::{
    render_graph::{NodeRunError, RenderGraphContext, ViewNode},
    render_resource::*,
    renderer::RenderContext,
    view::ViewTarget,
};

use super::pipeline::ScreenTextureBindGroupLayout;
use super::pipelines::EffectPipelines;
use super::prepare::PreparedEffects;

/// Render graph node that applies all active screen effects.
///
/// Effects are applied in sequence:
/// 1. Distortion effects (shockwave, radial blur)
/// 2. Glitch effects (RGB split, scanlines, etc.)
/// 3. Feedback effects (vignette, flash)
#[derive(Default)]
pub struct ScreenEffectsNode;

impl ViewNode for ScreenEffectsNode {
    type ViewQuery = &'static ViewTarget;

    fn run<'w>(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext<'w>,
        view_target: &ViewTarget,
        world: &'w World,
    ) -> Result<(), NodeRunError> {
        // Get prepared effects data
        let Some(prepared) = world.get_resource::<PreparedEffects>() else {
            return Ok(());
        };

        // Skip if no effects are active
        if prepared.shockwave_count == 0
            && prepared.radial_blur_count == 0
            && prepared.raindrops_count == 0
            && prepared.world_heat_shimmer_count == 0
            && prepared.rgb_split_count == 0
            && !prepared.has_glitch
            && prepared.emp_count == 0
            && prepared.crt_count == 0
            && prepared.vignette_count == 0
            && prepared.flash_count == 0
        {
            return Ok(());
        }

        // Get pipelines and layouts
        let Some(pipelines) = world.get_resource::<EffectPipelines>() else {
            return Ok(());
        };
        let Some(texture_layout) = world.get_resource::<ScreenTextureBindGroupLayout>() else {
            return Ok(());
        };
        let pipeline_cache = world.resource::<PipelineCache>();
        let device = render_context.render_device();

        // Create sampler for screen texture
        let sampler = device.create_sampler(&SamplerDescriptor {
            label: Some("screen_effects_sampler"),
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            ..default()
        });

        // Apply effects in order, ping-ponging the view target as needed

        // 1. Shockwave
        if prepared.shockwave_count > 0 {
            if let Some(bind_group) = &prepared.shockwave_bind_group {
                if let Some(pipeline_id) = pipelines.shockwave {
                    self.apply_effect(
                        render_context,
                        pipeline_cache,
                        view_target,
                        &texture_layout.layout,
                        &sampler,
                        pipeline_id,
                        bind_group,
                        "shockwave_pass",
                    );
                }
            }
        }

        // 2. Radial blur
        if prepared.radial_blur_count > 0 {
            if let Some(bind_group) = &prepared.radial_blur_bind_group {
                if let Some(pipeline_id) = pipelines.radial_blur {
                    self.apply_effect(
                        render_context,
                        pipeline_cache,
                        view_target,
                        &texture_layout.layout,
                        &sampler,
                        pipeline_id,
                        bind_group,
                        "radial_blur_pass",
                    );
                }
            }
        }

        // 3. Raindrops
        if prepared.raindrops_count > 0 {
            if let Some(bind_group) = &prepared.raindrops_bind_group {
                if let Some(pipeline_id) = pipelines.raindrops {
                    self.apply_effect(
                        render_context,
                        pipeline_cache,
                        view_target,
                        &texture_layout.layout,
                        &sampler,
                        pipeline_id,
                        bind_group,
                        "raindrops_pass",
                    );
                }
            }
        }

        // 4. World heat shimmer
        if prepared.world_heat_shimmer_count > 0 {
            if let Some(bind_group) = &prepared.world_heat_shimmer_bind_group {
                if let Some(pipeline_id) = pipelines.world_heat_shimmer {
                    self.apply_effect(
                        render_context,
                        pipeline_cache,
                        view_target,
                        &texture_layout.layout,
                        &sampler,
                        pipeline_id,
                        bind_group,
                        "world_heat_shimmer_pass",
                    );
                }
            }
        }

        // 5. RGB split
        if prepared.rgb_split_count > 0 {
            if let Some(bind_group) = &prepared.rgb_split_bind_group {
                if let Some(pipeline_id) = pipelines.rgb_split {
                    self.apply_effect(
                        render_context,
                        pipeline_cache,
                        view_target,
                        &texture_layout.layout,
                        &sampler,
                        pipeline_id,
                        bind_group,
                        "rgb_split_pass",
                    );
                }
            }
        }

        // 5. Glitch
        if prepared.has_glitch {
            if let Some(bind_group) = &prepared.glitch_bind_group {
                if let Some(pipeline_id) = pipelines.glitch {
                    self.apply_effect(
                        render_context,
                        pipeline_cache,
                        view_target,
                        &texture_layout.layout,
                        &sampler,
                        pipeline_id,
                        bind_group,
                        "glitch_pass",
                    );
                }
            }
        }

        // 6. EMP Interference
        if prepared.emp_count > 0 {
            if let Some(bind_group) = &prepared.emp_bind_group {
                if let Some(pipeline_id) = pipelines.emp {
                    self.apply_effect(
                        render_context,
                        pipeline_cache,
                        view_target,
                        &texture_layout.layout,
                        &sampler,
                        pipeline_id,
                        bind_group,
                        "emp_pass",
                    );
                }
            }
        }

        // 7. CRT effect
        if prepared.crt_count > 0 {
            if let Some(bind_group) = &prepared.crt_bind_group {
                if let Some(pipeline_id) = pipelines.crt {
                    self.apply_effect(
                        render_context,
                        pipeline_cache,
                        view_target,
                        &texture_layout.layout,
                        &sampler,
                        pipeline_id,
                        bind_group,
                        "crt_pass",
                    );
                }
            }
        }

        // 8. Damage vignette
        if prepared.vignette_count > 0 {
            if let Some(bind_group) = &prepared.vignette_bind_group {
                if let Some(pipeline_id) = pipelines.vignette {
                    self.apply_effect(
                        render_context,
                        pipeline_cache,
                        view_target,
                        &texture_layout.layout,
                        &sampler,
                        pipeline_id,
                        bind_group,
                        "vignette_pass",
                    );
                }
            }
        }

        // 8. Screen flash (applied last)
        if prepared.flash_count > 0 {
            if let Some(bind_group) = &prepared.flash_bind_group {
                if let Some(pipeline_id) = pipelines.flash {
                    self.apply_effect(
                        render_context,
                        pipeline_cache,
                        view_target,
                        &texture_layout.layout,
                        &sampler,
                        pipeline_id,
                        bind_group,
                        "flash_pass",
                    );
                }
            }
        }

        Ok(())
    }
}

impl ScreenEffectsNode {
    fn apply_effect(
        &self,
        render_context: &mut RenderContext,
        pipeline_cache: &PipelineCache,
        view_target: &ViewTarget,
        texture_layout: &BindGroupLayout,
        sampler: &Sampler,
        pipeline_id: CachedRenderPipelineId,
        uniforms_bind_group: &BindGroup,
        label: &str,
    ) {
        // Get the pipeline, skip if not ready
        let Some(pipeline) = pipeline_cache.get_render_pipeline(pipeline_id) else {
            return;
        };

        // Use post_process_write to handle ping-pong automatically
        let post_process = view_target.post_process_write();
        let device = render_context.render_device();

        // Create bind group for the source texture
        let texture_bind_group = device.create_bind_group(
            label,
            texture_layout,
            &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(post_process.source),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(sampler),
                },
            ],
        );

        // Create render pass
        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some(label),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: post_process.destination,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Load,
                    store: StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_render_pipeline(pipeline);
        render_pass.set_bind_group(0, &texture_bind_group, &[]);
        render_pass.set_bind_group(1, uniforms_bind_group, &[]);
        render_pass.draw(0..3, 0..1);
    }
}
