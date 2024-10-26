mod image_lib;

use image::{ImageBuffer, Rgba};
use image_lib::{bytes_to_rgba_image, film_effect, grayscale_filter, original_pixcel_filter};
use std::fs::File;
use std::io::BufReader;

fn main() {
    // Define input and output paths
    const FILE_PATH: &str = "./test-images/a_test.png";
    const GRAY_TEST_PATH: &str = "./test-images/grayscale_test.png";
    const PIXEL_TEST_PATH: &str = "./test-images/pixel_test.png";
    const FILM_TEST_PATH: &str = "./test-images/film_test.png";

    const COMPLEX_COEFFICIENT: f32 = 124.0;
    const MAX_DEPTH: u32 = 10;

    let file = File::open(FILE_PATH).expect("Failed to open input image file");
    let reader = BufReader::new(file);

    // Load and convert the image to RGBA8
    let img = image::load(reader, image::ImageFormat::Jpeg)
        .expect("Failed to load image")
        .to_rgba8();

    // Get image dimensions
    let (width, height) = img.dimensions();

    // Run the grayscale test
    if let Err(e) = grayscale_test(&img, width, height, GRAY_TEST_PATH) {
        eprintln!("Error processing image: {}", e);
    }

    // Run the pixel test
    if let Err(e) = original_pixcel_test(
        &img,
        width,
        height,
        PIXEL_TEST_PATH,
        COMPLEX_COEFFICIENT,
        MAX_DEPTH,
    ) {
        eprintln!("Error processing image: {}", e);
    }

    // Run the film test
    if let Err(e) = film_effect_test(&img, width, height, FILM_TEST_PATH) {
        eprintln!("Error processing image: {}", e);
    }
}

fn film_effect_test(
    img: &[u8],
    width: u32,
    height: u32,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Reconstruct the image and save it
    let img_data = bytes_to_rgba_image(img, width, height);

    let output = film_effect(img_data);

    output
        .save(output_path)
        .expect("Failed to save output image");
    Ok(())
}

fn original_pixcel_test(
    img: &[u8],
    width: u32,
    height: u32,
    output_path: &str,
    complex_coefficient: f32,
    max_depth: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    // Reconstruct the image and save it
    let img_data = bytes_to_rgba_image(img, width, height);

    let output_img = original_pixcel_filter(img_data, complex_coefficient, 0, max_depth);

    output_img
        .save(output_path)
        .expect("Failed to save output image");
    Ok(())
}

fn grayscale_test(
    img: &[u8],
    width: u32,
    height: u32,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Apply grayscale transformation
    let img_data = grayscale_filter(&img, width, height);

    // Reconstruct the image and save it
    let output_img = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, img_data)
        .expect("Failed to create output image buffer");

    output_img
        .save(output_path)
        .expect("Failed to save output image");
    Ok(())
}
