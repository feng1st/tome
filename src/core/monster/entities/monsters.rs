//! Monster spawning: game data only. The frontend attaches appearance
//! (sprite, animation) in reaction to the race handle added here.

use bevy::prelude::*;

use crate::core::creature::resources::unique_registry::UniqueRegistry;
use crate::core::map::resources::current_map::CurrentMap;
use crate::core::monster::resources::monster_registry::MonsterRegistry;
use crate::core::movement::components::position::Position;

/// Spawn every monster declared by the current map's spawn table, as pure
/// game data (kind handle + race handle + optional class and unique
/// handles + position at the cell's center). Monster ids resolve here,
/// not at map load: the map domain does not depend on this vocabulary
/// (positions flow the other way), so an unknown id surfaces at spawn —
/// still startup, the same launch. Nothing despawns on state exit today
/// (the app never leaves `Game`); a cleanup/rebuild strategy arrives
/// with map switching.
pub fn spawn_monsters(
    mut commands: Commands,
    current_map: Res<CurrentMap>,
    monster_registry: Res<MonsterRegistry>,
    unique_registry: Res<UniqueRegistry>,
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
        let monster_kind = monster_registry.monster_kind(monster_index);
        let race_index = monster_kind.race;
        let mut entity = commands.spawn((monster_index, race_index, Position::from(spawn.cell)));
        if let Some(class_index) = monster_kind.class {
            entity.insert(class_index);
        }
        if let Some(unique_id) = &monster_kind.unique_id {
            // Aggregated from the content files at app build time.
            let unique_index = unique_registry
                .get_index(unique_id)
                .expect("declared unique ids are aggregated at load");
            entity.insert(unique_index);
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use super::*;
    use crate::core::creature::components::class_index::ClassIndex;
    use crate::core::creature::components::race_index::RaceIndex;
    use crate::core::creature::components::unique_index::UniqueIndex;
    use crate::core::creature::resources::class_registry::parse_class_registry;
    use crate::core::creature::resources::race_registry::parse_race_registry;
    use crate::core::map::types::cell_coord::CellCoord;
    use crate::core::map::types::local_map::LocalMap;
    use crate::core::map::types::monster_spawn::MonsterSpawn;
    use crate::core::monster::components::monster_index::MonsterIndex;
    use crate::core::monster::resources::monster_registry::parse_monster_registry;

    #[test]
    fn spawns_carry_kind_identity_and_position() {
        let race_registry = parse_race_registry(
            "test",
            r#"[ ( race: "dog" ), ( race: "giant_white_rat" ) ]"#,
        );
        let class_registry = parse_class_registry("test", r#"[ ( class: "warrior" ) ]"#);
        let monster_registry = parse_monster_registry(
            "test",
            r#"[
                ( monster: "giant_white_rat", race: "giant_white_rat" ),
                ( monster: "grip", race: "dog", class: "warrior", unique_id: "grip" ),
            ]"#,
            &race_registry,
            &class_registry,
        );
        let rat_handle = monster_registry.get_index("giant_white_rat").unwrap();
        let grip_handle = monster_registry.get_index("grip").unwrap();
        let expected_rat_race = race_registry.get_index("giant_white_rat").unwrap();
        let expected_grip_race = race_registry.get_index("dog").unwrap();
        let expected_class = class_registry.get_index("warrior").unwrap();
        let mut unique_registry = UniqueRegistry::empty();
        unique_registry.extend(vec!["grip".to_string()]);
        let expected_unique = unique_registry.get_index("grip").unwrap();
        let local_map = LocalMap {
            width: 2,
            height: 1,
            tiles: vec![],
            spawns: vec![
                MonsterSpawn {
                    monster: "giant_white_rat".to_string(),
                    cell: CellCoord::new(0, 0),
                },
                MonsterSpawn {
                    monster: "grip".to_string(),
                    cell: CellCoord::new(1, 0),
                },
            ],
        };

        let mut world = World::new();
        world.insert_resource(CurrentMap::new(local_map));
        world.insert_resource(monster_registry);
        world.insert_resource(unique_registry);
        world.run_system_once(spawn_monsters).unwrap();

        // The rat carries kind + race only; the unique carries class and
        // unique handles on top.
        let mut query = world.query::<(
            &MonsterIndex,
            &RaceIndex,
            Option<&ClassIndex>,
            Option<&UniqueIndex>,
        )>();
        let entities: Vec<_> = query.iter(&world).collect();
        assert_eq!(entities.len(), 2);
        let rat = entities.iter().find(|(m, ..)| **m == rat_handle).unwrap();
        assert_eq!(*rat.1, expected_rat_race);
        assert!(rat.2.is_none() && rat.3.is_none());
        let grip = entities.iter().find(|(m, ..)| **m == grip_handle).unwrap();
        assert_eq!(*grip.1, expected_grip_race);
        assert_eq!(grip.2.copied(), Some(expected_class));
        assert_eq!(grip.3.copied(), Some(expected_unique));
    }
}
