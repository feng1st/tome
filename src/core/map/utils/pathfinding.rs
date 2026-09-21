//! A* over the walkable grid. Pure cell semantics — no pixel concepts.

use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use bevy::prelude::*;

use crate::core::map::resources::grid_map::GridMap;

/// A* over the walkable grid, 8 directions, cost 10 straight / 14 diagonal,
/// octile-distance heuristic. Diagonal steps only require the target cell to
/// be walkable (corner cutting allowed, matching the original game).
///
/// Costs are integers scaled by 10 so the heap can stay `i32`.
pub fn find_path(map: &GridMap, start: IVec2, goal: IVec2) -> Option<VecDeque<IVec2>> {
    if !map.walkable(start) || !map.walkable(goal) {
        return None;
    }
    if start == goal {
        return Some(VecDeque::new());
    }

    fn heuristic(a: IVec2, b: IVec2) -> i32 {
        let dx = (a.x - b.x).abs();
        let dy = (a.y - b.y).abs();
        let (min, max) = (dx.min(dy), dx.max(dy));
        14 * min + 10 * (max - min)
    }

    #[derive(PartialEq, Eq)]
    struct Node {
        f: i32,
        cell: IVec2,
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
    let mut g_score: HashMap<IVec2, i32> = HashMap::from([(start, 0)]);
    let mut came_from: HashMap<IVec2, IVec2> = HashMap::new();
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
        // Straight line between these cells crosses the pool (x 28..36, y 28..33).
        let start = IVec2::new(20, 30);
        let goal = IVec2::new(44, 30);
        let path = find_path(&map, start, goal).expect("path exists");
        assert_eq!(*path.back().unwrap(), goal);
        for cell in &path {
            assert!(map.walkable(*cell), "path steps on walkable cells only");
            assert!(map.get(*cell) != Some(TileKind::Water));
        }
        // Optimal detour uses 6 diagonal steps (3 up + 3 down), so the step
        // count stays at the horizontal distance of 24.
        assert!(path.len() >= 24);
    }

    #[test]
    fn path_unreachable() {
        let map = GridMap::demo_room();
        assert!(find_path(&map, IVec2::new(5, 5), IVec2::new(0, 0)).is_none()); // wall
        assert!(find_path(&map, IVec2::new(5, 5), IVec2::new(30, 30)).is_none()); // water
        assert!(find_path(&map, IVec2::new(5, 5), IVec2::new(-3, 5)).is_none());
        // oob
    }

    #[test]
    fn path_start_equals_goal() {
        let map = GridMap::demo_room();
        let path = find_path(&map, IVec2::new(5, 5), IVec2::new(5, 5)).unwrap();
        assert!(path.is_empty());
    }
}
