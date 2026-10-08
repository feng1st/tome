//! Monster spawning: game data only. The frontend attaches appearance
//! (sprite, animation) in reaction to the kind handle added here.

use bevy::prelude::*;

use crate::core::combat::components::armor_class::ArmorClass;
use crate::core::combat::components::blows::Blows;
use crate::core::combat::types::blow::Blow;
use crate::core::health::components::hit_points::HitPoints;
use crate::core::map::resources::current_map::CurrentMap;
use crate::core::monster::resources::monster_registry::MonsterRegistry;
use crate::core::monster::types::monster_kind::MonsterKind;
use crate::core::rng::resources::game_rng::GameRng;
use crate::core::world_clock::components::next_turn::NextTurn;

/// The monster blow power stand-in: 60, the plain-damage family's
/// power — what a blow without an attached effect behaves as.
/// Nothing attaches effects here, so every blow takes the plain
/// stand-in; deleted when the effect family lands and blows carry
/// their own effect (see OPEN_ISSUES entry 10).
const MONSTER_BLOW_POWER_STANDIN: i32 = 60;

/// Points of blow chance per point of monster level: a kind's blow
/// chance is the blow power stand-in plus three times its level.
const CHANCE_PER_LEVEL: i32 = 3;

/// The blows translated from a kind's row: one blow per declared blow,
/// chance the power stand-in plus three times the level, damage the
/// blow's dice. The row stays the authored source; the entity carries
/// the translated values.
fn translated_blows(monster_kind: &MonsterKind) -> Blows {
    let blows = monster_kind
        .blows
        .iter()
        .map(|blow| Blow {
            chance: MONSTER_BLOW_POWER_STANDIN + monster_kind.level * CHANCE_PER_LEVEL,
            damage: blow.damage,
        })
        .collect();
    Blows(blows)
}

/// Spawn every monster declared by the current map's spawn table as
/// pure game data: the kind handle, the spawn cell, birth hit points
/// rolled from the kind's hit dice with current equal to maximum, and
/// the blows and armor class translated from the kind's combat profile.
/// Monster ids resolve here, not at map load: the map domain does not
/// depend on this vocabulary (positions flow the other way), so an
/// unknown id surfaces at spawn — still startup, the same launch.
/// Nothing despawns on state exit today (the app never leaves `Game`);
/// a cleanup/rebuild strategy arrives with map switching.
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
        let max = monster_kind.hit_points.roll_with(&mut game_rng.rng);
        let blows = translated_blows(monster_kind);
        commands.spawn((
            monster_index,
            monster_kind.speed,
            spawn.cell,
            NextTurn::default(),
            HitPoints { current: max, max },
            blows,
            ArmorClass(monster_kind.armor_class),
        ));
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    use super::*;
    use crate::core::combat::components::armor_class::ArmorClass;
    use crate::core::combat::components::blows::Blows;
    use crate::core::combat::types::blow::Blow;
    use crate::core::dice::types::dice::Dice;
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
        // equals maximum, and each lands inside its die range.
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
    fn spawning_carries_the_blows_and_armor() {
        let local_map = LocalMap {
            width: 1,
            height: 1,
            tiles: vec![],
            spawns: vec![MonsterSpawn {
                monster: "giant_white_rat".to_string(),
                cell: CellCoord::new(0, 0),
            }],
        };
        let mut world = World::new();
        world.insert_resource(CurrentMap::new(local_map));
        world.insert_resource(monster_registry());
        world.insert_resource(GameRng::seeded(42));

        world.run_system_once(spawn_monsters).unwrap();

        // The rat's translated blows and armor: armor class 7, one blow with
        // the stand-in-60 chance plus three per level, damage 1d3.
        let mut query = world.query::<(&Blows, &ArmorClass)>();
        let (blows, armor_class) = query.single(&world).unwrap();
        assert_eq!(armor_class.0, 7);
        assert_eq!(
            blows.0,
            vec![Blow {
                chance: 60 + 3 * 4,
                damage: Dice { dice: 1, side: 3 },
            }]
        );
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
