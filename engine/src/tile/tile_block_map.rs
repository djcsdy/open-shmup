use crate::tile::{TileBlock, TileBlockSet};
use std::array;

pub struct TileBlockMap {
    tile_blocks: [[TileBlock; 8]; 512],
}

impl TileBlockMap {
    pub fn new(tile_block_set: TileBlockSet, map_data: &[u8; 4096]) -> Self {
        TileBlockMap {
            tile_blocks: array::from_fn(|row_index| {
                array::from_fn(|block_index| {
                    tile_block_set[map_data[row_index * 8 + block_index] as usize].clone()
                })
            }),
        }
    }
}
