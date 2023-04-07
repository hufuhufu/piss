#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod ocr;
mod scanner;
mod app;

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "pqh/images/items/"]
struct ItemAssets;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "PISS",
        native_options,
        Box::new(|cc| Box::new(app::App::new(cc))),
    )
}
