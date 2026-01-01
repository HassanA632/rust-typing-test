use egui::Ui;

use crate::app::Screen;
use crate::core::test::TestMode;

pub fn ui(ui: &mut Ui, screen: &mut Screen) {
    ui.heading("Rust Typing Test");
    ui.separator();

    if ui.button("Start").clicked() {
        *screen = Screen::Test(TestMode::Normal);
    }

    if ui.button("Practice").clicked() {
        *screen = Screen::Test(TestMode::Practice);
    }

    if ui.button("History").clicked() {
        *screen = Screen::History;
    }

    if ui.button("Options").clicked() {
        *screen = Screen::Options;
    }
}
