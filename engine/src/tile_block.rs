use crate::palette_filter::PaletteFilter;
use crate::tile::Tile;

#[derive(Clone)]
pub struct TileBlock {
    palette: PaletteFilter,
    tiles: [[Tile; 5]; 5],
}

impl TileBlock {
    pub fn new(palette: PaletteFilter, tiles: [[Tile; 5]; 5]) -> Self {
        Self { palette, tiles }
    }
}
