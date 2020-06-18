mod utils;
mod managers;

use std::panic;
use wasm_bindgen::prelude::*;

extern crate console_error_panic_hook;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn init() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello");
}

#[wasm_bindgen]
pub fn onAnimationFrame() {
    managers::media::getRawImageDataFromCanvas()
}