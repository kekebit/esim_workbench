mod state;
mod widgets;

use crate::state::*;
use crate::widgets::*;
use eframe::egui::{self, *};

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };
    eframe::run_native(
        "Image Viewer",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<MyApp>::default())
        }),
    )
}

#[derive(Default)]
struct MyApp {
    app_state: AppState,
    layout_state: LayoutState,
    img_viewer: ImageViewer,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        AboutWindow::new().ui(ctx, &mut self.app_state.show_about);
        egui::TopBottomPanel::top("top_panel")
            .default_height(32.)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
                    if egui::Button::new("load img").ui(ui).clicked() {
                        if let Some(img_file_path) = rfd::FileDialog::new()
                            .add_filter("select image files", &["png", "jpg", "jpeg", "webp"])
                            .pick_file()
                        {
                            let picked_img_path = img_file_path.display().to_string();
                            self.img_viewer.load(ctx, &picked_img_path);
                        }
                    }
                    if egui::Button::new("about").ui(ui).clicked() {
                        self.app_state.show_about = true;
                    }
                })
            });

        if self.layout_state.show_left_side {
            egui::SidePanel::left("left_panel")
                .resizable(true)
                .default_width(150.0)
                .width_range(120.0..=200.0)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Left Panel");
                    });
                });
        }
        if self.layout_state.show_right_side {
            egui::SidePanel::right("right_panel")
                .resizable(true)
                .default_width(150.0)
                .width_range(80.0..=200.0)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Right Panel");
                    });
                });
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            self.img_viewer.ui(ui);
        });
    }
}
