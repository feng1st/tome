//! Act attack: execute every strike action planned this frame.

use bevy::prelude::*;

use crate::core::combat::components::attack::Attack;
use crate::core::combat::components::combat_bonuses::CombatBonuses;
use crate::core::combat::utils::attack::attack_chance;
use crate::core::combat::utils::attack::attack_hits;
use crate::core::combat::utils::attack::unarmed_damage;
use crate::core::health::messages::damage::Damage;
use crate::core::monster::components::monster_index::MonsterIndex;
use crate::core::monster::resources::monster_registry::MonsterRegistry;
use crate::core::rng::resources::game_rng::GameRng;

/// Execute every strike planned this frame: roll the hit against the
/// target kind's armor class, and on a hit write one damage request
/// with the landed unarmed damage — a miss writes nothing. The action
/// component is consumed either way. The target is read fallibly:
/// planning and execution share one frame, so the target cannot die in
/// between, but the guarantee is a comment, not a type.
pub fn act_attack(
    mut commands: Commands,
    mut strikes: Query<(Entity, &Attack, &CombatBonuses), Added<Attack>>,
    monsters: Query<&MonsterIndex>,
    monster_registry: Res<MonsterRegistry>,
    mut game_rng: ResMut<GameRng>,
    mut damages: MessageWriter<Damage>,
) {
    for (attacker, strike, bonuses) in &mut strikes {
        commands.entity(attacker).remove::<Attack>();
        let Ok(kind) = monsters.get(strike.target) else {
            continue;
        };
        let chance = attack_chance(bonuses);
        let armor_class = monster_registry.monster_kind(*kind).armor_class;
        if attack_hits(chance, armor_class, &mut game_rng.rng) {
            damages.write(Damage {
                target: strike.target,
                amount: unarmed_damage(bonuses),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;
    use rand::rngs::StdRng;
    use rand::Rng;
    use rand::SeedableRng;

    use super::*;
    use crate::core::monster::resources::monster_registry::parse_monster_registry;

    fn monster_registry(armor_class: i32) -> MonsterRegistry {
        parse_monster_registry(
            "test",
            &format!(
                r#"[
                    (
                        monster: "rat",
                        speed: 110,
                        hit_points: "2d2",
                        armor_class: {armor_class},
                        level: 4,
                        blows: [ ( damage: "1d3" ) ],
                    ),
                ]"#
            ),
        )
    }

    /// A seed whose percentile draw sits at or above `min`, scanned so
    /// the strike's certain-band branch is pinned before the run.
    fn seed_with_percentile_at_least(min: i32) -> u64 {
        (0..10_000u64)
            .find(|seed| StdRng::seed_from_u64(*seed).random_range(0..100) >= min)
            .expect("a matching seed exists")
    }

    #[test]
    fn a_hit_writes_one_damage_request_with_the_unarmed_amount() {
        // Armor 0: any percentile past the bands hits, and the unarmed
        // damage with a +3 bonus is 4.
        let seed = seed_with_percentile_at_least(10);
        let mut world = World::new();
        world.insert_resource(monster_registry(0));
        world.insert_resource(GameRng::seeded(seed));
        world.init_resource::<Messages<Damage>>();
        let rat = world.spawn(MonsterIndex::from_index(0)).id();
        let player = world
            .spawn((
                Attack { target: rat },
                CombatBonuses {
                    hit: 0,
                    damage: 3,
                    armor: 0,
                },
            ))
            .id();

        world.run_system_once(act_attack).unwrap();

        let requests: Vec<Damage> = world.resource_mut::<Messages<Damage>>().drain().collect();
        assert_eq!(
            requests,
            vec![Damage {
                target: rat,
                amount: 4
            }]
        );
        assert!(world.get::<Attack>(player).is_none(), "strike consumed");
    }

    #[test]
    fn a_miss_writes_nothing_and_consumes_the_strike() {
        // Armor 1000: no power roll can beat three quarters of it, so
        // every seed misses.
        let mut world = World::new();
        world.insert_resource(monster_registry(1000));
        world.insert_resource(GameRng::seeded(42));
        world.init_resource::<Messages<Damage>>();
        let rat = world.spawn(MonsterIndex::from_index(0)).id();
        let player = world
            .spawn((
                Attack { target: rat },
                CombatBonuses {
                    hit: 0,
                    damage: 3,
                    armor: 0,
                },
            ))
            .id();

        world.run_system_once(act_attack).unwrap();

        let requests: Vec<Damage> = world.resource_mut::<Messages<Damage>>().drain().collect();
        assert!(requests.is_empty(), "a miss writes nothing");
        assert!(world.get::<Attack>(player).is_none(), "strike consumed");
    }

    #[test]
    fn a_vanished_target_is_skipped() {
        let mut world = World::new();
        world.insert_resource(monster_registry(0));
        world.insert_resource(GameRng::seeded(42));
        world.init_resource::<Messages<Damage>>();
        let player = world
            .spawn((
                Attack {
                    target: Entity::PLACEHOLDER,
                },
                CombatBonuses {
                    hit: 0,
                    damage: 3,
                    armor: 0,
                },
            ))
            .id();

        world.run_system_once(act_attack).unwrap();

        let requests: Vec<Damage> = world.resource_mut::<Messages<Damage>>().drain().collect();
        assert!(requests.is_empty());
        assert!(world.get::<Attack>(player).is_none(), "strike consumed");
    }
}
