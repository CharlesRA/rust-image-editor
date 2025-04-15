use image::DynamicImage;

use crate::args::Args;

pub fn resize_image(
    args: &Args,
    image: DynamicImage,
) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    if args.resize {
        if args.width.is_none() && args.height.is_none() {
            return Err("New height or width parameters are missing".into());
        }
        Ok(image.resize(
            args.width.unwrap_or(image.width()),
            args.height.unwrap_or(image.height()),
            image::imageops::FilterType::Lanczos3,
        ))
    } else {
        Ok(image)
    }
}
