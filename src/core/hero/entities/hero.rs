//! Hero spawning: game data only. The frontend attaches appearance
//! (sprite, animation clips, camera target) via `Added<Hero>`.

use bevy::prelude::*;

use crate::core::app_state::AppState;
use crate::core::hero::components::hero::Hero;
use crate::core::map::types::cell_coord::CellCoord;
use crate::core::movement::components::position::Position;

/// Starting cell of the hero in the demo room.
const HERO_START: CellCoord = CellCoord::new(32, 10);

/// Spawn the hero as pure game data (marker + position at the start
/// cell's center, i.e. integer cell coordinates). `DespawnOnExit` ties the
/// hero to `InGame`: leaving the state despawns it, so re-entering via
/// `OnEnter` is idempotent by construction.
pub fn spawn_hero(mut commands: Commands) {
    commands.spawn((
        Hero,
        Position::from(HERO_START),
        DespawnOnExit(AppState::InGame),
    ));
}
