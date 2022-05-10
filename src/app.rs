use std::time::Duration;
extern crate enigo;
use eframe::epaint::text::cursor;
use egui::{Rect, InnerResponse};
use egui::plot::{Plot, PlotUi};
use enigo::{Enigo, MouseControllable, KeyboardControllable};
use egui::{style::Visuals, Pos2, Vec2, Style};
extern crate rev_lines;
use rev_lines::RevLines;
use std::io::BufReader;
use std::fs::File;
use inputbot;
extern crate clipboard;
use clipboard::ClipboardProvider;
use crate::main_window::MainWindow;
extern crate mki_fork;

pub struct TemplateApp {
    // only runs on first frame or when window is reinitialized
    first_run : bool,

    // if cursor events should pass trough the window
    // cursor hittest true: events dont pass trough the window
    // cursor hittest false: events pass trough the window
    // edit_mode is used for edit mode button, if it is set to true
    // we have control of the window
    pub cursor_hittest: bool,
    pub edit_mode : bool,


    // subwindow handling
    show_window_1 : bool,
    some_window_open: bool,

    // windows size as vec2 in pixels
    pub window_size : Vec2,

    // window position as pos2 in pixels
    pub window_pos : Pos2,


    clipboard_manager: clipboard::ClipboardContext,

    hotkey_item_inspection_pressed : bool,
    hotkey_item_inspection_pressed_initial_position: Pos2,


    some_val : i32,
    hotkey_item_inspection_pressed_first: bool,

    pub edit_mode_tab : Vec<bool>,

    item_info : String,
    current_clipboard: String,

}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            cursor_hittest: false,
            edit_mode: false,
            show_window_1: false,
            window_size: Vec2 { x: 1919.0, y: 1079.0 },
            window_pos: Pos2 { x: 0.0, y: 0.0 },
            some_window_open : false,
            clipboard_manager : ClipboardProvider::new().unwrap(),
            first_run : true,
            hotkey_item_inspection_pressed : false,
            some_val : 0,
            hotkey_item_inspection_pressed_initial_position: Pos2 { x: 0.0, y: 0.0 },
            hotkey_item_inspection_pressed_first: true,
            edit_mode_tab : vec![true, false, false],
            item_info : String::from("NoneItem"),
            current_clipboard: String::from("NoneCurrent"),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    // just decides if the subwindow created is shown or hidden
    pub fn toogle_show_window1(&mut self){
        if self.show_window_1{
            self.show_window_1 = false;
        } else {
            self.show_window_1 = true;
        }
    }

    // if true: our program is in control, used when hovering over buttons for example
    // if false: our window wont be able to receive any inputs
    fn toogle_cursor_hittest(&mut self){
        if self.cursor_hittest{
            self.cursor_hittest = false;
        } else {
            self.cursor_hittest = true;
        }
    }

    // handles the edit mode, if true, our window will regain full control
    // if false, only specific parts programmed to do so will be able to get input
    pub fn toogle_edit_mode(&mut self){
        if self.edit_mode{
            self.edit_mode = false;
        } else {
            self.edit_mode = true;
        }
    }

    fn update_window(&self, frame: &mut eframe::Frame, pixels_per_point:f32){
        frame.set_window_pos(self.window_pos);
        frame.set_window_size(Vec2{x: (self.window_size.x / pixels_per_point), y: (self.window_size.y/ pixels_per_point)});

    }

    fn parse_clipboard(&mut self){
        let  current_clipboard_temp = self.clipboard_manager.get_contents();
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
            self.clipboard_manager.set_contents(self.current_clipboard.clone());
    }
    }
}

