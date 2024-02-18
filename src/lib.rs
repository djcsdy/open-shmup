use wasm_bindgen::prelude::wasm_bindgen;
use gloo_console::log;

#[wasm_bindgen]
pub fn start() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    log!("Hello, World!");
}
