//! Player spawning: game data only. The frontend attaches appearance
//! (sprite, animation) in reaction to the identity handles added here.

use bevy::prelude::*;

use crate::core::class::resources::class_registry::ClassRegistry;
use crate::core::health::components::hit_points::HitPoints;
use crate::core::health::utils::hp::max_hit_points;
use crate::core::map::components::cell_coord::CellCoord;
use crate::core::player::components::player::Player;
use crate::core::race::resources::race_registry::RaceRegistry;
use crate::core::speed::components::speed::Speed;
use crate::core::speed::constants::speed::STANDARD_SPEED;
use crate::core::stats::components::stats::Stats;
use crate::core::stats::constants::stat::Stat;
use crate::core::stats::utils::adjust::adjust_stat;
use crate::core::stats::utils::roll::roll_base_stats;
use crate::core::world_clock::components::next_turn::NextTurn;
use crate::core::world_clock::components::world_driver::WorldDriver;

/// Starting cell of the player in the test room.
const PLAYER_START: CellCoord = CellCoord::new(24, 10);

/// The player's race. Spawn content is a code constant until birth moves
/// to data files.
const PLAYER_RACE: &str = "human";

/// The player's class: the warrior. Spawn content is a code constant until
/// class picking at birth lands.
const PLAYER_CLASS: &str = "warrior";

/// Spawn the player as pure game data: marker, identity handles, the
/// start cell, standard speed — and the birth values. Birth rolls the
/// six base statistics, merges each with its summed race-and-class
/// modifier, and derives the hit-point ceiling from the merged
/// constitution against the full hit die (race share plus class share).
/// No occupation yet: the first action is free to execute at time zero,
/// like everyone else's birth. Nothing despawns on state exit today
/// (the app never leaves `Game`); a cleanup/rebuild strategy arrives
/// with map switching.
pub fn spawn_player(
    mut commands: Commands,
    race_registry: Res<RaceRegistry>,
    class_registry: Res<ClassRegistry>,
) {
    let race_index = race_registry
        .get_index(PLAYER_RACE)
        .expect("human is a declared race");
    let class_index = class_registry
        .get_index(PLAYER_CLASS)
        .expect("warrior is a declared class");
    let race_kind = race_registry.race_kind(race_index);
    let class_kind = class_registry.class_kind(class_index);
    let mut values = roll_base_stats();
    for stat in Stat::ALL {
        let modifier =
            race_kind.stat_modifiers[stat.index()] + class_kind.stat_modifiers[stat.index()];
        values[stat.index()] = adjust_stat(values[stat.index()], modifier);
    }
    let hit_die = i32::from(race_kind.hit_die) + i32::from(class_kind.hit_die);
    let max = max_hit_points(hit_die, values[Stat::Constitution.index()]);
    commands.spawn((
        Player,
        race_index,
        class_index,
        WorldDriver,
        PLAYER_START,
        Speed(STANDARD_SPEED),
        NextTurn::default(),
        Stats {
            max: values,
            current: values,
        },
        HitPoints { current: max, max },
    ));
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use super::*;
    use crate::core::class::components::class_index::ClassIndex;
    use crate::core::class::resources::class_registry::parse_class_registry;
    use crate::core::race::components::race_index::RaceIndex;
    use crate::core::race::resources::race_registry::parse_race_registry;

    /// Test birth data distinct from the real table so value assertions
    /// exercise the merge, not the content.
    fn race_registry() -> RaceRegistry {
        parse_race_registry(
            "test",
            r#"[ ( race: "human", stat_modifiers: [1, 0, 0, 0, 0, -1], hit_die: 10 ) ]"#,
        )
    }

    fn class_registry() -> ClassRegistry {
        parse_class_registry(
            "test",
            r#"[ ( class: "warrior", stat_modifiers: [2, 0, 0, 1, 1, 0], hit_die: 9 ) ]"#,
        )
    }

    #[test]
    fn player_spawns_with_marker_identity_birth_and_start() {
        let race_registry = race_registry();
        let class_registry = class_registry();
        let expected_race = race_registry.get_index("human").unwrap();
        let expected_class = class_registry.get_index("warrior").unwrap();
        // Copy the birth data out before the registries move into the
        // world.
        let modifiers: [i32; 6] = race_registry.race_kind(expected_race).stat_modifiers;
        let hit_die = i32::from(race_registry.race_kind(expected_race).hit_die)
            + i32::from(class_registry.class_kind(expected_class).hit_die);

        let mut world = World::new();
        world.insert_resource(race_registry);
        world.insert_resource(class_registry);
        world.run_system_once(spawn_player).unwrap();

        let mut query = world.query::<(
            &Player,
            &RaceIndex,
            &ClassIndex,
            &WorldDriver,
            &CellCoord,
            &Speed,
            &NextTurn,
            &Stats,
            &HitPoints,
        )>();
        let entities: Vec<_> = query.iter(&world).collect();
        assert_eq!(entities.len(), 1);
        let (_, race_index, class_index, _, cell, speed, next_turn, stats, hit_points) =
            entities[0];
        assert_eq!(*race_index, expected_race);
        assert_eq!(*class_index, expected_class);
        assert_eq!(*cell, PLAYER_START);
        assert_eq!(*speed, Speed(STANDARD_SPEED));
        assert_eq!(next_turn.at, 0, "the first action is free at time zero");
        // Birth statistics: the ceiling equals the current set (nothing
        // drains at birth), and every statistic at least reaches its
        // rolled floor plus the merged modifier — each merged point adds
        // at least one.
        assert_eq!(stats.max, stats.current);
        for stat in Stat::ALL {
            let floor = (8 + modifiers[stat.index()]).max(3);
            assert!(
                stats.max[stat.index()] >= floor,
                "{stat:?} merged to {}, below its floor {floor}",
                stats.max[stat.index()]
            );
        }
        // Hit points: the ceiling is the merged constitution's function,
        // and nothing is spent at birth.
        let constitution = stats.max[Stat::Constitution.index()];
        assert_eq!(hit_points.max, max_hit_points(hit_die, constitution));
        assert_eq!(hit_points.current, hit_points.max);
    }
}
