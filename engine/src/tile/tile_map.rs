use crate::tile::{Tile, TileSet};
use open_shmup_data::{Point, Rect};
use web_sys::CanvasRenderingContext2d;

pub struct TileMap {
    tile_blocks: Vec<Vec<Tile>>,
}

impl TileMap {
    pub fn new(tile_block_set: &TileSet, map_data: &[u8; 4096]) -> Self {
        TileMap {
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

    pub fn draw(&self, context: &CanvasRenderingContext2d, src_rect: Rect, dest_pos: Point) {
        let top_row = src_rect.top() / 40;
        let bottom_row = (src_rect.bottom() - 1) / 40;
        let row_count = bottom_row - top_row + 1;

        let top_rect = Rect::from_top_left_width_height(
            src_rect.top_left()
                + Point {
                    x: 0,
                    y: top_row * -40,
                },
            src_rect.width(),
            if row_count == 1 {
                src_rect.height()
            } else {
                40
            },
        );

        let mid_rect = Rect::from_top_left_width_height(
            Point {
                x: src_rect.left(),
                y: 0,
            },
            src_rect.width(),
            40,
        );

        let bottom_rect = Rect::from_top_left_bottom_right(
            Point {
                x: src_rect.left(),
                y: 0,
            },
            Point {
                x: src_rect.right(),
                y: if src_rect.bottom() % 40 == 0 {
                    40
                } else {
                    src_rect.bottom() % 40
                },
            },
        );

        for row_index in 0..row_count {
            self.draw_row(
                context,
                (top_row + row_index) as usize,
                if row_index == 0 {
                    top_rect
                } else if row_index == row_count - 1 {
                    bottom_rect
                } else {
                    mid_rect
                },
                dest_pos
                    + Point {
                        x: 0,
                        y: row_index * 40 - if row_index == 0 { 0 } else { top_rect.top() },
                    },
            );
        }
    }

    fn draw_row(
        &self,
        context: &CanvasRenderingContext2d,
        row: usize,
        src_rect: Rect,
        dest_pos: Point,
    ) {
        let left_tile = src_rect.left() / 40;
        let right_tile = (src_rect.right() - 1) / 40;
        let tile_count = right_tile - left_tile + 1;

        let left_rect = Rect::from_top_left_width_height(
            src_rect.top_left()
                + Point {
                    x: left_tile * -40,
                    y: 0,
                },
            if tile_count == 1 {
                src_rect.width()
            } else {
                40
            },
            src_rect.height(),
        );

        let mid_rect = Rect::from_top_left_width_height(
            Point {
                x: 0,
                y: src_rect.top(),
            },
            40,
            src_rect.height(),
        );

        let right_rect = Rect::from_top_left_bottom_right(
            Point {
                x: 0,
                y: src_rect.top(),
            },
            Point {
                x: if src_rect.right() % 40 == 0 {
                    40
                } else {
                    src_rect.right() % 40
                },
                y: src_rect.bottom(),
            },
        );

        for tile_index in 0..tile_count {
            self.tile_blocks[row][(left_tile + tile_index) as usize].draw(
                context,
                if tile_index == 0 {
                    left_rect
                } else if tile_index == tile_count - 1 {
                    right_rect
                } else {
                    mid_rect
                },
                dest_pos
                    + Point {
                        x: tile_index * 40 - if tile_index == 0 { 0 } else { left_rect.left() },
                        y: 0,
                    },
            )
        }
    }
}
