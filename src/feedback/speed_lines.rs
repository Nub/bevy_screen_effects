//! Speed lines effect.
//!
//! Manga/anime-style motion lines radiating from a focus point.

use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;

use crate::effect::{ScreenEffect, EffectIntensity};
use crate::lifetime::EffectLifetime;

pub struct SpeedLinesPlugin;

impl Plugin for SpeedLinesPlugin {
    fn build(&self, _app: &mut App) {}
}

/// Speed lines effect.
#[derive(Component, Clone, ExtractComponent)]
pub struct SpeedLines {
    /// Focus point (lines radiate from here).
    pub focus: Vec2,
    /// Line color.
    pub color: Color,
    /// Number of lines.
    pub line_count: u32,
    /// Line thickness.
    pub thickness: f32,
    /// How far lines extend from focus (0.0 to 1.0).
    pub length: f32,
    /// Animation speed.
    pub speed: f32,
}

impl Default for SpeedLines {
    fn default() -> Self {
        Self {
            focus: Vec2::new(0.5, 0.5),
            color: Color::WHITE,
            line_count: 32,
            thickness: 0.002,
            length: 0.5,
            speed: 10.0,
        }
    }
}

impl SpeedLines {
    /// Create with focus at screen center.
    pub fn centered() -> Self {
        Self::default()
    }

    /// Create with focus at a specific point.
    pub fn at(x: f32, y: f32) -> Self {
        Self {
            focus: Vec2::new(x, y),
            ..default()
        }
    }
}

#[derive(Bundle, Default)]
pub struct SpeedLinesBundle {
    pub speed_lines: SpeedLines,
    pub effect: ScreenEffect,
    pub intensity: EffectIntensity,
    pub lifetime: EffectLifetime,
}
