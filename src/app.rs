use std::time::Duration;

use egui::{style::Visuals, Pos2, Vec2, Style};

enum RunMode {
    Reactive,
    Continuous,
}


pub struct TemplateApp {
    label: String,
    value: f32,

    // continous runmode, always true
    runmode_continuous: bool,

    // if cursor events should pass trough the window
    // cursor hittest true: events dont pass trough the window
    // cursor hittest false: events pass trough the window
    // edit_mode is used for edit mode button, if it is set to true
    // we have control of the window
    cursor_hittest: bool,
    edit_mode : bool,


    // subwindow handling
    show_window_1 : bool,

    // windows size as vec2 in pixels
    pub window_size : Vec2,

    // window position as pos2 in pixels
    pub window_pos : Pos2,
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
            window_pos: Pos2 { x: 0.0, y: 0.0 }
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.


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
    fn update_window(&self, frame: &mut eframe::Frame){
        frame.set_window_pos(self.window_pos);
        frame.set_window_size(self.window_size);

    }
}
use enigo::Enigo;

impl eframe::App for TemplateApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::TRANSPARENT
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self { label, value, runmode_continuous,
             cursor_hittest, show_window_1, window_pos, window_size,
             edit_mode
         } = self;

        // getting our cursor position to check if our cursor is hovering over a specific thing
        let temp_cursor: (i32, i32) = Enigo::mouse_location();
        let cursor_location = Pos2{x: (temp_cursor.0 as f32 - window_pos.x),
             y: (temp_cursor.1 as f32 - window_pos.y)};


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
 

        // the main app
        egui::CentralPanel::default()
        .frame(egui::Frame{
            fill: background_color,
            ..egui::Frame::default()
        })
        .show(ctx, |ui| {

            ui.label("Hello World!");
            let open_butt = ui.button("Open Window");
            let edit_butt = ui.button("Edit Mode");

            if open_butt.rect.contains(cursor_location) {
                self.cursor_hittest = true;
                if open_butt.clicked(){
                    self.toogle_show_window1();
                }
            } else if edit_butt.rect.contains(cursor_location) {
                self.cursor_hittest = true;
                if edit_butt.clicked(){
                    self.toogle_cursor_hittest();
                    self.toogle_edit_mode();
                }
            } else {
                // edit mode on
                if self.edit_mode{
                    self.cursor_hittest = true;

                } 
                // dont capture any inputs
                else {
                    self.cursor_hittest = false;
                }
            }
         });


         if self.show_window_1{
            egui::Window::new("Window")
            .resizable(true)
            .default_pos(Pos2{x: 100.0, y: 100.0})
            .frame(egui::Frame{
                fill: egui::Color32::from_rgba_premultiplied(180, 180, 180, 180),
                ..egui::Frame::default()
            })
            .collapsible(false)
            .show(ctx, |ui| {
                ui.visuals_mut().override_text_color = Some(egui::Color32::BLACK);
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
         }

    }
}