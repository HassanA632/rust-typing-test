pub struct TypingApp;

impl TypingApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self
    }
}

impl eframe::App for TypingApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust Typing Test");
            ui.label("Scaffold running");
            ui.separator();
            ui.label("Start / History / Options");
        });
    }
}
