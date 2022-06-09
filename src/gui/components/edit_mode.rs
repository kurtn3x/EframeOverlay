use egui::{RichText, Vec2};

use super::super::App;
use super::hotkeymanager::{capture_key, Hotkey};
use super::AppComponent;
use bytes;
use image;
use inputbot::KeybdKey;
use strum::IntoEnumIterator;
use winapi::{
    ctypes::*,
    shared::{minwindef::*, windef::*},
    um::winuser::*,
};

#[derive(Debug)]
struct CaptureKeys {
    keys: Vec<KeybdKey>,
    keystate: Vec<bool>,
}

async fn get_bytes(url: &str) -> reqwest::Result<bytes::Bytes> {
    let img_bytes = reqwest::blocking::get("...")?.bytes()?;
    return Ok(img_bytes);
}

async fn download_stuff(target: &str) -> Result<bytes::Bytes, reqwest::Error> {
    let response = reqwest::get(target).await?;
    let content = response.bytes().await?;
    return Ok(content);
}

pub struct EditMode;
impl EditMode {
    pub fn tab0(ctx: &egui::Context, frame: &eframe::Frame, app: &mut App) {
        egui::Window::new("SETTINGS")
            .resizable(false)
            // .anchor(egui::Align2::CENTER_CENTER, Vec2 { x: 0.0, y: -40.0 })
            .frame(egui::Frame {
                fill: egui::Color32::from_rgba_premultiplied(245, 245, 245, 255),
                ..egui::Frame::default()
            })
            .collapsible(true)
            .title_bar(true)
            .resizable(false)
            .default_pos(egui::Pos2{x: egui::Pos2::default().x + 250.0 , y: egui::Pos2::default().y + 100.0})
            .show(ctx, |ui| {
                ui.set_min_size(Vec2 { x: 850.0, y: 900.0 });
                ui.visuals_mut().override_text_color = Some(egui::Color32::BLACK);
                egui::TopBottomPanel::top("ww").show_inside(ui, |ui| {
                    egui::menu::bar(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);
                            ui.style_mut().visuals = egui::Visuals {
                                widgets: egui::style::Widgets {
                                    ..egui::style::Widgets::default()
                                },
                                window_rounding: egui::Rounding::none(),
                                dark_mode: true,
                                button_frame: true,
                                ..egui::Visuals::default()
                            };
                            let tab0 = ui.toggle_value(&mut app.edit_mode_tab[0], "Tab0");
                            let tab1 = ui.toggle_value(&mut app.edit_mode_tab[1], "Tab1");
                            let tab2 = ui.toggle_value(&mut app.edit_mode_tab[2], "Tab2");
                            if tab0.clicked() {
                                app.edit_mode_tab = vec![true, false, false];
                            } else if tab1.clicked() {
                                app.edit_mode_tab = vec![false, true, false];
                            } else if tab2.clicked() {
                                app.edit_mode_tab = vec![false, false, true];
                            }
                        })
                    })
                });
                ui.heading("HOTKEY SETTINGS");
                ui.add(egui::widgets::Separator::default());
                egui::Grid::new("some_unique_id")
                    .spacing(egui::Vec2 { x: 0.0, y: 5.0 })
                    .show(ui, |ui| {
                    // Row: hotkey_item_inspection
                        ui.add_space(5.0);
                        ui.label("hotkey_item_inspection");
                        ui.add_space(15.0);
                        if ui.add(egui::Button::new("Capture Key")).clicked() {
                            let key = capture_key();
                            app.hotkey_settings.all_hotkeys.hotkey_item_inspection.update(key);

                        }
                        ui.add_space(15.0);
                        let str = format!(
                            "{:?}",
                            app.hotkey_settings.all_hotkeys.hotkey_item_inspection.key
                        );
                        ui.label(str);
                        ui.end_row();

                    // Row: Hotkey1
                        ui.add_space(5.0);
                        ui.label("Hotkey1");
                        ui.add_space(15.0);
                        if ui
                            .add_sized([40.0, 20.0], egui::Button::new("Capture Key"))
                            .clicked()
                        {
                            let key = capture_key();
                            app.hotkey_settings.all_hotkeys.hotkey1.update(key);

                        }
                        ui.add_space(15.0);
                        let str = format!("{:?}", app.hotkey_settings.all_hotkeys.hotkey1.key);
                        ui.label(str);
                        ui.end_row();

                    // Row: Hotkey2 
                        ui.add_space(5.0);
                        ui.label("Hotkey2");
                        ui.add_space(15.0);
                        if ui
                            .add_sized([40.0, 20.0], egui::Button::new("Capture Key"))
                            .clicked()
                        {
                            let key = capture_key();
                            app.hotkey_settings.all_hotkeys.hotkey2.update(key);

                        }
                        ui.add_space(15.0);
                        let str = format!("{:?}", app.hotkey_settings.all_hotkeys.hotkey2.key);
                        ui.label(str);
                        ui.end_row();

                    // Row: Hotkey3
                        ui.add_space(5.0);
                        ui.label("Hotkey3");
                        ui.add_space(15.0);
                        if ui
                            .add_sized([40.0, 20.0], egui::Button::new("Capture Key"))
                            .clicked()
                        {
                            let key = capture_key();
                            app.hotkey_settings.all_hotkeys.hotkey3.update(key);

                        }
                        ui.add_space(15.0);
                        let str = format!("{:?}", app.hotkey_settings.all_hotkeys.hotkey3.key);
                        ui.label(str);
                        ui.end_row();
                    });
            });
    }

    fn tab1(ctx: &egui::Context, frame: &eframe::Frame, app: &mut App) {
        egui::Window::new("SETTINGS")
        .resizable(false)
        // .anchor(egui::Align2::CENTER_CENTER, Vec2 { x: 0.0, y: -40.0 })
        .frame(egui::Frame {
            fill: egui::Color32::from_rgba_premultiplied(245, 245, 245, 255),
            ..egui::Frame::default()
        })
        .collapsible(true)
        .title_bar(true)
        .resizable(false)
        .default_pos(egui::Pos2{x: egui::Pos2::default().x + 250.0 , y: egui::Pos2::default().y + 100.0})
        .show(ctx, |ui| {
            ui.set_min_size(Vec2 { x: 850.0, y: 900.0 });
            ui.visuals_mut().override_text_color = Some(egui::Color32::BLACK);
            egui::TopBottomPanel::top("ww").show_inside(ui, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.horizontal(|ui| {
                            ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);
                            ui.style_mut().visuals = egui::Visuals {
                                widgets: egui::style::Widgets {
                                    ..egui::style::Widgets::default()
                                },
                                window_rounding: egui::Rounding::none(),
                                dark_mode: false,
                                button_frame: true,
                                ..egui::Visuals::default()
                            };
                            let tab0 = ui.toggle_value(&mut app.edit_mode_tab[0], "Tab0");
                            let tab1 = ui.toggle_value(&mut app.edit_mode_tab[1], "Tab1");
                            let tab2 = ui.toggle_value(&mut app.edit_mode_tab[2], "Tab2");
                            if tab0.clicked() {
                                app.edit_mode_tab = vec![true, false, false];
                            } else if tab1.clicked() {
                                app.edit_mode_tab = vec![false, true, false];
                            } else if tab2.clicked() {
                                app.edit_mode_tab = vec![false, false, true];
                            }
                        })
                    })
                    
                });
                ui.label("THIS IS TAB1");
            });
    }

    fn tab2(ctx: &egui::Context, frame: &eframe::Frame, app: &mut App) {
        let x = egui::Window::new("SETTINGS")
        .resizable(false)
        // .anchor(egui::Align2::CENTER_CENTER, Vec2 { x: 0.0, y: -40.0 })
        .frame(egui::Frame {
            fill: egui::Color32::from_rgba_premultiplied(245, 245, 245, 255),
            ..egui::Frame::default()
        })
        .collapsible(true)
        .title_bar(true)
        .resizable(false)
        .default_pos(egui::Pos2{x: egui::Pos2::default().x + 250.0 , y: egui::Pos2::default().y + 100.0})
        .show(ctx, |ui| {
            ui.set_min_size(Vec2 { x: 850.0, y: 900.0 });
            ui.visuals_mut().override_text_color = Some(egui::Color32::BLACK);
            egui::TopBottomPanel::top("ww").show_inside(ui, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.horizontal(|ui| {
                            ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);
                            ui.style_mut().visuals = egui::Visuals {
                                widgets: egui::style::Widgets {
                                    ..egui::style::Widgets::default()
                                },
                                window_rounding: egui::Rounding::none(),
                                dark_mode: false,
                                button_frame: true,
                                ..egui::Visuals::default()
                            };
                            let tab0 = ui.toggle_value(&mut app.edit_mode_tab[0], "Tab0");
                            let tab1 = ui.toggle_value(&mut app.edit_mode_tab[1], "Tab1");
                            let tab2 = ui.toggle_value(&mut app.edit_mode_tab[2], "Tab2");
                            if tab0.clicked() {
                                app.edit_mode_tab = vec![true, false, false];
                            } else if tab1.clicked() {
                                app.edit_mode_tab = vec![false, true, false];
                            } else if tab2.clicked() {
                                app.edit_mode_tab = vec![false, false, true];
                            }
                        })
                    })
                });
                ui.label("THIS IS TAB2");
            });
    }
}

