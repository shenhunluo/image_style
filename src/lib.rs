use std::io::Cursor;

use image::{EncodableLayout, ExtendedColorType, GenericImageView, ImageFormat, RgbaImage};
use oli::get_dominant_color_in_window;
use rayon::iter::{ParallelBridge, ParallelIterator};

pub mod oli;

pub fn get_oli_style(data: &[u8], window: u32) -> Vec<u8> {
    let img = image::ImageReader::new(Cursor::new(data))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();
    let (width, height) = img.dimensions();
    let mut output_img = RgbaImage::new(width, height);
    output_img
        .enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(x, y, pixel)| {
            *pixel = get_dominant_color_in_window(&img, x, y, width, height, window);
        });
    let mut writer = Cursor::new(Vec::new());
    image::write_buffer_with_format(&mut writer, output_img.as_bytes(), width, height, ExtendedColorType::Rgba8, ImageFormat::Png).unwrap();
    writer.into_inner()
}