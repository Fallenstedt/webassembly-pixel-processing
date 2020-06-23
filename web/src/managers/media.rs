use wasm_bindgen::JsCast;
use web_sys::*;

pub struct Frame {
    pub context: web_sys::CanvasRenderingContext2d,
}

// Research how to create a "safe" singleton
// https://rust-embedded.github.io/book/peripherals/singletons.html
impl Frame {
    pub fn new(canvas_id: &str) -> Frame {
        let doc: Document = Frame::get_document();
        let canvas = Frame::get_element_by_id(&doc, canvas_id);
        let context = Frame::get_2d_context(&canvas);

        Frame { context }
    }

    pub fn process_image_data(&self) {
        let width = 1280.0;
        let height = 720.0;
        let data = self
            .context
            .get_image_data(0.0, 0.0, width, height)
            .unwrap()
            .data()
            .to_vec();
    }

    fn get_document() -> Document {
        let doc = web_sys::window().unwrap().document().unwrap();

        doc
    }

    fn get_2d_context(canvas: &web_sys::HtmlCanvasElement) -> web_sys::CanvasRenderingContext2d {
        let context_2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        context_2d
    }

    fn get_element_by_id(d: &Document, id: &str) -> web_sys::HtmlCanvasElement {
        let element = d
            .get_element_by_id(id)
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        element
    }
}
