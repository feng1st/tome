// TODO: pending cleanup review — remove once stabilized
//! Input: devices translate raw input (PC mouse today) into
//! display-independent gestures, then resolvers interpret gestures into
//! core commands. Message registration lives in `gestures/`; system
//! registration sinks to each group (phases carry the ordering); the root
//! only orchestrates the phase chain.

pub mod gestures;
pub mod input_phase;
pub mod systems;

use bevy::prelude::*;

use crate::core::game_loop::GameLoop;
use input_phase::InputPhase;

/// Register the input domain: gesture message types, both system groups,
/// and the phase ordering (translate before resolve, inside
/// `GameLoop::Input`).
pub fn register(app: &mut App) {
    gestures::register(app);
    systems::register(app);
    app.configure_sets(
        Update,
        (InputPhase::Translate, InputPhase::Resolve)
            .chain()
            .in_set(GameLoop::Input),
    );
}
