use crate::c64::C64TileData;

pub struct C64MulticolourTileData<'tile_data>(&'tile_data C64TileData);

impl<'tile_data> C64MulticolourTileData<'tile_data> {
    pub fn new(tile_data: &'tile_data C64TileData) -> Self {
        Self(tile_data)
    }
}
