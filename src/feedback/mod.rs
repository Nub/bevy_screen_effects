//! Visual feedback screen effects.
//!
//! These effects provide gameplay feedback like damage indication,
//! flash effects, and speed lines.

mod damage_vignette;
mod flash;
mod speed_lines;

pub use damage_vignette::{DamageVignette, DamageVignetteBundle};
pub use flash::{ScreenFlash, ScreenFlashBundle};
pub use speed_lines::{SpeedLines, SpeedLinesBundle};

use bevy::prelude::*;

pub struct FeedbackPlugin;

impl Plugin for FeedbackPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            damage_vignette::DamageVignettePlugin,
            flash::FlashPlugin,
            speed_lines::SpeedLinesPlugin,
        ));
    }
}
