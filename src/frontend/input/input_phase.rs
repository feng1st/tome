// TODO: pending cleanup review — remove once stabilized
//! Ordering phases inside the input domain: device translation runs before
//! gesture resolution. Internal to input — the cross-side game-loop stages
//! (`GameLoop`) live in `core::game_loop`; sides own their own
//! sub-phases.
//!
//! Both phases run in `Update`: engine input state (`ButtonInput`) is
//! refreshed in `PreUpdate`, so `Update` reads it fresh without any
//! explicit ordering against engine internals.

use bevy::prelude::*;

/// Sub-phases within `GameLoop::Input`.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum InputPhase {
    /// Devices translate raw input into gestures.
    Translate,
    /// Resolvers interpret gestures into core commands.
    Resolve,
}
