//! Effect lifetime and timing management.

use bevy::prelude::*;
use crate::effect::{EffectIntensity, ScreenEffect};

pub struct LifetimePlugin;

impl Plugin for LifetimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_lifetimes, despawn_expired).chain());
    }
}

/// Controls the lifetime and intensity curve of an effect.
#[derive(Component, Clone)]
pub struct EffectLifetime {
    /// Total duration in seconds.
    pub duration: f32,
    /// Time spent fading in (included in duration).
    pub fade_in: f32,
    /// Time spent fading out (included in duration).
    pub fade_out: f32,
    /// Easing function for intensity.
    pub easing: EasingFunction,
    /// Current elapsed time.
    elapsed: f32,
}

impl Default for EffectLifetime {
    fn default() -> Self {
        Self {
            duration: 1.0,
            fade_in: 0.1,
            fade_out: 0.3,
            easing: EasingFunction::Linear,
            elapsed: 0.0,
        }
    }
}

impl EffectLifetime {
    /// Create a new lifetime with the given duration.
    pub fn new(duration: f32) -> Self {
        Self {
            duration,
            fade_out: duration * 0.3,
            ..default()
        }
    }

    /// Set fade in/out times.
    pub fn with_fades(mut self, fade_in: f32, fade_out: f32) -> Self {
        self.fade_in = fade_in;
        self.fade_out = fade_out;
        self
    }

    /// Set easing function.
    pub fn with_easing(mut self, easing: EasingFunction) -> Self {
        self.easing = easing;
        self
    }

    /// Get normalized progress (0.0 to 1.0).
    pub fn progress(&self) -> f32 {
        (self.elapsed / self.duration).clamp(0.0, 1.0)
    }

    /// Check if the effect has expired.
    pub fn is_expired(&self) -> bool {
        self.elapsed >= self.duration
    }

    /// Calculate current intensity based on fade curves.
    pub fn intensity(&self) -> f32 {
        let t = self.elapsed;
        let d = self.duration;

        let raw = if t < self.fade_in {
            // Fading in
            t / self.fade_in
        } else if t > d - self.fade_out {
            // Fading out
            (d - t) / self.fade_out
        } else {
            // Full intensity
            1.0
        };

        self.easing.apply(raw.clamp(0.0, 1.0))
    }

    fn tick(&mut self, delta: f32) {
        self.elapsed += delta;
    }
}

/// Easing functions for effect intensity.
#[derive(Clone, Copy, Default)]
pub enum EasingFunction {
    #[default]
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    /// Overshoots then settles - good for impact effects.
    Elastic,
    /// Bounces at the end - good for playful effects.
    Bounce,
}

impl EasingFunction {
    pub fn apply(&self, t: f32) -> f32 {
        match self {
            Self::Linear => t,
            Self::EaseIn => t * t,
            Self::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            Self::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
            Self::Elastic => {
                if t == 0.0 || t == 1.0 {
                    t
                } else {
                    let p = 0.3;
                    (2.0_f32).powf(-10.0 * t)
                        * ((t - p / 4.0) * std::f32::consts::TAU / p).sin()
                        + 1.0
                }
            }
            Self::Bounce => {
                let n1 = 7.5625;
                let d1 = 2.75;
                let mut t = t;

                if t < 1.0 / d1 {
                    n1 * t * t
                } else if t < 2.0 / d1 {
                    t -= 1.5 / d1;
                    n1 * t * t + 0.75
                } else if t < 2.5 / d1 {
                    t -= 2.25 / d1;
                    n1 * t * t + 0.9375
                } else {
                    t -= 2.625 / d1;
                    n1 * t * t + 0.984375
                }
            }
        }
    }
}

fn update_lifetimes(
    time: Res<Time>,
    mut query: Query<(&mut EffectLifetime, &mut EffectIntensity), With<ScreenEffect>>,
) {
    let delta = time.delta_secs();
    for (mut lifetime, mut intensity) in &mut query {
        lifetime.tick(delta);
        intensity.set(lifetime.intensity());
    }
}

fn despawn_expired(
    mut commands: Commands,
    query: Query<(Entity, &EffectLifetime), With<ScreenEffect>>,
) {
    for (entity, lifetime) in &query {
        if lifetime.is_expired() {
            commands.entity(entity).despawn();
        }
    }
}
