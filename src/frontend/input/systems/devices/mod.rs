//! Device translation systems: one file per input device (mouse, keyboard,
//! touch), each translating all of its device's controls into gestures.

pub mod mouse;

use bevy::prelude::*;

use crate::frontend::input::input_phase::InputPhase;

/// Register device translators into the Translate phase. A new device adds
/// a file above and a line here.
pub fn register(app: &mut App) {
    app.add_systems(Update, mouse::translate.in_set(InputPhase::Translate));
}
