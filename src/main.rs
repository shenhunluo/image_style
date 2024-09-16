use clap::{Parser, Subcommand};
use image::{GenericImageView, RgbaImage};
use oli::{get_dominant_color_in_window, get_dominant_color_in_window_with_weight};
use rayon::iter::{ParallelBridge, ParallelIterator};
use indicatif::ParallelProgressIterator;
mod oli;

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
    let progress_bar = indicatif::ProgressBar::new(width as u64 * height as u64);
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
                    .progress_with(progress_bar)
                    .for_each(|(x, y, pixel)| {
                        *pixel = get_dominant_color_in_window_with_weight(
                            &img, x, y, width, height, window,
                        );
                    })
            } else {
                output_img
                    .enumerate_pixels_mut()
                    .par_bridge()
                    .progress_with(progress_bar)
                    .for_each(|(x, y, pixel)| {
                        *pixel = get_dominant_color_in_window(&img, x, y, width, height, window);
                    })
            }
        }
    };
    output_img.save(args.output).unwrap();
}