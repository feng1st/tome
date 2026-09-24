//! Ordering phases inside the input domain: device translation runs before
//! gesture resolution. Internal to input — the cross-side frame phases
//! (`FramePhase`) live in `core::frame_phase`; sides own their own
//! sub-phases.

use bevy::prelude::*;

/// Sub-phases within `FramePhase::Input`.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum InputPhase {
    /// Devices translate raw input into gestures.
    Translate,
    /// Resolvers interpret gestures into core commands.
    Resolve,
}
