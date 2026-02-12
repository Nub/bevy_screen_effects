//! CRT screen effect.
//!
//! Simulates cathode ray tube display characteristics including barrel distortion,
//! scanlines, phosphor mask patterns, bloom, color bleed, vignette, and flicker.

use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;

use crate::effect::{ScreenEffect, EffectIntensity};
use crate::lifetime::EffectLifetime;

pub struct CrtPlugin;

impl Plugin for CrtPlugin {
    fn build(&self, _app: &mut App) {
        // Rendering is handled by ScreenEffectsRenderPlugin
    }
}

/// Phosphor mask type for CRT sub-pixel simulation.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum PhosphorMask {
    #[default]
    None,
    /// Dot triad pattern (most common on TVs).
    ShadowMask,
    /// Vertical RGB stripes (high-end monitors, Sony Trinitron).
    ApertureGrille,
    /// 2D repeating grid (slot-mask tubes).
    SlotMask,
}

impl PhosphorMask {
    fn as_u32(self) -> u32 {
        match self {
            PhosphorMask::None => 0,
            PhosphorMask::ShadowMask => 1,
            PhosphorMask::ApertureGrille => 2,
            PhosphorMask::SlotMask => 3,
        }
    }
}

/// Screen mask shape for the CRT border.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum CrtMaskShape {
    /// Rounded rectangle (classic TV shape).
    #[default]
    RoundedRect,
    /// Elliptical mask (round CRT / oscilloscope style).
    Ellipse,
}

impl CrtMaskShape {
    fn as_u32(self) -> u32 {
        match self {
            CrtMaskShape::RoundedRect => 0,
            CrtMaskShape::Ellipse => 1,
        }
    }
}

/// CRT screen effect component.
///
/// Simulates the look of a cathode ray tube display with configurable:
/// - Barrel distortion and rounded corners
/// - Scanlines and phosphor mask patterns
/// - Bloom, color bleed, and vignette
/// - Screen flicker and color grading
#[derive(Component, Clone, ExtractComponent)]
pub struct CrtEffect {
    /// Scanline darkness (0.0 = no scanlines, 1.0 = fully dark between lines).
    pub scanline_intensity: f32,
    /// Number of scanlines across screen height.
    pub scanline_count: f32,
    /// Barrel distortion amount (0.0 = flat, 0.3 = heavy curve).
    pub curvature: f32,
    /// Size of rounded black corners (0.0 = sharp, 0.1 = very rounded).
    pub corner_radius: f32,
    /// Screen mask shape (rounded rectangle or ellipse).
    pub mask_shape: CrtMaskShape,
    /// Phosphor mask type.
    pub phosphor: PhosphorMask,
    /// Phosphor mask visibility (0.0 = invisible, 1.0 = very pronounced).
    pub phosphor_intensity: f32,
    /// Bloom/glow amount for bright areas.
    pub bloom: f32,
    /// Edge vignette darkness.
    pub vignette: f32,
    /// Screen flicker amount (0.0 = stable, subtle values like 0.02 are realistic).
    pub flicker: f32,
    /// Horizontal color bleeding.
    pub color_bleed: f32,
    /// Brightness boost (1.0 = no change, 1.2 = brighter).
    pub brightness: f32,
    /// Color saturation (1.0 = no change, 1.3 = more saturated).
    pub saturation: f32,
}

impl Default for CrtEffect {
    fn default() -> Self {
        Self::retro_gaming()
    }
}

impl CrtEffect {
    /// Classic 90s arcade monitor - heavy scanlines, aperture grille, slight curve.
    pub fn arcade() -> Self {
        Self {
            scanline_intensity: 0.4,
            scanline_count: 240.0,
            curvature: 0.08,
            corner_radius: 0.03,
            mask_shape: CrtMaskShape::RoundedRect,
            phosphor: PhosphorMask::ApertureGrille,
            phosphor_intensity: 0.3,
            bloom: 0.15,
            vignette: 0.3,
            flicker: 0.01,
            color_bleed: 0.002,
            brightness: 1.2,
            saturation: 1.3,
        }
    }

    /// Old living room TV - shadow mask, strong curvature, heavy vignette, some flicker.
    pub fn old_tv() -> Self {
        Self {
            scanline_intensity: 0.35,
            scanline_count: 200.0,
            curvature: 0.15,
            corner_radius: 0.05,
            mask_shape: CrtMaskShape::Ellipse,
            phosphor: PhosphorMask::ShadowMask,
            phosphor_intensity: 0.25,
            bloom: 0.2,
            vignette: 0.5,
            flicker: 0.03,
            color_bleed: 0.003,
            brightness: 1.1,
            saturation: 1.2,
        }
    }

    /// Clean retro gaming look - subtle scanlines, mild curve, no phosphor.
    pub fn retro_gaming() -> Self {
        Self {
            scanline_intensity: 0.2,
            scanline_count: 240.0,
            curvature: 0.04,
            corner_radius: 0.02,
            mask_shape: CrtMaskShape::RoundedRect,
            phosphor: PhosphorMask::None,
            phosphor_intensity: 0.0,
            bloom: 0.1,
            vignette: 0.2,
            flicker: 0.0,
            color_bleed: 0.001,
            brightness: 1.1,
            saturation: 1.1,
        }
    }

    pub fn phosphor_type_u32(&self) -> u32 {
        self.phosphor.as_u32()
    }

    pub fn mask_shape_u32(&self) -> u32 {
        self.mask_shape.as_u32()
    }
}

/// Bundle for spawning a CRT screen effect.
#[derive(Bundle, Default)]
pub struct CrtEffectBundle {
    pub crt: CrtEffect,
    pub effect: ScreenEffect,
    pub intensity: EffectIntensity,
    pub lifetime: EffectLifetime,
}
