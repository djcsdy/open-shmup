use crate::tile::Tile;
use open_shmup_data::c64::C64TileSetData;
use std::ops::Index;

#[derive(Eq, PartialEq, Clone)]
pub struct TileSet(Vec<Tile>);

impl TileSet {
    pub fn new(tile_data: &C64TileSetData) -> Self {
        Self(
            tile_data
                .iter()
                .map(|tile_data| tile_data.as_array())
                .map(|tile_data| Tile(tile_data.clone()))
                .collect(),
        )
    }
}

impl Index<usize> for TileSet {
    type Output = Tile;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
