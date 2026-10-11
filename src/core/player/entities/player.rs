use bevy::prelude::*;

use crate::core::class::resources::class_registry::ClassRegistry;
use crate::core::hit_points::components::hit_points::HitPoints;
use crate::core::hit_points::utils::max_hit_points::max_hit_points;
use crate::core::map::components::cell_coord::CellCoord;
use crate::core::player::components::player::Player;
use crate::core::player::utils::birth::calc_birth_stats;
use crate::core::race::resources::race_registry::RaceRegistry;
use crate::core::rng::resources::game_rng::GameRng;
use crate::core::speed::components::speed::Speed;
use crate::core::speed::constants::speed::STANDARD_SPEED;
use crate::core::stats::components::curr_stats::CurrStats;
use crate::core::stats::components::max_stats::MaxStats;
use crate::core::world_clock::components::next_turn::NextTurn;

const PLAYER_START: CellCoord = CellCoord::new(24, 10);

const PLAYER_RACE: &str = "human";

const PLAYER_CLASS: &str = "warrior";

pub fn spawn_player(
    mut commands: Commands,
    race_registry: Res<RaceRegistry>,
    class_registry: Res<ClassRegistry>,
    mut game_rng: ResMut<GameRng>,
) {
    let race_index = race_registry
        .get_index(PLAYER_RACE)
        .expect("human is a declared race");
    let class_index = class_registry
        .get_index(PLAYER_CLASS)
        .expect("warrior is a declared class");
    let race_kind = race_registry.race_kind(race_index);
    let class_kind = class_registry.class_kind(class_index);
    let birth_stats = calc_birth_stats(race_kind, class_kind, false, &mut game_rng.rng);
    let curr_stats = CurrStats(birth_stats.curr_stats);
    let hit_dice_modifier = race_kind.hit_dice_modifier + class_kind.hit_dice_modifier;
    let max_hp = max_hit_points(hit_dice_modifier, &curr_stats);
    commands.spawn((
        Player,
        race_index,
        class_index,
        PLAYER_START,
        Speed(STANDARD_SPEED),
        NextTurn::default(),
        curr_stats,
        MaxStats(birth_stats.max_stats),
        HitPoints {
            current: max_hp,
            max: max_hp,
        },
    ));
}
