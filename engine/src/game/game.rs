use crate::rect::Rect;
use crate::screen::Screen;
use crate::tile::{TileBlockMap, TileBlockSet, TileSet};
use open_shmup_data::palette::SrgbPalette;
use open_shmup_data::GameData;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct Game {
    frame: u32,
    screen: Screen,
    palette: SrgbPalette<16>,
    tile_block_map: TileBlockMap,
}

impl Game {
    pub async fn new(data: GameData) -> Self {
        let palette = SrgbPalette::new_colodore();
        let tile_set = TileSet::new(&data.tile_set.tiles);
        let shared_tile_palette = palette.new_shared_tile_palette(&data.tile_set.shared_colours);
        let tile_subpalettes = palette.new_tile_subpalettes(&shared_tile_palette);
        let tile_block_set = TileBlockSet::new(
            &tile_set,
            &tile_subpalettes,
            &data.tile_set.block_colours,
            &data.tile_set.block_data,
        )
        .await;
        let tile_block_map = TileBlockMap::new(&tile_block_set, &data.background_scroll_data);

        Self {
            screen: Screen::C64_PAL,
            frame: 0,
            palette,
            tile_block_map,
        }
    }

    pub fn update(&mut self, frame: u32) {
        self.frame = frame;
    }

    pub fn draw(&self, context: &CanvasRenderingContext2d) {
        context.set_fill_style(&JsValue::from(self.palette[0].css()));
        context.fill_rect(
            0.0,
            0.0,
            self.screen.width as f64,
            self.screen.height as f64,
        );

        self.tile_block_map.draw(
            context,
            &Rect {
                x: 0,
                y: TileBlockMap::HEIGHT_PX as i32 - 192 - self.frame as i32,
                width: 320,
                height: 192,
            },
            &self.screen.play_area.top_left(),
        );
    }
}