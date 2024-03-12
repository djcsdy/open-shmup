use crate::tile::TileBlock;
use crate::tile::TileSet;
use arrayref::array_ref;
use futures::future;
use open_shmup_data::palette::SrgbPalette;
use std::ops::Index;

#[derive(Clone)]
pub struct TileBlockSet(Vec<TileBlock>);

impl TileBlockSet {
    pub async fn new(
        tile_set: &TileSet,
        palettes: &[SrgbPalette<4>; 8],
        block_colour_data: &[u8; 128],
        tile_block_data: &[u8; 3200],
    ) -> Self {
        Self(
            future::join_all((0..128).map(|block_index| {
                let colour_data = block_colour_data[block_index];
                let block_data = array_ref![tile_block_data, block_index * 25, 25];
                let palette = &palettes[(colour_data & 7) as usize];

                async move {
                    if (colour_data & 8) == 8 {
                        TileBlock::decode_multicolour(&tile_set, palette, block_data).await
                    } else {
                        TileBlock::decode_hires(&tile_set, palette, block_data).await
                    }
                }
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
