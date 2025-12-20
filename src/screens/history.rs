use egui::Ui;

use crate::app::Screen;

pub fn ui(ui: &mut Ui, screen: &mut Screen) {
    ui.heading("History");
    ui.label("Past results will show here.");
    ui.separator();

    if ui.button("Back").clicked() {
        *screen = Screen::Menu;
    }
}
