//! Hero spawning.

pub mod hero;

use bevy::prelude::*;

use crate::core::app_state::AppState;

/// Register hero entity spawning: on entering `InGame` (fires on frame
/// one — startup needs no special path).
pub fn register(app: &mut App) {
    app.add_systems(OnEnter(AppState::InGame), hero::spawn_hero);
}
