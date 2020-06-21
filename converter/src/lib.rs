mod utils;
mod managers;

// The wasm-pack uses wasm-bindgen to build and generate JavaScript binding file.
// Import the wasm-bindgen crate.
use wasm_bindgen::prelude::*;

extern crate console_error_panic_hook;
extern crate web_sys;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn init_web() {
    utils::set_panic_hook();
}

#[wasm_bindgen]
pub fn on_animation_frame() {
    let d = managers::media::Frame::new("canvas_element");
    d.process_image_data();
    log!("Processed data")
}