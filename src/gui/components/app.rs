use std::time::Duration;
extern crate enigo;
use eframe::epaint::text::cursor;
use egui::plot::{Plot, PlotUi};
use egui::{style::Visuals, Pos2, Style, Vec2};
use egui::{InnerResponse, Rect};
use enigo::{Enigo, KeyboardControllable, MouseControllable};
extern crate clipboard;
extern crate rev_lines;
use clipboard::ClipboardProvider;
extern crate input;
use super::super::{App, GeneralSettings, ItemInspectionSettings, MyHotkeys};
use super::background_mode::BackgroundMode;
use super::edit_mode::EditMode;
use super::hotkeymanager::{check_hotkeys, Hotkey};
use super::setup::SetupWindow;
use super::AppComponent;

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    // just decides if the subwindow created is shown or hidden
    pub fn toogle_show_window1(&mut self) {
        if self.show_window_1 {
            self.show_window_1 = false;
        } else {
            self.show_window_1 = true;
        }
    }

    // if true: our program is in control, used when hovering over buttons for example
    // if false: our window wont be able to receive any inputs
    fn toogle_cursor_hittest(&mut self) {
        if self.general_settings.cursor_hittest {
            self.general_settings.cursor_hittest = false;
        } else {
            self.general_settings.cursor_hittest = true;
        }
    }

    // handles the edit mode, if true, our window will regain full control
    // if false, only specific parts programmed to do so will be able to get input
    pub fn toogle_edit_mode(&mut self) {
        if self.edit_mode {
            self.edit_mode = false;
        } else {
            self.edit_mode = true;
        }
    }

    fn update_window(&self, frame: &mut eframe::Frame, pixels_per_point: f32) {
        frame.set_window_pos(self.general_settings.window_pos);
        frame.set_window_size(Vec2 {
            x: (self.general_settings.window_size.x / pixels_per_point),
            y: (self.general_settings.window_size.y / pixels_per_point),
        });
    }

    fn parse_clipboard(&mut self) {
        let current_clipboard_temp = self.clipboard_manager.get_contents();
        let current_clipboard = match current_clipboard_temp {
            Ok(clipboard_data) => clipboard_data,
            Err(error) => String::from("NoneCurrent"),
        };
        println!("Self.current_clipboard {}", self.current_clipboard);
        println!("self.item_info {}", self.item_info);
        println!("current_clipboard {}", current_clipboard);

        if current_clipboard != self.item_info || current_clipboard == self.current_clipboard {
            let mut enigo = Enigo::new();
            enigo.key_down(enigo::Key::Control);
            enigo.key_down(enigo::Key::Layout('c'));
            enigo.key_up(enigo::Key::Control);
            enigo.key_up(enigo::Key::Layout('c'));
            std::thread::sleep(Duration::from_millis(10));

            let later_clipboard_temp = self.clipboard_manager.get_contents();
            let later_clipboard = match later_clipboard_temp {
                Ok(clipboard_data) => clipboard_data,
                Err(error) => String::from("NoneItem"),
            };
            self.current_clipboard = current_clipboard;
            self.item_info = later_clipboard;
            println!("later_clipboard {}", self.item_info);
            self.clipboard_manager
                .set_contents(self.current_clipboard.clone());
        }
    }
}

impl eframe::App for App {
    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::TRANSPARENT
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self {
            show_window_1,
            general_settings,
            edit_mode,
            some_window_open,
            clipboard_manager,
            some_val,
            edit_mode_tab,
            item_info,
            current_clipboard,
            some_option,
            item_inspection_settings,
            my_hotkeys,
        } = self;
        // getting our cursor position to check if our cursor is hovering over a specific thing . windows only
        let temp_cursor: (i32, i32) = enigo::Enigo::mouse_location();
        self.general_settings.cursor_location = Pos2 {
            x: (temp_cursor.0 as f32 - self.general_settings.window_pos.x),
            y: (temp_cursor.1 as f32 - self.general_settings.window_pos.y),
        };
        println!("{:?}", frame.info.current_monitor.size());

        // frame.get_resolution();
        // this will set up the windo and fix dpi errors as well as setup hotkeys,
        // idk how this runs some resolutions, only tested 1920x1080
        // can also be reset to reinitialize windows / hotkeys
        if self.general_settings.first_run {
            let pixels_per_point = ctx.pixels_per_point();
            self.general_settings.window_size = Vec2 { x: 0.0, y: 0.0 };
            self.update_window(frame, pixels_per_point);
            self.general_settings.first_run = false;
            ctx.set_pixels_per_point(1.0);
            // self.my_hotkeys.all_hotkeys.push(self.my_hotkeys.hotkey_item_inspection);

            // let hotkey_with_2_keys = Hotkey::new(vec![inputbot::KeybdKey::CapsLockKey, inputbot::KeybdKey::TabKey], "first_hotkey");
            // self.my_hotkeys.all_hotkeys.push(hotkey_with_2_keys);
            let hotkey_with_1_key = Hotkey::new(
                vec![inputbot::KeybdKey::LControlKey],
                "hotkey_item_inspection",
            );
            self.my_hotkeys.all_hotkeys.push(hotkey_with_1_key);
        }

