use crate::palette_filter::PaletteFilter;
use crate::tile_set::TileSet;
use std::array;

#[derive(Clone)]
pub struct TileBlockSet {
    tile_set: TileSet,
    palettes: [PaletteFilter; 8],
    blocks: [TileBlockIndices; 128],
}

#[derive(Clone)]
struct TileBlockIndices {
    palette_index: u8,
    tile_indices: [[u8; 5]; 5],
}

impl TileBlockSet {
    pub fn new(
        tile_set: TileSet,
        palettes: [PaletteFilter; 8],
        block_colour_data: &[u8; 128],
        tile_block_data: &[u8; 3072],
    ) -> Self {
        Self {
            tile_set,
            palettes,
            blocks: array::from_fn(|block_index| TileBlockIndices {
                palette_index: block_colour_data[block_index],
                tile_indices: array::from_fn(|row_index| {
                    array::from_fn(|tile_index| {
                        tile_block_data[block_index * 25 + row_index * 5 + tile_index]
                    })
                }),
            }),
        }
    }
}
