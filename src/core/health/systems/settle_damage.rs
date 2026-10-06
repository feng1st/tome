//! Damage application and death handling: the one settle pass.

use bevy::prelude::*;

use crate::core::health::components::dead::Dead;
use crate::core::health::components::hit_points::HitPoints;
use crate::core::health::messages::damage::Damage;
use crate::core::monster::components::monster_index::MonsterIndex;
use crate::core::world_clock::components::world_driver::WorldDriver;

/// Apply every pending damage request, then handle the deaths it left
/// behind.
///
/// The drain is destructive on purpose. The system registers once per
/// resolve phase — one instance after the driver's action, one after
/// the world's — and reader state is per system instance, so two
/// ordinary `MessageReader`s would apply every request twice.
/// `Messages::drain` empties the buffer: whichever instance runs after
/// a request was written applies it, and the other finds nothing left.
///
/// Death is strictly negative hit points; zero is alive. A dead monster
/// is despawned — its entity leaves the world, presentation included. A
/// dead driver gains the `Dead` marker and stays: commands and planning
/// stop through the marker's readers, and the clock pins on the
/// driver's never-spent turn. A negative entity that is neither species
/// is left untouched — no third species exists. A request naming a
/// despawned target is dropped.
pub fn settle_damage(
    mut commands: Commands,
    mut damage_messages: ResMut<Messages<Damage>>,
    mut creatures: Query<(
        Entity,
        &mut HitPoints,
        Option<&MonsterIndex>,
        Option<&WorldDriver>,
    )>,
) {
    for damage in damage_messages.drain() {
        debug_assert!(
            damage.amount >= 0,
            "damage amount is non-negative by contract"
        );
        if let Ok((_, mut hit_points, _, _)) = creatures.get_mut(damage.target) {
            // No floor: negative results are legal. The current value
            // only decreases, so the ceiling invariant needs no clamp.
            hit_points.current -= damage.amount;
        }
    }
    for (entity, hit_points, monster, driver) in &mut creatures {
        if hit_points.current < 0 {
            if monster.is_some() {
                commands.entity(entity).despawn();
            } else if driver.is_some() {
                commands.entity(entity).insert(Dead);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::entity::Entity;

    use super::*;
    use crate::core::health;
    use crate::core::health::components::hit_points::HitPoints;
    use crate::core::monster::components::monster_index::MonsterIndex;

    fn app() -> App {
        let mut app = App::new();
        app.add_message::<Damage>()
            .add_systems(Update, settle_damage);
        app
    }

    fn spawn_monster(app: &mut App, current: i32, max: i32) -> Entity {
        app.world_mut()
            .spawn((MonsterIndex::from_index(0), HitPoints { current, max }))
            .id()
    }

    fn spawn_driver(app: &mut App, current: i32, max: i32) -> Entity {
        app.world_mut()
            .spawn((WorldDriver, HitPoints { current, max }))
            .id()
    }

    fn hit_points(app: &App, entity: Entity) -> i32 {
        app.world().get::<HitPoints>(entity).unwrap().current
    }

    #[test]
    fn damage_reduces_the_current_value() {
        let mut app = app();
        let rat = spawn_monster(&mut app, 12, 19);
        app.world_mut().write_message(Damage {
            target: rat,
            amount: 5,
        });
        app.update();
        assert_eq!(hit_points(&app, rat), 7);
        assert!(app.world().get::<HitPoints>(rat).is_some());
    }

    #[test]
    fn the_reduction_is_not_clamped_at_zero() {
        let mut app = app();
        let driver = spawn_driver(&mut app, 3, 19);
        app.world_mut().write_message(Damage {
            target: driver,
            amount: 5,
        });
        app.update();
        assert_eq!(hit_points(&app, driver), -2);
    }

    #[test]
    fn damage_never_lifts_the_current_value_past_the_ceiling() {
        let mut app = app();
        let rat = spawn_monster(&mut app, 3, 19);
        app.world_mut().write_message(Damage {
            target: rat,
            amount: 2,
        });
        app.update();
        let hit_points = app.world().get::<HitPoints>(rat).unwrap();
        assert!(hit_points.current <= hit_points.max);
    }

    #[test]
    fn a_monster_at_exactly_zero_is_alive_and_stays() {
        let mut app = app();
        let rat = spawn_monster(&mut app, 5, 19);
        app.world_mut().write_message(Damage {
            target: rat,
            amount: 5,
        });
        app.update();
        assert_eq!(hit_points(&app, rat), 0);
        assert!(app.world().get::<HitPoints>(rat).is_some());
    }

    #[test]
    fn a_monster_below_zero_leaves_the_world() {
        let mut app = app();
        let rat = spawn_monster(&mut app, 1, 19);
        app.world_mut().write_message(Damage {
            target: rat,
            amount: 2,
        });
        app.update();
        assert!(app.world().get::<HitPoints>(rat).is_none());
    }

    #[test]
    fn a_driver_below_zero_is_marked_and_stays() {
        let mut app = app();
        let driver = spawn_driver(&mut app, 1, 19);
        app.world_mut().write_message(Damage {
            target: driver,
            amount: 2,
        });
        app.update();
        assert!(app.world().get::<HitPoints>(driver).is_some());
        assert!(app.world().get::<Dead>(driver).is_some());
    }

    #[test]
    fn a_driver_at_exactly_zero_is_unmarked() {
        let mut app = app();
        let driver = spawn_driver(&mut app, 5, 19);
        app.world_mut().write_message(Damage {
            target: driver,
            amount: 5,
        });
        app.update();
        assert_eq!(hit_points(&app, driver), 0);
        assert!(app.world().get::<Dead>(driver).is_none());
    }

    /// The drain is what keeps the twin registrations honest: two
    /// instances in one frame would each read a shared buffer through
    /// ordinary readers and apply every request twice.
    #[test]
    fn two_instances_apply_each_request_once() {
        let mut app = App::new();
        app.add_message::<Damage>()
            .add_systems(Update, (settle_damage, settle_damage).chain());
        let rat = spawn_monster(&mut app, 12, 19);
        app.world_mut().write_message(Damage {
            target: rat,
            amount: 5,
        });
        app.update();
        assert_eq!(hit_points(&app, rat), 7, "applied once, not twice");
    }

    /// A drained buffer stays drained: the next frame finds nothing to
    /// apply, so the value holds.
    #[test]
    fn a_drained_request_leaves_nothing_for_the_next_frame() {
        let mut app = app();
        let rat = spawn_monster(&mut app, 12, 19);
        app.world_mut().write_message(Damage {
            target: rat,
            amount: 5,
        });
        app.update();
        app.update();
        assert_eq!(hit_points(&app, rat), 7);
    }

    /// The real registration wires the channel end to end: a request
    /// emitted before the frame lands exactly once through the
    /// registered twin instances.
    #[test]
    fn the_real_register_processes_the_channel() {
        let mut app = App::new();
        health::register(&mut app);
        let rat = spawn_monster(&mut app, 12, 19);
        app.world_mut().write_message(Damage {
            target: rat,
            amount: 5,
        });
        app.update();
        assert_eq!(hit_points(&app, rat), 7);
    }
}
