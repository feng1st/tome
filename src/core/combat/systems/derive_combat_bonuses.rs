//! Derive combat bonuses: react to statistics changes and recompute
//! the combat-bonus component wholesale.

use bevy::prelude::*;

use crate::core::combat::components::combat_bonuses::CombatBonuses;
use crate::core::combat::utils::combat_bonus::{armor_bonus, damage_bonus, hit_bonus};
use crate::core::stats::components::stats::Stats;

/// Recompute the combat-bonus component of every entity whose
/// statistics changed: all three bonuses together, from the current
/// values, from scratch. Birth is included — statistics insertion
/// counts as a change.
pub fn derive_combat_bonuses(
    mut commands: Commands,
    stat_changes: Query<(Entity, &Stats), Changed<Stats>>,
) {
    for (entity, stats) in &stat_changes {
        commands.entity(entity).insert(CombatBonuses {
            hit: hit_bonus(stats),
            damage: damage_bonus(stats),
            armor: armor_bonus(stats),
        });
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use super::*;
    use crate::core::class::resources::class_registry::parse_class_registry;
    use crate::core::player::entities::player::spawn_player;
    use crate::core::race::resources::race_registry::parse_race_registry;
    use crate::core::rng::resources::game_rng::GameRng;
    use crate::core::stats::constants::stat::Stat;

    fn world() -> World {
        let mut world = World::new();
        world.insert_resource(parse_race_registry(
            "test",
            r#"[ ( race: "human", stat_modifiers: [1, 0, 0, 0, 0, -1], hit_die: 10 ) ]"#,
        ));
        world.insert_resource(parse_class_registry(
            "test",
            r#"[ ( class: "warrior", stat_modifiers: [2, 0, 0, 1, 1, 0], hit_die: 9 ) ]"#,
        ));
        world.insert_resource(GameRng::seeded(42));
        world
    }

    #[test]
    fn birth_statistics_derive_into_the_component() {
        let mut world = world();
        world.run_system_once(spawn_player).unwrap();
        world.run_system_once(derive_combat_bonuses).unwrap();

        let entity = world
            .query::<(Entity, &Stats)>()
            .iter(&world)
            .next()
            .unwrap()
            .0;
        let stats = world.entity(entity).get::<Stats>().unwrap();
        let bonuses = world.entity(entity).get::<CombatBonuses>().unwrap();
        assert_eq!(bonuses.hit, hit_bonus(stats));
        assert_eq!(bonuses.damage, damage_bonus(stats));
        assert_eq!(bonuses.armor, armor_bonus(stats));
    }

    #[test]
    fn a_statistics_change_recomputes_the_component() {
        let mut world = world();
        world.run_system_once(spawn_player).unwrap();
        world.run_system_once(derive_combat_bonuses).unwrap();

        let entity = world
            .query::<(Entity, &Stats)>()
            .iter(&world)
            .next()
            .unwrap()
            .0;
        world.entity_mut(entity).get_mut::<Stats>().unwrap().current[Stat::Strength.index()] = 3;
        world.run_system_once(derive_combat_bonuses).unwrap();

        let stats = world.entity(entity).get::<Stats>().unwrap();
        let bonuses = world.entity(entity).get::<CombatBonuses>().unwrap();
        assert_eq!(bonuses.hit, hit_bonus(stats));
        assert_eq!(bonuses.damage, damage_bonus(stats));
        assert_eq!(bonuses.armor, armor_bonus(stats));
    }
}
