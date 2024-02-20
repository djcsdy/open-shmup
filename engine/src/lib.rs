mod ext;
mod tile_set;

extern crate wee_alloc;

use crate::ext::{DocumentExt, HtmlCanvasElementExt, OptionExt};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn start(canvas: Option<HtmlCanvasElement>) -> Result<(), JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    let document = window().ok_or_type_error()?.document().ok_or_type_error()?;
    let body = document.body().ok_or_type_error()?;

    let context: CanvasRenderingContext2d = canvas
        .ok_or_type_error()
        .or_else(|_| {
            let canvas: HtmlCanvasElement = document.create_canvas_element()?;
            body.append_child(&canvas)?;
            Ok::<HtmlCanvasElement, JsValue>(canvas)
        })?
        .get_context_2d()?
        .ok_or_type_error()?;

    context.set_text_baseline("top");
    context.set_font("32px sans-serif");
    context.fill_text("Hello, World!", 0.0, 0.0)?;

    Ok(())
}
