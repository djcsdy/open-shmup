use crate::c64::{C64TileData, C64TileDecode};

pub struct C64HiresTileData<'tile_data>(&'tile_data C64TileData);

impl<'tile_data> C64HiresTileData<'tile_data> {
    pub fn new(tile_data: &'tile_data C64TileData) -> Self {
        Self(tile_data)
    }
}

impl C64TileDecode for C64HiresTileData<'_> {
    fn width(&self) -> usize {
        C64TileData::WIDTH
    }

    fn height(&self) -> usize {
        C64TileData::HEIGHT
    }

    fn colour_index_at(&self, x: usize, y: usize) -> u8 {
        let line = self.0 .0[y];
        if line & (128 >> x) == 0 {
            0
        } else {
            3
        }
    }
}
