use open_shmup_data::image::SrgbaBitmap;
use open_shmup_data::{Point, Rect};
use wasm_bindgen::Clamped;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    window, CanvasRenderingContext2d, ImageBitmap, ImageBitmapOptions, ImageData, PremultiplyAlpha,
};

#[derive(Clone)]
pub struct Tile(ImageBitmap);

impl Tile {
    pub async fn new(bitmap: &SrgbaBitmap) -> Self {
        Self(
            JsFuture::from(
                window()
                    .unwrap()
                    .create_image_bitmap_with_image_data_and_image_bitmap_options(
                        &ImageData::new_with_u8_clamped_array(
                            Clamped(&bitmap.bitmap),
                            bitmap.width as u32,
                        )
                        .unwrap(),
                        ImageBitmapOptions::new().premultiply_alpha(PremultiplyAlpha::None),
                    )
                    .unwrap(),
            )
            .await
            .unwrap()
            .into(),
        )
    }

    pub fn draw(&self, context: &CanvasRenderingContext2d, block_rect: &Rect, dest_point: &Point) {
        context
            .draw_image_with_image_bitmap_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &self.0,
                block_rect.left() as f64,
                block_rect.top() as f64,
                block_rect.width() as f64,
                block_rect.height() as f64,
                dest_point.x as f64,
                dest_point.y as f64,
                block_rect.width() as f64,
                block_rect.height() as f64,
            )
            .unwrap();
    }
}
