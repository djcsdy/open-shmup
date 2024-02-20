use wasm_bindgen::JsValue;
use web_sys::js_sys::TypeError;

pub trait OptionExt<T> {
    fn ok_or_type_error(self) -> Result<T, JsValue>;
}

impl<T> OptionExt<T> for Option<T> {
    fn ok_or_type_error(self) -> Result<T, JsValue> {
        self.ok_or_else(|| JsValue::from(TypeError::new("")))
    }
}
