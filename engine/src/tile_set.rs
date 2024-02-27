use crate::ext::OptionExt;
use crate::tile::Tile;
use std::array;
use std::ops::Index;
use wasm_bindgen::{Clamped, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, ImageBitmap, ImageBitmapOptions, ImageData, PremultiplyAlpha};

#[derive(Eq, PartialEq, Clone)]
pub struct TileSet([Tile; 254]);

impl TileSet {
    pub async fn new(tile_data: &[u8; 2040]) -> Result<Self, JsValue> {
        let mut image_data = [0u8; 254 * 8 * 8 * 4];
        for tile_index in 0..254 {
            let tile_offset = tile_index * 8;
            for tile_y in 0..8 {
                let line = tile_data[tile_offset + tile_y];
                for tile_x in 0..4 {
                    let out_x_1 = tile_index * 8 + tile_x * 2;
                    let out_x_2 = out_x_1 + 1;
                    let out_pixel_y_offset = tile_y * 254 * 8;
                    let out_pixel_offset_l = out_pixel_y_offset + out_x_1;
                    let out_pixel_offset_r = out_pixel_y_offset + out_x_2;
                    let colour = line.wrapping_shr((8 - tile_x * 2) as u32) & 3;
                    if colour > 0 {
                        image_data[out_pixel_offset_l * 4 + colour as usize - 1] = 255;
                        image_data[out_pixel_offset_r * 4 + colour as usize - 1] = 255;
                    }
                    image_data[out_pixel_offset_l * 4 + 3] = 255;
                    image_data[out_pixel_offset_r * 4 + 3] = 255;
                }
            }
        }
        let image_data = ImageData::new_with_u8_clamped_array(Clamped(&image_data), 254 * 8)?;
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

        Ok(Self(array::from_fn(|index| Tile {
            bitmap: image_bitmap.clone(),
            index: index as u8,
        })))
    }
}

impl Index<usize> for TileSet {
    type Output = Tile;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
