use bevy::prelude::*;

use crate::core::hit_points::components::hit_points::HitPoints;
use crate::core::map::components::cell_coord::CellCoord;
use crate::core::map::resources::current_map::CurrentMap;
use crate::core::map::resources::terrain_registry::TerrainRegistry;
use crate::core::map::types::terrain::Terrain;
use crate::core::monster::components::monster_index::MonsterIndex;
use crate::core::player::commands::target_cell::TargetCell;
use crate::core::player::components::order::Order;
use crate::core::player::components::player::Player;

pub fn target_cell(
    mut commands: Commands,
    mut target_cell_reader: MessageReader<TargetCell>,
    current_map: Res<CurrentMap>,
    terrain_registry: Res<TerrainRegistry>,
    monsters: Query<(Entity, &CellCoord, &HitPoints), With<MonsterIndex>>,
    player: Query<(Entity, &CellCoord, &HitPoints), With<Player>>,
) {
    let Ok((player, &player_cell_coord, player_hit_points)) = player.single() else {
        return;
    };
    if player_hit_points.is_dead() {
        return;
    }
    let local_map = current_map.local_map();
    for target_cell in target_cell_reader.read() {
        let target_cell_coord = target_cell.0;
        if target_cell_coord == player_cell_coord {
            continue;
        }
        if let Some((monster, _, _)) = monsters.iter().find(|(_, cell_coord, hit_points)| {
            **cell_coord == target_cell_coord && !hit_points.is_dead()
        }) {
            commands
                .entity(player)
                .insert(Order::Attack { target: monster });
            continue;
        }
        let target_walkable = local_map
            .get(target_cell_coord)
            .and_then(|terrain_index| terrain_registry.get(terrain_index))
            .is_some_and(Terrain::is_walkable);
        if !target_walkable {
            continue;
        }
        commands.entity(player).insert(Order::Move {
            target: target_cell_coord,
        });
    }
}
