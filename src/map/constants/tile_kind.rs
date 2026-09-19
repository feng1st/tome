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
