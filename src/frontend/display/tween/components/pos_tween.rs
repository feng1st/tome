use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
pub struct PosTween {
    pub from: Vec2,
    pub to: Vec2,
    pub elapsed: f32,
    pub interval: f32,
}
