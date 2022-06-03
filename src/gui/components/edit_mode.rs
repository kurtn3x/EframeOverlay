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
        egui::Window::new("Edit Mode Window")
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, Vec2 { x: 0.0, y: -40.0 })
            .frame(egui::Frame {
                fill: egui::Color32::from_rgba_premultiplied(180, 180, 180, 255),
                ..egui::Frame::default()
            })
            .collapsible(false)
            .title_bar(false)
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
                    // .striped(true)
                    .spacing(egui::Vec2 { x: 0.0, y: 5.0 })
                    .show(ui, |ui| {
                    // Row: hotkey_item_inspection
                        ui.add_space(5.0);
                        ui.label("hotkey_item_inspection");
                        ui.add_space(15.0);
                        if ui.add(egui::Button::new("Capture Key")).clicked() {
                            let key = capture_key();
                            app.hotkey_settings.custom_hotkeys.hotkey_item_inspection = key;
                            app.hotkey_settings.reinitialize_hotkeys = true;
                        }
                        ui.add_space(15.0);
                        let str = format!(
                            "{:?}",
                            app.hotkey_settings.custom_hotkeys.hotkey_item_inspection
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
                            app.hotkey_settings.custom_hotkeys.hotkey1 = key;
                            app.hotkey_settings.reinitialize_hotkeys = true;
                        }
                        ui.add_space(15.0);
                        let str = format!("{:?}", app.hotkey_settings.custom_hotkeys.hotkey1);
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
                            app.hotkey_settings.custom_hotkeys.hotkey2 = key;
                            app.hotkey_settings.reinitialize_hotkeys = true;
                        }
                        ui.add_space(15.0);
                        let str = format!("{:?}", app.hotkey_settings.custom_hotkeys.hotkey2);
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
                            app.hotkey_settings.custom_hotkeys.hotkey3 = key;
                            app.hotkey_settings.reinitialize_hotkeys = true;
                        }
                        ui.add_space(15.0);
                        let str = format!("{:?}", app.hotkey_settings.custom_hotkeys.hotkey3);
                        ui.label(str);
                        ui.end_row();
                    });
            });
    }

    fn tab1(ctx: &egui::Context, frame: &eframe::Frame, app: &mut App) {
        egui::Window::new("Edit Mode Window")
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, Vec2 { x: 0.0, y: -40.0 })
            .frame(egui::Frame {
                fill: egui::Color32::from_rgba_premultiplied(180, 180, 180, 255),
                ..egui::Frame::default()
            })
            .collapsible(false)
            .title_bar(false)
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
        egui::Window::new("Edit Mode Window")
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, Vec2 { x: 0.0, y: -40.0 })
            .frame(egui::Frame {
                fill: egui::Color32::from_rgba_premultiplied(180, 180, 180, 255),
                ..egui::Frame::default()
            })
            .collapsible(false)
            .title_bar(false)
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
    fn run(ctx: &egui::Context, frame: &eframe::Frame, app: &mut App) {
        egui::CentralPanel::default()
            .frame(egui::Frame {
                fill: egui::Color32::from_rgba_premultiplied(18, 18, 18, 180),
                ..egui::Frame::default()
            })
            .show(ctx, |ui| {
                let open_butt = ui.add_sized(
                    Vec2 { x: 100.0, y: 50.0 },
                    egui::Button::new("Open Window").fill(egui::Color32::WHITE).sense(egui::Sense::click_and_drag()),
                );
                let edit_butt = ui.add_sized(
                    Vec2 { x: 100.0, y: 50.0 },
                    egui::Button::new("Edit Mode").fill(egui::Color32::WHITE).sense(egui::Sense::click_and_drag()),
                );
                if edit_butt
                    .rect
                    .contains(app.general_settings.cursor_location)
                {
                    app.general_settings.cursor_hittest = true;
                    if edit_butt.clicked() {
                        app.toogle_edit_mode();
                    }
                } 
                // here
                    if open_butt.clicked() {
                        app.toogle_show_window1()
                    } else if open_butt.drag_started(){
                        open_butt.drag_released();
                    } else if open_butt.drag_released(){
                        println!("X");
                    }
                    println!("{:?}", open_butt.drag_delta());

                    // edit mode on
                    if app.edit_mode {
                        app.general_settings.cursor_hittest = true;
                    }
                    // dont capture any inputs
                    else {
                        app.general_settings.cursor_hittest = false;
                    }
                
                let painter = ui.painter();
                let rect = ui.max_rect();
                painter.text(
                    egui::Pos2 {
                        x: rect.center_top().x,
                        y: rect.center_top().y + 15.0,
                    },
                    egui::Align2::CENTER_CENTER,
                    "Edit Mode enabled",
                    egui::FontId {
                        size: 25.0,
                        family: egui::FontFamily::Monospace,
                    },
                    egui::Color32::GREEN,
                );
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
