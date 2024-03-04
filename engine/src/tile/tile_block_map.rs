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
                &Rect {
                    x: 0,
                    y: 0,
                    width: 320,
                    height: 40,
                },
                &Point {
                    x: 0,
                    y: ((row - top_row) * 40) + y_offset,
                },
            );
        }
    }

    fn draw_row(
        &self,
        context: &CanvasRenderingContext2d,
        row: usize,
        src_rect: &Rect,
        dest_pos: &Point,
    ) {
        let left_tile = src_rect.x / 40;
        let right_tile = (src_rect.x + src_rect.width as i32 - 1) / 40;
        let tile_count = right_tile - left_tile + 1;

        let left_rect = Rect {
            x: src_rect.x - left_tile * 40,
            y: src_rect.y,
            width: if tile_count == 1 { src_rect.width } else { 40 },
            height: src_rect.height,
        };

        let mid_rect = Rect {
            x: 0,
            y: src_rect.y,
            width: 40,
            height: src_rect.height,
        };

        let src_right = src_rect.x as u32 + src_rect.width;

        let right_rect = Rect {
            x: 0,
            y: src_rect.y,
            width: if src_right % 40 == 0 {
                40
            } else {
                src_right % 40
            },
            height: src_rect.height,
        };

        for tile_index in 0..tile_count {
            self.tile_blocks[row][(left_tile + tile_index) as usize].draw(
                context,
                if tile_index == 0 {
                    &left_rect
                } else if tile_index == tile_count - 1 {
                    &right_rect
                } else {
                    &mid_rect
                },
                &Point {
                    x: dest_pos.x + tile_index * 40 - if tile_index == 0 { 0 } else { left_rect.x },
                    y: dest_pos.y,
                },
            )
        }
    }
}
