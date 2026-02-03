//! Scanline glitch effect.

use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;

use crate::effect::{ScreenEffect, EffectIntensity};
use crate::lifetime::EffectLifetime;

pub struct ScanlinePlugin;

impl Plugin for ScanlinePlugin {
    fn build(&self, _app: &mut App) {}
}

/// Scanline glitch effect.
#[derive(Component, Clone, ExtractComponent)]
pub struct ScanlineGlitch {
    /// Probability of a scanline being affected (0.0 to 1.0).
    pub density: f32,
    /// Maximum horizontal displacement.
    pub displacement: f32,
    /// Scanline thickness in pixels.
    pub line_height: f32,
    /// How fast glitch lines change.
    pub flicker_speed: f32,
}

impl Default for ScanlineGlitch {
    fn default() -> Self {
        Self {
            density: 0.1,
            displacement: 0.05,
            line_height: 2.0,
            flicker_speed: 30.0,
        }
    }
}

#[derive(Bundle, Default)]
pub struct ScanlineGlitchBundle {
    pub scanline: ScanlineGlitch,
    pub effect: ScreenEffect,
    pub intensity: EffectIntensity,
    pub lifetime: EffectLifetime,
}
