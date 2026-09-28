//! Monster spawning: game data only. The frontend attaches appearance
//! (sprite, animation clips) in reaction to the kind handle added here.

use bevy::prelude::*;

use crate::core::map::resources::current_map::CurrentMap;
use crate::core::monster::resources::monster_registry::MonsterRegistry;
use crate::core::movement::components::position::Position;

/// Spawn every monster declared by the current map's spawn table, as pure
/// game data (kind handle + position at the cell's center). Monster ids
/// resolve here, not at map load: the map domain does not depend on this
/// vocabulary (positions flow the other way), so an unknown id surfaces
/// at spawn — still startup, the same launch. Nothing despawns on state
/// exit today (the app never leaves `Game`); a cleanup/rebuild strategy
/// arrives with map switching.
pub fn spawn_monsters(
    mut commands: Commands,
    current_map: Res<CurrentMap>,
    monster_registry: Res<MonsterRegistry>,
) {
    for spawn in &current_map.map().spawns {
        let monster_index = monster_registry
            .get_index(&spawn.monster)
            .unwrap_or_else(|| {
                panic!(
                    "map declares unknown monster '{}' at ({}, {})",
                    spawn.monster, spawn.cell.x, spawn.cell.y
                )
            });
        commands.spawn((monster_index, Position::from(spawn.cell)));
    }
}
