//! Plan wander: the monster's RAND_25 action — at each due turn, 75%
//! standing still, 25% one random step.

use bevy::ecs::query::QueryData;
use bevy::prelude::*;
use rand::Rng;

use crate::core::creature::components::speed::Speed;
use crate::core::map::components::cell_coord::CellCoord;
use crate::core::map::resources::current_map::CurrentMap;
use crate::core::map::resources::terrain_registry::TerrainRegistry;
use crate::core::map::types::terrain::Terrain;
use crate::core::monster::components::monster_index::MonsterIndex;
use crate::core::movement::components::r#move::Move;
use crate::core::time::components::next_turn::NextTurn;
use crate::core::time::components::presenting::Presenting;
use crate::core::time::components::world_driver::WorldDriver;
use crate::core::time::resources::world_clock::WorldClock;
use crate::core::time::utils::action_cost::standard_action_cost;

/// A monster's wander-planning state: cell and speed for pricing, the
/// persistent turn slot, and its own presentation gate.
#[derive(QueryData)]
#[query_data(mutable)]
pub struct MonsterQuery {
    entity: Entity,
    cell: &'static CellCoord,
    speed: &'static Speed,
    turn: &'static mut NextTurn,
    presenting: Has<Presenting>,
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

/// Every monster with a due turn plans one action: 75% standing still
/// (spending the turn all the same — standing is an action), 25% a step
/// into a random walkable neighbor; a walled-in monster stands. The world
/// starts with the driver's first action; before that, nobody plans. A
/// monster whose own hop is still in flight waits — cell by cell, never
/// gliding.
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
    let Ok(hero_turn) = world_driver.single() else {
        return;
    };
    if hero_turn.at == 0 {
        return;
    }
    let mut rng = rand::rng();
    for mut monster in &mut monsters {
        // Own hop in flight, or the turn is not due yet.
        if monster.presenting || world_clock.now < monster.turn.at {
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
        let candidates: Vec<CellCoord> = DIRECTIONS
            .iter()
            .map(|direction| *monster.cell + *direction)
            .filter(|target| walkable(*target))
            .collect();
        let target = (!stands)
            .then(|| candidates.get(rng.random_range(0..candidates.len().max(1))))
            .flatten();
        match target {
            Some(&to) => {
                commands.entity(monster.entity).insert(Move { to });
                monster.turn.at = world_clock.now + standard_action_cost(monster.speed.0);
            }
            None => {
                monster.turn.at = world_clock.now + standard_action_cost(monster.speed.0);
            }
        }
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
        app.world_mut().entity_mut(rat).insert(Presenting);
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
