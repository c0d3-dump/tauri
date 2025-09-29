mod commands;
mod events;
mod plugins;

use std::sync::Mutex;

use tauri::Listener;
use tauri_plugin_global_shortcut::ShortcutState;

#[derive(Debug, Default)]
struct AppData {
    counter: u32,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            app.listen("download-started", events::download_started);

            Ok(())
        })
        .manage(Mutex::new(AppData::default()))
        .on_webview_event(events::webview)
        .plugin(plugins::global_shortcut_plugin())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::result,
            commands::option,
            commands::async_func,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
