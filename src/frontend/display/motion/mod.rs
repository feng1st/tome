pub mod components;
pub mod constants;
pub mod systems;
pub mod utils;

use bevy::prelude::*;

use self::systems::is_performing::update_is_performing;
use self::systems::move_cells::move_cells;
use crate::frontend::display::display_phase::DisplayPhase;

/// Register the motion domain: glides arm and the flag speaks in the
/// Motion phase, ahead of the tweens' advance in Animate.
pub fn register(app: &mut App) {
    app.add_systems(
        Update,
        (move_cells, update_is_performing)
            .chain()
            .in_set(DisplayPhase::Motion),
    );
}
