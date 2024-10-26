mod image_lib;

use crate::image_lib::{bytes_to_rgba_image, grayscale_filter, original_pixcel_filter};
use image_lib::film_effect;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn grayscale(img_data: &[u8], width: u32, height: u32) -> Vec<u8> {
    return grayscale_filter(img_data, width, height);
}

#[wasm_bindgen]
pub fn original_pixcel(
    img_data: &[u8],
    width: u32,
    height: u32,
    complex: f32,
    max_depth: u32,
) -> Vec<u8> {
    let img = bytes_to_rgba_image(img_data, width, height);
    let filterd_img = original_pixcel_filter(img, complex, 0, max_depth);
    return filterd_img.as_raw().to_vec();
}

#[wasm_bindgen]
pub fn film_effect_1(img_data: &[u8], width: u32, height: u32) -> Vec<u8> {
    let img = bytes_to_rgba_image(img_data, width, height);
    let filterd_img = film_effect(img);
    return filterd_img.as_raw().to_vec();
}
