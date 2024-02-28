use crate::ext::ElementExt;
use crate::xml_namespace;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use base64::Engine;
use std::cell::Cell;
use web_sys::{
    window, Document, HtmlCanvasElement, SvgDefsElement, SvgElement, SvgFilterElement,
    SvgfeColorMatrixElement,
};

pub trait DocumentExt {
    fn create_canvas_element(&self) -> HtmlCanvasElement;
    fn create_svg_element(&self) -> SvgElement;
    fn create_svg_defs_element(&self) -> SvgDefsElement;
    fn create_svg_fe_color_matrix(&self) -> SvgfeColorMatrixElement;
    fn create_svg_filter_element(&self) -> SvgFilterElement;
    fn new_unique_id(&self) -> String;
}

thread_local! {
    static UNIQUE_ID_PREFIX: Cell<Option<String>> = Cell::new(None);
    static UNIQUE_ID_NEXT: Cell<u64> = Cell::new(0);
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

    fn create_svg_fe_color_matrix(&self) -> SvgfeColorMatrixElement {
        self.create_element_ns(Some(xml_namespace::SVG), "feColorMatrix")
            .unwrap()
            .into_svg_fe_color_matrix()
    }

    fn create_svg_filter_element(&self) -> SvgFilterElement {
        self.create_element_ns(Some(xml_namespace::SVG), "filter")
            .unwrap()
            .into_svg_filter()
    }

    fn new_unique_id(&self) -> String {
        let mut prefix = UNIQUE_ID_PREFIX.take();
        let next = UNIQUE_ID_NEXT.take();

        let window = window().unwrap();
        let crypto = window.crypto().unwrap();

        if prefix
            .as_ref()
            .and_then(|prefix| {
                self.get_element_by_id(&String::from_iter([
                    prefix.as_str(),
                    next.to_string().as_str(),
                ]))
            })
            .is_some()
        {
            prefix = None;
        }

        while (&prefix).is_none() {
            let mut buffer = [0u8; 8];
            crypto.get_random_values_with_u8_array(&mut buffer).unwrap();
            let candidate_prefix = BASE64_URL_SAFE_NO_PAD.encode(&buffer);
            if self
                .get_element_by_id(&String::from_iter([
                    candidate_prefix.as_str(),
                    next.to_string().as_str(),
                ]))
                .is_none()
            {
                prefix = Some(candidate_prefix);
            }
        }

        let id = String::from_iter([prefix.clone().unwrap().as_str(), next.to_string().as_str()]);

        UNIQUE_ID_PREFIX.set(prefix);
        UNIQUE_ID_NEXT.set(next + 1);

        id
    }
}
