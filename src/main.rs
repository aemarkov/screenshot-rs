mod capture;
mod image;
mod save;
mod ui;

use anyhow::Result;

fn main() -> Result<()> {
    // let image = capture::x11::X11Capture::all()?;
    // save::save_png(&image, Path::new("/tmp/screenshot.png"))?;
    ui::run();
    Ok(())
}
