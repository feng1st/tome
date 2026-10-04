//! The map currently in play: the resource, its construction from the
//! fixed map file, and the file's parsing and validation — one theme,
//! one file.

use std::fs;

use bevy::prelude::*;

use crate::core::map::resources::terrain_registry::TerrainRegistry;
use crate::core::map::types::local_map::LocalMap;
use crate::core::map::types::map_file::MapFile;
use crate::core::map::types::monster_spawn::MonsterSpawn;

/// The fixed map loaded at startup: the test room.
pub const TEST_ROOM_PATH: &str = "data/maps/test_room.ron";

/// Resource holding the active map. Holds the `LocalMap` directly today;
/// when map switching lands (persistent dungeon levels, world map), it
/// becomes a handle into a map store — readers go through `map()` and are
/// unaffected.
#[derive(Resource)]
pub struct CurrentMap(LocalMap);

impl CurrentMap {
    pub fn new(local_map: LocalMap) -> Self {
        CurrentMap(local_map)
    }

    /// The active map's data.
    pub fn map(&self) -> &LocalMap {
        &self.0
    }
}

impl FromWorld for CurrentMap {
    /// Load the test room, pulling the terrain registry into existence
    /// if it is not built yet — registration order never matters. When
    /// map switching lands, construction stops being a `FromWorld` (the
    /// map to load becomes runtime state) and this impl goes away.
    fn from_world(world: &mut World) -> Self {
        let text = fs::read_to_string(TEST_ROOM_PATH)
            .unwrap_or_else(|e| panic!("cannot read map '{TEST_ROOM_PATH}': {e}"));
        let terrain_registry = world.get_resource_or_init::<TerrainRegistry>();
        CurrentMap::new(parse_local_map(TEST_ROOM_PATH, &text, &terrain_registry))
    }
}

