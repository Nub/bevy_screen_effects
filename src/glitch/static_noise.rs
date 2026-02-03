//! Static noise / interference effect.

use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;

use crate::effect::{ScreenEffect, EffectIntensity};
use crate::lifetime::EffectLifetime;

pub struct StaticNoisePlugin;

impl Plugin for StaticNoisePlugin {
    fn build(&self, _app: &mut App) {}
}

/// Static noise effect.
#[derive(Component, Clone, ExtractComponent)]
pub struct StaticNoise {
    /// Noise density/grain size.
    pub grain_size: f32,
    /// Color vs monochrome noise (0.0 = mono, 1.0 = full color).
    pub color_amount: f32,
    /// How noise is blended (0.0 = additive, 1.0 = replace).
    pub blend_mode: f32,
}

impl Default for StaticNoise {
    fn default() -> Self {
        Self {
            grain_size: 1.0,
            color_amount: 0.0,
            blend_mode: 0.3,
        }
    }
}

#[derive(Bundle, Default)]
pub struct StaticNoiseBundle {
    pub static_noise: StaticNoise,
    pub effect: ScreenEffect,
    pub intensity: EffectIntensity,
    pub lifetime: EffectLifetime,
}
