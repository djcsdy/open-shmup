use wasm_bindgen::JsValue;
use web_sys::{Element, HtmlCanvasElement};

pub trait ElementExt {
    fn into_canvas(self) -> HtmlCanvasElement;
}

impl ElementExt for Element {
    fn into_canvas(self) -> HtmlCanvasElement {
        JsValue::from(self).into()
    }
}
