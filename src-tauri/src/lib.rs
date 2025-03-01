use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager};
use tauri_plugin_updater::UpdaterExt;

// Define a struct to store update progress
struct UpdateState {
    progress: Arc<Mutex<UpdateProgress>>,
}

struct UpdateProgress {
    downloading: bool,
    percent: f64,
    downloaded: u64,
    total: Option<u64>,
    complete: bool,
    error: Option<String>,
}

impl Default for UpdateProgress {
    fn default() -> Self {
        Self {
            downloading: false,
            percent: 0.0,
            downloaded: 0,
            total: None,
            complete: false,
            error: None,
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialize the state
            app.manage(UpdateState {
                progress: Arc::new(Mutex::new(UpdateProgress::default())),
            });

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Add updater plugin only for desktop platforms
            #[cfg(not(any(target_os = "android", target_os = "ios")))]
            app.handle()
                .plugin(tauri_plugin_updater::Builder::new().build())?;

            // Add dialog plugin for messages
            #[cfg(not(any(target_os = "android", target_os = "ios")))]
            app.handle().plugin(tauri_plugin_dialog::init())?;

            // Add HTTP plugin
            #[cfg(not(any(target_os = "android", target_os = "ios")))]
            app.handle().plugin(tauri_plugin_http::init())?;

            Ok(())
        })
        // Register the commands
        .invoke_handler(tauri::generate_handler![
            check_for_updates,
            download_and_install_update,
            get_update_progress
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Command to check for updates but not install
#[tauri::command]
async fn check_for_updates(app_handle: tauri::AppHandle) -> Result<String, String> {
    let updater = app_handle.updater().map_err(|e| e.to_string())?;

    match updater.check().await {
        Ok(update) => {
            if let Some(update) = update {
                // Return update info as JSON

                let update_info = serde_json::json!({
                    "available": true,
                    "version": update.version,
                    "body": update.body,
                    "downloadUrl": update.download_url
                });
                Ok(serde_json::to_string(&update_info).unwrap())
            } else {
                Ok(r#"{"available": false}"#.to_string())
            }
        }
        Err(e) => Err(format!("Failed to check for updates: {}", e)),
    }
}

// Command to download and install the update
#[tauri::command]
async fn download_and_install_update(app_handle: tauri::AppHandle) -> Result<(), String> {
    // Get the state
    let state = app_handle.state::<UpdateState>();
    let progress_clone = state.progress.clone();

    // Set downloading flag
    {
        let mut progress = progress_clone.lock().unwrap();
        progress.downloading = true;
        progress.percent = 0.0;
        progress.downloaded = 0;
        progress.complete = false;
        progress.error = None;
    }

    let updater = app_handle.updater().map_err(|e| e.to_string())?;

    // Check for update
    let update = match updater.check().await {
        Ok(Some(update)) => update,
        Ok(None) => {
            // Update state with error
            let mut progress = progress_clone.lock().unwrap();
            progress.error = Some("No update available".to_string());
            return Err("No update available".to_string());
        }
        Err(e) => {
            // Update state with error
            let mut progress = progress_clone.lock().unwrap();
            progress.error = Some(e.to_string());
            return Err(e.to_string());
        }
    };

    // Get clones for the closures
    let progress_for_progress = progress_clone.clone();
    let progress_for_complete = progress_clone.clone();

    // Download and install with progress tracking
    match update
        .download_and_install(
            // Progress callback
            // Progress callback
            move |chunk_length, content_length| {
                let mut progress = progress_for_progress.lock().unwrap();
                // Convert from usize to u64
                progress.downloaded += chunk_length as u64;
                progress.total = content_length; // No conversion needed, it's already Option<u64>

                // Calculate percentage if total is known
                if let Some(total) = content_length {
                    if total > 0 {
                        progress.percent = (progress.downloaded as f64 / total as f64) * 100.0;
                    }
                }

                // With this:
                println!(
                    "Downloaded {} of {} bytes ({:.1}%)",
                    progress.downloaded,
                    content_length.unwrap_or(0),
                    progress.percent
                );
            },
            // Completion callback
            move || {
                let mut progress = progress_for_complete.lock().unwrap();
                progress.complete = true;
                progress.percent = 100.0;
                println!("Download finished");
            },
        )
        .await
    {
        Ok(_) => {
            println!("Update installed, will restart");
            Ok(())
        }
        Err(e) => {
            // Update state with error
            let mut progress = progress_clone.lock().unwrap();
            progress.error = Some(e.to_string());
            Err(e.to_string())
        }
    }
}

// Command to get the current update progress
#[tauri::command]
fn get_update_progress(app_handle: tauri::AppHandle) -> Result<String, String> {
    let state = app_handle.state::<UpdateState>();
    let progress = state.progress.lock().unwrap();

    // Convert to JSON
    let progress_json = serde_json::json!({
        "downloading": progress.downloading,
        "percent": progress.percent,
        "downloaded": progress.downloaded,
        "total": progress.total,
        "complete": progress.complete,
        "error": progress.error
    });

    Ok(serde_json::to_string(&progress_json).unwrap())
}

#[tauri::command]
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
