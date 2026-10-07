//! The movement flag: raised while a glide has road left, dropped one
//! frame before the landing.

use bevy::prelude::*;

use crate::core::display::components::is_moving::IsMoving;
use crate::frontend::display::tween::components::pos_tween::PosTween;

/// Keep `IsMoving` in step with the glides: a tween with more than one
/// frame of road left holds the flag up (the world waits for its
/// pictures); what fits inside one frame's advance drops the flag a
/// frame early — the core plans the next action while the glide
/// finishes, and the next tween absorbs the remainder with no
/// standstill. A creature with no glide carries no flag.
pub fn update_is_moving(
    time: Res<Time>,
    mut commands: Commands,
    gliding: Query<(Entity, &PosTween), Without<IsMoving>>,
    flagged: Query<(Entity, Option<&PosTween>), With<IsMoving>>,
) {
    let step = time.delta_secs();
    for (entity, pos_tween) in &gliding {
        let remaining = pos_tween.interval - pos_tween.elapsed;
        if remaining > step {
            commands.entity(entity).insert(IsMoving);
        }
    }
    for (entity, pos_tween) in &flagged {
        let drops = match pos_tween {
            Some(pos_tween) => pos_tween.interval - pos_tween.elapsed <= step,
            None => true,
        };
        if drops {
            commands.entity(entity).remove::<IsMoving>();
        }
    }
}
