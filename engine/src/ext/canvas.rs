use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub trait HtmlCanvasElementExt {
    fn get_context_2d(&self) -> Result<Option<CanvasRenderingContext2d>, JsValue>;
}

impl HtmlCanvasElementExt for HtmlCanvasElement {
    fn get_context_2d(&self) -> Result<Option<CanvasRenderingContext2d>, JsValue> {
        let context: JsValue = self.get_context("2d")?.into();
        Ok(if context.is_undefined() || context.is_null() {
            None
        } else {
            Some(context.into())
        })
    }
}
