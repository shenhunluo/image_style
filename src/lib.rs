use std::{io::Cursor, sync::atomic::AtomicUsize};

use image::{EncodableLayout, ExtendedColorType, GenericImageView, ImageFormat, RgbaImage};
use oli::get_dominant_color_in_window;
use rayon::iter::{ParallelBridge, ParallelIterator};

pub mod oli;

pub fn get_oli_style<F>(data: &[u8], window: u32, f: F) -> Vec<u8> where F: Fn(f32) + Sync {
    let img = image::ImageReader::new(Cursor::new(data))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();
    let (width, height) = img.dimensions();
    let mut output_img = RgbaImage::new(width, height);
    let i = AtomicUsize::new(0);
    output_img
        .enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(x, y, pixel)| {
            let i = i.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
            f(i as f32 / (width * height) as f32);
            *pixel = get_dominant_color_in_window(&img, x, y, width, height, window);
        });
    let mut writer = Cursor::new(Vec::new());
    image::write_buffer_with_format(&mut writer, output_img.as_bytes(), width, height, ExtendedColorType::Rgba8, ImageFormat::Png).unwrap();
    writer.into_inner()
}