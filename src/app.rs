//! App-level state and screen routing.
//!
//! This file holds the top-level state machine (`Screen`) and the root `eframe::App`
//! implementation. Individual screen UIs live in `src/screens/*` so the UI code stays
//! modular while sharing a single app state.

pub enum Screen {
    Menu,
    Test,
    History,
    Options,
}

pub struct TypingApp {
    screen: Screen,
}

impl TypingApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            screen: Screen::Menu,
        }
    }
}

impl eframe::App for TypingApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| match self.screen {
            Screen::Menu => crate::screens::menu::ui(ui, &mut self.screen),
            Screen::Test => crate::screens::test::ui(ui, &mut self.screen),
            Screen::History => crate::screens::history::ui(ui, &mut self.screen),
            Screen::Options => crate::screens::options::ui(ui, &mut self.screen),
        });
    }
}
