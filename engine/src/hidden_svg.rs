use std::cell::Cell;
use std::rc::{Rc, Weak};

use web_sys::{window, Element, SvgDefsElement, SvgElement};

use crate::ext::DocumentExt;

#[derive(Clone)]
pub struct HiddenSvg(Rc<Internal>);

#[derive(Clone)]
struct Internal {
    svg: SvgElement,
    defs: SvgDefsElement,
}

thread_local! {
    static CACHE: Cell<Weak<Internal>> = Cell::new(Weak::new());
}

impl HiddenSvg {
    pub fn get() -> Self {
        let cache = CACHE.take().upgrade().unwrap_or_else(|| {
            let document = window().unwrap().document().unwrap();
            let body = document.body().unwrap();
            let svg = document.create_svg_element();
            svg.set_attribute("style", "display: none").unwrap();
            let defs = document.create_svg_defs_element();
            svg.append_child(&defs).unwrap();
            body.append_child(&svg).unwrap();
            Rc::new(Internal { svg, defs })
        });

        CACHE.set(Rc::downgrade(&cache));

        Self(cache)
    }

    pub fn defs(&self) -> &SvgDefsElement {
        &self.0.defs
    }
}

impl Drop for Internal {
    fn drop(&mut self) {
        match self.svg.parent_element() {
            None => {}
            Some(parent) => {
                parent.remove_child(&self.svg).unwrap();
            }
        }
    }
}
