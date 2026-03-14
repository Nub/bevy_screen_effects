//! Bitmask-based layer system for targeting effects to specific cameras.

use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;

/// Bitmask-based layer assignment for effects and cameras.
///
/// An effect applies to a camera only if their layers overlap (bitwise AND).
/// Missing `EffectLayer` on either side means "match everything" (backwards compatible).
#[derive(Component, Clone, Copy, Debug)]
pub struct EffectLayer(pub u32);

impl Default for EffectLayer {
    fn default() -> Self {
        Self::ALL
    }
}

impl EffectLayer {
    pub const ALL: Self = Self(u32::MAX);
    pub const NONE: Self = Self(0);

    /// Create a layer with a single bit set.
    pub fn layer(n: u32) -> Self {
        Self(1 << n)
    }

    /// Add a layer bit to the existing mask.
    pub fn with(self, n: u32) -> Self {
        Self(self.0 | (1 << n))
    }

    /// Check if two layer masks overlap.
    pub fn matches(&self, other: &EffectLayer) -> bool {
        (self.0 & other.0) != 0
    }
}

impl ExtractComponent for EffectLayer {
    type QueryData = &'static EffectLayer;
    type QueryFilter = ();
    type Out = Self;

    fn extract_component(item: &EffectLayer) -> Option<Self::Out> {
        Some(*item)
    }
}

/// Marker component to skip all screen effects on a camera.
///
/// When present on a camera entity, the render node early-returns
/// without applying any effects. Superseded by `EffectLayer` for
/// granular control, but kept for simple on/off toggling.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct SkipScreenEffects;

impl ExtractComponent for SkipScreenEffects {
    type QueryData = &'static SkipScreenEffects;
    type QueryFilter = ();
    type Out = Self;

    fn extract_component(item: &SkipScreenEffects) -> Option<Self::Out> {
        Some(*item)
    }
}
