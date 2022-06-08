pub mod app;
pub mod background_mode;
pub mod edit_mode;
pub mod hotkeymanager;
pub mod setup;
use super::super::App;
use eframe::{egui::Ui, Frame};

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
    fn run(ctx: &egui::Context, frame: &mut Frame, app: &mut App) {}
}
