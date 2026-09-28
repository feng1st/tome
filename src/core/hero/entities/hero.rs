//! Hero spawning: game data only. The frontend attaches appearance
//! (sprite, animation clips, camera target) in reaction to the figure
//! handle added here.

use bevy::prelude::*;

use crate::core::figure::resources::figure_registry::FigureRegistry;
use crate::core::hero::components::hero::Hero;
use crate::core::map::types::cell_coord::CellCoord;
use crate::core::movement::components::position::Position;

/// Starting cell of the hero in the test room.
const HERO_START: CellCoord = CellCoord::new(24, 10);

/// The hero's figure: the unarmored warrior. The one place code names a
/// figure id — spawn content is a code constant until creatures move to
/// data files.
const HERO_FIGURE: &str = "warrior";

/// Spawn the hero as pure game data (marker + figure handle + position
/// at the start cell's center, i.e. integer cell coordinates). Nothing
/// despawns on state exit today (the app never leaves `Game`); a
/// cleanup/rebuild strategy arrives with map switching.
pub fn spawn_hero(mut commands: Commands, figure_registry: Res<FigureRegistry>) {
    let figure_index = figure_registry
        .get_index(HERO_FIGURE)
        .expect("warrior is a declared figure");
    commands.spawn((Hero, figure_index, Position::from(HERO_START)));
}
