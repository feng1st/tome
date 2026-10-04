//! Move sprite: every creature's picture moves toward its logical
//! cell, one axis at a time — x and y each approach their target at
//! the same fixed pace, so a diagonal step lands exactly with a
//! straight step.

use bevy::prelude::*;

use crate::core::display::components::is_moving::IsMoving;
use crate::core::map::components::cell_coord::CellCoord;
use crate::frontend::display::motion::components::curr_position::CurrPosition;
use crate::frontend::display::motion::utils::position::{approach, reached};

/// Seconds per cell per axis, flat for every creature: speed is a logic
/// concept — turn frequency — never a picture pace.
const CELL_SECS: f32 = 0.1;

/// Move every picture toward its logical cell, one frame's worth per
/// axis, and keep `IsMoving` in step with the road left. The flag drops
/// one frame before the landing — what remains after that frame fits
/// inside one frame's step, so the core plans the next action while the
/// move finishes and the new target absorbs the remainder with no
/// standstill.
pub fn move_sprite(
    mut commands: Commands,
    time: Res<Time>,
    mut creatures: Query<(Entity, &CellCoord, &mut CurrPosition, Has<IsMoving>)>,
) {
    let step = time.delta_secs() / CELL_SECS;
    for (entity, cell, mut curr_position, is_moving_curr) in &mut creatures {
        let landed = reached(*curr_position, *cell, step);
        if landed {
            // Both axes land within this frame: arrive exactly.
            *curr_position = CurrPosition::from(*cell);
        } else {
            approach(&mut curr_position, *cell, step);
        }
        // The flag drops one frame before the landing: what remains
        // after this frame's motion fits inside one frame's step, so
        // the core plans the next action while the move finishes — the
        // new target then absorbs the remainder with no standstill.
        let lands_next_frame = reached(*curr_position, *cell, step);
        let is_moving_next = !landed && !lands_next_frame;
        if is_moving_next != is_moving_curr {
            if is_moving_next {
                commands.entity(entity).insert(IsMoving);
            } else {
                commands.entity(entity).remove::<IsMoving>();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    fn app() -> App {
        let mut app = App::new();
        app.insert_resource(Time::<()>::default())
            .add_systems(Update, move_sprite);
        app
    }

    fn advance(app: &mut App, secs: f32) {
        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(Duration::from_secs_f32(secs));
        app.update();
    }

    #[test]
    fn straight_move_lands_within_one_interval() {
        let mut app = app();
        let rat = app
            .world_mut()
            .spawn((
                CellCoord::new(1, 0),
                CurrPosition::from(CellCoord::new(0, 0)),
            ))
            .id();
        advance(&mut app, 0.05);
        let mid = *app.world().get::<CurrPosition>(rat).unwrap();
        assert!((mid.x - 0.5).abs() < 1e-6, "halfway at half the interval");
        advance(&mut app, 0.06);
        assert_eq!(
            *app.world().get::<CurrPosition>(rat).unwrap(),
            CurrPosition::from(CellCoord::new(1, 0)),
            "landing clamps exactly, no drift"
        );
    }

    /// A diagonal step covers both axes at once: same duration as a
    /// straight step, position moving through the shared boundary.
    #[test]
    fn diagonal_takes_the_same_time_as_straight() {
        let mut app = app();
        let rat = app
            .world_mut()
            .spawn((
                CellCoord::new(1, 1),
                CurrPosition::from(CellCoord::new(0, 0)),
            ))
            .id();
        advance(&mut app, 0.05);
        let mid = *app.world().get::<CurrPosition>(rat).unwrap();
        assert!((mid.x - 0.5).abs() < 1e-6 && (mid.y - 0.5).abs() < 1e-6);
        advance(&mut app, 0.06);
        assert_eq!(
            *app.world().get::<CurrPosition>(rat).unwrap(),
            CurrPosition::from(CellCoord::new(1, 1)),
            "diagonal lands exactly with the straight step"
        );
    }

    #[test]
    fn is_moving_tracks_the_move() {
        let mut app = app();
        let rat = app
            .world_mut()
            .spawn((
                CellCoord::new(1, 0),
                CurrPosition::from(CellCoord::new(0, 0)),
            ))
            .id();
        advance(&mut app, 0.02);
        assert!(
            app.world().get::<IsMoving>(rat).is_some(),
            "mid-move with a full frame of road left: the flag is up"
        );
        advance(&mut app, 0.06);
        let position = *app.world().get::<CurrPosition>(rat).unwrap();
        assert!(
            app.world().get::<IsMoving>(rat).is_none(),
            "one frame from landing: the flag releases early"
        );
        assert!(
            position.x > 0.5 && position.x < 1.0,
            "not arrived yet: pos = {}",
            position.x
        );
        advance(&mut app, 0.06);
        assert_eq!(
            *app.world().get::<CurrPosition>(rat).unwrap(),
            CurrPosition::from(CellCoord::new(1, 0)),
            "the landing completes without a standstill"
        );
    }

    #[test]
    fn flag_releases_one_frame_before_landing() {
        let mut app = app();
        let rat = app
            .world_mut()
            .spawn((
                CellCoord::new(1, 0),
                CurrPosition::from(CellCoord::new(0, 0)),
            ))
            .id();
        // One frame of motion leaves less than one frame's step: the
        // landing is next frame, and the flag must already be down —
        // the core plans while the move finishes.
        advance(&mut app, 0.06);
        let position = *app.world().get::<CurrPosition>(rat).unwrap();
        assert!((position.x - 0.6).abs() < 1e-6);
        assert!(app.world().get::<IsMoving>(rat).is_none());
        advance(&mut app, 0.05);
        assert_eq!(
            *app.world().get::<CurrPosition>(rat).unwrap(),
            CurrPosition::from(CellCoord::new(1, 0))
        );
    }

    #[test]
    fn retarget_mid_move_never_stalls() {
        let mut app = app();
        let rat = app
            .world_mut()
            .spawn((
                CellCoord::new(1, 0),
                CurrPosition::from(CellCoord::new(0, 0)),
            ))
            .id();
        advance(&mut app, 0.05);
        // The cell coordinate changes again mid-move: the picture retargets
        // and keeps the same pace — position never returns to zero gap.
        *app.world_mut()
            .entity_mut(rat)
            .get_mut::<CellCoord>()
            .unwrap() = CellCoord::new(2, 0);
        advance(&mut app, 0.05);
        let mid = *app.world().get::<CurrPosition>(rat).unwrap();
        assert!(
            mid.x >= 1.0 && mid.x < 2.0,
            "reaches the boundary and keeps moving"
        );
        advance(&mut app, 0.15);
        assert_eq!(
            *app.world().get::<CurrPosition>(rat).unwrap(),
            CurrPosition::from(CellCoord::new(2, 0))
        );
    }

    #[test]
    fn idle_creature_holds_no_flag() {
        let mut app = app();
        app.world_mut().spawn((
            CellCoord::new(1, 0),
            CurrPosition::from(CellCoord::new(1, 0)),
        ));
        advance(&mut app, 0.3);
        let flags = app
            .world_mut()
            .query::<&IsMoving>()
            .iter(app.world())
            .count();
        assert_eq!(flags, 0, "an idle creature holds no flag");
    }
}
