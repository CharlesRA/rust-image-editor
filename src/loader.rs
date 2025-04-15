use std::{
    io::{self, Cursor},
    path::Path,
};

use image::DynamicImage;
use reqwest::blocking::get;
use thiserror::Error;
use url::Url;

use crate::args::Args;

#[derive(Debug, Error)]
pub enum LoaderError {
    #[error("HTTP error")]
    Http(#[from] reqwest::Error),

    #[error("Image decoding error")]
    Image(#[from] image::ImageError),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Invalid URL format: {0}")]
    InvalidUrl(String),

    #[error("No input provided (URL or path expected)")]
    NoInput,
}

fn load_from_file(path: &String) -> Result<DynamicImage, LoaderError> {
    Ok(image::open(path)?)
}

pub fn get_image_extension(filename: &String) -> Result<&str, Box<dyn std::error::Error>> {
    let ext = Path::new(filename)
        .extension()
        .ok_or("No file extension found")?
        .to_str()
        .ok_or("Extension is not valid UTF-8")?;

    Ok(ext)
}

fn load_image_from_url(url: &String) -> Result<DynamicImage, LoaderError> {
    Url::parse(url).map_err(|_| LoaderError::InvalidUrl(url.to_string()))?;
    let response = get(url)?;
    Ok(image::ImageReader::new(Cursor::new(response.bytes()?))
        .with_guessed_format()?
        .decode()?)
}

pub fn extract_image(args: &Args) -> Result<DynamicImage, LoaderError> {
    let image = match (&args.url, &args.path) {
        (Some(url), None) => load_image_from_url(url),
        (None, Some(path)) => load_from_file(path),
        _ => Err(LoaderError::NoInput),
    }?;
    Ok(image)
}
