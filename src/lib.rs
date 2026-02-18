//! # Bevy Screen Effects
//!
//! Dynamic screen space effects for games - shockwaves, glitches, radial blur, and more.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use bevy::prelude::*;
//! use bevy_screen_effects::prelude::*;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugins(ScreenEffectsPlugin)
//!         .add_systems(Update, spawn_effects)
//!         .run();
//! }
//!
//! fn spawn_effects(mut commands: Commands, input: Res<ButtonInput<KeyCode>>) {
//!     if input.just_pressed(KeyCode::Space) {
//!         commands.spawn(ShockwaveBundle {
//!             shockwave: Shockwave {
//!                 center: Vec2::new(0.5, 0.5), // normalized screen coords
//!                 intensity: 0.3,
//!                 ..default()
//!             },
//!             lifetime: EffectLifetime::new(0.5),
//!             ..default()
//!         });
//!     }
//! }
//! ```

mod effect;
mod lifetime;
mod render;

#[cfg(feature = "distortion")]
pub mod distortion;

#[cfg(feature = "glitch")]
pub mod glitch;

#[cfg(feature = "feedback")]
pub mod feedback;

pub mod prelude {
    pub use crate::effect::{ScreenEffect, EffectIntensity, EffectTarget, SkipScreenEffects};
    pub use crate::lifetime::{EffectLifetime, EasingFunction};
    pub use crate::ScreenEffectsPlugin;

    #[cfg(feature = "distortion")]
    pub use crate::distortion::*;

    #[cfg(feature = "glitch")]
    pub use crate::glitch::*;

    #[cfg(feature = "feedback")]
    pub use crate::feedback::*;
}

use bevy::prelude::*;

pub struct ScreenEffectsPlugin;

impl Plugin for ScreenEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(lifetime::LifetimePlugin)
            .add_plugins(render::ScreenEffectsRenderPlugin);

        #[cfg(feature = "distortion")]
        app.add_plugins(distortion::DistortionPlugin);

        #[cfg(feature = "glitch")]
        app.add_plugins(glitch::GlitchPlugin);

        #[cfg(feature = "feedback")]
        app.add_plugins(feedback::FeedbackPlugin);
    }
}
