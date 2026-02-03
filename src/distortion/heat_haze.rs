//! Heat haze / underwater distortion effect.
//!
//! Creates a wavy distortion effect, useful for heat shimmer, underwater
//! views, or dream sequences.

use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;

use crate::effect::{ScreenEffect, EffectIntensity};
use crate::lifetime::EffectLifetime;

pub struct HeatHazePlugin;

impl Plugin for HeatHazePlugin {
    fn build(&self, _app: &mut App) {
        // Register shader, pipeline, etc.
    }
}

/// Heat haze distortion effect.
#[derive(Component, Clone, ExtractComponent)]
pub struct HeatHaze {
    /// Distortion amplitude.
    pub amplitude: f32,
    /// Wave frequency.
    pub frequency: f32,
    /// Animation speed.
    pub speed: f32,
    /// Direction of the wave (normalized).
    pub direction: Vec2,
}

impl Default for HeatHaze {
    fn default() -> Self {
        Self {
            amplitude: 0.01,
            frequency: 20.0,
            speed: 2.0,
            direction: Vec2::new(0.0, 1.0),
        }
    }
}

/// Bundle for spawning heat haze effect.
#[derive(Bundle, Default)]
pub struct HeatHazeBundle {
    pub heat_haze: HeatHaze,
    pub effect: ScreenEffect,
    pub intensity: EffectIntensity,
    pub lifetime: EffectLifetime,
}
