use egui::{RichText, Vec2, Pos2};

use super::super::App;
use super::AppComponent;

pub struct BackgroundMode;

impl BackgroundMode {}

impl AppComponent for BackgroundMode {
    fn run(ctx: &egui::Context, frame: &mut eframe::Frame, app: &mut App) {
        egui::CentralPanel::default()
            .frame(egui::Frame {
                fill: egui::Color32::TRANSPARENT,
                ..egui::Frame::default()
            })
            .show(ctx, |ui| {
                let open_butt = ui.add_sized(
                    Vec2 { x: 100.0, y: 50.0 },
                    egui::Button::new("Open Window").fill(egui::Color32::WHITE),
                );
                let edit_butt = ui.put(
                    app.widget_settings.edit_button.position,
                    egui::Button::new(RichText::new("⚙")
                    .size(app.widget_settings.edit_button.size)
                    .color(app.widget_settings.edit_button.color)).frame(false).sense(egui::Sense::click_and_drag()),
                );

                let quit_button = ui.put(
                    app.widget_settings.quit_button.position,
                    egui::Button::new(RichText::new("❌")
                    .color(app.widget_settings.quit_button.color)
                    .size(app.widget_settings.quit_button.size)).frame(false).sense(egui::Sense::click_and_drag()),
                );
                if edit_butt
                    .rect
                    .contains(app.general_settings.cursor_location)
                {
                    app.general_settings.cursor_hittest = true;
                    app.widget_settings.edit_button.color = egui::Color32::WHITE;
                    app.widget_settings.quit_button.color = egui::Color32::RED;
                    if edit_butt.clicked() {
                        app.toogle_edit_mode();
                    } 
                } else if quit_button
                .rect
                .contains(app.general_settings.cursor_location)
                {
                    app.general_settings.cursor_hittest = true;
                    app.widget_settings.quit_button.color = egui::Color32::LIGHT_RED;
                    app.widget_settings.edit_button.color = egui::Color32::LIGHT_GRAY;
                    if quit_button.clicked(){
                        frame.quit()
                    }

                }else if open_butt
                    .rect
                    .contains(app.general_settings.cursor_location)
                {
                    app.general_settings.cursor_hittest = true;
                    if open_butt.clicked() {
                        app.toogle_show_window1();
                    }
                } else {

                        app.general_settings.cursor_hittest = false;
                        app.widget_settings.quit_button.color = egui::Color32::RED;
                        app.widget_settings.edit_button.color = egui::Color32::LIGHT_GRAY;

                    
                }
            });
    }
}
