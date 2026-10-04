//! Plan wander: the monster's idle wander — at each due turn, 75%
//! standing still, 25% a random direction.

use bevy::ecs::query::QueryData;
use bevy::prelude::*;
use rand::Rng;

use crate::core::display::components::is_moving::IsMoving;
use crate::core::map::components::cell_coord::CellCoord;
use crate::core::map::resources::current_map::CurrentMap;
use crate::core::map::resources::terrain_registry::TerrainRegistry;
use crate::core::map::types::terrain::Terrain;
use crate::core::monster::components::monster_index::MonsterIndex;
use crate::core::movement::components::r#move::Move;
use crate::core::speed::components::speed::Speed;
use crate::core::speed::constants::action_duration::STANDARD_ACTION_DURATION;
use crate::core::speed::utils::action_duration::action_duration;
use crate::core::world_clock::components::next_turn::NextTurn;
use crate::core::world_clock::components::world_driver::WorldDriver;
use crate::core::world_clock::resources::world_clock::WorldClock;

/// A monster's wander-planning state: cell and speed for pricing, the
/// persistent next turn, and its own moving flag.
#[derive(QueryData)]
#[query_data(mutable)]
pub struct MonsterQuery {
    entity: Entity,
    cell: &'static CellCoord,
    speed: &'static Speed,
    next_turn: &'static mut NextTurn,
    is_moving: Has<IsMoving>,
}

/// The 8 step directions a wander can pick.
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

/// How many independent random directions one wander attempt tries
/// before giving up: the first walkable pick wins, and a monster that
/// fails all of them stands.
const RANDOM_ATTEMPTS: usize = 4;

