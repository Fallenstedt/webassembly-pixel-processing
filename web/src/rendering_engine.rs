use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    CanvasRenderingContext2d, Document, Event, HtmlCanvasElement, HtmlVideoElement, MediaStream,
    Window,
};

#[derive(Debug)]
struct Dimensions {
    width: f64,
    height: f64,
}

#[derive(Debug)]
struct RenderingEngineCanvas {
    element: HtmlCanvasElement,
    context_2d: CanvasRenderingContext2d,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct RenderingEngine {
    canvas: Rc<RenderingEngineCanvas>,
    render_targets: Rc<RefCell<Vec<RenderingEngineCanvas>>>,
    cancel: Rc<RefCell<bool>>,
}

#[wasm_bindgen]
impl RenderingEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> RenderingEngine {
        let canvas = Rc::new(RenderingEngine::create_buffer_canvas());
        let render_targets = Rc::new(RefCell::new(Vec::new()));
        let cancel = Rc::new(RefCell::new(false));

        RenderingEngine {
            canvas,
            render_targets,
            cancel,
        }
    }

    #[wasm_bindgen(method)]
    pub fn start(&self, media_stream: &MediaStream) {
        let video = RenderingEngine::create_video_element(media_stream);
        &self.start_animation_loop(&video);
    }

    fn start_animation_loop(&self, video: &Rc<HtmlVideoElement>) {
        let video = video.clone();
        let canvas = self.canvas.clone();
        let cancel = self.cancel.clone();
        let render_targets = self.render_targets.clone();

        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            if *cancel.borrow() == true {
                let _ = f.borrow_mut().take();
                return;
            }

            // Ready state should be >= HAVE_CURRENT_DATA, see: https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/readyState
            if video.ready_state() >= 2 {
                let video_dimensions = Dimensions {
                    width: video.video_width() as f64,
                    height: video.video_height() as f64,
                };
                // draw frame onto buffer canvas
                // perform any pixel manipulation you need on this canvas
                canvas.element.set_width(video_dimensions.width as u32);
                canvas.element.set_height(video_dimensions.height as u32);
                canvas
                    .context_2d
                    .draw_image_with_html_video_element(&video, 0.0, 0.0)
                    .expect("failed to draw video frame to <canvas> element");

                // render resulting image onto target canvas
                for target in render_targets.borrow().iter() {
                    // Use scrollWidth/scrollHeight so we fill the canvas element.
                    let target_dimensions = Dimensions {
                        width: target.element.scroll_width() as f64,
                        height: target.element.scroll_height() as f64,
                    };
                    let scaled_dimensions = RenderingEngine::get_scaled_video_size(
                        &video_dimensions,
                        &target_dimensions,
                    );
                    let offset = Dimensions {
                        width: (target_dimensions.width - scaled_dimensions.width) / 2.0,
                        height: (target_dimensions.height - scaled_dimensions.height) / 2.0,
                    };

                    // Ensure the target canvas has a set width/height, otherwise rendering breaks.
                    target.element.set_width(target_dimensions.width as u32);
                    target.element.set_height(target_dimensions.height as u32);

                    target
                        .context_2d
                        .draw_image_with_html_canvas_element_and_dw_and_dh(
                            &canvas.element,
                            offset.width,
                            offset.height,
                            scaled_dimensions.width,
                            scaled_dimensions.height,
                        )
                        .expect("failed to draw buffer <canvas> to target <canvas>");
                }
            }

            RenderingEngine::request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));
        RenderingEngine::request_animation_frame(g.borrow().as_ref().unwrap());
    }

    #[wasm_bindgen(method)]
    pub fn add_target_canvas(&mut self, canvas: HtmlCanvasElement) {
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .expect("failed to obtain 2d rendering context for target <canvas>");

        let container = RenderingEngineCanvas {
            element: canvas,
            context_2d: context,
        };

        let mut render_targets = self.render_targets.borrow_mut();
        render_targets.push(container);
    }

    #[wasm_bindgen(method)]
    pub fn stop(&mut self) {
        *self.cancel.borrow_mut() = true;
    }

    fn get_scaled_video_size(video: &Dimensions, container: &Dimensions) -> Dimensions {
        let video_landscape = video.width > video.height;
        let container_landscape = container.width > container.height;
        let same_orientation = video_landscape == container_landscape;

        let aspect_ratio = Dimensions {
            width: video.width,
            height: video.height,
        };
        let mut size = Dimensions {
            width: container.width,
            height: container.height,
        };

        if same_orientation {
            let m_w = size.width / aspect_ratio.width;
            let m_h = size.height / aspect_ratio.height;

            if m_h > m_w {
                size.width = (size.height / aspect_ratio.height) * aspect_ratio.width;
            } else if m_w > m_h {
                size.height = (size.width / aspect_ratio.width) * aspect_ratio.height;
            }
        } else {
            let m_w = size.width / aspect_ratio.width;
            let m_h = size.height / aspect_ratio.height;

            if m_h < m_w {
                size.width = (size.height / aspect_ratio.height) * aspect_ratio.width;
            } else if m_w < m_h {
                size.height = (size.width / aspect_ratio.width) * aspect_ratio.height;
            }
        }

        size
    }

    fn create_video_element(media_stream: &MediaStream) -> Rc<HtmlVideoElement> {
        let video = RenderingEngine::get_document()
            .create_element("video")
            .unwrap()
            .dyn_into::<HtmlVideoElement>()
            .expect("failed to create <video> element for MediaStream");

        video.set_autoplay(true);
        video
            .set_attribute("crossOrigin", "anonymous")
            .expect("the attribute crossOrigin must be set to anonymous");
        video.set_src_object(Some(media_stream));

        let video = Rc::new(video);
        RenderingEngine::play_video_onloaded_meta_data(&video);

        video
    }

    fn play_video_onloaded_meta_data(video: &Rc<HtmlVideoElement>) {
        let closure = Closure::wrap(Box::new(move |event: Event| {
            let video_element: HtmlVideoElement = event
                .target()
                .unwrap()
                .dyn_into::<HtmlVideoElement>()
                .expect("could not cast event target to video element");

            &video_element.play();
        }) as Box<dyn FnMut(_)>);

        video
            .add_event_listener_with_callback("loadedmetadata", closure.as_ref().unchecked_ref())
            .expect("failed to add loadedmetadata listener to <video> element");

        closure.forget();
    }

    fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        RenderingEngine::get_window()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");
    }

    fn create_buffer_canvas() -> RenderingEngineCanvas {
        let canvas = RenderingEngine::get_document()
            .create_element("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .expect("failed to create buffer <canvas> element");

        let context_2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .expect("failed to obtain 2d rendering context for buffer <canvas>");

        RenderingEngineCanvas {
            element: canvas,
            context_2d,
        }
    }

    fn get_window() -> Window {
        web_sys::window().unwrap()
    }

    fn get_document() -> Document {
        RenderingEngine::get_window().document().unwrap()
    }
}
