mod app;
mod screens;

// The same app can run natively (desktop) or in the browser (WASM).
// `#[cfg(...)]` selects the correct startup path at compile time.
fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let options = eframe::NativeOptions::default();
        eframe::run_native(
            "Typing Test",
            options,
            Box::new(|cc| Ok(Box::new(app::TypingApp::new(cc)))),
        )
        .expect("failed to start native app");
    }

    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
        wasm_logger::init(wasm_logger::Config::default());

        wasm_bindgen_futures::spawn_local(async {
            use wasm_bindgen::JsCast;

            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");
            let canvas = document
                .get_element_by_id("the_canvas_id")
                .expect("canvas element not found")
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .expect("element is not a canvas");

            let web_options = eframe::WebOptions::default();
            let runner = eframe::WebRunner::new();
            runner
                .start(
                    canvas,
                    web_options,
                    Box::new(|cc| Ok(Box::new(app::TypingApp::new(cc)))),
                )
                .await
                .expect("failed to start eframe");
        });
    }
}
