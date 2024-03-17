use crate::rect::Rect;
use crate::screen::Screen;
use crate::tile::{TileMap, TileSet};
use open_shmup_data::palette::{SrgbColour, SrgbPalette};
use open_shmup_data::GameData;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct Game {
    frame: u32,
    screen: Screen,
    background_colour: SrgbColour,
    tile_block_map: TileMap,
}

impl Game {
    pub async fn new(data: GameData) -> Self {
        let palette = SrgbPalette::new_colodore();
        let tile_block_set = TileSet::new(&palette, &data.tile_set).await;
        let tile_block_map = TileMap::new(&tile_block_set, &data.background_scroll_data);

        Self {
            screen: Screen::C64_PAL,
            frame: 0,
            background_colour: palette[0],
            tile_block_map,
        }
    }

    pub fn update(&mut self, frame: u32) {
        self.frame = frame;
    }

    pub fn draw(&self, context: &CanvasRenderingContext2d) {
        context.set_fill_style(&JsValue::from(self.background_colour.css()));
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
                y: TileMap::HEIGHT_PX as i32 - 192 - self.frame as i32,
                width: 320,
                height: 192,
            },
            &self.screen.play_area.top_left(),
        );
    }
}
