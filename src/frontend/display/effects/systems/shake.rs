//! The heavy-hit shake trigger: a hard blow to the player shakes the
//! camera.

use bevy::prelude::*;

use crate::core::health::messages::damaged::Damaged;
use crate::core::world_clock::components::world_driver::WorldDriver;
use crate::frontend::display::camera::messages::shake_camera::ShakeCamera;

/// The shake duration in seconds.
const SHAKE_SECS: f32 = 0.3;

/// Present every heavy hit to the player: damage exceeding a quarter
/// of the maximum requests a camera shake — the magnitude is the
/// amount over the quarter, integer-divided, gated to the 1..5 band;
/// damage at or below the quarter requests nothing. The quarter is
/// integer arithmetic; a maximum under four makes the quarter zero,
/// in which case any positive damage is heavy and the magnitude
/// saturates the band.
pub fn shake(
    mut damaged_reader: MessageReader<Damaged>,
    drivers: Query<(), With<WorldDriver>>,
    mut shake_camera_writer: MessageWriter<ShakeCamera>,
) {
    for damage in damaged_reader.read() {
        if drivers.get(damage.target).is_err() {
            continue;
        }
        let quarter = damage.max / 4;
        if damage.amount <= quarter {
            continue;
        }
        // A zero quarter (a maximum under four) makes the proportion
        // unbounded: the magnitude saturates the band.
        let magnitude = if quarter == 0 {
            5
        } else {
            (damage.amount / quarter).clamp(1, 5)
        };
        shake_camera_writer.write(ShakeCamera {
            magnitude: magnitude as f32,
            duration: SHAKE_SECS,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::health::messages::damaged::Damaged;
    use crate::core::map::components::cell_coord::CellCoord;

    fn app() -> App {
        let mut app = App::new();
        app.init_resource::<Messages<Damaged>>()
            .init_resource::<Messages<ShakeCamera>>()
            .add_systems(Update, shake);
        app
    }

    fn shake_requests(app: &mut App) -> Vec<ShakeCamera> {
        app.world_mut()
            .resource_mut::<Messages<ShakeCamera>>()
            .drain()
            .collect()
    }

    #[test]
    fn a_heavy_hit_shakes() {
        let mut app = app();
        let driver = app.world_mut().spawn(WorldDriver).id();
        app.world_mut().write_message(Damaged {
            target: driver,
            cell: CellCoord::new(1, 1),
            source_cell: None,
            amount: 10,
            max: 20,
        });
        app.update();
        assert_eq!(
            shake_requests(&mut app),
            vec![ShakeCamera {
                magnitude: 2.0,
                duration: 0.3
            }]
        );
    }

    #[test]
    fn a_light_hit_stays_still() {
        let mut app = app();
        let driver = app.world_mut().spawn(WorldDriver).id();
        app.world_mut().write_message(Damaged {
            target: driver,
            cell: CellCoord::new(1, 1),
            source_cell: None,
            amount: 5,
            max: 20,
        });
        app.update();
        assert!(shake_requests(&mut app).is_empty());
    }

    #[test]
    fn a_hard_monster_wound_never_shakes() {
        let mut app = app();
        let rat = app.world_mut().spawn(()).id();
        app.world_mut().write_message(Damaged {
            target: rat,
            cell: CellCoord::new(1, 1),
            source_cell: None,
            amount: 10,
            max: 12,
        });
        app.update();
        assert!(shake_requests(&mut app).is_empty());
    }

    #[test]
    fn a_tiny_maximum_saturates_the_band() {
        let mut app = app();
        // Maximum 3: the quarter is zero, any wound is heavy, and the
        // magnitude saturates at five.
        let driver = app.world_mut().spawn(WorldDriver).id();
        app.world_mut().write_message(Damaged {
            target: driver,
            cell: CellCoord::new(1, 1),
            source_cell: None,
            amount: 2,
            max: 3,
        });
        app.update();
        assert_eq!(
            shake_requests(&mut app),
            vec![ShakeCamera {
                magnitude: 5.0,
                duration: 0.3
            }]
        );
    }
}
