use bevy::prelude::*;

#[derive(Component)]
pub struct AnimTimer {
    pub timer: Timer,
    pub cursor: usize,
}

impl AnimTimer {
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
