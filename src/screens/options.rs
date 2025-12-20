use egui::Ui;

use crate::app::Screen;

pub fn ui(ui: &mut Ui, screen: &mut Screen) {
    ui.heading("Options");
    ui.label("Theme + text size will go here.");
    ui.separator();

    if ui.button("Back").clicked() {
        *screen = Screen::Menu;
    }
}
