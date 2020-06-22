use std::error::Error;

extern crate rustface;

use rustface::{Detector, FaceInfo, ImageData};

pub fn detect_face(bytes: *const u8, width: u32, height: u32) {
    let mut detector = rustface::create_detector("seeta_fd_frontal_v1.0").unwrap();
    detector.set_min_face_size(20);
    detector.set_score_thresh(2.0);
    detector.set_pyramid_scale_factor(0.8);
    detector.set_slide_window_step(4, 4);

    let mut image = ImageData::new(bytes, width, height);
    for face in detector.detect(&mut image).into_iter() {
        // print confidence score and coordinates
        println!("found face: {:?}", face);
    }
}
