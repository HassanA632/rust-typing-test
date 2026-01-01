//! App-level state and screen routing.
//!
//! This file holds the top-level state machine (`Screen`) and the root `eframe::App`
//! implementation. Individual screen UIs live in `src/screens/*` so the UI code stays
//! modular while sharing a single app state.
//!
use crate::core::history::ResultEntry;
use crate::core::settings::{Settings, Theme};
use crate::core::storage;
use crate::core::test::TestSession;
use crate::core::weak_words::WeakWords;

pub enum Screen {
    Menu,
    Test,
    History,
    Options,
}

pub struct TypingApp {
    screen: Screen,
    settings: Settings,
    test: Option<TestSession>,
    results: Vec<ResultEntry>,
    weak_words: WeakWords,
}

impl TypingApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let settings = storage::load_settings();
        let results = storage::load_results();
        let weak_words = storage::load_weak_words();

        Self {
            screen: Screen::Menu,
            settings,
            test: None,
            results,
            weak_words,
        }
    }
}

impl eframe::App for TypingApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply theme + font sizing every frame.
        match self.settings.theme {
            Theme::Light => ctx.set_visuals(egui::Visuals::light()),
            Theme::Dark => ctx.set_visuals(egui::Visuals::dark()),
        }

        let mut style = (*ctx.style()).clone();
        style.text_styles.iter_mut().for_each(|(_, font)| {
            font.size = self.settings.text_size.points();
        });
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(ui.available_height() * 0.15); // push content down a bit

                egui::Frame::NONE.show(ui, |ui| {
                    ui.set_max_width(600.0); // keeps UI from stretching too wide

                    match self.screen {
                        Screen::Menu => crate::screens::menu::ui(ui, &mut self.screen),
                        Screen::Test => crate::screens::test::ui(
                            ui,
                            &mut self.screen,
                            &mut self.test,
                            &mut self.results,
                            &mut self.weak_words,
                        ),
                        Screen::History => {
                            crate::screens::history::ui(ui, &mut self.screen, &mut self.results)
                        }

                        Screen::Options => {
                            crate::screens::options::ui(ui, &mut self.screen, &mut self.settings)
                        }
                    }
                });
            });
        });
    }
}
