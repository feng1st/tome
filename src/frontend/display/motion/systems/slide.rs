//! Slide: every creature's picture glides toward its logical cell, one
//! axis at a time — x and y each approach their target at the same fixed
//! pace, so a diagonal step lands exactly with a straight step.

use bevy::prelude::*;

use crate::core::map::components::cell_coord::CellCoord;
use crate::core::time::components::presenting::Presenting;
use crate::frontend::display::motion::components::presentation_position::PresentationPosition;

/// Seconds per cell per axis, flat for every creature (PD's
/// `MOVE_INTERVAL`): speed is a logic concept — turn frequency — never a
/// picture pace.
const CELL_SECS: f32 = 0.1;

/// Glide every picture toward its logical cell, one frame's worth per
/// axis, and hold the presentation gate while the glide has road left.
/// The gate drops one frame before the landing — what remains after that
/// frame fits inside one frame's step, so the core plans the next action
/// while the slide finishes and the new target absorbs the remainder
/// with no standstill.
pub fn slide(
    mut commands: Commands,
    time: Res<Time>,
    mut creatures: Query<(
        Entity,
        &CellCoord,
        &mut PresentationPosition,
        Has<Presenting>,
    )>,
) {
    let step = time.delta_secs() / CELL_SECS;
    for (entity, cell, mut presentation, presenting) in &mut creatures {
        let current_x = presentation.x;
        let current_y = presentation.y;
        let landed = rem(current_x, cell.x as f32) <= step && rem(current_y, cell.y as f32) <= step;
        if landed {
            // Both axes land within this frame: arrive exactly.
            *presentation = PresentationPosition::from(*cell);
        } else {
            presentation.x = approach(current_x, cell.x as f32, step);
            presentation.y = approach(current_y, cell.y as f32, step);
        }
        // The gate drops one frame before the landing: what remains after
        // this frame's motion fits inside one frame's step, so the core
        // plans the next action while the slide finishes — the new target
        // then absorbs the remainder with no standstill.
        let lands_next_frame = rem(presentation.x, cell.x as f32) <= step
            && rem(presentation.y, cell.y as f32) <= step;
        let hold_gate = !landed && !lands_next_frame;
        match (hold_gate, presenting) {
            (true, false) => {
                commands.entity(entity).insert(Presenting);
            }
            (false, true) => {
                commands.entity(entity).remove::<Presenting>();
            }
            _ => {}
        }
    }
}

/// Move `current` toward `target` by at most `step`, landing exactly.
fn approach(current: f32, target: f32, step: f32) -> f32 {
    let remaining = target - current;
    if remaining.abs() <= step {
        target
    } else {
        current + remaining.signum() * step
    }
}

