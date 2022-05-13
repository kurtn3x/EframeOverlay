pub mod app;
pub mod hotkeymanager;
pub mod edit_mode;
pub mod background_mode;
use eframe::{egui::Ui, Frame};
use super::super::App;



#[derive(Debug, PartialEq, Eq)]
pub enum EditModeTabs {
    Tab0,
    Tab1,
    Tab2,
    Tab3,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GUIMode {
    BackgroundMode,
    EditMode,
}

pub trait AppComponent {

    // #[allow(unused)]
    // fn add(ctx: &mut Self::Context, ui: &mut Ui) {}

    #[allow(unused)]
    fn add (ctx: &egui::Context, frame: &Frame, app: &mut App) {}
}
