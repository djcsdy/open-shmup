use crate::palette_filter::PaletteFilter;
use crate::tile::Tile;
use web_sys::CanvasRenderingContext2d;

#[derive(Clone)]
pub struct TileBlock {
    palette: PaletteFilter,
    tiles: [[Tile; 5]; 5],
}

impl TileBlock {
    pub fn new(palette: PaletteFilter, tiles: [[Tile; 5]; 5]) -> Self {
        Self { palette, tiles }
    }

    pub fn draw(&self, context: &CanvasRenderingContext2d, x: f64, y: f64) {
        context.save();
        context.set_filter(self.palette.css());
        for row in 0..5 {
            for tile in 0..5 {
                self.tiles[row][tile].draw(context, x + tile as f64 * 8.0, y + row as f64 * 8.0);
            }
        }
        context.restore();
    }
}
