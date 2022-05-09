#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod app;
use app::TemplateApp;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]

fn main() {
    use egui::Pos2;

    let native_options = eframe::NativeOptions{
        initial_window_pos: Some(TemplateApp::default().window_pos),
        initial_window_size : Some(TemplateApp::default().window_size),
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

// use hotkey;

// fn main() {
//     let mut hk = hotkey::Listener::new();
//     hk.register_hotkey(
//         hotkey::modifiers::CONTROL | hotkey::modifiers::SHIFT,
//         'A' as u32,
//         || println!("Ctrl-Shift-A pressed!"),
//     )
//     .unwrap();

//     hk.listen();
// }
