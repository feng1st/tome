use crate::core::map::constants::terrain_flags::TerrainFlags;

#[derive(Debug)]
pub struct Terrain {
    pub id: String,
    pub flags: TerrainFlags,
}

impl Terrain {
    pub fn is_walkable(&self) -> bool {
        self.flags.contains(TerrainFlags::PASSABLE)
    }

    pub fn is_liquid(&self) -> bool {
        self.flags.contains(TerrainFlags::LIQUID)
    }
}
