//! Per-entity frame-advance timer.

use bevy::prelude::*;

/// Drives frame advancement for the entity's current clip. `fps` is
/// per-clip, so one timer is shared across states and reset on switches.

#[derive(Component)]
pub struct AnimTimer {
    pub timer: Timer,
    pub cursor: usize,
}

impl AnimTimer {
    /// `fps` is per-animation-clip; `set_fps` is called on every state switch
    /// so a slow idle and a fast run share one timer.
    pub fn new(fps: f32) -> Self {
        AnimTimer {
            timer: Timer::from_seconds(1.0 / fps, TimerMode::Repeating),
            cursor: 0,
        }
    }

    pub fn set_fps(&mut self, fps: f32) {
        self.timer = Timer::from_seconds(1.0 / fps, TimerMode::Repeating);
        self.timer.reset();
        self.cursor = 0;
    }
}
