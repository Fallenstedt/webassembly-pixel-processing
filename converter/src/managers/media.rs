use wasm_bindgen::JsCast;
use web_sys::*;

pub fn getRawImageDataFromCanvas() {
    let doc: Document = web_sys::window().unwrap().document().unwrap();
    let canvas_el: web_sys::HtmlCanvasElement = get_element_by_id(&doc, "canvas_element")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

}

fn get_element_by_id(d: &Document, id: &str) -> Element {
    let element = d.get_element_by_id(id);
    match element {
        Some(el) => el,
        None => panic!("no element with id '{:?}' found", id),
    }
}
