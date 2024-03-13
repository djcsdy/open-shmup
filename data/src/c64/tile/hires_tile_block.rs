use crate::c64::{C64TileBlockData, C64TileSetData};

pub struct C64HiresTileBlockData<'tile_block, 'tile_set> {
    tile_block: &'tile_block C64TileBlockData,
    tile_set: &'tile_set C64TileSetData,
}

impl<'tile_block, 'tile_set> C64HiresTileBlockData<'tile_block, 'tile_set> {
    pub fn new(
        tile_block: &'tile_block C64TileBlockData,
        tile_set: &'tile_set C64TileSetData,
    ) -> Self {
        Self {
            tile_block,
            tile_set,
        }
    }
}
