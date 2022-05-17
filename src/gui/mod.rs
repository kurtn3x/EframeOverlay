mod components;
use components::app::ItemInspectionSettings;
use components::app::GeneralSettings;
use components::app::MyHotkeys;

pub struct App {
    pub edit_mode : bool,
    // subwindow handling
    show_window_1 : bool,
    some_window_open: bool,
    clipboard_manager: clipboard::ClipboardContext,
    some_val : i32,
    pub edit_mode_tab : Vec<bool>,
    item_info : String,
    current_clipboard: String,
    pub some_option: u8,
    pub item_inspection_settings: ItemInspectionSettings,
    pub general_settings: GeneralSettings,
    pub my_hotkeys: MyHotkeys,
}