use crate::c64::{C64TileData, C64TileDecode};

pub struct C64MulticolourTileData<'tile_data>(&'tile_data C64TileData);

impl<'tile_data> C64MulticolourTileData<'tile_data> {
    pub fn new(tile_data: &'tile_data C64TileData) -> Self {
        Self(tile_data)
    }
}

impl C64TileDecode for C64MulticolourTileData<'_> {
    fn colour_index_at(&self, x: usize, y: usize) -> u8 {
        let x_in_tile = x / 2;
        let line = self.0 .0[y];
        line.wrapping_shr((6 - x_in_tile * 2) as u32) & 3
    }
}
