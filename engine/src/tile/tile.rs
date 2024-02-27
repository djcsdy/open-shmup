use web_sys::{CanvasRenderingContext2d, ImageBitmap};

#[derive(Eq, PartialEq, Clone)]
pub struct Tile {
    pub(super) bitmap: ImageBitmap,
    pub(super) index: u8,
}

impl Tile {
    pub fn draw(&self, context: &CanvasRenderingContext2d, x: f64, y: f64) {
        context
            .draw_image_with_image_bitmap_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &self.bitmap,
                self.index as f64 * 8.0,
                0.0,
                8.0,
                8.0,
                x,
                y,
                8.0,
                8.0,
            )
            .unwrap()
    }
}
