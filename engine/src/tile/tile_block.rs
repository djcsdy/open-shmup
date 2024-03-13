use crate::point::Point;
use crate::rect::Rect;
use crate::tile::TileSet;
use open_shmup_data::palette::SrgbPalette;
use wasm_bindgen::Clamped;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    window, CanvasRenderingContext2d, ImageBitmap, ImageBitmapOptions, ImageData, PremultiplyAlpha,
};

#[derive(Clone)]
pub struct TileBlock(ImageBitmap);

impl TileBlock {
    pub async fn decode_multicolour(
        tile_set: &TileSet,
        palette: &SrgbPalette<4>,
        tile_block_data: &[u8; 25],
    ) -> Self {
        Self::decode(true, tile_set, palette, tile_block_data).await
    }

    pub async fn decode_hires(
        tile_set: &TileSet,
        palette: &SrgbPalette<4>,
        tile_block_data: &[u8; 25],
    ) -> Self {
        Self::decode(false, tile_set, palette, tile_block_data).await
    }

    async fn decode(
        multicolour: bool,
        tile_set: &TileSet,
        palette: &SrgbPalette<4>,
        tile_block_data: &[u8; 25],
    ) -> Self {
        let mut image_data_bytes = [0u8; 40 * 40 * 4];

        for tile_index_in_block in 0..25 {
            let tile_x_in_block = (tile_index_in_block % 5) * 8;
            let tile_y_in_block = (tile_index_in_block / 5) * 8;
            let tile_data = tile_set[tile_block_data[tile_index_in_block] as usize].as_array();

            for y_in_tile in 0..8 {
                let line = tile_data[y_in_tile];
                if multicolour {
                    for x_in_tile in 0..4 {
                        let out_x_l = tile_x_in_block + x_in_tile * 2;
                        let out_x_r = out_x_l + 1;
                        let out_y_pixel_offset = (tile_y_in_block + y_in_tile) * 40;
                        let out_byte_offset_l = (out_y_pixel_offset + out_x_l) * 4;
                        let out_byte_offset_r = (out_y_pixel_offset + out_x_r) * 4;
                        let colour =
                            palette[(line.wrapping_shr((6 - x_in_tile * 2) as u32) & 3) as usize];
                        image_data_bytes[out_byte_offset_l] = colour.red();
                        image_data_bytes[out_byte_offset_l + 1] = colour.green();
                        image_data_bytes[out_byte_offset_l + 2] = colour.blue();
                        image_data_bytes[out_byte_offset_l + 3] = 255;
                        image_data_bytes[out_byte_offset_r] = colour.red();
                        image_data_bytes[out_byte_offset_r + 1] = colour.green();
                        image_data_bytes[out_byte_offset_r + 2] = colour.blue();
                        image_data_bytes[out_byte_offset_r + 3] = 255;
                    }
                } else {
                    for x_in_tile in 0..8 {
                        let out_x = tile_x_in_block + x_in_tile;
                        let out_y_pixel_offset = (tile_y_in_block + y_in_tile) * 40;
                        let out_byte_offset = (out_y_pixel_offset + out_x) * 4;
                        let colour = palette[if line & (128 >> x_in_tile) == 0 { 0 } else { 3 }];
                        image_data_bytes[out_byte_offset] = colour.red();
                        image_data_bytes[out_byte_offset + 1] = colour.green();
                        image_data_bytes[out_byte_offset + 2] = colour.blue();
                        image_data_bytes[out_byte_offset + 3] = 255;
                    }
                }
            }
        }

        Self(
            JsFuture::from(
                window()
                    .unwrap()
                    .create_image_bitmap_with_image_data_and_image_bitmap_options(
                        &ImageData::new_with_u8_clamped_array(Clamped(&image_data_bytes), 40)
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
                block_rect.x as f64,
                block_rect.y as f64,
                block_rect.width as f64,
                block_rect.height as f64,
                dest_point.x as f64,
                dest_point.y as f64,
                block_rect.width as f64,
                block_rect.height as f64,
            )
            .unwrap();
    }
}
