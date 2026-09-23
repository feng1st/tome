//! Hero command messages, one file per command. Each command's executor
//! lives at the mirrored path under `systems/commands/`.

pub mod move_to_cell;

use bevy::prelude::*;

use move_to_cell::MoveToCell;

/// Register the hero domain's command message types.
pub fn register(app: &mut App) {
    app.add_message::<MoveToCell>();
}
