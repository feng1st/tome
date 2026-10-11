use bevy::prelude::*;

use crate::core::combat::components::armor_class::ArmorClass;
use crate::core::combat::components::blows::Blows;
use crate::core::combat::types::blow::Blow;
use crate::core::hit_points::components::hit_points::HitPoints;
use crate::core::map::resources::current_map::CurrentMap;
use crate::core::monster::resources::monster_registry::MonsterRegistry;
use crate::core::monster::types::monster_kind::MonsterKind;
use crate::core::rng::resources::game_rng::GameRng;
use crate::core::world_clock::components::next_turn::NextTurn;

const MONSTER_BLOW_POWER_STANDIN: i32 = 60;

const CHANCE_PER_LEVEL: i32 = 3;

/// ToME2, melee1.c, make_attack_normal, check_hit
fn translated_blows(monster_kind: &MonsterKind) -> Blows {
    let blows = monster_kind
        .blows
        .iter()
        .map(|blow| Blow {
            chance: MONSTER_BLOW_POWER_STANDIN + monster_kind.level * CHANCE_PER_LEVEL,
            damage: blow.damage,
        })
        .collect();
    Blows(blows)
}

pub fn spawn_monsters(
    mut commands: Commands,
    current_map: Res<CurrentMap>,
    monster_registry: Res<MonsterRegistry>,
    mut game_rng: ResMut<GameRng>,
) {
    for spawn in &current_map.local_map().spawns {
        let monster_index = monster_registry
            .get_index(&spawn.monster)
            .unwrap_or_else(|| {
                panic!(
                    "map declares unknown monster '{}' at ({}, {})",
                    spawn.monster, spawn.cell_coord.x, spawn.cell_coord.y
                )
            });
        let monster_kind = monster_registry.monster_kind(monster_index);
        // ToME2, monster2.c, place_monster_one, m_ptr->maxhp
        let hit_points = monster_kind.hit_points.roll(&mut game_rng.rng);
        let blows = translated_blows(monster_kind);
        commands.spawn((
            monster_index,
            monster_kind.speed,
            spawn.cell_coord,
            NextTurn::default(),
            HitPoints {
                current: hit_points,
                max: hit_points,
            },
            blows,
            ArmorClass(monster_kind.armor_class),
        ));
    }
}
