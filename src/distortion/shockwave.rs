//! Shockwave/explosion distortion effect.
//!
//! Creates an expanding ring of distortion from a point, commonly used for
//! explosions, impacts, or ability activations.

use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;

use crate::effect::{ScreenEffect, EffectIntensity, EffectOrigin};
use crate::lifetime::EffectLifetime;

pub struct ShockwavePlugin;

impl Plugin for ShockwavePlugin {
    fn build(&self, _app: &mut App) {
        // Register shader, pipeline, etc.
    }
}

/// Shockwave distortion effect component.
///
/// Creates a ring of distortion that expands outward from the origin.
#[derive(Component, Clone, ExtractComponent)]
pub struct Shockwave {
    /// Center of the shockwave in normalized screen coords (0.0 to 1.0).
    pub center: Vec2,
    /// Maximum distortion intensity.
    pub intensity: f32,
    /// Width of the distortion ring.
    pub ring_width: f32,
    /// Maximum radius the shockwave expands to.
    pub max_radius: f32,
    /// Whether to also apply chromatic aberration.
    pub chromatic: bool,
}

impl Default for Shockwave {
    fn default() -> Self {
        Self {
            center: Vec2::new(0.5, 0.5),
            intensity: 0.25,
            ring_width: 0.1,
            max_radius: 0.8,
            chromatic: true,
        }
    }
}

impl Shockwave {
    /// Create a shockwave at the given screen position.
    pub fn at(x: f32, y: f32) -> Self {
        Self {
            center: Vec2::new(x, y),
            ..default()
        }
    }

    /// Set the intensity (distortion strength).
    pub fn with_intensity(mut self, intensity: f32) -> Self {
        self.intensity = intensity;
        self
    }

    /// Set the ring width.
    pub fn with_ring_width(mut self, width: f32) -> Self {
        self.ring_width = width;
        self
    }

    /// Set the maximum radius.
    pub fn with_max_radius(mut self, radius: f32) -> Self {
        self.max_radius = radius;
        self
    }

    /// Enable or disable chromatic aberration.
    pub fn with_chromatic(mut self, enabled: bool) -> Self {
        self.chromatic = enabled;
        self
    }
}

/// Bundle for spawning a shockwave effect.
#[derive(Bundle, Default)]
pub struct ShockwaveBundle {
    pub shockwave: Shockwave,
    pub effect: ScreenEffect,
    pub intensity: EffectIntensity,
    pub lifetime: EffectLifetime,
}

impl ShockwaveBundle {
    /// Create a shockwave at the given normalized screen position.
    pub fn at(x: f32, y: f32) -> Self {
        Self {
            shockwave: Shockwave::at(x, y),
            lifetime: EffectLifetime::new(0.5),
            ..default()
        }
    }

    /// Create a shockwave from a world position.
    pub fn from_world(
        world_pos: Vec3,
        camera: &Camera,
        camera_transform: &GlobalTransform,
    ) -> Option<Self> {
        EffectOrigin::from_world(world_pos, camera, camera_transform).map(|origin| Self {
            shockwave: Shockwave::at(origin.0.x, origin.0.y),
            lifetime: EffectLifetime::new(0.5),
            ..default()
        })
    }

    pub fn with_intensity(mut self, intensity: f32) -> Self {
        self.shockwave.intensity = intensity;
        self
    }

    pub fn with_duration(mut self, duration: f32) -> Self {
        self.lifetime = EffectLifetime::new(duration);
        self
    }
}

/// World-space shockwave that tracks camera movement.
///
/// Unlike [`Shockwave`] which uses screen coordinates, this effect takes a 3D
/// world position and re-projects it to screen space every frame. The effect
/// stays anchored to the world position as the camera moves.
#[derive(Component, Clone)]
pub struct WorldShockwave {
    /// World-space position of the shockwave center.
    pub world_pos: Vec3,
    /// Maximum distortion intensity.
    pub intensity: f32,
    /// Width of the distortion ring.
    pub ring_width: f32,
    /// Maximum radius the shockwave expands to (in screen space).
    pub max_radius: f32,
    /// Whether to also apply chromatic aberration.
    pub chromatic: bool,
}

impl Default for WorldShockwave {
    fn default() -> Self {
        Self {
            world_pos: Vec3::ZERO,
            intensity: 0.25,
            ring_width: 0.1,
            max_radius: 0.8,
            chromatic: true,
        }
    }
}

impl WorldShockwave {
    /// Create a world-space shockwave at the given position.
    pub fn at(pos: Vec3) -> Self {
        Self {
            world_pos: pos,
            ..default()
        }
    }

    /// Set the intensity (distortion strength).
    pub fn with_intensity(mut self, intensity: f32) -> Self {
        self.intensity = intensity;
        self
    }

    /// Set the ring width.
    pub fn with_ring_width(mut self, width: f32) -> Self {
        self.ring_width = width;
        self
    }

    /// Set the maximum radius.
    pub fn with_max_radius(mut self, radius: f32) -> Self {
        self.max_radius = radius;
        self
    }

    /// Enable or disable chromatic aberration.
    pub fn with_chromatic(mut self, enabled: bool) -> Self {
        self.chromatic = enabled;
        self
    }
}

/// Bundle for spawning a world-space shockwave effect.
#[derive(Bundle, Default)]
pub struct WorldShockwaveBundle {
    pub shockwave: WorldShockwave,
    pub effect: ScreenEffect,
    pub intensity: EffectIntensity,
    pub lifetime: EffectLifetime,
}

impl WorldShockwaveBundle {
    /// Create a world-space shockwave at the given position.
    pub fn at(pos: Vec3) -> Self {
        Self {
            shockwave: WorldShockwave::at(pos),
            lifetime: EffectLifetime::new(0.5),
            ..default()
        }
    }

    pub fn with_intensity(mut self, intensity: f32) -> Self {
        self.shockwave.intensity = intensity;
        self
    }

    pub fn with_duration(mut self, duration: f32) -> Self {
        self.lifetime = EffectLifetime::new(duration);
        self
    }
}
