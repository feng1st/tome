//! Application mode protocol: the cross-side state tree. Modes live in
//! states; data about modes (which map, load readiness) lives in resources
//! and messages — never in state variants.
//!
//! `AppState` is the top-level mode and fires `OnEnter` on frame one when
//! it is the initial state, so startup needs no special path. World-map
//! vs. local-map distinction is game-internal state (read by resolvers
//! and executors), not a Bevy substate.
//!
//! Mode-specific systems gate themselves with `run_if(in_state(...))` at
//! their domain register — phases carry ordering, states carry gating, and
//! the two never merge into combined labels.

use bevy::prelude::*;

/// Top-level application modes.
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[allow(dead_code)] // Protocol defined in full; only Game is reachable this iteration.
pub enum AppState {
    MainMenu,
    #[default]
    Game,
}
