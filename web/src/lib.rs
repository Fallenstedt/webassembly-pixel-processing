// This must be first so later modules can use the macros it defines.
mod utils;

mod managers;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-rust-exports/start.html
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    utils::set_panic_hook();
    Ok(())
}

#[wasm_bindgen]
pub fn on_animation_frame() {
    // let frame_manager = managers::media::Frame::new("canvas_element");
    // frame_manager.process_image_data();
    // log!("Processed data")
}