impl AppComponent for EditMode {
    fn run(ctx: &egui::Context, frame: &mut eframe::Frame, app: &mut App) {
        egui::CentralPanel::default()
            .frame(egui::Frame {
                fill: egui::Color32::from_rgba_premultiplied(25, 25, 25, 40),
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
                    app.widget_settings.edit_button.color = egui::Color32::WHITE;
                    app.widget_settings.quit_button.color = egui::Color32::RED;
                    if edit_butt.clicked() {
                        app.toogle_edit_mode();
                    } 
                } else if edit_butt.dragged(){
                    app.widget_settings.edit_button.position = egui::Rect{min: egui::Pos2{x:app.widget_settings.edit_button.position.min.x + edit_butt.drag_delta().x
                        , y:app.widget_settings.edit_button.position.min.y + edit_butt.drag_delta().y
                    } , max: egui::Pos2 { x: app.widget_settings.edit_button.position.max.x + edit_butt.drag_delta().x
                        , y: app.widget_settings.edit_button.position.max.y + edit_butt.drag_delta().y}};
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
                } else if quit_button.dragged(){
                    app.widget_settings.quit_button.position = egui::Rect{min: egui::Pos2{x:app.widget_settings.quit_button.position.min.x + quit_button.drag_delta().x
                        , y:app.widget_settings.quit_button.position.min.y + quit_button.drag_delta().y
                    } , max: egui::Pos2 { x: app.widget_settings.quit_button.position.max.x + quit_button.drag_delta().x
                        , y: app.widget_settings.quit_button.position.max.y + quit_button.drag_delta().y}};

                }else {
                    // edit mode on
     
                        app.general_settings.cursor_hittest = true;
                        app.widget_settings.quit_button.color = egui::Color32::RED;
                        app.widget_settings.edit_button.color = egui::Color32::LIGHT_GRAY;

                    
                }
                
                if app.edit_mode_tab[0] {
                    Self::tab0(ctx, frame, app);
                } else if app.edit_mode_tab[1] {
                    Self::tab1(ctx, frame, app);
                } else if app.edit_mode_tab[2] {
                    Self::tab2(ctx, frame, app);
                };
            });
    }
}
