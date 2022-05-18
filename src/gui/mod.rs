use self::components::hotkeymanager::Hotkey;
use clipboard::ClipboardProvider;
use egui::{Pos2, Vec2};
use inputbot::KeybdKey;
mod components;

pub struct ItemInspectionSettings {
    pub hotkey_item_inspection_pressed: bool,
    pub hotkey_item_inspection_pressed_initial_position: Pos2,
    pub hotkey_item_inspection_pressed_first: bool,
}

pub struct MyHotkeys {
    pub capture_key: bool,
    pub reinizialize_hotkeys: bool,
    pub all_hotkeys: Vec<Hotkey<'static>>,
    pub hotkey_item_inspection: Vec<KeybdKey>,
}

pub struct GeneralSettings {
    pub cursor_hittest: bool,
    pub window_size: Vec2,
    pub window_pos: Pos2,
    pub first_run: bool,
    pub setup: bool,
    pub cursor_location: Pos2,
    pub window_on_top: bool,
    pub transparent: bool,
    pub decorated: bool,
}

pub struct App {
    pub edit_mode: bool,
    // subwindow handling
    show_window_1: bool,
    some_window_open: bool,
    clipboard_manager: clipboard::ClipboardContext,
    some_val: i32,
    pub edit_mode_tab: Vec<bool>,
    item_info: String,
    current_clipboard: String,
    pub some_option: u8,
    pub item_inspection_settings: ItemInspectionSettings,
    pub general_settings: GeneralSettings,
    pub my_hotkeys: MyHotkeys,
}

impl Default for App {
    fn default() -> Self {
        Self {
            my_hotkeys: MyHotkeys {
                reinizialize_hotkeys: true,
                capture_key: false,
                all_hotkeys: vec![],
                hotkey_item_inspection: vec![],
            },
            general_settings: GeneralSettings {
                cursor_hittest: false,
                window_size: Vec2 {
                    x: 1919.0,
                    y: 1032.0,
                },
                window_pos: Pos2 { x: 0.0, y: 0.0 },
                first_run: true,
                cursor_location: Pos2 { x: 0.0, y: 0.0 },
                setup: true,
                window_on_top: true,
                decorated: false,
                transparent: true,
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
            some_option: 1,
        }
    }
}
