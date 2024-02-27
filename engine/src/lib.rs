extern crate wee_alloc;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::js_sys::Error;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

use open_shmup_data::Game;

use crate::ext::{DocumentExt, HtmlCanvasElementExt, OptionExt};
use crate::palette::Palette;
use crate::palette::PaletteFilter;
use crate::screen::Screen;
use crate::tile::TileBlockSet;
use crate::tile::TileSet;

mod ext;
mod hidden_svg;
mod palette;
mod screen;
mod tile;
mod xml_namespace;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub async fn start(game: Vec<u8>, canvas: Option<HtmlCanvasElement>) -> Result<(), JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    let document = window().unwrap().document().unwrap();
    let body = document.body().unwrap();

    let screen = Screen::C64_PAL;

    let canvas2 = canvas.unwrap_or_else(|| {
        let canvas = document.create_canvas_element();
        body.append_child(&canvas).unwrap();
        canvas
    });

    canvas2.set_width(screen.width());
    canvas2.set_height(screen.height());

    let context: CanvasRenderingContext2d = canvas2.get_context_2d().unwrap().unwrap();

    let game = Game::read(&mut game.as_slice()).map_err(|error| Error::new(&error.to_string()))?;

    let palette = Palette::new_colodore();

    context.set_fill_style(&JsValue::from(&palette[0].css()));
    context.fill_rect(0.0, 0.0, 384.0, 288.0);

    let tile_set = TileSet::new(&game.background_tiles).await;

    screen.with_play_area(&context, |context| {
        let tile_subpalettes = palette
            .new_tile_subpalettes(&game.background_colours)
            .map(|subpalette| PaletteFilter::new(&subpalette));
        let tile_block_set = TileBlockSet::new(
            tile_set,
            tile_subpalettes,
            &game.block_colours,
            &game.block_data,
        );
        for i in 0..128 {
            let x = (i & 7) as f64 * 40.0;
            let y = (i / 8) as f64 * 40.0;
            tile_block_set[i].draw(&context, x, y);
        }
    });

    Ok(())
}
