use crate::tile::Tile;
use std::ops::Index;

#[derive(Eq, PartialEq, Clone)]
pub struct TileSet(Vec<Tile>);

impl TileSet {
    pub fn new(tile_data: &[u8; 2032]) -> Self {
        Self(
            tile_data
                .chunks_exact(8)
                .map(|chunk| chunk.try_into().unwrap())
                .map(|chunk| Tile(chunk))
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
