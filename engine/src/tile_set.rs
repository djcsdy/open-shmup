use crate::ext::OptionExt;
use wasm_bindgen::{Clamped, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, ImageBitmap, ImageBitmapOptions, ImageData, PremultiplyAlpha};

pub struct TileSet(ImageBitmap);

impl TileSet {
    pub async fn new(tile_data: &[u8; 2040]) -> Result<Self, JsValue> {
        let mut image_data = [0u8; 253 * 8 * 8];
        for tile_index in 0..254 {
            let tile_offset = tile_index * 8;
            for tile_y in 0..8 {
                let line = tile_data[tile_offset + tile_y];
                for tile_x in 0..4 {
                    let out_x = tile_index * 8 + tile_x * 2;
                    let out_offset = out_x * 4;
                    let colour = (line >> (8 - tile_x * 2)) & 3;
                    image_data[out_offset + colour as usize] = 255;
                }
            }
        }
        let image_data = ImageData::new_with_u8_clamped_array(Clamped(&image_data), 253 * 8)?;
        let image_bitmap: ImageBitmap = JsFuture::from(
            window()
                .ok_or_type_error()?
                .create_image_bitmap_with_image_data_and_image_bitmap_options(
                    &image_data,
                    ImageBitmapOptions::new().premultiply_alpha(PremultiplyAlpha::None),
                )?,
        )
        .await?
        .into();
        Ok(Self(image_bitmap))
    }
}
