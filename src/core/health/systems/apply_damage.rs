//! Damage application and death handling: the one apply pass.

use bevy::ecs::query::QueryData;
use bevy::prelude::*;

use crate::core::health::components::dead::Dead;
use crate::core::health::components::hit_points::HitPoints;
use crate::core::health::messages::damage::Damage;
use crate::core::health::messages::damage_applied::DamageApplied;
use crate::core::map::components::cell_coord::CellCoord;
use crate::core::player::components::order::Order;

/// One woundable creature's apply inputs: identity, the life record,
/// the cell for the facts, and the death marker both loops respect.
#[derive(QueryData)]
#[query_data(mutable)]
pub struct WoundableQuery {
    pub entity: Entity,
    pub hit_points: &'static mut HitPoints,
    pub cell: &'static CellCoord,
    pub dead: Has<Dead>,
}

/// Apply every pending damage request, then mark the deaths it left
/// behind — and broadcast the applications as past-tense facts.
///
/// The drain is destructive on purpose. The system registers once per
/// resolve phase — one instance after the driver's action, one after
/// the world's — and reader state is per system instance, so two
/// ordinary `MessageReader`s would apply every request twice.
/// `Messages::drain` empties the buffer: whichever instance runs after
/// a request was written applies it, and the other finds nothing left.
/// The facts ride the same guarantee: the applying instance emits them,
/// so each request is fact-stamped exactly once.
///
/// The dead are not woundable: a request naming a dead creature (its
/// body may linger as a death presentation) drops, and the sweep marks
/// only the unmarked — no re-handling, no re-marking.
///
/// Death is uniform: any creature whose current hit points crossed
/// below zero gains the death marker, and nothing else happens here.
/// What death means for a creature's schedule, its body, and its
/// departure belongs to the domains that own those things: the clock
/// drops dead turns (the driver's excepted — its unspent turn is the
/// game-over hold), the body plays its death presentation until it
/// releases the world, and `despawn_dead` takes what remains.
///
/// A landed blow interrupts: applying damage strips the target's
/// standing order, so whatever the creature was doing is dropped and
/// (for the driver) the world waits for fresh input.
pub fn apply_damage(
    mut commands: Commands,
    cells: Query<&CellCoord>,
    mut damage_messages: ResMut<Messages<Damage>>,
    mut damages_applied: MessageWriter<DamageApplied>,
    mut creatures: Query<WoundableQuery>,
) {
    for damage in damage_messages.drain() {
        debug_assert!(
            damage.amount >= 0,
            "damage amount is non-negative by contract"
        );
        if let Ok(mut target) = creatures.get_mut(damage.target) {
            if target.dead {
                // The dead are beyond wounding: their bodies linger
                // only as a presentation, and a stray request onto one
                // drops.
                continue;
            }
            // No floor: negative results are legal. The current value
            // only decreases, so the maximum invariant needs no clamp.
            target.hit_points.current -= damage.amount;
            // Attribution resolves at application: a source that left
            // the world mid-drain reads as none.
            let source_cell = damage
                .source
                .and_then(|source| cells.get(source).ok().copied());
            damages_applied.write(DamageApplied {
                target: damage.target,
                cell: *target.cell,
                source_cell,
                amount: damage.amount,
                max: target.hit_points.max,
            });
            // Being hit interrupts: a landed blow strips the target's
            // standing order. A target without an order (every monster)
            // is a harmless no-op.
            commands.entity(damage.target).remove::<Order>();
        }
    }
    for creature in &mut creatures {
        if creature.hit_points.current < 0 && !creature.dead {
            commands.entity(creature.entity).insert(Dead);
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::entity::Entity;

    use super::*;
    use crate::core::health;
    use crate::core::health::components::hit_points::HitPoints;
    use crate::core::map::components::cell_coord::CellCoord;

    fn app() -> App {
        let mut app = App::new();
        app.add_message::<Damage>()
            .add_message::<DamageApplied>()
            .add_systems(Update, apply_damage);
        app
    }

    fn spawn_creature(app: &mut App, current: i32, max: i32) -> Entity {
        app.world_mut()
            .spawn((CellCoord::new(3, 1), HitPoints { current, max }))
            .id()
    }

    fn hit_points(app: &App, entity: Entity) -> i32 {
        app.world().get::<HitPoints>(entity).unwrap().current
    }

    fn damages_applied(app: &mut App) -> Vec<DamageApplied> {
        app.world_mut()
            .resource_mut::<Messages<DamageApplied>>()
            .drain()
            .collect()
    }

    #[test]
    fn damage_reduces_the_current_value() {
        let mut app = app();
        let creature = spawn_creature(&mut app, 12, 19);
        app.world_mut().write_message(Damage {
            target: creature,
            source: None,
            amount: 5,
        });
        app.update();
        assert_eq!(hit_points(&app, creature), 7);
        assert!(app.world().get::<HitPoints>(creature).is_some());
    }

    #[test]
    fn the_reduction_is_not_clamped_at_zero() {
        let mut app = app();
        let creature = spawn_creature(&mut app, 3, 19);
        app.world_mut().write_message(Damage {
            target: creature,
            source: None,
            amount: 5,
        });
        app.update();
        assert_eq!(hit_points(&app, creature), -2);
    }

    #[test]
    fn damage_never_lifts_the_current_value_past_the_maximum() {
        let mut app = app();
        let creature = spawn_creature(&mut app, 3, 19);
        app.world_mut().write_message(Damage {
            target: creature,
            source: None,
            amount: 2,
        });
        app.update();
        let hit_points = app.world().get::<HitPoints>(creature).unwrap();
        assert!(hit_points.current <= hit_points.max);
    }

    #[test]
    fn a_creature_at_exactly_zero_is_alive_and_unmarked() {
        let mut app = app();
        let creature = spawn_creature(&mut app, 5, 19);
        app.world_mut().write_message(Damage {
            target: creature,
            source: None,
            amount: 5,
        });
        app.update();
        assert_eq!(hit_points(&app, creature), 0);
        assert!(app.world().get::<HitPoints>(creature).is_some());
        assert!(app.world().get::<Dead>(creature).is_none(), "zero is alive");
    }

    #[test]
    fn below_zero_marks_the_dead_and_the_body_stays() {
        let mut app = app();
        let creature = spawn_creature(&mut app, 1, 19);
        app.world_mut().write_message(Damage {
            target: creature,
            source: None,
            amount: 2,
        });
        app.update();
        // Death is the marker; the body's departure is despawn_dead's,
        // not this pass.
        assert!(app.world().get::<Dead>(creature).is_some());
        assert!(app.world().get::<HitPoints>(creature).is_some());
    }

    /// The drain is what keeps the twin registrations honest: two
    /// instances in one frame would each read a shared buffer through
    /// ordinary readers and apply every request twice.
    #[test]
    fn two_instances_apply_each_request_once() {
        let mut app = App::new();
        app.add_message::<Damage>()
            .add_message::<DamageApplied>()
            .add_systems(Update, (apply_damage, apply_damage).chain());
        let creature = spawn_creature(&mut app, 12, 19);
        app.world_mut().write_message(Damage {
            target: creature,
            source: None,
            amount: 5,
        });
        app.update();
        assert_eq!(hit_points(&app, creature), 7, "applied once, not twice");
        assert_eq!(
            damages_applied(&mut app).len(),
            1,
            "the fact is stamped once, not twice"
        );
    }

    /// A drained buffer stays drained: the next frame finds nothing to
    /// apply, so the value holds.
    #[test]
    fn a_drained_request_leaves_nothing_for_the_next_frame() {
        let mut app = app();
        let creature = spawn_creature(&mut app, 12, 19);
        app.world_mut().write_message(Damage {
            target: creature,
            source: None,
            amount: 5,
        });
        app.update();
        app.update();
        assert_eq!(hit_points(&app, creature), 7);
    }

    /// The real registration wires the channel end to end: a request
    /// emitted before the frame lands exactly once through the
    /// registered twin instances.
    #[test]
    fn the_real_register_processes_the_channel() {
        let mut app = App::new();
        health::register(&mut app);
        let creature = spawn_creature(&mut app, 12, 19);
        app.world_mut().write_message(Damage {
            target: creature,
            source: None,
            amount: 5,
        });
        app.update();
        assert_eq!(hit_points(&app, creature), 7);
    }

    /// A landed blow interrupts: the struck driver drops its standing
    /// order along with the hit points.
    #[test]
    fn a_hit_strips_the_target_s_standing_order() {
        let mut app = app();
        let creature = spawn_creature(&mut app, 12, 19);
        app.world_mut()
            .entity_mut(creature)
            .insert(Order::Attack { target: creature });
        app.world_mut().write_message(Damage {
            target: creature,
            source: None,
            amount: 5,
        });
        app.update();
        assert_eq!(hit_points(&app, creature), 7);
        assert!(app.world().entity(creature).get::<Order>().is_none());
    }

    /// No damage, no interruption: an undamaged creature keeps its
    /// orders.
    #[test]
    fn an_undamaged_creature_keeps_its_orders() {
        let mut app = app();
        let creature = spawn_creature(&mut app, 12, 19);
        app.world_mut().entity_mut(creature).insert(Order::Move {
            target: CellCoord::new(3, 3),
        });
        let other = spawn_creature(&mut app, 12, 19);
        app.world_mut().write_message(Damage {
            target: other,
            source: None,
            amount: 5,
        });
        app.update();
        assert!(app.world().entity(creature).get::<Order>().is_some());
    }

    /// Each applied request fact-stamps itself: target, both cells,
    /// amount, and maximum.
    #[test]
    fn an_applied_request_emits_its_fact() {
        let mut app = app();
        let wounded = spawn_creature(&mut app, 12, 19);
        let source = spawn_creature(&mut app, 19, 19);
        app.world_mut().write_message(Damage {
            target: wounded,
            source: Some(source),
            amount: 5,
        });
        app.update();
        assert_eq!(
            damages_applied(&mut app),
            vec![DamageApplied {
                target: wounded,
                cell: CellCoord::new(3, 1),
                source_cell: Some(CellCoord::new(3, 1)),
                amount: 5,
                max: 19,
            }]
        );
    }

    /// A source that left the world before application reads as none —
    /// attribution never resurrects a lookup.
    #[test]
    fn a_vanished_source_reads_as_none() {
        let mut app = app();
        let wounded = spawn_creature(&mut app, 12, 19);
        let ghost_source = app.world_mut().spawn_empty().id();
        app.world_mut().write_message(Damage {
            target: wounded,
            source: Some(ghost_source),
            amount: 5,
        });
        // The source despawns after the request was written, before the
        // apply pass runs.
        app.world_mut().entity_mut(ghost_source).despawn();
        app.update();
        let facts = damages_applied(&mut app);
        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].source_cell, None);
    }

    /// The dead are not woundable: a stray request onto a body that
    /// lingers as a death presentation drops without a fact.
    #[test]
    fn a_request_onto_a_dead_body_drops() {
        let mut app = app();
        let body = spawn_creature(&mut app, 1, 19);
        app.world_mut().write_message(Damage {
            target: body,
            source: None,
            amount: 2,
        });
        app.update();
        assert!(app.world().get::<Dead>(body).is_some());
        let before = hit_points(&app, body);
        // Clear the killing blow's own fact before the probe.
        damages_applied(&mut app);
        // A second request onto the marked body: no change, no fact.
        app.world_mut().write_message(Damage {
            target: body,
            source: None,
            amount: 4,
        });
        app.update();
        assert_eq!(hit_points(&app, body), before);
        assert!(damages_applied(&mut app).is_empty());
    }

    /// The sweep marks only the unmarked: a body already carrying the
    /// marker passes through unhandled, however many sweeps run.
    #[test]
    fn the_sweep_never_remarks_the_marked() {
        let mut app = App::new();
        app.add_message::<Damage>()
            .add_message::<DamageApplied>()
            .add_systems(Update, (apply_damage, apply_damage).chain());
        let creature = spawn_creature(&mut app, 1, 19);
        app.world_mut().write_message(Damage {
            target: creature,
            source: None,
            amount: 5,
        });
        // Two frames, two sweeps each: the marker exists, the handling
        // never repeats.
        app.update();
        app.update();
        assert!(app.world().get::<Dead>(creature).is_some());
        assert!(app.world().get::<HitPoints>(creature).is_some());
    }
}
