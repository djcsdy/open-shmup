use crate::ext::ElementExt;
use web_sys::{Document, HtmlCanvasElement};

pub trait DocumentExt {
    fn create_canvas_element(&self) -> HtmlCanvasElement;
}

impl DocumentExt for Document {
    fn create_canvas_element(&self) -> HtmlCanvasElement {
        self.create_element("canvas").unwrap().into_canvas()
    }
}
