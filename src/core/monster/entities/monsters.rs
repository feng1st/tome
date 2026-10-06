//! Monster spawning: game data only. The frontend attaches appearance
//! (sprite, animation) in reaction to the kind handle added here.

use bevy::prelude::*;

use crate::core::dice::utils::roll::roll_with;
use crate::core::health::components::hit_points::HitPoints;
use crate::core::map::resources::current_map::CurrentMap;
use crate::core::monster::resources::monster_registry::MonsterRegistry;
use crate::core::rng::resources::game_rng::GameRng;
use crate::core::world_clock::components::next_turn::NextTurn;

/// Spawn every monster declared by the current map's spawn table as
/// pure game data: the kind handle, the spawn cell, and birth hit
/// points rolled from the kind's hit dice with current equal to
/// ceiling. Monster ids resolve here, not at map load: the map domain
/// does not depend on this vocabulary (positions flow the other way),
/// so an unknown id surfaces at spawn — still startup, the same
/// launch. Nothing despawns on state exit today (the app never leaves
/// `Game`); a cleanup/rebuild strategy arrives with map switching.
pub fn spawn_monsters(
    mut commands: Commands,
    current_map: Res<CurrentMap>,
    monster_registry: Res<MonsterRegistry>,
    mut game_rng: ResMut<GameRng>,
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
        let max = roll_with(monster_kind.hit_points, &mut game_rng.rng);
        commands.spawn((
            monster_index,
            monster_kind.speed,
            spawn.cell,
            NextTurn::default(),
            HitPoints { current: max, max },
        ));
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    use super::*;
    use crate::core::map::components::cell_coord::CellCoord;
    use crate::core::map::types::local_map::LocalMap;
    use crate::core::map::types::monster_spawn::MonsterSpawn;
    use crate::core::monster::components::monster_index::MonsterIndex;
    use crate::core::monster::resources::monster_registry::parse_monster_registry;
    use crate::core::speed::components::speed::Speed;

    fn monster_registry() -> MonsterRegistry {
        parse_monster_registry(
            "test",
            r#"[
                (
                    monster: "giant_white_rat",
                    speed: 110,
                    hit_points: "2d2",
                    armor_class: 7,
                    level: 4,
                    blows: [ ( damage: "1d3" ) ],
                ),
                (
                    monster: "jackal",
                    speed: 120,
                    hit_points: "1d4",
                    armor_class: 4,
                    level: 2,
                    blows: [ ( damage: "1d2" ) ],
                ),
            ]"#,
        )
    }

    #[test]
    fn spawns_carry_kind_and_position() {
        let monster_registry = monster_registry();
        let rat_handle = monster_registry.get_index("giant_white_rat").unwrap();
        let jackal_handle = monster_registry.get_index("jackal").unwrap();
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
                    monster: "jackal".to_string(),
                    cell: CellCoord::new(1, 0),
                },
            ],
        };

        let mut world = World::new();
        world.insert_resource(CurrentMap::new(local_map));
        world.insert_resource(monster_registry);
        world.insert_resource(GameRng::seeded(42));
        world.run_system_once(spawn_monsters).unwrap();

        // Each spawn carries its kind, speed, cell as logical position,
        // and birth hit points — and no race, class, or unique handles.
        let mut query = world.query::<(&MonsterIndex, &Speed, &CellCoord, &HitPoints)>();
        let entities: Vec<_> = query.iter(&world).collect();
        assert_eq!(entities.len(), 2);
        let rat = entities.iter().find(|(m, ..)| **m == rat_handle).unwrap();
        assert_eq!(*rat.1, Speed(110));
        assert_eq!(*rat.2, CellCoord::new(0, 0));
        let jackal = entities
            .iter()
            .find(|(m, ..)| **m == jackal_handle)
            .unwrap();
        assert_eq!(*jackal.1, Speed(120));
        assert_eq!(*jackal.2, CellCoord::new(1, 0));

        // Birth hit points come from the seeded source in spawn-table
        // order: the rat draws its 2d2, then the jackal its 1d4. Current
        // equals ceiling, and each lands inside its die range.
        let mut replay = StdRng::seed_from_u64(42);
        let expected_rat_max = replay.random_range(1..=2) + replay.random_range(1..=2);
        let expected_jackal_max = replay.random_range(1..=4);
        assert_eq!(rat.3.max, expected_rat_max, "seeded 2d2 roll");
        assert_eq!(rat.3.current, rat.3.max);
        assert!((2..=4).contains(&rat.3.max), "2d2 range");
        assert_eq!(jackal.3.max, expected_jackal_max, "seeded 1d4 roll");
        assert_eq!(jackal.3.current, jackal.3.max);
        assert!((1..=4).contains(&jackal.3.max), "1d4 range");
    }

    #[test]
    #[should_panic(expected = "map declares unknown monster 'wolf'")]
    fn unknown_monster_panics() {
        let monster_registry = parse_monster_registry(
            "test",
            r#"[
                (
                    monster: "giant_white_rat",
                    speed: 110,
                    hit_points: "2d2",
                    armor_class: 7,
                    level: 4,
                    blows: [ ( damage: "1d3" ) ],
                ),
            ]"#,
        );
        let local_map = LocalMap {
            width: 1,
            height: 1,
            tiles: vec![],
            spawns: vec![MonsterSpawn {
                monster: "wolf".to_string(),
                cell: CellCoord::new(0, 0),
            }],
        };
        let mut world = World::new();
        world.insert_resource(CurrentMap::new(local_map));
        world.insert_resource(monster_registry);
        world.insert_resource(GameRng::seeded(42));
        world.run_system_once(spawn_monsters).unwrap();
    }
}
