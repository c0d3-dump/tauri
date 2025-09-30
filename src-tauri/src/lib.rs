mod commands;
mod events;
mod plugins;

use std::{sync::Mutex, time::Duration};

use tauri::Listener;
use tauri_plugin_global_shortcut::ShortcutState;
use tauri_plugin_updater::UpdaterExt;

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

            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                loop {
                    match update(handle.clone()).await {
                        Ok(_) => println!("Update check completed"),
                        Err(e) => eprintln!("Update check failed: {}", e),
                    }

                    // Wait 5 minutes before next check
                    tokio::time::sleep(Duration::from_secs(600)).await;
                }
            });

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

async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    println!("update called!");

    if let Some(update) = app.updater()?.check().await? {
        println!("update available!");
        let mut downloaded = 0;

        update
            .download_and_install(
                |chunk_length, content_length| {
                    downloaded += chunk_length;
                    println!("downloaded {downloaded} from {content_length:?}");
                },
                || {
                    println!("download finished!");
                },
            )
            .await?;

        println!("update installed");
        app.restart();
    } else {
        println!("no update required");
    }

    Ok(())
}
