use egui::text::LayoutJob;
use egui::{Align, Color32, FontId, Key, TextEdit, TextFormat, TextStyle, Ui};

use crate::app::Screen;
use crate::core::history::ResultEntry;
use crate::core::storage;
use crate::core::{
    test::{TestSession, TestState},
    words,
};

pub fn ui(
    ui: &mut Ui,
    screen: &mut Screen,
    session: &mut Option<TestSession>,
    results: &mut Vec<crate::core::history::ResultEntry>,
) {
    let s = session.get_or_insert_with(|| TestSession::new(words::generate_prompt(30)));

    ui.heading("Typing Test");
    ui.separator();

    // Preview next 6 words
    let preview_words = s.next_preview(6);

    let base_size = ui
        .style()
        .text_styles
        .get(&TextStyle::Body)
        .map(|f| f.size)
        .unwrap_or(18.0);

    let preview_size = base_size + 8.0;

    let mut job = LayoutJob::default();
    job.halign = Align::Center;

    for (i, w) in preview_words.iter().enumerate() {
        if i > 0 {
            job.append(
                " ",
                0.0,
                TextFormat {
                    font_id: FontId::proportional(preview_size),
                    ..Default::default()
                },
            );
        }

        let color = if i == 0 {
            Color32::from_rgb(80, 200, 120)
        } else {
            ui.visuals().text_color()
        };

        job.append(
            w,
            0.0,
            TextFormat {
                font_id: FontId::proportional(preview_size),
                color,
                ..Default::default()
            },
        );
    }

    ui.label(job);

    ui.add_space(8.0);

    // Fixed message line (prevents layout jumping)
    let msg = s.last_feedback.as_deref().unwrap_or("");
    ui.label(msg);

    // Input with stable id + focus once
    let input_id = egui::Id::new("typing_input");
    let response = ui.add(TextEdit::singleline(&mut s.input).id(input_id));

    if s.should_focus_input {
        response.request_focus();
        s.should_focus_input = false;
    }

    // Prevent spaces inside the input box (spaces are used to submit)
    if s.input.contains(' ') {
        s.input = s.input.replace(' ', "");
    }

    // Start timer on first real input
    if response.changed() && s.state == TestState::Waiting && !s.input.is_empty() {
        s.on_first_input_if_needed();
    }

    // Submit only when the input is focused
    if response.has_focus() && ui.input(|i| i.key_pressed(Key::Space)) {
        s.attempt_submit_current_word();
    }

    ui.add_space(8.0);

    ui.label(format!("Progress: {}/{}", s.current_index, s.words.len()));

    if let Some(elapsed) = s.elapsed() {
        ui.label(format!("Time: {:.1}s", elapsed.as_secs_f32()));
    }

    // Finished state
    if s.state == TestState::Finished {
        if !s.result_saved {
            let wpm = s.wpm().unwrap_or(0.0);
            let elapsed_secs = s.elapsed().map(|d| d.as_secs_f32()).unwrap_or(0.0);

            results.push(ResultEntry {
                wpm,
                elapsed_secs,
                correct_words: s.correct_words as u32,
                total_words: s.words.len() as u32,
            });

            storage::save_results(results);
            s.result_saved = true;
        }

        ui.separator();
        let wpm = s.wpm().unwrap_or(0.0);
        ui.heading(format!("Done! WPM: {:.1}", wpm));

        if ui.button("Back to menu").clicked() {
            *screen = Screen::Menu;
            *session = None; // reset for next time
        }
        return;
    }

    ui.separator();

    let button_h = ui.spacing().interact_size.y;
    let restart_w = 90.0;
    let exit_w = 60.0;
    let gap = 10.0;
    let total_w = restart_w + gap + exit_w;

    let available = ui.available_rect_before_wrap();
    let x = available.center().x - total_w / 2.0;
    let y = ui.cursor().min.y;

    let rect = egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(total_w, button_h));
    ui.allocate_ui_at_rect(rect, |ui| {
        if ui
            .add_sized([restart_w, button_h], egui::Button::new("Restart"))
            .clicked()
        {
            *session = None;
        }

        ui.add_space(gap);

        if ui
            .add_sized([exit_w, button_h], egui::Button::new("Exit"))
            .clicked()
        {
            *screen = Screen::Menu;
            *session = None;
        }
    });

    // If restart was clicked, stop drawing the rest of this frame.
    if session.is_none() {
        return;
    }
}
