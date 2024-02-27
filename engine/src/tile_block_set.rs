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
        tile_block_data: &[u8; 3072],
    ) -> Self {
        Self(array::from_fn(|block_index| {
            TileBlock::new(
                palettes[block_colour_data[block_index] as usize].clone(),
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
