use egui::{Vec2, RichText};

use super::AppComponent;
use super::super::App;

pub struct BackgroundMode ;

impl BackgroundMode{

}

impl AppComponent for BackgroundMode{
    fn add(ctx: &egui::Context, frame: &eframe::Frame, app: &mut App) {
        egui::CentralPanel::default()
        .frame(egui::Frame{
            fill: egui::Color32::TRANSPARENT,
            ..egui::Frame::default()
        })
        .show(ctx, |ui| {
            let open_butt = ui.add_sized(Vec2{x: 100.0, y: 50.0},egui::Button::new(RichText::new("Open Window").size(16.0)).fill(egui::Color32::WHITE));
            let edit_butt = ui.add_sized(Vec2{x: 100.0, y: 50.0},egui::Button::new(RichText::new("Edit Mode").size(16.0)).fill(egui::Color32::WHITE));

            if edit_butt.rect.contains(app.general_settings.cursor_location) {
                app.general_settings.cursor_hittest = true;
                if edit_butt.clicked(){
                    app.toogle_edit_mode();
                }
            } else if open_butt.rect.contains(app.general_settings.cursor_location)  {
                app.general_settings.cursor_hittest = true;
                if open_butt.clicked(){
                    app.toogle_show_window1();
                }
            } else {
                // edit mode on
                if app.edit_mode{
                    app.general_settings.cursor_hittest = true;
                } 
                // dont capture any inputs
                else {
                    app.general_settings.cursor_hittest = false;
                }
            }
        }
    );


    }



}