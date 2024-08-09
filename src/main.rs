mod capture;
mod image;
mod save;

use anyhow::Result;
use eframe::egui::{self, Layout, Ui};

fn main() -> Result<()> {
    // let image = capture::x11::X11Capture::all()?;
    // save::save_png(&image, Path::new("/tmp/screenshot.png"))?;

    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| {
            let style = egui::Style {
                visuals: egui::Visuals::light(),
                ..egui::Style::default()
            };
            cc.egui_ctx.set_style(style);
            Ok(Box::new(ScreenshotEditor::new(cc)))
        }),
    );

    Ok(())
}

#[derive(Default)]
struct ScreenshotEditor {}

impl ScreenshotEditor {
    fn new(_: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for ScreenshotEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left")
            .resizable(false)
            .default_width(80.0)
            .show(ctx, |ui| {
                UiHelpers::justified(ui, |ui| {
                    ui.separator();
                    ui.label("Tools");
                    ui.separator();
                    let _ = ui.button("Line");
                    let _ = ui.button("Arrow");
                    let _ = ui.button("Rect");
                    let _ = ui.button("Circle");
                    let _ = ui.button("Text");

                    ui.separator();
                    ui.label("Actions");
                    ui.separator();
                    let _ = ui.button("Copy");
                    let _ = ui.button("Save");
                    let _ = ui.button("Exit");
                });
            });

        egui::CentralPanel::default().show(ctx, |_ui| {});
    }
}

struct UiHelpers;
impl UiHelpers {
    /// Creates a centered justified layout
    fn justified<R>(
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> egui::InnerResponse<R> {
        ui.with_layout(
            Layout {
                main_justify: false,
                cross_justify: true,
                cross_align: egui::Align::Center,
                ..Default::default()
            },
            add_contents,
        )
    }
}
