use wasm_bindgen::JsCast;
use web_sys::*;

pub struct Frame {
    canvas: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
}

// TODO
// Pass raw image data over instead of having rust search the DOM 60fps.
// Or research how to create a "safe" singleton 
// https://rust-embedded.github.io/book/peripherals/singletons.html

impl Frame {
    pub fn new(canvas_id: &str) -> Frame {
        let doc: Document = Frame::get_document();
        let canvas = Frame::get_element_by_id(&doc, canvas_id);        
        let context = Frame::get_2d_context(&canvas);

        Frame{
            canvas,
            context
        }
        
    }

    fn get_document() -> Document {
        let doc = web_sys::window().unwrap().document().unwrap();

        doc
    }

    fn get_2d_context(canvas: &web_sys::HtmlCanvasElement) -> web_sys::CanvasRenderingContext2d {
        let context_2d = canvas.get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        
            context_2d
        
    }

    fn get_element_by_id(d: &Document, id: &str) -> web_sys::HtmlCanvasElement {
        let element = d.get_element_by_id(id)
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

            element
    }
}


