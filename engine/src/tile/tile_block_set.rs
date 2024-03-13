use crate::tile::TileBlock;
use crate::tile::TileSet;
use futures::future;
use open_shmup_data::c64::C64TileBlockData;
use open_shmup_data::palette::SrgbPalette;
use std::ops::Index;

#[derive(Clone)]
pub struct TileBlockSet(Vec<TileBlock>);

impl TileBlockSet {
    pub async fn new(
        tile_set: &TileSet,
        palettes: &[SrgbPalette<4>; 8],
        blocks: &[C64TileBlockData; 128],
    ) -> Self {
        Self(
            future::join_all(blocks.iter().map(|block| {
                let palette = &palettes[(block.colour_data & 7) as usize];

                async move {
                    if (block.colour_data & 8) == 8 {
                        TileBlock::decode_multicolour(&tile_set, palette, &block.tile_indices).await
                    } else {
                        TileBlock::decode_hires(&tile_set, palette, &block.tile_indices).await
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
