use egui::Ui;

use crate::app::Screen;
use crate::core::history::ResultEntry;
use crate::core::storage;

pub fn ui(ui: &mut Ui, screen: &mut Screen, results: &mut Vec<ResultEntry>) {
    ui.heading("History");
    ui.separator();

    if results.is_empty() {
        ui.label("No results yet. Do a test first!");
    } else {
        // Latest test first
        for (i, r) in results.iter().rev().enumerate() {
            ui.label(format!(
                "{}. WPM: {:.1} | Time: {:.1}s | {}/{} correct",
                i + 1,
                r.wpm,
                r.elapsed_secs,
                r.correct_words,
                r.total_words
            ));
        }
    }

    ui.add_space(12.0);

    ui.horizontal(|ui| {
        let total_buttons_w = 120.0 + 8.0 + 60.0;
        let remaining = ui.available_width() - total_buttons_w;
        if remaining > 0.0 {
            ui.add_space(remaining / 2.0);
        }

        if ui.button("Clear history").clicked() {
            results.clear();
            storage::save_results(results);
        }

        ui.add_space(8.0);

        if ui.button("Back").clicked() {
            *screen = Screen::Menu;
        }
    });
}
