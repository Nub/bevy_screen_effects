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

/// World-space heat shimmer that creates a rising column of distortion.
///
/// Unlike [`HeatHaze`] which is fullscreen, this effect is localized to a
/// vertical column at a world position. The effect tracks camera movement
/// and scales with distance.
#[derive(Component, Clone)]
pub struct WorldHeatShimmer {
    /// World-space base position of the heat column.
    pub world_pos: Vec3,
    /// Width of the column in world units.
    pub width: f32,
    /// Height of the column in world units.
    pub height: f32,
    /// Distortion amplitude.
    pub amplitude: f32,
    /// Wave frequency.
    pub frequency: f32,
    /// Rising speed (how fast waves move upward).
    pub speed: f32,
    /// Edge softness (0.0 = hard edge, 1.0 = very soft).
    pub softness: f32,
}

impl Default for WorldHeatShimmer {
    fn default() -> Self {
        Self {
            world_pos: Vec3::ZERO,
            width: 1.0,
            height: 2.0,
            amplitude: 0.008,
            frequency: 40.0,
            speed: 0.5,
            softness: 0.1,
        }
    }
}

impl WorldHeatShimmer {
    /// Create a heat shimmer at the given world position.
    pub fn at(pos: Vec3) -> Self {
        Self {
            world_pos: pos,
            ..default()
        }
    }

    /// Set the column size (width and height in world units).
    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set the distortion amplitude.
    pub fn with_amplitude(mut self, amplitude: f32) -> Self {
        self.amplitude = amplitude;
        self
    }

    /// Set the wave frequency.
    pub fn with_frequency(mut self, frequency: f32) -> Self {
        self.frequency = frequency;
        self
    }

    /// Set the rising speed.
    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    /// Set the edge softness.
    pub fn with_softness(mut self, softness: f32) -> Self {
        self.softness = softness;
        self
    }
}

/// Bundle for spawning a world-space heat shimmer effect.
#[derive(Bundle, Default)]
pub struct WorldHeatShimmerBundle {
    pub shimmer: WorldHeatShimmer,
    pub effect: ScreenEffect,
    pub intensity: EffectIntensity,
    pub lifetime: EffectLifetime,
}

impl WorldHeatShimmerBundle {
    /// Create a heat shimmer at the given world position.
    pub fn at(pos: Vec3) -> Self {
        Self {
            shimmer: WorldHeatShimmer::at(pos),
            lifetime: EffectLifetime::new(5.0),
            ..default()
        }
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.shimmer.width = width;
        self.shimmer.height = height;
        self
    }

    pub fn with_duration(mut self, duration: f32) -> Self {
        self.lifetime = EffectLifetime::new(duration);
        self
    }
}
