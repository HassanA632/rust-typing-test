use crate::core::history::ResultEntry;
use crate::core::settings::Settings;

const WEB_KEY: &str = "rust_typing_test_settings";
const RESULTS_WEB_KEY: &str = "rust_typing_test_results";

pub fn load_settings() -> Settings {
    #[cfg(target_arch = "wasm32")]
    {
        load_settings_web().unwrap_or_default()
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        load_settings_native().unwrap_or_default()
    }
}

pub fn save_settings(settings: &Settings) {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = save_settings_web(settings);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = save_settings_native(settings);
    }
}

#[cfg(target_arch = "wasm32")]
fn load_settings_web() -> Option<Settings> {
    use web_sys::window;

    let storage = window()?.local_storage().ok()??;
    let raw = storage.get_item(WEB_KEY).ok()??;
    serde_json::from_str(&raw).ok()
}

#[cfg(target_arch = "wasm32")]
fn save_settings_web(settings: &Settings) -> Result<(), ()> {
    use web_sys::window;

    let storage = window()
        .ok_or(())?
        .local_storage()
        .map_err(|_| ())?
        .ok_or(())?;
    let raw = serde_json::to_string(settings).map_err(|_| ())?;
    storage.set_item(WEB_KEY, &raw).map_err(|_| ())?;
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn load_settings_native() -> Option<Settings> {
    let path = native_settings_path()?;
    let raw = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&raw).ok()
}

#[cfg(not(target_arch = "wasm32"))]
fn save_settings_native(settings: &Settings) -> Option<()> {
    let path = native_settings_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).ok()?;
    }
    let raw = serde_json::to_string_pretty(settings).ok()?;
    std::fs::write(path, raw).ok()?;
    Some(())
}

#[cfg(not(target_arch = "wasm32"))]
fn native_settings_path() -> Option<std::path::PathBuf> {
    Some(std::path::PathBuf::from("data").join("settings.json"))
}

pub fn load_results() -> Vec<ResultEntry> {
    #[cfg(target_arch = "wasm32")]
    {
        load_results_web().unwrap_or_default()
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        load_results_native().unwrap_or_default()
    }
}

pub fn save_results(results: &[ResultEntry]) {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = save_results_web(results);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = save_results_native(results);
    }
}

#[cfg(target_arch = "wasm32")]
fn load_results_web() -> Option<Vec<ResultEntry>> {
    use web_sys::window;

    let storage = window()?.local_storage().ok()??;
    let raw = storage.get_item(RESULTS_WEB_KEY).ok()??;
    serde_json::from_str(&raw).ok()
}

#[cfg(target_arch = "wasm32")]
fn save_results_web(results: &[ResultEntry]) -> Result<(), ()> {
    use web_sys::window;

    let storage = window()
        .ok_or(())?
        .local_storage()
        .map_err(|_| ())?
        .ok_or(())?;
    let raw = serde_json::to_string(results).map_err(|_| ())?;
    storage.set_item(RESULTS_WEB_KEY, &raw).map_err(|_| ())?;
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn load_results_native() -> Option<Vec<ResultEntry>> {
    let path = native_results_path()?;
    let raw = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&raw).ok()
}

#[cfg(not(target_arch = "wasm32"))]
fn save_results_native(results: &[ResultEntry]) -> Option<()> {
    let path = native_results_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).ok()?;
    }
    let raw = serde_json::to_string_pretty(results).ok()?;
    std::fs::write(path, raw).ok()?;
    Some(())
}

#[cfg(not(target_arch = "wasm32"))]
fn native_results_path() -> Option<std::path::PathBuf> {
    Some(std::path::PathBuf::from("data").join("results.json"))
}
