//! Block displacement / datamosh glitch effect.

use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;

use crate::effect::{ScreenEffect, EffectIntensity};
use crate::lifetime::EffectLifetime;

pub struct BlockDisplacementPlugin;

impl Plugin for BlockDisplacementPlugin {
    fn build(&self, _app: &mut App) {}
}

/// Block displacement glitch effect.
///
/// Displaces rectangular blocks of the image, simulating video compression artifacts.
#[derive(Component, Clone, ExtractComponent)]
pub struct BlockDisplacement {
    /// Size of displacement blocks (as fraction of screen).
    pub block_size: Vec2,
    /// Maximum displacement distance.
    pub max_displacement: f32,
    /// Probability of a block being displaced.
    pub probability: f32,
    /// How often blocks update.
    pub update_rate: f32,
}

impl Default for BlockDisplacement {
    fn default() -> Self {
        Self {
            block_size: Vec2::new(0.1, 0.05),
            max_displacement: 0.1,
            probability: 0.3,
            update_rate: 15.0,
        }
    }
}

#[derive(Bundle, Default)]
pub struct BlockDisplacementBundle {
    pub block_displacement: BlockDisplacement,
    pub effect: ScreenEffect,
    pub intensity: EffectIntensity,
    pub lifetime: EffectLifetime,
}
