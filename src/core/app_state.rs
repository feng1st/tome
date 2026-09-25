//! Application mode protocol: the cross-side state tree. Modes live in
//! states; data about modes (which map, load readiness) lives in resources
//! and messages — never in state variants.
//!
//! `AppState` is the top-level mode. `InGameState` is a substate that
//! exists only while `AppState::InGame` is active (the engine removes it
//! on parent exit). Both fire `OnEnter` on frame one when they are the
//! initial states, so startup needs no special path.
//!
//! Mode-specific systems gate themselves with `run_if(in_state(...))` at
//! their domain register — phases carry ordering, states carry gating, and
//! the two never merge into combined labels.

use bevy::prelude::*;

/// Top-level application modes.
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[allow(dead_code)] // Protocol defined in full; only InGame is reachable this iteration.
pub enum AppState {
    MainMenu,
    #[default]
    InGame,
    GameOver,
}

/// Play modes within `InGame`: world-map travel vs. inside a local map
/// (town / wilderness / dungeon).
#[derive(SubStates, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[source(AppState = AppState::InGame)]
#[allow(dead_code)] // Protocol defined in full; only LocalMap is reachable this iteration.
pub enum InGameState {
    WorldMap,
    #[default]
    LocalMap,
}
