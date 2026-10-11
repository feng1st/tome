use std::collections::HashSet;

use bevy::prelude::*;
use rand::Rng;

use crate::core::combat::components::armor_class::ArmorClass;
use crate::core::combat::components::blows::Blows;
use crate::core::combat::messages::attacked::Attacked;
use crate::core::combat::types::blow::Blow;
use crate::core::combat::utils::armor_class::armor_class;
use crate::core::combat::utils::attack::{attack_chance, test_hit_melee, unarmed_damage};
use crate::core::dice::types::dice::Dice;
use crate::core::display_protocol::components::is_performing::IsPerforming;
use crate::core::hit_points::components::hit_points::HitPoints;
use crate::core::hit_points::messages::damaged::Damaged;
use crate::core::map::components::cell_coord::CellCoord;
use crate::core::map::resources::current_map::CurrentMap;
use crate::core::map::resources::terrain_registry::TerrainRegistry;
use crate::core::map::types::terrain::Terrain;
use crate::core::map::utils::adjacency::adjacent;
use crate::core::map::utils::pathfinding::find_path;
use crate::core::player::components::order::Order;
use crate::core::player::components::player::Player;
use crate::core::rng::resources::game_rng::GameRng;
use crate::core::speed::components::speed::Speed;
use crate::core::speed::constants::action_duration::STANDARD_ACTION_DURATION;
use crate::core::speed::utils::action_duration::action_duration;
use crate::core::stats::components::curr_stats::CurrStats;
use crate::core::world_clock::components::next_turn::NextTurn;
use crate::core::world_clock::resources::world_clock::WorldClock;

const DIRECTIONS: [IVec2; 8] = [
    IVec2::new(1, 0),
    IVec2::new(-1, 0),
    IVec2::new(0, 1),
    IVec2::new(0, -1),
    IVec2::new(1, 1),
    IVec2::new(1, -1),
    IVec2::new(-1, 1),
    IVec2::new(-1, -1),
];

const RANDOM_ATTEMPTS: usize = 4;

