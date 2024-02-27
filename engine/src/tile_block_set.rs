use crate::palette_filter::PaletteFilter;
use crate::tile_block::TileBlock;
use crate::tile_set::TileSet;
use std::array;
use std::ops::Index;

#[derive(Clone)]
pub struct TileBlockSet([TileBlock; 128]);

impl TileBlockSet {
    pub fn new(
        tile_set: TileSet,
        palettes: [PaletteFilter; 8],
        block_colour_data: &[u8; 128],
        tile_block_data: &[u8; 3200],
    ) -> Self {
        Self(array::from_fn(|block_index| {
            TileBlock::new(
                // FIXME: If the eighth bit is not set, we should render the block as hires
                palettes[(block_colour_data[block_index] & 7) as usize].clone(),
                array::from_fn(|row_index| {
                    array::from_fn(|tile_index| {
                        tile_set[tile_block_data[block_index * 25 + row_index * 5 + tile_index]
                            as usize]
                            .clone()
                    })
                }),
            )
        }))
    }
}

impl Index<usize> for TileBlockSet {
    type Output = TileBlock;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
