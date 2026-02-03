//! Screen flash effect.
//!
//! Full-screen flash for impacts, flashbangs, or transitions.

use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;

use crate::effect::{ScreenEffect, EffectIntensity};
use crate::lifetime::EffectLifetime;

pub struct FlashPlugin;

impl Plugin for FlashPlugin {
    fn build(&self, _app: &mut App) {}
}

/// Screen flash effect.
#[derive(Component, Clone, ExtractComponent)]
pub struct ScreenFlash {
    /// Flash color.
    pub color: Color,
    /// Blend mode (0.0 = additive, 1.0 = replace).
    pub blend: f32,
}

impl Default for ScreenFlash {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            blend: 0.0, // Additive by default
        }
    }
}

impl ScreenFlash {
    /// Pure white flash (flashbang style).
    pub fn white() -> Self {
        Self {
            color: Color::WHITE,
            blend: 1.0,
        }
    }

    /// Impact flash (brief, additive).
    pub fn impact() -> Self {
        Self {
            color: Color::srgba(1.0, 0.9, 0.8, 0.3),
            blend: 0.0,
        }
    }

    /// Custom color flash.
    pub fn with_color(color: Color) -> Self {
        Self {
            color,
            ..default()
        }
    }
}

#[derive(Bundle)]
pub struct ScreenFlashBundle {
    pub flash: ScreenFlash,
    pub effect: ScreenEffect,
    pub intensity: EffectIntensity,
    pub lifetime: EffectLifetime,
}

impl Default for ScreenFlashBundle {
    fn default() -> Self {
        Self {
            flash: ScreenFlash::default(),
            effect: ScreenEffect,
            intensity: EffectIntensity::default(),
            lifetime: EffectLifetime::new(0.15).with_fades(0.0, 0.15),
        }
    }
}
