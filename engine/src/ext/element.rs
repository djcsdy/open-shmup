use wasm_bindgen::JsValue;
use web_sys::{Element, HtmlCanvasElement, SvgElement};

pub trait ElementExt {
    fn into_canvas(self) -> HtmlCanvasElement;
    fn into_svg(self) -> SvgElement;
}

impl ElementExt for Element {
    fn into_canvas(self) -> HtmlCanvasElement {
        JsValue::from(self).into()
    }

    fn into_svg(self) -> SvgElement {
        JsValue::from(self).into()
    }
}
