use self::components::hotkeymanager::Hotkey;
use bytes::Bytes;
use clipboard::ClipboardProvider;
use egui::{Pos2, Vec2, Ui, ColorImage};
use image::ImageError;
use inputbot::KeybdKey;
mod components;

pub struct Texture{
    pub texture_handle: Option<egui::TextureHandle>,
    pub memory_location: Option<egui::TextureHandle>,
    pub loaded: bool,
}

pub struct Textures{
    pub test_image: Texture,

}

pub struct ItemInspectionSettings {
    pub hotkey_item_inspection_pressed: bool,
    pub hotkey_item_inspection_pressed_initial_position: Pos2,
    pub hotkey_item_inspection_pressed_first: bool,
}

#[derive(Debug)]
pub struct CustomHotkeys {
    pub hotkey_item_inspection: Hotkey<'static>,
    pub hotkey1: Hotkey<'static>,
    pub hotkey2: Hotkey<'static>,
    pub hotkey3: Hotkey<'static>,
    // append more hotkeys here
}

#[derive(Debug)]
pub struct HotkeySettings {
    pub capture_key: bool,
    pub reinitialize_hotkeys: bool,
    pub all_hotkeys: CustomHotkeys,
}

pub struct ScaleSettings{
    pub global_scale: f32,
}
pub struct GeneralSettings {
    pub scaling: ScaleSettings,
    pub cursor_hittest: bool,
    pub window_size: Vec2,
    pub window_pos: Pos2,
    pub reinitialize: bool,
    pub setup: bool,
    pub cursor_location: Pos2,
    pub window_on_top: bool,
    pub transparent: bool,
    pub always_on_top: bool,
}

pub struct UiSettings{
    pub position: egui::Rect,
    pub size: f32,
    pub color: egui::Color32,
}
pub struct WidgetSettings{
    pub edit_button : UiSettings,
    pub open_window_button: UiSettings,
    pub quit_button: UiSettings,
}

pub struct App {

    // Temporary
    show_window_1: bool,
    some_window_open: bool,
    clipboard_manager: clipboard::ClipboardContext,
    some_val: i32,
    item_info: String,
    current_clipboard: String,

    // any textures are stored here
    pub textures: Textures,
    // if the gui is in edit mode
    pub edit_mode: bool,
    // which tab is currently active in the edit mode
    pub edit_mode_tab: Vec<bool>,
    // Settings for the item inspection
    pub item_inspection_settings: ItemInspectionSettings,
    // General Settings (regarding the window and window setup)
    pub general_settings: GeneralSettings,
    // Hotkey Settings (Our Hotkeys)
    pub hotkey_settings: HotkeySettings,
    // Widget Settings (Settings of the widget Positions & Sizes)
    pub widget_settings: WidgetSettings,
}

impl Default for App {
    fn default() -> Self {
        Self {
            textures: Textures { test_image: Texture { texture_handle: None, memory_location:None, loaded: false } },
            widget_settings: WidgetSettings { 
                edit_button: UiSettings { position: egui::Rect{min: egui::Pos2{x:50.0, y:100.0},
                                                            max: egui::Pos2{x:50.0, y:200.0}},
                                        size: 100.0,
                                        color: egui::Color32::LIGHT_GRAY }, 
                open_window_button: UiSettings { position: egui::Rect{min: egui::Pos2{x:50.0, y:200.0},
                                                                    max: egui::Pos2{x:50.0, y:300.0}},
                                                size: 100.0,
                                                color: egui::Color32::BLACK }, 
                quit_button: UiSettings {position: egui::Rect{min: egui::Pos2{x:50.0, y:200.0},
                                                            max: egui::Pos2{x:50.0, y:300.0}},
                                        size: 100.0, 
                                        color: egui::Color32::RED  } },
            hotkey_settings: HotkeySettings {
                reinitialize_hotkeys: true,
                capture_key: false,
                all_hotkeys: CustomHotkeys {
                    hotkey_item_inspection: Hotkey::new(vec![], "hotkey_item_inspection"),
                    hotkey1: Hotkey::new(vec![], "hotkey1"),
                    hotkey2: Hotkey::new(vec![], "hotkey2"),
                    hotkey3: Hotkey::new(vec![], "hotkey3"),
                    // append more hotkeys here
                },

            },
            general_settings: GeneralSettings {
                scaling: ScaleSettings { global_scale: 1.0 },
                cursor_hittest: false,
                window_size: Vec2 {
                    x: 1919.0,
                    y: 1032.0,
                },
                window_pos: Pos2 { x: 0.0, y: 0.0 },
                reinitialize: true,
                cursor_location: Pos2 { x: 0.0, y: 0.0 },
                setup: false,
                window_on_top: true,
                transparent: true,
                always_on_top: false,
            },
            item_inspection_settings: ItemInspectionSettings {
                hotkey_item_inspection_pressed: false,
                hotkey_item_inspection_pressed_initial_position: Pos2 { x: 0.0, y: 0.0 },
                hotkey_item_inspection_pressed_first: true,
            },
            edit_mode: false,
            show_window_1: false,
            some_window_open: false,
            clipboard_manager: ClipboardProvider::new().unwrap(),
            some_val: 0,
            edit_mode_tab: vec![true, false, false],
            item_info: String::from("NoneItem"),
            current_clipboard: String::from("NoneCurrent"),
        }
    }
}
