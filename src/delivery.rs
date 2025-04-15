use crate::args::Args;
use crate::loader::get_image_extension;

use image::{DynamicImage, ExtendedColorType, ImageFormat, codecs::jpeg::JpegEncoder};
use std::io::{self, Cursor, Write};

fn stdout_buffer(image: DynamicImage) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Cursor::new(Vec::new());
    let rgb = image.to_rgb8();
    JpegEncoder::new_with_quality(&mut buffer, 100).encode(
        &rgb,
        rgb.width(),
        rgb.height(),
        ExtendedColorType::Rgb8,
    )?;
    let mut handle = io::stdout().lock();
    handle.write_all(&buffer.into_inner())?;
    handle.flush()?;
    Ok(())
}

fn save_file(image: DynamicImage, filename: &String) -> Result<(), Box<dyn std::error::Error>> {
    let extension = get_image_extension(filename)?;
    match ImageFormat::from_extension(extension) {
        Some(format) => Ok(image.save_with_format(filename, format)?),
        None => Err("Unable to determine image format".into()),
    }
}

pub fn execute_delivery(
    args: &Args,
    image: DynamicImage,
) -> Result<(), Box<dyn std::error::Error>> {
    if args.buffer {
        Ok(stdout_buffer(image)?)
    } else if let Some(output) = &args.output {
        Ok(save_file(image, output)?)
    } else {
        Err("Unable to determine image delivery".into())
    }
}
