#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod gui;

use gui::App;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]

fn main() {
    let native_options = eframe::NativeOptions {
        initial_window_pos: Some(App::default().general_settings.window_pos),
        initial_window_size: Some(App::default().general_settings.window_size),
        resizable: true,
        decorated: false,
        transparent: true,
        always_on_top: true,
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(
        "RustyPoe",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    );
}
