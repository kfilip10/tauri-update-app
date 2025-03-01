mod update; // Import the update module

use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager};
use update::UpdateProgress;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(update::UpdateState {
                progress: Arc::new(Mutex::new(UpdateProgress::default())),
            });

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            #[cfg(not(any(target_os = "android", target_os = "ios")))]
            app.handle()
                .plugin(tauri_plugin_updater::Builder::new().build())?;

            #[cfg(not(any(target_os = "android", target_os = "ios")))]
            app.handle().plugin(tauri_plugin_dialog::init())?;

            #[cfg(not(any(target_os = "android", target_os = "ios")))]
            app.handle().plugin(tauri_plugin_http::init())?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            update::check_for_updates, // Move updates to the new module
            update::download_and_install_update,
            update::get_update_progress,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
