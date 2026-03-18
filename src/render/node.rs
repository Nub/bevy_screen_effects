//! Render graph node for applying screen effects.

use bevy::prelude::*;
use bevy::render::{
    render_graph::{NodeRunError, RenderGraphContext, ViewNode},
    render_resource::*,
    renderer::RenderContext,
    view::ViewTarget,
};

use crate::layer::{EffectLayer, SkipScreenEffects};

use super::pipeline::ScreenTextureBindGroupLayout;
use super::pipelines::EffectPipelines;
use super::prepare::PreparedEffects;

/// Render graph node that applies all active screen effects.
///
/// Effects are applied in sequence:
/// 1. Distortion effects (shockwave, radial blur)
/// 2. Glitch effects (RGB split, scanlines, etc.)
/// 3. Feedback effects (vignette, flash)
///
/// Each effect is filtered by `EffectLayer` bitmask — an effect only applies
/// to a camera if their layers overlap. Missing layers match everything.
#[derive(Default)]
pub struct ScreenEffectsNode;

impl ViewNode for ScreenEffectsNode {
    type ViewQuery = (&'static ViewTarget, Option<&'static EffectLayer>, Has<SkipScreenEffects>);

    fn run<'w>(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext<'w>,
        (view_target, camera_layer, skip_effects): (&ViewTarget, Option<&EffectLayer>, bool),
        world: &'w World,
    ) -> Result<(), NodeRunError> {
        // SkipScreenEffects = skip everything on this camera
        if skip_effects {
            return Ok(());
        }

        // Camera layer mask: None = match everything
        let camera_mask = camera_layer.map_or(u32::MAX, |l| l.0);

        // Get prepared effects data
        let Some(prepared) = world.get_resource::<PreparedEffects>() else {
            return Ok(());
        };

        // Skip if no effects are active
        if !prepared.has_any() {
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

        // Select SDR or HDR pipeline variant based on this camera's target format
        let target_format = view_target.main_texture_format();

        // Create sampler for screen texture
        let sampler = device.create_sampler(&SamplerDescriptor {
            label: Some("screen_effects_sampler"),
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            ..default()
        });

        // Apply effects in order, ping-ponging the view target as needed
        // Each effect is gated by layer mask overlap: (effect_layer & camera_mask) != 0

        // 1. Shockwave
        for instance in &prepared.shockwaves {
            if (instance.effect_layer & camera_mask) != 0 {
                if let Some(pipeline_id) = pipelines.shockwave.for_format(target_format) {
                    self.apply_effect(
                        render_context,
                        pipeline_cache,
                        view_target,
                        &texture_layout.layout,
                        &sampler,
                        pipeline_id,
                        &instance.bind_group,
                        "shockwave_pass",
                    );
                }
                break;
            }
        }

        // 2. Radial blur
        for instance in &prepared.radial_blurs {
            if (instance.effect_layer & camera_mask) != 0 {
                if let Some(pipeline_id) = pipelines.radial_blur.for_format(target_format) {
                    self.apply_effect(
                        render_context,
                        pipeline_cache,
                        view_target,
                        &texture_layout.layout,
                        &sampler,
                        pipeline_id,
                        &instance.bind_group,
                        "radial_blur_pass",
                    );
                }
                break;
            }
        }

        // 3. Raindrops
        for instance in &prepared.raindrops {
            if (instance.effect_layer & camera_mask) != 0 {
                if let Some(pipeline_id) = pipelines.raindrops.for_format(target_format) {
                    self.apply_effect(
                        render_context,
                        pipeline_cache,
                        view_target,
                        &texture_layout.layout,
                        &sampler,
                        pipeline_id,
                        &instance.bind_group,
                        "raindrops_pass",
                    );
                }
                break;
            }
        }

        // 4. World heat shimmer
        for instance in &prepared.world_heat_shimmers {
            if (instance.effect_layer & camera_mask) != 0 {
                if let Some(pipeline_id) = pipelines.world_heat_shimmer.for_format(target_format) {
                    self.apply_effect(
                        render_context,
                        pipeline_cache,
                        view_target,
                        &texture_layout.layout,
                        &sampler,
                        pipeline_id,
                        &instance.bind_group,
                        "world_heat_shimmer_pass",
                    );
                }
                break;
            }
        }

        // 5. RGB split
        for instance in &prepared.rgb_splits {
            if (instance.effect_layer & camera_mask) != 0 {
                if let Some(pipeline_id) = pipelines.rgb_split.for_format(target_format) {
                    self.apply_effect(
                        render_context,
                        pipeline_cache,
                        view_target,
                        &texture_layout.layout,
                        &sampler,
                        pipeline_id,
                        &instance.bind_group,
                        "rgb_split_pass",
                    );
                }
                break;
            }
        }

        // 6. Glitch
        for instance in &prepared.glitches {
            if (instance.effect_layer & camera_mask) != 0 {
                if let Some(pipeline_id) = pipelines.glitch.for_format(target_format) {
                    self.apply_effect(
                        render_context,
                        pipeline_cache,
                        view_target,
                        &texture_layout.layout,
                        &sampler,
                        pipeline_id,
                        &instance.bind_group,
                        "glitch_pass",
                    );
                }
                break;
            }
        }

        // 7. EMP Interference
        for instance in &prepared.emps {
            if (instance.effect_layer & camera_mask) != 0 {
                if let Some(pipeline_id) = pipelines.emp.for_format(target_format) {
                    self.apply_effect(
                        render_context,
                        pipeline_cache,
                        view_target,
                        &texture_layout.layout,
                        &sampler,
                        pipeline_id,
                        &instance.bind_group,
                        "emp_pass",
                    );
                }
                break;
            }
        }

        // 8. CRT effect
        for instance in &prepared.crts {
            if (instance.effect_layer & camera_mask) != 0 {
                if let Some(pipeline_id) = pipelines.crt.for_format(target_format) {
                    self.apply_effect(
                        render_context,
                        pipeline_cache,
                        view_target,
                        &texture_layout.layout,
                        &sampler,
                        pipeline_id,
                        &instance.bind_group,
                        "crt_pass",
                    );
                }
                break;
            }
        }

        // 9. Damage vignette
        for instance in &prepared.vignettes {
            if (instance.effect_layer & camera_mask) != 0 {
                if let Some(pipeline_id) = pipelines.vignette.for_format(target_format) {
                    self.apply_effect(
                        render_context,
                        pipeline_cache,
                        view_target,
                        &texture_layout.layout,
                        &sampler,
                        pipeline_id,
                        &instance.bind_group,
                        "vignette_pass",
                    );
                }
                break;
            }
        }

        // 10. Screen flash (applied last)
        for instance in &prepared.flashes {
            if (instance.effect_layer & camera_mask) != 0 {
                if let Some(pipeline_id) = pipelines.flash.for_format(target_format) {
                    self.apply_effect(
                        render_context,
                        pipeline_cache,
                        view_target,
                        &texture_layout.layout,
                        &sampler,
                        pipeline_id,
                        &instance.bind_group,
                        "flash_pass",
                    );
                }
                break;
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
