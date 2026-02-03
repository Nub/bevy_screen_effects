//! Radial blur effect.
//!
//! Blurs the image in a radial pattern from a center point, commonly used
//! for speed effects, impacts, or focus transitions.

use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;

use crate::effect::{ScreenEffect, EffectIntensity};
use crate::lifetime::EffectLifetime;

pub struct RadialBlurPlugin;

impl Plugin for RadialBlurPlugin {
    fn build(&self, _app: &mut App) {
        // Register shader, pipeline, etc.
    }
}

/// Radial blur effect component.
#[derive(Component, Clone, ExtractComponent)]
pub struct RadialBlur {
    /// Center of the blur in normalized screen coords.
    pub center: Vec2,
    /// Blur intensity (sample distance).
    pub intensity: f32,
    /// Number of blur samples.
    pub samples: u32,
}

impl Default for RadialBlur {
    fn default() -> Self {
        Self {
            center: Vec2::new(0.5, 0.5),
            intensity: 0.1,
            samples: 8,
        }
    }
}

/// Bundle for spawning a radial blur effect.
#[derive(Bundle, Default)]
pub struct RadialBlurBundle {
    pub radial_blur: RadialBlur,
    pub effect: ScreenEffect,
    pub intensity: EffectIntensity,
    pub lifetime: EffectLifetime,
}
