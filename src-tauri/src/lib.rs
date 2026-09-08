mod resizer;

use resizer::{process_batch_images, ProcessProgress, ProcessResult, ResizeOptions};
use tauri::ipc::Channel;

#[tauri::command]
async fn resize_images(
    paths: Vec<String>,
    options: ResizeOptions,
    on_progress: Channel<ProcessProgress>,
) -> Result<Vec<ProcessResult>, String> {
    tauri::async_runtime::spawn_blocking(move || process_batch_images(paths, options, on_progress))
        .await
        .map_err(|e| format!("Task execution failed: {}", e))
}

#[tauri::command]
fn open_folder(path: String) -> Result<(), String> {
    // Ensure directory exists on disk before attempting to open it in OS file manager
    let _ = std::fs::create_dir_all(&path);

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![resize_images, open_folder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
