use bevy::prelude::*;

use crate::core::combat::messages::attacked::Attacked;
use crate::core::display_protocol::components::is_disappearing::IsDisappearing;
use crate::core::hit_points::components::hit_points::HitPoints;
use crate::core::map::components::cell_coord::CellCoord;
use crate::core::player::components::order::Order;
use crate::core::player::components::player::Player;
use crate::frontend::display::motion::components::curr_position::CellPosition;
use crate::frontend::display::motion::utils::position::{flip_x, is_moving};
use crate::frontend::display::sprite_animation::components::anim_kind::AnimKind;
use crate::frontend::display::sprite_animation::components::anim_state::AnimState;

#[allow(clippy::type_complexity)]
pub fn sync_animation_and_direction(
    time: Res<Time>,
    mut attacked_reader: MessageReader<Attacked>,
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &CellCoord,
        &CellPosition,
        Option<&Order>,
        Option<&HitPoints>,
        Option<&Player>,
        &AnimKind,
        &mut AnimState,
        &mut Sprite,
    )>,
) {
    // FIXME
    let attack_targets: Vec<(Entity, CellCoord)> = attacked_reader
        .read()
        .map(|attacked| (attacked.attacker, attacked.target_cell_coord))
        .collect();
    for (
        entity,
        &cell_coord,
        &cell_position,
        order,
        hit_points,
        player,
        &anim_kind,
        mut anim_state,
        mut sprite,
    ) in &mut query
    {
        // A fresh blow re-arms a settled swing, so the second exchange
        // plays its own swing instead of sitting on the last frame.
        if attack_targets
            .iter()
            .any(|(attacker, _)| *attacker == entity)
        {
            anim_state.restart_attack(time.elapsed_secs());
        }
        if hit_points.is_some_and(HitPoints::is_dead) {
            if anim_kind != AnimKind::Die {
                let mut entity = commands.entity(entity);
                entity.insert(AnimKind::Die);
                if player.is_none() {
                    entity.insert(IsDisappearing);
                }
            }
            continue;
        }
        if !anim_state.looped && !anim_state.finished {
            continue;
        }
        // FIXME
        let attack_target = attack_targets
            .iter()
            .find(|(attacker, _)| *attacker == entity)
            .map(|(_, target_cell)| *target_cell);
        let desired = if attack_target.is_some() {
            AnimKind::Attack
        } else if order.is_some() || is_moving(cell_position, cell_coord) {
            AnimKind::Run
        } else {
            AnimKind::Idle
        };
        if anim_kind != desired {
            commands.entity(entity).insert(desired);
        }
        let facing_target = attack_target.unwrap_or(cell_coord);
        if let Some(flip) = flip_x(cell_position, facing_target) {
            sprite.flip_x = flip;
        }
    }
}
