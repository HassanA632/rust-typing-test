use egui::{Key, Ui};

use crate::app::Screen;
use crate::core::{
    test::{TestSession, TestState},
    words,
};

pub fn ui(ui: &mut Ui, screen: &mut Screen, session: &mut Option<TestSession>) {
    // Create a session the first time we enter this screen
    let s = session.get_or_insert_with(|| TestSession::new(words::generate_prompt(5)));

    ui.heading("Typing Test");
    ui.separator();

    // Preview next 6 words
    let preview = s.next_preview(6).join(" ");
    ui.label(preview);

    ui.add_space(8.0);

    // Text input
    let response = ui.text_edit_singleline(&mut s.input);

    // Start timer on first keystroke (when input changes)
    if response.changed() && s.state == TestState::Waiting {
        s.on_first_input_if_needed();
    }

    // Submit word on space or enter
    if ui.input(|i| i.key_pressed(Key::Space) || i.key_pressed(Key::Enter)) {
        s.submit_current_word();
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

    if ui.button("Exit").clicked() {
        *screen = Screen::Menu;
        *session = None;
    }
}
