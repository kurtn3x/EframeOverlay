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

#[derive(Debug)]
pub struct CustomHotkeys {
    // those are temp hotkeys, they are used to store the hotkey and later copy the hotkey into the respective one in all_hotkeys
    pub hotkey_item_inspection: Vec<KeybdKey>,
    pub hotkey1: Vec<KeybdKey>,
    pub hotkey2: Vec<KeybdKey>,
    pub hotkey3: Vec<KeybdKey>,
    // append more hotkeys here
}

#[derive(Debug)]
pub struct HotkeySettings {
    pub capture_key: bool,
    pub reinitialize_hotkeys: bool,
    pub all_hotkeys: Vec<Hotkey<'static>>,
    pub custom_hotkeys: CustomHotkeys,
}

pub struct GeneralSettings {
    pub global_scale: f32,
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

pub struct App {
    pub edit_mode: bool,
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
    pub hotkey_settings: HotkeySettings,
}

impl Default for App {
    fn default() -> Self {
        Self {
            hotkey_settings: HotkeySettings {
                reinitialize_hotkeys: true,
                capture_key: false,
                // those are the real hotkeys, that we will use to iterate over
                all_hotkeys: vec![],

                // those are temp hotkeys, they are used to store the hotkey and later copy the hotkey into the respective one in all_hotkeys
                custom_hotkeys: CustomHotkeys {
                    hotkey_item_inspection: vec![],
                    hotkey1: vec![],
                    hotkey2: vec![],
                    hotkey3: vec![],
                    // append more hotkeys here
                },
            },
            general_settings: GeneralSettings {
                global_scale: 1.0,
                cursor_hittest: false,
                window_size: Vec2 {
                    x: 1919.0,
                    y: 1032.0,
                },
                window_pos: Pos2 { x: 0.0, y: 0.0 },
                reinitialize: true,
                cursor_location: Pos2 { x: 0.0, y: 0.0 },
                setup: true,
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
            some_option: 1,
        }
    }
}
