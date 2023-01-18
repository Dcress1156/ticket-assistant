#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]

fn main() {

    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();
    
    let _options = eframe::NativeOptions {

    decorated: true, 
    transparent: false,
    always_on_top: false,
    vsync: true,
    centered: true,
    resizable: true,
    min_window_size: Some(egui::vec2(800.0, 535.0)),
    initial_window_size: Some(egui::vec2(800.0, 535.0)),
    ..Default::default()
    };

    eframe::run_native(
        "Email Hinter",
        _options,
        Box::new(|cc| Box::new(email_hinter::Ehinter::new(cc))),
    );
}