/// Parse and validate map text, resolving legend ids through the
/// registry. Split from file IO (`FromWorld`) so tests can exercise it
/// with inline documents.
pub(crate) fn parse_local_map(
    path: &str,
    text: &str,
    terrain_registry: &TerrainRegistry,
) -> LocalMap {
    let file: MapFile =
        ron::from_str(text).unwrap_or_else(|e| panic!("map '{path}' is not valid RON: {e}"));
    let height = file.rows.len();
    assert!(height > 0, "map '{path}' has no rows");
    let width = file.rows[0].chars().count();
    assert!(width > 0, "map '{path}' has empty rows");

    let mut tiles = Vec::with_capacity(width * height);
    for (y, row) in file.rows.iter().enumerate() {
        assert_eq!(
            row.chars().count(),
            width,
            "map '{path}': row {y} has a different width"
        );
        for (x, ch) in row.chars().enumerate() {
            let Some(id) = file.legend.get(&ch) else {
                panic!("map '{path}': row {y} column {x}: '{ch}' is not in the legend");
            };
            let Some(terrain_index) = terrain_registry.get_index(id) else {
                panic!("map '{path}': legend maps '{ch}' to unknown terrain '{id}'");
            };
            tiles.push(terrain_index);
        }
    }

    // Spawn entries stay raw (the monster id is not resolved here — the
    // map domain does not depend on the monster vocabulary); only the
    // cell needs this domain's knowledge: the map bounds.
    let spawns = file
        .monsters
        .into_iter()
        .map(|entry| {
            let cell = entry.cell;
            assert!(
                cell.x >= 0 && cell.y >= 0 && cell.x < width as i32 && cell.y < height as i32,
                "map '{path}': spawn at ({}, {}) is outside the map",
                cell.x,
                cell.y
            );
            MonsterSpawn {
                monster: entry.monster,
                cell,
            }
        })
        .collect();

    LocalMap {
        width,
        height,
        tiles,
        spawns,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::map::resources::terrain_registry::parse_terrain_registry;

    const TERRAINS: &str = r#"[
        ( terrain: "floor", flags: ["PASSABLE"] ),
        ( terrain: "wall",  flags: [] ),
    ]"#;

    const MAP: &str = r####"(
        legend: { '#': "wall", '.': "floor" },
        rows: [
            "###",
            "#.#",
            "###",
        ],
    )"####;

    fn terrain_registry() -> TerrainRegistry {
        parse_terrain_registry("test", TERRAINS)
    }

    #[test]
    fn dimensions_come_from_rows() {
        let local_map = parse_local_map("test", MAP, &terrain_registry());
        assert_eq!((local_map.width, local_map.height), (3, 3));
    }

    #[test]
    fn characters_resolve_through_legend() {
        let terrain_registry = terrain_registry();
        let local_map = parse_local_map("test", MAP, &terrain_registry);
        let wall = terrain_registry.get_index("wall").unwrap();
        let floor = terrain_registry.get_index("floor").unwrap();
        assert_eq!(local_map.tiles[0], wall);
        assert_eq!(local_map.tiles[4], floor);
    }

    #[test]
    #[should_panic(expected = "row 2 has a different width")]
    fn ragged_rows_panic() {
        let doc = r####"(
            legend: { '#': "wall", '.': "floor" },
            rows: [ "###", "#.#", "##" ],
        )"####;
        parse_local_map("test", doc, &terrain_registry());
    }

    #[test]
    #[should_panic(expected = "row 1 column 1: '?' is not in the legend")]
    fn unmapped_character_panics() {
        let doc = r####"(
            legend: { '#': "wall" },
            rows: [ "###", "#?#", "###" ],
        )"####;
        parse_local_map("test", doc, &terrain_registry());
    }

    #[test]
    #[should_panic(expected = "legend maps '.' to unknown terrain 'lava'")]
    fn unknown_terrain_panics() {
        let doc = r#"(
            legend: { '.': "lava" },
            rows: [ "." ],
        )"#;
        parse_local_map("test", doc, &terrain_registry());
    }

    #[test]
    fn spawn_entries_parse_to_raw_ids_and_cells() {
        use crate::core::map::components::cell_coord::CellCoord;

        let doc = r####"(
            legend: { '#': "wall", '.': "floor" },
            rows: [ "###", "#.#", "###" ],
            monsters: [
                ( monster: "giant_white_rat", cell: ( x: 1, y: 1 ) ),
            ],
        )"####;
        let local_map = parse_local_map("test", doc, &terrain_registry());
        // The monster id stays the raw file string: the map domain does
        // not resolve it against the monster vocabulary.
        assert_eq!(
            local_map.spawns,
            [MonsterSpawn {
                monster: "giant_white_rat".to_string(),
                cell: CellCoord::new(1, 1),
            }]
        );
    }

    #[test]
    fn absent_spawn_section_means_no_spawns() {
        let local_map = parse_local_map("test", MAP, &terrain_registry());
        assert!(local_map.spawns.is_empty());
    }

    #[test]
    #[should_panic(expected = "spawn at (3, 0) is outside the map")]
    fn out_of_bounds_spawn_panics() {
        let doc = r####"(
            legend: { '#': "wall", '.': "floor" },
            rows: [ "###", "#.#", "###" ],
            monsters: [
                ( monster: "giant_white_rat", cell: ( x: 3, y: 0 ) ),
            ],
        )"####;
        parse_local_map("test", doc, &terrain_registry());
    }

    // Spec-alignment test: the real data files on disk reproduce the
    // test room's spatial contract. CWD of `cargo test` is the crate
    // root, so the production paths work as-is.
    #[test]
    fn test_room_data_reproduces_the_room() {
        use crate::core::map::components::cell_coord::CellCoord;
        use crate::core::map::resources::terrain_registry::TERRAIN_TABLE_PATH;
        use crate::core::map::types::local_map::LocalMap;
        use crate::core::map::types::terrain::Terrain;
        use crate::core::map::utils::pathfinding::find_path;

        fn walkable(
            local_map: &LocalMap,
            terrain_registry: &TerrainRegistry,
            cell_coord: CellCoord,
        ) -> bool {
            local_map
                .get(cell_coord)
                .and_then(|terrain_index| terrain_registry.get(terrain_index))
                .is_some_and(Terrain::walkable)
        }

        let text = std::fs::read_to_string(TERRAIN_TABLE_PATH).unwrap();
        let terrain_registry = parse_terrain_registry(TERRAIN_TABLE_PATH, &text);
        let text = std::fs::read_to_string(TEST_ROOM_PATH).unwrap();
        let local_map = parse_local_map(TEST_ROOM_PATH, &text, &terrain_registry);

        // 48x32, larger than the window's world-pixel view.
        assert_eq!((local_map.width, local_map.height), (48, 32));

        // The border is sealed with impassable cells.
        for x in 0..48 {
            for y in [0, 31] {
                assert!(!walkable(
                    &local_map,
                    &terrain_registry,
                    CellCoord::new(x, y)
                ));
            }
        }
        for y in 0..32 {
            for x in [0, 47] {
                assert!(!walkable(
                    &local_map,
                    &terrain_registry,
                    CellCoord::new(x, y)
                ));
            }
        }

        // The pool blocks the straight line between the room's left and
        // right halves, and a detour path exists. (With corner-cutting
        // diagonals the detour keeps the straight line's step count, so
        // blocking is asserted on the line itself, not the length.)
        let start = CellCoord::new(16, 19);
        let goal = CellCoord::new(32, 19);
        assert!(!walkable(
            &local_map,
            &terrain_registry,
            CellCoord::new(24, 19)
        )); // pool center
        let path = find_path(&local_map, &terrain_registry, start, goal)
            .expect("a detour around the pool exists");
        for cell in &path {
            assert!(walkable(&local_map, &terrain_registry, *cell));
        }

        // The player's starting cell (player/entities/player.rs
        // PLAYER_START) is
        // walkable.
        assert!(walkable(
            &local_map,
            &terrain_registry,
            CellCoord::new(24, 10)
        ));

        // Two giant white rats spawn on open floor near the player start.
        let mut spawn_cells: Vec<CellCoord> =
            local_map.spawns.iter().map(|spawn| spawn.cell).collect();
        spawn_cells.sort_by_key(|cell| (cell.x, cell.y));
        assert_eq!(
            spawn_cells,
            [CellCoord::new(24, 13), CellCoord::new(28, 10)]
        );
        for spawn in &local_map.spawns {
            assert_eq!(spawn.monster, "giant_white_rat");
            assert!(walkable(&local_map, &terrain_registry, spawn.cell));
        }
    }
}
