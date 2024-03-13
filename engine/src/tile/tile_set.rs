use crate::tile::Tile;
use futures::future;
use open_shmup_data::c64::C64TileBlockSetData;
use open_shmup_data::palette::SrgbPalette;
use std::ops::Index;

#[derive(Clone)]
pub struct TileSet(Vec<Tile>);

impl TileSet {
    pub async fn new(palette: &SrgbPalette<16>, tile_block_set_data: &C64TileBlockSetData) -> Self {
        Self(
            future::join_all(
                tile_block_set_data
                    .to_srgba_bitmap_iter(palette)
                    .map(|bitmap| async move { Tile::new(&bitmap).await }),
            )
            .await,
        )
    }
}

impl Index<usize> for TileSet {
    type Output = Tile;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
