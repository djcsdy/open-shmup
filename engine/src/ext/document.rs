use crate::ext::element::ElementExt;
use wasm_bindgen::JsValue;
use web_sys::{Document, HtmlCanvasElement};

pub trait DocumentExt {
    fn create_canvas_element(&self) -> Result<HtmlCanvasElement, JsValue>;
}

impl DocumentExt for Document {
    fn create_canvas_element(&self) -> Result<HtmlCanvasElement, JsValue> {
        Ok(self.create_element("canvas")?.into_canvas())
    }
}
