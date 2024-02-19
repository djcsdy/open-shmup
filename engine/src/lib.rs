extern crate wee_alloc;

use wasm_bindgen::prelude::wasm_bindgen;
use gloo_console::log;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn start() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    log!("Hello, World!");
}
