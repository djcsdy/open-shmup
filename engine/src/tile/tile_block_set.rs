use crate::palette::Palette;
use crate::tile::TileBlock;
use crate::tile::TileSet;
use futures::future;
use std::ops::Index;
use wasm_bindgen::Clamped;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, ImageBitmapOptions, ImageData, PremultiplyAlpha};

#[derive(Clone)]
pub struct TileBlockSet(Vec<TileBlock>);

impl TileBlockSet {
    pub async fn new(
        tile_set: &TileSet,
        palettes: &[Palette<4>; 8],
        block_colour_data: &[u8; 128],
        tile_block_data: &[u8; 3200],
    ) -> Self {
        Self(
            future::join_all(
                (0..128)
                    .map(|block_index| {
                        let mut image_data_bytes = [0u8; 40 * 40 * 4];

                        // FIXME: If the eighth bit is not set, we should render the block as hires
                        let palette = &palettes[(block_colour_data[block_index] & 7) as usize];

                        for tile_index_in_block in 0..25 {
                            let tile_x_in_block = (tile_index_in_block % 5) * 8;
                            let tile_y_in_block = (tile_index_in_block / 5) * 8;
                            let tile_data = tile_set
                                [tile_block_data[block_index * 25 + tile_index_in_block] as usize]
                                .tile_data();

                            for y_in_tile in 0..8 {
                                let line = tile_data[y_in_tile];
                                for x_in_tile in 0..4 {
                                    let out_x_l = tile_x_in_block + x_in_tile * 2;
                                    let out_x_r = out_x_l + 1;
                                    let out_y_pixel_offset = (tile_y_in_block + y_in_tile) * 40;
                                    let out_byte_offset_l = (out_y_pixel_offset + out_x_l) * 4;
                                    let out_byte_offset_r = (out_y_pixel_offset + out_x_r) * 4;
                                    let colour =
                                        palette[(line.wrapping_shr((6 - x_in_tile * 2) as u32) & 3)
                                            as usize];
                                    image_data_bytes[out_byte_offset_l] = colour.red_simple_srgb();
                                    image_data_bytes[out_byte_offset_l + 1] =
                                        colour.green_simple_srgb();
                                    image_data_bytes[out_byte_offset_l + 2] =
                                        colour.blue_simple_srgb();
                                    image_data_bytes[out_byte_offset_l + 3] = 255;
                                    image_data_bytes[out_byte_offset_r] = colour.red_simple_srgb();
                                    image_data_bytes[out_byte_offset_r + 1] =
                                        colour.green_simple_srgb();
                                    image_data_bytes[out_byte_offset_r + 2] =
                                        colour.blue_simple_srgb();
                                    image_data_bytes[out_byte_offset_r + 3] = 255;
                                }
                            }
                        }

                        JsFuture::from(
                            window()
                                .unwrap()
                                .create_image_bitmap_with_image_data_and_image_bitmap_options(
                                    &ImageData::new_with_u8_clamped_array(
                                        Clamped(&image_data_bytes),
                                        40,
                                    )
                                    .unwrap(),
                                    ImageBitmapOptions::new()
                                        .premultiply_alpha(PremultiplyAlpha::None),
                                )
                                .unwrap(),
                        )
                    })
                    .map(move |image_data| async {
                        TileBlock::new(image_data.await.unwrap().into())
                    }),
            )
            .await,
        )
    }
}

impl Index<usize> for TileBlockSet {
    type Output = TileBlock;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