        if self.my_hotkeys.reinizialize_hotkeys {
            for hotkeys in self.my_hotkeys.all_hotkeys.iter_mut() {
                if hotkeys.identifier == "hotkey_item_inspection" {
                    hotkeys.key = self.my_hotkeys.hotkey_item_inspection.clone();
                }
            }
            self.my_hotkeys.reinizialize_hotkeys = false;
        }

        check_hotkeys(self);

        // if true: our window has control of input (as normal),
        // if false: our window lets any input trough to the next window
        if self.general_settings.cursor_hittest == true {
            frame.set_cursor_hittest(true);
        } else {
            frame.set_cursor_hittest(false);
        }

        // the main panel that covers almost the full screen

        if self.general_settings.setup {
            SetupWindow::run(ctx, frame, self);
        } else if self.edit_mode && !self.general_settings.setup {
            EditMode::run(ctx, frame, self);
            show_bottom_panel(ctx, frame, self.general_settings.cursor_location, self);
        } else if !self.edit_mode && !self.general_settings.setup {
            BackgroundMode::run(ctx, frame, self);
            show_bottom_panel(ctx, frame, self.general_settings.cursor_location, self);
        }
        // bottom panel
        if self.show_window_1 {
            egui::Window::new("Some Window")
                .resizable(false)
                .default_pos(Pos2 { x: 100.0, y: 100.0 })
                .frame(egui::Frame {
                    fill: egui::Color32::from_rgba_premultiplied(180, 180, 180, 180),
                    ..egui::Frame::default()
                })
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.set_min_size(Vec2 { x: 300.0, y: 150.0 });
                    ui.visuals_mut().override_text_color = Some(egui::Color32::BLACK);
                    ui.label("{}");
                });
        }

        if self.item_inspection_settings.hotkey_item_inspection_pressed {
            show_item_inspection_window(ctx, frame, self.general_settings.cursor_location, self)
        }

        // update our window all the time with a small sleep value to reduce CPU usage
        ctx.request_repaint();
        std::thread::sleep(Duration::from_millis(1));
    }
}

fn show_bottom_panel(
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    cursor_location: Pos2,
    app: &mut App,
) {
    egui::TopBottomPanel::bottom("bottom_panel")
        .frame(egui::Frame {
            fill: egui::Color32::TRANSPARENT,
            ..egui::Frame::default()
        })
        // .min_height(100.0)
        .show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let quit_button = ui.add_sized(
                    Vec2 { x: 100.0, y: 50.0 },
                    egui::Button::new("Quit").fill(egui::Color32::WHITE),
                );
                if quit_button.rect.contains(cursor_location) {
                    app.general_settings.cursor_hittest = true;
                    if quit_button.clicked() {
                        frame.quit();
                    };
                }
            });
        });
}

fn show_item_inspection_window(
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    cursor_location: Pos2,
    app: &mut App,
) {
    app.item_inspection_settings
        .hotkey_item_inspection_pressed_initial_position = if app
        .item_inspection_settings
        .hotkey_item_inspection_pressed_first
    {
        cursor_location
    } else {
        app.item_inspection_settings
            .hotkey_item_inspection_pressed_initial_position
    };
    egui::Window::new("Item Inspection")
        .current_pos(
            app.item_inspection_settings
                .hotkey_item_inspection_pressed_initial_position,
        )
        .resizable(false)
        .frame(egui::Frame {
            fill: egui::Color32::from_rgba_premultiplied(180, 180, 180, 180),
            ..egui::Frame::default()
        })
        .collapsible(false)
        .show(ctx, |ui| {
            ui.visuals_mut().override_text_color = Some(egui::Color32::BLACK);
            app.some_val += 1;
            let temp = app.some_val.to_string();
            ui.label(temp);
            let inspection_window_close_button =
                ui.add(egui::Button::new("Close").fill(egui::Color32::WHITE));
            if inspection_window_close_button
                .rect
                .contains(cursor_location)
            {
                app.general_settings.cursor_hittest = true;
                if inspection_window_close_button.clicked() {
                    app.item_inspection_settings.hotkey_item_inspection_pressed = false;
                }
            }
        });
    app.item_inspection_settings
        .hotkey_item_inspection_pressed_first = false;
}
