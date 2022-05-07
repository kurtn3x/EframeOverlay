/// We derive Deserialize/Serialize so we can persist app state on shutdown.
/// 
enum RunMode {
    /// This is the default for the demo.
    ///
    /// If this is selected, egui is only updated if are input events
    /// (like mouse movements) or there are some animations in the GUI.
    ///
    /// Reactive mode saves CPU.
    ///
    /// The downside is that the UI can become out-of-date if something it is supposed to monitor changes.
    /// For instance, a GUI for a thermostat need to repaint each time the temperature changes.
    /// To ensure the UI is up to date you need to call `egui::Context::request_repaint()` each
    /// time such an event happens. You can also chose to call `request_repaint()` once every second
    /// or after every single frame - this is called [`Continuous`](RunMode::Continuous) mode,
    /// and for games and interactive tools that need repainting every frame anyway, this should be the default.
    Reactive,

    /// This will call `egui::Context::request_repaint()` at the end of each frame
    /// to request the backend to repaint as soon as possible.
    ///
    /// On most platforms this will mean that egui will run at the display refresh rate of e.g. 60 Hz.
    ///
    /// For this demo it is not any reason to do so except to
    /// demonstrate how quickly egui runs.
    ///
    /// For games or other interactive apps, this is probably what you want to do.
    /// It will guarantee that egui is always up-to-date.
    Continuous,
}
use std::time::Duration;

use egui::{style::Visuals, Pos2, Vec2};


pub struct TemplateApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    value: f32,


    // continous runmode, always true
    runmode_continuous: bool,

    // if cursor events should pass trough the window
    // cursor hittest true: events dont pass trough the window
    // cursor hittest false: events pass trough the window
    // block_cursor_hittest is used for edit mode button, if it is set to true
    // we have control of the window
    cursor_hittest: bool,
    block_cursor_hittest: bool,

    // window handling
    show_window_1 : bool,


    // windows size as vec2 in pixels
    pub window_size : Vec2,

    // window position as pos2 in pixels
    pub window_pos : Pos2,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            runmode_continuous: true,
            cursor_hittest: false,
            block_cursor_hittest: false,
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

    fn toogle_show_window1(&mut self){
        if self.show_window_1{
            self.show_window_1 = false;
        } else {
            self.show_window_1 = true;
        }
    }

    fn toogle_cursor_hittest(&mut self){
        if self.cursor_hittest{
            self.cursor_hittest = false;
        } else {
            self.cursor_hittest = true;
        }
    }

    fn toogle_block(&mut self){
        if self.block_cursor_hittest{
            self.block_cursor_hittest = false;
        } else {
            self.block_cursor_hittest = true;
        }
    }

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
             block_cursor_hittest
         } = self;

        // getting our cursor position
        let temp_cursor: (i32, i32) = Enigo::mouse_location();
        let cursor_location = Pos2{x: (temp_cursor.0 as f32 - window_pos.x),
             y: (temp_cursor.1 as f32 - window_pos.y)};


        // update all the time with a small sleep value to reduce CPU usage
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
        // 
        egui::SidePanel::right("my_left_panel")
        .frame(egui::Frame{
            fill: egui::Color32::TRANSPARENT,
            ..egui::Frame::default()
        })
        .resizable(false)
        .show(ctx, |ui| {
            ui.label("Hello World!");
            let open_butt = ui.button("Open Window");
            let edit_butt = ui.button("Edit Mode");

            if open_butt.rect.contains(cursor_location){
                self.cursor_hittest = true;
                if open_butt.clicked(){
                    self.toogle_show_window1();
                }
            } else if edit_butt.rect.contains(cursor_location){
                self.cursor_hittest = true;
                if edit_butt.clicked(){
                    self.toogle_cursor_hittest();
                    self.toogle_block();
                }
            } else {
                if self.block_cursor_hittest{
                    self.cursor_hittest = true;

                } else {
                    self.cursor_hittest = false;
                }
            }
         });


         if self.show_window_1{
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
         }

    }
}