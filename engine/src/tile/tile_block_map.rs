use crate::tile::{TileBlock, TileBlockSet};
use web_sys::CanvasRenderingContext2d;

pub struct TileBlockMap {
    tile_blocks: Vec<Vec<TileBlock>>,
}

impl TileBlockMap {
    pub fn new(tile_block_set: &TileBlockSet, map_data: &[u8; 4096]) -> Self {
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

    pub fn draw(&self, context: &CanvasRenderingContext2d, scroll_y: i32) {
        let first_row = scroll_y / 40;
        let y_offset = scroll_y - first_row * 40;

        for row in first_row.max(0)..(first_row + 6).min(512) {
            for tile in 0..8 {
                self.tile_blocks[row as usize][tile].draw(
                    context,
                    tile as f64 * 40.0,
                    (152 - ((row - first_row) * 40) + y_offset) as f64,
                );
            }
        }
    }
}
