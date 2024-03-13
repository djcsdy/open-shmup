use crate::c64::{C64TileBlockData, C64TileData, C64TileDecode, C64TileSetData};

pub struct C64MulticolourTileBlockData<'tile_block, 'tile_set> {
    tile_block: &'tile_block C64TileBlockData,
    tile_set: &'tile_set C64TileSetData,
}

impl<'tile_block, 'tile_set> C64MulticolourTileBlockData<'tile_block, 'tile_set> {
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

impl C64TileDecode for C64MulticolourTileBlockData<'_, '_> {
    fn colour_index_at(&self, x: usize, y: usize) -> u8 {
        let tile_index = self
            .tile_block
            .get_tile_index(x / C64TileData::WIDTH, y / C64TileData::HEIGHT);
        let x_in_tile = x % C64TileData::WIDTH;
        let y_in_tile = y % C64TileData::HEIGHT;
        self.tile_set[tile_index]
            .as_multicolour()
            .colour_index_at(x_in_tile, y_in_tile)
    }
}
