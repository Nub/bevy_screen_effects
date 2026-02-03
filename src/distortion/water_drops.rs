//! Raindrops screen effect.
//!
//! Simulates raindrops on the screen/camera lens with refraction.

use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;

use crate::effect::{ScreenEffect, EffectIntensity};
use crate::lifetime::EffectLifetime;

pub struct RaindropsPlugin;

impl Plugin for RaindropsPlugin {
    fn build(&self, _app: &mut App) {
        // Rendering is handled by ScreenEffectsRenderPlugin
    }
}

/// Raindrops effect component.
///
/// Creates procedurally-generated raindrops that fall down the screen
/// with realistic refraction/distortion.
#[derive(Component, Clone, ExtractComponent)]
pub struct Raindrops {
    /// Size of individual drops (0.01 - 0.1 typical).
    pub drop_size: f32,
    /// Density of drops (0.0 - 1.0).
    pub density: f32,
    /// Fall speed multiplier.
    pub speed: f32,
    /// Refraction/distortion strength.
    pub refraction: f32,
    /// Strength of trailing streaks behind drops.
    pub trail_strength: f32,
}

impl Default for Raindrops {
    fn default() -> Self {
        Self {
            drop_size: 0.03,
            density: 0.5,
            speed: 0.3,
            refraction: 0.02,
            trail_strength: 0.5,
        }
    }
}

impl Raindrops {
    /// Light rain with small, sparse drops.
    pub fn light() -> Self {
        Self {
            drop_size: 0.02,
            density: 0.3,
            speed: 0.2,
            refraction: 0.015,
            trail_strength: 0.3,
        }
    }

    /// Heavy rain with larger, denser drops.
    pub fn heavy() -> Self {
        Self {
            drop_size: 0.04,
            density: 0.7,
            speed: 0.5,
            refraction: 0.03,
            trail_strength: 0.7,
        }
    }

    /// Dramatic storm effect.
    pub fn storm() -> Self {
        Self {
            drop_size: 0.05,
            density: 0.9,
            speed: 0.8,
            refraction: 0.04,
            trail_strength: 0.9,
        }
    }

    /// Gentle drizzle.
    pub fn drizzle() -> Self {
        Self {
            drop_size: 0.015,
            density: 0.4,
            speed: 0.15,
            refraction: 0.01,
            trail_strength: 0.2,
        }
    }

    /// Builder: set drop size.
    pub fn with_drop_size(mut self, size: f32) -> Self {
        self.drop_size = size;
        self
    }

    /// Builder: set density.
    pub fn with_density(mut self, density: f32) -> Self {
        self.density = density.clamp(0.0, 1.0);
        self
    }

    /// Builder: set fall speed.
    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    /// Builder: set refraction strength.
    pub fn with_refraction(mut self, refraction: f32) -> Self {
        self.refraction = refraction;
        self
    }

    /// Builder: set trail strength.
    pub fn with_trail(mut self, strength: f32) -> Self {
        self.trail_strength = strength;
        self
    }
}

/// Bundle for spawning raindrops effect.
#[derive(Bundle, Default)]
pub struct RaindropsBundle {
    pub raindrops: Raindrops,
    pub effect: ScreenEffect,
    pub intensity: EffectIntensity,
    pub lifetime: EffectLifetime,
}
