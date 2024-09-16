use std::collections::HashMap;

use clap::{Parser, Subcommand};
use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};
use rayon::iter::{ParallelBridge, ParallelIterator};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    style: Style,

    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
enum Style {
    Oli {
        #[arg(short, long, default_value_t = 5)]
        window: u32,
        #[arg(long, default_value_t = false)]
        with_weight: bool,
    },
}

fn main() {
    let args = Args::parse();
    let img = image::ImageReader::open(args.input)
        .unwrap()
        .decode()
        .unwrap();
    let (width, height) = img.dimensions();
    let mut output_img = RgbaImage::new(width, height);
    match args.style {
        Style::Oli {
            window,
            with_weight,
        } => {
            if with_weight {
                output_img
                    .enumerate_pixels_mut()
                    .par_bridge()
                    .for_each(|(x, y, pixel)| {
                        *pixel = get_dominant_color_in_window_with_weight(
                            &img, x, y, width, height, window,
                        );
                    })
            } else {
                output_img
                    .enumerate_pixels_mut()
                    .par_bridge()
                    .for_each(|(x, y, pixel)| {
                        *pixel = get_dominant_color_in_window(&img, x, y, width, height, window);
                    })
            }
        }
    };
    output_img.save(args.output).unwrap();
}

fn get_dominant_color_in_window_with_weight(
    img: &DynamicImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    window: u32,
) -> Rgba<u8> {
    let mut r_sum = 0.0;
    let mut g_sum = 0.0;
    let mut b_sum = 0.0;
    let mut a_sum = 0.0;
    let mut weight_sum = 0.0;
    let center_pixel = img.get_pixel(x, y).0;
    for offset_x in 0..window {
        for offset_y in 0..window {
            let px = x + offset_x;
            let py = y + offset_y;
            if px < width && py < height {
                let current_pixel = img.get_pixel(px, py).0;
                let weight = 1.0 / (1.0 + color_distance(center_pixel, current_pixel));
                r_sum += current_pixel[0] as f64 * weight;
                g_sum += current_pixel[1] as f64 * weight;
                b_sum += current_pixel[2] as f64 * weight;
                a_sum += current_pixel[3] as f64 * weight;
                weight_sum += weight;
            }
        }
    }
    Rgba([
        (r_sum / weight_sum) as u8,
        (g_sum / weight_sum) as u8,
        (b_sum / weight_sum) as u8,
        (a_sum / weight_sum) as u8,
    ])
}

fn get_dominant_color_in_window(
    img: &DynamicImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    window: u32,
) -> Rgba<u8> {
    let mut color_frequency = HashMap::new();
    for offset_x in 0..window {
        for offset_y in 0..window {
            let px = x + offset_x;
            let py = y + offset_y;
            if px < width && py < height {
                let pixel = img.get_pixel(px, py).0;
                *color_frequency.entry(pixel).or_insert(0) += 1;
            }
        }
    }
    color_frequency
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(color, _)| Rgba(color))
        .unwrap_or(Rgba([0, 0, 0, 255]))
}

fn color_distance(c1: [u8; 4], c2: [u8; 4]) -> f64 {
    let r_diff = (c1[0] as f64 - c2[0] as f64).powi(2);
    let g_diff = (c1[1] as f64 - c2[1] as f64).powi(2);
    let b_diff = (c1[2] as f64 - c2[1] as f64).powi(2);
    let a_diff = (c1[3] as f64 - c2[3] as f64).powi(2);
    (r_diff + g_diff + b_diff + a_diff).sqrt()
}
