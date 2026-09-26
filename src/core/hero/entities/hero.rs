//! Hero spawning: game data only. The frontend attaches appearance
//! (sprite, animation clips, camera target) via `Added<Hero>`.

use bevy::prelude::*;

use crate::core::appearance::components::appearance_kind::AppearanceKind;
use crate::core::hero::components::hero::Hero;
use crate::core::map::constants::layout::MAP_W;
use crate::core::map::types::cell_coord::CellCoord;
use crate::core::movement::components::position::Position;

/// Starting cell of the hero in the demo room.
const HERO_START: CellCoord = CellCoord::new(MAP_W as i32 / 2, 10);

/// Spawn the hero as pure game data (marker + position at the start
/// cell's center, i.e. integer cell coordinates). `DespawnOnExit` ties the
/// hero to `Game`: leaving the state despawns it, so re-entering via
/// `OnEnter` is idempotent by construction.
pub fn spawn_hero(mut commands: Commands) {
    commands.spawn((Hero, AppearanceKind::Warrior, Position::from(HERO_START)));
}
