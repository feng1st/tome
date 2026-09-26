//! A* over the walkable grid. Pure cell semantics — no pixel concepts.

use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use bevy::prelude::*;

use crate::core::map::types::cell_coord::CellCoord;
use crate::core::map::types::grid_map::GridMap;

/// A* over the walkable grid, 8 directions, cost 10 straight / 14 diagonal,
/// octile-distance heuristic. Diagonal steps only require the target cell to
/// be walkable (corner cutting allowed).
///
/// Costs are integers scaled by 10 so the heap can stay `i32`.
pub fn find_path(map: &GridMap, start: CellCoord, goal: CellCoord) -> Option<VecDeque<CellCoord>> {
    if !map.walkable(start) || !map.walkable(goal) {
        return None;
    }
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
            if !map.walkable(next) {
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
    use crate::core::map::constants::tile_kind::TileKind;

    #[test]
    fn path_around_pool() {
        let map = GridMap::demo_room();
        // Straight line between these cells crosses the pool (8x5 cells,
        // slightly below the room center).
        let start = CellCoord::new(16, 19);
        let goal = CellCoord::new(32, 19);
        let path = find_path(&map, start, goal).expect("path exists");
        assert_eq!(*path.back().unwrap(), goal);
        for cell in &path {
            assert!(map.walkable(*cell), "path steps on walkable cells only");
            assert!(map.get(*cell) != Some(TileKind::Water));
        }
        // A shortest route detours with 6 diagonal steps (3 up, 3 down), so
        // it keeps the step count at the horizontal distance of 16.
        assert_eq!(path.len(), 16);
    }

    #[test]
    fn diagonal_step_only_needs_its_target_walkable() {
        // A 4x4 grid; walls at (2,1) and (1,2) — the two orthogonal cells
        // flanking the step — leave the diagonal from (1,1) to (2,2) legal.
        const W: usize = 4;
        let mut tiles = vec![TileKind::Floor; W * W];
        tiles[2 + W] = TileKind::Wall;
        tiles[1 + 2 * W] = TileKind::Wall;
        let map = GridMap {
            width: W,
            height: W,
            tiles,
        };
        let path = find_path(&map, CellCoord::new(1, 1), CellCoord::new(2, 2))
            .expect("the diagonal only needs its target walkable");
        assert_eq!(path.len(), 1);
        assert_eq!(path[0], CellCoord::new(2, 2));
    }

    #[test]
    fn path_unreachable() {
        let map = GridMap::demo_room();
        assert!(find_path(&map, CellCoord::new(5, 5), CellCoord::new(0, 0)).is_none()); // wall
        assert!(find_path(&map, CellCoord::new(5, 5), CellCoord::new(24, 19)).is_none()); // water
        assert!(find_path(&map, CellCoord::new(5, 5), CellCoord::new(-3, 5)).is_none());
        // oob
    }

    #[test]
    fn path_start_equals_goal() {
        let map = GridMap::demo_room();
        let path = find_path(&map, CellCoord::new(5, 5), CellCoord::new(5, 5)).unwrap();
        assert!(path.is_empty());
    }
}
