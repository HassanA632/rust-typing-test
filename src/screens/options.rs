use egui::Ui;

use crate::app::Screen;
use crate::core::settings::{Settings, TextSize, Theme};
use crate::core::storage;

pub fn ui(ui: &mut Ui, screen: &mut Screen, settings: &mut Settings) {
    ui.heading("Options");
    ui.separator();

    ui.label("Theme");
    ui.horizontal(|ui| {
        ui.selectable_value(&mut settings.theme, Theme::Dark, "Dark");
        ui.selectable_value(&mut settings.theme, Theme::Light, "Light");
    });

    ui.add_space(8.0);
    ui.label("Text size");
    ui.horizontal(|ui| {
        ui.selectable_value(&mut settings.text_size, TextSize::Small, "Small");
        ui.selectable_value(&mut settings.text_size, TextSize::Medium, "Medium");
        ui.selectable_value(&mut settings.text_size, TextSize::Large, "Large");
    });

    ui.add_space(12.0);

    if ui.button("Save").clicked() {
        storage::save_settings(settings);
    }

    if ui.button("Back").clicked() {
        *screen = Screen::Menu;
    }
}
