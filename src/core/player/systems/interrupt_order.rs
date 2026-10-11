use bevy::prelude::*;

use crate::core::combat::messages::attacked::Attacked;
use crate::core::player::components::order::Order;
use crate::core::player::components::player::Player;

pub fn interrupt_order(
    mut commands: Commands,
    mut attacked_reader: MessageReader<Attacked>,
    players: Query<(Entity, &Order), With<Player>>,
) {
    for attacked in attacked_reader.read() {
        let Ok((player, order)) = players.get(attacked.target) else {
            continue;
        };
        if matches!(order, Order::Move { .. }) {
            commands.entity(player).remove::<Order>();
        }
    }
}
