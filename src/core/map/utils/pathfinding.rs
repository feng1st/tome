use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use bevy::prelude::*;

use crate::core::map::components::cell_coord::CellCoord;
use crate::core::map::resources::terrain_registry::TerrainRegistry;
use crate::core::map::types::local_map::LocalMap;
use crate::core::map::types::terrain::Terrain;

const STRAIGHT_COST: i32 = 10;

const DIAGONAL_COST: i32 = 14;

const DIRECTIONS: [(IVec2, i32); 8] = [
    (IVec2::new(1, 0), STRAIGHT_COST),
    (IVec2::new(-1, 0), STRAIGHT_COST),
    (IVec2::new(0, 1), STRAIGHT_COST),
    (IVec2::new(0, -1), STRAIGHT_COST),
    (IVec2::new(1, 1), DIAGONAL_COST),
    (IVec2::new(1, -1), DIAGONAL_COST),
    (IVec2::new(-1, 1), DIAGONAL_COST),
    (IVec2::new(-1, -1), DIAGONAL_COST),
];

pub fn find_path(
    local_map: &LocalMap,
    terrain_registry: &TerrainRegistry,
    start: CellCoord,
    goal: CellCoord,
    obstacles: &HashSet<CellCoord>,
) -> Option<VecDeque<CellCoord>> {
    let is_walkable = |cell_coord: CellCoord| {
        local_map
            .get(cell_coord)
            .and_then(|terrain_index| terrain_registry.get(terrain_index))
            .is_some_and(Terrain::is_walkable)
    };
    if !is_walkable(start) || !is_walkable(goal) {
        return None;
    }
    if start == goal {
        return Some(VecDeque::new());
    }
    let blocked = |cell_coord: CellCoord| cell_coord != goal && obstacles.contains(&cell_coord);

    let start_h = octile(start, goal);
    let mut open = BinaryHeap::from([Node {
        f: start_h,
        h: start_h,
        cell_coord: start,
    }]);
    let mut g_score: HashMap<CellCoord, i32> = HashMap::from([(start, 0)]);
    let mut came_from: HashMap<CellCoord, CellCoord> = HashMap::new();
    let mut closed = HashSet::new();

    while let Some(Node { cell_coord, .. }) = open.pop() {
        if cell_coord == goal {
            let mut path = VecDeque::from([goal]);
            let mut current = goal;
            while let Some(&previous) = came_from.get(&current) {
                path.push_front(previous);
                current = previous;
            }
            path.pop_front();
            return Some(path);
        }
        if !closed.insert(cell_coord) {
            continue;
        }
        let g = g_score[&cell_coord];
        for (direction, cost) in DIRECTIONS {
            let next = cell_coord + direction;
            if !is_walkable(next) || blocked(next) {
                continue;
            }
            let next_g = g + cost;
            if next_g < *g_score.get(&next).unwrap_or(&i32::MAX) {
                g_score.insert(next, next_g);
                came_from.insert(next, cell_coord);
                let next_h = octile(next, goal);
                open.push(Node {
                    f: next_g + next_h,
                    h: next_h,
                    cell_coord: next,
                });
            }
        }
    }
    None
}

fn octile(a: CellCoord, b: CellCoord) -> i32 {
    let dx = (a.x - b.x).abs();
    let dy = (a.y - b.y).abs();
    let (min, max) = (dx.min(dy), dx.max(dy));
    DIAGONAL_COST * min + STRAIGHT_COST * (max - min)
}

#[derive(PartialEq, Eq, Debug)]
struct Node {
    f: i32,
    h: i32,
    cell_coord: CellCoord,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.cmp(&self.f).then(other.h.cmp(&self.h))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
