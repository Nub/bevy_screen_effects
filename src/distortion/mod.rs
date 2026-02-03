//! Distortion-based screen effects.
//!
//! These effects warp the screen image by displacing pixels.

mod shockwave;
mod radial_blur;
mod water_drops;
mod heat_haze;

pub use shockwave::{Shockwave, ShockwaveBundle, WorldShockwave, WorldShockwaveBundle};
pub use radial_blur::{RadialBlur, RadialBlurBundle};
pub use water_drops::{Raindrops, RaindropsBundle};
pub use heat_haze::{HeatHaze, HeatHazeBundle, WorldHeatShimmer, WorldHeatShimmerBundle};

use bevy::prelude::*;

pub struct DistortionPlugin;

impl Plugin for DistortionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            shockwave::ShockwavePlugin,
            radial_blur::RadialBlurPlugin,
            water_drops::RaindropsPlugin,
            heat_haze::HeatHazePlugin,
        ));
    }
}
