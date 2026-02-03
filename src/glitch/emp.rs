//! EMP/Electronic interference effect.
//!
//! Simulates electromagnetic pulse interference with flickering,
//! color banding, static bursts, and scan line disruption.

use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;

use crate::effect::{ScreenEffect, EffectIntensity};
use crate::lifetime::EffectLifetime;

pub struct EmpPlugin;

impl Plugin for EmpPlugin {
    fn build(&self, _app: &mut App) {
        // Rendering is handled by ScreenEffectsRenderPlugin
    }
}

/// EMP/Electronic interference effect component.
///
/// Creates an electromagnetic interference look with:
/// - Screen flickering
/// - Horizontal color bands that shift
/// - Static noise bursts
/// - Scan line displacement
/// - Color channel separation
#[derive(Component, Clone, ExtractComponent)]
pub struct EmpInterference {
    /// Flicker frequency (higher = faster flashing).
    pub flicker_rate: f32,
    /// Flicker intensity (0.0 - 1.0).
    pub flicker_strength: f32,
    /// Number of color bands across the screen.
    pub band_count: f32,
    /// How much the bands shift colors.
    pub band_intensity: f32,
    /// Speed at which bands scroll.
    pub band_speed: f32,
    /// Static noise intensity (0.0 - 1.0).
    pub static_intensity: f32,
    /// Probability of static bursts (0.0 - 1.0).
    pub burst_probability: f32,
    /// Scan line displacement amount.
    pub scanline_displacement: f32,
    /// RGB channel separation amount.
    pub chromatic_amount: f32,
}

impl Default for EmpInterference {
    fn default() -> Self {
        Self {
            flicker_rate: 30.0,
            flicker_strength: 0.3,
            band_count: 8.0,
            band_intensity: 0.4,
            band_speed: 2.0,
            static_intensity: 0.2,
            burst_probability: 0.1,
            scanline_displacement: 0.02,
            chromatic_amount: 0.01,
        }
    }
}

impl EmpInterference {
    /// Light interference - subtle electronic glitching.
    pub fn light() -> Self {
        Self {
            flicker_rate: 15.0,
            flicker_strength: 0.15,
            band_count: 4.0,
            band_intensity: 0.2,
            band_speed: 1.0,
            static_intensity: 0.1,
            burst_probability: 0.05,
            scanline_displacement: 0.01,
            chromatic_amount: 0.005,
        }
    }

    /// Heavy interference - strong EMP effect.
    pub fn heavy() -> Self {
        Self {
            flicker_rate: 45.0,
            flicker_strength: 0.5,
            band_count: 12.0,
            band_intensity: 0.6,
            band_speed: 4.0,
            static_intensity: 0.4,
            burst_probability: 0.2,
            scanline_displacement: 0.04,
            chromatic_amount: 0.02,
        }
    }

    /// Critical interference - severe disruption.
    pub fn critical() -> Self {
        Self {
            flicker_rate: 60.0,
            flicker_strength: 0.7,
            band_count: 16.0,
            band_intensity: 0.8,
            band_speed: 6.0,
            static_intensity: 0.6,
            burst_probability: 0.35,
            scanline_displacement: 0.06,
            chromatic_amount: 0.03,
        }
    }

    /// Radio static - more noise, less banding.
    pub fn radio_static() -> Self {
        Self {
            flicker_rate: 20.0,
            flicker_strength: 0.4,
            band_count: 2.0,
            band_intensity: 0.1,
            band_speed: 0.5,
            static_intensity: 0.5,
            burst_probability: 0.3,
            scanline_displacement: 0.01,
            chromatic_amount: 0.005,
        }
    }

    /// Builder: set flicker parameters.
    pub fn with_flicker(mut self, rate: f32, strength: f32) -> Self {
        self.flicker_rate = rate;
        self.flicker_strength = strength.clamp(0.0, 1.0);
        self
    }

    /// Builder: set color band parameters.
    pub fn with_bands(mut self, count: f32, intensity: f32, speed: f32) -> Self {
        self.band_count = count;
        self.band_intensity = intensity.clamp(0.0, 1.0);
        self.band_speed = speed;
        self
    }

    /// Builder: set static noise parameters.
    pub fn with_static(mut self, intensity: f32, burst_prob: f32) -> Self {
        self.static_intensity = intensity.clamp(0.0, 1.0);
        self.burst_probability = burst_prob.clamp(0.0, 1.0);
        self
    }

    /// Builder: set chromatic aberration amount.
    pub fn with_chromatic(mut self, amount: f32) -> Self {
        self.chromatic_amount = amount;
        self
    }

    /// Builder: set scanline displacement.
    pub fn with_scanline_displacement(mut self, amount: f32) -> Self {
        self.scanline_displacement = amount;
        self
    }
}

/// Bundle for spawning EMP interference effect.
#[derive(Bundle, Default)]
pub struct EmpInterferenceBundle {
    pub emp: EmpInterference,
    pub effect: ScreenEffect,
    pub intensity: EffectIntensity,
    pub lifetime: EffectLifetime,
}
