//! Act attack: resolve every attack action planned this frame through
//! the shared pipeline.

use bevy::prelude::*;

use crate::core::action::components::attack_action::AttackAction;
use crate::core::combat::components::armor_class::ArmorClass;
use crate::core::combat::components::blows::Blows;
use crate::core::combat::messages::attacked::Attacked;
use crate::core::combat::utils::attack::attack_hits;
use crate::core::health::components::dead::Dead;
use crate::core::health::messages::damage::Damage;
use crate::core::map::components::cell_coord::CellCoord;
use crate::core::rng::resources::game_rng::GameRng;

type ActorFilter = (Added<AttackAction>, Without<Dead>);

/// Resolve every attack planned this frame: for each blow the attacker
/// carries, judge the hit against the target's armor class and write
/// one damage request per hit, the blow's damage drawn from the central
/// source. Each resolution also emits its fact — attacker, both cells,
/// and the landed amounts — hit or miss: the swing itself is the fact.
/// The action component is consumed either way, and an attacker
/// carrying no blows spends the action swinging nothing. No vocabulary
/// and no attacker or target kind: every number comes from the blows
/// and armor-class components the creatures carry. The target is read
/// fallibly: planning and execution share one frame, so the target
/// cannot die in between, but the guarantee is a comment, not a type —
/// a vanished or dead target is skipped (its attack emits no fact:
/// nothing was swung at), and an absent armor class reads as zero.
pub fn act_attack(
    mut commands: Commands,
    mut actors: Query<(Entity, &AttackAction, &CellCoord, Option<&Blows>), ActorFilter>,
    // The target is read fallibly: a target that left the world, or a
    // dead one (the marked driver stays in the world), reads as no
    // target at all — the filter sees to the dead, the fallible get to
    // the gone. Absent armor reads as zero.
    targets: Query<(&CellCoord, Option<&ArmorClass>), Without<Dead>>,
    mut game_rng: ResMut<GameRng>,
    mut damage_writer: MessageWriter<Damage>,
    mut attacked_writer: MessageWriter<Attacked>,
) {
    for (actor, action, &actor_cell_coord, actor_blows) in &mut actors {
        commands.entity(actor).remove::<AttackAction>();
        let Ok((&target_cell_coord, target_armor_class)) = targets.get(action.target) else {
            // The target left the world or died since planning:
            // nothing to attack.
            continue;
        };
        let target_armor_class = target_armor_class.map_or(0, |armor_class| armor_class.0);
        let mut damage_amounts = Vec::new();
        if let Some(actor_blows) = actor_blows {
            for actor_blow in &actor_blows.0 {
                if attack_hits(actor_blow.chance, target_armor_class, &mut game_rng.rng) {
                    let damage_amount = actor_blow.damage.roll_with(&mut game_rng.rng);
                    damage_writer.write(Damage {
                        target: action.target,
                        source: Some(actor),
                        amount: damage_amount,
                    });
                    damage_amounts.push(damage_amount);
                }
            }
        }
        attacked_writer.write(Attacked {
            attacker: actor,
            attacker_cell_coord: actor_cell_coord,
            target_cell_coord,
            damage_amounts,
        });
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;
    use rand::rngs::StdRng;
    use rand::Rng;
    use rand::SeedableRng;

    use super::*;
    use crate::core::combat::types::blow::Blow;
    use crate::core::dice::types::dice::Dice;
    use crate::core::health::components::dead::Dead;

    /// The world pieces every test needs: the seeded source and the
    /// two message channels — requests and facts.
    fn world(seed: u64) -> World {
        let mut world = World::new();
        world.insert_resource(GameRng::seeded(seed));
        world.init_resource::<Messages<Damage>>();
        world.init_resource::<Messages<Attacked>>();
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
                    damage: Dice {
                        dice: *damage,
                        side: 1,
                    },
                })
                .collect(),
        )
    }

    fn damages(world: &mut World) -> Vec<Damage> {
        world.resource_mut::<Messages<Damage>>().drain().collect()
    }

    fn attacked(world: &mut World) -> Vec<Attacked> {
        world.resource_mut::<Messages<Attacked>>().drain().collect()
    }

    #[test]
    fn a_hit_writes_the_blows_damage_and_consumes_the_action() {
        // Armor 0 and a huge chance: any percentile past the bands hits.
        let seed = seed_with_percentile_at_least(10);
        let mut world = world(seed);
        let target = world.spawn((ArmorClass(0), CellCoord::new(3, 0))).id();
        let attacker = world
            .spawn((
                AttackAction { target },
                blows(&[(1000, 4)]),
                CellCoord::new(1, 0),
            ))
            .id();

        world.run_system_once(act_attack).unwrap();

        assert_eq!(
            damages(&mut world),
            vec![Damage {
                target,
                source: Some(attacker),
                amount: 4
            }]
        );
        assert!(
            world.get::<AttackAction>(attacker).is_none(),
            "action consumed"
        );
    }

    #[test]
    fn a_miss_writes_nothing_and_consumes_the_action() {
        // Armor 1000: three quarters is 750, beyond any power roll a
        // chance of 20 can draw.
        let seed = seed_with_percentile_at_least(10);
        let mut world = world(seed);
        let target = world.spawn((ArmorClass(1000), CellCoord::new(3, 0))).id();
        let attacker = world
            .spawn((
                AttackAction { target },
                blows(&[(20, 4)]),
                CellCoord::new(1, 0),
            ))
            .id();

        world.run_system_once(act_attack).unwrap();

        assert!(damages(&mut world).is_empty(), "a miss writes nothing");
        assert!(
            world.get::<AttackAction>(attacker).is_none(),
            "action consumed"
        );
    }

    #[test]
    fn blows_judge_on_their_own() {
        // The second blow's chance is non-positive: it must miss while
        // the first hits, and only the first blow's damage lands.
        let seed = seed_with_percentile_at_least(10);
        let mut world = world(seed);
        let target = world.spawn((ArmorClass(0), CellCoord::new(3, 0))).id();
        let attacker = world
            .spawn((
                AttackAction { target },
                blows(&[(1000, 4), (0, 5)]),
                CellCoord::new(1, 0),
            ))
            .id();

        world.run_system_once(act_attack).unwrap();

        assert_eq!(
            damages(&mut world),
            vec![Damage {
                target,
                source: Some(attacker),
                amount: 4
            }]
        );
        assert!(
            world.get::<AttackAction>(attacker).is_none(),
            "action consumed"
        );
    }

    #[test]
    fn a_multi_blow_attack_rolls_each_hit_separately() {
        let seed = seed_with_percentile_at_least(10);
        let mut world = world(seed);
        let target = world.spawn((ArmorClass(0), CellCoord::new(3, 0))).id();
        let attacker = world
            .spawn((
                AttackAction { target },
                Blows(vec![
                    Blow {
                        chance: 1000,
                        damage: Dice { dice: 1, side: 3 },
                    },
                    Blow {
                        chance: 1000,
                        damage: Dice { dice: 1, side: 3 },
                    },
                ]),
                CellCoord::new(1, 0),
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
                    source: Some(attacker),
                    amount: first
                },
                Damage {
                    target,
                    source: Some(attacker),
                    amount: second
                },
            ]
        );
        assert!(
            world.get::<AttackAction>(attacker).is_none(),
            "action consumed"
        );
    }

    #[test]
    fn a_target_without_armor_reads_zero() {
        // One point of chance and no armor component: the power roll
        // (only 0) can pass only against an armor threshold of zero.
        let seed = seed_with_percentile_at_least(10);
        let mut world = world(seed);
        let target = world.spawn(CellCoord::new(3, 0)).id();
        let attacker = world
            .spawn((
                AttackAction { target },
                blows(&[(1, 3)]),
                CellCoord::new(1, 0),
            ))
            .id();

        world.run_system_once(act_attack).unwrap();

        assert_eq!(
            damages(&mut world),
            vec![Damage {
                target,
                source: Some(attacker),
                amount: 3
            }]
        );
        assert!(
            world.get::<AttackAction>(attacker).is_none(),
            "action consumed"
        );
    }

    #[test]
    fn an_attacker_without_blows_attacks_nothing() {
        let seed = seed_with_percentile_at_least(10);
        let mut world = world(seed);
        let target = world.spawn((ArmorClass(0), CellCoord::new(3, 0))).id();
        let bare = world
            .spawn((AttackAction { target }, CellCoord::new(1, 0)))
            .id();
        let empty = world
            .spawn((AttackAction { target }, Blows(vec![]), CellCoord::new(2, 0)))
            .id();

        world.run_system_once(act_attack).unwrap();

        assert!(damages(&mut world).is_empty());
        assert!(world.get::<AttackAction>(bare).is_none(), "action consumed");
        assert!(
            world.get::<AttackAction>(empty).is_none(),
            "action consumed"
        );
    }

    #[test]
    fn a_vanished_target_is_skipped() {
        let mut world = world(42);
        let attacker = world
            .spawn((
                AttackAction {
                    target: Entity::PLACEHOLDER,
                },
                CellCoord::new(1, 0),
            ))
            .id();

        world.run_system_once(act_attack).unwrap();

        assert!(damages(&mut world).is_empty());
        assert!(attacked(&mut world).is_empty(), "nothing was swung at");
        assert!(
            world.get::<AttackAction>(attacker).is_none(),
            "action consumed"
        );
    }

    #[test]
    fn a_dead_target_is_skipped() {
        // The marked driver stays in the world; an attack planned
        // against it must not land on the corpse.
        let mut world = world(42);
        let target = world
            .spawn((ArmorClass(0), CellCoord::new(4, 1), Dead))
            .id();
        let attacker = world
            .spawn((
                AttackAction { target },
                blows(&[(1000, 4)]),
                CellCoord::new(1, 1),
            ))
            .id();

        world.run_system_once(act_attack).unwrap();

        assert!(damages(&mut world).is_empty());
        assert!(attacked(&mut world).is_empty(), "nothing was swung at");
        assert!(
            world.get::<AttackAction>(attacker).is_none(),
            "action consumed"
        );
    }

    #[test]
    fn an_attack_emits_its_fact() {
        // One blow lands for 4, one misses: the fact names attacker,
        // both cells, and the single landed amount.
        let seed = seed_with_percentile_at_least(10);
        let mut world = world(seed);
        let target = world.spawn((ArmorClass(0), CellCoord::new(4, 1))).id();
        let attacker = world
            .spawn((
                AttackAction { target },
                blows(&[(1000, 4), (0, 5)]),
                CellCoord::new(1, 1),
            ))
            .id();

        world.run_system_once(act_attack).unwrap();

        assert_eq!(
            attacked(&mut world),
            vec![Attacked {
                attacker,
                attacker_cell_coord: CellCoord::new(1, 1),
                target_cell_coord: CellCoord::new(4, 1),
                damage_amounts: vec![4],
            }]
        );
        assert!(
            world.get::<AttackAction>(attacker).is_none(),
            "action consumed"
        );
    }

    #[test]
    fn a_full_miss_emits_an_empty_fact() {
        let seed = seed_with_percentile_at_least(10);
        let mut world = world(seed);
        let target = world.spawn((ArmorClass(1000), CellCoord::new(4, 1))).id();
        let attacker = world
            .spawn((
                AttackAction { target },
                blows(&[(20, 4)]),
                CellCoord::new(1, 1),
            ))
            .id();

        world.run_system_once(act_attack).unwrap();

        assert_eq!(
            attacked(&mut world),
            vec![Attacked {
                attacker,
                attacker_cell_coord: CellCoord::new(1, 1),
                target_cell_coord: CellCoord::new(4, 1),
                damage_amounts: vec![],
            }]
        );
        assert!(
            world.get::<AttackAction>(attacker).is_none(),
            "action consumed"
        );
    }

    #[test]
    fn a_blows_free_attacker_emits_an_empty_fact() {
        // The swing happened (the action is spent), so the fact goes
        // out with an empty list.
        let mut world = world(42);
        let target = world.spawn((ArmorClass(0), CellCoord::new(4, 1))).id();
        let attacker = world
            .spawn((AttackAction { target }, CellCoord::new(1, 1)))
            .id();

        world.run_system_once(act_attack).unwrap();

        assert_eq!(
            attacked(&mut world),
            vec![Attacked {
                attacker,
                attacker_cell_coord: CellCoord::new(1, 1),
                target_cell_coord: CellCoord::new(4, 1),
                damage_amounts: vec![],
            }]
        );
        assert!(
            world.get::<AttackAction>(attacker).is_none(),
            "action consumed"
        );
    }
}
