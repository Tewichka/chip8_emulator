#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod chip8;
mod disasm;
mod ui;

use app::MyApp; 
use eframe::egui;
use std::env;

fn main() -> Result<(), eframe::Error> {
    let args: Vec<String> = env::args().collect();
    let rom_path = args.get(1).map(|s| s.as_str());

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1024.0, 512.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "CHIP-8 Emulator",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new(rom_path)))),
    )
}