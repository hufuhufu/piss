#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod assets;
mod ocr;
mod scanner;

fn main() -> eframe::Result<()> {
    let item_assets = assets::ItemAssets::new();
    assets::ITEM_ASSETS.set(item_assets).unwrap();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "PISS",
        native_options,
        Box::new(|cc| Box::new(app::App::new(cc))),
    )
}
