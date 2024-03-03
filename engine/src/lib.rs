extern crate wee_alloc;

use crate::ext::{DocumentExt, HtmlCanvasElementExt};
use crate::game::Game;
use crate::screen::Screen;
use open_shmup_data::GameData;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::js_sys::{Date, Error};
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

mod ext;
mod game;
mod palette;
mod point;
mod rect;
mod screen;
mod stage;
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

    let game_data =
        GameData::read(&mut game.as_slice()).map_err(|error| Error::new(&error.to_string()))?;
    let mut game = Game::new(game_data).await;

    let start_time = Date::now();

    let context: CanvasRenderingContext2d = canvas2.get_context_2d().unwrap().unwrap();

    let on_animation_frame: Rc<RefCell<Option<Closure<dyn FnMut() -> ()>>>> =
        Rc::new(RefCell::new(None));
    let on_animation_frame_2 = on_animation_frame.clone();
    *on_animation_frame.borrow_mut() = Some(Closure::<dyn FnMut() -> ()>::new(move || {
        let frame = ((Date::now() - start_time) / (1000.0 / 50.0)) as u32;
        game.update(frame);
        game.draw(&context);
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
