mod capture;
mod image;
mod save;

use anyhow::Result;
use capture::ScreenCapture;
use std::path::Path;

fn main() -> Result<()> {
    let image = capture::x11::X11Capture::all()?;
    save::save_png(&image, Path::new("/tmp/screenshot.png"))?;
    Ok(())
}
