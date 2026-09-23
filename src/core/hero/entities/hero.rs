//! Hero spawning: game data only. The frontend attaches appearance
//! (sprite, animation clips, camera target) via `Added<Hero>`.

use bevy::prelude::*;

use crate::core::hero::components::hero::Hero;
use crate::core::map::cell_pos::CellPos;
use crate::core::movement::components::position::Position;

/// Starting cell of the hero in the demo room.
const HERO_START: CellPos = CellPos::new(32, 10);

/// Startup system: spawn the hero as pure game data (marker + position at
/// the start cell's center, i.e. integer cell coordinates).
pub fn spawn_hero(mut commands: Commands) {
    commands.spawn((Hero, Position(HERO_START.as_vec2())));
}
