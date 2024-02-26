use std::cell::Cell;
use std::rc::Rc;

use web_sys::{window, Element, SvgDefsElement, SvgElement};

use crate::ext::DocumentExt;

#[derive(Clone)]
pub struct HiddenSvg(Rc<Internal>);

#[derive(Clone)]
struct Internal(Cache);

#[derive(Clone)]
struct Cache {
    svg: SvgElement,
    defs: SvgDefsElement,
}

thread_local! {
    static CACHE: Cell<Option<Cache>> = Cell::new(None);
}

impl HiddenSvg {
    pub fn get() -> Self {
        let cache = CACHE.take().unwrap_or_else(|| {
            let document = window().unwrap().document().unwrap();
            let body = document.body().unwrap();
            let svg = document.create_svg_element();
            svg.set_attribute("style", "display: none").unwrap();
            let defs = document.create_svg_defs_element();
            svg.append_child(&defs).unwrap();
            body.append_child(&svg).unwrap();
            Cache { svg, defs }
        });

        CACHE.set(Some(cache.clone()));

        Self(Rc::new(Internal(cache)))
    }
}

impl Drop for Internal {
    fn drop(&mut self) {
        match self.0.svg.parent_element() {
            None => {}
            Some(parent) => {
                parent.remove_child(&self.0.svg).unwrap();
            }
        }
        CACHE.set(None)
    }
}
