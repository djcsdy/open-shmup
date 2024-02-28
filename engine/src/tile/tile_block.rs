use web_sys::{CanvasRenderingContext2d, ImageBitmap};

#[derive(Clone)]
pub struct TileBlock(ImageBitmap);

impl TileBlock {
    pub fn new(bitmap: ImageBitmap) -> Self {
        Self(bitmap)
    }

    pub fn draw(&self, context: &CanvasRenderingContext2d, x: f64, y: f64) {
        context.draw_image_with_image_bitmap(&self.0, x, y).unwrap();
    }
}
