//! Core-side phase labels: ordering within `GameLoop::Core`. The classic
//! Sense–Plan–Act agent loop. The hero's action arrives via commands (see
//! `hero::systems::commands`); monsters will plan from world queries.

use bevy::prelude::*;

/// Sub-phases within `GameLoop::Core`.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CorePhase {
    /// Perception is computed and stored (FOV, alertness, memory). Empty
    /// until the first AI needs materialized perception.
    Sense,
    /// Set or modify targets: hero commands are validated and materialized
    /// into paths, monster AI chooses intents. The world state is a
    /// read-only snapshot throughout this phase.
    Plan,
    /// Chosen actions execute: movement, world state. Members are
    /// deliberately unordered — a command's movement starting one frame
    /// late is invisible; split a new phase if that ever matters.
    Act,
}
