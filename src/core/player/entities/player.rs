//! Player spawning: game data only. The frontend attaches appearance
//! (sprite, animation) in reaction to the identity handles added here.

use bevy::prelude::*;

use crate::core::creature::resources::class_registry::ClassRegistry;
use crate::core::creature::resources::race_registry::RaceRegistry;
use crate::core::map::components::cell_coord::CellCoord;
use crate::core::player::components::player::Player;
use crate::core::speed::components::speed::Speed;
use crate::core::speed::constants::speed::STANDARD_SPEED;
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
/// start cell, and standard speed. No occupation yet: the first action
/// is free to execute at time zero, like everyone else's birth. Nothing
/// despawns on state exit today (the app never leaves `Game`); a
/// cleanup/rebuild strategy arrives with map switching.
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
    commands.spawn((
        Player,
        race_index,
        class_index,
        WorldDriver,
        PLAYER_START,
        Speed(STANDARD_SPEED),
        NextTurn::default(),
    ));
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use super::*;
    use crate::core::creature::components::class_index::ClassIndex;
    use crate::core::creature::components::race_index::RaceIndex;
    use crate::core::creature::resources::class_registry::parse_class_registry;
    use crate::core::creature::resources::race_registry::parse_race_registry;

    #[test]
    fn player_spawns_with_marker_identities_and_start_position() {
        let race_registry = parse_race_registry("test", r#"[ ( race: "human" ) ]"#);
        let class_registry = parse_class_registry("test", r#"[ ( class: "warrior" ) ]"#);
        let expected_race = race_registry.get_index("human").unwrap();
        let expected_class = class_registry.get_index("warrior").unwrap();

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
        )>();
        let entities: Vec<_> = query.iter(&world).collect();
        assert_eq!(entities.len(), 1);
        let (_, race_index, class_index, _, cell, speed, next_turn) = entities[0];
        assert_eq!(*race_index, expected_race);
        assert_eq!(*class_index, expected_class);
        assert_eq!(*cell, PLAYER_START);
        assert_eq!(*speed, Speed(STANDARD_SPEED));
        assert_eq!(next_turn.at, 0, "the first action is free at time zero");
    }
}
