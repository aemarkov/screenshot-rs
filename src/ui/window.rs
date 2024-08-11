//! Main application window

use eframe::egui::{self, Layout, TextureHandle, Ui};
use image::RgbaImage;

#[derive(Default)]
pub struct ScreenshotEditor {
    image: RgbaImage,
    texture: Option<TextureHandle>,
}

impl ScreenshotEditor {
    pub fn new(_: &eframe::CreationContext<'_>, image: RgbaImage) -> Self {
        Self {
            image,
            texture: None,
        }
    }
}

impl eframe::App for ScreenshotEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let texture = self.texture.get_or_insert_with(|| {
            // Texture should be loaded once

            // NOTE: maybe it's better to create own dumb container for pixel data rather than
            // converting &[u8] -> Vec -> Image -> back into &[8]
            let size = [self.image.width() as usize, self.image.height() as usize];
            let pixels = self.image.as_flat_samples();
            let image = egui::ColorImage::from_rgba_unmultiplied(
                size,
                pixels.as_slice(),
            );

            ctx.load_texture(
                "screenshot-texture",
                image,
                Default::default())
        });

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

        egui::CentralPanel::default()
        .frame(egui::Frame::none())
        .show(ctx, |ui| {
            // we load texture at first update() call, so for now it should be loaded
            ui.image((texture.id(), texture.size_vec2()));
        });
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
