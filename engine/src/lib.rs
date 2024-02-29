extern crate wee_alloc;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::js_sys::{Date, Error};
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

use open_shmup_data::Game;

use crate::ext::{DocumentExt, HtmlCanvasElementExt};
use crate::palette::SrgbPalette;
use crate::screen::Screen;
use crate::tile::TileSet;
use crate::tile::{TileBlockMap, TileBlockSet};

mod ext;
mod palette;
mod screen;
mod tile;

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

    let palette = SrgbPalette::new_colodore();

    context.set_fill_style(&JsValue::from(&palette[0].css()));
    context.fill_rect(0.0, 0.0, 384.0, 288.0);

    let tile_set = TileSet::new(&game.background_tiles);
    let shared_tile_palette = palette.new_shared_tile_palette(&game.background_colours);
    let tile_subpalettes = palette.new_tile_subpalettes(&shared_tile_palette);
    let tile_block_set = TileBlockSet::new(
        &tile_set,
        &tile_subpalettes,
        &game.block_colours,
        &game.block_data,
    )
    .await;
    let tile_block_map = TileBlockMap::new(&tile_block_set, &game.background_scroll_data);

    let start_time = Date::now();

    let on_animation_frame: Rc<RefCell<Option<Closure<dyn Fn() -> ()>>>> =
        Rc::new(RefCell::new(None));
    let on_animation_frame_2 = on_animation_frame.clone();
    *on_animation_frame.borrow_mut() = Some(Closure::<dyn Fn() -> ()>::new(move || {
        let frame = (Date::now() - start_time) / (1000.0 / 50.0);
        screen.with_play_area(&context, |context| {
            tile_block_map.draw(context, frame.floor());
        });
        window()
            .unwrap()
            .request_animation_frame(
                on_animation_frame_2
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .as_ref()
                    .unchecked_ref(),
            )
            .unwrap();
    }));
    window()
        .unwrap()
        .request_animation_frame(
            on_animation_frame
                .borrow()
                .as_ref()
                .unwrap()
                .as_ref()
                .unchecked_ref(),
        )
        .unwrap();

    Ok(())
}
