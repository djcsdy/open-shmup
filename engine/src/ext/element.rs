use wasm_bindgen::JsValue;
use web_sys::{Element, HtmlCanvasElement, SvgDefsElement, SvgElement};

pub trait ElementExt {
    fn into_canvas(self) -> HtmlCanvasElement;
    fn into_svg(self) -> SvgElement;
    fn into_svg_defs(self) -> SvgDefsElement;
}

impl ElementExt for Element {
    fn into_canvas(self) -> HtmlCanvasElement {
        JsValue::from(self).into()
    }

    fn into_svg(self) -> SvgElement {
        JsValue::from(self).into()
    }
    
    fn into_svg_defs(self) -> SvgDefsElement {
        JsValue::from(self).into()
    }
}
