//! The input plugin: translates raw input (PC mouse today) into
//! display-independent core gestures. Replaceable as a whole — e.g. with
//! touch or keyboard-cursor input — without touching the core.

pub mod systems;

use bevy::prelude::*;

use crate::core::sets::InputSet;

/// Register input systems into `InputSet`. New modalities (touch, keyboard
/// cursor) add their own systems here; the assembly layer never names them.
pub fn register(app: &mut App) {
    app.add_systems(Update, systems::mouse::mouse_primary_action.in_set(InputSet));
}
