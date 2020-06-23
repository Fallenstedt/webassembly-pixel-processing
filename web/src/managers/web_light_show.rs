use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

#[wasm_bindgen]
pub struct ClosureHandle(Closure<dyn FnMut()>);

#[derive(Debug)]
struct Dimensions {
    width: f64,
    height: f64,
}

#[derive(Debug)]
struct WebLightShowVideo {
    element: web_sys::HtmlVideoElement,
}

#[derive(Debug)]
struct WebLightShowCanvas {
    context_2d: web_sys::CanvasRenderingContext2d,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct WebLightShow {
    dimensions: Dimensions,
    video: WebLightShowVideo,
    canvas: WebLightShowCanvas,
    animation_id: i32,
}

#[wasm_bindgen]
impl WebLightShow {
    #[wasm_bindgen(constructor)]
    pub fn new(video_id: &str, width: f64, height: f64) -> WebLightShow {
        log!("Creating new WebLightShow");
        let doc: Document = WebLightShow::get_document();

        WebLightShow {
            dimensions: Dimensions { width, height },
            video: WebLightShow::get_video(&doc, video_id),
            canvas: WebLightShow::create_buffer_canvas(&doc),
            animation_id: -1,
        }
    }

    #[wasm_bindgen(method)]
    pub fn test(&self) {
        log!("Hello");
    }

    #[wasm_bindgen(method)]
    pub fn run(self) {
        // Here we want to call `requestAnimationFrame` in a loop, but only a fixed
        // number of times. After it's done we want all our resources cleaned up. To
        // achieve this we're using an `Rc`. The `Rc` will eventually store the
        // closure we want to execute on each frame, but to start out it contains
        // `None`.
        //
        // After the `Rc` is made we'll actually create the closure, and the closure
        // will reference one of the `Rc` instances. The other `Rc` reference is
        // used to store the closure, request the first frame, and then is dropped
        // by this function.
        //
        // Inside the closure we've got a persistent `Rc` reference, which we use
        // for all future iterations of the loop
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        let mut i = 0;
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            if i > 300 {
                // Drop our handle to this closure so that it will get cleaned
                // up once we return.
                let _ = f.borrow_mut().take();
                log!("Done");
                return;
            }
            self.draw_frame_onto_canvas();
            i += 1;
            log!("{:?}", i);

            WebLightShow::request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));
        WebLightShow::request_animation_frame(g.borrow().as_ref().unwrap());
    }

    fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        WebLightShow::get_window()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");
    }

    fn draw_frame_onto_canvas(&self) {
        log!("drawing frame onto canvas");
        &self.canvas.context_2d.draw_image_with_html_video_element(
            &self.video.element,
            self.dimensions.width,
            self.dimensions.height,
        );
    }

    fn create_buffer_canvas(d: &Document) -> WebLightShowCanvas {
        let canvas = d
            .create_element("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>();
        let canvas = match canvas {
            Ok(el) => el,
            Err(e) => panic!("failed to create buffer canvas: {:?}", e),
        };

        let context_2d = &canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>();
        let context_2d = match context_2d {
            Ok(el) => el.clone(),
            Err(e) => panic!("failed to obtain 2d context for buffer canves: {:?}", e),
        };

        WebLightShowCanvas { context_2d }
    }

    fn get_window() -> Window {
        let win = web_sys::window().unwrap();
        win
    }

    fn get_document() -> Document {
        let doc = WebLightShow::get_window().document().unwrap();
        doc
    }

    fn get_video(d: &Document, id: &str) -> WebLightShowVideo {
        let element = d
            .get_element_by_id(id)
            .unwrap()
            .dyn_into::<web_sys::HtmlVideoElement>()
            .unwrap();

        WebLightShowVideo { element }
    }
}