impl eframe::App for TemplateApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::TRANSPARENT
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self {
            cursor_hittest, show_window_1, window_pos, window_size,
            edit_mode, some_window_open, clipboard_manager, first_run,
            hotkey_item_inspection_pressed, some_val, hotkey_item_inspection_pressed_initial_position,
            hotkey_item_inspection_pressed_first, edit_mode_tab, item_info, current_clipboard
        } = self;

        // getting our cursor position to check if our cursor is hovering over a specific thing . windows only
        let temp_cursor: (i32, i32) = enigo::Enigo::mouse_location();
        let cursor_location = Pos2{x: (temp_cursor.0 as f32 - window_pos.x), y: (temp_cursor.1 as f32 - window_pos.y)};

        if ctx.input().key_released(egui::Key::Tab){
            println!("{}", self.some_val);
            self.some_val += 1;
        }

        // this will set up the windo and fix dpi errors, idk how this runs some resolutions, only tested 1920x1080
        if self.first_run{
            let pixels_per_point = ctx.pixels_per_point();
            self.update_window(frame,pixels_per_point);
            self.first_run = false;
            ctx.set_pixels_per_point(1.0);
        }
        // mki_fork::register_hotkey(&[mki_fork::Keyboard::CapsLock], || println!("CTRL+B pressed"));


        if inputbot::KeybdKey::CapsLockKey.is_pressed() && self.hotkey_item_inspection_pressed == false {
            self.parse_clipboard();
            self.hotkey_item_inspection_pressed = true;
            self.hotkey_item_inspection_pressed_first = true;

        }


        // if mki_fork::are_pressed(&[mki_fork::Keyboard::CapsLock]){
        //     println!("{}", self.some_val);
        //     self.some_val += 1
        // }

        // if true: our window has control of input (as normal), 
        // if false: our window lets any input trough to the next window
        if self.cursor_hittest == true{
            frame.set_cursor_hittest(true);
        } else {
            frame.set_cursor_hittest(false);
        }
        
        // the main panel that covers almost the full screen

        if self.edit_mode{
            let mut main_window = MainWindow{cursor_location, app: self, ctx, frame};
            main_window.run_edit()
        } else {
            let mut main_window = MainWindow{cursor_location, app: self, ctx, frame};
            main_window.run_background()
        }
        
        // bottom panel
        show_bottom_panel(ctx, frame,cursor_location, self);

        if self.show_window_1{
            egui::Window::new("Some Window")
            .resizable(false)
            .default_pos(Pos2{x: 100.0, y: 100.0})
            .frame(egui::Frame{
                fill: egui::Color32::from_rgba_premultiplied(180, 180, 180, 180),
                ..egui::Frame::default()
            })
            .collapsible(false)
            .show(ctx, |ui| {
                ui.set_min_size(Vec2{x:300.0, y:150.0});
                ui.visuals_mut().override_text_color = Some(egui::Color32::BLACK);
                ui.label("{}", );
            });
        }

        if self.hotkey_item_inspection_pressed{

            self.hotkey_item_inspection_pressed_initial_position = if self.hotkey_item_inspection_pressed_first { cursor_location } else { self.hotkey_item_inspection_pressed_initial_position };
            egui::Window::new("Item Inspection")
            .current_pos(self.hotkey_item_inspection_pressed_initial_position)
            .resizable(true)
            .frame(egui::Frame{
                fill: egui::Color32::from_rgba_premultiplied(180, 180, 180, 180),
                ..egui::Frame::default()
            })
            .collapsible(false)
            .show(ctx, |ui| {
                ui.visuals_mut().override_text_color = Some(egui::Color32::BLACK);
                self.some_val += 1;
                let temp = self.some_val.to_string();
                ui.label(temp);
                let inspection_window_close_button = ui.add(egui::Button::new("Close").fill(egui::Color32::WHITE));
                if inspection_window_close_button.rect.contains(cursor_location) {
                    self.cursor_hittest = true;
                    if inspection_window_close_button.clicked(){
                        self.hotkey_item_inspection_pressed = false;

                    }
                }
            });
            self.hotkey_item_inspection_pressed_first = false;
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
    app: &mut TemplateApp,
) {
    egui::TopBottomPanel::bottom("bottom_panel")
    .frame(egui::Frame{
       fill: egui::Color32::TRANSPARENT,
       ..egui::Frame::default()
    })
    .min_height(100.0)
    .show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
           let quit_button = ui.add_sized(Vec2{x:100.0, y:50.0},egui::Button::new("Quit").fill(egui::Color32::WHITE));
        if quit_button.rect.contains(cursor_location){
            app.cursor_hittest = true;
            if quit_button.clicked() {
                    frame.quit();
            };
        }
        });
   });
}