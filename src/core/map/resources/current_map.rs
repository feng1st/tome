use std::fs;

use bevy::prelude::*;

use crate::core::map::resources::terrain_registry::TerrainRegistry;
use crate::core::map::types::local_map::LocalMap;
use crate::core::map::types::map_file::MapFile;
use crate::core::map::types::monster_spawn::MonsterSpawn;

pub const TEST_ROOM_PATH: &str = "data/maps/test_room.ron";

#[derive(Resource)]
pub struct CurrentMap(LocalMap);

impl CurrentMap {
    pub fn local_map(&self) -> &LocalMap {
        &self.0
    }
}

impl FromWorld for CurrentMap {
    fn from_world(world: &mut World) -> Self {
        let test_room_content = fs::read_to_string(TEST_ROOM_PATH)
            .unwrap_or_else(|e| panic!("cannot read map '{TEST_ROOM_PATH}': {e}"));
        let terrain_registry = world.get_resource_or_init::<TerrainRegistry>();
        CurrentMap(parse_test_room(
            TEST_ROOM_PATH,
            &test_room_content,
            &terrain_registry,
        ))
    }
}

pub(crate) fn parse_test_room(
    path: &str,
    test_room_content: &str,
    terrain_registry: &TerrainRegistry,
) -> LocalMap {
    let map_file: MapFile = ron::from_str(test_room_content)
        .unwrap_or_else(|e| panic!("map '{path}' is not valid RON: {e}"));
    let rows = map_file.rows.len();
    assert!(rows > 0, "map '{path}' has no rows");
    let columns = map_file.rows[0].chars().count();
    assert!(columns > 0, "map '{path}' has empty rows");

    let mut tiles = Vec::with_capacity(columns * rows);
    for (y, row) in map_file.rows.iter().enumerate() {
        assert_eq!(
            row.chars().count(),
            columns,
            "map '{path}': row {y} has a different width"
        );
        for (x, ch) in row.chars().enumerate() {
            let Some(id) = map_file.legend.get(&ch) else {
                panic!("map '{path}': row {y} column {x}: '{ch}' is not in the legend");
            };
            let Some(terrain_index) = terrain_registry.get_index(id) else {
                panic!("map '{path}': legend maps '{ch}' to unknown terrain '{id}'");
            };
            tiles.push(terrain_index);
        }
    }

    let spawns = map_file
        .monsters
        .into_iter()
        .map(|entry| {
            let cell_coord = entry.cell;
            assert!(
                cell_coord.x >= 0
                    && cell_coord.y >= 0
                    && cell_coord.x < columns as i32
                    && cell_coord.y < rows as i32,
                "map '{path}': spawn at ({}, {}) is outside the map",
                cell_coord.x,
                cell_coord.y
            );
            MonsterSpawn {
                monster: entry.monster,
                cell_coord,
            }
        })
        .collect();

    LocalMap {
        columns,
        rows,
        tiles,
        spawns,
    }
}
