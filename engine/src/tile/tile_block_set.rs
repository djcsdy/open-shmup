use crate::tile::TileBlock;
use futures::future;
use open_shmup_data::c64::C64TileBlockSetData;
use open_shmup_data::palette::SrgbPalette;
use std::ops::Index;

#[derive(Clone)]
pub struct TileBlockSet(Vec<TileBlock>);

impl TileBlockSet {
    pub async fn new(
        palettes: &[SrgbPalette<4>; 8],
        tile_block_set_data: &C64TileBlockSetData,
    ) -> Self {
        Self(
            future::join_all(tile_block_set_data.blocks.iter().map(|block| async move {
                TileBlock::decode(&tile_block_set_data.tile_set, palettes, block).await
            }))
            .await,
        )
    }
}

impl Index<usize> for TileBlockSet {
    type Output = TileBlock;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
