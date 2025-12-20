use egui::Ui;

use crate::app::Screen;

pub fn ui(ui: &mut Ui, screen: &mut Screen) {
    ui.heading("Test");
    ui.label("Typing test screen");
    ui.separator();

    if ui.button("Back").clicked() {
        *screen = Screen::Menu;
    }
}
