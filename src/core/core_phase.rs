// TODO: pending cleanup review — remove once stabilized
//! Core-side phase labels: ordering within `GameLoop::Core`. The hero's
//! action arrives via commands (see `hero::systems::commands`); `Decide`
//! is reserved for agent AI choosing intents, `Act` executes them.

use bevy::prelude::*;

/// Sub-phases within `GameLoop::Core`.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CorePhase {
    /// Agents choose their intents (monster AI; future).
    Decide,
    /// Chosen actions execute: commands, movement, world state. Members are
    /// deliberately unordered — a command's movement starting one frame
    /// late is invisible; split a new phase if that ever matters.
    Act,
}
