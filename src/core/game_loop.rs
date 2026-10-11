use bevy::prelude::*;

#[derive(SystemSet, Clone, Copy, PartialEq, Eq, Hash, Debug)]

pub enum GameLoop {
    Input,
    Core,
    Display,
}
