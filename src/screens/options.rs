use egui::Ui;

use crate::app::Screen;
use crate::core::settings::{Settings, TextSize, Theme};
use crate::core::storage;

pub fn ui(ui: &mut Ui, screen: &mut Screen, settings: &mut Settings) {
    ui.heading("Options");
    ui.separator();

    egui::Grid::new("options_grid")
        .num_columns(2)
        .spacing([16.0, 10.0])
        .show(ui, |ui| {
            ui.label("Theme");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut settings.theme, Theme::Dark, "Dark");
                ui.selectable_value(&mut settings.theme, Theme::Light, "Light");
            });
            ui.end_row();

            ui.label("Text size");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut settings.text_size, TextSize::Small, "Small");
                ui.selectable_value(&mut settings.text_size, TextSize::Medium, "Medium");
                ui.selectable_value(&mut settings.text_size, TextSize::Large, "Large");
            });
            ui.end_row();
        });

    ui.add_space(12.0);

    ui.horizontal(|ui| {
        let total_buttons_w = 52.0 + 8.0 + 52.0; // approx widths for "Save" + gap + "Back"
        let remaining = ui.available_width() - total_buttons_w;
        if remaining > 0.0 {
            ui.add_space(remaining / 2.0);
        }

        if ui.button("Save").clicked() {
            storage::save_settings(settings);
        }

        ui.add_space(8.0);

        if ui.button("Back").clicked() {
            *screen = Screen::Menu;
        }
    });
}
