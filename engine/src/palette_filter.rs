use crate::ext::DocumentExt;
use crate::hidden_svg::HiddenSvg;
use crate::palette::Palette;
use std::rc::Rc;
use web_sys::{window, SvgFilterElement};

pub struct PaletteFilter(Rc<Internal>);

#[derive(Clone)]
struct Internal {
    element: SvgFilterElement,
    css: String,
}

impl PaletteFilter {
    pub async fn new(palette: Palette<4>) -> Self {
        let hidden_svg = HiddenSvg::get();
        let document = window().unwrap().document().unwrap();
        let id = document.new_unique_id();

        let filter = document.create_svg_filter_element();
        filter.set_id(&id);
        hidden_svg.defs().append_child(&filter).unwrap();

        let fe_color_matrix = document.create_svg_fe_color_matrix();
        fe_color_matrix
            .set_attribute("in", "SourceGraphic")
            .unwrap();
        fe_color_matrix.set_attribute("type", "matrix").unwrap();
        filter.append_child(&fe_color_matrix).unwrap();

        let values = [
            palette[1].red() - palette[0].red(),
            palette[2].red() - palette[0].red(),
            palette[3].red() - palette[0].red(),
            0.0,
            palette[0].red(),
            palette[1].green() - palette[0].green(),
            palette[2].green() - palette[0].green(),
            palette[3].green() - palette[0].green(),
            0.0,
            palette[0].green(),
            palette[1].blue() - palette[0].blue(),
            palette[2].blue() - palette[0].blue(),
            palette[3].blue() - palette[0].blue(),
            0.0,
            palette[0].blue(),
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
        ];

        fe_color_matrix
            .set_attribute("values", &values.map(|value| value.to_string()).join(" "))
            .unwrap();

        let css = String::from_iter(["url(#", &id, ")"]);

        Self(Rc::new(Internal { element: filter, css }))
    }

    pub fn css(&self) -> &str {
        &self.0.css
    }
}

impl Drop for Internal {
    fn drop(&mut self) {
        match self.element.parent_element() {
            None => {}
            Some(parent) => {
                parent.remove_child(&self.element).unwrap();
            }
        }
    }
}
