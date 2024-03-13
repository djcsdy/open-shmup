use open_shmup_data::c64::{C64TileData, C64TileSetData};
use std::ops::Index;

#[derive(Eq, PartialEq, Clone)]
pub struct TileSet(Vec<C64TileData>);

impl TileSet {
    pub fn new(tile_data: &C64TileSetData) -> Self {
        Self(
            tile_data
                .iter()
                .map(|tile_data| tile_data.clone())
                .collect(),
        )
    }
}

impl Index<usize> for TileSet {
    type Output = C64TileData;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
