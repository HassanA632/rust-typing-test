use egui::{Key, TextEdit, Ui};

use crate::app::Screen;
use crate::core::{
    test::{TestSession, TestState},
    words,
};

pub fn ui(ui: &mut Ui, screen: &mut Screen, session: &mut Option<TestSession>) {
    let s = session.get_or_insert_with(|| TestSession::new(words::generate_prompt(30)));

    ui.heading("Typing Test");
    ui.separator();

    // Preview next words
    let preview = s.next_preview(6).join(" ");
    ui.label(preview);

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
