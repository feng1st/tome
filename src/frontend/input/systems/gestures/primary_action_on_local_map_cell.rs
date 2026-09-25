//! Resolves `PrimaryActionOnLocalMapCell` gestures.

use bevy::prelude::*;

use crate::core::hero::commands::move_to_cell::MoveToCell;
use crate::frontend::input::gestures::primary_action_on_local_map_cell::PrimaryActionOnLocalMapCell;

/// Today every cell gesture means walking there; target-aware policies
/// grow as sibling resolvers for new gesture types. Validation
/// (walkability, reachability) stays in the core — resolve only decides
/// meaning.
pub fn resolve(
    mut gestures: MessageReader<PrimaryActionOnLocalMapCell>,
    mut commands: MessageWriter<MoveToCell>,
) {
    for gesture in gestures.read() {
        commands.write(MoveToCell(gesture.0));
    }
}