/// Every monster with a due turn plans one action: 75% standing still,
/// 25% a random direction — up to four independent picks, the first
/// walkable one wins, and a fully walled-in monster stands. Standing is
/// an action all the same: the turn is spent either way. The world
/// starts with the driver's first action; before that, nobody plans. A
/// monster whose own hop is still in flight waits — cell by cell,
/// never gliding.
pub fn plan_wander(
    mut commands: Commands,
    world_clock: Res<WorldClock>,
    current_map: Res<CurrentMap>,
    terrain_registry: Res<TerrainRegistry>,
    world_driver: Query<&NextTurn, With<WorldDriver>>,
    mut monsters: Query<MonsterQuery, (With<MonsterIndex>, Without<WorldDriver>)>,
) {
    // The world starts with the driver's first action: its next turn
    // leaves zero the moment the first plan lands.
    let Ok(world_driver_next_turn) = world_driver.single() else {
        return;
    };
    if world_driver_next_turn.at == 0 {
        return;
    }
    let mut rng = rand::rng();
    for mut monster in &mut monsters {
        // Own hop in flight, or the turn is not due yet.
        if monster.is_moving || world_clock.now < monster.next_turn.at {
            continue;
        }
        let stands = rng.random_range(0..100) >= 25;
        let walkable = |target: CellCoord| {
            current_map
                .map()
                .get(target)
                .and_then(|terrain_index| terrain_registry.get(terrain_index))
                .is_some_and(Terrain::walkable)
        };
        let mut target = None;
        if !stands {
            for _ in 0..RANDOM_ATTEMPTS {
                let direction = DIRECTIONS[rng.random_range(0..DIRECTIONS.len())];
                let to = *monster.cell + direction;
                if walkable(to) {
                    target = Some(to);
                    break;
                }
            }
        }
        if let Some(to) = target {
            commands.entity(monster.entity).insert(Move { to });
        }
        monster.next_turn.at =
            world_clock.now + action_duration(STANDARD_ACTION_DURATION, monster.speed.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::map::resources::current_map::parse_local_map;
    use crate::core::map::resources::terrain_registry::parse_terrain_registry;
    use crate::core::map::types::local_map::LocalMap;

    const TERRAINS: &str = r#"[
        ( terrain: "floor", flags: ["PASSABLE"] ),
        ( terrain: "wall",  flags: [] ),
    ]"#;

    /// A walled room; `floor` is the list of floor cells inside it.
    fn room(width: usize, height: usize, floor: &[CellCoord]) -> LocalMap {
        let rows: Vec<String> = (0..height)
            .map(|y| {
                (0..width)
                    .map(|x| {
                        if floor.contains(&CellCoord::new(x as i32, y as i32)) {
                            '.'
                        } else {
                            '#'
                        }
                    })
                    .collect()
            })
            .collect();
        let rows: Vec<String> = rows.iter().map(|r| format!("\"{r}\"")).collect();
        let doc = format!(
            "( legend: {{ '#': \"wall\", '.': \"floor\" }}, rows: [ {} ], )",
            rows.join(", ")
        );
        parse_local_map("test", &doc, &parse_terrain_registry("test", TERRAINS))
    }

    fn open_room() -> LocalMap {
        let floor: Vec<CellCoord> = (1..5)
            .flat_map(|y| (1..5).map(move |x| CellCoord::new(x, y)))
            .collect();
        room(6, 6, &floor)
    }

    fn app_with(map: LocalMap) -> App {
        let mut app = App::new();
        app.init_resource::<WorldClock>()
            .insert_resource(parse_terrain_registry("test", TERRAINS))
            .insert_resource(CurrentMap::new(map))
            .add_systems(Update, plan_wander);
        app
    }

    /// A started world: the driver has planned once (turn left zero).
    fn spawn_started_driver(app: &mut App) {
        app.world_mut()
            .spawn((WorldDriver, CellCoord::new(0, 0), NextTurn { at: 10 }));
    }

    fn spawn_rat(app: &mut App, cell: CellCoord) -> Entity {
        app.world_mut()
            .spawn((
                MonsterIndex::from_index(0),
                Speed(110),
                cell,
                NextTurn::default(),
            ))
            .id()
    }

    #[test]
    fn world_not_started_no_plans() {
        let mut app = app_with(open_room());
        app.world_mut()
            .spawn((WorldDriver, CellCoord::new(0, 0), NextTurn::default()));
        let rat = spawn_rat(&mut app, CellCoord::new(2, 2));
        app.update();
        assert_eq!(app.world().get::<NextTurn>(rat).unwrap().at, 0);
        assert!(app.world().get::<Move>(rat).is_none());
    }

    #[test]
    fn due_monster_spends_its_turn() {
        let mut app = app_with(open_room());
        spawn_started_driver(&mut app);
        let rat = spawn_rat(&mut app, CellCoord::new(2, 2));
        app.update();
        assert_eq!(app.world().get::<NextTurn>(rat).unwrap().at, 100);
        if let Some(m) = app.world().get::<Move>(rat) {
            let dx = (m.to.x - 2).abs();
            let dy = (m.to.y - 2).abs();
            assert!(dx <= 1 && dy <= 1 && (dx + dy) > 0);
        }
    }

    #[test]
    fn slot_not_due_plans_nothing() {
        let mut app = app_with(open_room());
        spawn_started_driver(&mut app);
        let rat = spawn_rat(&mut app, CellCoord::new(2, 2));
        app.world_mut()
            .entity_mut(rat)
            .insert(NextTurn { at: 1000 });
        app.update();
        assert_eq!(app.world().get::<NextTurn>(rat).unwrap().at, 1000);
        assert!(app.world().get::<Move>(rat).is_none());
    }

    #[test]
    fn own_hop_in_flight_plans_nothing() {
        let mut app = app_with(open_room());
        spawn_started_driver(&mut app);
        let rat = spawn_rat(&mut app, CellCoord::new(2, 2));
        app.world_mut().entity_mut(rat).insert(IsMoving);
        app.update();
        assert_eq!(app.world().get::<NextTurn>(rat).unwrap().at, 0);
        assert!(app.world().get::<Move>(rat).is_none());
    }

    #[test]
    fn walled_in_spends_the_turn_without_a_step() {
        let mut app = app_with(room(3, 3, &[CellCoord::new(1, 1)]));
        spawn_started_driver(&mut app);
        let rat = spawn_rat(&mut app, CellCoord::new(1, 1));
        app.update();
        assert_eq!(app.world().get::<NextTurn>(rat).unwrap().at, 100);
        assert!(
            app.world().get::<Move>(rat).is_none(),
            "no walkable neighbor: stand, not a wasted step"
        );
    }
}
