mod window;

use eframe::egui;
use image::RgbaImage;

/// Run the GUI
/// Displays captured screenshot `image` in an editor window
pub fn show_editor(image: RgbaImage) {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Screenshot editor",
        native_options,
        Box::new(|cc| {
            let style = egui::Style {
                visuals: egui::Visuals::light(),
                ..egui::Style::default()
            };
            cc.egui_ctx.set_style(style);
            Ok(Box::new(window::ScreenshotEditor::new(cc, image)))
        }),
    );
}
