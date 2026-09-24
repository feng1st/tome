//! Input: devices translate raw input (PC mouse today) into
//! display-independent gestures, then resolvers interpret gestures into
//! core commands. New devices and new gestures add their own files and a
//! line in their group's register; this module never names them.

pub mod gestures;
pub mod input_phase;
pub mod systems;

use bevy::prelude::*;

use crate::core::frame_phase::FramePhase;
use input_phase::InputPhase;

/// Register the input domain: gesture message types, both system groups,
/// and the internal phase ordering (translate before resolve, both inside
/// `FramePhase::Input`).
pub fn register(app: &mut App) {
    gestures::register(app);
    systems::register(app);
    app.configure_sets(
        Update,
        (InputPhase::Translate, InputPhase::Resolve)
            .chain()
            .in_set(FramePhase::Input),
    );
}
