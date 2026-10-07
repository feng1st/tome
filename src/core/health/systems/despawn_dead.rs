//! Despawn the dead whose disappearance is done.

use bevy::prelude::*;

use crate::core::display::components::is_disappearing::IsDisappearing;
use crate::core::health::components::dead::Dead;
use crate::core::world_clock::components::world_driver::WorldDriver;

/// Remove every dead body whose disappearance has run its course. A
/// body still disappearing — its death animation and fade held under
/// the display side's `IsDisappearing` flag — stays until the flag
/// drops; a body nobody claimed (no display side running: headless)
/// leaves on this same sweep, so death alone empties a headless world.
/// The driver is never removed: its death presentation is the frozen
/// world itself, and the death wrap-up owns its exit.
///
/// Runs in the Derive phase, ahead of the resolve phases where new
/// deaths land: a same-frame death can never be seen here unfaded —
/// by the time this sweep next runs, the display side has raised the
/// flag over any fresh body, so an unflagged corpse is always one
/// whose presentation is over (or one that never had one).
/// The dead filter — bodies this sweep may take: dead, no
/// disappearance running, and not the driver.
type DeadFilter = (With<Dead>, Without<IsDisappearing>, Without<WorldDriver>);

pub fn despawn_dead(mut commands: Commands, dead: Query<Entity, DeadFilter>) {
    for entity in &dead {
        commands.entity(entity).despawn();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::display::components::is_disappearing::IsDisappearing;
    use crate::core::health::components::dead::Dead;
    use crate::core::health::components::hit_points::HitPoints;
    use crate::core::map::components::cell_coord::CellCoord;
    use crate::core::world_clock::components::world_driver::WorldDriver;

    fn app() -> App {
        let mut app = App::new();
        app.add_systems(Update, despawn_dead);
        app
    }

    fn spawn_dead_body(app: &mut App) -> Entity {
        app.world_mut()
            .spawn((
                CellCoord::new(1, 1),
                HitPoints {
                    current: -1,
                    max: 19,
                },
                Dead,
            ))
            .id()
    }

    #[test]
    fn an_unclaimed_body_leaves() {
        let mut app = app();
        let body = spawn_dead_body(&mut app);
        app.update();
        assert!(
            app.world().get_entity(body).is_err(),
            "headless death empties the world on the next sweep"
        );
    }

    #[test]
    fn a_disappearing_body_stays() {
        let mut app = app();
        let body = spawn_dead_body(&mut app);
        app.world_mut().entity_mut(body).insert(IsDisappearing);
        app.update();
        assert!(
            app.world().get_entity(body).is_ok(),
            "the presentation holds the body"
        );
    }

    #[test]
    fn the_dead_driver_stays() {
        let mut app = app();
        let driver = app
            .world_mut()
            .spawn((WorldDriver, CellCoord::new(1, 1), Dead))
            .id();
        app.update();
        app.update();
        assert!(
            app.world().get_entity(driver).is_ok(),
            "the driver's exit belongs to the death wrap-up"
        );
    }

    #[test]
    fn the_living_are_untouched() {
        let mut app = app();
        let creature = app
            .world_mut()
            .spawn((
                CellCoord::new(1, 1),
                HitPoints {
                    current: 5,
                    max: 19,
                },
            ))
            .id();
        app.update();
        assert!(app.world().get_entity(creature).is_ok());
    }

    /// The frame-ordering invariant: a death that lands later in the
    /// frame (a resolve phase) survives this sweep — the display side
    /// raises its flag in between — and leaves on the next one once
    /// the flag drops.
    #[test]
    fn a_same_frame_death_is_never_swept_early() {
        let mut app = App::new();
        // Derive runs despawn_dead first; the resolve phase afterwards
        // marks a death and the display step raises the flag before the
        // next frame's sweep.
        app.add_systems(
            Update,
            (despawn_dead, |mut commands: Commands| {
                commands.spawn((
                    CellCoord::new(1, 1),
                    HitPoints {
                        current: -1,
                        max: 19,
                    },
                    Dead,
                    IsDisappearing,
                ));
            })
                .chain(),
        );
        app.update();
        let mut bodies = app.world_mut().query_filtered::<Entity, With<Dead>>();
        let body = bodies.single(app.world()).unwrap();
        // The flag drops; the next sweep takes the body.
        app.world_mut().entity_mut(body).remove::<IsDisappearing>();
        app.update();
        assert!(app.world().get_entity(body).is_err());
    }
}