// ToME2, melee1.c, make_attack_normal
#[allow(clippy::too_many_arguments)]
fn strike(
    attacker: Entity,
    attacker_cell_coord: CellCoord,
    blows: &[Blow],
    target: Entity,
    target_cell_coord: CellCoord,
    target_armor_class: i32,
    rng: &mut impl Rng,
    hit_points: &mut Query<&mut HitPoints>,
    attacked_writer: &mut MessageWriter<Attacked>,
    damaged_writer: &mut MessageWriter<Damaged>,
) {
    attacked_writer.write(Attacked {
        attacker,
        attacker_cell_coord,
        target,
        target_cell_coord,
    });
    for blow in blows {
        if test_hit_melee(blow.chance, target_armor_class, rng) {
            let amount = blow.damage.roll(rng);
            if let Ok(mut target_hit_points) = hit_points.get_mut(target) {
                target_hit_points.current -= amount;
                damaged_writer.write(Damaged {
                    source: Some(attacker),
                    source_cell_coord: Some(attacker_cell_coord),
                    target,
                    target_cell_coord,
                    amount,
                    max_hp: target_hit_points.max,
                });
                // ToME2, melee1.c, mon_take_hit
                if target_hit_points.is_dead() {
                    break;
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn step(
    actors: &mut Query<(
        Entity,
        &mut CellCoord,
        &Speed,
        &mut NextTurn,
        Option<&Player>,
        Option<&Order>,
        Option<&CurrStats>,
        Option<&Blows>,
        Option<&ArmorClass>,
    )>,
    occupied: &mut HashSet<CellCoord>,
    entity: Entity,
    from: CellCoord,
    to: CellCoord,
    next_turn_at: i64,
) {
    let Ok((_, mut cell_coord, _, mut next_turn, ..)) = actors.get_mut(entity) else {
        return;
    };
    next_turn.at = next_turn_at;
    *cell_coord = to;
    occupied.remove(&from);
    occupied.insert(to);
}

// Pixel Dungeon, Actor.process
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn act_turns(
    mut commands: Commands,
    mut world_clock: ResMut<WorldClock>,
    current_map: Res<CurrentMap>,
    terrain_registry: Res<TerrainRegistry>,
    mut game_rng: ResMut<GameRng>,
    mut actors: Query<(
        Entity,
        &mut CellCoord,
        &Speed,
        &mut NextTurn,
        Option<&Player>,
        Option<&Order>,
        Option<&CurrStats>,
        Option<&Blows>,
        Option<&ArmorClass>,
    )>,
    mut hit_points: Query<&mut HitPoints>,
    is_performings: Query<(), With<IsPerforming>>,
    mut attacked_writer: MessageWriter<Attacked>,
    mut damaged_writer: MessageWriter<Damaged>,
) {
    let now = world_clock.now;
    let rng = &mut game_rng.rng;
    let duration = |speed: i32| action_duration(STANDARD_ACTION_DURATION, speed);

    // earliest turn first; ties go to the player
    let mut due: Vec<(i64, bool, Entity)> = actors
        .iter()
        .filter(|(entity, _, _, next_turn, ..)| {
            now >= next_turn.at
                && hit_points
                    .get(*entity)
                    .is_ok_and(|hit_points| !hit_points.is_dead())
        })
        .map(|(entity, _, _, next_turn, player, ..)| (next_turn.at, player.is_none(), entity))
        .collect();
    due.sort_unstable();

    let mut occupied: HashSet<CellCoord> = actors
        .iter()
        .filter(|(entity, ..)| {
            hit_points
                .get(*entity)
                .is_ok_and(|hit_points| !hit_points.is_dead())
        })
        .map(|(_, &cell_coord, ..)| cell_coord)
        .collect();
    let player_entity = actors
        .iter()
        .find(|(_, _, _, _, player, ..)| player.is_some())
        .map(|(entity, ..)| entity);
    let is_walkable = |occupied: &HashSet<CellCoord>, target: CellCoord| {
        !occupied.contains(&target)
            && current_map
                .local_map()
                .get(target)
                .and_then(|terrain_index| terrain_registry.get(terrain_index))
                .is_some_and(Terrain::is_walkable)
    };

    for &(_, _, entity) in &due {
        if world_clock
            .solo_performer
            .is_some_and(|solo| solo != entity)
        {
            break;
        }
        let Ok((_, &cell_coord, speed, _, player, order, curr_stats, blows, _)) =
            actors.get(entity)
        else {
            continue;
        };
        let actor_cell_coord = cell_coord;
        let actor_speed = speed.0;
        let actor_is_player = player.is_some();
        let order = order.copied();
        // monsters: stamped at spawn; player: computed at attack time
        let blows: Vec<Blow> = match (blows, curr_stats) {
            (Some(blows), _) => blows.0.clone(),
            (None, Some(curr_stats)) => vec![Blow {
                chance: attack_chance(curr_stats),
                damage: Dice {
                    num: unarmed_damage(curr_stats),
                    sides: 1,
                },
            }],
            (None, None) => Vec::new(),
        };
        // died since the due list was built, e.g. a damage-over-time tick
        if hit_points
            .get(entity)
            .is_ok_and(|hit_points| hit_points.is_dead())
        {
            continue;
        }
        // Pixel Dungeon, CharSprite.isMoving
        if is_performings.contains(entity) {
            // a lagging picture holds the world only for the player
            if actor_is_player {
                world_clock.solo_performer = Some(entity);
            }
            break;
        }

        if actor_is_player {
            match order {
                Some(Order::Attack { target }) => {
                    let target_info = actors
                        .get(target)
                        .ok()
                        .filter(|_| {
                            hit_points
                                .get(target)
                                .is_ok_and(|hit_points| !hit_points.is_dead())
                        })
                        .map(|(_, &cell_coord, _, _, _, _, _, _, armor_class)| {
                            (
                                cell_coord,
                                armor_class.map_or(0, |armor_class| armor_class.0),
                            )
                        });
                    match target_info {
                        Some((target_cell_coord, target_armor_class))
                            if adjacent(actor_cell_coord, target_cell_coord) =>
                        {
                            if let Ok((_, _, _, mut next_turn, ..)) = actors.get_mut(entity) {
                                next_turn.at = now + duration(actor_speed);
                            }
                            commands.entity(entity).remove::<Order>();
                            strike(
                                entity,
                                actor_cell_coord,
                                &blows,
                                target,
                                target_cell_coord,
                                target_armor_class,
                                rng,
                                &mut hit_points,
                                &mut attacked_writer,
                                &mut damaged_writer,
                            );
                            // Pixel Dungeon, Mob.doAttack
                            world_clock.solo_performer = Some(entity);
                        }
                        Some((target_cell_coord, _)) => {
                            let obstacles: HashSet<CellCoord> = actors
                                .iter()
                                .map(|(_, &cell_coord, ..)| cell_coord)
                                .filter(|cell_coord| {
                                    *cell_coord != actor_cell_coord
                                        && *cell_coord != target_cell_coord
                                })
                                .collect();
                            let step_target = find_path(
                                current_map.local_map(),
                                &terrain_registry,
                                actor_cell_coord,
                                target_cell_coord,
                                &obstacles,
                            )
                            .and_then(|route| route.front().copied());
                            match step_target {
                                Some(step_target) => {
                                    step(
                                        &mut actors,
                                        &mut occupied,
                                        entity,
                                        actor_cell_coord,
                                        step_target,
                                        now + duration(actor_speed),
                                    );
                                    world_clock.solo_performer = None;
                                }
                                None => {
                                    commands.entity(entity).remove::<Order>();
                                    // Pixel Dungeon, Hero.ready
                                    world_clock.solo_performer = Some(entity);
                                }
                            }
                        }
                        None => {
                            commands.entity(entity).remove::<Order>();
                            // Pixel Dungeon, Hero.ready
                            world_clock.solo_performer = Some(entity);
                        }
                    }
                }
                Some(Order::Move { target }) => {
                    let obstacles: HashSet<CellCoord> = actors
                        .iter()
                        .map(|(_, &cell_coord, ..)| cell_coord)
                        .filter(|cell_coord| *cell_coord != actor_cell_coord)
                        .collect();
                    let step_target = if actor_cell_coord == target {
                        None
                    } else {
                        find_path(
                            current_map.local_map(),
                            &terrain_registry,
                            actor_cell_coord,
                            target,
                            &obstacles,
                        )
                        .and_then(|route| route.front().copied())
                    };
                    match step_target {
                        Some(step_target) if !obstacles.contains(&step_target) => {
                            step(
                                &mut actors,
                                &mut occupied,
                                entity,
                                actor_cell_coord,
                                step_target,
                                now + duration(actor_speed),
                            );
                            world_clock.solo_performer = None;
                        }
                        _ => {
                            commands.entity(entity).remove::<Order>();
                            // Pixel Dungeon, Hero.ready
                            world_clock.solo_performer = Some(entity);
                        }
                    }
                }
                None => {
                    // Pixel Dungeon, Hero.ready
                    world_clock.solo_performer = Some(entity);
                }
            }
        } else {
            let player_info = player_entity.and_then(|player_entity| {
                let (_, &cell_coord, _, _, _, _, curr_stats, ..) =
                    actors.get(player_entity).ok()?;
                if hit_points
                    .get(player_entity)
                    .is_ok_and(|hit_points| hit_points.is_dead())
                {
                    return None;
                }
                Some((player_entity, cell_coord, curr_stats.map_or(0, armor_class)))
            });
            if let Some((player_entity, player_cell_coord, player_armor_class)) = player_info {
                if adjacent(actor_cell_coord, player_cell_coord) {
                    if let Ok((_, _, _, mut next_turn, ..)) = actors.get_mut(entity) {
                        next_turn.at = now + duration(actor_speed);
                    }
                    strike(
                        entity,
                        actor_cell_coord,
                        &blows,
                        player_entity,
                        player_cell_coord,
                        player_armor_class,
                        rng,
                        &mut hit_points,
                        &mut attacked_writer,
                        &mut damaged_writer,
                    );
                    // Pixel Dungeon, Mob.doAttack
                    world_clock.solo_performer = Some(entity);
                    break;
                }
            }
            let stands = rng.random_range(0..100) >= 25;
            let mut acted = false;
            if !stands {
                for _ in 0..RANDOM_ATTEMPTS {
                    let direction = DIRECTIONS[rng.random_range(0..DIRECTIONS.len())];
                    let to = actor_cell_coord + direction;
                    if is_walkable(&occupied, to) {
                        step(
                            &mut actors,
                            &mut occupied,
                            entity,
                            actor_cell_coord,
                            to,
                            now + duration(actor_speed),
                        );
                        acted = true;
                        break;
                    }
                }
            }
            if !acted {
                if let Ok((_, _, _, mut next_turn, ..)) = actors.get_mut(entity) {
                    next_turn.at = now + duration(actor_speed);
                }
            }
        }
    }
}
