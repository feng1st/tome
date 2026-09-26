//! Terrain kinds and their game-data properties (walkability).

/// Terrain kinds. Walkability lives here (not in the tileset): only floor
/// can be stepped on; walls and the water pool block movement and
/// pathfinding alike.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TileKind {
    Floor,
    Wall,
    Water,
}

impl TileKind {
    pub fn walkable(self) -> bool {
        matches!(self, TileKind::Floor)
    }
}
