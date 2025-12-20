use egui::Ui;

use crate::app::Screen;

pub fn ui(ui: &mut Ui, screen: &mut Screen) {
    ui.heading("Rust Typing Test");
    ui.separator();

    if ui.button("Start").clicked() {
        *screen = Screen::Test;
    }
    if ui.button("History").clicked() {
        *screen = Screen::History;
    }
    if ui.button("Options").clicked() {
        *screen = Screen::Options;
    }
}
