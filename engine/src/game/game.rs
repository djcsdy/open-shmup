use crate::game::Stage;
use crate::screen::Screen;
use open_shmup_data::palette::{SrgbColour, SrgbPalette};
use open_shmup_data::GameData;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct Game {
    frame: u32,
    screen: Screen,
    background_colour: SrgbColour,
    stage: Stage,
}

impl Game {
    pub async fn new(data: GameData) -> Self {
        let palette = SrgbPalette::new_colodore();
        let stage = Stage::new(&palette, &data).await;

        Self {
            screen: Screen::C64_PAL,
            frame: 0,
            background_colour: palette[0],
            stage,
        }
    }

    pub fn update(&mut self, frame: u32) {
        for _ in 0..(frame - self.frame).clamp(0, 4) {
            self.stage.update();
        }
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

        self.stage.draw(&self.screen, context);
    }
}
