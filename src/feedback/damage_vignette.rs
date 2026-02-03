//! Damage vignette effect.
//!
//! Red (or custom color) vignette that pulses at screen edges to indicate damage.

use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;

use crate::effect::{ScreenEffect, EffectIntensity};
use crate::lifetime::EffectLifetime;

pub struct DamageVignettePlugin;

impl Plugin for DamageVignettePlugin {
    fn build(&self, _app: &mut App) {}
}

/// Damage vignette effect.
#[derive(Component, Clone, ExtractComponent)]
pub struct DamageVignette {
    /// Color of the vignette.
    pub color: Color,
    /// How far the vignette extends from edges (0.0 to 1.0).
    pub size: f32,
    /// Edge softness.
    pub softness: f32,
    /// Pulsing frequency (0 = no pulse).
    pub pulse_frequency: f32,
}

impl Default for DamageVignette {
    fn default() -> Self {
        Self {
            color: Color::srgba(0.8, 0.0, 0.0, 0.6),
            size: 0.4,
            softness: 0.3,
            pulse_frequency: 8.0,
        }
    }
}

impl DamageVignette {
    /// Create with a custom color.
    pub fn with_color(color: Color) -> Self {
        Self { color, ..default() }
    }

    /// Healing effect (green).
    pub fn healing() -> Self {
        Self {
            color: Color::srgba(0.0, 0.8, 0.2, 0.5),
            pulse_frequency: 4.0,
            ..default()
        }
    }

    /// Shield/armor effect (blue).
    pub fn shield() -> Self {
        Self {
            color: Color::srgba(0.2, 0.4, 1.0, 0.5),
            pulse_frequency: 0.0,
            ..default()
        }
    }
}

#[derive(Bundle, Default)]
pub struct DamageVignetteBundle {
    pub vignette: DamageVignette,
    pub effect: ScreenEffect,
    pub intensity: EffectIntensity,
    pub lifetime: EffectLifetime,
}
