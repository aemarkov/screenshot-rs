mod capture;
mod ui;

use anyhow::Result;
use capture::x11::X11Capture;
use capture::ScreenCapture;

fn main() -> Result<()> {
    let image = X11Capture::all()?;
    ui::show_editor(image);
    Ok(())
}
