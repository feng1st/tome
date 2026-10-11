use bitflags::bitflags;

bitflags! {
    #[derive(Debug)]
    pub struct TerrainFlags: u32 {
        const PASSABLE = 0b001;
        const LIQUID = 0b010;
    }
}
