use eframe::egui;
use eframe::egui::{Align2, Window};

pub(crate) struct AboutWindow;

impl AboutWindow {
    pub fn new() -> Self {
        Self {}
    }

    pub fn ui(&self, ctx: &egui::Context, open: &mut bool) {
        Window::new("About")
            .collapsible(false)
            .resizable(false)
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .open(open)
            .show(ctx, |ui| {
                egui::Frame::new()
                    .inner_margin(egui::Margin::same(8))
                    .show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading("esim workbench");
                            ui.separator();
                            ui.label("Version 0.1.0");
                        });
                    });
            });
    }
}
