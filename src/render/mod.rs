//! Render infrastructure for screen effects.
//!
//! This module provides the render graph integration and common utilities
//! for applying screen-space effects.

mod extract;
mod node;
mod pipeline;
mod pipelines;
mod prepare;

pub use node::ScreenEffectsNode;
pub use pipeline::ScreenTextureBindGroupLayout;
pub use pipelines::{EffectPipelines, EffectShaders};

use bevy::prelude::*;
use bevy::asset::embedded_asset;
use bevy::core_pipeline::core_2d::graph::{Core2d, Node2d};
use bevy::core_pipeline::core_3d::graph::{Core3d, Node3d};
use bevy::render::{
    extract_component::ExtractComponentPlugin,
    render_graph::{RenderLabel, ViewNodeRunner},
    Render, RenderApp,
};

use crate::effect::SkipScreenEffects;
use extract::{extract_effects, ExtractedEffects};
use prepare::{prepare_effects, EffectBindGroupLayouts, PreparedEffects};
use pipelines::queue_effect_pipelines;

pub struct ScreenEffectsRenderPlugin;

impl Plugin for ScreenEffectsRenderPlugin {
    fn build(&self, app: &mut App) {
        // Register extraction of SkipScreenEffects marker to the render world
        app.add_plugins(ExtractComponentPlugin::<SkipScreenEffects>::default());

        // Load embedded shaders
        embedded_asset!(app, "shaders/shockwave.wgsl");
        embedded_asset!(app, "shaders/radial_blur.wgsl");
        embedded_asset!(app, "shaders/raindrops.wgsl");
        embedded_asset!(app, "shaders/rgb_split.wgsl");
        embedded_asset!(app, "shaders/glitch.wgsl");
        embedded_asset!(app, "shaders/emp.wgsl");
        embedded_asset!(app, "shaders/vignette.wgsl");
        embedded_asset!(app, "shaders/flash.wgsl");
        embedded_asset!(app, "shaders/world_heat_shimmer.wgsl");
        embedded_asset!(app, "shaders/crt.wgsl");
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        // Get shader handles
        let asset_server = render_app.world().resource::<AssetServer>();
        let shaders = EffectShaders {
            shockwave: asset_server.load("embedded://bevy_screen_effects/render/shaders/shockwave.wgsl"),
            radial_blur: asset_server.load("embedded://bevy_screen_effects/render/shaders/radial_blur.wgsl"),
            raindrops: asset_server.load("embedded://bevy_screen_effects/render/shaders/raindrops.wgsl"),
            rgb_split: asset_server.load("embedded://bevy_screen_effects/render/shaders/rgb_split.wgsl"),
            glitch: asset_server.load("embedded://bevy_screen_effects/render/shaders/glitch.wgsl"),
            emp: asset_server.load("embedded://bevy_screen_effects/render/shaders/emp.wgsl"),
            vignette: asset_server.load("embedded://bevy_screen_effects/render/shaders/vignette.wgsl"),
            flash: asset_server.load("embedded://bevy_screen_effects/render/shaders/flash.wgsl"),
            world_heat_shimmer: asset_server.load("embedded://bevy_screen_effects/render/shaders/world_heat_shimmer.wgsl"),
            crt: asset_server.load("embedded://bevy_screen_effects/render/shaders/crt.wgsl"),
        };

        render_app
            // Resources
            .insert_resource(shaders)
            .init_resource::<ExtractedEffects>()
            .init_resource::<PreparedEffects>()
            .init_resource::<EffectPipelines>()
            .init_resource::<ScreenTextureBindGroupLayout>()
            .init_resource::<EffectBindGroupLayouts>()
            // Systems
            .add_systems(ExtractSchedule, extract_effects)
            .add_systems(Render, (prepare_effects, queue_effect_pipelines).chain());

        // Add render graph node to both Core3d and Core2d
        let world = render_app.world_mut();
        let node_3d = ViewNodeRunner::new(ScreenEffectsNode, world);
        let node_2d = ViewNodeRunner::new(ScreenEffectsNode, world);
        let mut render_graph = world.resource_mut::<bevy::render::render_graph::RenderGraph>();
        if let Some(graph_3d) = render_graph.get_sub_graph_mut(Core3d) {
            graph_3d.add_node(ScreenEffectsLabel, node_3d);
            graph_3d.add_node_edge(Node3d::Tonemapping, ScreenEffectsLabel);
            graph_3d.add_node_edge(ScreenEffectsLabel, Node3d::EndMainPassPostProcessing);
        }
        if let Some(graph_2d) = render_graph.get_sub_graph_mut(Core2d) {
            graph_2d.add_node(ScreenEffectsLabel, node_2d);
            graph_2d.add_node_edge(Node2d::Tonemapping, ScreenEffectsLabel);
            graph_2d.add_node_edge(ScreenEffectsLabel, Node2d::EndMainPassPostProcessing);
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct ScreenEffectsLabel;
