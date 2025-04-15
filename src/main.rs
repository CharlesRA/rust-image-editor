mod actions;
mod args;
mod delivery;
mod loader;

use actions::resize_image;
use args::Args;
use clap::Parser;
use delivery::execute_delivery;
use loader::{extract_final_file_format, extract_image};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut image = extract_image(&args)?;
    let file_format = extract_final_file_format(&args)?;

    image = resize_image(&args, image)?;
    execute_delivery(&args, image, file_format)?;

    Ok(())
}
