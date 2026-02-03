//! Core effect types and traits.

use bevy::prelude::*;

/// Marker component for active screen effects.
///
/// All effect entities must have this component to be processed by the render pipeline.
#[derive(Component, Default, Clone, Copy)]
pub struct ScreenEffect;

/// Current intensity multiplier for an effect.
///
/// This is typically driven by `EffectLifetime` but can be manually controlled.
/// Range: 0.0 (invisible) to 1.0 (full intensity).
#[derive(Component, Clone, Copy)]
pub struct EffectIntensity(pub f32);

impl Default for EffectIntensity {
    fn default() -> Self {
        Self(1.0)
    }
}

impl EffectIntensity {
    pub fn new(intensity: f32) -> Self {
        Self(intensity.clamp(0.0, 1.0))
    }

    pub fn get(&self) -> f32 {
        self.0
    }

    pub fn set(&mut self, intensity: f32) {
        self.0 = intensity.clamp(0.0, 1.0);
    }
}

/// Screen position for effects that originate from a point.
///
/// Uses normalized screen coordinates (0.0 to 1.0).
#[derive(Component, Clone, Copy, Default)]
pub struct EffectOrigin(pub Vec2);

impl EffectOrigin {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }

    pub fn center() -> Self {
        Self(Vec2::new(0.5, 0.5))
    }

    /// Convert world position to screen position given camera and window.
    /// Returns normalized screen coords where y=0 is top, y=1 is bottom.
    pub fn from_world(
        world_pos: Vec3,
        camera: &Camera,
        camera_transform: &GlobalTransform,
    ) -> Option<Self> {
        camera
            .world_to_ndc(camera_transform, world_pos)
            .map(|ndc| Self(Vec2::new(ndc.x * 0.5 + 0.5, -ndc.y * 0.5 + 0.5)))
    }
}
