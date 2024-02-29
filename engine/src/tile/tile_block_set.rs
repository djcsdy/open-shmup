use crate::palette::SrgbPalette;
use crate::tile::TileBlock;
use crate::tile::TileSet;
use arrayref::array_ref;
use futures::future;
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
                TileBlock::decode_multicolour(
                    &tile_set,
                    // FIXME: If the eighth bit is not set, we should render the block as hires
                    &palettes[(block_colour_data[block_index] & 7) as usize],
                    array_ref![tile_block_data, block_index * 25, 25],
                )
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
