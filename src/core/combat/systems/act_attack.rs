//! Act attack: resolve every strike action planned this frame through
//! the shared pipeline.

use bevy::prelude::*;

use crate::core::combat::components::armor_class::ArmorClass;
use crate::core::combat::components::attack::Attack;
use crate::core::combat::components::blows::Blows;
use crate::core::combat::utils::attack::attack_hits;
use crate::core::health::messages::damage::Damage;
use crate::core::rng::resources::game_rng::GameRng;

/// Resolve every strike planned this frame: for each blow the attacker
/// carries, judge the hit against the target's armor class and write
/// one damage request per hit, the blow's damage drawn from the central
/// source. The action component is consumed either way, and an
/// attacker carrying no blows spends the action striking nothing. No
/// vocabulary and no attacker or target kind: every number comes from
/// the blows and armor-class components the creatures carry. The target
/// is read fallibly: planning and execution share one frame, so the
/// target cannot die in between, but the guarantee is a comment, not a
/// type — a vanished target is skipped, and an absent armor class
/// reads as zero.
pub fn act_attack(
    mut commands: Commands,
    mut strikes: Query<(Entity, &Attack, Option<&Blows>), Added<Attack>>,
    armor_classes: Query<Option<&ArmorClass>>,
    mut game_rng: ResMut<GameRng>,
    mut damages: MessageWriter<Damage>,
) {
    for (attacker, strike, blows) in &mut strikes {
        commands.entity(attacker).remove::<Attack>();
        let Ok(armor_class) = armor_classes.get(strike.target) else {
            // The target left the world since planning: nothing to strike.
            continue;
        };
        let armor_class = armor_class.map_or(0, |armor_class| armor_class.0);
        let Some(blows) = blows else {
            // A creature without blows cannot strike; the action is spent.
            continue;
        };
        for blow in &blows.0 {
            if attack_hits(blow.chance, armor_class, &mut game_rng.rng) {
                damages.write(Damage {
                    target: strike.target,
                    amount: blow.damage.roll(&mut game_rng.rng),
                });
            }
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
    use crate::core::combat::types::blow::{Blow, BlowDamage};
    use crate::core::dice::types::dice::Dice;

    /// The world pieces every test needs: the seeded source and the
    /// damage message channel.
    fn world(seed: u64) -> World {
        let mut world = World::new();
        world.insert_resource(GameRng::seeded(seed));
        world.init_resource::<Messages<Damage>>();
        world
    }

    /// A seed whose percentile draw sits at or above `min`, scanned so
    /// the certain-band branches stay out of the way before the run.
    fn seed_with_percentile_at_least(min: i32) -> u64 {
        (0..10_000u64)
            .find(|seed| StdRng::seed_from_u64(*seed).random_range(0..100) >= min)
            .expect("a matching seed exists")
    }

    /// Fixed-damage blows, chance and amount as given.
    fn blows(specs: &[(i32, i32)]) -> Blows {
        Blows(
            specs
                .iter()
                .map(|(chance, damage)| Blow {
                    chance: *chance,
                    damage: BlowDamage::Fixed(*damage),
                })
                .collect(),
        )
    }

    fn damages(world: &mut World) -> Vec<Damage> {
        world.resource_mut::<Messages<Damage>>().drain().collect()
    }

    #[test]
    fn a_hit_writes_the_blows_damage_and_consumes_the_action() {
        // Armor 0 and a huge chance: any percentile past the bands hits.
        let seed = seed_with_percentile_at_least(10);
        let mut world = world(seed);
        let target = world.spawn(ArmorClass(0)).id();
        let attacker = world.spawn((Attack { target }, blows(&[(1000, 4)]))).id();

        world.run_system_once(act_attack).unwrap();

        assert_eq!(damages(&mut world), vec![Damage { target, amount: 4 }]);
        assert!(world.get::<Attack>(attacker).is_none(), "action consumed");
    }

    #[test]
    fn a_miss_writes_nothing_and_consumes_the_action() {
        // Armor 1000: three quarters is 750, beyond any power roll a
        // chance of 20 can draw.
        let seed = seed_with_percentile_at_least(10);
        let mut world = world(seed);
        let target = world.spawn(ArmorClass(1000)).id();
        let attacker = world.spawn((Attack { target }, blows(&[(20, 4)]))).id();

        world.run_system_once(act_attack).unwrap();

        assert!(damages(&mut world).is_empty(), "a miss writes nothing");
        assert!(world.get::<Attack>(attacker).is_none(), "action consumed");
    }

    #[test]
    fn blows_judge_on_their_own() {
        // The second blow's chance is non-positive: it must miss while
        // the first hits, and only the first blow's damage lands.
        let seed = seed_with_percentile_at_least(10);
        let mut world = world(seed);
        let target = world.spawn(ArmorClass(0)).id();
        let attacker = world
            .spawn((Attack { target }, blows(&[(1000, 4), (0, 5)])))
            .id();

        world.run_system_once(act_attack).unwrap();

        assert_eq!(damages(&mut world), vec![Damage { target, amount: 4 }]);
        assert!(world.get::<Attack>(attacker).is_none(), "action consumed");
    }

    #[test]
    fn a_multi_blow_strike_rolls_each_hit_separately() {
        let seed = seed_with_percentile_at_least(10);
        let mut world = world(seed);
        let target = world.spawn(ArmorClass(0)).id();
        let attacker = world
            .spawn((
                Attack { target },
                Blows(vec![
                    Blow {
                        chance: 1000,
                        damage: BlowDamage::Roll(Dice { n: 1, m: 3 }),
                    },
                    Blow {
                        chance: 1000,
                        damage: BlowDamage::Roll(Dice { n: 1, m: 3 }),
                    },
                ]),
            ))
            .id();

        world.run_system_once(act_attack).unwrap();

        // Replay the draw order: percentile, power roll, damage die —
        // twice, blow after blow.
        let mut replay = StdRng::seed_from_u64(seed);
        let _ = replay.random_range(0..100);
        let _: i32 = replay.random_range(0..1000);
        let first: i32 = replay.random_range(1..=3);
        let _ = replay.random_range(0..100);
        let _: i32 = replay.random_range(0..1000);
        let second: i32 = replay.random_range(1..=3);
        assert_eq!(
            damages(&mut world),
            vec![
                Damage {
                    target,
                    amount: first
                },
                Damage {
                    target,
                    amount: second
                },
            ]
        );
        assert!(world.get::<Attack>(attacker).is_none(), "action consumed");
    }

    #[test]
    fn a_target_without_armor_reads_zero() {
        // One point of chance and no armor component: the power roll
        // (only 0) can pass only against an armor threshold of zero.
        let seed = seed_with_percentile_at_least(10);
        let mut world = world(seed);
        let target = world.spawn_empty().id();
        let attacker = world.spawn((Attack { target }, blows(&[(1, 3)]))).id();

        world.run_system_once(act_attack).unwrap();

        assert_eq!(damages(&mut world), vec![Damage { target, amount: 3 }]);
        assert!(world.get::<Attack>(attacker).is_none(), "action consumed");
    }

    #[test]
    fn an_attacker_without_blows_strikes_nothing() {
        let seed = seed_with_percentile_at_least(10);
        let mut world = world(seed);
        let target = world.spawn(ArmorClass(0)).id();
        let bare = world.spawn(Attack { target }).id();
        let empty = world.spawn((Attack { target }, Blows(vec![]))).id();

        world.run_system_once(act_attack).unwrap();

        assert!(damages(&mut world).is_empty());
        assert!(world.get::<Attack>(bare).is_none(), "action consumed");
        assert!(world.get::<Attack>(empty).is_none(), "action consumed");
    }

    #[test]
    fn a_vanished_target_is_skipped() {
        let mut world = world(42);
        let attacker = world
            .spawn((
                Attack {
                    target: Entity::PLACEHOLDER,
                },
                blows(&[(1000, 4)]),
            ))
            .id();

        world.run_system_once(act_attack).unwrap();

        assert!(damages(&mut world).is_empty());
        assert!(world.get::<Attack>(attacker).is_none(), "action consumed");
    }
}
