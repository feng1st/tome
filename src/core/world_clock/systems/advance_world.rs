use bevy::prelude::*;

use crate::core::display_protocol::components::is_performing::IsPerforming;
use crate::core::hit_points::components::hit_points::HitPoints;
use crate::core::player::components::player::Player;
use crate::core::world_clock::components::next_turn::NextTurn;
use crate::core::world_clock::resources::world_clock::WorldClock;

// Pixel Dungeon, Actor.process
pub fn advance_world(
    next_turns: Query<(&NextTurn, &HitPoints, Option<&Player>)>,
    is_performings: Query<(), With<IsPerforming>>,
    mut world_clock: ResMut<WorldClock>,
) {
    // has a world-stopping effect, e.g. turn-taking attack anim
    if let Some(solo) = world_clock.solo_performer {
        if !is_performings.contains(solo) {
            world_clock.solo_performer = None;
        }
    }
    if world_clock.solo_performer.is_some() {
        return;
    }
    // the world advances only when someone's next turn does (in the performers' loops)
    if let Some(min_next_turn_at) = next_turns
        .iter()
        .filter(|(_, hit_points, player)| !hit_points.is_dead() || player.is_some())
        .map(|(next_turn, _, _)| next_turn.at)
        .min()
    {
        world_clock.now = world_clock.now.max(min_next_turn_at);
    }
}
