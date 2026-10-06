//! A* over the walkable grid. Pure cell semantics — no pixel concepts.

use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use bevy::prelude::*;

use crate::core::map::components::cell_coord::CellCoord;
use crate::core::map::resources::terrain_registry::TerrainRegistry;
use crate::core::map::types::local_map::LocalMap;
use crate::core::map::types::terrain::Terrain;

/// A* over the walkable grid, 8 directions, cost 10 straight / 14 diagonal,
/// octile-distance heuristic. Cells in `obstacles` are impassable, except
/// the destination: a route may end on an occupied goal, and the caller
/// decides what the final step means. Diagonal steps only require the
/// target cell to be walkable (corner cutting allowed).
///
/// Costs are integers scaled by 10 so the heap can stay `i32`.
pub fn find_path(
    local_map: &LocalMap,
    terrain_registry: &TerrainRegistry,
    start: CellCoord,
    goal: CellCoord,
    obstacles: &HashSet<CellCoord>,
) -> Option<VecDeque<CellCoord>> {
    let walkable = |cell: CellCoord| {
        local_map
            .get(cell)
            .and_then(|terrain_index| terrain_registry.get(terrain_index))
            .is_some_and(Terrain::walkable)
    };
    if !walkable(start) || !walkable(goal) {
        return None;
    }
    // The destination is exempt from the obstacle overlay: a route may
    // end on an occupied goal, and the order's planning decides on the
    // final step.
    let blocked = |cell: CellCoord| cell != goal && obstacles.contains(&cell);
    if start == goal {
        return Some(VecDeque::new());
    }

    fn heuristic(a: CellCoord, b: CellCoord) -> i32 {
        let dx = (a.x - b.x).abs();
        let dy = (a.y - b.y).abs();
        let (min, max) = (dx.min(dy), dx.max(dy));
        14 * min + 10 * (max - min)
    }

    #[derive(PartialEq, Eq)]
    struct Node {
        f: i32,
        cell: CellCoord,
    }
    // BinaryHeap is a max-heap; invert the comparison for lowest-f-first.
    impl Ord for Node {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.f.cmp(&self.f) // min-heap
        }
    }
    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    const DIRS: [(IVec2, i32); 8] = [
        (IVec2::new(1, 0), 10),
        (IVec2::new(-1, 0), 10),
        (IVec2::new(0, 1), 10),
        (IVec2::new(0, -1), 10),
        (IVec2::new(1, 1), 14),
        (IVec2::new(1, -1), 14),
        (IVec2::new(-1, 1), 14),
        (IVec2::new(-1, -1), 14),
    ];

    let mut open = BinaryHeap::from([Node {
        f: heuristic(start, goal),
        cell: start,
    }]);
    let mut g_score: HashMap<CellCoord, i32> = HashMap::from([(start, 0)]);
    let mut came_from: HashMap<CellCoord, CellCoord> = HashMap::new();
    let mut closed = HashSet::new();

    while let Some(Node { cell, .. }) = open.pop() {
        if cell == goal {
            let mut path = VecDeque::from([goal]);
            let mut cur = goal;
            while let Some(&prev) = came_from.get(&cur) {
                path.push_front(prev);
                cur = prev;
            }
            path.pop_front(); // drop the start cell
            return Some(path);
        }
        if !closed.insert(cell) {
            continue;
        }
        let g = g_score[&cell];
        for (dir, cost) in DIRS {
            let next = cell + dir;
            if !walkable(next) || blocked(next) {
                continue;
            }
            let next_g = g + cost;
            if next_g < *g_score.get(&next).unwrap_or(&i32::MAX) {
                g_score.insert(next, next_g);
                came_from.insert(next, cell);
                open.push(Node {
                    f: next_g + heuristic(next, goal),
                    cell: next,
                });
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::map::resources::current_map::parse_local_map;
    use crate::core::map::resources::terrain_registry::parse_terrain_registry;

    const TERRAINS: &str = r#"[
        ( terrain: "floor", flags: ["PASSABLE"] ),
        ( terrain: "wall",  flags: [] ),
        ( terrain: "water", flags: ["LIQUID"] ),
    ]"#;

    fn terrain_registry() -> TerrainRegistry {
        parse_terrain_registry("test", TERRAINS)
    }

    fn walkable(local_map: &LocalMap, terrain_registry: &TerrainRegistry, cell: CellCoord) -> bool {
        local_map
            .get(cell)
            .and_then(|terrain_index| terrain_registry.get(terrain_index))
            .is_some_and(Terrain::walkable)
    }

    fn map_from(rows: &[&str]) -> LocalMap {
        let legend = r#"{ '#': "wall", '.': "floor", '~': "water" }"#;
        let rows = rows
            .iter()
            .map(|r| format!("\"{r}\""))
            .collect::<Vec<_>>()
            .join(", ");
        let doc = format!("( legend: {legend}, rows: [ {rows} ], )");
        parse_local_map("test", &doc, &terrain_registry())
    }

    /// A walled room with a pool blocking the straight line between its
    /// left and right halves (4x5 cells).
    fn room_with_pool() -> LocalMap {
        let mut rows = vec!["#..............#".to_string(); 8];
        rows.insert(0, "################".to_string());
        rows.push("################".to_string());
        for (y, row) in rows.iter_mut().enumerate() {
            if (3..8).contains(&y) {
                row.replace_range(5..9, "~~~~");
            }
        }
        let rows: Vec<&str> = rows.iter().map(String::as_str).collect();
        map_from(&rows)
    }

    #[test]
    fn path_around_pool() {
        let terrain_registry = terrain_registry();
        let local_map = room_with_pool();
        // Straight line between these cells crosses the pool.
        let start = CellCoord::new(2, 5);
        let goal = CellCoord::new(13, 5);
        let path = find_path(&local_map, &terrain_registry, start, goal, &HashSet::new())
            .expect("path exists");
        assert_eq!(*path.back().unwrap(), goal);
        let water = terrain_registry.get_index("water").unwrap();
        for cell in &path {
            assert!(
                walkable(&local_map, &terrain_registry, *cell),
                "path steps on walkable cells only"
            );
            assert!(local_map.get(*cell) != Some(water));
        }
        // A shortest route detours with 6 diagonal steps (3 up, 3 down),
        // keeping the step count at the horizontal distance of 11.
        assert_eq!(path.len(), 11);
    }

    #[test]
    fn diagonal_step_only_needs_its_target_walkable() {
        // Walls at (2,1) and (1,2) — the two orthogonal cells flanking
        // the step — leave the diagonal from (1,1) to (2,2) legal.
        let terrain_registry = terrain_registry();
        let local_map = map_from(&["....", "..#.", ".#..", "...."]);
        let path = find_path(
            &local_map,
            &terrain_registry,
            CellCoord::new(1, 1),
            CellCoord::new(2, 2),
            &HashSet::new(),
        )
        .expect("the diagonal only needs its target walkable");
        assert_eq!(path.len(), 1);
        assert_eq!(path[0], CellCoord::new(2, 2));
    }

    #[test]
    fn path_unreachable() {
        let terrain_registry = terrain_registry();
        let local_map = room_with_pool();
        assert!(find_path(
            &local_map,
            &terrain_registry,
            CellCoord::new(5, 2),
            CellCoord::new(0, 0),
            &HashSet::new(),
        )
        .is_none()); // wall
        assert!(find_path(
            &local_map,
            &terrain_registry,
            CellCoord::new(5, 2),
            CellCoord::new(6, 4),
            &HashSet::new(),
        )
        .is_none()); // water
        assert!(find_path(
            &local_map,
            &terrain_registry,
            CellCoord::new(5, 2),
            CellCoord::new(-3, 2),
            &HashSet::new(),
        )
        .is_none());
        // oob
    }

    #[test]
    fn path_start_equals_goal() {
        let terrain_registry = terrain_registry();
        let local_map = room_with_pool();
        let path = find_path(
            &local_map,
            &terrain_registry,
            CellCoord::new(5, 2),
            CellCoord::new(5, 2),
            &HashSet::new(),
        )
        .unwrap();
        assert!(path.is_empty());
    }

    #[test]
    fn path_routes_around_an_obstacle() {
        let terrain_registry = terrain_registry();
        let local_map = map_from(&[".....", ".....", "....."]);
        let mut obstacles = HashSet::new();
        obstacles.insert(CellCoord::new(2, 1)); // the straight line's middle
        let path = find_path(
            &local_map,
            &terrain_registry,
            CellCoord::new(0, 1),
            CellCoord::new(4, 1),
            &obstacles,
        )
        .expect("a detour around the obstacle exists");
        assert_eq!(*path.back().unwrap(), CellCoord::new(4, 1));
        assert!(
            !path.contains(&CellCoord::new(2, 1)),
            "the obstacle cell is avoided"
        );
        for cell in &path {
            assert!(walkable(&local_map, &terrain_registry, *cell));
        }
    }

    #[test]
    fn an_occupied_goal_still_resolves_a_route() {
        // The destination is exempt from the overlay: the route may end
        // on the occupied goal — the order's planning decides on the
        // final step.
        let terrain_registry = terrain_registry();
        let local_map = map_from(&[".....", ".....", "....."]);
        let goal = CellCoord::new(4, 1);
        let mut obstacles = HashSet::new();
        obstacles.insert(goal);
        let path = find_path(
            &local_map,
            &terrain_registry,
            CellCoord::new(0, 1),
            goal,
            &obstacles,
        )
        .expect("the goal's own obstacle does not block the route");
        assert_eq!(*path.back().unwrap(), goal);
        assert!(path.iter().take(path.len() - 1).all(|cell| *cell != goal));
    }
}
