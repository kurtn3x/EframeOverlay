use super::App;
pub struct SetupWindow {}

impl SetupWindow {
    pub fn run(ctx: &egui::Context, frame: &eframe::Frame, app: &mut App) {
        egui::Window::new("Initial Setup")
            .current_pos(
                app.item_inspection_settings
                    .hotkey_item_inspection_pressed_initial_position,
            )
            .resizable(true)
            .frame(egui::Frame {
                fill: egui::Color32::from_rgba_premultiplied(180, 180, 180, 180),
                ..egui::Frame::default()
            })
            .collapsible(true)
            .title_bar(true)
            .show(ctx, |ui| {
                // let x = winit::monitor::MonitorHandle {};
                ui.visuals_mut().override_text_color = Some(egui::Color32::BLACK);
                let inspection_window_close_button =
                    ui.add(egui::Button::new("Close").fill(egui::Color32::WHITE));
                if inspection_window_close_button
                    .rect
                    .contains(app.general_settings.cursor_location)
                {
                    app.general_settings.cursor_hittest = true;
                    if inspection_window_close_button.clicked() {
                        app.general_settings.setup = false;
                    }
                }
            });
    }
}
