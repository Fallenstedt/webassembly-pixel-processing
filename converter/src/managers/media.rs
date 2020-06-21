use wasm_bindgen::JsCast;
use web_sys::*;

pub struct Frame {
    canvas_el: web_sys::HtmlCanvasElement
}

impl Frame {
    pub fn new(canvas_id: &str) -> Frame {
        let doc: Document = web_sys::window().unwrap().document().unwrap();
        let canvas_el: web_sys::HtmlCanvasElement = Frame::get_element_by_id(&doc, canvas_id)
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        
        Frame {
            canvas_el,
        }
    }

    fn get_element_by_id(d: &Document, id: &str) -> Element {
        let element = d.get_element_by_id(id);
        match element {
            Some(el) => el,
            None => {
                panic!("no element with id '{:?}' found", id)
            }
        }
    }
}


