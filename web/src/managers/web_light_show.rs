use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

#[wasm_bindgen]
pub struct WebLightShow {
    video: web_sys::HtmlVideoElement,
}

#[wasm_bindgen]
impl WebLightShow {
    #[wasm_bindgen(constructor)]
    pub fn new(video_id: &str) -> WebLightShow {
        log!("Creating new WebLightShow");
        let doc: Document = WebLightShow::get_document();
        let video: HtmlVideoElement = WebLightShow::get_video(&doc, video_id);

        WebLightShow { video }
    }

    #[wasm_bindgen(method)]
    pub fn test(&self) {
        log!("Hello");
    }

    fn get_document() -> Document {
        let doc = web_sys::window().unwrap().document().unwrap();

        doc
    }

    fn get_video(d: &Document, id: &str) -> web_sys::HtmlVideoElement {
        let element = d
            .get_element_by_id(id)
            .unwrap()
            .dyn_into::<web_sys::HtmlVideoElement>()
            .map_err(|_| ())
            .unwrap();

        element
    }
}
