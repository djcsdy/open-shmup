use crate::point::Point;
use crate::rect::Rect;
use open_shmup_data::c64::{C64TileBlockData, C64TileDecode, C64TileSetData};
use open_shmup_data::palette::SrgbPalette;
use wasm_bindgen::Clamped;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    window, CanvasRenderingContext2d, ImageBitmap, ImageBitmapOptions, ImageData, PremultiplyAlpha,
};

#[derive(Clone)]
pub struct TileBlock(ImageBitmap);

impl TileBlock {
    pub async fn decode(
        tile_set: &C64TileSetData,
        palettes: &[SrgbPalette<4>; 8],
        tile_block_data: &C64TileBlockData,
    ) -> Self {
        let palette = &palettes[(tile_block_data.colour_data & 7) as usize];
        if tile_block_data.colour_data & 8 == 8 {
            Self::decode_internal(palette, tile_block_data.as_multicolour(tile_set)).await
        } else {
            Self::decode_internal(palette, tile_block_data.as_hires(tile_set)).await
        }
    }

    async fn decode_internal<D: C64TileDecode>(
        palette: &SrgbPalette<4>,
        tile_block_decode: D,
    ) -> Self {
        let bitmap = tile_block_decode.to_srgba_bitmap(palette);

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
