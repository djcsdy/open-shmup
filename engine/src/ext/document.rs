use crate::ext::element::ElementExt;
use crate::xml_namespace;
use web_sys::{Document, HtmlCanvasElement, SvgDefsElement, SvgElement};

pub trait DocumentExt {
    fn create_canvas_element(&self) -> HtmlCanvasElement;
    fn create_svg_element(&self) -> SvgElement;
    fn create_svg_defs_element(&self) -> SvgDefsElement;
}

impl DocumentExt for Document {
    fn create_canvas_element(&self) -> HtmlCanvasElement {
        self.create_element("canvas").unwrap().into_canvas()
    }

    fn create_svg_element(&self) -> SvgElement {
        self.create_element_ns(Some(xml_namespace::SVG), "svg")
            .unwrap()
            .into_svg()
    }

    fn create_svg_defs_element(&self) -> SvgDefsElement {
        self.create_element_ns(Some(xml_namespace::SVG), "defs")
            .unwrap()
            .into_svg_defs()
    }
}
