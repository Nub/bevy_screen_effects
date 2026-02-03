//! RGB channel split / chromatic aberration effect.

use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;

use crate::effect::{ScreenEffect, EffectIntensity};
use crate::lifetime::EffectLifetime;

pub struct RgbSplitPlugin;

impl Plugin for RgbSplitPlugin {
    fn build(&self, _app: &mut App) {}
}

/// RGB channel split effect.
#[derive(Component, Clone, ExtractComponent)]
pub struct RgbSplit {
    /// Red channel offset.
    pub red_offset: Vec2,
    /// Green channel offset (usually zero).
    pub green_offset: Vec2,
    /// Blue channel offset.
    pub blue_offset: Vec2,
    /// Whether offsets should animate/jitter.
    pub animated: bool,
}

impl Default for RgbSplit {
    fn default() -> Self {
        Self {
            red_offset: Vec2::new(-0.01, 0.0),
            green_offset: Vec2::ZERO,
            blue_offset: Vec2::new(0.01, 0.0),
            animated: true,
        }
    }
}

impl RgbSplit {
    /// Create with a simple horizontal split.
    pub fn horizontal(amount: f32) -> Self {
        Self {
            red_offset: Vec2::new(-amount, 0.0),
            green_offset: Vec2::ZERO,
            blue_offset: Vec2::new(amount, 0.0),
            animated: false,
        }
    }

    /// Create with a diagonal split.
    pub fn diagonal(amount: f32) -> Self {
        Self {
            red_offset: Vec2::new(-amount, -amount * 0.5),
            green_offset: Vec2::ZERO,
            blue_offset: Vec2::new(amount, amount * 0.5),
            animated: false,
        }
    }
}

#[derive(Bundle, Default)]
pub struct RgbSplitBundle {
    pub rgb_split: RgbSplit,
    pub effect: ScreenEffect,
    pub intensity: EffectIntensity,
    pub lifetime: EffectLifetime,
}
