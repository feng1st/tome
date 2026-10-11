//! The heavy-hit shake trigger: a hard blow to the player shakes the
//! camera.

use bevy::prelude::*;

use crate::core::hit_points::messages::damaged::Damaged;
use crate::core::player::components::player::Player;
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
    player: Query<(), With<Player>>,
    mut shake_camera_writer: MessageWriter<ShakeCamera>,
) {
    for damage in damaged_reader.read() {
        if player.get(damage.target).is_err() {
            continue;
        }
        let quarter = damage.max_hp / 4;
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