/// Axis distance between the picture and the cell.
fn rem(picture: f32, cell: f32) -> f32 {
    (cell - picture).abs()
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    fn app() -> App {
        let mut app = App::new();
        app.insert_resource(Time::<()>::default())
            .add_systems(Update, slide);
        app
    }

    fn advance(app: &mut App, secs: f32) {
        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(Duration::from_secs_f32(secs));
        app.update();
    }

    #[test]
    fn straight_glide_lands_within_one_interval() {
        let mut app = app();
        let rat = app
            .world_mut()
            .spawn((
                CellCoord::new(1, 0),
                PresentationPosition::from(CellCoord::new(0, 0)),
            ))
            .id();
        advance(&mut app, 0.05);
        let mid = *app.world().get::<PresentationPosition>(rat).unwrap();
        assert!((mid.x - 0.5).abs() < 1e-6, "halfway at half the interval");
        advance(&mut app, 0.06);
        assert_eq!(
            *app.world().get::<PresentationPosition>(rat).unwrap(),
            PresentationPosition::from(CellCoord::new(1, 0)),
            "landing clamps exactly, no drift"
        );
    }

    /// A diagonal step covers both axes at once: same duration as a
    /// straight step, position gliding through the shared boundary.
    #[test]
    fn diagonal_takes_the_same_time_as_straight() {
        let mut app = app();
        let rat = app
            .world_mut()
            .spawn((
                CellCoord::new(1, 1),
                PresentationPosition::from(CellCoord::new(0, 0)),
            ))
            .id();
        advance(&mut app, 0.05);
        let mid = *app.world().get::<PresentationPosition>(rat).unwrap();
        assert!((mid.x - 0.5).abs() < 1e-6 && (mid.y - 0.5).abs() < 1e-6);
        advance(&mut app, 0.06);
        assert_eq!(
            *app.world().get::<PresentationPosition>(rat).unwrap(),
            PresentationPosition::from(CellCoord::new(1, 1)),
            "diagonal lands exactly with the straight step"
        );
    }

    #[test]
    fn gate_tracks_the_glide() {
        let mut app = app();
        let rat = app
            .world_mut()
            .spawn((
                CellCoord::new(1, 0),
                PresentationPosition::from(CellCoord::new(0, 0)),
            ))
            .id();
        advance(&mut app, 0.02);
        assert!(
            app.world().get::<Presenting>(rat).is_some(),
            "mid-glide with a full frame of road left: gate up"
        );
        advance(&mut app, 0.06);
        let pos = *app.world().get::<PresentationPosition>(rat).unwrap();
        assert!(
            app.world().get::<Presenting>(rat).is_none(),
            "one frame from landing: the gate releases early"
        );
        assert!(
            pos.x > 0.5 && pos.x < 1.0,
            "not arrived yet: pos = {}",
            pos.x
        );
        advance(&mut app, 0.06);
        assert_eq!(
            *app.world().get::<PresentationPosition>(rat).unwrap(),
            PresentationPosition::from(CellCoord::new(1, 0)),
            "the landing completes without a standstill"
        );
    }

    #[test]
    fn gate_releases_one_frame_before_landing() {
        let mut app = app();
        let rat = app
            .world_mut()
            .spawn((
                CellCoord::new(1, 0),
                PresentationPosition::from(CellCoord::new(0, 0)),
            ))
            .id();
        // One frame of motion leaves less than one frame's step: the
        // landing is next frame, and the gate must already be down —
        // the core plans while the slide finishes.
        advance(&mut app, 0.06);
        let pos = *app.world().get::<PresentationPosition>(rat).unwrap();
        assert!((pos.x - 0.6).abs() < 1e-6);
        assert!(app.world().get::<Presenting>(rat).is_none());
        advance(&mut app, 0.05);
        assert_eq!(
            *app.world().get::<PresentationPosition>(rat).unwrap(),
            PresentationPosition::from(CellCoord::new(1, 0))
        );
    }

    #[test]
    fn retarget_mid_glide_never_stalls() {
        let mut app = app();
        let rat = app
            .world_mut()
            .spawn((
                CellCoord::new(1, 0),
                PresentationPosition::from(CellCoord::new(0, 0)),
            ))
            .id();
        advance(&mut app, 0.05);
        // The logical cell flips again mid-glide: the glide retargets and
        // keeps the same pace — position never returns to zero gap.
        *app.world_mut()
            .entity_mut(rat)
            .get_mut::<CellCoord>()
            .unwrap() = CellCoord::new(2, 0);
        advance(&mut app, 0.05);
        let mid = *app.world().get::<PresentationPosition>(rat).unwrap();
        assert!(
            mid.x >= 1.0 && mid.x < 2.0,
            "reaches the boundary and keeps gliding"
        );
        advance(&mut app, 0.15);
        assert_eq!(
            *app.world().get::<PresentationPosition>(rat).unwrap(),
            PresentationPosition::from(CellCoord::new(2, 0))
        );
    }

    #[test]
    fn idle_creature_holds_no_gate() {
        let mut app = app();
        app.world_mut().spawn((
            CellCoord::new(1, 0),
            PresentationPosition::from(CellCoord::new(1, 0)),
        ));
        advance(&mut app, 0.3);
        let gates = app
            .world_mut()
            .query::<&Presenting>()
            .iter(app.world())
            .count();
        assert_eq!(gates, 0, "an idle creature holds no gate");
    }
}
