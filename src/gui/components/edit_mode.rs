use egui::{Vec2, RichText};

use super::AppComponent;
use super::super::App;

pub struct EditMode ;
impl EditMode{
    pub fn tab0(ctx: &egui::Context, frame: &eframe::Frame, app: &mut App){
        egui::Window::new("Edit Mode Window")
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, Vec2{x:0.0, y:-40.0})
        .frame(egui::Frame{
            fill: egui::Color32::from_rgba_premultiplied(180, 180, 180, 255),
            ..egui::Frame::default()
        })
        .collapsible(false)
        .title_bar(false)
        .show(ctx, |ui| {
            ui.set_min_size(Vec2{x:850.0, y:900.0});
            ui.visuals_mut().override_text_color = Some(egui::Color32::BLACK);
            egui::TopBottomPanel::top("ww").show_inside(ui, |ui|{
                egui::menu::bar(ui, |ui| {
                    ui.horizontal(|ui| {
                    ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);
                    ui.style_mut().visuals = egui::Visuals{
                        widgets: egui::style::Widgets{..egui::style::Widgets::default()},
                        window_rounding: egui::Rounding::none(),
                        dark_mode : true,
                        button_frame : true,
                        ..egui::Visuals::default()
                    };
                    let tab0 = ui.toggle_value(&mut app.edit_mode_tab[0], "Tab0");
                    let tab1 = ui.toggle_value(&mut app.edit_mode_tab[1], "Tab1");
                    let tab2= ui.toggle_value(&mut app.edit_mode_tab[2], "Tab2");
                    if tab0.clicked(){
                        app.edit_mode_tab = vec![true, false, false];
                    }else if tab1.clicked(){
                        app.edit_mode_tab = vec![false, true, false];
                    }else if tab2.clicked(){
                        app.edit_mode_tab = vec![false, false, true];
                    }
                })
                })

            });
            ui.add_space(20.0);
            ui.horizontal(|ui| {
                ui.add_space(20.0);
                ui.heading("ITEM INSPECTION HOTKEY");
                if ui.add_sized([40.0, 20.0], egui::Button::new("Capture Hotkey....")).clicked(){
                    // app.my_hotkeys.capture_key = true;
                }

                // if app.my_hotkeys.capture_key{
                //     let key = capture_key();
                //     println!("{:?}", key);
                //     self.app.my_hotkeys.capture_key = false;
                // }

                // let str = format!("{}", app.my_hotkeys.hotkey_item_inspection);
                // ui.label(egui::RichText::new(str));
            });
        ui.add(egui::widgets::Separator::default());
        egui::Grid::new("some_unique_id")
        .striped(true)
        .show(ui, |ui| {
                ui.label("Description1");
                // let str = format!("{}", app.my_hotkeys.hotkey_item_inspection);
                // ui.label(str);
                if ui.add_sized([40.0, 20.0], egui::Button::new("Capture Key1")).clicked(){
                    // let key = capture_key();
                    // if self.app.my_hotkeys.hotkey_item_inspection.key.len() == 0{
                    //     self.app.my_hotkeys.hotkey_item_inspection.key.push(key);
                    // } else {
                    //     self.app.my_hotkeys.hotkey_item_inspection.key[0] =  key;
                    // }
                };
                if ui.add_sized([40.0, 20.0], egui::Button::new("Capture Key2")).clicked(){
                    // let key = capture_key();
                    // self.app.my_hotkeys.hotkey_item_inspection.key[1] =  key;
                };
                if ui.add_sized([40.0, 20.0], egui::Button::new("Capture Key3")).clicked(){
                    // let key = capture_key();
                    // self.app.my_hotkeys.hotkey_item_inspection.key[2] =  key;
                };
                ui.end_row();
            
                ui.label("Description2");
                ui.label("KeybindsInfo2");
                ui.label("Keybind1");
                ui.label("Keybind2");
                ui.label("Keybind3");
                ui.end_row();
            
                ui.label("Description3");
                ui.label("KeybindsInfo3");
                ui.label("Keybind1");
                ui.label("Keybind2");
                ui.label("Keybind3");
                ui.end_row();
            });
        });

    }

    fn tab1(ctx: &egui::Context, frame: &eframe::Frame, app: &mut App){
        egui::Window::new("Edit Mode Window")
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, Vec2{x:0.0, y:-40.0})
                .frame(egui::Frame{
                    fill: egui::Color32::from_rgba_premultiplied(180, 180, 180, 255),
                    ..egui::Frame::default()
                })
                .collapsible(false)
                .title_bar(false)
                .show(ctx, |ui| {
                    ui.set_min_size(Vec2{x:850.0, y:900.0});
                    ui.visuals_mut().override_text_color = Some(egui::Color32::BLACK);
                    egui::TopBottomPanel::top("ww").show_inside(ui, |ui|{
                        egui::menu::bar(ui, |ui| {
                            ui.horizontal(|ui| {
                            ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);
                            ui.style_mut().visuals = egui::Visuals{
                                widgets: egui::style::Widgets{..egui::style::Widgets::default()},
                                window_rounding: egui::Rounding::none(),
                                dark_mode : false,
                                button_frame : true,
                                ..egui::Visuals::default()
                            };
                            let tab0 = ui.toggle_value(&mut app.edit_mode_tab[0], "Tab0");
                            let tab1 = ui.toggle_value(&mut app.edit_mode_tab[1], "Tab1");
                            let tab2= ui.toggle_value(&mut app.edit_mode_tab[2], "Tab2");
                            if tab0.clicked(){
                                app.edit_mode_tab = vec![true, false, false];
                            }else if tab1.clicked(){
                                app.edit_mode_tab = vec![false, true, false];
                            }else if tab2.clicked(){
                                app.edit_mode_tab = vec![false, false, true];
                            }
                            })
                        })
                    });
                    ui.label("THIS IS TAB1");
                });

    }

    fn tab2(ctx: &egui::Context, frame: &eframe::Frame, app: &mut App){
        egui::Window::new("Edit Mode Window")
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, Vec2{x:0.0, y:-40.0})
        .frame(egui::Frame{
            fill: egui::Color32::from_rgba_premultiplied(180, 180, 180, 255),
            ..egui::Frame::default()
        })
        .collapsible(false)
        .title_bar(false)
        .show(ctx, |ui| {
            ui.set_min_size(Vec2{x:850.0, y:900.0});
            ui.visuals_mut().override_text_color = Some(egui::Color32::BLACK);
            egui::TopBottomPanel::top("ww").show_inside(ui, |ui|{
                egui::menu::bar(ui, |ui| {
                    ui.horizontal(|ui| {
                    ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);
                    ui.style_mut().visuals = egui::Visuals{
                        widgets: egui::style::Widgets{..egui::style::Widgets::default()},
                        window_rounding: egui::Rounding::none(),
                        dark_mode : false,
                        button_frame : true,
                        ..egui::Visuals::default()
                    };
                    let tab0 = ui.toggle_value(&mut app.edit_mode_tab[0], "Tab0");
                    let tab1 = ui.toggle_value(&mut app.edit_mode_tab[1], "Tab1");
                    let tab2= ui.toggle_value(&mut app.edit_mode_tab[2], "Tab2");
                    if tab0.clicked(){
                        app.edit_mode_tab = vec![true, false, false];
                    }else if tab1.clicked(){
                        app.edit_mode_tab = vec![false, true, false];
                    }else if tab2.clicked(){
                        app.edit_mode_tab = vec![false, false, true];
                    }
                })
                })
            });
            ui.label("THIS IS TAB2");
        });

    }

}

impl AppComponent for EditMode{
    fn add(ctx: &egui::Context, frame: &eframe::Frame, app: &mut App) {
        egui::CentralPanel::default()
        .frame(egui::Frame{
            fill: egui::Color32::from_rgba_premultiplied(18, 18, 18, 180),
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
            let painter = ui.painter();
            let rect = ui.max_rect();
            painter.text(
                egui::Pos2{x: rect.center_top().x ,y: rect.center_top().y + 15.0},
                egui::Align2::CENTER_CENTER,
                "Edit Mode enabled",
                egui::FontId{size: 25.0, family: egui::FontFamily::Monospace},
                egui::Color32::GREEN,
            );
            if app.edit_mode_tab[0] {
                Self::tab0(ctx, frame, app);
            } else  if app.edit_mode_tab[1] {
                Self::tab1(ctx, frame, app);
            } else  if app.edit_mode_tab[2] {
                Self::tab2(ctx, frame, app);
            } 

            ;
        });


    }


}