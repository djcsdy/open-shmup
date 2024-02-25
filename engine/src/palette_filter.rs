use crate::palette::Palette;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::{Array, Promise};
use web_sys::{window, Blob, BlobPropertyBag, Url, XmlSerializer};

pub struct PaletteFilter {
    blob_url: String,
    css: String,
}

impl PaletteFilter {
    pub async fn new(palette: Palette<4>) -> Self {
        let namespace = Some("http://www.w3.org/2000/svg");

        let document = window()
            .unwrap()
            .document()
            .unwrap()
            .implementation()
            .unwrap()
            .create_document(namespace, "svg")
            .unwrap();

        let svg = document.document_element().unwrap();

        let defs = document.create_element_ns(namespace, "defs").unwrap();
        svg.append_child(&defs).unwrap();

        let filter = document.create_element_ns(namespace, "filter").unwrap();
        filter.set_id("f");
        defs.append_child(&filter).unwrap();

        let fe_color_matrix = document
            .create_element_ns(namespace, "feColorMatrix")
            .unwrap();
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

        let svg_text = XmlSerializer::new()
            .unwrap()
            .serialize_to_string(&svg)
            .unwrap();

        let blob = Blob::new_with_str_sequence_and_options(
            &Array::from_iter(&[&JsValue::from_str(&svg_text)]),
            BlobPropertyBag::new().type_("image/svg+xml"),
        )
        .unwrap();

        let blob_url = Url::create_object_url_with_blob(&blob).unwrap();
        let css = String::from_iter(["url(", &blob_url, "#f)"]);

        // The Blob URL does not become usable until the next turn of the event loop
        JsFuture::from(Promise::resolve(&JsValue::undefined()))
            .await
            .unwrap();

        Self { blob_url, css }
    }

    pub fn as_css(&self) -> &str {
        &self.css
    }
}

impl Drop for PaletteFilter {
    fn drop(&mut self) {
        Url::revoke_object_url(&self.blob_url).unwrap();
    }
}
