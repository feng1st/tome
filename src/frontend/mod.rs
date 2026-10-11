pub mod display;
pub mod input;

use bevy::prelude::*;

pub fn register(app: &mut App) {
    display::register(app);
    input::register(app);
}
