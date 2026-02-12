//! Digital glitch screen effects.
//!
//! These effects simulate digital artifacts, interference, and corruption.

mod rgb_split;
mod scanline;
mod block_displacement;
mod static_noise;
mod emp;
mod crt;

pub use rgb_split::{RgbSplit, RgbSplitBundle};
pub use scanline::{ScanlineGlitch, ScanlineGlitchBundle};
pub use block_displacement::{BlockDisplacement, BlockDisplacementBundle};
pub use static_noise::{StaticNoise, StaticNoiseBundle};
pub use emp::{EmpInterference, EmpInterferenceBundle};
pub use crt::{CrtEffect, CrtEffectBundle, CrtMaskShape, PhosphorMask};

use bevy::prelude::*;

pub struct GlitchPlugin;

impl Plugin for GlitchPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            rgb_split::RgbSplitPlugin,
            scanline::ScanlinePlugin,
            block_displacement::BlockDisplacementPlugin,
            static_noise::StaticNoisePlugin,
            emp::EmpPlugin,
            crt::CrtPlugin,
        ));
    }
}
