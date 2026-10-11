pub mod components;
pub mod messages;
pub mod systems;
pub mod types;

use bevy::prelude::*;

use self::messages::anim_finished::AnimFinished;
use crate::frontend::display::display_phase::DisplayPhase;

pub fn register(app: &mut App) {
    app.add_message::<AnimFinished>().add_systems(
        Update,
        (
            systems::sync_animation_and_direction::sync_animation_and_direction
                .in_set(DisplayPhase::Sync),
            systems::animate::animate.in_set(DisplayPhase::Animate),
        ),
    );
}
