use std::io::Cursor;
use std::time::Duration;
extern crate enigo;
use bytes::Bytes;
use eframe::epaint::text::cursor;
use egui::plot::{Plot, PlotUi};
use egui::{style::Visuals, Pos2, Style, Vec2};
use egui::{InnerResponse, Rect, Response};
use enigo::{Enigo, KeyboardControllable, MouseControllable};
extern crate clipboard;
extern crate rev_lines;
use clipboard::ClipboardProvider;
extern crate reqwest;
extern crate input;
use super::super::{App, GeneralSettings, HotkeySettings, ItemInspectionSettings, WidgetSettings, UiSettings};
use super::background_mode::BackgroundMode;
use super::edit_mode::EditMode;
use super::hotkeymanager::{check_hotkeys, Hotkey};
use super::setup::SetupWindow;
use super::AppComponent;
use egui::FontFamily::Proportional;
use egui::FontId;
use egui::TextStyle::*;
use inputbot::KeybdKey;
extern crate windows_win;
extern crate load_image;

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
            x: ((self.general_settings.window_size.x - 1.0) / pixels_per_point),
            y: ((self.general_settings.window_size.y - 1.0) / pixels_per_point),
        });
    }
}

fn load_image_from_memory(image_data: &[u8]) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::load_from_memory(image_data)?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}


impl eframe::App for App {
    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::TRANSPARENT
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self {
            widget_settings,
            show_window_1,
            general_settings,
            edit_mode,
            edit_mode_tab,
            item_inspection_settings,
            hotkey_settings,
            some_val,
        } = self;

        // this will reinitialize all window settings, scale settings etc., will be called at the start, as well as when something gets updated.
        if self.general_settings.reinitialize.global {
            self.general_settings.reinitialize.global = false;


            // set up the window according to the DPI (Zoom in windows display settings) and the display resolution
            if self.general_settings.reinitialize.window_settings{
                self.general_settings.window_size = Vec2 {
                    x: frame.info.current_monitor.size().width as f32,
                    y: frame.info.current_monitor.size().height as f32,
                };
                let pixels_per_point = ctx.pixels_per_point();
                self.update_window(frame, pixels_per_point);
                ctx.set_pixels_per_point(1.0);
                frame.set_always_on_top(self.general_settings.always_on_top);
                self.general_settings.reinitialize.window_settings = false;
            }

            // set global style of the UI
            if self.general_settings.reinitialize.style_settings{
                let mut style = (*ctx.style()).clone();
                style.text_styles = [
                    (Heading, FontId::new(30.0 * self.general_settings.scaling.text_scale, Proportional)),
                    (Name("Heading2".into()), FontId::new(25.0* self.general_settings.scaling.text_scale, Proportional)),
                    (Name("Context".into()), FontId::new(23.0* self.general_settings.scaling.text_scale, Proportional)),
                    (Body, FontId::new(18.0* self.general_settings.scaling.text_scale, Proportional)),
                    (Monospace, FontId::new(14.0* self.general_settings.scaling.text_scale, Proportional)),
                    (Button, FontId::new(20.0* self.general_settings.scaling.text_scale, Proportional)),
                    (Small, FontId::new(10.0* self.general_settings.scaling.text_scale, Proportional)),
                ]
                .into();

                style.spacing.slider_width = 500.0;
                // Mutate global style with above changes
                ctx.set_style(style);
                self.general_settings.reinitialize.style_settings = false;
            }
        }

        // getting our cursor position to check if our cursor is hovering over a specific thing . windows only
        let temp_cursor: (i32, i32) = enigo::Enigo::mouse_location();
        self.general_settings.cursor_location = Pos2 {
            x: (temp_cursor.0 as f32 - self.general_settings.window_pos.x),
            y: (temp_cursor.1 as f32 - self.general_settings.window_pos.y),
        };

        check_hotkeys(self);

        // if true: our window has control of input (as normal),
        // if false: our window lets any input trough to the next window
        if self.general_settings.cursor_hittest {
            frame.set_cursor_hittest(true);
        } else {
            frame.set_cursor_hittest(false);
        }

        // the main panel that covers almost the full screen
        if self.general_settings.setup {
            SetupWindow::run(ctx, frame, self);
        } else if self.edit_mode && !self.general_settings.setup {
            EditMode::run(ctx, frame, self);
        } else if !self.edit_mode && !self.general_settings.setup {
            BackgroundMode::run(ctx, frame, self);
        }

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
                    // if self.textures.test_image.loaded == false {
                    //     self.textures.test_image.loaded = true;
                    //     let texture: &egui::TextureHandle = self.textures.test_image.texture_handle.get_or_insert_with(|| {
                    //         // Load the texture only once.
                    //         ui.ctx().load_texture("my-image", img)
                    //     });
                    //     self.textures.test_image.texture_handle = Some(texture.clone());
                    // }
                    // ui.add(egui::Image::new(&*self.textures.test_image.texture_handle.as_ref().unwrap(), Vec2{x:100.0, y:100.0}));
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
