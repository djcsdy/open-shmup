extern crate wee_alloc;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::js_sys::Error;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

use open_shmup_data::Game;

use crate::ext::{DocumentExt, HtmlCanvasElementExt, OptionExt};
use crate::palette::Palette;
use crate::palette_filter::PaletteFilter;
use crate::tile_set::TileSet;

mod colour;
mod ext;
mod hidden_svg;
mod palette;
mod palette_filter;
mod tile_set;
mod xml_namespace;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub async fn start(game: Vec<u8>, canvas: Option<HtmlCanvasElement>) -> Result<(), JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    let document = window().unwrap().document().unwrap();
    let body = document.body().unwrap();

    let context: CanvasRenderingContext2d = canvas
        .unwrap_or_else(|| {
            let canvas = document.create_canvas_element();
            canvas.set_width(384);
            canvas.set_height(288);
            body.append_child(&canvas).unwrap();
            canvas
        })
        .get_context_2d()
        .unwrap()
        .unwrap();

    let game = Game::read(&mut game.as_slice()).map_err(|error| Error::new(&error.to_string()))?;

    let palette = Palette::new_colodore();
    for i in 0..palette.len() {
        let colour = palette[i];
        context.set_fill_style(&JsValue::from_str(&colour.css()));
        context.fill_rect(i as f64 * 20.0, 0.0, 20.0, 20.0);
    }

    let common_tile_subpalette =
        Palette::new_common_tile_subpalette(&palette, &game.background_colours);

    let background_palette = Palette::new([
        common_tile_subpalette[0],
        common_tile_subpalette[1],
        common_tile_subpalette[2],
        palette[7],
    ]);

    let background_filter = PaletteFilter::new(background_palette).await;
    context.set_filter(background_filter.css());

    let tile_set = TileSet::new(&game.background_tiles).await.unwrap();
    for i in 0..254 {
        let x = (i & 31) as f64 * 8.0;
        let y = 20.0 + (i / 32) as f64 * 8.0;
        tile_set.draw_tile(&context, i, x, y);
    }

    context.set_fill_style(&JsValue::from_str("#000"));
    context.fill_rect(0.0, 84.0, 20.0, 20.0);

    context.set_fill_style(&JsValue::from_str("#f00"));
    context.fill_rect(20.0, 84.0, 20.0, 20.0);

    context.set_fill_style(&JsValue::from_str("#0f0"));
    context.fill_rect(40.0, 84.0, 20.0, 20.0);

    context.set_fill_style(&JsValue::from_str("#00f"));
    context.fill_rect(60.0, 84.0, 20.0, 20.0);

    Ok(())
}
