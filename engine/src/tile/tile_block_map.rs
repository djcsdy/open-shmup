use crate::point::Point;
use crate::rect::Rect;
use crate::tile::{TileBlock, TileBlockSet};
use web_sys::CanvasRenderingContext2d;

pub struct TileBlockMap {
    tile_blocks: Vec<Vec<TileBlock>>,
}

impl TileBlockMap {
    pub const HEIGHT_PX: u32 = 512 * 40;

    pub fn new(tile_block_set: &TileBlockSet, map_data: &[u8; 4096]) -> Self {
        TileBlockMap {
            tile_blocks: (0..512)
                .map(|row_index| {
                    (0..8)
                        .map(|block_index| {
                            tile_block_set[map_data[(511 - row_index) * 8 + block_index] as usize]
                                .clone()
                        })
                        .collect()
                })
                .collect(),
        }
    }

    pub fn draw(&self, context: &CanvasRenderingContext2d, src_y: i32) {
        let top_row = src_y / 40;
        let y_offset = top_row * 40 - src_y;

        for row in top_row.max(0)..(top_row + 6).min(512) {
            self.draw_row(
                context,
                row as usize,
                &Point {
                    x: 0,
                    y: ((row - top_row) * 40) + y_offset,
                },
            );
        }
    }

    fn draw_row(&self, context: &CanvasRenderingContext2d, row: usize, dest_pos: &Point) {
        for tile in 0..8 {
            self.tile_blocks[row][tile].draw(
                context,
                &Rect {
                    x: 0,
                    y: 0,
                    width: 40,
                    height: 40,
                },
                &Point {
                    x: tile as i32 * 40 + dest_pos.x,
                    y: dest_pos.y,
                },
            )
        }
    }
}
