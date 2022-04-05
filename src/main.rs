#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Font Rendering Bug Example",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

#[derive(Default)]
struct MyApp;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.colored_label(egui::Rgba::from_gray(1.0), "white");
            ui.colored_label(egui::Rgba::from_gray(0.5), "gray");
            ui.colored_label(egui::Rgba::from_gray(0.0), "black");
        });
        // Resize the native window to be just the size we need it to be:
        frame.set_window_size(ctx.used_size());
    }
}
