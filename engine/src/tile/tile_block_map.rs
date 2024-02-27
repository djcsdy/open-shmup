use crate::tile::{TileBlock, TileBlockSet};

pub struct TileBlockMap {
    tile_blocks: Vec<Vec<TileBlock>>,
}

impl TileBlockMap {
    pub fn new(tile_block_set: TileBlockSet, map_data: &[u8; 4096]) -> Self {
        TileBlockMap {
            tile_blocks: (0..512)
                .map(|row_index| {
                    (0..8)
                        .map(|block_index| {
                            tile_block_set[map_data[row_index * 8 + block_index] as usize].clone()
                        })
                        .collect()
                })
                .collect(),
        }
    }
}
