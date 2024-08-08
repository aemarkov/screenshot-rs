//! Utilities for saving files

use crate::image::Image;
use anyhow::Result;
use png::{BitDepth, ColorType, Encoder};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

/// Save image to the png file
pub fn save_png(image: &Image, path: &Path) -> Result<()> {
    let file = File::create(path)?;
    let w = BufWriter::new(file);

    let mut encoder = Encoder::new(w, image.size.width, image.size.height);
    encoder.set_color(ColorType::Rgba);
    encoder.set_depth(BitDepth::Eight);

    let mut writer = encoder.write_header()?;
    writer.write_image_data(&image.data)?;

    Ok(())
}
