use bevy::prelude::*;

#[derive(SystemSet, Clone, PartialEq, Eq, Hash, Debug)]
pub enum DisplayPhase {
    Attach,
    Sync,
    Motion,
    Animate,
    Camera,
    Snap,
}
