use wasm_bindgen::JsValue;
use web_sys::{Element, HtmlCanvasElement, SvgDefsElement, SvgElement, SvgFilterElement};

pub trait ElementExt {
    fn into_canvas(self) -> HtmlCanvasElement;
    fn into_svg(self) -> SvgElement;
    fn into_svg_defs(self) -> SvgDefsElement;
    fn into_svg_filter(self) -> SvgFilterElement;
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

    fn into_svg_filter(self) -> SvgFilterElement {
        JsValue::from(self).into()
    }
}
