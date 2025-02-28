// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::command;
use tauri::{AppHandle, Manager};

#[command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
#[tauri::command]
async fn download_file(app_handle: AppHandle, url: String) -> Result<String, String> {
    println!("Downloading file from: {}", url);

    // Access the app's local data directory
    let app_dir = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;

    let download_dir = app_dir.join("downloads");

    // Create the downloads directory if it doesn't exist
    tokio::fs::create_dir_all(&download_dir)
        .await
        .map_err(|e| e.to_string())?;

    // Extract the filename from the URL
    let file_name = url.split('/').last().unwrap_or("download.zip");
    let file_path = download_dir.join(file_name);

    println!("Saving to: {:?}", file_path);

    // Download the file
    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Failed to download: HTTP {}", response.status()));
    }

    // Save the file
    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    tokio::fs::write(&file_path, &bytes)
        .await
        .map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(file_path.to_string_lossy().to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init()) // Add dialog plugin
        .invoke_handler(tauri::generate_handler![greet, download_file])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
