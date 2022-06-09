use super::App;
pub struct SetupWindow {}
use super::hotkeymanager::{Hotkey};
use egui::{style::Visuals, Pos2, Style, Vec2};
use inputbot::KeybdKey;

impl SetupWindow {
    pub fn run(ctx: &egui::Context, frame: &eframe::Frame, app: &mut App) {
        egui::Window::new("Initial Setup")
            .default_pos(Pos2 {
                x: app.general_settings.window_size.x / 2.0,
                y: app.general_settings.window_size.y / 2.0,
            })
            .resizable(true)
            .frame(egui::Frame {
                fill: egui::Color32::from_rgba_premultiplied(255, 255, 255, 255),
                ..egui::Frame::default()
            })
            .collapsible(true)
            .title_bar(true)
            .show(ctx, |ui| {
                // let x = winit::monitor::MonitorHandle {};
                ui.visuals_mut().override_text_color = Some(egui::Color32::BLACK);
                let inspection_window_close_button =
                    ui.add(egui::Button::new("Close").fill(egui::Color32::WHITE));
                if ctx
                    .used_rect()
                    .contains(app.general_settings.cursor_location)
                {
                    app.general_settings.cursor_hittest = true;
                    if inspection_window_close_button.clicked() {
                        app.general_settings.setup = false;
                        app.general_settings.always_on_top = true;
                        app.general_settings.reinitialize = true;
                    }
                } else {
                    app.general_settings.cursor_hittest = false;
                }
            });
    }
}
