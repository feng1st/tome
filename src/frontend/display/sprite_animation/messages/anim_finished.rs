use bevy::prelude::*;

use crate::frontend::display::sprite_animation::components::anim_kind::AnimKind;

#[derive(Message, Clone, Copy, PartialEq, Eq, Debug)]
pub struct AnimFinished {
    pub entity: Entity,
    pub anim_kind: AnimKind,
}
