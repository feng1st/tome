//! Derive attack: react to combat-bonus changes and recompute the
//! entity's blows and armor class wholesale.

use bevy::prelude::*;

use crate::core::combat::components::armor_class::ArmorClass;
use crate::core::combat::components::blows::Blows;
use crate::core::combat::components::combat_bonuses::CombatBonuses;
use crate::core::combat::types::blow::Blow;
use crate::core::combat::utils::armor_class::armor_class;
use crate::core::combat::utils::attack::{attack_chance, unarmed_damage};
use crate::core::dice::types::dice::Dice;

/// Recompute the blows and armor class of every entity whose combat
/// bonuses changed: one unarmed blow (chance = the attack chance,
/// damage = the unarmed damage riding a one-faced die, so the fixed
/// amount rolls itself) and the armor class, both from scratch. The
/// write is unconditional — a skip-if-equal would stall the change
/// chain the next derive stage hangs off. Runs chained after the bonus
/// derive, so the fresh bonuses are visible in the same pass.
pub fn derive_attack(
    mut commands: Commands,
    bonus_changes: Query<(Entity, &CombatBonuses), Changed<CombatBonuses>>,
) {
    for (entity, bonuses) in &bonus_changes {
        commands.entity(entity).insert((
            Blows(vec![Blow {
                chance: attack_chance(bonuses),
                damage: Dice {
                    dice: unarmed_damage(bonuses),
                    side: 1,
                },
            }]),
            ArmorClass(armor_class(bonuses)),
        ));
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use super::*;
    use crate::core::class::resources::class_registry::parse_class_registry;
    use crate::core::combat::systems::derive_combat_bonuses::derive_combat_bonuses;
    use crate::core::player::entities::player::spawn_player;
    use crate::core::race::resources::race_registry::parse_race_registry;
    use crate::core::rng::resources::game_rng::GameRng;
    use crate::core::stats::components::stats::Stats;
    use crate::core::stats::constants::stat::Stat;

    /// A world with the two registries and a seed; nothing spawned yet.
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

    /// The born player's entity, bonuses derived.
    fn spawn_born_player(world: &mut World) -> Entity {
        world.run_system_once(spawn_player).unwrap();
        let entity = world
            .query::<(Entity, &Stats)>()
            .iter(world)
            .next()
            .unwrap()
            .0;
        world.run_system_once(derive_combat_bonuses).unwrap();
        entity
    }

    #[test]
    fn the_player_blow_and_armor_derive_from_the_bonuses() {
        let mut world = world();
        let entity = spawn_born_player(&mut world);
        world.run_system_once(derive_attack).unwrap();

        let bonuses = world.entity(entity).get::<CombatBonuses>().unwrap();
        let blows = world.entity(entity).get::<Blows>().unwrap();
        assert_eq!(blows.0.len(), 1, "exactly one unarmed blow");
        assert_eq!(blows.0[0].chance, attack_chance(bonuses));
        assert_eq!(
            blows.0[0].damage,
            Dice {
                dice: unarmed_damage(bonuses),
                side: 1
            }
        );
        assert_eq!(
            world.entity(entity).get::<ArmorClass>().unwrap().0,
            armor_class(bonuses)
        );
    }

    #[test]
    fn a_statistics_change_renews_the_blow_and_armor() {
        let mut world = world();
        let entity = spawn_born_player(&mut world);
        world.run_system_once(derive_attack).unwrap();
        let before = world.entity(entity).get::<Blows>().unwrap().0[0].chance;

        world.entity_mut(entity).get_mut::<Stats>().unwrap().current[Stat::Strength.index()] = 3;
        world.run_system_once(derive_combat_bonuses).unwrap();
        world.run_system_once(derive_attack).unwrap();

        let bonuses = world.entity(entity).get::<CombatBonuses>().unwrap();
        let blows = world.entity(entity).get::<Blows>().unwrap();
        assert_eq!(blows.0[0].chance, attack_chance(bonuses));
        assert_ne!(before, blows.0[0].chance, "strength 3 drops the chance");
        assert_eq!(
            world.entity(entity).get::<ArmorClass>().unwrap().0,
            armor_class(bonuses)
        );
    }

    /// The derive chain through real chained systems in real frames: a
    /// statistics change reaches the blow in the same update. Every
    /// stage writes unconditionally, so the change the next stage hangs
    /// off never stalls.
    #[test]
    fn a_statistics_change_reaches_the_blow_in_the_same_update() {
        let mut app = App::new();
        app.insert_resource(parse_race_registry(
            "test",
            r#"[ ( race: "human", stat_modifiers: [1, 0, 0, 0, 0, -1], hit_die: 10 ) ]"#,
        ));
        app.insert_resource(parse_class_registry(
            "test",
            r#"[ ( class: "warrior", stat_modifiers: [2, 0, 0, 1, 1, 0], hit_die: 9 ) ]"#,
        ));
        app.insert_resource(GameRng::seeded(42));
        app.add_systems(
            Update,
            (spawn_player, (derive_combat_bonuses, derive_attack).chain()).chain(),
        );
        app.update();

        let entity = {
            let world = app.world_mut();
            let mut query = world.query::<(Entity, &Stats)>();
            query.iter(world).next().expect("the player spawned").0
        };
        let before = app
            .world()
            .get::<Blows>(entity)
            .expect("birth derived the blow")
            .0[0]
            .chance;

        app.world_mut().get_mut::<Stats>(entity).unwrap().current[Stat::Strength.index()] = 3;
        app.update();

        let bonuses = app.world().get::<CombatBonuses>(entity).unwrap();
        let blows = app.world().get::<Blows>(entity).unwrap();
        assert_eq!(blows.0[0].chance, attack_chance(bonuses));
        assert_ne!(before, blows.0[0].chance, "the renewal reached the blow");
    }
}
