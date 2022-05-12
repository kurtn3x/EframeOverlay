#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod app;
use app::TemplateApp;
mod main_window;
mod hotkeymanager;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]

fn main() {
    let native_options = eframe::NativeOptions{
        initial_window_pos: Some(TemplateApp::default().general_settings.window_pos),
        initial_window_size : Some(TemplateApp::default().general_settings.window_size),
        resizable:true,
        decorated: false,
        transparent: true,
        always_on_top:true,
        ..eframe::NativeOptions::default()};
    eframe::run_native(
        "RustyPoe",
        native_options,
        Box::new(|cc| Box::new(TemplateApp::new(cc))),
    );
}