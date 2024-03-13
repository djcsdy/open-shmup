use crate::image::SrgbaBitmap;
use crate::palette::SrgbPalette;

pub trait C64TileDecode {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn colour_index_at(&self, x: usize, y: usize) -> u8;

    fn to_srgba_bitmap(&self, palette: &SrgbPalette<4>) -> SrgbaBitmap {
        SrgbaBitmap::from_srgb_fn(self.width(), self.height(), |x, y| {
            palette[self.colour_index_at(x, y) as usize]
        })
    }
}
