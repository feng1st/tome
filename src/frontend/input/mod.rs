//! Input modalities and gesture resolution: translate raw input (PC mouse
//! today) into display-independent gestures, then resolve gestures into
//! core commands. New modalities (touch, keyboard cursor) add their own
//! systems here; the assembly layer never names them.

pub mod gestures;
pub mod systems;

use bevy::prelude::*;

use crate::core::system_sets::InputSet;

/// Register gesture message types (delegated to the gestures side) and the
/// input chain into `InputSet`: modalities emit gestures, resolvers turn
/// them into core commands.
pub fn register(app: &mut App) {
    gestures::register(app);
    app.add_systems(
        Update,
        (
            systems::mouse::left_click::left_click,
            systems::gestures::primary_action_on_cell::resolve,
        )
            .chain()
            .in_set(InputSet),
    );
}
