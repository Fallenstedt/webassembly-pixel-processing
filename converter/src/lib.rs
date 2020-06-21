mod utils;
mod managers;


use std::panic;
use wasm_bindgen::prelude::*;

extern crate console_error_panic_hook;
extern crate web_sys;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn init(canvas_id: &str) {
    utils::set_panic_hook();
    let f = managers::media::Frame::new(canvas_id);
}

#[wasm_bindgen]
pub fn onAnimationFrame() {
    // managers::media::getRawImageDataFromCanvas()
}