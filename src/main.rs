mod app;
mod fetch;
mod model;
mod time;

use app::ScheduleApp;
use eframe::egui;
use std::fs;

fn main() -> eframe::Result<()> {
    let icon = load_icon();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_title("PB Schedule Viewer")
            .with_icon(icon),
        ..Default::default()
    };

    eframe::run_native(
        "PB Schedule Viewer",
        options,
        Box::new(|_cc| Box::new(ScheduleApp::default())),
    )
}

fn load_icon() -> egui::IconData {
    let image = image::load_from_memory(include_bytes!("../assets/icon.png"))
        .expect("Failed to load icon")
        .into_rgba8();

    let (width, height) = image.dimensions();
    let rgba = image.into_raw();

    egui::IconData {
        rgba,
        width,
        height,
    }
}

