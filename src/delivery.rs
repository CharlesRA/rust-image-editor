use crate::args::Args;

use image::ImageEncoder;
use image::codecs::bmp::BmpEncoder;
use image::codecs::gif::GifEncoder;
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::PngEncoder;
use image::{DynamicImage, ExtendedColorType, ImageFormat};
use std::io::{self, Write};

fn stdout_buffer(
    image: DynamicImage,
    file_format: ImageFormat,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut handle = io::stdout().lock();

    match file_format {
        ImageFormat::Png => {
            let encoder = PngEncoder::new(&mut handle);
            let rgba = image.to_rgb8();
            encoder.write_image(&rgba, rgba.width(), rgba.height(), ExtendedColorType::Rgb8)?;
        }
        ImageFormat::Jpeg => {
            let encoder = JpegEncoder::new(&mut handle);
            let rgb = image.to_rgba8();
            encoder.write_image(&rgb, rgb.width(), rgb.height(), ExtendedColorType::Rgba8)?;
        }
        ImageFormat::Bmp => {
            let encoder = BmpEncoder::new(&mut handle);
            let rgb = image.to_rgba8();
            encoder.write_image(&rgb, rgb.width(), rgb.height(), ExtendedColorType::Rgba8)?;
        }
        ImageFormat::Gif => {
            let mut encoder = GifEncoder::new(&mut handle);
            let rgba = image.to_rgba8();
            encoder.encode(&rgba, rgba.width(), rgba.height(), ExtendedColorType::Rgba8)?;
        }
        unsupported_format => {
            return Err(format!("Unsupported format {:?}", unsupported_format).into());
        }
    };

    Ok(())
}

fn save_file(
    image: DynamicImage,
    filename: &String,
    file_format: ImageFormat,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(image.save_with_format(filename, file_format)?)
}

pub fn execute_delivery(
    args: &Args,
    image: DynamicImage,
    file_format: ImageFormat,
) -> Result<(), Box<dyn std::error::Error>> {
    if args.buffer {
        Ok(stdout_buffer(image, file_format)?)
    } else if let Some(output) = &args.output {
        Ok(save_file(image, output, file_format)?)
    } else {
        Err("Unable to determine image delivery".into())
    }
}
