#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use ff_profile_switcher::{get_profiles, open_profile};

fn main() {
    #[cfg(debug_assertions)]
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_pos: Some(egui::pos2(800.0, 660.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Firefox Profile Viewer",
        options,
        Box::new(|_cc| Box::new(FFPViewer::default())),
    )
}

#[derive(Default)]
struct FFPViewer(Vec<String>);

impl eframe::App for FFPViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.0 = get_profiles();
            dbg!(&self.0);
            ui.heading("FFPViewer");
        });
    }
}
