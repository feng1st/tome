//! Cell adjacency: the eight neighboring cells.

use crate::core::map::components::cell_coord::CellCoord;

/// Two distinct cells are adjacent when they differ by at most one row
/// and one column — the eight neighboring cells, the reach of one step
/// and of one strike. A cell is not adjacent to itself.
pub fn adjacent(a: CellCoord, b: CellCoord) -> bool {
    a != b && (a.x - b.x).abs() <= 1 && (a.y - b.y).abs() <= 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_eight_neighbors_are_adjacent() {
        let center = CellCoord::new(3, 3);
        for d in [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ] {
            assert!(
                adjacent(center, CellCoord::new(center.x + d.0, center.y + d.1)),
                "({d:?}) is adjacent"
            );
        }
    }

    #[test]
    fn distant_cells_and_the_cell_itself_are_not() {
        let center = CellCoord::new(3, 3);
        assert!(!adjacent(center, center));
        assert!(!adjacent(center, CellCoord::new(5, 3)));
        assert!(!adjacent(center, CellCoord::new(3, 5)));
        assert!(!adjacent(center, CellCoord::new(2, 1)));
    }
}
