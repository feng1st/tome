use bevy::prelude::*;

#[derive(SystemSet, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum CorePhase {
    WorldAdvance,
    Derive,
    Command,
    Turn,
}
