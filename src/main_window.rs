use crate::TemplateApp;
use egui::{Pos2, Vec2, Widget, RichText};
pub struct MainWindow <'a>{
    pub cursor_location:  Pos2,
    pub app: &'a mut TemplateApp,
    pub ctx: &'a  egui::Context,
    pub frame: &'a mut eframe::Frame,
}

impl MainWindow <'_>{
    pub fn run_edit(&mut self){
        egui::CentralPanel::default()
        .frame(egui::Frame{
            fill: egui::Color32::from_rgba_premultiplied(18, 18, 18, 180),
            ..egui::Frame::default()
        })
        .show(self.ctx, |ui| {
            let open_butt = ui.add_sized(Vec2{x: 100.0, y: 50.0},egui::Button::new(RichText::new("Open Window").size(16.0)).fill(egui::Color32::WHITE));
            let edit_butt = ui.add_sized(Vec2{x: 100.0, y: 50.0},egui::Button::new(RichText::new("Edit Mode").size(16.0)).fill(egui::Color32::WHITE));
            if edit_butt.rect.contains(self.cursor_location) {
                self.app.cursor_hittest = true;
                if edit_butt.clicked(){
                    self.app.toogle_edit_mode();
                }
            } else if open_butt.rect.contains(self.cursor_location)  {
                self.app.cursor_hittest = true;
                if open_butt.clicked(){
                    self.app.toogle_show_window1();
                }
            } else {
                // edit mode on
                if self.app.edit_mode{
                    self.app.cursor_hittest = true;
                } 
                // dont capture any inputs
                else {
                    self.app.cursor_hittest = false;
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
            if self.app.edit_mode_tab[0] {
                self.tab0_run();
            } else  if self.app.edit_mode_tab[1] {
                self.tab1_run();
            } else  if self.app.edit_mode_tab[2] {
                self.tab2_run();
            } 

            ;
        });

    }

    // TAB 0 EDIT MODE
    fn tab0_run(&mut self){
        egui::Window::new("Edit Mode Window")
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, Vec2{x:0.0, y:-40.0})
        .frame(egui::Frame{
            fill: egui::Color32::from_rgba_premultiplied(180, 180, 180, 255),
            ..egui::Frame::default()
        })
        .collapsible(false)
        .title_bar(false)
        .show(self.ctx, |ui| {
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
                    let tab0 = ui.toggle_value(&mut self.app.edit_mode_tab[0], "Tab0");
                    let tab1 = ui.toggle_value(&mut self.app.edit_mode_tab[1], "Tab1");
                    let tab2= ui.toggle_value(&mut self.app.edit_mode_tab[2], "Tab2");
                    if tab0.clicked(){
                        self.app.edit_mode_tab = vec![true, false, false];
                    }else if tab1.clicked(){
                        self.app.edit_mode_tab = vec![false, true, false];
                    }else if tab2.clicked(){
                        self.app.edit_mode_tab = vec![false, false, true];
                    }
                })
                })

            });
            ui.add_space(20.0);
            ui.horizontal(|ui| {
                ui.add_space(20.0);
                ui.heading("THIS IS TAB0");
                let mut selected = String::from("lol");
                egui::ComboBox::from_label("Select one!")
                .selected_text(format!("{:?}", selected))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut selected, String::from("1"), "First");
                    ui.selectable_value(&mut selected, String::from("2"), "Second");
                    ui.selectable_value(&mut selected, String::from("3"), "Third");
                });
                println!("{}", selected);
            });
        });

    }

    // TAB 1 EDIT MODE
    fn tab1_run(&mut self){
        egui::Window::new("Edit Mode Window")
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, Vec2{x:0.0, y:-40.0})
                .frame(egui::Frame{
                    fill: egui::Color32::from_rgba_premultiplied(180, 180, 180, 255),
                    ..egui::Frame::default()
                })
                .collapsible(false)
                .title_bar(false)
                .show(self.ctx, |ui| {
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
                            let tab0 = ui.toggle_value(&mut self.app.edit_mode_tab[0], "Tab0");
                            let tab1 = ui.toggle_value(&mut self.app.edit_mode_tab[1], "Tab1");
                            let tab2= ui.toggle_value(&mut self.app.edit_mode_tab[2], "Tab2");
                            if tab0.clicked(){
                                self.app.edit_mode_tab = vec![true, false, false];
                            }else if tab1.clicked(){
                                self.app.edit_mode_tab = vec![false, true, false];
                            }else if tab2.clicked(){
                                self.app.edit_mode_tab = vec![false, false, true];
                            }
                            })
                        })
                    });
                    ui.label("THIS IS TAB1");
                });
    }

    // TAB 2 EDIT MODE
    fn tab2_run(&mut self){
        egui::Window::new("Edit Mode Window")
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, Vec2{x:0.0, y:-40.0})
        .frame(egui::Frame{
            fill: egui::Color32::from_rgba_premultiplied(180, 180, 180, 255),
            ..egui::Frame::default()
        })
        .collapsible(false)
        .title_bar(false)
        .show(self.ctx, |ui| {
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
                    let tab0 = ui.toggle_value(&mut self.app.edit_mode_tab[0], "Tab0");
                    let tab1 = ui.toggle_value(&mut self.app.edit_mode_tab[1], "Tab1");
                    let tab2= ui.toggle_value(&mut self.app.edit_mode_tab[2], "Tab2");
                    if tab0.clicked(){
                        self.app.edit_mode_tab = vec![true, false, false];
                    }else if tab1.clicked(){
                        self.app.edit_mode_tab = vec![false, true, false];
                    }else if tab2.clicked(){
                        self.app.edit_mode_tab = vec![false, false, true];
                    }
                })
                })
            });
            ui.label("THIS IS TAB2");
        });
    }

    //////////////////////////////////////////////////////
    pub fn run_background(&mut self){
        egui::CentralPanel::default()
        .frame(egui::Frame{
            fill: egui::Color32::TRANSPARENT,
            ..egui::Frame::default()
        })
        .show(self.ctx, |ui| {
            let open_butt = ui.add_sized(Vec2{x: 100.0, y: 50.0},egui::Button::new(RichText::new("Open Window").size(16.0)).fill(egui::Color32::WHITE));
            let edit_butt = ui.add_sized(Vec2{x: 100.0, y: 50.0},egui::Button::new(RichText::new("Edit Mode").size(16.0)).fill(egui::Color32::WHITE));

            if edit_butt.rect.contains(self.cursor_location) {
                self.app.cursor_hittest = true;
                if edit_butt.clicked(){
                    self.app.toogle_edit_mode();
                }
            } else if open_butt.rect.contains(self.cursor_location)  {
                self.app.cursor_hittest = true;
                if open_butt.clicked(){
                    self.app.toogle_show_window1();
                }
            } else {
                // edit mode on
                if self.app.edit_mode{
                    self.app.cursor_hittest = true;
                } 
                // dont capture any inputs
                else {
                    self.app.cursor_hittest = false;
                }
            }
        }
    );
    }
}
