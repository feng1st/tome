//! The motion domain: a step's picture glides toward its landed cell,
//! plus the `IsMoving` flag the core reads. One mechanism over the
//! tween domain's `PosTween`: this domain decides when a glide starts
//! (the logical cell changed) and when the flag speaks (road left).

pub mod components;
pub mod constants;
pub mod systems;
pub mod utils;

use bevy::prelude::*;

use self::systems::is_moving::update_is_moving;
use self::systems::move_cells::move_cells;
use crate::frontend::display::display_phase::DisplayPhase;

/// Register the motion domain: glides arm and flags speak in the
/// Motion phase, ahead of the tweens' advance in Animate.
pub fn register(app: &mut App) {
    app.add_systems(
        Update,
        (move_cells, update_is_moving)
            .chain()
            .in_set(DisplayPhase::Motion),
    );
}
