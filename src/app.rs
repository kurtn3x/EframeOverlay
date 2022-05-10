use std::time::Duration;
extern crate enigo;
use eframe::epaint::text::cursor;
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




enum RunMode {
    Reactive,
    Continuous,
}


pub struct TemplateApp {
    label: String,
    value: f32,

    // continous runmode, always true
    runmode_continuous: bool,

    // only runs for 1. frame
    first_run : bool,

    // if cursor events should pass trough the window
    // cursor hittest true: events dont pass trough the window
    // cursor hittest false: events pass trough the window
    // edit_mode is used for edit mode button, if it is set to true
    // we have control of the window
    cursor_hittest: bool,
    edit_mode : bool,


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

}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            label: "Hello World!".to_owned(),
            value: 2.7,
            runmode_continuous: true,
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

        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    // just decides if the subwindow created is shown or hidden
    fn toogle_show_window1(&mut self){
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
    fn toogle_edit_mode(&mut self){
        if self.edit_mode{
            self.edit_mode = false;
        } else {
            self.edit_mode = true;
        }
    }

    // no use for now
    fn update_window(&self, frame: &mut eframe::Frame, pixels_per_point:f32){
        frame.set_window_pos(self.window_pos);
        // println!("{}", (self.window_size.x / pixels_per_point));
        frame.set_window_size(Vec2{x: (self.window_size.x / pixels_per_point), y: (self.window_size.y/ pixels_per_point)});
        // frame.set_window_size(self.window_size);

    }

    fn parse_clipboard(&mut self) -> String{
        let  current_clipboard_temp = self.clipboard_manager.get_contents();
        let current_clipboard = match current_clipboard_temp {
            Ok(clipboard_data) => clipboard_data,
            Err(error) => String::from("None"),
        };
        let mut enigo = Enigo::new();
        enigo.key_down(enigo::Key::Control);
        enigo.key_down(enigo::Key::Layout('c'));
        enigo.key_up(enigo::Key::Control);
        enigo.key_up(enigo::Key::Layout('c'));
        current_clipboard
    }
}

impl eframe::App for TemplateApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::TRANSPARENT
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self { label, value, runmode_continuous,
             cursor_hittest, show_window_1, window_pos, window_size,
             edit_mode, some_window_open, clipboard_manager, first_run
             , hotkey_item_inspection_pressed, some_val, hotkey_item_inspection_pressed_initial_position,
             hotkey_item_inspection_pressed_first
         } = self;

         ctx.debug_painter();

        // getting our cursor position to check if our cursor is hovering over a specific thing

        //only works for windows
        // let temp_cursor: (i32, i32) = (0,0);
        let temp_cursor: (i32, i32) = enigo::Enigo::mouse_location();
        let cursor_location = Pos2{x: (temp_cursor.0 as f32 - window_pos.x),
            y: (temp_cursor.1 as f32 - window_pos.y)};
        
        if self.first_run{
            let pixels_per_point = ctx.pixels_per_point();
            self.update_window(frame,pixels_per_point);
            self.first_run = false;
            ctx.set_pixels_per_point(1.0);
        }

        if inputbot::KeybdKey::CapsLockKey.is_pressed() && self.hotkey_item_inspection_pressed == false {
            println!("WASD");
            let current_clipboard = self.parse_clipboard();
            self.hotkey_item_inspection_pressed = true;
            self.hotkey_item_inspection_pressed_first = true;
        }


        // update our window all the time with a small sleep value to reduce CPU usage 

        if self.runmode_continuous == true {
            ctx.request_repaint();
            std::thread::sleep(Duration::from_millis(1));
        }

        // if true: our window has control of input (as normal), 
        // if false: our window lets any input trough to the next window
        if self.cursor_hittest == true{
            frame.set_cursor_hittest(true);
        } else {
            frame.set_cursor_hittest(false);
        }

        // set slightly darker background color in edit mode so we know we are in it
        let background_color = if self. edit_mode { egui::Color32::from_rgba_premultiplied(18, 18, 18, 180)} 
            else { egui::Color32::TRANSPARENT };
        
        // the main panel that covers almost the full screen
        show_central_panel(ctx, frame, background_color, cursor_location, self);
        
        // bottom panel
        show_bottom_panel(ctx, frame,cursor_location, self);

        if self.show_window_1{
            egui::Window::new("Window2")
            .resizable(true)
            .default_pos(Pos2{x: 100.0, y: 100.0})
            .frame(egui::Frame{
                fill: egui::Color32::from_rgba_premultiplied(180, 180, 180, 180),
                ..egui::Frame::default()
            })
            .collapsible(false)
            .show(ctx, |ui| {
                ui.visuals_mut().override_text_color = Some(egui::Color32::BLACK);
                ui.label("{}", );
                
            });
        }

        if self.hotkey_item_inspection_pressed{
            self.hotkey_item_inspection_pressed_initial_position = if self.hotkey_item_inspection_pressed_first { cursor_location } else { self.hotkey_item_inspection_pressed_initial_position };
            egui::Window::new("Window")
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
            // if inputbot::KeybdKey::CapsLockKey.is_toggled() && self.hotkey_item_inspection_pressed == true {
            //     self.hotkey_item_inspection_pressed = false;
            // }
         }

    }
}

fn show_central_panel(
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    background_color: egui::Color32,
    cursor_location: Pos2,
    app: &mut TemplateApp,
) {
    egui::CentralPanel::default()
    .frame(egui::Frame{
        fill: background_color,
        ..egui::Frame::default()
    })
    .show(ctx, |ui| {
        let open_butt = ui.add(egui::Button::new("Open Window").fill(egui::Color32::WHITE));
        // println!("EDITLOC {:?}", ui.next_widget_position());
        let edit_butt = ui.add(egui::Button::new("Edit Mode").fill(egui::Color32::WHITE));


        if edit_butt.rect.contains(cursor_location) {
            app.cursor_hittest = true;
            if edit_butt.clicked(){
                app.toogle_edit_mode();
            }
            

        } else if open_butt.rect.contains(cursor_location)  {
            app.cursor_hittest = true;
            if open_butt.clicked(){
                app.toogle_show_window1();
            }
        } else {
            // edit mode on
            if app.edit_mode{
                app.cursor_hittest = true;
            } 
            // dont capture any inputs
            else {
                app.cursor_hittest = false;
            }
        }
     });
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